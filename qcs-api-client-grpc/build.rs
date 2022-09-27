// Copyright 2022 Rigetti Computing
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{env, path::PathBuf};

fn main() {
    let proto_relative_paths = [
        "controller/job.proto",
        "controller/readout.proto",
        "controller/service.proto",
        "translation/service.proto",
    ];

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("proto");

    let proto_files: Vec<PathBuf> = proto_relative_paths
        .into_iter()
        .map(|p| root.join(p))
        .collect();

    // Tell cargo to recompile if any of these proto files are changed
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
    }

    let descriptor_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("proto_descriptor.bin");

    let mut config = prost_build::Config::new();

    config
        // Save descriptors to file
        .file_descriptor_set_path(&descriptor_path)
        // Override prost-types with pbjson-types
        .compile_well_known_types()
        .extern_path(".google.protobuf", "::pbjson_types");

    config.type_attribute(
        "services.controller.ExecuteControllerJobRequest.job",
        "#[derive(serde::Deserialize)]",
    );

    config.protoc_arg("--experimental_allow_proto3_optional");
    tonic_build::configure()
        .build_client(true)
        .build_server(std::env::var("CARGO_FEATURE_SERVER").is_ok())
        .compile_with_config(config, &proto_files, &[root])
        .expect("failed to build");

    let descriptor_set =
        std::fs::read(descriptor_path).expect("failed to read proto descriptor file");

    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)
        .expect("failed to register descriptors")
        .build(&[".models.controller", ".models.common", ".services"])
        .expect("failed to build with pbjson");
}
