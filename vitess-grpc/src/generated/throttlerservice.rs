/// Generated client implementations.
pub mod throttler_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Throttler defines the throttler RPC calls.
    #[derive(Debug, Clone)]
    pub struct ThrottlerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ThrottlerClient<tonic::transport::Channel> {
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
    impl<T> ThrottlerClient<T>
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
        ) -> ThrottlerClient<InterceptedService<T, F>>
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
            ThrottlerClient::new(InterceptedService::new(inner, interceptor))
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
        /// MaxRates returns the current max rate for each throttler of the process.
        pub async fn max_rates(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::throttlerdata::MaxRatesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::throttlerdata::MaxRatesResponse>,
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
                "/throttlerservice.Throttler/MaxRates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("throttlerservice.Throttler", "MaxRates"));
            self.inner.unary(req, path, codec).await
        }
        /// SetMaxRate allows to change the current max rate for all throttlers
        /// of the process.
        pub async fn set_max_rate(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::throttlerdata::SetMaxRateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::throttlerdata::SetMaxRateResponse>,
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
                "/throttlerservice.Throttler/SetMaxRate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("throttlerservice.Throttler", "SetMaxRate"));
            self.inner.unary(req, path, codec).await
        }
        /// GetConfiguration returns the configuration of the MaxReplicationlag module
        /// for the given throttler or all throttlers if "throttler_name" is empty.
        pub async fn get_configuration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::throttlerdata::GetConfigurationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::throttlerdata::GetConfigurationResponse>,
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
                "/throttlerservice.Throttler/GetConfiguration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("throttlerservice.Throttler", "GetConfiguration"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateConfiguration (partially) updates the configuration of the
        /// MaxReplicationlag module for the given throttler or all throttlers if
        /// "throttler_name" is empty.
        /// If "copy_zero_values" is true, fields with zero values will be copied
        /// as well.
        pub async fn update_configuration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::throttlerdata::UpdateConfigurationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::throttlerdata::UpdateConfigurationResponse>,
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
                "/throttlerservice.Throttler/UpdateConfiguration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("throttlerservice.Throttler", "UpdateConfiguration"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ResetConfiguration resets the configuration of the MaxReplicationlag module
        /// to the initial configuration for the given throttler or all throttlers if
        /// "throttler_name" is empty.
        pub async fn reset_configuration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::throttlerdata::ResetConfigurationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::throttlerdata::ResetConfigurationResponse>,
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
                "/throttlerservice.Throttler/ResetConfiguration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("throttlerservice.Throttler", "ResetConfiguration"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
