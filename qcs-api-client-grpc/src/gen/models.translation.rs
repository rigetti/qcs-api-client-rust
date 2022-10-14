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


/// Information about the result of Quil translation that may be useful for the client,
/// but which is not needed for execution of the translated `ControllerJob`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuilTranslationMetadata {
    /// Mapping of (Quil memory address as string) to (readout stream)
    /// This allows a Quil program author to write and execute `MEASURE 0 ro`,
    /// while being able to interpret the readout results for one of the post-processed
    /// readout streams as representing the result of the `MEASURE`.
    #[prost(map="string, string", tag="1")]
    pub readout_mappings: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}

