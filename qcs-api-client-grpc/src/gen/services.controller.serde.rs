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


impl serde::Serialize for BatchExecuteControllerJobsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requests.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.BatchExecuteControllerJobsRequest", len)?;
        if !self.requests.is_empty() {
            struct_ser.serialize_field("requests", &self.requests)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchExecuteControllerJobsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requests",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requests,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "requests" => Ok(GeneratedField::Requests),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchExecuteControllerJobsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.BatchExecuteControllerJobsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchExecuteControllerJobsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requests__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requests => {
                            if requests__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requests"));
                            }
                            requests__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchExecuteControllerJobsRequest {
                    requests: requests__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("services.controller.BatchExecuteControllerJobsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BatchExecuteControllerJobsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.responses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.BatchExecuteControllerJobsResponse", len)?;
        if !self.responses.is_empty() {
            struct_ser.serialize_field("responses", &self.responses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BatchExecuteControllerJobsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "responses",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Responses,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "responses" => Ok(GeneratedField::Responses),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BatchExecuteControllerJobsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.BatchExecuteControllerJobsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BatchExecuteControllerJobsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut responses__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Responses => {
                            if responses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("responses"));
                            }
                            responses__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BatchExecuteControllerJobsResponse {
                    responses: responses__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("services.controller.BatchExecuteControllerJobsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelControllerJobsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.job_ids.is_empty() {
            len += 1;
        }
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.CancelControllerJobsRequest", len)?;
        if !self.job_ids.is_empty() {
            struct_ser.serialize_field("jobIds", &self.job_ids)?;
        }
        if let Some(v) = self.target.as_ref() {
            match v {
                cancel_controller_jobs_request::Target::QuantumProcessorId(v) => {
                    struct_ser.serialize_field("quantumProcessorId", v)?;
                }
                cancel_controller_jobs_request::Target::EndpointId(v) => {
                    struct_ser.serialize_field("endpointId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelControllerJobsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job_ids",
            "jobIds",
            "quantum_processor_id",
            "quantumProcessorId",
            "endpoint_id",
            "endpointId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JobIds,
            QuantumProcessorId,
            EndpointId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "jobIds" | "job_ids" => Ok(GeneratedField::JobIds),
                            "quantumProcessorId" | "quantum_processor_id" => Ok(GeneratedField::QuantumProcessorId),
                            "endpointId" | "endpoint_id" => Ok(GeneratedField::EndpointId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelControllerJobsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.CancelControllerJobsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelControllerJobsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job_ids__ = None;
                let mut target__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JobIds => {
                            if job_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobIds"));
                            }
                            job_ids__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuantumProcessorId => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantumProcessorId"));
                            }
                            target__ = map_.next_value::<::std::option::Option<_>>()?.map(cancel_controller_jobs_request::Target::QuantumProcessorId);
                        }
                        GeneratedField::EndpointId => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointId"));
                            }
                            target__ = map_.next_value::<::std::option::Option<_>>()?.map(cancel_controller_jobs_request::Target::EndpointId);
                        }
                    }
                }
                Ok(CancelControllerJobsRequest {
                    job_ids: job_ids__.unwrap_or_default(),
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct("services.controller.CancelControllerJobsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CancelControllerJobsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("services.controller.CancelControllerJobsResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelControllerJobsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CancelControllerJobsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.CancelControllerJobsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelControllerJobsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CancelControllerJobsResponse {
                })
            }
        }
        deserializer.deserialize_struct("services.controller.CancelControllerJobsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EstimatedDelay {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.minimum.is_some() {
            len += 1;
        }
        if self.maximum.is_some() {
            len += 1;
        }
        if self.now.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.EstimatedDelay", len)?;
        if let Some(v) = self.minimum.as_ref() {
            struct_ser.serialize_field("minimum", v)?;
        }
        if let Some(v) = self.maximum.as_ref() {
            struct_ser.serialize_field("maximum", v)?;
        }
        if let Some(v) = self.now.as_ref() {
            struct_ser.serialize_field("now", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EstimatedDelay {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "minimum",
            "maximum",
            "now",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Minimum,
            Maximum,
            Now,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "minimum" => Ok(GeneratedField::Minimum),
                            "maximum" => Ok(GeneratedField::Maximum),
                            "now" => Ok(GeneratedField::Now),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EstimatedDelay;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.EstimatedDelay")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EstimatedDelay, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut minimum__ = None;
                let mut maximum__ = None;
                let mut now__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Minimum => {
                            if minimum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minimum"));
                            }
                            minimum__ = map_.next_value()?;
                        }
                        GeneratedField::Maximum => {
                            if maximum__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maximum"));
                            }
                            maximum__ = map_.next_value()?;
                        }
                        GeneratedField::Now => {
                            if now__.is_some() {
                                return Err(serde::de::Error::duplicate_field("now"));
                            }
                            now__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EstimatedDelay {
                    minimum: minimum__,
                    maximum: maximum__,
                    now: now__,
                })
            }
        }
        deserializer.deserialize_struct("services.controller.EstimatedDelay", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteControllerJobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.execution_configurations.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if self.job.is_some() {
            len += 1;
        }
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.ExecuteControllerJobRequest", len)?;
        if !self.execution_configurations.is_empty() {
            struct_ser.serialize_field("executionConfigurations", &self.execution_configurations)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
        }
        if let Some(v) = self.job.as_ref() {
            match v {
                execute_controller_job_request::Job::Encrypted(v) => {
                    struct_ser.serialize_field("encrypted", v)?;
                }
            }
        }
        if let Some(v) = self.target.as_ref() {
            match v {
                execute_controller_job_request::Target::QuantumProcessorId(v) => {
                    struct_ser.serialize_field("quantumProcessorId", v)?;
                }
                execute_controller_job_request::Target::EndpointId(v) => {
                    struct_ser.serialize_field("endpointId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteControllerJobRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "execution_configurations",
            "executionConfigurations",
            "options",
            "encrypted",
            "quantum_processor_id",
            "quantumProcessorId",
            "endpoint_id",
            "endpointId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExecutionConfigurations,
            Options,
            Encrypted,
            QuantumProcessorId,
            EndpointId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "executionConfigurations" | "execution_configurations" => Ok(GeneratedField::ExecutionConfigurations),
                            "options" => Ok(GeneratedField::Options),
                            "encrypted" => Ok(GeneratedField::Encrypted),
                            "quantumProcessorId" | "quantum_processor_id" => Ok(GeneratedField::QuantumProcessorId),
                            "endpointId" | "endpoint_id" => Ok(GeneratedField::EndpointId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteControllerJobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.ExecuteControllerJobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteControllerJobRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut execution_configurations__ = None;
                let mut options__ = None;
                let mut job__ = None;
                let mut target__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExecutionConfigurations => {
                            if execution_configurations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionConfigurations"));
                            }
                            execution_configurations__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map_.next_value()?;
                        }
                        GeneratedField::Encrypted => {
                            if job__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encrypted"));
                            }
                            job__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_controller_job_request::Job::Encrypted)
;
                        }
                        GeneratedField::QuantumProcessorId => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantumProcessorId"));
                            }
                            target__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_controller_job_request::Target::QuantumProcessorId);
                        }
                        GeneratedField::EndpointId => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointId"));
                            }
                            target__ = map_.next_value::<::std::option::Option<_>>()?.map(execute_controller_job_request::Target::EndpointId);
                        }
                    }
                }
                Ok(ExecuteControllerJobRequest {
                    execution_configurations: execution_configurations__.unwrap_or_default(),
                    options: options__,
                    job: job__,
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct("services.controller.ExecuteControllerJobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteControllerJobResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.job_execution_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.ExecuteControllerJobResponse", len)?;
        if !self.job_execution_ids.is_empty() {
            struct_ser.serialize_field("jobExecutionIds", &self.job_execution_ids)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteControllerJobResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job_execution_ids",
            "jobExecutionIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JobExecutionIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "jobExecutionIds" | "job_execution_ids" => Ok(GeneratedField::JobExecutionIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteControllerJobResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.ExecuteControllerJobResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecuteControllerJobResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job_execution_ids__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JobExecutionIds => {
                            if job_execution_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobExecutionIds"));
                            }
                            job_execution_ids__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ExecuteControllerJobResponse {
                    job_execution_ids: job_execution_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("services.controller.ExecuteControllerJobResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bypass_settings_protection {
            len += 1;
        }
        if self.timeout.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.ExecutionOptions", len)?;
        if self.bypass_settings_protection {
            struct_ser.serialize_field("bypassSettingsProtection", &self.bypass_settings_protection)?;
        }
        if let Some(v) = self.timeout.as_ref() {
            struct_ser.serialize_field("timeout", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bypass_settings_protection",
            "bypassSettingsProtection",
            "timeout",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BypassSettingsProtection,
            Timeout,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "bypassSettingsProtection" | "bypass_settings_protection" => Ok(GeneratedField::BypassSettingsProtection),
                            "timeout" => Ok(GeneratedField::Timeout),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.ExecutionOptions")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecutionOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bypass_settings_protection__ = None;
                let mut timeout__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BypassSettingsProtection => {
                            if bypass_settings_protection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bypassSettingsProtection"));
                            }
                            bypass_settings_protection__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Timeout => {
                            if timeout__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timeout"));
                            }
                            timeout__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ExecutionOptions {
                    bypass_settings_protection: bypass_settings_protection__.unwrap_or_default(),
                    timeout: timeout__,
                })
            }
        }
        deserializer.deserialize_struct("services.controller.ExecutionOptions", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetControllerJobResultsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.job_execution_id.is_empty() {
            len += 1;
        }
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.GetControllerJobResultsRequest", len)?;
        if !self.job_execution_id.is_empty() {
            struct_ser.serialize_field("jobExecutionId", &self.job_execution_id)?;
        }
        if let Some(v) = self.target.as_ref() {
            match v {
                get_controller_job_results_request::Target::QuantumProcessorId(v) => {
                    struct_ser.serialize_field("quantumProcessorId", v)?;
                }
                get_controller_job_results_request::Target::EndpointId(v) => {
                    struct_ser.serialize_field("endpointId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetControllerJobResultsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job_execution_id",
            "jobExecutionId",
            "quantum_processor_id",
            "quantumProcessorId",
            "endpoint_id",
            "endpointId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JobExecutionId,
            QuantumProcessorId,
            EndpointId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "jobExecutionId" | "job_execution_id" => Ok(GeneratedField::JobExecutionId),
                            "quantumProcessorId" | "quantum_processor_id" => Ok(GeneratedField::QuantumProcessorId),
                            "endpointId" | "endpoint_id" => Ok(GeneratedField::EndpointId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetControllerJobResultsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.GetControllerJobResultsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetControllerJobResultsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job_execution_id__ = None;
                let mut target__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JobExecutionId => {
                            if job_execution_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobExecutionId"));
                            }
                            job_execution_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuantumProcessorId => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantumProcessorId"));
                            }
                            target__ = map_.next_value::<::std::option::Option<_>>()?.map(get_controller_job_results_request::Target::QuantumProcessorId);
                        }
                        GeneratedField::EndpointId => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointId"));
                            }
                            target__ = map_.next_value::<::std::option::Option<_>>()?.map(get_controller_job_results_request::Target::EndpointId);
                        }
                    }
                }
                Ok(GetControllerJobResultsRequest {
                    job_execution_id: job_execution_id__.unwrap_or_default(),
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct("services.controller.GetControllerJobResultsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetControllerJobResultsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.GetControllerJobResultsResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetControllerJobResultsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetControllerJobResultsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.GetControllerJobResultsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetControllerJobResultsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetControllerJobResultsResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("services.controller.GetControllerJobResultsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetControllerJobStatusRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.job_id.is_empty() {
            len += 1;
        }
        if self.target.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.GetControllerJobStatusRequest", len)?;
        if !self.job_id.is_empty() {
            struct_ser.serialize_field("jobId", &self.job_id)?;
        }
        if let Some(v) = self.target.as_ref() {
            match v {
                get_controller_job_status_request::Target::QuantumProcessorId(v) => {
                    struct_ser.serialize_field("quantumProcessorId", v)?;
                }
                get_controller_job_status_request::Target::EndpointId(v) => {
                    struct_ser.serialize_field("endpointId", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetControllerJobStatusRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job_id",
            "jobId",
            "quantum_processor_id",
            "quantumProcessorId",
            "endpoint_id",
            "endpointId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            JobId,
            QuantumProcessorId,
            EndpointId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "jobId" | "job_id" => Ok(GeneratedField::JobId),
                            "quantumProcessorId" | "quantum_processor_id" => Ok(GeneratedField::QuantumProcessorId),
                            "endpointId" | "endpoint_id" => Ok(GeneratedField::EndpointId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetControllerJobStatusRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.GetControllerJobStatusRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetControllerJobStatusRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job_id__ = None;
                let mut target__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::JobId => {
                            if job_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jobId"));
                            }
                            job_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::QuantumProcessorId => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantumProcessorId"));
                            }
                            target__ = map_.next_value::<::std::option::Option<_>>()?.map(get_controller_job_status_request::Target::QuantumProcessorId);
                        }
                        GeneratedField::EndpointId => {
                            if target__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endpointId"));
                            }
                            target__ = map_.next_value::<::std::option::Option<_>>()?.map(get_controller_job_status_request::Target::EndpointId);
                        }
                    }
                }
                Ok(GetControllerJobStatusRequest {
                    job_id: job_id__.unwrap_or_default(),
                    target: target__,
                })
            }
        }
        deserializer.deserialize_struct("services.controller.GetControllerJobStatusRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetControllerJobStatusResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status != 0 {
            len += 1;
        }
        if self.estimated_job_completion_delay.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.controller.GetControllerJobStatusResponse", len)?;
        if self.status != 0 {
            let v = get_controller_job_status_response::Status::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.estimated_job_completion_delay.as_ref() {
            struct_ser.serialize_field("estimatedJobCompletionDelay", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetControllerJobStatusResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status",
            "estimated_job_completion_delay",
            "estimatedJobCompletionDelay",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Status,
            EstimatedJobCompletionDelay,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "status" => Ok(GeneratedField::Status),
                            "estimatedJobCompletionDelay" | "estimated_job_completion_delay" => Ok(GeneratedField::EstimatedJobCompletionDelay),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetControllerJobStatusResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.controller.GetControllerJobStatusResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetControllerJobStatusResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status__ = None;
                let mut estimated_job_completion_delay__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<get_controller_job_status_response::Status>()? as i32);
                        }
                        GeneratedField::EstimatedJobCompletionDelay => {
                            if estimated_job_completion_delay__.is_some() {
                                return Err(serde::de::Error::duplicate_field("estimatedJobCompletionDelay"));
                            }
                            estimated_job_completion_delay__ = map_.next_value()?;
                        }
                    }
                }
                Ok(GetControllerJobStatusResponse {
                    status: status__.unwrap_or_default(),
                    estimated_job_completion_delay: estimated_job_completion_delay__,
                })
            }
        }
        deserializer.deserialize_struct("services.controller.GetControllerJobStatusResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_controller_job_status_response::Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Queued => "QUEUED",
            Self::Running => "RUNNING",
            Self::Succeeded => "SUCCEEDED",
            Self::Failed => "FAILED",
            Self::Canceled => "CANCELED",
            Self::PostProcessing => "POST_PROCESSING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for get_controller_job_status_response::Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "QUEUED",
            "RUNNING",
            "SUCCEEDED",
            "FAILED",
            "CANCELED",
            "POST_PROCESSING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_controller_job_status_response::Status;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(get_controller_job_status_response::Status::Unknown),
                    "QUEUED" => Ok(get_controller_job_status_response::Status::Queued),
                    "RUNNING" => Ok(get_controller_job_status_response::Status::Running),
                    "SUCCEEDED" => Ok(get_controller_job_status_response::Status::Succeeded),
                    "FAILED" => Ok(get_controller_job_status_response::Status::Failed),
                    "CANCELED" => Ok(get_controller_job_status_response::Status::Canceled),
                    "POST_PROCESSING" => Ok(get_controller_job_status_response::Status::PostProcessing),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}

