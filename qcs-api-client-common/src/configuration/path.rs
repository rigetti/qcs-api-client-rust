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

use std::path::PathBuf;

use super::LoadError;

pub(crate) fn path_from_env_or_home(env: &str, file_name: &str) -> Result<PathBuf, LoadError> {
    match std::env::var(env) {
        Ok(path) => Ok(PathBuf::from(path)),
        Err(_) => dirs::home_dir()
            .map(|path| path.join(".qcs").join(file_name))
            .ok_or_else(|| LoadError::HomeDirError {
                env: env.to_string(),
            }),
    }
}
