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

use qcs_api_client_common::ClientConfiguration;
use qcs_api_client_grpc::channel::{
    get_channel, parse_uri, wrap_channel_with, wrap_channel_with_retry, Error,
};
use qcs_api_client_grpc::client_configuration::RefreshError;
use qcs_api_client_grpc::services::translation::translation_client::TranslationClient;

#[cfg(feature = "grpc-web")]
use qcs_api_client_grpc::channel::wrap_channel_with_grpc_web;

#[allow(dead_code)]
async fn build_client() -> Result<(), Error<RefreshError>> {
    let config = ClientConfiguration::load_default().await?;

    let service = wrap_channel_with_retry(wrap_channel_with(get_channel(parse_uri("")?)?, config));

    #[cfg(feature = "grpc-web")]
    let service = wrap_channel_with_grpc_web(service);

    let _client = TranslationClient::new(service);

    Ok(())
}
