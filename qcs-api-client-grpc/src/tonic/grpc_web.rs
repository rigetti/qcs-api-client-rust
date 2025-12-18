// Copyright 2024 Rigetti Computing
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

use super::BoxBody;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tonic::client::GrpcService;
use tonic::codegen::http::{Request, Response};
use tonic_web::{GrpcWebCall, GrpcWebClientService};
use tower::{Layer, ServiceBuilder};

/// Add grpc-web support to the `channel`, converting
/// all gRPC requests to use the grpc-web protocol and HTTP/1.1
pub fn wrap_channel_with_grpc_web<C>(channel: C) -> GrpcWebWrapperLayerService<C>
where
    C: GrpcService<BoxBody>,
{
    ServiceBuilder::new()
        .layer(GrpcWebWrapperLayer)
        .service(channel)
}

/// A [`Layer`] that provides [`Request<BoxBody>`] compatibility for
/// [`GrpcWebClientService`] (which uses [`Request<GrpcWebCall<BoxBody>>`]).
/// This allows it to be used with other layers in this module like
/// [`RefreshLayer`] and [`RetryLayer`].
#[derive(Copy, Clone)]
pub struct GrpcWebWrapperLayer;

/// The type of [`GrpcService`] created by [`GrpcWebWrapperLayer`].
pub type GrpcWebWrapperLayerService<S> = GrpcWebClientService<GrpcWebWrapperService<S>>;

impl<S> Layer<S> for GrpcWebWrapperLayer
where
    S: GrpcService<BoxBody>,
{
    type Service = GrpcWebWrapperLayerService<S>;

    fn layer(&self, service: S) -> Self::Service {
        GrpcWebClientService::new(GrpcWebWrapperService { service })
    }
}

#[derive(Clone)]
/// The [`GrpcService`] that wraps the gRPC client in order to convert all
/// gRPC requests to use the grpc-web protocol and HTTP/1.1.
pub struct GrpcWebWrapperService<S> {
    service: S,
}

impl<S> tower::Service<Request<GrpcWebCall<BoxBody>>> for GrpcWebWrapperService<S>
where
    S: GrpcService<BoxBody> + Clone + Send + 'static,
    <S as GrpcService<BoxBody>>::Future: Send,
    <S as GrpcService<BoxBody>>::ResponseBody: Send,
{
    type Response = Response<<S as GrpcService<BoxBody>>::ResponseBody>;
    type Error = <S as GrpcService<BoxBody>>::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request<GrpcWebCall<BoxBody>>) -> Self::Future {
        // Unpack the GrpcWebCall part
        let mut service = self.service.clone();
        std::mem::swap(&mut self.service, &mut service);
        super::common::pin_future_with_otel_context_if_available(async move {
            let headers = req.headers().clone();
            let method = req.method().clone();
            let uri = req.uri().clone();
            let version = req.version();
            let body = BoxBody::new(req.into_body());

            let mut builder = Request::builder();

            for (key, value) in headers {
                if let Some(key) = key {
                    builder = builder.header(key, value);
                }
            }

            let request = builder
                .method(method)
                .uri(uri)
                .version(version)
                .body(body)
                .expect("valid request");
            service.call(request).await
        })
    }
}
