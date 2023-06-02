/// Generated client implementations.
pub mod vitess_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Vitess is the main service to access a Vitess cluster. It is the API that vtgate
    /// exposes to serve all queries.
    #[derive(Debug, Clone)]
    pub struct VitessClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VitessClient<tonic::transport::Channel> {
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
    impl<T> VitessClient<T>
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
        ) -> VitessClient<InterceptedService<T, F>>
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
            VitessClient::new(InterceptedService::new(inner, interceptor))
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
        /// Execute tries to route the query to the right shard.
        /// It depends on the query and bind variables to provide enough
        /// information in conjunction with the vindexes to route the query.
        /// API group: v3
        pub async fn execute(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtgate::ExecuteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtgate::ExecuteResponse>,
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
                "/vtgateservice.Vitess/Execute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtgateservice.Vitess", "Execute"));
            self.inner.unary(req, path, codec).await
        }
        /// ExecuteBatch tries to route the list of queries on the right shards.
        /// It depends on the query and bind variables to provide enough
        /// information in conjunction with the vindexes to route the query.
        /// API group: v3
        pub async fn execute_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtgate::ExecuteBatchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtgate::ExecuteBatchResponse>,
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
                "/vtgateservice.Vitess/ExecuteBatch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtgateservice.Vitess", "ExecuteBatch"));
            self.inner.unary(req, path, codec).await
        }
        /// StreamExecute executes a streaming query based on shards.
        /// It depends on the query and bind variables to provide enough
        /// information in conjunction with the vindexes to route the query.
        /// Use this method if the query returns a large number of rows.
        /// API group: v3
        pub async fn stream_execute(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtgate::StreamExecuteRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::vtgate::StreamExecuteResponse>,
            >,
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
                "/vtgateservice.Vitess/StreamExecute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtgateservice.Vitess", "StreamExecute"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// ResolveTransaction resolves a transaction.
        /// API group: Transactions
        pub async fn resolve_transaction(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtgate::ResolveTransactionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtgate::ResolveTransactionResponse>,
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
                "/vtgateservice.Vitess/ResolveTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtgateservice.Vitess", "ResolveTransaction"));
            self.inner.unary(req, path, codec).await
        }
        /// VStream streams binlog events from the requested sources.
        pub async fn v_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtgate::VStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::vtgate::VStreamResponse>,
            >,
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
                "/vtgateservice.Vitess/VStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtgateservice.Vitess", "VStream"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// Prepare is used by the MySQL server plugin as part of supporting prepared statements.
        pub async fn prepare(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtgate::PrepareRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtgate::PrepareResponse>,
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
                "/vtgateservice.Vitess/Prepare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtgateservice.Vitess", "Prepare"));
            self.inner.unary(req, path, codec).await
        }
        /// CloseSession closes the session, rolling back any implicit transactions.
        /// This has the same effect as if a "rollback" statement was executed,
        /// but does not affect the query statistics.
        pub async fn close_session(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtgate::CloseSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtgate::CloseSessionResponse>,
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
                "/vtgateservice.Vitess/CloseSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtgateservice.Vitess", "CloseSession"));
            self.inner.unary(req, path, codec).await
        }
    }
}
