#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableDefinition {
    /// the table name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// the SQL to run to create the table
    #[prost(string, tag = "2")]
    pub schema: ::prost::alloc::string::String,
    /// the columns in the order that will be used to dump and load the data
    #[prost(string, repeated, tag = "3")]
    pub columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// the primary key columns in the primary key order
    #[prost(string, repeated, tag = "4")]
    pub primary_key_columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// type is either mysqlctl.TableBaseTable or mysqlctl.TableView
    #[prost(string, tag = "5")]
    pub r#type: ::prost::alloc::string::String,
    /// how much space the data file takes.
    #[prost(uint64, tag = "6")]
    pub data_length: u64,
    /// approximate number of rows
    #[prost(uint64, tag = "7")]
    pub row_count: u64,
    /// column names along with their types.
    /// NOTE: this is a superset of columns.
    #[prost(message, repeated, tag = "8")]
    pub fields: ::prost::alloc::vec::Vec<super::query::Field>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaDefinition {
    #[prost(string, tag = "1")]
    pub database_schema: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub table_definitions: ::prost::alloc::vec::Vec<TableDefinition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchemaChangeResult {
    /// before_schema holds the schema before each change.
    #[prost(message, optional, tag = "1")]
    pub before_schema: ::core::option::Option<SchemaDefinition>,
    /// after_schema holds the schema after each change.
    #[prost(message, optional, tag = "2")]
    pub after_schema: ::core::option::Option<SchemaDefinition>,
}
/// UserPermission describes a single row in the mysql.user table
/// Primary key is Host+User
/// PasswordChecksum is the crc64 of the password, for security reasons
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserPermission {
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub password_checksum: u64,
    #[prost(map = "string, string", tag = "4")]
    pub privileges: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// DbPermission describes a single row in the mysql.db table
/// Primary key is Host+Db+User
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbPermission {
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub db: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub user: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "4")]
    pub privileges: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Permissions have all the rows in mysql.{user,db} tables,
/// (all rows are sorted by primary key)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permissions {
    #[prost(message, repeated, tag = "1")]
    pub user_permissions: ::prost::alloc::vec::Vec<UserPermission>,
    #[prost(message, repeated, tag = "2")]
    pub db_permissions: ::prost::alloc::vec::Vec<DbPermission>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
    #[prost(string, tag = "1")]
    pub payload: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
    #[prost(string, tag = "1")]
    pub payload: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SleepRequest {
    /// duration is in nanoseconds
    #[prost(int64, tag = "1")]
    pub duration: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SleepResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteHookRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub parameters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, string", tag = "3")]
    pub extra_env: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteHookResponse {
    #[prost(int64, tag = "1")]
    pub exit_status: i64,
    #[prost(string, tag = "2")]
    pub stdout: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub stderr: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaRequest {
    #[prost(string, repeated, tag = "1")]
    pub tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "2")]
    pub include_views: bool,
    #[prost(string, repeated, tag = "3")]
    pub exclude_tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// TableSchemaOnly specifies whether to limit the results to just table/view
    /// schema definition (CREATE TABLE/VIEW statements) and skip column/field information
    #[prost(bool, tag = "4")]
    pub table_schema_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaResponse {
    #[prost(message, optional, tag = "1")]
    pub schema_definition: ::core::option::Option<SchemaDefinition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermissionsRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermissionsResponse {
    #[prost(message, optional, tag = "1")]
    pub permissions: ::core::option::Option<Permissions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReadOnlyRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReadOnlyResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReadWriteRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReadWriteResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeTypeRequest {
    #[prost(enumeration = "super::topodata::TabletType", tag = "1")]
    pub tablet_type: i32,
    #[prost(bool, tag = "2")]
    pub semi_sync: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeTypeResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshStateRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshStateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunHealthCheckRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunHealthCheckResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemaRequest {
    /// wait_position allows scheduling a schema reload to occur after a
    /// given DDL has replicated to this server, by specifying a replication
    /// position to wait for. Leave empty to trigger the reload immediately.
    #[prost(string, tag = "1")]
    pub wait_position: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemaResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreflightSchemaRequest {
    #[prost(string, repeated, tag = "1")]
    pub changes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreflightSchemaResponse {
    /// change_results has for each change the schema before and after it.
    /// The number of elements is identical to the length of "changes" in the request.
    #[prost(message, repeated, tag = "1")]
    pub change_results: ::prost::alloc::vec::Vec<SchemaChangeResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplySchemaRequest {
    #[prost(string, tag = "1")]
    pub sql: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub force: bool,
    #[prost(bool, tag = "3")]
    pub allow_replication: bool,
    #[prost(message, optional, tag = "4")]
    pub before_schema: ::core::option::Option<SchemaDefinition>,
    #[prost(message, optional, tag = "5")]
    pub after_schema: ::core::option::Option<SchemaDefinition>,
    #[prost(string, tag = "6")]
    pub sql_mode: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplySchemaResponse {
    #[prost(message, optional, tag = "1")]
    pub before_schema: ::core::option::Option<SchemaDefinition>,
    #[prost(message, optional, tag = "2")]
    pub after_schema: ::core::option::Option<SchemaDefinition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockTablesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockTablesResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockTablesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockTablesResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteQueryRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub query: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub db_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub max_rows: u64,
    /// caller_id identifies the caller. This is the effective caller ID,
    /// set by the application to further identify the caller.
    #[prost(message, optional, tag = "4")]
    pub caller_id: ::core::option::Option<super::vtrpc::CallerId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteQueryResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<super::query::QueryResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFetchAsDbaRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub query: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub db_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub max_rows: u64,
    #[prost(bool, tag = "4")]
    pub disable_binlogs: bool,
    #[prost(bool, tag = "5")]
    pub reload_schema: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFetchAsDbaResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<super::query::QueryResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFetchAsAllPrivsRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub query: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub db_name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub max_rows: u64,
    #[prost(bool, tag = "4")]
    pub reload_schema: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFetchAsAllPrivsResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<super::query::QueryResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFetchAsAppRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub query: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub max_rows: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFetchAsAppResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<super::query::QueryResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStatusRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::replicationdata::Status>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrimaryStatusRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrimaryStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::replicationdata::PrimaryStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrimaryPositionRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrimaryPositionResponse {
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForPositionRequest {
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitForPositionResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationMinimumRequest {
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub wait_timeout: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationMinimumResponse {
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartReplicationRequest {
    #[prost(bool, tag = "1")]
    pub semi_sync: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartReplicationResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartReplicationUntilAfterRequest {
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub wait_timeout: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartReplicationUntilAfterResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReplicasRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReplicasResponse {
    #[prost(string, repeated, tag = "1")]
    pub addrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetReplicationRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetReplicationResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VReplicationExecRequest {
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VReplicationExecResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<super::query::QueryResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VReplicationWaitForPosRequest {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub position: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VReplicationWaitForPosResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitPrimaryRequest {
    #[prost(bool, tag = "1")]
    pub semi_sync: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitPrimaryResponse {
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopulateReparentJournalRequest {
    #[prost(int64, tag = "1")]
    pub time_created_ns: i64,
    #[prost(string, tag = "2")]
    pub action_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub primary_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, tag = "4")]
    pub replication_position: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PopulateReparentJournalResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitReplicaRequest {
    #[prost(message, optional, tag = "1")]
    pub parent: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, tag = "2")]
    pub replication_position: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub time_created_ns: i64,
    #[prost(bool, tag = "4")]
    pub semi_sync: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitReplicaResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DemotePrimaryRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DemotePrimaryResponse {
    /// PrimaryStatus represents the response from calling `SHOW MASTER STATUS` on a primary that has been demoted.
    #[prost(message, optional, tag = "2")]
    pub primary_status: ::core::option::Option<super::replicationdata::PrimaryStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndoDemotePrimaryRequest {
    #[prost(bool, tag = "1")]
    pub semi_sync: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndoDemotePrimaryResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaWasPromotedRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaWasPromotedResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetReplicationParametersRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetReplicationParametersResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullStatusRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::replicationdata::FullStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReplicationSourceRequest {
    #[prost(message, optional, tag = "1")]
    pub parent: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(int64, tag = "2")]
    pub time_created_ns: i64,
    #[prost(bool, tag = "3")]
    pub force_start_replication: bool,
    #[prost(string, tag = "4")]
    pub wait_position: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub semi_sync: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReplicationSourceResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaWasRestartedRequest {
    /// the parent alias the tablet should have
    #[prost(message, optional, tag = "1")]
    pub parent: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaWasRestartedResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationAndGetStatusRequest {
    #[prost(enumeration = "super::replicationdata::StopReplicationMode", tag = "1")]
    pub stop_replication_mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationAndGetStatusResponse {
    /// Status represents the replication status call right before, and right after telling the replica to stop.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::replicationdata::StopReplicationStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteReplicaRequest {
    #[prost(bool, tag = "1")]
    pub semi_sync: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteReplicaResponse {
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRequest {
    #[prost(int64, tag = "1")]
    pub concurrency: i64,
    #[prost(bool, tag = "2")]
    pub allow_primary: bool,
    /// IncrementalFromPos indicates a position of a previous backup. When this value is non-empty
    /// then the backup becomes incremental and applies as of given position.
    #[prost(string, tag = "3")]
    pub incremental_from_pos: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupResponse {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreFromBackupRequest {
    #[prost(message, optional, tag = "1")]
    pub backup_time: ::core::option::Option<super::vttime::Time>,
    /// RestoreToPos indicates a position for a point-in-time recovery. The recovery
    /// is expected to utilize one full backup, followed by zero or more incremental backups,
    /// that reach the precise desired position
    #[prost(string, tag = "2")]
    pub restore_to_pos: ::prost::alloc::string::String,
    /// Dry run does not actually performs the restore, but validates the steps and availability of backups
    #[prost(bool, tag = "3")]
    pub dry_run: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreFromBackupResponse {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VDiffRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub workflow: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub action: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub action_arg: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub vdiff_uuid: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub options: ::core::option::Option<VDiffOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VDiffResponse {
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(message, optional, tag = "2")]
    pub output: ::core::option::Option<super::query::QueryResult>,
    #[prost(string, tag = "3")]
    pub vdiff_uuid: ::prost::alloc::string::String,
}
/// options that influence the tablet selected by the picker for streaming data from
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VDiffPickerOptions {
    #[prost(string, tag = "1")]
    pub tablet_types: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_cell: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub target_cell: ::prost::alloc::string::String,
}
/// options that only influence how vdiff differences are reported
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VDiffReportOptions {
    #[prost(bool, tag = "1")]
    pub only_pks: bool,
    #[prost(bool, tag = "2")]
    pub debug_query: bool,
    #[prost(string, tag = "3")]
    pub format: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VDiffCoreOptions {
    #[prost(string, tag = "1")]
    pub tables: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub auto_retry: bool,
    #[prost(int64, tag = "3")]
    pub max_rows: i64,
    #[prost(bool, tag = "4")]
    pub checksum: bool,
    #[prost(int64, tag = "5")]
    pub sample_pct: i64,
    #[prost(int64, tag = "6")]
    pub timeout_seconds: i64,
    #[prost(int64, tag = "7")]
    pub max_extra_rows_to_compare: i64,
    #[prost(bool, tag = "8")]
    pub update_table_stats: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VDiffOptions {
    #[prost(message, optional, tag = "1")]
    pub picker_options: ::core::option::Option<VDiffPickerOptions>,
    #[prost(message, optional, tag = "2")]
    pub core_options: ::core::option::Option<VDiffCoreOptions>,
    #[prost(message, optional, tag = "3")]
    pub report_options: ::core::option::Option<VDiffReportOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVrWorkflowRequest {
    #[prost(string, tag = "1")]
    pub workflow: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub tablet_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "super::binlogdata::OnDdlAction", tag = "4")]
    pub on_ddl: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateVrWorkflowResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<super::query::QueryResult>,
}
