/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query defines the tablet query service, implemented by vttablet.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
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
    impl<T> QueryClient<T>
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
        ) -> QueryClient<InterceptedService<T, F>>
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
            QueryClient::new(InterceptedService::new(inner, interceptor))
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
        /// Execute executes the specified SQL query (might be in a
        /// transaction context, if Query.transaction_id is set).
        pub async fn execute(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::ExecuteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::ExecuteResponse>,
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
                "/queryservice.Query/Execute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "Execute"));
            self.inner.unary(req, path, codec).await
        }
        /// StreamExecute executes a streaming query. Use this method if the
        /// query returns a large number of rows. The first QueryResult will
        /// contain the Fields, subsequent QueryResult messages will contain
        /// the rows.
        pub async fn stream_execute(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::StreamExecuteRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::query::StreamExecuteResponse>,
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
                "/queryservice.Query/StreamExecute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "StreamExecute"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// Begin a transaction.
        pub async fn begin(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::BeginRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::BeginResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/queryservice.Query/Begin");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("queryservice.Query", "Begin"));
            self.inner.unary(req, path, codec).await
        }
        /// Commit a transaction.
        pub async fn commit(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::CommitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::CommitResponse>,
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
                "/queryservice.Query/Commit",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("queryservice.Query", "Commit"));
            self.inner.unary(req, path, codec).await
        }
        /// Rollback a transaction.
        pub async fn rollback(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::RollbackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::RollbackResponse>,
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
                "/queryservice.Query/Rollback",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "Rollback"));
            self.inner.unary(req, path, codec).await
        }
        /// Prepare preares a transaction.
        pub async fn prepare(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::PrepareRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::PrepareResponse>,
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
                "/queryservice.Query/Prepare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "Prepare"));
            self.inner.unary(req, path, codec).await
        }
        /// CommitPrepared commits a prepared transaction.
        pub async fn commit_prepared(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::CommitPreparedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::CommitPreparedResponse>,
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
                "/queryservice.Query/CommitPrepared",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "CommitPrepared"));
            self.inner.unary(req, path, codec).await
        }
        /// RollbackPrepared rolls back a prepared transaction.
        pub async fn rollback_prepared(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::query::RollbackPreparedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::query::RollbackPreparedResponse>,
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
                "/queryservice.Query/RollbackPrepared",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "RollbackPrepared"));
            self.inner.unary(req, path, codec).await
        }
        /// CreateTransaction creates the metadata for a 2pc transaction.
        pub async fn create_transaction(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::query::CreateTransactionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::query::CreateTransactionResponse>,
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
                "/queryservice.Query/CreateTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "CreateTransaction"));
            self.inner.unary(req, path, codec).await
        }
        /// StartCommit initiates a commit for a 2pc transaction.
        pub async fn start_commit(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::StartCommitRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::StartCommitResponse>,
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
                "/queryservice.Query/StartCommit",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "StartCommit"));
            self.inner.unary(req, path, codec).await
        }
        /// SetRollback marks the 2pc transaction for rollback.
        pub async fn set_rollback(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::SetRollbackRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::SetRollbackResponse>,
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
                "/queryservice.Query/SetRollback",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "SetRollback"));
            self.inner.unary(req, path, codec).await
        }
        /// ConcludeTransaction marks the 2pc transaction as resolved.
        pub async fn conclude_transaction(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::query::ConcludeTransactionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::query::ConcludeTransactionResponse>,
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
                "/queryservice.Query/ConcludeTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "ConcludeTransaction"));
            self.inner.unary(req, path, codec).await
        }
        /// ReadTransaction returns the 2pc transaction info.
        pub async fn read_transaction(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::ReadTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::ReadTransactionResponse>,
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
                "/queryservice.Query/ReadTransaction",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "ReadTransaction"));
            self.inner.unary(req, path, codec).await
        }
        /// BeginExecute executes a begin and the specified SQL query.
        pub async fn begin_execute(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::BeginExecuteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::BeginExecuteResponse>,
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
                "/queryservice.Query/BeginExecute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "BeginExecute"));
            self.inner.unary(req, path, codec).await
        }
        /// BeginStreamExecute executes a begin and the specified SQL query.
        pub async fn begin_stream_execute(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::query::BeginStreamExecuteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::query::BeginStreamExecuteResponse>,
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
                "/queryservice.Query/BeginStreamExecute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "BeginStreamExecute"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// MessageStream streams messages from a message table.
        pub async fn message_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::MessageStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::query::MessageStreamResponse>,
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
                "/queryservice.Query/MessageStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "MessageStream"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// MessageAck acks messages for a table.
        pub async fn message_ack(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::MessageAckRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::MessageAckResponse>,
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
                "/queryservice.Query/MessageAck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "MessageAck"));
            self.inner.unary(req, path, codec).await
        }
        /// ReserveExecute executes a query on a reserved connection
        pub async fn reserve_execute(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::ReserveExecuteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::ReserveExecuteResponse>,
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
                "/queryservice.Query/ReserveExecute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "ReserveExecute"));
            self.inner.unary(req, path, codec).await
        }
        /// ReserveBeginExecute starts a transaction and executes a query in the transaction on a reserved connection
        pub async fn reserve_begin_execute(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::query::ReserveBeginExecuteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::query::ReserveBeginExecuteResponse>,
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
                "/queryservice.Query/ReserveBeginExecute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "ReserveBeginExecute"));
            self.inner.unary(req, path, codec).await
        }
        /// ReserveStreamExecute executes a streaming query on a reserved connection
        pub async fn reserve_stream_execute(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::query::ReserveStreamExecuteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::query::ReserveStreamExecuteResponse,
                >,
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
                "/queryservice.Query/ReserveStreamExecute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "ReserveStreamExecute"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// ReserveBeginStreamExecute starts a transaction and executes a streaming query in the transaction on a reserved connection
        pub async fn reserve_begin_stream_execute(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::query::ReserveBeginStreamExecuteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::query::ReserveBeginStreamExecuteResponse,
                >,
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
                "/queryservice.Query/ReserveBeginStreamExecute",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("queryservice.Query", "ReserveBeginStreamExecute"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Release releases the connection
        pub async fn release(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::ReleaseRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::query::ReleaseResponse>,
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
                "/queryservice.Query/Release",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "Release"));
            self.inner.unary(req, path, codec).await
        }
        /// StreamHealth runs a streaming RPC to the tablet, that returns the
        /// current health of the tablet on a regular basis.
        pub async fn stream_health(
            &mut self,
            request: impl tonic::IntoRequest<super::super::query::StreamHealthRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::query::StreamHealthResponse>,
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
                "/queryservice.Query/StreamHealth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "StreamHealth"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// VStream streams vreplication events.
        pub async fn v_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::super::binlogdata::VStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::binlogdata::VStreamResponse>,
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
                "/queryservice.Query/VStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "VStream"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// VStreamRows streams rows from the specified starting point.
        pub async fn v_stream_rows(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::binlogdata::VStreamRowsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::binlogdata::VStreamRowsResponse>,
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
                "/queryservice.Query/VStreamRows",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "VStreamRows"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// VStreamResults streams results along with the gtid of the snapshot.
        pub async fn v_stream_results(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::binlogdata::VStreamResultsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::binlogdata::VStreamResultsResponse>,
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
                "/queryservice.Query/VStreamResults",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("queryservice.Query", "VStreamResults"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
