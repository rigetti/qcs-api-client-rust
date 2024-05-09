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


// This file is @generated by prost-build.
/// Complex64 is a 64-bit complex value with float32 real and imaginary parts
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Complex64 {
    #[prost(float, tag = "1")]
    pub real: f32,
    #[prost(float, tag = "2")]
    pub imaginary: f32,
}
/// ReadoutValues are data readout values that have been read out from the quantum processor
/// and optionally processed by a readout transformation pipeline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadoutValues {
    #[prost(oneof = "readout_values::Values", tags = "1, 2")]
    pub values: ::core::option::Option<readout_values::Values>,
}
/// Nested message and enum types in `ReadoutValues`.
pub mod readout_values {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Values {
        #[prost(message, tag = "1")]
        IntegerValues(super::IntegerReadoutValues),
        #[prost(message, tag = "2")]
        ComplexValues(super::Complex64ReadoutValues),
    }
}
/// IntegerReadoutValues are integer arrays emitted by a readout receiver or transformation pipeline.
/// These may include (but are not limited to) qudit values or raw ADC capture data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegerReadoutValues {
    #[prost(int32, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<i32>,
}
/// Complex64ReadoutValues are arrays of complex numbers emitted by a readout receiver or transformation pipeline.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Complex64ReadoutValues {
    #[prost(message, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<Complex64>,
}
/// An EncryptedControllerJob includes the configuration necessary to execute an instance of
/// the contained job data on control hardware in encrypted format.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptedControllerJob {
    /// Encrypted form of ControllerJob.
    #[prost(bytes = "vec", tag = "1")]
    pub job: ::prost::alloc::vec::Vec<u8>,
    /// Information about the means by which `inner` was encrypted.
    #[prost(message, optional, tag = "2")]
    pub encryption: ::core::option::Option<JobEncryption>,
}
/// Information about the means by which a ControllerJob was encrypted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobEncryption {
    /// Opaque identifier for the key to use in decryption
    #[prost(string, tag = "1")]
    pub key_id: ::prost::alloc::string::String,
    /// If relevant, the nonce to use in decryption
    #[prost(bytes = "vec", tag = "2")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobExecutionConfiguration {
    /// Memory values to be patched into the program by the Controller Service prior to execution.
    /// The string key is used to match the name of the memory region as defined in the
    /// InstrumentProgram. The type of the DataValue must match the defined type of the region.
    #[prost(map = "string, message", tag = "3")]
    pub memory_values: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        DataValue,
    >,
}
/// The value of the data to insert into memory corresponding to a MemoryRegion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataValue {
    #[prost(oneof = "data_value::Value", tags = "101, 102, 103")]
    pub value: ::core::option::Option<data_value::Value>,
}
/// Nested message and enum types in `DataValue`.
pub mod data_value {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Binary value, corresponding to both BIT and OCTET data types in Quil.
        #[prost(message, tag = "101")]
        Binary(super::BinaryDataValue),
        /// Signed integer value, corresponding to INTEGER in Quil.
        #[prost(message, tag = "102")]
        Integer(super::IntegerDataValue),
        /// Real number value, corresponding to REAL in Quil.
        #[prost(message, tag = "103")]
        Real(super::RealDataValue),
    }
}
/// Binary value, corresponding to both BIT and OCTET data types in Quil.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryDataValue {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Signed integer value, corresponding to INTEGER in Quil.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegerDataValue {
    #[prost(int64, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<i64>,
}
/// Real number value, corresponding to REAL in Quil.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealDataValue {
    #[prost(double, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<f64>,
}
/// A ControllerJobExecutionResult includes the result data from a single
/// execution of a ControllerJob.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControllerJobExecutionResult {
    /// The contents of each memory region, keyed on region name
    #[prost(map = "string, message", tag = "1")]
    pub memory_values: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        DataValue,
    >,
    /// The contents of readout data published by the readout transformation
    /// pipeline, keyed on the node ID of the publishing readout transformation
    /// node.
    #[prost(map = "string, message", tag = "2")]
    pub readout_values: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ReadoutValues,
    >,
    #[prost(enumeration = "controller_job_execution_result::Status", tag = "3")]
    pub status: i32,
    /// Optional message providing context to the result's status.
    #[prost(string, optional, tag = "4")]
    pub status_message: ::core::option::Option<::prost::alloc::string::String>,
    /// Duration (µs) job held exclusive access to control hardware.
    #[prost(uint64, tag = "5")]
    pub execution_duration_microseconds: u64,
}
/// Nested message and enum types in `ControllerJobExecutionResult`.
pub mod controller_job_execution_result {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Status {
        Unknown = 0,
        Success = 1,
        /// Failure state caused by an error in the service.
        ServiceFailure = 2,
        /// Failure state caused by user.
        UserFailure = 3,
        /// Job was canceled by user before execution completed.
        UserCancellation = 4,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unknown => "UNKNOWN",
                Status::Success => "SUCCESS",
                Status::ServiceFailure => "SERVICE_FAILURE",
                Status::UserFailure => "USER_FAILURE",
                Status::UserCancellation => "USER_CANCELLATION",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SUCCESS" => Some(Self::Success),
                "SERVICE_FAILURE" => Some(Self::ServiceFailure),
                "USER_FAILURE" => Some(Self::UserFailure),
                "USER_CANCELLATION" => Some(Self::UserCancellation),
                _ => None,
            }
        }
    }
}

