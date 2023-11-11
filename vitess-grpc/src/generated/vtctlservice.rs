/// Generated client implementations.
pub mod vtctl_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service Vtctl allows you to call vt commands through gRPC.
    #[derive(Debug, Clone)]
    pub struct VtctlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VtctlClient<tonic::transport::Channel> {
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
    impl<T> VtctlClient<T>
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
        ) -> VtctlClient<InterceptedService<T, F>>
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
            VtctlClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn execute_vtctl_command(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ExecuteVtctlCommandRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::vtctldata::ExecuteVtctlCommandResponse,
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
                "/vtctlservice.Vtctl/ExecuteVtctlCommand",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctl", "ExecuteVtctlCommand"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod vtctld_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service Vtctld exposes gRPC endpoints for each vt command.
    #[derive(Debug, Clone)]
    pub struct VtctldClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VtctldClient<tonic::transport::Channel> {
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
    impl<T> VtctldClient<T>
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
        ) -> VtctldClient<InterceptedService<T, F>>
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
            VtctldClient::new(InterceptedService::new(inner, interceptor))
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
        /// AddCellInfo registers a local topology service in a new cell by creating
        /// the CellInfo with the provided parameters.
        pub async fn add_cell_info(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::AddCellInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::AddCellInfoResponse>,
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
                "/vtctlservice.Vtctld/AddCellInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "AddCellInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// AddCellsAlias defines a group of cells that can be referenced by a single
        /// name (the alias).
        ///
        /// When routing query traffic, replica/rdonly traffic can be routed across
        /// cells within the group (alias). Only primary traffic can be routed across
        /// cells not in the same group (alias).
        pub async fn add_cells_alias(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::AddCellsAliasRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::AddCellsAliasResponse>,
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
                "/vtctlservice.Vtctld/AddCellsAlias",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "AddCellsAlias"));
            self.inner.unary(req, path, codec).await
        }
        /// ApplyRoutingRules applies the VSchema routing rules.
        pub async fn apply_routing_rules(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ApplyRoutingRulesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ApplyRoutingRulesResponse>,
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
                "/vtctlservice.Vtctld/ApplyRoutingRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ApplyRoutingRules"));
            self.inner.unary(req, path, codec).await
        }
        /// ApplySchema applies a schema to a keyspace.
        pub async fn apply_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::ApplySchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ApplySchemaResponse>,
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
                "/vtctlservice.Vtctld/ApplySchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ApplySchema"));
            self.inner.unary(req, path, codec).await
        }
        /// ApplyShardRoutingRules applies the VSchema shard routing rules.
        pub async fn apply_shard_routing_rules(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ApplyShardRoutingRulesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ApplyShardRoutingRulesResponse>,
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
                "/vtctlservice.Vtctld/ApplyShardRoutingRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "ApplyShardRoutingRules"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ApplyVSchema applies a vschema to a keyspace.
        pub async fn apply_v_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ApplyVSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ApplyVSchemaResponse>,
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
                "/vtctlservice.Vtctld/ApplyVSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ApplyVSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// Backup uses the BackupEngine and BackupStorage services on the specified
        /// tablet to create and store a new backup.
        pub async fn backup(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::BackupRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::vtctldata::BackupResponse>,
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
                "/vtctlservice.Vtctld/Backup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "Backup"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// BackupShard chooses a tablet in the shard and uses it to create a backup.
        pub async fn backup_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::BackupShardRequest>,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::vtctldata::BackupResponse>,
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
                "/vtctlservice.Vtctld/BackupShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "BackupShard"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// CancelSchemaMigration cancels one or all migrations, terminating any runnign ones as needed.
        pub async fn cancel_schema_migration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::CancelSchemaMigrationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::CancelSchemaMigrationResponse>,
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
                "/vtctlservice.Vtctld/CancelSchemaMigration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "CancelSchemaMigration"));
            self.inner.unary(req, path, codec).await
        }
        /// ChangeTabletType changes the db type for the specified tablet, if possible.
        /// This is used primarily to arrange replicas, and it will not convert a
        /// primary. For that, use InitShardPrimary.
        ///
        /// NOTE: This command automatically updates the serving graph.
        pub async fn change_tablet_type(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ChangeTabletTypeRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ChangeTabletTypeResponse>,
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
                "/vtctlservice.Vtctld/ChangeTabletType",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ChangeTabletType"));
            self.inner.unary(req, path, codec).await
        }
        /// CleanupSchemaMigration marks a schema migration as ready for artifact cleanup.
        pub async fn cleanup_schema_migration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::CleanupSchemaMigrationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::CleanupSchemaMigrationResponse>,
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
                "/vtctlservice.Vtctld/CleanupSchemaMigration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "CleanupSchemaMigration"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CompleteSchemaMigration completes one or all migrations executed with --postpone-completion.
        pub async fn complete_schema_migration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::CompleteSchemaMigrationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::CompleteSchemaMigrationResponse>,
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
                "/vtctlservice.Vtctld/CompleteSchemaMigration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "CompleteSchemaMigration"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// CreateKeyspace creates the specified keyspace in the topology. For a
        /// SNAPSHOT keyspace, the request must specify the name of a base keyspace,
        /// as well as a snapshot time.
        pub async fn create_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::CreateKeyspaceRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::CreateKeyspaceResponse>,
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
                "/vtctlservice.Vtctld/CreateKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "CreateKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// CreateShard creates the specified shard in the topology.
        pub async fn create_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::CreateShardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::CreateShardResponse>,
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
                "/vtctlservice.Vtctld/CreateShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "CreateShard"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteCellInfo deletes the CellInfo for the provided cell. The cell cannot
        /// be referenced by any Shard record in the topology.
        pub async fn delete_cell_info(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::DeleteCellInfoRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::DeleteCellInfoResponse>,
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
                "/vtctlservice.Vtctld/DeleteCellInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "DeleteCellInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteCellsAlias deletes the CellsAlias for the provided alias.
        pub async fn delete_cells_alias(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::DeleteCellsAliasRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::DeleteCellsAliasResponse>,
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
                "/vtctlservice.Vtctld/DeleteCellsAlias",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "DeleteCellsAlias"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteKeyspace deletes the specified keyspace from the topology. In
        /// recursive mode, it also recursively deletes all shards in the keyspace.
        /// Otherwise, the keyspace must be empty (have no shards), or DeleteKeyspace
        /// returns an error.
        pub async fn delete_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::DeleteKeyspaceRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::DeleteKeyspaceResponse>,
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
                "/vtctlservice.Vtctld/DeleteKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "DeleteKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteShards deletes the specified shards from the topology. In recursive
        /// mode, it also deletes all tablets belonging to the shard. Otherwise, the
        /// shard must be empty (have no tablets) or DeleteShards returns an error for
        /// that shard.
        pub async fn delete_shards(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::DeleteShardsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::DeleteShardsResponse>,
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
                "/vtctlservice.Vtctld/DeleteShards",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "DeleteShards"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteSrvVSchema deletes the SrvVSchema object in the specified cell.
        pub async fn delete_srv_v_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::DeleteSrvVSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::DeleteSrvVSchemaResponse>,
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
                "/vtctlservice.Vtctld/DeleteSrvVSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "DeleteSrvVSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteTablets deletes one or more tablets from the topology.
        pub async fn delete_tablets(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::DeleteTabletsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::DeleteTabletsResponse>,
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
                "/vtctlservice.Vtctld/DeleteTablets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "DeleteTablets"));
            self.inner.unary(req, path, codec).await
        }
        /// EmergencyReparentShard reparents the shard to the new primary. It assumes
        /// the old primary is dead or otherwise not responding.
        pub async fn emergency_reparent_shard(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::EmergencyReparentShardRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::EmergencyReparentShardResponse>,
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
                "/vtctlservice.Vtctld/EmergencyReparentShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "EmergencyReparentShard"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ExecuteFetchAsApp executes a SQL query on the remote tablet as the App user.
        pub async fn execute_fetch_as_app(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ExecuteFetchAsAppRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ExecuteFetchAsAppResponse>,
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
                "/vtctlservice.Vtctld/ExecuteFetchAsApp",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ExecuteFetchAsApp"));
            self.inner.unary(req, path, codec).await
        }
        /// ExecuteFetchAsDBA executes a SQL query on the remote tablet as the DBA user.
        pub async fn execute_fetch_as_dba(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ExecuteFetchAsDbaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ExecuteFetchAsDbaResponse>,
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
                "/vtctlservice.Vtctld/ExecuteFetchAsDBA",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ExecuteFetchAsDBA"));
            self.inner.unary(req, path, codec).await
        }
        /// ExecuteHook runs the hook on the tablet.
        pub async fn execute_hook(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::ExecuteHookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ExecuteHookResponse>,
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
                "/vtctlservice.Vtctld/ExecuteHook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ExecuteHook"));
            self.inner.unary(req, path, codec).await
        }
        /// FindAllShardsInKeyspace returns a map of shard names to shard references
        /// for a given keyspace.
        pub async fn find_all_shards_in_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::FindAllShardsInKeyspaceRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::FindAllShardsInKeyspaceResponse>,
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
                "/vtctlservice.Vtctld/FindAllShardsInKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "FindAllShardsInKeyspace"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetBackups returns all the backups for a shard.
        pub async fn get_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::GetBackupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetBackupsResponse>,
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
                "/vtctlservice.Vtctld/GetBackups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetBackups"));
            self.inner.unary(req, path, codec).await
        }
        /// GetCellInfo returns the information for a cell.
        pub async fn get_cell_info(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::GetCellInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetCellInfoResponse>,
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
                "/vtctlservice.Vtctld/GetCellInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetCellInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// GetCellInfoNames returns all the cells for which we have a CellInfo object,
        /// meaning we have a topology service registered.
        pub async fn get_cell_info_names(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetCellInfoNamesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetCellInfoNamesResponse>,
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
                "/vtctlservice.Vtctld/GetCellInfoNames",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetCellInfoNames"));
            self.inner.unary(req, path, codec).await
        }
        /// GetCellsAliases returns a mapping of cell alias to cells identified by that
        /// alias.
        pub async fn get_cells_aliases(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetCellsAliasesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetCellsAliasesResponse>,
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
                "/vtctlservice.Vtctld/GetCellsAliases",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetCellsAliases"));
            self.inner.unary(req, path, codec).await
        }
        /// GetFullStatus returns the full status of MySQL including the replication information, semi-sync information, GTID information among others
        pub async fn get_full_status(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetFullStatusRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetFullStatusResponse>,
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
                "/vtctlservice.Vtctld/GetFullStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetFullStatus"));
            self.inner.unary(req, path, codec).await
        }
        /// GetKeyspace reads the given keyspace from the topo and returns it.
        pub async fn get_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::GetKeyspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetKeyspaceResponse>,
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
                "/vtctlservice.Vtctld/GetKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// GetKeyspaces returns the keyspace struct of all keyspaces in the topo.
        pub async fn get_keyspaces(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetKeyspacesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetKeyspacesResponse>,
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
                "/vtctlservice.Vtctld/GetKeyspaces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetKeyspaces"));
            self.inner.unary(req, path, codec).await
        }
        /// GetPermissions returns the permissions set on the remote tablet.
        pub async fn get_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetPermissionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetPermissionsResponse>,
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
                "/vtctlservice.Vtctld/GetPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetPermissions"));
            self.inner.unary(req, path, codec).await
        }
        /// GetRoutingRules returns the VSchema routing rules.
        pub async fn get_routing_rules(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetRoutingRulesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetRoutingRulesResponse>,
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
                "/vtctlservice.Vtctld/GetRoutingRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetRoutingRules"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSchema returns the schema for a tablet, or just the schema for the
        /// specified tables in that tablet.
        pub async fn get_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::GetSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetSchemaResponse>,
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
                "/vtctlservice.Vtctld/GetSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSchemaMigrations returns one or more online schema migrations for the
        /// specified keyspace, analagous to `SHOW VITESS_MIGRATIONS`.
        ///
        /// Different fields in the request message result in different filtering
        /// behaviors. See the documentation on GetSchemaMigrationsRequest for details.
        pub async fn get_schema_migrations(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetSchemaMigrationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetSchemaMigrationsResponse>,
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
                "/vtctlservice.Vtctld/GetSchemaMigrations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetSchemaMigrations"));
            self.inner.unary(req, path, codec).await
        }
        /// GetShard returns information about a shard in the topology.
        pub async fn get_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::GetShardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetShardResponse>,
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
                "/vtctlservice.Vtctld/GetShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetShard"));
            self.inner.unary(req, path, codec).await
        }
        /// GetShardRoutingRules returns the VSchema shard routing rules.
        pub async fn get_shard_routing_rules(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetShardRoutingRulesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetShardRoutingRulesResponse>,
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
                "/vtctlservice.Vtctld/GetShardRoutingRules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetShardRoutingRules"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSrvKeyspaceNames returns a mapping of cell name to the keyspaces served
        /// in that cell.
        pub async fn get_srv_keyspace_names(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetSrvKeyspaceNamesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetSrvKeyspaceNamesResponse>,
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
                "/vtctlservice.Vtctld/GetSrvKeyspaceNames",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetSrvKeyspaceNames"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSrvKeyspaces returns the SrvKeyspaces for a keyspace in one or more
        /// cells.
        pub async fn get_srv_keyspaces(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetSrvKeyspacesRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetSrvKeyspacesResponse>,
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
                "/vtctlservice.Vtctld/GetSrvKeyspaces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetSrvKeyspaces"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateThrottlerConfig updates the tablet throttler configuration
        pub async fn update_throttler_config(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::UpdateThrottlerConfigRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::UpdateThrottlerConfigResponse>,
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
                "/vtctlservice.Vtctld/UpdateThrottlerConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "UpdateThrottlerConfig"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSrvVSchema returns the SrvVSchema for a cell.
        pub async fn get_srv_v_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetSrvVSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetSrvVSchemaResponse>,
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
                "/vtctlservice.Vtctld/GetSrvVSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetSrvVSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSrvVSchemas returns a mapping from cell name to SrvVSchema for all cells,
        /// optionally filtered by cell name.
        pub async fn get_srv_v_schemas(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetSrvVSchemasRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetSrvVSchemasResponse>,
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
                "/vtctlservice.Vtctld/GetSrvVSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetSrvVSchemas"));
            self.inner.unary(req, path, codec).await
        }
        /// GetTablet returns information about a tablet.
        pub async fn get_tablet(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::GetTabletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetTabletResponse>,
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
                "/vtctlservice.Vtctld/GetTablet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetTablet"));
            self.inner.unary(req, path, codec).await
        }
        /// GetTablets returns tablets, optionally filtered by keyspace and shard.
        pub async fn get_tablets(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::GetTabletsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetTabletsResponse>,
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
                "/vtctlservice.Vtctld/GetTablets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetTablets"));
            self.inner.unary(req, path, codec).await
        }
        /// GetTopologyPath returns the topology cell at a given path.
        pub async fn get_topology_path(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetTopologyPathRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetTopologyPathResponse>,
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
                "/vtctlservice.Vtctld/GetTopologyPath",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetTopologyPath"));
            self.inner.unary(req, path, codec).await
        }
        /// GetVersion returns the version of a tablet from its debug vars.
        pub async fn get_version(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::GetVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetVersionResponse>,
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
                "/vtctlservice.Vtctld/GetVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetVersion"));
            self.inner.unary(req, path, codec).await
        }
        /// GetVSchema returns the vschema for a keyspace.
        pub async fn get_v_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::GetVSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetVSchemaResponse>,
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
                "/vtctlservice.Vtctld/GetVSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetVSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// GetWorkflows returns a list of workflows for the given keyspace.
        pub async fn get_workflows(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::GetWorkflowsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::GetWorkflowsResponse>,
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
                "/vtctlservice.Vtctld/GetWorkflows",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "GetWorkflows"));
            self.inner.unary(req, path, codec).await
        }
        /// InitShardPrimary sets the initial primary for a shard. Will make all other
        /// tablets in the shard replicas of the provided primary.
        ///
        /// WARNING: This could cause data loss on an already replicating shard.
        /// PlannedReparentShard or EmergencyReparentShard should be used in those
        /// cases instead.
        pub async fn init_shard_primary(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::InitShardPrimaryRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::InitShardPrimaryResponse>,
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
                "/vtctlservice.Vtctld/InitShardPrimary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "InitShardPrimary"));
            self.inner.unary(req, path, codec).await
        }
        /// LaunchSchemaMigration launches one or all migrations executed with --postpone-launch.
        pub async fn launch_schema_migration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::LaunchSchemaMigrationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::LaunchSchemaMigrationResponse>,
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
                "/vtctlservice.Vtctld/LaunchSchemaMigration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "LaunchSchemaMigration"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn lookup_vindex_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::LookupVindexCreateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::LookupVindexCreateResponse>,
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
                "/vtctlservice.Vtctld/LookupVindexCreate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "LookupVindexCreate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn lookup_vindex_externalize(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::LookupVindexExternalizeRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::LookupVindexExternalizeResponse>,
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
                "/vtctlservice.Vtctld/LookupVindexExternalize",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "LookupVindexExternalize"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// MaterializeCreate creates a workflow to materialize one or more tables
        /// from a source keyspace to a target keyspace using a provided expressions.
        pub async fn materialize_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::MaterializeCreateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::MaterializeCreateResponse>,
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
                "/vtctlservice.Vtctld/MaterializeCreate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "MaterializeCreate"));
            self.inner.unary(req, path, codec).await
        }
        /// MigrateCreate creates a workflow which migrates one or more tables from an
        /// external cluster into Vitess.
        pub async fn migrate_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::MigrateCreateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::WorkflowStatusResponse>,
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
                "/vtctlservice.Vtctld/MigrateCreate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "MigrateCreate"));
            self.inner.unary(req, path, codec).await
        }
        /// MountRegister registers a new external Vitess cluster.
        pub async fn mount_register(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::MountRegisterRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::MountRegisterResponse>,
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
                "/vtctlservice.Vtctld/MountRegister",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "MountRegister"));
            self.inner.unary(req, path, codec).await
        }
        /// MountUnregister unregisters an external Vitess cluster.
        pub async fn mount_unregister(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::MountUnregisterRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::MountUnregisterResponse>,
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
                "/vtctlservice.Vtctld/MountUnregister",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "MountUnregister"));
            self.inner.unary(req, path, codec).await
        }
        /// MountShow returns information about an external Vitess cluster.
        pub async fn mount_show(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::MountShowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::MountShowResponse>,
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
                "/vtctlservice.Vtctld/MountShow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "MountShow"));
            self.inner.unary(req, path, codec).await
        }
        /// MountList lists all registered external Vitess clusters.
        pub async fn mount_list(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::MountListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::MountListResponse>,
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
                "/vtctlservice.Vtctld/MountList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "MountList"));
            self.inner.unary(req, path, codec).await
        }
        /// MoveTablesCreate creates a workflow which moves one or more tables from a
        /// source keyspace to a target keyspace.
        pub async fn move_tables_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::MoveTablesCreateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::WorkflowStatusResponse>,
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
                "/vtctlservice.Vtctld/MoveTablesCreate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "MoveTablesCreate"));
            self.inner.unary(req, path, codec).await
        }
        /// MoveTablesComplete completes the move and cleans up the workflow and
        /// its related artifacts.
        pub async fn move_tables_complete(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::MoveTablesCompleteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::MoveTablesCompleteResponse>,
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
                "/vtctlservice.Vtctld/MoveTablesComplete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "MoveTablesComplete"));
            self.inner.unary(req, path, codec).await
        }
        /// PingTablet checks that the specified tablet is awake and responding to RPCs.
        /// This command can be blocked by other in-flight operations.
        pub async fn ping_tablet(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::PingTabletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::PingTabletResponse>,
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
                "/vtctlservice.Vtctld/PingTablet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "PingTablet"));
            self.inner.unary(req, path, codec).await
        }
        /// PlannedReparentShard reparents the shard to the new primary, or away from
        /// an old primary. Both the old and new primaries need to be reachable and
        /// running.
        ///
        /// **NOTE**: The vtctld will not consider any replicas outside the cell the
        /// current shard primary is in for promotion unless NewPrimary is explicitly
        /// provided in the request.
        pub async fn planned_reparent_shard(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::PlannedReparentShardRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::PlannedReparentShardResponse>,
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
                "/vtctlservice.Vtctld/PlannedReparentShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "PlannedReparentShard"));
            self.inner.unary(req, path, codec).await
        }
        /// RebuildKeyspaceGraph rebuilds the serving data for a keyspace.
        ///
        /// This may trigger an update to all connected clients.
        pub async fn rebuild_keyspace_graph(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::RebuildKeyspaceGraphRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::RebuildKeyspaceGraphResponse>,
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
                "/vtctlservice.Vtctld/RebuildKeyspaceGraph",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "RebuildKeyspaceGraph"));
            self.inner.unary(req, path, codec).await
        }
        /// RebuildVSchemaGraph rebuilds the per-cell SrvVSchema from the global
        /// VSchema objects in the provided cells (or all cells in the topo none
        /// provided).
        pub async fn rebuild_v_schema_graph(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::RebuildVSchemaGraphRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::RebuildVSchemaGraphResponse>,
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
                "/vtctlservice.Vtctld/RebuildVSchemaGraph",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "RebuildVSchemaGraph"));
            self.inner.unary(req, path, codec).await
        }
        /// RefreshState reloads the tablet record on the specified tablet.
        pub async fn refresh_state(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::RefreshStateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::RefreshStateResponse>,
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
                "/vtctlservice.Vtctld/RefreshState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "RefreshState"));
            self.inner.unary(req, path, codec).await
        }
        /// RefreshStateByShard calls RefreshState on all the tablets in the given shard.
        pub async fn refresh_state_by_shard(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::RefreshStateByShardRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::RefreshStateByShardResponse>,
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
                "/vtctlservice.Vtctld/RefreshStateByShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "RefreshStateByShard"));
            self.inner.unary(req, path, codec).await
        }
        /// ReloadSchema instructs the remote tablet to reload its schema.
        pub async fn reload_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ReloadSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ReloadSchemaResponse>,
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
                "/vtctlservice.Vtctld/ReloadSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ReloadSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// ReloadSchemaKeyspace reloads the schema on all tablets in a keyspace.
        pub async fn reload_schema_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ReloadSchemaKeyspaceRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ReloadSchemaKeyspaceResponse>,
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
                "/vtctlservice.Vtctld/ReloadSchemaKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ReloadSchemaKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// ReloadSchemaShard reloads the schema on all tablets in a shard.
        ///
        /// In general, we don't always expect all replicas to be ready to reload, and
        /// the periodic schema reload makes them self-healing anyway. So, we do this
        /// on a best-effort basis, and log warnings for any tablets that fail to
        /// reload within the context deadline.
        pub async fn reload_schema_shard(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ReloadSchemaShardRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ReloadSchemaShardResponse>,
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
                "/vtctlservice.Vtctld/ReloadSchemaShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ReloadSchemaShard"));
            self.inner.unary(req, path, codec).await
        }
        /// RemoveBackup removes a backup from the BackupStorage used by vtctld.
        pub async fn remove_backup(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::RemoveBackupRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::RemoveBackupResponse>,
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
                "/vtctlservice.Vtctld/RemoveBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "RemoveBackup"));
            self.inner.unary(req, path, codec).await
        }
        /// RemoveKeyspaceCell removes the specified cell from the Cells list for all
        /// shards in the specified keyspace (by calling RemoveShardCell on every
        /// shard). It also removes the SrvKeyspace for that keyspace in that cell.
        pub async fn remove_keyspace_cell(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::RemoveKeyspaceCellRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::RemoveKeyspaceCellResponse>,
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
                "/vtctlservice.Vtctld/RemoveKeyspaceCell",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "RemoveKeyspaceCell"));
            self.inner.unary(req, path, codec).await
        }
        /// RemoveShardCell removes the specified cell from the specified shard's Cells
        /// list.
        pub async fn remove_shard_cell(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::RemoveShardCellRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::RemoveShardCellResponse>,
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
                "/vtctlservice.Vtctld/RemoveShardCell",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "RemoveShardCell"));
            self.inner.unary(req, path, codec).await
        }
        /// ReparentTablet reparents a tablet to the current primary in the shard. This
        /// only works if the current replica position matches the last known reparent
        /// action.
        pub async fn reparent_tablet(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ReparentTabletRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ReparentTabletResponse>,
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
                "/vtctlservice.Vtctld/ReparentTablet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ReparentTablet"));
            self.inner.unary(req, path, codec).await
        }
        /// ReshardCreate creates a workflow to reshard a keyspace.
        pub async fn reshard_create(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ReshardCreateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::WorkflowStatusResponse>,
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
                "/vtctlservice.Vtctld/ReshardCreate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ReshardCreate"));
            self.inner.unary(req, path, codec).await
        }
        /// RestoreFromBackup stops mysqld for the given tablet and restores a backup.
        pub async fn restore_from_backup(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::RestoreFromBackupRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                tonic::codec::Streaming<
                    super::super::vtctldata::RestoreFromBackupResponse,
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
                "/vtctlservice.Vtctld/RestoreFromBackup",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "RestoreFromBackup"));
            self.inner.server_streaming(req, path, codec).await
        }
        /// RetrySchemaMigration marks a given schema migration for retry.
        pub async fn retry_schema_migration(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::RetrySchemaMigrationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::RetrySchemaMigrationResponse>,
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
                "/vtctlservice.Vtctld/RetrySchemaMigration",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "RetrySchemaMigration"));
            self.inner.unary(req, path, codec).await
        }
        /// RunHealthCheck runs a healthcheck on the remote tablet.
        pub async fn run_health_check(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::RunHealthCheckRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::RunHealthCheckResponse>,
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
                "/vtctlservice.Vtctld/RunHealthCheck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "RunHealthCheck"));
            self.inner.unary(req, path, codec).await
        }
        /// SetKeyspaceDurabilityPolicy updates the DurabilityPolicy for a keyspace.
        pub async fn set_keyspace_durability_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::SetKeyspaceDurabilityPolicyRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<
                super::super::vtctldata::SetKeyspaceDurabilityPolicyResponse,
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
                "/vtctlservice.Vtctld/SetKeyspaceDurabilityPolicy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "SetKeyspaceDurabilityPolicy"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// SetShardIsPrimaryServing adds or removes a shard from serving.
        ///
        /// This is meant as an emergency function. It does not rebuild any serving
        /// graph (i.e. it does not run RebuildKeyspaceGraph).
        pub async fn set_shard_is_primary_serving(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::SetShardIsPrimaryServingRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::SetShardIsPrimaryServingResponse>,
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
                "/vtctlservice.Vtctld/SetShardIsPrimaryServing",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "SetShardIsPrimaryServing"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// SetShardTabletControl updates the TabletControl topo record for a shard and
        /// tablet type.
        ///
        /// This should only be used for an emergency fix, or after a finished
        /// Reshard. See the documentation on SetShardTabletControlRequest for more
        /// information about the different update modes.
        pub async fn set_shard_tablet_control(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::SetShardTabletControlRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::SetShardTabletControlResponse>,
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
                "/vtctlservice.Vtctld/SetShardTabletControl",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "SetShardTabletControl"));
            self.inner.unary(req, path, codec).await
        }
        /// SetWritable sets a tablet as read-write (writable=true) or read-only (writable=false).
        pub async fn set_writable(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::SetWritableRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::SetWritableResponse>,
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
                "/vtctlservice.Vtctld/SetWritable",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "SetWritable"));
            self.inner.unary(req, path, codec).await
        }
        /// ShardReplicationAdd adds an entry to a topodata.ShardReplication object.
        ///
        /// It is a low-level function and should generally not be called.
        pub async fn shard_replication_add(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ShardReplicationAddRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ShardReplicationAddResponse>,
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
                "/vtctlservice.Vtctld/ShardReplicationAdd",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ShardReplicationAdd"));
            self.inner.unary(req, path, codec).await
        }
        /// ShardReplicationFix walks the replication graph for a shard in a cell and
        /// attempts to fix the first problem encountered, returning information about
        /// the problem fixed, if any.
        pub async fn shard_replication_fix(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ShardReplicationFixRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ShardReplicationFixResponse>,
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
                "/vtctlservice.Vtctld/ShardReplicationFix",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ShardReplicationFix"));
            self.inner.unary(req, path, codec).await
        }
        /// ShardReplicationPositions returns the replication position of each tablet
        /// in a shard. This RPC makes a best-effort to return partial results. For
        /// example, if one tablet in the shard graph is unreachable, then
        /// ShardReplicationPositions will return non-error, and include valid results
        /// for the reachable tablets.
        pub async fn shard_replication_positions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ShardReplicationPositionsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ShardReplicationPositionsResponse>,
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
                "/vtctlservice.Vtctld/ShardReplicationPositions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "ShardReplicationPositions"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ShardReplicationRemove removes an entry from a topodata.ShardReplication
        /// object.
        ///
        /// It is a low-level function and should generally not be called.
        pub async fn shard_replication_remove(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ShardReplicationRemoveRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ShardReplicationRemoveResponse>,
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
                "/vtctlservice.Vtctld/ShardReplicationRemove",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "ShardReplicationRemove"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// SleepTablet blocks the aciton queue on the specified tablet for the
        /// specified duration.
        ///
        /// This is typically used for testing.
        pub async fn sleep_tablet(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::SleepTabletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::SleepTabletResponse>,
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
                "/vtctlservice.Vtctld/SleepTablet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "SleepTablet"));
            self.inner.unary(req, path, codec).await
        }
        /// SourceShardAdd adds the SourceShard record with the provided index. This
        /// should be used only as an emergency function.
        ///
        /// It does not call RefreshState for the shard primary.
        pub async fn source_shard_add(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::SourceShardAddRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::SourceShardAddResponse>,
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
                "/vtctlservice.Vtctld/SourceShardAdd",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "SourceShardAdd"));
            self.inner.unary(req, path, codec).await
        }
        /// SourceShardDelete deletes the SourceShard record with the provided index.
        /// This should be used only as an emergency cleanup function.
        ///
        /// It does not call RefreshState for the shard primary.
        pub async fn source_shard_delete(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::SourceShardDeleteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::SourceShardDeleteResponse>,
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
                "/vtctlservice.Vtctld/SourceShardDelete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "SourceShardDelete"));
            self.inner.unary(req, path, codec).await
        }
        /// StartReplication starts replication on the specified tablet.
        pub async fn start_replication(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::StartReplicationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::StartReplicationResponse>,
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
                "/vtctlservice.Vtctld/StartReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "StartReplication"));
            self.inner.unary(req, path, codec).await
        }
        /// StopReplication stops replication on the specified tablet.
        pub async fn stop_replication(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::StopReplicationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::StopReplicationResponse>,
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
                "/vtctlservice.Vtctld/StopReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "StopReplication"));
            self.inner.unary(req, path, codec).await
        }
        /// TabletExternallyReparented changes metadata in the topology server to
        /// acknowledge a shard primary change performed by an external tool (e.g.
        /// orchestrator).
        ///
        /// See the Reparenting guide for more information:
        /// https://vitess.io/docs/user-guides/configuration-advanced/reparenting/#external-reparenting.
        pub async fn tablet_externally_reparented(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::TabletExternallyReparentedRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::TabletExternallyReparentedResponse>,
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
                "/vtctlservice.Vtctld/TabletExternallyReparented",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "TabletExternallyReparented"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// UpdateCellInfo updates the content of a CellInfo with the provided
        /// parameters. Empty values are ignored. If the cell does not exist, the
        /// CellInfo will be created.
        pub async fn update_cell_info(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::UpdateCellInfoRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::UpdateCellInfoResponse>,
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
                "/vtctlservice.Vtctld/UpdateCellInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "UpdateCellInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateCellsAlias updates the content of a CellsAlias with the provided
        /// parameters. Empty values are ignored. If the alias does not exist, the
        /// CellsAlias will be created.
        pub async fn update_cells_alias(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::UpdateCellsAliasRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::UpdateCellsAliasResponse>,
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
                "/vtctlservice.Vtctld/UpdateCellsAlias",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "UpdateCellsAlias"));
            self.inner.unary(req, path, codec).await
        }
        /// Validate validates that all nodes from the global replication graph are
        /// reachable, and that all tablets in discoverable cells are consistent.
        pub async fn validate(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::ValidateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ValidateResponse>,
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
                "/vtctlservice.Vtctld/Validate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "Validate"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidateKeyspace validates that all nodes reachable from the specified
        /// keyspace are consistent.
        pub async fn validate_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ValidateKeyspaceRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ValidateKeyspaceResponse>,
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
                "/vtctlservice.Vtctld/ValidateKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ValidateKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidateSchemaKeyspace validates that the schema on the primary tablet for shard 0 matches the schema on all of the other tablets in the keyspace.
        pub async fn validate_schema_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ValidateSchemaKeyspaceRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ValidateSchemaKeyspaceResponse>,
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
                "/vtctlservice.Vtctld/ValidateSchemaKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "ValidateSchemaKeyspace"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ValidateShard validates that all nodes reachable from the specified shard
        /// are consistent.
        pub async fn validate_shard(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ValidateShardRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ValidateShardResponse>,
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
                "/vtctlservice.Vtctld/ValidateShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ValidateShard"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidateVersionKeyspace validates that the version on the primary of shard 0 matches all of the other tablets in the keyspace.
        pub async fn validate_version_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ValidateVersionKeyspaceRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ValidateVersionKeyspaceResponse>,
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
                "/vtctlservice.Vtctld/ValidateVersionKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtctlservice.Vtctld", "ValidateVersionKeyspace"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ValidateVersionShard validates that the version on the primary matches all of the replicas.
        pub async fn validate_version_shard(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ValidateVersionShardRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ValidateVersionShardResponse>,
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
                "/vtctlservice.Vtctld/ValidateVersionShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ValidateVersionShard"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidateVSchema compares the schema of each primary tablet in "keyspace/shards..." to the vschema and errs if there are differences.
        pub async fn validate_v_schema(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::ValidateVSchemaRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::ValidateVSchemaResponse>,
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
                "/vtctlservice.Vtctld/ValidateVSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "ValidateVSchema"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn v_diff_create(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::VDiffCreateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::VDiffCreateResponse>,
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
                "/vtctlservice.Vtctld/VDiffCreate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "VDiffCreate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn v_diff_delete(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::VDiffDeleteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::VDiffDeleteResponse>,
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
                "/vtctlservice.Vtctld/VDiffDelete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "VDiffDelete"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn v_diff_resume(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::VDiffResumeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::VDiffResumeResponse>,
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
                "/vtctlservice.Vtctld/VDiffResume",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "VDiffResume"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn v_diff_show(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::VDiffShowRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::VDiffShowResponse>,
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
                "/vtctlservice.Vtctld/VDiffShow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "VDiffShow"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn v_diff_stop(
            &mut self,
            request: impl tonic::IntoRequest<super::super::vtctldata::VDiffStopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::VDiffStopResponse>,
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
                "/vtctlservice.Vtctld/VDiffStop",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "VDiffStop"));
            self.inner.unary(req, path, codec).await
        }
        /// WorkflowDelete deletes a vreplication workflow.
        pub async fn workflow_delete(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::WorkflowDeleteRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::WorkflowDeleteResponse>,
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
                "/vtctlservice.Vtctld/WorkflowDelete",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "WorkflowDelete"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn workflow_status(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::WorkflowStatusRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::WorkflowStatusResponse>,
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
                "/vtctlservice.Vtctld/WorkflowStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "WorkflowStatus"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn workflow_switch_traffic(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::WorkflowSwitchTrafficRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::WorkflowSwitchTrafficResponse>,
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
                "/vtctlservice.Vtctld/WorkflowSwitchTraffic",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "WorkflowSwitchTraffic"));
            self.inner.unary(req, path, codec).await
        }
        /// WorkflowUpdate updates the configuration of a vreplication workflow
        /// using the provided updated parameters.
        pub async fn workflow_update(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::vtctldata::WorkflowUpdateRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::super::vtctldata::WorkflowUpdateResponse>,
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
                "/vtctlservice.Vtctld/WorkflowUpdate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtctlservice.Vtctld", "WorkflowUpdate"));
            self.inner.unary(req, path, codec).await
        }
    }
}
