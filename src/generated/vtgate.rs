/// Session objects are exchanged like cookies through various
/// calls to VTGate. The behavior differs between V2 & V3 APIs.
/// V3 APIs are Execute, ExecuteBatch and StreamExecute. All
/// other APIs are V2. For the V3 APIs, the session
/// must be sent with every call to Execute or ExecuteBatch.
/// For the V2 APIs, Begin does not accept a session. It instead
/// returns a brand new one with in_transaction set to true.
/// After a call to Commit or Rollback, the session can be
/// discarded. If you're not in a transaction, Session is
/// an optional parameter for the V2 APIs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    /// in_transaction is set to true if the session is in a transaction.
    #[prost(bool, tag = "1")]
    pub in_transaction: bool,
    /// shard_sessions keep track of per-shard transaction info.
    #[prost(message, repeated, tag = "2")]
    pub shard_sessions: ::prost::alloc::vec::Vec<session::ShardSession>,
    /// autocommit specifies if the session is in autocommit mode.
    /// This is used only for V3.
    #[prost(bool, tag = "4")]
    pub autocommit: bool,
    /// target_string is the target expressed as a string. Valid
    /// names are: keyspace:shard@target, keyspace@target or @target.
    /// This is used only for V3.
    #[prost(string, tag = "5")]
    pub target_string: ::prost::alloc::string::String,
    /// options is used only for V3.
    #[prost(message, optional, tag = "6")]
    pub options: ::core::option::Option<super::query::ExecuteOptions>,
    /// transaction_mode specifies the current transaction mode.
    #[prost(enumeration = "TransactionMode", tag = "7")]
    pub transaction_mode: i32,
    /// warnings contains non-fatal warnings from the previous query
    #[prost(message, repeated, tag = "8")]
    pub warnings: ::prost::alloc::vec::Vec<super::query::QueryWarning>,
    /// pre_sessions contains sessions that have to be committed first.
    #[prost(message, repeated, tag = "9")]
    pub pre_sessions: ::prost::alloc::vec::Vec<session::ShardSession>,
    /// post_sessions contains sessions that have to be committed last.
    #[prost(message, repeated, tag = "10")]
    pub post_sessions: ::prost::alloc::vec::Vec<session::ShardSession>,
    /// last_insert_id keeps track of the last seen insert_id for this session
    #[prost(uint64, tag = "11")]
    pub last_insert_id: u64,
    /// found_rows keeps track of how many rows the last query returned
    #[prost(uint64, tag = "12")]
    pub found_rows: u64,
    /// user_defined_variables contains all the @variables defined for this session
    #[prost(map = "string, message", tag = "13")]
    pub user_defined_variables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::query::BindVariable,
    >,
    /// system_variables keeps track of all session variables set for this connection
    /// TODO: systay should we keep this so we can apply it ordered?
    #[prost(map = "string, string", tag = "14")]
    pub system_variables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// row_count keeps track of the last seen rows affected for this session
    #[prost(int64, tag = "15")]
    pub row_count: i64,
    /// Stores savepoint and release savepoint calls inside a transaction
    /// and is reset once transaction is committed or rolled back.
    #[prost(string, repeated, tag = "16")]
    pub savepoints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// in_reserved_conn is set to true if the session should be using reserved connections.
    #[prost(bool, tag = "17")]
    pub in_reserved_conn: bool,
    /// lock_session keep tracks of shard on which the lock query is sent.
    #[prost(message, optional, tag = "18")]
    pub lock_session: ::core::option::Option<session::ShardSession>,
    /// last_lock_heartbeat keep tracks of when last lock heartbeat was sent.
    #[prost(int64, tag = "19")]
    pub last_lock_heartbeat: i64,
    /// read_after_write tracks the ReadAfterWrite settings for this session.
    #[prost(message, optional, tag = "20")]
    pub read_after_write: ::core::option::Option<ReadAfterWrite>,
    /// DDL strategy
    #[prost(string, tag = "21")]
    pub ddl_strategy: ::prost::alloc::string::String,
    /// Session UUID
    #[prost(string, tag = "22")]
    pub session_uuid: ::prost::alloc::string::String,
    /// enable_system_settings defines if we can use reserved connections.
    #[prost(bool, tag = "23")]
    pub enable_system_settings: bool,
    #[prost(map = "string, int64", tag = "24")]
    pub advisory_lock: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
