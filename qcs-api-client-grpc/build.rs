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

    use qcs_dependencies_client::codegen_proto;

    let mut builder = codegen_proto::Builder::new("proto", "src/gen");

    builder.prost_config.type_attribute(
        "services.controller.ExecuteControllerJobRequest.job",
        "#[derive(serde::Deserialize)]",
    );
    builder
        .prost_config
        .protoc_arg("--experimental_allow_proto3_optional");

    builder
        .pbjson_prefixes([
            ".models.controller",
            ".models.common",
            ".models.translation",
            ".services",
        ])
        .tonic(
            codegen_proto::tonic_prost_build::configure()
                .server_mod_attribute("services.controller", r#"#[cfg(feature = "server")]"#)
                .server_mod_attribute("services.translation", r#"#[cfg(feature = "server")]"#),
        )
        .license_header(include_str!("./license_header"))
        .build();
}
