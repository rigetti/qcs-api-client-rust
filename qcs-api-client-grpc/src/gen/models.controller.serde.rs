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


impl serde::Serialize for BinaryDataValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.BinaryDataValue", len)?;
        if !self.data.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("data", pbjson::private::base64::encode(&self.data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BinaryDataValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BinaryDataValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.BinaryDataValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BinaryDataValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BinaryDataValue {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.controller.BinaryDataValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Complex64 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.real != 0. {
            len += 1;
        }
        if self.imaginary != 0. {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.Complex64", len)?;
        if self.real != 0. {
            struct_ser.serialize_field("real", &self.real)?;
        }
        if self.imaginary != 0. {
            struct_ser.serialize_field("imaginary", &self.imaginary)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Complex64 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "real",
            "imaginary",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Real,
            Imaginary,
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
                            "real" => Ok(GeneratedField::Real),
                            "imaginary" => Ok(GeneratedField::Imaginary),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Complex64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.Complex64")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Complex64, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut real__ = None;
                let mut imaginary__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Real => {
                            if real__.is_some() {
                                return Err(serde::de::Error::duplicate_field("real"));
                            }
                            real__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Imaginary => {
                            if imaginary__.is_some() {
                                return Err(serde::de::Error::duplicate_field("imaginary"));
                            }
                            imaginary__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Complex64 {
                    real: real__.unwrap_or_default(),
                    imaginary: imaginary__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.controller.Complex64", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Complex64ReadoutValues {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.Complex64ReadoutValues", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Complex64ReadoutValues {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
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
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Complex64ReadoutValues;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.Complex64ReadoutValues")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Complex64ReadoutValues, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Complex64ReadoutValues {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.controller.Complex64ReadoutValues", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ControllerJobExecutionResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.memory_values.is_empty() {
            len += 1;
        }
        if !self.readout_values.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.status_message.is_some() {
            len += 1;
        }
        if self.execution_duration_microseconds != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.ControllerJobExecutionResult", len)?;
        if !self.memory_values.is_empty() {
            struct_ser.serialize_field("memoryValues", &self.memory_values)?;
        }
        if !self.readout_values.is_empty() {
            struct_ser.serialize_field("readoutValues", &self.readout_values)?;
        }
        if self.status != 0 {
            let v = controller_job_execution_result::Status::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.status_message.as_ref() {
            struct_ser.serialize_field("statusMessage", v)?;
        }
        if self.execution_duration_microseconds != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("executionDurationMicroseconds", ToString::to_string(&self.execution_duration_microseconds).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ControllerJobExecutionResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "memory_values",
            "memoryValues",
            "readout_values",
            "readoutValues",
            "status",
            "status_message",
            "statusMessage",
            "execution_duration_microseconds",
            "executionDurationMicroseconds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MemoryValues,
            ReadoutValues,
            Status,
            StatusMessage,
            ExecutionDurationMicroseconds,
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
                            "memoryValues" | "memory_values" => Ok(GeneratedField::MemoryValues),
                            "readoutValues" | "readout_values" => Ok(GeneratedField::ReadoutValues),
                            "status" => Ok(GeneratedField::Status),
                            "statusMessage" | "status_message" => Ok(GeneratedField::StatusMessage),
                            "executionDurationMicroseconds" | "execution_duration_microseconds" => Ok(GeneratedField::ExecutionDurationMicroseconds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ControllerJobExecutionResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.ControllerJobExecutionResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ControllerJobExecutionResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut memory_values__ = None;
                let mut readout_values__ = None;
                let mut status__ = None;
                let mut status_message__ = None;
                let mut execution_duration_microseconds__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MemoryValues => {
                            if memory_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryValues"));
                            }
                            memory_values__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::ReadoutValues => {
                            if readout_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readoutValues"));
                            }
                            readout_values__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<controller_job_execution_result::Status>()? as i32);
                        }
                        GeneratedField::StatusMessage => {
                            if status_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusMessage"));
                            }
                            status_message__ = map_.next_value()?;
                        }
                        GeneratedField::ExecutionDurationMicroseconds => {
                            if execution_duration_microseconds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionDurationMicroseconds"));
                            }
                            execution_duration_microseconds__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ControllerJobExecutionResult {
                    memory_values: memory_values__.unwrap_or_default(),
                    readout_values: readout_values__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    status_message: status_message__,
                    execution_duration_microseconds: execution_duration_microseconds__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.controller.ControllerJobExecutionResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for controller_job_execution_result::Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Success => "SUCCESS",
            Self::ServiceFailure => "SERVICE_FAILURE",
            Self::UserFailure => "USER_FAILURE",
            Self::UserCancellation => "USER_CANCELLATION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for controller_job_execution_result::Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "SUCCESS",
            "SERVICE_FAILURE",
            "USER_FAILURE",
            "USER_CANCELLATION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = controller_job_execution_result::Status;

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
                    "UNKNOWN" => Ok(controller_job_execution_result::Status::Unknown),
                    "SUCCESS" => Ok(controller_job_execution_result::Status::Success),
                    "SERVICE_FAILURE" => Ok(controller_job_execution_result::Status::ServiceFailure),
                    "USER_FAILURE" => Ok(controller_job_execution_result::Status::UserFailure),
                    "USER_CANCELLATION" => Ok(controller_job_execution_result::Status::UserCancellation),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for DataValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.DataValue", len)?;
        if let Some(v) = self.value.as_ref() {
            match v {
                data_value::Value::Binary(v) => {
                    struct_ser.serialize_field("binary", v)?;
                }
                data_value::Value::Integer(v) => {
                    struct_ser.serialize_field("integer", v)?;
                }
                data_value::Value::Real(v) => {
                    struct_ser.serialize_field("real", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "binary",
            "integer",
            "real",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Binary,
            Integer,
            Real,
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
                            "binary" => Ok(GeneratedField::Binary),
                            "integer" => Ok(GeneratedField::Integer),
                            "real" => Ok(GeneratedField::Real),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DataValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.DataValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DataValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Binary => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("binary"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(data_value::Value::Binary)
;
                        }
                        GeneratedField::Integer => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integer"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(data_value::Value::Integer)
;
                        }
                        GeneratedField::Real => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("real"));
                            }
                            value__ = map_.next_value::<::std::option::Option<_>>()?.map(data_value::Value::Real)
;
                        }
                    }
                }
                Ok(DataValue {
                    value: value__,
                })
            }
        }
        deserializer.deserialize_struct("models.controller.DataValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EncryptedControllerJob {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.job.is_empty() {
            len += 1;
        }
        if self.encryption.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.EncryptedControllerJob", len)?;
        if !self.job.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("job", pbjson::private::base64::encode(&self.job).as_str())?;
        }
        if let Some(v) = self.encryption.as_ref() {
            struct_ser.serialize_field("encryption", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for EncryptedControllerJob {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "job",
            "encryption",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Job,
            Encryption,
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
                            "encryption" => Ok(GeneratedField::Encryption),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EncryptedControllerJob;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.EncryptedControllerJob")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<EncryptedControllerJob, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut job__ = None;
                let mut encryption__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Job => {
                            if job__.is_some() {
                                return Err(serde::de::Error::duplicate_field("job"));
                            }
                            job__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Encryption => {
                            if encryption__.is_some() {
                                return Err(serde::de::Error::duplicate_field("encryption"));
                            }
                            encryption__ = map_.next_value()?;
                        }
                    }
                }
                Ok(EncryptedControllerJob {
                    job: job__.unwrap_or_default(),
                    encryption: encryption__,
                })
            }
        }
        deserializer.deserialize_struct("models.controller.EncryptedControllerJob", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IntegerDataValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.IntegerDataValue", len)?;
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", &self.data.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntegerDataValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntegerDataValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.IntegerDataValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntegerDataValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(IntegerDataValue {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.controller.IntegerDataValue", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IntegerReadoutValues {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.IntegerReadoutValues", len)?;
        if !self.values.is_empty() {
            struct_ser.serialize_field("values", &self.values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IntegerReadoutValues {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "values",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Values,
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
                            "values" => Ok(GeneratedField::Values),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IntegerReadoutValues;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.IntegerReadoutValues")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<IntegerReadoutValues, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Values => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            values__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(IntegerReadoutValues {
                    values: values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.controller.IntegerReadoutValues", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JobEncryption {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key_id.is_empty() {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.JobEncryption", len)?;
        if !self.key_id.is_empty() {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if !self.nonce.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", pbjson::private::base64::encode(&self.nonce).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JobEncryption {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key_id",
            "keyId",
            "nonce",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyId,
            Nonce,
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
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            "nonce" => Ok(GeneratedField::Nonce),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobEncryption;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.JobEncryption")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JobEncryption, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_id__ = None;
                let mut nonce__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(JobEncryption {
                    key_id: key_id__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.controller.JobEncryption", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JobExecutionConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.memory_values.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.JobExecutionConfiguration", len)?;
        if !self.memory_values.is_empty() {
            struct_ser.serialize_field("memoryValues", &self.memory_values)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JobExecutionConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "memory_values",
            "memoryValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MemoryValues,
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
                            "memoryValues" | "memory_values" => Ok(GeneratedField::MemoryValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobExecutionConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.JobExecutionConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JobExecutionConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut memory_values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MemoryValues => {
                            if memory_values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryValues"));
                            }
                            memory_values__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(JobExecutionConfiguration {
                    memory_values: memory_values__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.controller.JobExecutionConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReadoutValues {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.values.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.ReadoutValues", len)?;
        if let Some(v) = self.values.as_ref() {
            match v {
                readout_values::Values::IntegerValues(v) => {
                    struct_ser.serialize_field("integerValues", v)?;
                }
                readout_values::Values::ComplexValues(v) => {
                    struct_ser.serialize_field("complexValues", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReadoutValues {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "integer_values",
            "integerValues",
            "complex_values",
            "complexValues",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IntegerValues,
            ComplexValues,
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
                            "integerValues" | "integer_values" => Ok(GeneratedField::IntegerValues),
                            "complexValues" | "complex_values" => Ok(GeneratedField::ComplexValues),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReadoutValues;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.ReadoutValues")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReadoutValues, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut values__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::IntegerValues => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("integerValues"));
                            }
                            values__ = map_.next_value::<::std::option::Option<_>>()?.map(readout_values::Values::IntegerValues)
;
                        }
                        GeneratedField::ComplexValues => {
                            if values__.is_some() {
                                return Err(serde::de::Error::duplicate_field("complexValues"));
                            }
                            values__ = map_.next_value::<::std::option::Option<_>>()?.map(readout_values::Values::ComplexValues)
;
                        }
                    }
                }
                Ok(ReadoutValues {
                    values: values__,
                })
            }
        }
        deserializer.deserialize_struct("models.controller.ReadoutValues", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RealDataValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.controller.RealDataValue", len)?;
        if !self.data.is_empty() {
            struct_ser.serialize_field("data", &self.data)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RealDataValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "data",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Data,
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
                            "data" => Ok(GeneratedField::Data),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RealDataValue;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.controller.RealDataValue")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RealDataValue, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut data__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Data => {
                            if data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("data"));
                            }
                            data__ = 
                                Some(map_.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(RealDataValue {
                    data: data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.controller.RealDataValue", FIELDS, GeneratedVisitor)
    }
}

