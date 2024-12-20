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
/// Options specified on RPCs that translate Quil to a ControllerJob. Intended to support custom pre-processing
/// and other translation features.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TranslationOptions {
    /// When set, the client program will be pre-processed via the Q-CTRL API
    /// before translation.
    #[prost(message, optional, tag = "1")]
    pub q_ctrl: ::core::option::Option<translation_options::QCtrl>,
    /// The backend to use for translation, to include relevant options.
    /// If neither is specified, the implementing service may select the
    /// translation backend and options.
    #[prost(oneof = "translation_options::TranslationBackend", tags = "101, 102")]
    pub translation_backend: ::core::option::Option<
        translation_options::TranslationBackend,
    >,
}
/// Nested message and enum types in `TranslationOptions`.
pub mod translation_options {
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct QCtrl {
        /// Indicates whether Q-CTRL pre-processing should consider the set of
        /// program qubits to be fixed. If true, Q-CTRL may only remap qubits to
        /// others specified within the user-submitted program. If false, Q-CTRL may
        /// remap program qubits to any physical qubit on the device in order to
        /// optimize the program.
        ///
        /// Note, this attribute is used to set `fixed_layouts` attribute on Q-CTRL's
        /// `CompileOptions` message. Q-CTRL supports compilation of mutliple programs
        /// at once, while Rigetti's `TranslationOptions` are currently scoped to a
        /// single program. As such, we use `fixed_layout` rather than `fixed_layouts`.
        #[prost(bool, optional, tag = "1")]
        pub fixed_layout: ::core::option::Option<bool>,
    }
    /// The backend to use for translation, to include relevant options.
    /// If neither is specified, the implementing service may select the
    /// translation backend and options.
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum TranslationBackend {
        #[prost(message, tag = "101")]
        V1(super::BackendV1Options),
        #[prost(message, tag = "102")]
        V2(super::BackendV2Options),
    }
}
/// Options for translation backend V1
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BackendV1Options {}
/// Options for translation backend V2
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BackendV2Options {
    /// Whether to prepend the default calibrations for a particular QPU to the program.
    /// This may be set to false if you have prepended your own calibrations, or are submitting
    /// a pre-calibrated pulse-level program which does not need further expansion.
    #[prost(bool, optional, tag = "1")]
    pub prepend_default_calibrations: ::core::option::Option<bool>,
    /// The number of seconds to stall at the beginning of each num-shots loop iteration in order to allow adiabatic reset.
    #[prost(double, optional, tag = "2")]
    pub passive_reset_delay_seconds: ::core::option::Option<f64>,
    /// Whether to disable bounds checks on dynamic memory access. Only available to authorized users.
    #[prost(bool, optional, tag = "4")]
    pub allow_unchecked_pointer_arithmetic: ::core::option::Option<bool>,
    /// Whether to skip program frame validation against Rigetti calibrations.
    #[prost(bool, optional, tag = "5")]
    pub allow_frame_redefinition: ::core::option::Option<bool>,
    /// Whether to force all real-time-classified readout values to be stored in sequencer memory. If false or unset, only readout values that are
    /// read by the program are written to sequencer memory following readout.
    #[prost(bool, optional, tag = "6")]
    pub store_all_readout_values: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslateQuilToEncryptedControllerJobRequest {
    #[prost(string, tag = "1")]
    pub quantum_processor_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub quil_program: ::prost::alloc::string::String,
    /// Specification of optional translation features.
    #[prost(message, optional, tag = "4")]
    pub options: ::core::option::Option<TranslationOptions>,
    #[prost(
        oneof = "translate_quil_to_encrypted_controller_job_request::NumShots",
        tags = "3"
    )]
    pub num_shots: ::core::option::Option<
        translate_quil_to_encrypted_controller_job_request::NumShots,
    >,
}
/// Nested message and enum types in `TranslateQuilToEncryptedControllerJobRequest`.
pub mod translate_quil_to_encrypted_controller_job_request {
    #[derive(Clone, Copy, PartialEq, ::prost::Oneof)]
    pub enum NumShots {
        #[prost(uint32, tag = "3")]
        NumShotsValue(u32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TranslateQuilToEncryptedControllerJobResponse {
    #[prost(message, optional, tag = "1")]
    pub job: ::core::option::Option<
        super::super::models::controller::EncryptedControllerJob,
    >,
    #[prost(message, optional, tag = "2")]
    pub metadata: ::core::option::Option<
        super::super::models::translation::QuilTranslationMetadata,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuantumProcessorQuilCalibrationProgram {
    /// The Quil program containing the requested calibrations
    #[prost(string, tag = "1")]
    pub quil_calibration_program: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetQuantumProcessorQuilCalibrationProgramRequest {
    /// The quantum processor for which to retrieve the calibration program.
    #[prost(string, tag = "1")]
    pub quantum_processor_id: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod translation_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct TranslationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TranslationClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TranslationClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TranslationClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            TranslationClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn translate_quil_to_encrypted_controller_job(
            &mut self,
            request: impl tonic::IntoRequest<
                super::TranslateQuilToEncryptedControllerJobRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::TranslateQuilToEncryptedControllerJobResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.translation.Translation/TranslateQuilToEncryptedControllerJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "services.translation.Translation",
                        "TranslateQuilToEncryptedControllerJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get the current calibration program for the given quantum processor.
        pub async fn get_quantum_processor_quil_calibration_program(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetQuantumProcessorQuilCalibrationProgramRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QuantumProcessorQuilCalibrationProgram>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.translation.Translation/GetQuantumProcessorQuilCalibrationProgram",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "services.translation.Translation",
                        "GetQuantumProcessorQuilCalibrationProgram",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "server")]
pub mod translation_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TranslationServer.
    #[async_trait]
    pub trait Translation: std::marker::Send + std::marker::Sync + 'static {
        async fn translate_quil_to_encrypted_controller_job(
            &self,
            request: tonic::Request<super::TranslateQuilToEncryptedControllerJobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TranslateQuilToEncryptedControllerJobResponse>,
            tonic::Status,
        >;
        /// Get the current calibration program for the given quantum processor.
        async fn get_quantum_processor_quil_calibration_program(
            &self,
            request: tonic::Request<
                super::GetQuantumProcessorQuilCalibrationProgramRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::QuantumProcessorQuilCalibrationProgram>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct TranslationServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> TranslationServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TranslationServer<T>
    where
        T: Translation,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/services.translation.Translation/TranslateQuilToEncryptedControllerJob" => {
                    #[allow(non_camel_case_types)]
                    struct TranslateQuilToEncryptedControllerJobSvc<T: Translation>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Translation,
                    > tonic::server::UnaryService<
                        super::TranslateQuilToEncryptedControllerJobRequest,
                    > for TranslateQuilToEncryptedControllerJobSvc<T> {
                        type Response = super::TranslateQuilToEncryptedControllerJobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::TranslateQuilToEncryptedControllerJobRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Translation>::translate_quil_to_encrypted_controller_job(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = TranslateQuilToEncryptedControllerJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/services.translation.Translation/GetQuantumProcessorQuilCalibrationProgram" => {
                    #[allow(non_camel_case_types)]
                    struct GetQuantumProcessorQuilCalibrationProgramSvc<T: Translation>(
                        pub Arc<T>,
                    );
                    impl<
                        T: Translation,
                    > tonic::server::UnaryService<
                        super::GetQuantumProcessorQuilCalibrationProgramRequest,
                    > for GetQuantumProcessorQuilCalibrationProgramSvc<T> {
                        type Response = super::QuantumProcessorQuilCalibrationProgram;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetQuantumProcessorQuilCalibrationProgramRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Translation>::get_quantum_processor_quil_calibration_program(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetQuantumProcessorQuilCalibrationProgramSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for TranslationServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "services.translation.Translation";
    impl<T> tonic::server::NamedService for TranslationServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}

