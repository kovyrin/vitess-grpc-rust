/// Status is the replication status for MySQL/MariaDB/File-based. Returned by a
/// flavor-specific command and parsed into a Position and fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub replication_lag_seconds: u32,
    #[prost(string, tag = "5")]
    pub source_host: ::prost::alloc::string::String,
    #[prost(int32, tag = "6")]
    pub source_port: i32,
    #[prost(int32, tag = "7")]
    pub connect_retry: i32,
    /// RelayLogPosition will be empty for flavors that do not support returning the full GTIDSet from the relay log, such as MariaDB.
    #[prost(string, tag = "8")]
    pub relay_log_position: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub file_position: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub relay_log_source_binlog_equivalent_position: ::prost::alloc::string::String,
    #[prost(uint32, tag = "11")]
    pub source_server_id: u32,
    #[prost(string, tag = "12")]
    pub source_uuid: ::prost::alloc::string::String,
    #[prost(int32, tag = "13")]
    pub io_state: i32,
    #[prost(string, tag = "14")]
    pub last_io_error: ::prost::alloc::string::String,
    #[prost(int32, tag = "15")]
    pub sql_state: i32,
    #[prost(string, tag = "16")]
    pub last_sql_error: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub relay_log_file_position: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub source_user: ::prost::alloc::string::String,
    #[prost(uint32, tag = "19")]
    pub sql_delay: u32,
    #[prost(bool, tag = "20")]
    pub auto_position: bool,
    #[prost(bool, tag = "21")]
    pub using_gtid: bool,
    #[prost(bool, tag = "22")]
    pub has_replication_filters: bool,
    #[prost(bool, tag = "23")]
    pub ssl_allowed: bool,
    #[prost(bool, tag = "24")]
    pub replication_lag_unknown: bool,
}
/// StopReplicationStatus represents the replication status before calling StopReplication, and the replication status collected immediately after
/// calling StopReplication.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationStatus {
    #[prost(message, optional, tag = "1")]
    pub before: ::core::option::Option<Status>,
    #[prost(message, optional, tag = "2")]
    pub after: ::core::option::Option<Status>,
}
/// PrimaryStatus is the replication status for a MySQL primary (returned by 'show master status').
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrimaryStatus {
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub file_position: ::prost::alloc::string::String,
}
/// FullStatus contains the full status of MySQL including the replication information, semi-sync information, GTID information among others
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullStatus {
    #[prost(uint32, tag = "1")]
    pub server_id: u32,
    #[prost(string, tag = "2")]
    pub server_uuid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub replication_status: ::core::option::Option<Status>,
    #[prost(message, optional, tag = "4")]
    pub primary_status: ::core::option::Option<PrimaryStatus>,
    #[prost(string, tag = "5")]
    pub gtid_purged: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub version_comment: ::prost::alloc::string::String,
    #[prost(bool, tag = "8")]
    pub read_only: bool,
    #[prost(string, tag = "9")]
    pub gtid_mode: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub binlog_format: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub binlog_row_image: ::prost::alloc::string::String,
    #[prost(bool, tag = "12")]
    pub log_bin_enabled: bool,
    #[prost(bool, tag = "13")]
    pub log_replica_updates: bool,
    #[prost(bool, tag = "14")]
    pub semi_sync_primary_enabled: bool,
    #[prost(bool, tag = "15")]
    pub semi_sync_replica_enabled: bool,
    #[prost(bool, tag = "16")]
    pub semi_sync_primary_status: bool,
    #[prost(bool, tag = "17")]
    pub semi_sync_replica_status: bool,
    #[prost(uint32, tag = "18")]
    pub semi_sync_primary_clients: u32,
    #[prost(uint64, tag = "19")]
    pub semi_sync_primary_timeout: u64,
    #[prost(uint32, tag = "20")]
    pub semi_sync_wait_for_replica_count: u32,
}
/// StopReplicationMode is used to provide controls over how replication is stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopReplicationMode {
    Ioandsqlthread = 0,
    Iothreadonly = 1,
}
impl StopReplicationMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopReplicationMode::Ioandsqlthread => "IOANDSQLTHREAD",
            StopReplicationMode::Iothreadonly => "IOTHREADONLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IOANDSQLTHREAD" => Some(Self::Ioandsqlthread),
            "IOTHREADONLY" => Some(Self::Iothreadonly),
            _ => None,
        }
    }
}
