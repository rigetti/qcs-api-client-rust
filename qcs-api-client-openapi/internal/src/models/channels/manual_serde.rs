//! Manual `serde` implementations for [`Channels`].
//!
//! The derived implementation of `Deserialize` introduce failures into
//! how channels are parsed, as it removes the `_type` tag field that the inner
//! types require.
//!
//! This code is copied from the derived implementation using `cargo-expand`, with minor
//! edits to fix the behavior. No effort has been made to simplify or clean up the
//! implementation.
//!
//! # Warning
//!
//! The copied code uses hidden public items from the `serde` crate -- types that are
//! not intended to be used outside of the derived code and are not subject to semver
//! guarantees. That means any update of `serde` _could_ break this code, though it
//! seems unlikely.

use super::Channels;
use serde as _serde;

impl<'de> _serde::Deserialize<'de> for Channels {
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        enum __Field {
            __field0,
            __field1,
            __field2,
            __field3,
            __field4,
            __field5,
            __field6,
            __field7,
            __field8,
            __field9,
            __field10,
            __field11,
            __field12,
            __field13,
            __field14,
            __field15,
            __field16,
            __field17,
            __field18,
            __field19,
        }
        struct __FieldVisitor;
        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
            type Value = __Field;
            fn expecting(
                &self,
                __formatter: &mut _serde::__private::Formatter,
            ) -> _serde::__private::fmt::Result {
                _serde::__private::Formatter::write_str(__formatter, "variant identifier")
            }
            fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
            where
                __E: _serde::de::Error,
            {
                match __value {
                    0u64 => _serde::__private::Ok(__Field::__field0),
                    1u64 => _serde::__private::Ok(__Field::__field1),
                    2u64 => _serde::__private::Ok(__Field::__field2),
                    3u64 => _serde::__private::Ok(__Field::__field3),
                    4u64 => _serde::__private::Ok(__Field::__field4),
                    5u64 => _serde::__private::Ok(__Field::__field5),
                    6u64 => _serde::__private::Ok(__Field::__field6),
                    7u64 => _serde::__private::Ok(__Field::__field7),
                    8u64 => _serde::__private::Ok(__Field::__field8),
                    9u64 => _serde::__private::Ok(__Field::__field9),
                    10u64 => _serde::__private::Ok(__Field::__field10),
                    11u64 => _serde::__private::Ok(__Field::__field11),
                    12u64 => _serde::__private::Ok(__Field::__field12),
                    13u64 => _serde::__private::Ok(__Field::__field13),
                    14u64 => _serde::__private::Ok(__Field::__field14),
                    15u64 => _serde::__private::Ok(__Field::__field15),
                    16u64 => _serde::__private::Ok(__Field::__field16),
                    17u64 => _serde::__private::Ok(__Field::__field17),
                    18u64 => _serde::__private::Ok(__Field::__field18),
                    _ => _serde::__private::Ok(__Field::__field19),
                }
            }
            fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
            where
                __E: _serde::de::Error,
            {
                match __value {
                    "CWChannel" => _serde::__private::Ok(__Field::__field0),
                    "QDOFastFluxChannel" => _serde::__private::Ok(__Field::__field1),
                    "QDOSlowFluxChannel" => _serde::__private::Ok(__Field::__field2),
                    "QFDChannel" => _serde::__private::Ok(__Field::__field3),
                    "QGSChannel" => _serde::__private::Ok(__Field::__field4),
                    "QRRChannel" => _serde::__private::Ok(__Field::__field5),
                    "QRTChannel" => _serde::__private::Ok(__Field::__field6),
                    "YokogawaGS200Channel" => _serde::__private::Ok(__Field::__field7),
                    "LegacyUSRPSequencer" => _serde::__private::Ok(__Field::__field8),
                    "QFDSequencer" => _serde::__private::Ok(__Field::__field9),
                    "QFDx2Sequencer" => _serde::__private::Ok(__Field::__field10),
                    "QDOSequencer" => _serde::__private::Ok(__Field::__field11),
                    "QGSSequencer" => _serde::__private::Ok(__Field::__field12),
                    "QGSx2Sequencer" => _serde::__private::Ok(__Field::__field13),
                    "QRRSequencer" => _serde::__private::Ok(__Field::__field14),
                    "QRTSequencer" => _serde::__private::Ok(__Field::__field15),
                    "QRTx2Sequencer" => _serde::__private::Ok(__Field::__field16),
                    "USICardSequencer" => _serde::__private::Ok(__Field::__field17),
                    "USITargetSequencer" => _serde::__private::Ok(__Field::__field18),
                    _ => _serde::__private::Ok(__Field::__field19),
                }
            }
            fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
            where
                __E: _serde::de::Error,
            {
                match __value {
                    b"CWChannel" => _serde::__private::Ok(__Field::__field0),
                    b"QDOFastFluxChannel" => _serde::__private::Ok(__Field::__field1),
                    b"QDOSlowFluxChannel" => _serde::__private::Ok(__Field::__field2),
                    b"QFDChannel" => _serde::__private::Ok(__Field::__field3),
                    b"QGSChannel" => _serde::__private::Ok(__Field::__field4),
                    b"QRRChannel" => _serde::__private::Ok(__Field::__field5),
                    b"QRTChannel" => _serde::__private::Ok(__Field::__field6),
                    b"YokogawaGS200Channel" => _serde::__private::Ok(__Field::__field7),
                    b"LegacyUSRPSequencer" => _serde::__private::Ok(__Field::__field8),
                    b"QFDSequencer" => _serde::__private::Ok(__Field::__field9),
                    b"QFDx2Sequencer" => _serde::__private::Ok(__Field::__field10),
                    b"QDOSequencer" => _serde::__private::Ok(__Field::__field11),
                    b"QGSSequencer" => _serde::__private::Ok(__Field::__field12),
                    b"QGSx2Sequencer" => _serde::__private::Ok(__Field::__field13),
                    b"QRRSequencer" => _serde::__private::Ok(__Field::__field14),
                    b"QRTSequencer" => _serde::__private::Ok(__Field::__field15),
                    b"QRTx2Sequencer" => _serde::__private::Ok(__Field::__field16),
                    b"USICardSequencer" => _serde::__private::Ok(__Field::__field17),
                    b"USITargetSequencer" => _serde::__private::Ok(__Field::__field18),
                    _ => _serde::__private::Ok(__Field::__field19),
                }
            }
        }

        impl<'de> _serde::Deserialize<'de> for __Field {
            #[inline]
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
            }
        }

        let __content = _serde::__private::de::Content::<'de>::deserialize(__deserializer)?;
        let __deserializer =
            _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content.clone());

        let (__tag, __content) = match _serde::Deserializer::deserialize_any(
            __deserializer,
            _serde::__private::de::TaggedContentVisitor::<__Field>::new(
                "_type",
                "internally tagged enum Channels",
            ),
        ) {
            // __val.content has the __type field removed, but it needs to be present
            _serde::__private::Ok(__val) => (__val.tag, __content),
            _serde::__private::Err(__err) => (__Field::__field19, __content),
        };

        match __tag {
            __Field::__field0 => _serde::__private::Result::map(
                <crate::models::CwChannel as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::CwChannel,
            ),
            __Field::__field1 => _serde::__private::Result::map(
                <crate::models::QdoFastFluxChannel as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QdoFastFluxChannel,
            ),
            __Field::__field2 => _serde::__private::Result::map(
                <crate::models::QdoSlowFluxChannel as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QdoSlowFluxChannel,
            ),
            __Field::__field3 => _serde::__private::Result::map(
                <crate::models::QfdChannel as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QfdChannel,
            ),
            __Field::__field4 => _serde::__private::Result::map(
                <crate::models::QgsChannel as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QgsChannel,
            ),
            __Field::__field5 => _serde::__private::Result::map(
                <crate::models::QrrChannel as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QrrChannel,
            ),
            __Field::__field6 => _serde::__private::Result::map(
                <crate::models::QrtChannel as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QrtChannel,
            ),
            __Field::__field7 => _serde::__private::Result::map(
                <crate::models::YokogawaGs200Channel as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::YokogawaGs200Channel,
            ),
            __Field::__field8 => _serde::__private::Result::map(
                <crate::models::LegacyUsrpSequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::LegacyUsrpSequencer,
            ),
            __Field::__field9 => _serde::__private::Result::map(
                <crate::models::QfdSequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QfdSequencer,
            ),
            __Field::__field10 => _serde::__private::Result::map(
                <crate::models::Qfdx2Sequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::Qfdx2Sequencer,
            ),
            __Field::__field11 => _serde::__private::Result::map(
                <crate::models::QdoSequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QdoSequencer,
            ),
            __Field::__field12 => _serde::__private::Result::map(
                <crate::models::QgsSequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QgsSequencer,
            ),
            __Field::__field13 => _serde::__private::Result::map(
                <crate::models::Qgsx2Sequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::Qgsx2Sequencer,
            ),
            __Field::__field14 => _serde::__private::Result::map(
                <crate::models::QrrSequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QrrSequencer,
            ),
            __Field::__field15 => _serde::__private::Result::map(
                <crate::models::QrtSequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::QrtSequencer,
            ),
            __Field::__field16 => _serde::__private::Result::map(
                <crate::models::Qrtx2Sequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::Qrtx2Sequencer,
            ),
            __Field::__field17 => _serde::__private::Result::map(
                <crate::models::UsiCardSequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::UsiCardSequencer,
            ),
            __Field::__field18 => _serde::__private::Result::map(
                <crate::models::UsiTargetSequencer as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::UsiTargetSequencer,
            ),
            __Field::__field19 => _serde::__private::Result::map(
                <serde_json::Value as _serde::Deserialize>::deserialize(
                    _serde::__private::de::ContentDeserializer::<__D::Error>::new(__content),
                ),
                Channels::SerdeJsonValue,
            ),
        }
    }
}

