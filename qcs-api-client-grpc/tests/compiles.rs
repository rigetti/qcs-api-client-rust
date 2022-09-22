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

use qcs_api_client_grpc::common::grpc::{get_channel, parse_uri, wrap_channel_with, Error};
use qcs_api_client_grpc::common::ClientConfiguration;
use qcs_api_client_grpc::services::translation::translation_client::TranslationClient;

async fn do_stuff() -> Result<(), Error> {
    let config = ClientConfiguration::load().await?;

    let service = wrap_channel_with(get_channel(parse_uri("")?), config.clone());

    let client = TranslationClient::new(service);

    Ok(())
}