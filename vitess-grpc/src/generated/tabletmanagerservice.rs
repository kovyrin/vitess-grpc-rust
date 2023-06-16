/// Generated client implementations.
pub mod tablet_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// TabletManager is a service definition for tabletmanagerdata.TabletManager.
    #[derive(Debug, Clone)]
    pub struct TabletManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TabletManagerClient<tonic::transport::Channel> {
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
    impl<T> TabletManagerClient<T>
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
        ) -> TabletManagerClient<InterceptedService<T, F>>
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
            TabletManagerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Ping returns the input payload
        pub async fn ping(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::PingRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::PingResponse>,
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
                "/tabletmanagerservice.TabletManager/Ping",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tabletmanagerservice.TabletManager", "Ping"));
            self.inner.unary(req, path, codec).await
        }
        /// Sleep sleeps for the provided duration
        pub async fn sleep(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::SleepRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::SleepResponse>,
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
                "/tabletmanagerservice.TabletManager/Sleep",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tabletmanagerservice.TabletManager", "Sleep"));
            self.inner.unary(req, path, codec).await
        }
        /// ExecuteHook executes the hook remotely
        pub async fn execute_hook(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ExecuteHookRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::ExecuteHookResponse>,
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
                "/tabletmanagerservice.TabletManager/ExecuteHook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "ExecuteHook"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetSchema asks the tablet for its schema
        pub async fn get_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::GetSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::GetSchemaResponse>,
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
                "/tabletmanagerservice.TabletManager/GetSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "GetSchema"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetPermissions asks the tablet for its permissions
        pub async fn get_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::GetPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::GetPermissionsResponse>,
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
                "/tabletmanagerservice.TabletManager/GetPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "GetPermissions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_read_only(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::SetReadOnlyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::SetReadOnlyResponse>,
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
                "/tabletmanagerservice.TabletManager/SetReadOnly",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "SetReadOnly"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn set_read_write(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::SetReadWriteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::SetReadWriteResponse>,
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
                "/tabletmanagerservice.TabletManager/SetReadWrite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "SetReadWrite"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ChangeType asks the remote tablet to change its type
        pub async fn change_type(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ChangeTypeRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::ChangeTypeResponse>,
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
                "/tabletmanagerservice.TabletManager/ChangeType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "ChangeType"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn refresh_state(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::RefreshStateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::RefreshStateResponse>,
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
                "/tabletmanagerservice.TabletManager/RefreshState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "RefreshState"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn run_health_check(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::RunHealthCheckRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::RunHealthCheckResponse>,
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
                "/tabletmanagerservice.TabletManager/RunHealthCheck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "RunHealthCheck",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reload_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ReloadSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::ReloadSchemaResponse>,
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
                "/tabletmanagerservice.TabletManager/ReloadSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "ReloadSchema"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn preflight_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::PreflightSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::PreflightSchemaResponse>,
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
                "/tabletmanagerservice.TabletManager/PreflightSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "PreflightSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn apply_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ApplySchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::ApplySchemaResponse>,
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
                "/tabletmanagerservice.TabletManager/ApplySchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "ApplySchema"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn lock_tables(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::LockTablesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::LockTablesResponse>,
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
                "/tabletmanagerservice.TabletManager/LockTables",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "LockTables"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_tables(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::UnlockTablesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::UnlockTablesResponse>,
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
                "/tabletmanagerservice.TabletManager/UnlockTables",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "UnlockTables"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_query(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ExecuteQueryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::ExecuteQueryResponse>,
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
                "/tabletmanagerservice.TabletManager/ExecuteQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "ExecuteQuery"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_fetch_as_dba(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ExecuteFetchAsDbaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::ExecuteFetchAsDbaResponse>,
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
                "/tabletmanagerservice.TabletManager/ExecuteFetchAsDba",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "ExecuteFetchAsDba",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_fetch_as_all_privs(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ExecuteFetchAsAllPrivsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::tabletmanagerdata::ExecuteFetchAsAllPrivsResponse,
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
                "/tabletmanagerservice.TabletManager/ExecuteFetchAsAllPrivs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "ExecuteFetchAsAllPrivs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn execute_fetch_as_app(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ExecuteFetchAsAppRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::ExecuteFetchAsAppResponse>,
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
                "/tabletmanagerservice.TabletManager/ExecuteFetchAsApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "ExecuteFetchAsApp",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ReplicationStatus returns the current replication status.
        pub async fn replication_status(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ReplicationStatusRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::ReplicationStatusResponse>,
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
                "/tabletmanagerservice.TabletManager/ReplicationStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "ReplicationStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PrimaryStatus returns the current primary status.
        pub async fn primary_status(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::PrimaryStatusRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::PrimaryStatusResponse>,
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
                "/tabletmanagerservice.TabletManager/PrimaryStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "PrimaryStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PrimaryPosition returns the current primary position
        pub async fn primary_position(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::PrimaryPositionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::PrimaryPositionResponse>,
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
                "/tabletmanagerservice.TabletManager/PrimaryPosition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "PrimaryPosition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// WaitForPosition waits for the position to be reached
        pub async fn wait_for_position(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::WaitForPositionRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::WaitForPositionResponse>,
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
                "/tabletmanagerservice.TabletManager/WaitForPosition",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "WaitForPosition",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// StopReplication makes mysql stop its replication
        pub async fn stop_replication(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::StopReplicationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::StopReplicationResponse>,
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
                "/tabletmanagerservice.TabletManager/StopReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "StopReplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// StopReplicationMinimum stops the mysql replication after it reaches
        /// the provided minimum point
        pub async fn stop_replication_minimum(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::StopReplicationMinimumRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::tabletmanagerdata::StopReplicationMinimumResponse,
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
                "/tabletmanagerservice.TabletManager/StopReplicationMinimum",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "StopReplicationMinimum",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// StartReplication starts the mysql replication
        pub async fn start_replication(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::StartReplicationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::StartReplicationResponse>,
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
                "/tabletmanagerservice.TabletManager/StartReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "StartReplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// StartReplicationUnitAfter starts the mysql replication until and including
        /// the provided position
        pub async fn start_replication_until_after(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::StartReplicationUntilAfterRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::tabletmanagerdata::StartReplicationUntilAfterResponse,
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
                "/tabletmanagerservice.TabletManager/StartReplicationUntilAfter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "StartReplicationUntilAfter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetReplicas asks for the list of mysql replicas
        pub async fn get_replicas(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::GetReplicasRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::GetReplicasResponse>,
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
                "/tabletmanagerservice.TabletManager/GetReplicas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "GetReplicas"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// VReplication API
        pub async fn v_replication_exec(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::VReplicationExecRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::VReplicationExecResponse>,
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
                "/tabletmanagerservice.TabletManager/VReplicationExec",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "VReplicationExec",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn v_replication_wait_for_pos(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::VReplicationWaitForPosRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::tabletmanagerdata::VReplicationWaitForPosResponse,
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
                "/tabletmanagerservice.TabletManager/VReplicationWaitForPos",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "VReplicationWaitForPos",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// VDiff API
        pub async fn v_diff(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::VDiffRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::VDiffResponse>,
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
                "/tabletmanagerservice.TabletManager/VDiff",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tabletmanagerservice.TabletManager", "VDiff"));
            self.inner.unary(req, path, codec).await
        }
        /// ResetReplication makes the target not replicating
        pub async fn reset_replication(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ResetReplicationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::ResetReplicationResponse>,
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
                "/tabletmanagerservice.TabletManager/ResetReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "ResetReplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// InitPrimary initializes the tablet as a primary
        pub async fn init_primary(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::InitPrimaryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::InitPrimaryResponse>,
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
                "/tabletmanagerservice.TabletManager/InitPrimary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "InitPrimary"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PopulateReparentJournal tells the tablet to add an entry to its
        /// reparent journal
        pub async fn populate_reparent_journal(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::PopulateReparentJournalRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::tabletmanagerdata::PopulateReparentJournalResponse,
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
                "/tabletmanagerservice.TabletManager/PopulateReparentJournal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "PopulateReparentJournal",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// InitReplica tells the tablet to reparent to the primary unconditionally
        pub async fn init_replica(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::InitReplicaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::InitReplicaResponse>,
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
                "/tabletmanagerservice.TabletManager/InitReplica",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "InitReplica"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// DemotePrimary tells the soon-to-be-former primary it's gonna change
        pub async fn demote_primary(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::DemotePrimaryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::DemotePrimaryResponse>,
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
                "/tabletmanagerservice.TabletManager/DemotePrimary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "DemotePrimary",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UndoDemotePrimary reverts all changes made by DemotePrimary
        pub async fn undo_demote_primary(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::UndoDemotePrimaryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::UndoDemotePrimaryResponse>,
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
                "/tabletmanagerservice.TabletManager/UndoDemotePrimary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "UndoDemotePrimary",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ReplicaWasPromoted tells the remote tablet it is now the primary
        pub async fn replica_was_promoted(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ReplicaWasPromotedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::ReplicaWasPromotedResponse>,
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
                "/tabletmanagerservice.TabletManager/ReplicaWasPromoted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "ReplicaWasPromoted",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ResetReplicationParameters resets the replica replication parameters
        pub async fn reset_replication_parameters(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ResetReplicationParametersRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::tabletmanagerdata::ResetReplicationParametersResponse,
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
                "/tabletmanagerservice.TabletManager/ResetReplicationParameters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "ResetReplicationParameters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// FullStatus collects and returns the full status of MySQL including the replication information, semi-sync information, GTID information among others
        pub async fn full_status(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::FullStatusRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::FullStatusResponse>,
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
                "/tabletmanagerservice.TabletManager/FullStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("tabletmanagerservice.TabletManager", "FullStatus"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// SetReplicationSource tells the replica to reparent
        pub async fn set_replication_source(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::SetReplicationSourceRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::tabletmanagerdata::SetReplicationSourceResponse,
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
                "/tabletmanagerservice.TabletManager/SetReplicationSource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "SetReplicationSource",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ReplicaWasRestarted tells the remote tablet its primary has changed
        pub async fn replica_was_restarted(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::ReplicaWasRestartedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::tabletmanagerdata::ReplicaWasRestartedResponse,
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
                "/tabletmanagerservice.TabletManager/ReplicaWasRestarted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "ReplicaWasRestarted",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// StopReplicationAndGetStatus stops MySQL replication, and returns the
        /// replication status
        pub async fn stop_replication_and_get_status(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::StopReplicationAndGetStatusRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::tabletmanagerdata::StopReplicationAndGetStatusResponse,
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
                "/tabletmanagerservice.TabletManager/StopReplicationAndGetStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "StopReplicationAndGetStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// PromoteReplica makes the replica the new primary
        pub async fn promote_replica(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::PromoteReplicaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::PromoteReplicaResponse>,
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
                "/tabletmanagerservice.TabletManager/PromoteReplica",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "PromoteReplica",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn backup(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::BackupRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::tabletmanagerdata::BackupResponse>,
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
                "/tabletmanagerservice.TabletManager/Backup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tabletmanagerservice.TabletManager", "Backup"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// RestoreFromBackup deletes all local data and restores it from the latest backup.
        pub async fn restore_from_backup(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::RestoreFromBackupRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::tabletmanagerdata::RestoreFromBackupResponse,
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
                "/tabletmanagerservice.TabletManager/RestoreFromBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tabletmanagerservice.TabletManager",
                        "RestoreFromBackup",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Generic VExec request. Can be used for various purposes
        pub async fn v_exec(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::tabletmanagerdata::VExecRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::tabletmanagerdata::VExecResponse>,
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
                "/tabletmanagerservice.TabletManager/VExec",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("tabletmanagerservice.TabletManager", "VExec"));
            self.inner.unary(req, path, codec).await
        }
    }
}