impl _serde::Serialize for Channels {
    fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
    where
        __S: _serde::Serializer,
    {
        match *self {
            Channels::CwChannel(ref __field0) => _serde::__private::ser::serialize_tagged_newtype(
                __serializer,
                "Channels",
                "CwChannel",
                "_type",
                "CWChannel",
                __field0,
            ),
            Channels::QdoFastFluxChannel(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "QdoFastFluxChannel",
                    "_type",
                    "QDOFastFluxChannel",
                    __field0,
                )
            }
            Channels::QdoSlowFluxChannel(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "QdoSlowFluxChannel",
                    "_type",
                    "QDOSlowFluxChannel",
                    __field0,
                )
            }
            Channels::QfdChannel(ref __field0) => _serde::__private::ser::serialize_tagged_newtype(
                __serializer,
                "Channels",
                "QfdChannel",
                "_type",
                "QFDChannel",
                __field0,
            ),
            Channels::QgsChannel(ref __field0) => _serde::__private::ser::serialize_tagged_newtype(
                __serializer,
                "Channels",
                "QgsChannel",
                "_type",
                "QGSChannel",
                __field0,
            ),
            Channels::QrrChannel(ref __field0) => _serde::__private::ser::serialize_tagged_newtype(
                __serializer,
                "Channels",
                "QrrChannel",
                "_type",
                "QRRChannel",
                __field0,
            ),
            Channels::QrtChannel(ref __field0) => _serde::__private::ser::serialize_tagged_newtype(
                __serializer,
                "Channels",
                "QrtChannel",
                "_type",
                "QRTChannel",
                __field0,
            ),
            Channels::YokogawaGs200Channel(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "YokogawaGs200Channel",
                    "_type",
                    "YokogawaGS200Channel",
                    __field0,
                )
            }
            Channels::LegacyUsrpSequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "LegacyUsrpSequencer",
                    "_type",
                    "LegacyUSRPSequencer",
                    __field0,
                )
            }
            Channels::QfdSequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "QfdSequencer",
                    "_type",
                    "QFDSequencer",
                    __field0,
                )
            }
            Channels::Qfdx2Sequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "Qfdx2Sequencer",
                    "_type",
                    "QFDx2Sequencer",
                    __field0,
                )
            }
            Channels::QdoSequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "QdoSequencer",
                    "_type",
                    "QDOSequencer",
                    __field0,
                )
            }
            Channels::QgsSequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "QgsSequencer",
                    "_type",
                    "QGSSequencer",
                    __field0,
                )
            }
            Channels::Qgsx2Sequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "Qgsx2Sequencer",
                    "_type",
                    "QGSx2Sequencer",
                    __field0,
                )
            }
            Channels::QrrSequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "QrrSequencer",
                    "_type",
                    "QRRSequencer",
                    __field0,
                )
            }
            Channels::QrtSequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "QrtSequencer",
                    "_type",
                    "QRTSequencer",
                    __field0,
                )
            }
            Channels::Qrtx2Sequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "Qrtx2Sequencer",
                    "_type",
                    "QRTx2Sequencer",
                    __field0,
                )
            }
            Channels::UsiCardSequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "UsiCardSequencer",
                    "_type",
                    "USICardSequencer",
                    __field0,
                )
            }
            Channels::UsiTargetSequencer(ref __field0) => {
                _serde::__private::ser::serialize_tagged_newtype(
                    __serializer,
                    "Channels",
                    "UsiTargetSequencer",
                    "_type",
                    "USITargetSequencer",
                    __field0,
                )
            }
            Channels::SerdeJsonValue(ref __field0) => __field0.serialize(__serializer),
        }
    }
}
