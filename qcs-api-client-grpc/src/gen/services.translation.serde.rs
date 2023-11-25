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


impl serde::Serialize for BackendV1Options {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("services.translation.BackendV1Options", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BackendV1Options {
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
            type Value = BackendV1Options;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.translation.BackendV1Options")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BackendV1Options, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(BackendV1Options {
                })
            }
        }
        deserializer.deserialize_struct("services.translation.BackendV1Options", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BackendV2Options {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.prepend_default_calibrations.is_some() {
            len += 1;
        }
        if self.passive_reset_delay_seconds.is_some() {
            len += 1;
        }
        if self.allow_unchecked_pointer_arithmetic.is_some() {
            len += 1;
        }
        if self.allow_frame_redefinition.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.translation.BackendV2Options", len)?;
        if let Some(v) = self.prepend_default_calibrations.as_ref() {
            struct_ser.serialize_field("prependDefaultCalibrations", v)?;
        }
        if let Some(v) = self.passive_reset_delay_seconds.as_ref() {
            struct_ser.serialize_field("passiveResetDelaySeconds", v)?;
        }
        if let Some(v) = self.allow_unchecked_pointer_arithmetic.as_ref() {
            struct_ser.serialize_field("allowUncheckedPointerArithmetic", v)?;
        }
        if let Some(v) = self.allow_frame_redefinition.as_ref() {
            struct_ser.serialize_field("allowFrameRedefinition", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BackendV2Options {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "prepend_default_calibrations",
            "prependDefaultCalibrations",
            "passive_reset_delay_seconds",
            "passiveResetDelaySeconds",
            "allow_unchecked_pointer_arithmetic",
            "allowUncheckedPointerArithmetic",
            "allow_frame_redefinition",
            "allowFrameRedefinition",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrependDefaultCalibrations,
            PassiveResetDelaySeconds,
            AllowUncheckedPointerArithmetic,
            AllowFrameRedefinition,
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
                            "prependDefaultCalibrations" | "prepend_default_calibrations" => Ok(GeneratedField::PrependDefaultCalibrations),
                            "passiveResetDelaySeconds" | "passive_reset_delay_seconds" => Ok(GeneratedField::PassiveResetDelaySeconds),
                            "allowUncheckedPointerArithmetic" | "allow_unchecked_pointer_arithmetic" => Ok(GeneratedField::AllowUncheckedPointerArithmetic),
                            "allowFrameRedefinition" | "allow_frame_redefinition" => Ok(GeneratedField::AllowFrameRedefinition),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BackendV2Options;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.translation.BackendV2Options")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BackendV2Options, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut prepend_default_calibrations__ = None;
                let mut passive_reset_delay_seconds__ = None;
                let mut allow_unchecked_pointer_arithmetic__ = None;
                let mut allow_frame_redefinition__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PrependDefaultCalibrations => {
                            if prepend_default_calibrations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prependDefaultCalibrations"));
                            }
                            prepend_default_calibrations__ = map.next_value()?;
                        }
                        GeneratedField::PassiveResetDelaySeconds => {
                            if passive_reset_delay_seconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("passiveResetDelaySeconds"));
                            }
                            passive_reset_delay_seconds__ = 
                                map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::AllowUncheckedPointerArithmetic => {
                            if allow_unchecked_pointer_arithmetic__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowUncheckedPointerArithmetic"));
                            }
                            allow_unchecked_pointer_arithmetic__ = map.next_value()?;
                        }
                        GeneratedField::AllowFrameRedefinition => {
                            if allow_frame_redefinition__.is_some() {
                                return Err(serde::de::Error::duplicate_field("allowFrameRedefinition"));
                            }
                            allow_frame_redefinition__ = map.next_value()?;
                        }
                    }
                }
                Ok(BackendV2Options {
                    prepend_default_calibrations: prepend_default_calibrations__,
                    passive_reset_delay_seconds: passive_reset_delay_seconds__,
                    allow_unchecked_pointer_arithmetic: allow_unchecked_pointer_arithmetic__,
                    allow_frame_redefinition: allow_frame_redefinition__,
                })
            }
        }
        deserializer.deserialize_struct("services.translation.BackendV2Options", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TranslateQuilToEncryptedControllerJobRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.quantum_processor_id.is_empty() {
            len += 1;
        }
        if !self.quil_program.is_empty() {
            len += 1;
        }
        if self.options.is_some() {
            len += 1;
        }
        if self.num_shots.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.translation.TranslateQuilToEncryptedControllerJobRequest", len)?;
        if !self.quantum_processor_id.is_empty() {
            struct_ser.serialize_field("quantumProcessorId", &self.quantum_processor_id)?;
        }
        if !self.quil_program.is_empty() {
            struct_ser.serialize_field("quilProgram", &self.quil_program)?;
        }
        if let Some(v) = self.options.as_ref() {
            struct_ser.serialize_field("options", v)?;
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
            "quantum_processor_id",
            "quantumProcessorId",
            "quil_program",
            "quilProgram",
            "options",
            "num_shots_value",
            "numShotsValue",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QuantumProcessorId,
            QuilProgram,
            Options,
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
                            "quantumProcessorId" | "quantum_processor_id" => Ok(GeneratedField::QuantumProcessorId),
                            "quilProgram" | "quil_program" => Ok(GeneratedField::QuilProgram),
                            "options" => Ok(GeneratedField::Options),
                            "numShotsValue" | "num_shots_value" => Ok(GeneratedField::NumShotsValue),
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
                let mut options__ = None;
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
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = map.next_value()?;
                        }
                        GeneratedField::NumShotsValue => {
                            if num_shots__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numShotsValue"));
                            }
                            num_shots__ = map.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| translate_quil_to_encrypted_controller_job_request::NumShots::NumShotsValue(x.0));
                        }
                    }
                }
                Ok(TranslateQuilToEncryptedControllerJobRequest {
                    quantum_processor_id: quantum_processor_id__.unwrap_or_default(),
                    quil_program: quil_program__.unwrap_or_default(),
                    options: options__,
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
                            job__ = map.next_value()?;
                        }
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map.next_value()?;
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
impl serde::Serialize for TranslationOptions {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.translation_backend.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("services.translation.TranslationOptions", len)?;
        if let Some(v) = self.translation_backend.as_ref() {
            match v {
                translation_options::TranslationBackend::V1(v) => {
                    struct_ser.serialize_field("v1", v)?;
                }
                translation_options::TranslationBackend::V2(v) => {
                    struct_ser.serialize_field("v2", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TranslationOptions {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "v1",
            "v2",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            V1,
            V2,
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
                            "v1" => Ok(GeneratedField::V1),
                            "v2" => Ok(GeneratedField::V2),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TranslationOptions;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct services.translation.TranslationOptions")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TranslationOptions, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut translation_backend__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::V1 => {
                            if translation_backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v1"));
                            }
                            translation_backend__ = map.next_value::<::std::option::Option<_>>()?.map(translation_options::TranslationBackend::V1)
;
                        }
                        GeneratedField::V2 => {
                            if translation_backend__.is_some() {
                                return Err(serde::de::Error::duplicate_field("v2"));
                            }
                            translation_backend__ = map.next_value::<::std::option::Option<_>>()?.map(translation_options::TranslationBackend::V2)
;
                        }
                    }
                }
                Ok(TranslationOptions {
                    translation_backend: translation_backend__,
                })
            }
        }
        deserializer.deserialize_struct("services.translation.TranslationOptions", FIELDS, GeneratedVisitor)
    }
}

