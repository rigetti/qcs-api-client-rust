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


/// A request to execute multiple ControllerJobs as if they were sent as separate requests.
/// Note that the job execution IDs will be returned in the same order as the requests,
/// but execution itself may occur out of that order depending on executor configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchExecuteControllerJobsRequest {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<ExecuteControllerJobRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchExecuteControllerJobsResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<ExecuteControllerJobResponse>,
}
/// A request to execute a given ControllerJob on a specific target with one or more configurations.
/// Note that a request to execute a job with zero configurations will result in an error.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteControllerJobRequest {
    /// One or more configurations against which to execute the provided job.
    ///
    /// The response will include one `job_execution_id` for each entry in this list,
    /// each corresponding to its configuration in the same order.
    #[prost(message, repeated, tag = "3")]
    pub execution_configurations: ::prost::alloc::vec::Vec<
        super::super::models::controller::JobExecutionConfiguration,
    >,
    #[prost(message, optional, tag = "4")]
    pub options: ::core::option::Option<ExecutionOptions>,
    #[prost(oneof = "execute_controller_job_request::Job", tags = "201")]
    pub job: ::core::option::Option<execute_controller_job_request::Job>,
    /// Required by the gateway to forward requests to the correct execution host.
    #[prost(oneof = "execute_controller_job_request::Target", tags = "101, 102")]
    pub target: ::core::option::Option<execute_controller_job_request::Target>,
}
/// Nested message and enum types in `ExecuteControllerJobRequest`.
pub mod execute_controller_job_request {
    #[derive(serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Job {
        #[prost(message, tag = "201")]
        Encrypted(super::super::super::models::controller::EncryptedControllerJob),
    }
    /// Required by the gateway to forward requests to the correct execution host.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        #[prost(string, tag = "101")]
        QuantumProcessorId(::prost::alloc::string::String),
        #[prost(string, tag = "102")]
        EndpointId(::prost::alloc::string::String),
    }
}
/// Options specified on execution requests describing any features or processes requested before or after job execution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionOptions {
    /// If jobs contain settings that would cause managed settings to change values,
    /// that job will be rejected unless this field is set to true and the submitter has the appropriate authorization.
    #[prost(bool, tag = "3")]
    pub bypass_settings_protection: bool,
    /// The timeout while running a job; the job will be evicted from the hardware
    /// once this time has elapsed.
    ///
    /// If unset, the job's estimated duration will be used;
    /// if the job does not have an estimated duration, the default
    /// timeout is selected by the service.
    ///
    /// The service may also enforce a maximum value for this field.
    #[prost(message, optional, tag = "4")]
    pub timeout: ::core::option::Option<::pbjson_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteControllerJobResponse {
    /// One execution ID per input JobExecutionConfiguration, in the same order as the input.
    #[prost(string, repeated, tag = "1")]
    pub job_execution_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetControllerJobResultsRequest {
    /// Which Controller Job execution to query for results
    #[prost(string, tag = "1")]
    pub job_execution_id: ::prost::alloc::string::String,
    /// Required by the gateway to forward requests to the correct execution host.
    #[prost(oneof = "get_controller_job_results_request::Target", tags = "101, 102")]
    pub target: ::core::option::Option<get_controller_job_results_request::Target>,
}
/// Nested message and enum types in `GetControllerJobResultsRequest`.
pub mod get_controller_job_results_request {
    /// Required by the gateway to forward requests to the correct execution host.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        #[prost(string, tag = "101")]
        QuantumProcessorId(::prost::alloc::string::String),
        #[prost(string, tag = "102")]
        EndpointId(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetControllerJobResultsResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<
        super::super::models::controller::ControllerJobExecutionResult,
    >,
}
/// Cancel all given jobs that have yet to begin executing.
/// This endpoint is *not* atomic, and will attempt to cancel every job even
/// when some jobs cannot be canceled. A job can be canceled only if it
/// has not yet started executing.
///
/// Success response indicates only that the request was received. Cancellation
/// is not guaranteed, as it is based on job state at time of cancellation, and is
/// completed on a best-effort basis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelControllerJobsRequest {
    #[prost(string, repeated, tag = "1")]
    pub job_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelControllerJobsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetControllerJobStatusRequest {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetControllerJobStatusResponse {
    #[prost(enumeration = "get_controller_job_status_response::Status", tag = "1")]
    pub status: i32,
}
/// Nested message and enum types in `GetControllerJobStatusResponse`.
pub mod get_controller_job_status_response {
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
        Queued = 1,
        Running = 2,
        Succeeded = 3,
        Failed = 4,
        Canceled = 5,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unknown => "UNKNOWN",
                Status::Queued => "QUEUED",
                Status::Running => "RUNNING",
                Status::Succeeded => "SUCCEEDED",
                Status::Failed => "FAILED",
                Status::Canceled => "CANCELED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "QUEUED" => Some(Self::Queued),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "CANCELED" => Some(Self::Canceled),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod controller_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct ControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ControllerClient<tonic::transport::Channel> {
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
    impl<T> ControllerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> ControllerClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + Send + Sync,
        {
            ControllerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn execute_controller_job(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecuteControllerJobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteControllerJobResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.controller.Controller/ExecuteControllerJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "services.controller.Controller",
                        "ExecuteControllerJob",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_execute_controller_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchExecuteControllerJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchExecuteControllerJobsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.controller.Controller/BatchExecuteControllerJobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "services.controller.Controller",
                        "BatchExecuteControllerJobs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_controller_job_results(
            &mut self,
            request: impl tonic::IntoRequest<super::GetControllerJobResultsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetControllerJobResultsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.controller.Controller/GetControllerJobResults",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "services.controller.Controller",
                        "GetControllerJobResults",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_controller_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelControllerJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelControllerJobsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.controller.Controller/CancelControllerJobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "services.controller.Controller",
                        "CancelControllerJobs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_controller_job_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetControllerJobStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetControllerJobStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/services.controller.Controller/GetControllerJobStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "services.controller.Controller",
                        "GetControllerJobStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "server")]
pub mod controller_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ControllerServer.
    #[async_trait]
    pub trait Controller: Send + Sync + 'static {
        async fn execute_controller_job(
            &self,
            request: tonic::Request<super::ExecuteControllerJobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExecuteControllerJobResponse>,
            tonic::Status,
        >;
        async fn batch_execute_controller_jobs(
            &self,
            request: tonic::Request<super::BatchExecuteControllerJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchExecuteControllerJobsResponse>,
            tonic::Status,
        >;
        async fn get_controller_job_results(
            &self,
            request: tonic::Request<super::GetControllerJobResultsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetControllerJobResultsResponse>,
            tonic::Status,
        >;
        async fn cancel_controller_jobs(
            &self,
            request: tonic::Request<super::CancelControllerJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelControllerJobsResponse>,
            tonic::Status,
        >;
        async fn get_controller_job_status(
            &self,
            request: tonic::Request<super::GetControllerJobStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetControllerJobStatusResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ControllerServer<T: Controller> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Controller> ControllerServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ControllerServer<T>
    where
        T: Controller,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
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
            let inner = self.inner.clone();
            match req.uri().path() {
                "/services.controller.Controller/ExecuteControllerJob" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteControllerJobSvc<T: Controller>(pub Arc<T>);
                    impl<
                        T: Controller,
                    > tonic::server::UnaryService<super::ExecuteControllerJobRequest>
                    for ExecuteControllerJobSvc<T> {
                        type Response = super::ExecuteControllerJobResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecuteControllerJobRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).execute_controller_job(request).await
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
                        let inner = inner.0;
                        let method = ExecuteControllerJobSvc(inner);
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
                "/services.controller.Controller/BatchExecuteControllerJobs" => {
                    #[allow(non_camel_case_types)]
                    struct BatchExecuteControllerJobsSvc<T: Controller>(pub Arc<T>);
                    impl<
                        T: Controller,
                    > tonic::server::UnaryService<
                        super::BatchExecuteControllerJobsRequest,
                    > for BatchExecuteControllerJobsSvc<T> {
                        type Response = super::BatchExecuteControllerJobsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BatchExecuteControllerJobsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).batch_execute_controller_jobs(request).await
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
                        let inner = inner.0;
                        let method = BatchExecuteControllerJobsSvc(inner);
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
                "/services.controller.Controller/GetControllerJobResults" => {
                    #[allow(non_camel_case_types)]
                    struct GetControllerJobResultsSvc<T: Controller>(pub Arc<T>);
                    impl<
                        T: Controller,
                    > tonic::server::UnaryService<super::GetControllerJobResultsRequest>
                    for GetControllerJobResultsSvc<T> {
                        type Response = super::GetControllerJobResultsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetControllerJobResultsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_controller_job_results(request).await
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
                        let inner = inner.0;
                        let method = GetControllerJobResultsSvc(inner);
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
                "/services.controller.Controller/CancelControllerJobs" => {
                    #[allow(non_camel_case_types)]
                    struct CancelControllerJobsSvc<T: Controller>(pub Arc<T>);
                    impl<
                        T: Controller,
                    > tonic::server::UnaryService<super::CancelControllerJobsRequest>
                    for CancelControllerJobsSvc<T> {
                        type Response = super::CancelControllerJobsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelControllerJobsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).cancel_controller_jobs(request).await
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
                        let inner = inner.0;
                        let method = CancelControllerJobsSvc(inner);
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
                "/services.controller.Controller/GetControllerJobStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetControllerJobStatusSvc<T: Controller>(pub Arc<T>);
                    impl<
                        T: Controller,
                    > tonic::server::UnaryService<super::GetControllerJobStatusRequest>
                    for GetControllerJobStatusSvc<T> {
                        type Response = super::GetControllerJobStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetControllerJobStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_controller_job_status(request).await
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
                        let inner = inner.0;
                        let method = GetControllerJobStatusSvc(inner);
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
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Controller> Clone for ControllerServer<T> {
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
    impl<T: Controller> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Controller> tonic::server::NamedService for ControllerServer<T> {
        const NAME: &'static str = "services.controller.Controller";
    }
}

