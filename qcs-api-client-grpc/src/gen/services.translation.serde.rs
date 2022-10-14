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


impl serde::Serialize for TranslateQuilToEncryptedControllerJobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.quantum_processor_id.is_some() {
            len += 1;
        }
        if self.quil_program.is_some() {
            len += 1;
        }
        if self.num_shots.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.translation.TranslateQuilToEncryptedControllerJobRequest", len)?;
        if let Some(v) = self.quantum_processor_id.as_ref() {
            struct_ser.serialize_field("quantumProcessorId", v)?;
        }
        if let Some(v) = self.quil_program.as_ref() {
            struct_ser.serialize_field("quilProgram", v)?;
        }
        if let Some(v) = self.num_shots.as_ref() {
            match v {
                translate_quil_to_encrypted_controller_job_request::NumShots::NumShotsValue(v) => {
                    struct_ser.serialize_field("numShotsValue", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranslateQuilToEncryptedControllerJobRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "quantumProcessorId",
            "quilProgram",
            "numShotsValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QuantumProcessorId,
            QuilProgram,
            NumShotsValue,
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
                            "quantumProcessorId" => Ok(GeneratedField::QuantumProcessorId),
                            "quilProgram" => Ok(GeneratedField::QuilProgram),
                            "numShotsValue" => Ok(GeneratedField::NumShotsValue),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranslateQuilToEncryptedControllerJobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.translation.TranslateQuilToEncryptedControllerJobRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TranslateQuilToEncryptedControllerJobRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut quantum_processor_id__ = None;
                let mut quil_program__ = None;
                let mut num_shots__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::QuantumProcessorId => {
                            if quantum_processor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quantumProcessorId"));
                            }
                            quantum_processor_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::QuilProgram => {
                            if quil_program__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quilProgram"));
                            }
                            quil_program__ = Some(map.next_value()?);
                        }
                        GeneratedField::NumShotsValue => {
                            if num_shots__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numShotsValue"));
                            }
                            num_shots__ = Some(translate_quil_to_encrypted_controller_job_request::NumShots::NumShotsValue(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            ));
                        }
                    }
                }
                Ok(TranslateQuilToEncryptedControllerJobRequest {
                    quantum_processor_id: quantum_processor_id__,
                    quil_program: quil_program__,
                    num_shots: num_shots__,
                })
            }
        }
        deserializer.deserialize_struct("services.translation.TranslateQuilToEncryptedControllerJobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranslateQuilToEncryptedControllerJobResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.job.is_some() {
            len += 1;
        }
        if self.metadata.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.translation.TranslateQuilToEncryptedControllerJobResponse", len)?;
        if let Some(v) = self.job.as_ref() {
            struct_ser.serialize_field("job", v)?;
        }
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranslateQuilToEncryptedControllerJobResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job",
            "metadata",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Job,
            Metadata,
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
                            "job" => Ok(GeneratedField::Job),
                            "metadata" => Ok(GeneratedField::Metadata),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranslateQuilToEncryptedControllerJobResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.translation.TranslateQuilToEncryptedControllerJobResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TranslateQuilToEncryptedControllerJobResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job__ = None;
                let mut metadata__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Job => {
                            if job__.is_some() {
                                return Err(serde::de::Error::duplicate_field("job"));
                            }
                            job__ = Some(map.next_value()?);
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TranslateQuilToEncryptedControllerJobResponse {
                    job: job__,
                    metadata: metadata__,
                })
            }
        }
        deserializer.deserialize_struct("services.translation.TranslateQuilToEncryptedControllerJobResponse", FIELDS, GeneratedVisitor)
    }
}

