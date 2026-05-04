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


impl serde::Serialize for QuilTranslationMetadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.readout_mappings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("models.translation.QuilTranslationMetadata", len)?;
        if !self.readout_mappings.is_empty() {
            struct_ser.serialize_field("readoutMappings", &self.readout_mappings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuilTranslationMetadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "readout_mappings",
            "readoutMappings",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ReadoutMappings,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl serde::de::Visitor<'_> for GeneratedVisitor {
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
                            "readoutMappings" | "readout_mappings" => Ok(GeneratedField::ReadoutMappings),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuilTranslationMetadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct models.translation.QuilTranslationMetadata")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuilTranslationMetadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut readout_mappings__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ReadoutMappings => {
                            if readout_mappings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("readoutMappings"));
                            }
                            readout_mappings__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(QuilTranslationMetadata {
                    readout_mappings: readout_mappings__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("models.translation.QuilTranslationMetadata", FIELDS, GeneratedVisitor)
    }
}