/// Nested message and enum types in `Session`.
pub mod session {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShardSession {
        #[prost(message, optional, tag = "1")]
        pub target: ::core::option::Option<super::super::query::Target>,
        #[prost(int64, tag = "2")]
        pub transaction_id: i64,
        #[prost(message, optional, tag = "3")]
        pub tablet_alias: ::core::option::Option<super::super::topodata::TabletAlias>,
        /// reserved connection if a dedicated connection is needed
        #[prost(int64, tag = "4")]
        pub reserved_id: i64,
        #[prost(bool, tag = "5")]
        pub vindex_only: bool,
    }
}
/// ReadAfterWrite contains information regarding gtid set and timeout
/// Also if the gtid information needs to be passed to client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAfterWrite {
    #[prost(string, tag = "1")]
    pub read_after_write_gtid: ::prost::alloc::string::String,
    #[prost(double, tag = "2")]
    pub read_after_write_timeout: f64,
    #[prost(bool, tag = "3")]
    pub session_track_gtids: bool,
}
/// ExecuteRequest is the payload to Execute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteRequest {
    /// caller_id identifies the caller. This is the effective caller ID,
    /// set by the application to further identify the caller.
    #[prost(message, optional, tag = "1")]
    pub caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    /// session carries the session state.
    #[prost(message, optional, tag = "2")]
    pub session: ::core::option::Option<Session>,
    /// query is the query and bind variables to execute.
    #[prost(message, optional, tag = "3")]
    pub query: ::core::option::Option<super::query::BoundQuery>,
}
/// ExecuteResponse is the returned value from Execute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteResponse {
    /// error contains an application level error if necessary. Note the
    /// session may have changed, even when an error is returned (for
    /// instance if a database integrity error happened).
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
    /// session is the updated session information.
    #[prost(message, optional, tag = "2")]
    pub session: ::core::option::Option<Session>,
    /// result contains the query result, only set if error is unset.
    #[prost(message, optional, tag = "3")]
    pub result: ::core::option::Option<super::query::QueryResult>,
}
/// ExecuteBatchRequest is the payload to ExecuteBatch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteBatchRequest {
    /// caller_id identifies the caller. This is the effective caller ID,
    /// set by the application to further identify the caller.
    #[prost(message, optional, tag = "1")]
    pub caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    /// session carries the session state.
    #[prost(message, optional, tag = "2")]
    pub session: ::core::option::Option<Session>,
    /// queries is a list of query and bind variables to execute.
    #[prost(message, repeated, tag = "3")]
    pub queries: ::prost::alloc::vec::Vec<super::query::BoundQuery>,
}
/// ExecuteBatchResponse is the returned value from ExecuteBatch.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteBatchResponse {
    /// error contains an application level error if necessary. Note the
    /// session may have changed, even when an error is returned (for
    /// instance if a database integrity error happened).
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
    /// session is the updated session information.
    #[prost(message, optional, tag = "2")]
    pub session: ::core::option::Option<Session>,
    /// results contains the query results, only set if application level error is unset.
    #[prost(message, repeated, tag = "3")]
    pub results: ::prost::alloc::vec::Vec<super::query::ResultWithError>,
}
/// StreamExecuteRequest is the payload to StreamExecute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamExecuteRequest {
    /// caller_id identifies the caller. This is the effective caller ID,
    /// set by the application to further identify the caller.
    #[prost(message, optional, tag = "1")]
    pub caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    /// query is the query and bind variables to execute.
    #[prost(message, optional, tag = "2")]
    pub query: ::core::option::Option<super::query::BoundQuery>,
    /// session carries the session state.
    #[prost(message, optional, tag = "6")]
    pub session: ::core::option::Option<Session>,
}
/// StreamExecuteResponse is the returned value from StreamExecute.
/// The session is currently not returned because StreamExecute is
/// not expected to modify it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamExecuteResponse {
    /// result contains the result data.
    /// The first value contains only Fields information.
    /// The next values contain the actual rows, a few values per result.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<super::query::QueryResult>,
}
/// ResolveTransactionRequest is the payload to ResolveTransaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveTransactionRequest {
    /// caller_id identifies the caller. This is the effective caller ID,
    /// set by the application to further identify the caller.
    #[prost(message, optional, tag = "1")]
    pub caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    /// dtid is the dtid of the transaction to be resolved.
    #[prost(string, tag = "2")]
    pub dtid: ::prost::alloc::string::String,
}
/// ResolveTransactionResponse is the returned value from Rollback.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolveTransactionResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VStreamFlags {
    /// align streams
    #[prost(bool, tag = "1")]
    pub minimize_skew: bool,
    /// how often heartbeats must be sent when idle (seconds)
    #[prost(uint32, tag = "2")]
    pub heartbeat_interval: u32,
    /// stop streams on a reshard (journal event)
    #[prost(bool, tag = "3")]
    pub stop_on_reshard: bool,
    /// if specified, these cells (comma-separated) are used to pick source tablets from.
    /// defaults to the cell of the vtgate serving the VStream API.
    #[prost(string, tag = "4")]
    pub cells: ::prost::alloc::string::String,
}
/// VStreamRequest is the payload for VStream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VStreamRequest {
    #[prost(message, optional, tag = "1")]
    pub caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(enumeration = "super::topodata::TabletType", tag = "2")]
    pub tablet_type: i32,
    /// position specifies the starting point of the bin log positions
    /// as well as the keyspace-shards to pull events from.
    /// position is of the form 'ks1:0@MySQL56/<mysql_pos>|ks2:-80@MySQL56/<mysql_pos>'.
    #[prost(message, optional, tag = "3")]
    pub vgtid: ::core::option::Option<super::binlogdata::VGtid>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<super::binlogdata::Filter>,
    #[prost(message, optional, tag = "5")]
    pub flags: ::core::option::Option<VStreamFlags>,
}
/// VStreamResponse is streamed by VStream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VStreamResponse {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<super::binlogdata::VEvent>,
}
/// PrepareRequest is the payload to Prepare.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareRequest {
    /// caller_id identifies the caller. This is the effective caller ID,
    /// set by the application to further identify the caller.
    #[prost(message, optional, tag = "1")]
    pub caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    /// session carries the session state.
    #[prost(message, optional, tag = "2")]
    pub session: ::core::option::Option<Session>,
    /// query is the query and bind variables to execute.
    #[prost(message, optional, tag = "3")]
    pub query: ::core::option::Option<super::query::BoundQuery>,
}
/// PrepareResponse is the returned value from Prepare.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareResponse {
    /// error contains an application level error if necessary. Note the
    /// session may have changed, even when an error is returned (for
    /// instance if a database integrity error happened).
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
    /// session is the updated session information.
    #[prost(message, optional, tag = "2")]
    pub session: ::core::option::Option<Session>,
    /// fields contains the fields, only set if error is unset.
    #[prost(message, repeated, tag = "3")]
    pub fields: ::prost::alloc::vec::Vec<super::query::Field>,
}
/// CloseSessionRequest is the payload to CloseSession.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseSessionRequest {
    /// caller_id identifies the caller. This is the effective caller ID,
    /// set by the application to further identify the caller.
    #[prost(message, optional, tag = "1")]
    pub caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    /// session carries the session state.
    #[prost(message, optional, tag = "2")]
    pub session: ::core::option::Option<Session>,
}
/// CloseSessionResponse is the returned value from CloseSession.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseSessionResponse {
    /// error contains an application level error if necessary. Note the
    /// session may have changed, even when an error is returned (for
    /// instance if a database integrity error happened).
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
}
/// TransactionMode controls the execution of distributed transaction
/// across multiple shards.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionMode {
    /// UNSPECIFIED uses the transaction mode set by the VTGate flag 'transaction_mode'.
    Unspecified = 0,
    /// SINGLE disallows distributed transactions.
    Single = 1,
    /// MULTI allows distributed transactions with best effort commit.
    Multi = 2,
    /// TWOPC is for distributed transactions with atomic commits.
    Twopc = 3,
}
impl TransactionMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionMode::Unspecified => "UNSPECIFIED",
            TransactionMode::Single => "SINGLE",
            TransactionMode::Multi => "MULTI",
            TransactionMode::Twopc => "TWOPC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "SINGLE" => Some(Self::Single),
            "MULTI" => Some(Self::Multi),
            "TWOPC" => Some(Self::Twopc),
            _ => None,
        }
    }
}
/// CommitOrder is used to designate which of the ShardSessions
/// get used for transactions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommitOrder {
    /// NORMAL is the default commit order.
    Normal = 0,
    /// PRE is used to designate pre_sessions.
    Pre = 1,
    /// POST is used to designate post_sessions.
    Post = 2,
    /// AUTOCOMMIT is used to run the statement as autocommitted transaction.
    Autocommit = 3,
}
impl CommitOrder {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommitOrder::Normal => "NORMAL",
            CommitOrder::Pre => "PRE",
            CommitOrder::Post => "POST",
            CommitOrder::Autocommit => "AUTOCOMMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL" => Some(Self::Normal),
            "PRE" => Some(Self::Pre),
            "POST" => Some(Self::Post),
            "AUTOCOMMIT" => Some(Self::Autocommit),
            _ => None,
        }
    }
}
