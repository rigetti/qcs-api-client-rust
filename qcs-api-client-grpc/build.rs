// Copyright 2023 Rigetti Computing
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

use std::io::Write;
use std::{env, fs::OpenOptions, path::PathBuf};

// If the `regen` feature is set when building, this script will:
//   1. Regenerate the protobuf code, and
//   2. Copy the generated code to `./src/gen`.
// Regeneration should be performed if the protobuf definitions
// (under `./proto`) are changed. The regenerated code should then
// be tested and committed to the repo if necessary.

fn main() {
    if cfg!(not(feature = "regen")) {
        // No need to continue if there's no intention to regenerate the
        // protobuf code.
        return;
    }

    let out_dir =
        env::var("OUT_DIR").expect("OUT_DIR environment variable should be availabe in build.rs");

    let proto_relative_paths = [
        "controller/job.proto",
        "controller/readout.proto",
        "controller/service.proto",
        "translation/metadata.proto",
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

    let descriptor_path = PathBuf::from(&out_dir).join("proto_descriptor.bin");

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
        .server_mod_attribute("services.controller", r#"#[cfg(feature = "server")]"#)
        .server_mod_attribute("services.translation", r#"#[cfg(feature = "server")]"#)
        .compile_protos_with_config(config, &proto_files, &[root])
        .expect("Should compile protobuf code for server and client");

    let descriptor_set =
        std::fs::read(descriptor_path).expect("failed to read proto descriptor file");

    pbjson_build::Builder::new()
        .register_descriptors(&descriptor_set)
        .expect("failed to register descriptors")
        .build(&[
            ".models.controller",
            ".models.common",
            ".models.translation",
            ".services",
        ])
        .expect("failed to build with pbjson");

    let gen_code_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("gen");
    for entry in std::fs::read_dir(PathBuf::from(&out_dir))
        .expect("OUT_DIR environment variable should point to a valid directory")
    {
        let src_path = entry.expect("Should find valid files in OUT_DIR").path();
        if src_path.to_string_lossy().ends_with(".rs") {
            let dest = gen_code_dir.join(
                src_path
                    .file_name()
                    .expect("Should have a valid file path ending with a file name"),
            );

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(dest)
                .expect("Should open file for writing");
            writeln!(
                file,
                r##"// Copyright 2023 Rigetti Computing
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

"##
            )
            .expect("Should write license header");

            writeln!(
                file,
                "{}",
                std::fs::read_to_string(src_path).expect("Should read file contents")
            )
            .expect("Should write file contents");
        }
    }
}
