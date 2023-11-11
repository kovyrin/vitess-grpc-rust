#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartRequest {
    #[prost(string, repeated, tag = "1")]
    pub mysqld_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShutdownRequest {
    #[prost(bool, tag = "1")]
    pub wait_for_mysqld: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShutdownResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunMysqlUpgradeRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunMysqlUpgradeResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyBinlogFileRequest {
    #[prost(string, tag = "1")]
    pub binlog_file_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub binlog_restore_position: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub binlog_restore_datetime: ::core::option::Option<super::vttime::Time>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyBinlogFileResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadBinlogFilesTimestampsRequest {
    #[prost(string, repeated, tag = "1")]
    pub binlog_file_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadBinlogFilesTimestampsResponse {
    /// FirstTimestamp is the timestamp of the first found transaction searching in order of given binlog files
    #[prost(message, optional, tag = "1")]
    pub first_timestamp: ::core::option::Option<super::vttime::Time>,
    /// FirstTimestampBinlog is the name of the binary log in which the first timestamp is found
    #[prost(string, tag = "2")]
    pub first_timestamp_binlog: ::prost::alloc::string::String,
    /// LastTimestamp is the timestamp of the last found transaction in given binlog files
    #[prost(message, optional, tag = "3")]
    pub last_timestamp: ::core::option::Option<super::vttime::Time>,
    /// LastTimestampBinlog is the name of the binary log in which the last timestamp is found
    #[prost(string, tag = "4")]
    pub last_timestamp_binlog: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReinitConfigRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReinitConfigResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshConfigRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshConfigResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionStringRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionStringResponse {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
/// BackupInfo is the read-only attributes of a mysqlctl/backupstorage.BackupHandle.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub directory: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub shard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<super::vttime::Time>,
    /// Engine is the name of the backupengine implementation used to create
    /// this backup.
    #[prost(string, tag = "7")]
    pub engine: ::prost::alloc::string::String,
    #[prost(enumeration = "backup_info::Status", tag = "8")]
    pub status: i32,
}
/// Nested message and enum types in `BackupInfo`.
pub mod backup_info {
    /// Status is an enum representing the possible status of a backup.
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
        Incomplete = 1,
        Complete = 2,
        /// A backup status of INVALID should be set if the backup is complete
        /// but unusable in some way (partial upload, corrupt file, etc).
        Invalid = 3,
        /// A backup status of VALID should be set if the backup is both
        /// complete and usuable.
        Valid = 4,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unknown => "UNKNOWN",
                Status::Incomplete => "INCOMPLETE",
                Status::Complete => "COMPLETE",
                Status::Invalid => "INVALID",
                Status::Valid => "VALID",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "INCOMPLETE" => Some(Self::Incomplete),
                "COMPLETE" => Some(Self::Complete),
                "INVALID" => Some(Self::Invalid),
                "VALID" => Some(Self::Valid),
                _ => None,
            }
        }
    }
}
/// Generated client implementations.
pub mod mysql_ctl_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// MysqlCtl is the service definition
    #[derive(Debug, Clone)]
    pub struct MysqlCtlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MysqlCtlClient<tonic::transport::Channel> {
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
    impl<T> MysqlCtlClient<T>
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
        ) -> MysqlCtlClient<InterceptedService<T, F>>
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
            MysqlCtlClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn start(
            &mut self,
            request: impl tonic::IntoRequest<super::StartRequest>,
        ) -> std::result::Result<tonic::Response<super::StartResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/mysqlctl.MysqlCtl/Start");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("mysqlctl.MysqlCtl", "Start"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn shutdown(
            &mut self,
            request: impl tonic::IntoRequest<super::ShutdownRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShutdownResponse>,
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
                "/mysqlctl.MysqlCtl/Shutdown",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("mysqlctl.MysqlCtl", "Shutdown"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn run_mysql_upgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::RunMysqlUpgradeRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RunMysqlUpgradeResponse>,
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
                "/mysqlctl.MysqlCtl/RunMysqlUpgrade",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("mysqlctl.MysqlCtl", "RunMysqlUpgrade"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn apply_binlog_file(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyBinlogFileRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ApplyBinlogFileResponse>,
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
                "/mysqlctl.MysqlCtl/ApplyBinlogFile",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("mysqlctl.MysqlCtl", "ApplyBinlogFile"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_binlog_files_timestamps(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadBinlogFilesTimestampsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadBinlogFilesTimestampsResponse>,
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
                "/mysqlctl.MysqlCtl/ReadBinlogFilesTimestamps",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("mysqlctl.MysqlCtl", "ReadBinlogFilesTimestamps"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn reinit_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ReinitConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReinitConfigResponse>,
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
                "/mysqlctl.MysqlCtl/ReinitConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("mysqlctl.MysqlCtl", "ReinitConfig"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn refresh_config(
            &mut self,
            request: impl tonic::IntoRequest<super::RefreshConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RefreshConfigResponse>,
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
                "/mysqlctl.MysqlCtl/RefreshConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("mysqlctl.MysqlCtl", "RefreshConfig"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn version_string(
            &mut self,
            request: impl tonic::IntoRequest<super::VersionStringRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VersionStringResponse>,
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
                "/mysqlctl.MysqlCtl/VersionString",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("mysqlctl.MysqlCtl", "VersionString"));
            self.inner.unary(req, path, codec).await
        }
    }
}
