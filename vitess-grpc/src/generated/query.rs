/// Target describes what the client expects the tablet is.
/// If the tablet does not match, an error is returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Target {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(enumeration = "super::topodata::TabletType", tag = "3")]
    pub tablet_type: i32,
    /// cell is used for routing queries between vtgate and vttablets. It
    /// is not used when Target is part of the Session sent by the client.
    #[prost(string, tag = "4")]
    pub cell: ::prost::alloc::string::String,
}
/// VTGateCallerID is sent by VTGate to VTTablet to describe the
/// caller. If possible, this information is secure. For instance,
/// if using unique certificates that guarantee that VTGate->VTTablet
/// traffic cannot be spoofed, then VTTablet can trust this information,
/// and VTTablet will use it for tablet ACLs, for instance.
/// Because of this security guarantee, this is different than the CallerID
/// structure, which is not secure at all, because it is provided
/// by the Vitess client.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VtGateCallerId {
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventToken is a structure that describes a point in time in a
/// replication stream on one shard. The most recent known replication
/// position can be retrieved from vttablet when executing a query. It
/// is also sent with the replication streams from the binlog service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventToken {
    /// timestamp is the MySQL timestamp of the statements. Seconds since Epoch.
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    /// The shard name that applied the statements. Note this is not set when
    /// streaming from a vttablet. It is only used on the client -> vtgate link.
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    /// The position on the replication stream after this statement was applied.
    /// It is not the transaction ID / GTID, but the position / GTIDSet.
    #[prost(string, tag = "3")]
    pub position: ::prost::alloc::string::String,
}
/// Value represents a typed value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(enumeration = "Type", tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// BindVariable represents a single bind variable in a Query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BindVariable {
    #[prost(enumeration = "Type", tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// values are set if type is TUPLE.
    #[prost(message, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
/// BoundQuery is a query with its bind variables
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundQuery {
    /// sql is the SQL query to execute
    #[prost(string, tag = "1")]
    pub sql: ::prost::alloc::string::String,
    /// bind_variables is a map of all bind variables to expand in the query.
    /// nil values are not allowed. Use NULL_TYPE to express a NULL value.
    #[prost(map = "string, message", tag = "2")]
    pub bind_variables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        BindVariable,
    >,
}
/// ExecuteOptions is passed around for all Execute calls.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteOptions {
    /// Controls what fields are returned in Field message responses from mysql, i.e.
    /// field name, table name, etc. This is an optimization for high-QPS queries where
    /// the client knows what it's getting
    #[prost(enumeration = "execute_options::IncludedFields", tag = "4")]
    pub included_fields: i32,
    /// client_rows_found specifies if rows_affected should return
    /// rows found instead of rows affected. Behavior is defined
    /// by MySQL's CLIENT_FOUND_ROWS flag.
    #[prost(bool, tag = "5")]
    pub client_found_rows: bool,
    /// workload specifies the type of workload:
    /// OLTP: DMLs allowed, results have row count limit, and
    /// query timeouts are shorter.
    /// OLAP: DMLS not allowed, no limit on row count, timeouts
    /// can be as high as desired.
    /// DBA: no limit on rowcount or timeout, all queries allowed
    /// but intended for long DMLs and DDLs.
    #[prost(enumeration = "execute_options::Workload", tag = "6")]
    pub workload: i32,
    /// sql_select_limit sets an implicit limit on all select statements. Since
    /// vitess also sets a rowcount limit on queries, the smallest value wins.
    #[prost(int64, tag = "8")]
    pub sql_select_limit: i64,
    #[prost(enumeration = "execute_options::TransactionIsolation", tag = "9")]
    pub transaction_isolation: i32,
    /// skip_query_plan_cache specifies if the query plan should be cached by vitess.
    /// By default all query plans are cached.
    #[prost(bool, tag = "10")]
    pub skip_query_plan_cache: bool,
    /// PlannerVersion specifies which planner to use.
    /// If DEFAULT is chosen, whatever vtgate was started with will be used
    #[prost(enumeration = "execute_options::PlannerVersion", tag = "11")]
    pub planner_version: i32,
    /// has_created_temp_tables signals whether plans created in this session should be cached or not
    /// if the user has created temp tables, Vitess will not reuse plans created for this session in other sessions.
    /// The current session can still use other sessions cached plans.
    #[prost(bool, tag = "12")]
    pub has_created_temp_tables: bool,
    #[prost(enumeration = "execute_options::Consolidator", tag = "13")]
    pub consolidator: i32,
    /// TransactionAccessMode specifies the access modes to be used while starting the transaction i.e. READ WRITE/READ ONLY/WITH CONSISTENT SNAPSHOT
    /// If not specified, the transaction will be started with the default access mode on the connection.
    #[prost(
        enumeration = "execute_options::TransactionAccessMode",
        repeated,
        tag = "14"
    )]
    pub transaction_access_mode: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `ExecuteOptions`.
pub mod execute_options {
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
    pub enum IncludedFields {
        TypeAndName = 0,
        TypeOnly = 1,
        All = 2,
    }
    impl IncludedFields {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IncludedFields::TypeAndName => "TYPE_AND_NAME",
                IncludedFields::TypeOnly => "TYPE_ONLY",
                IncludedFields::All => "ALL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_AND_NAME" => Some(Self::TypeAndName),
                "TYPE_ONLY" => Some(Self::TypeOnly),
                "ALL" => Some(Self::All),
                _ => None,
            }
        }
    }
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
    pub enum Workload {
        Unspecified = 0,
        Oltp = 1,
        Olap = 2,
        Dba = 3,
    }
    impl Workload {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Workload::Unspecified => "UNSPECIFIED",
                Workload::Oltp => "OLTP",
                Workload::Olap => "OLAP",
                Workload::Dba => "DBA",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "OLTP" => Some(Self::Oltp),
                "OLAP" => Some(Self::Olap),
                "DBA" => Some(Self::Dba),
                _ => None,
            }
        }
    }
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
    pub enum TransactionIsolation {
        Default = 0,
        RepeatableRead = 1,
        ReadCommitted = 2,
        ReadUncommitted = 3,
        Serializable = 4,
        /// This is not an "official" transaction level but it will do a
        /// START TRANSACTION WITH CONSISTENT SNAPSHOT, READ ONLY
        ConsistentSnapshotReadOnly = 5,
        /// This not an "official" transaction level, it will send queries to mysql
        /// without wrapping them in a transaction
        Autocommit = 6,
    }
    impl TransactionIsolation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TransactionIsolation::Default => "DEFAULT",
                TransactionIsolation::RepeatableRead => "REPEATABLE_READ",
                TransactionIsolation::ReadCommitted => "READ_COMMITTED",
                TransactionIsolation::ReadUncommitted => "READ_UNCOMMITTED",
                TransactionIsolation::Serializable => "SERIALIZABLE",
                TransactionIsolation::ConsistentSnapshotReadOnly => {
                    "CONSISTENT_SNAPSHOT_READ_ONLY"
                }
                TransactionIsolation::Autocommit => "AUTOCOMMIT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT" => Some(Self::Default),
                "REPEATABLE_READ" => Some(Self::RepeatableRead),
                "READ_COMMITTED" => Some(Self::ReadCommitted),
                "READ_UNCOMMITTED" => Some(Self::ReadUncommitted),
                "SERIALIZABLE" => Some(Self::Serializable),
                "CONSISTENT_SNAPSHOT_READ_ONLY" => Some(Self::ConsistentSnapshotReadOnly),
                "AUTOCOMMIT" => Some(Self::Autocommit),
                _ => None,
            }
        }
    }
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
    pub enum PlannerVersion {
        DefaultPlanner = 0,
        V3 = 1,
        Gen4 = 2,
        Gen4Greedy = 3,
        Gen4Left2Right = 4,
        Gen4WithFallback = 5,
        Gen4CompareV3 = 6,
    }
    impl PlannerVersion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PlannerVersion::DefaultPlanner => "DEFAULT_PLANNER",
                PlannerVersion::V3 => "V3",
                PlannerVersion::Gen4 => "Gen4",
                PlannerVersion::Gen4Greedy => "Gen4Greedy",
                PlannerVersion::Gen4Left2Right => "Gen4Left2Right",
                PlannerVersion::Gen4WithFallback => "Gen4WithFallback",
                PlannerVersion::Gen4CompareV3 => "Gen4CompareV3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEFAULT_PLANNER" => Some(Self::DefaultPlanner),
                "V3" => Some(Self::V3),
                "Gen4" => Some(Self::Gen4),
                "Gen4Greedy" => Some(Self::Gen4Greedy),
                "Gen4Left2Right" => Some(Self::Gen4Left2Right),
                "Gen4WithFallback" => Some(Self::Gen4WithFallback),
                "Gen4CompareV3" => Some(Self::Gen4CompareV3),
                _ => None,
            }
        }
    }
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
    pub enum Consolidator {
        Unspecified = 0,
        Disabled = 1,
        Enabled = 2,
        EnabledReplicas = 3,
    }
    impl Consolidator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Consolidator::Unspecified => "CONSOLIDATOR_UNSPECIFIED",
                Consolidator::Disabled => "CONSOLIDATOR_DISABLED",
                Consolidator::Enabled => "CONSOLIDATOR_ENABLED",
                Consolidator::EnabledReplicas => "CONSOLIDATOR_ENABLED_REPLICAS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONSOLIDATOR_UNSPECIFIED" => Some(Self::Unspecified),
                "CONSOLIDATOR_DISABLED" => Some(Self::Disabled),
                "CONSOLIDATOR_ENABLED" => Some(Self::Enabled),
                "CONSOLIDATOR_ENABLED_REPLICAS" => Some(Self::EnabledReplicas),
                _ => None,
            }
        }
    }
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
    pub enum TransactionAccessMode {
        ConsistentSnapshot = 0,
        ReadWrite = 1,
        ReadOnly = 2,
    }
    impl TransactionAccessMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TransactionAccessMode::ConsistentSnapshot => "CONSISTENT_SNAPSHOT",
                TransactionAccessMode::ReadWrite => "READ_WRITE",
                TransactionAccessMode::ReadOnly => "READ_ONLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONSISTENT_SNAPSHOT" => Some(Self::ConsistentSnapshot),
                "READ_WRITE" => Some(Self::ReadWrite),
                "READ_ONLY" => Some(Self::ReadOnly),
                _ => None,
            }
        }
    }
}
/// Field describes a single column returned by a query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    /// name of the field as returned by mysql C API
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// vitess-defined type. Conversion function is in sqltypes package.
    #[prost(enumeration = "Type", tag = "2")]
    pub r#type: i32,
    /// Remaining fields from mysql C API.
    /// These fields are only populated when ExecuteOptions.included_fields
    /// is set to IncludedFields.ALL.
    #[prost(string, tag = "3")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub org_table: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub database: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub org_name: ::prost::alloc::string::String,
    /// column_length is really a uint32. All 32 bits can be used.
    #[prost(uint32, tag = "7")]
    pub column_length: u32,
    /// charset is actually a uint16. Only the lower 16 bits are used.
    #[prost(uint32, tag = "8")]
    pub charset: u32,
    /// decimals is actually a uint8. Only the lower 8 bits are used.
    #[prost(uint32, tag = "9")]
    pub decimals: u32,
    /// flags is actually a uint16. Only the lower 16 bits are used.
    #[prost(uint32, tag = "10")]
    pub flags: u32,
    /// column_type is optionally populated from information_schema.columns
    #[prost(string, tag = "11")]
    pub column_type: ::prost::alloc::string::String,
}
/// Row is a database row.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    /// lengths contains the length of each value in values.
    /// A length of -1 means that the field is NULL. While
    /// reading values, you have to accummulate the length
    /// to know the offset where the next value begins in values.
    #[prost(sint64, repeated, tag = "1")]
    pub lengths: ::prost::alloc::vec::Vec<i64>,
    /// values contains a concatenation of all values in the row.
    #[prost(bytes = "vec", tag = "2")]
    pub values: ::prost::alloc::vec::Vec<u8>,
}
/// QueryResult is returned by Execute and ExecuteStream.
///
/// As returned by Execute, len(fields) is always equal to len(row)
/// (for each row in rows).
///
/// As returned by StreamExecute, the first QueryResult has the fields
/// set, and subsequent QueryResult have rows set. And as Execute,
/// len(QueryResult\[0\].fields) is always equal to len(row) (for each
/// row in rows for each QueryResult in QueryResult\[1:\]).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResult {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    #[prost(uint64, tag = "2")]
    pub rows_affected: u64,
    #[prost(uint64, tag = "3")]
    pub insert_id: u64,
    #[prost(message, repeated, tag = "4")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
    #[prost(string, tag = "6")]
    pub info: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub session_state_changes: ::prost::alloc::string::String,
}
/// QueryWarning is used to convey out of band query execution warnings
/// by storing in the vtgate.Session
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWarning {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// StreamEvent describes a set of transformations that happened as a
/// single transactional unit on a server. It is streamed back by the
/// Update Stream calls.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamEvent {
    /// The statements in this transaction.
    #[prost(message, repeated, tag = "1")]
    pub statements: ::prost::alloc::vec::Vec<stream_event::Statement>,
    /// The Event Token for this event.
    #[prost(message, optional, tag = "2")]
    pub event_token: ::core::option::Option<EventToken>,
}
/// Nested message and enum types in `StreamEvent`.
pub mod stream_event {
    /// One individual Statement in a transaction.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Statement {
        #[prost(enumeration = "statement::Category", tag = "1")]
        pub category: i32,
        /// table_name, primary_key_fields and primary_key_values are set for DML.
        #[prost(string, tag = "2")]
        pub table_name: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "3")]
        pub primary_key_fields: ::prost::alloc::vec::Vec<super::Field>,
        #[prost(message, repeated, tag = "4")]
        pub primary_key_values: ::prost::alloc::vec::Vec<super::Row>,
        /// sql is set for all queries.
        /// FIXME(alainjobart) we may not need it for DMLs.
        #[prost(bytes = "vec", tag = "5")]
        pub sql: ::prost::alloc::vec::Vec<u8>,
    }
    /// Nested message and enum types in `Statement`.
    pub mod statement {
        /// The category of one statement.
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
        pub enum Category {
            Error = 0,
            Dml = 1,
            Ddl = 2,
        }
        impl Category {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Category::Error => "Error",
                    Category::Dml => "DML",
                    Category::Ddl => "DDL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "Error" => Some(Self::Error),
                    "DML" => Some(Self::Dml),
                    "DDL" => Some(Self::Ddl),
                    _ => None,
                }
            }
        }
    }
}
/// ExecuteRequest is the payload to Execute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(message, optional, tag = "4")]
    pub query: ::core::option::Option<BoundQuery>,
    #[prost(int64, tag = "5")]
    pub transaction_id: i64,
    #[prost(message, optional, tag = "6")]
    pub options: ::core::option::Option<ExecuteOptions>,
    #[prost(int64, tag = "7")]
    pub reserved_id: i64,
}
/// ExecuteResponse is the returned value from Execute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<QueryResult>,
}
/// ResultWithError represents a query response
/// in the form of result or error but not both.
/// TODO: To be used in ExecuteBatchResponse and BeginExecuteBatchResponse.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResultWithError {
    /// error contains an query level error, only set if result is unset.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
    /// result contains the query result, only set if error is unset.
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<QueryResult>,
}
/// StreamExecuteRequest is the payload to StreamExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamExecuteRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(message, optional, tag = "4")]
    pub query: ::core::option::Option<BoundQuery>,
    #[prost(message, optional, tag = "5")]
    pub options: ::core::option::Option<ExecuteOptions>,
    #[prost(int64, tag = "6")]
    pub transaction_id: i64,
    #[prost(int64, tag = "7")]
    pub reserved_id: i64,
}
/// StreamExecuteResponse is the returned value from StreamExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamExecuteResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<QueryResult>,
}
/// BeginRequest is the payload to Begin
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(message, optional, tag = "4")]
    pub options: ::core::option::Option<ExecuteOptions>,
}
/// BeginResponse is the returned value from Begin
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginResponse {
    #[prost(int64, tag = "1")]
    pub transaction_id: i64,
    #[prost(message, optional, tag = "2")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    /// The session_state_changes might be set if the transaction is a snapshot transaction
    /// and the MySQL implementation supports getting a start gtid on snapshot
    #[prost(string, tag = "3")]
    pub session_state_changes: ::prost::alloc::string::String,
}
/// CommitRequest is the payload to Commit
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(int64, tag = "4")]
    pub transaction_id: i64,
}
/// CommitResponse is the returned value from Commit
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitResponse {
    #[prost(int64, tag = "1")]
    pub reserved_id: i64,
}
/// RollbackRequest is the payload to Rollback
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(int64, tag = "4")]
    pub transaction_id: i64,
}
/// RollbackResponse is the returned value from Rollback
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackResponse {
    #[prost(int64, tag = "1")]
    pub reserved_id: i64,
}
/// PrepareRequest is the payload to Prepare
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(int64, tag = "4")]
    pub transaction_id: i64,
    #[prost(string, tag = "5")]
    pub dtid: ::prost::alloc::string::String,
}
/// PrepareResponse is the returned value from Prepare
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrepareResponse {}
/// CommitPreparedRequest is the payload to CommitPrepared
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitPreparedRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(string, tag = "4")]
    pub dtid: ::prost::alloc::string::String,
}
/// CommitPreparedResponse is the returned value from CommitPrepared
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitPreparedResponse {}
/// RollbackPreparedRequest is the payload to RollbackPrepared
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackPreparedRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(int64, tag = "4")]
    pub transaction_id: i64,
    #[prost(string, tag = "5")]
    pub dtid: ::prost::alloc::string::String,
}
/// RollbackPreparedResponse is the returned value from RollbackPrepared
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackPreparedResponse {}
/// CreateTransactionRequest is the payload to CreateTransaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTransactionRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(string, tag = "4")]
    pub dtid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub participants: ::prost::alloc::vec::Vec<Target>,
}
/// CreateTransactionResponse is the returned value from CreateTransaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTransactionResponse {}
/// StartCommitRequest is the payload to StartCommit
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartCommitRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(int64, tag = "4")]
    pub transaction_id: i64,
    #[prost(string, tag = "5")]
    pub dtid: ::prost::alloc::string::String,
}
/// StartCommitResponse is the returned value from StartCommit
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartCommitResponse {}
/// SetRollbackRequest is the payload to SetRollback
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRollbackRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(int64, tag = "4")]
    pub transaction_id: i64,
    #[prost(string, tag = "5")]
    pub dtid: ::prost::alloc::string::String,
}
/// SetRollbackResponse is the returned value from SetRollback
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRollbackResponse {}
/// ConcludeTransactionRequest is the payload to ConcludeTransaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConcludeTransactionRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(string, tag = "4")]
    pub dtid: ::prost::alloc::string::String,
}
/// ConcludeTransactionResponse is the returned value from ConcludeTransaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConcludeTransactionResponse {}
/// ReadTransactionRequest is the payload to ReadTransaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadTransactionRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(string, tag = "4")]
    pub dtid: ::prost::alloc::string::String,
}
/// ReadTransactionResponse is the returned value from ReadTransaction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadTransactionResponse {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<TransactionMetadata>,
}
/// BeginExecuteRequest is the payload to BeginExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginExecuteRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(message, optional, tag = "4")]
    pub query: ::core::option::Option<BoundQuery>,
    #[prost(message, optional, tag = "5")]
    pub options: ::core::option::Option<ExecuteOptions>,
    #[prost(int64, tag = "6")]
    pub reserved_id: i64,
    #[prost(string, repeated, tag = "7")]
    pub pre_queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// BeginExecuteResponse is the returned value from BeginExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginExecuteResponse {
    /// error contains an application level error if necessary. Note the
    /// transaction_id may be set, even when an error is returned, if the begin
    /// worked but the execute failed.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<QueryResult>,
    /// transaction_id might be non-zero even if an error is present.
    #[prost(int64, tag = "3")]
    pub transaction_id: i64,
    #[prost(message, optional, tag = "4")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    /// The session_state_changes might be set if the transaction is a snapshot transaction
    /// and the MySQL implementation supports getting a start gtid on snapshot
    #[prost(string, tag = "5")]
    pub session_state_changes: ::prost::alloc::string::String,
}
/// BeginStreamExecuteRequest is the payload to BeginStreamExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginStreamExecuteRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(message, optional, tag = "4")]
    pub query: ::core::option::Option<BoundQuery>,
    #[prost(message, optional, tag = "5")]
    pub options: ::core::option::Option<ExecuteOptions>,
    #[prost(string, repeated, tag = "6")]
    pub pre_queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "7")]
    pub reserved_id: i64,
}
/// BeginStreamExecuteResponse is the returned value from BeginStreamExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeginStreamExecuteResponse {
    /// error contains an application level error if necessary. Note the
    /// transaction_id may be set, even when an error is returned, if the begin
    /// worked but the stream execute failed.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<QueryResult>,
    /// transaction_id might be non-zero even if an error is present.
    #[prost(int64, tag = "3")]
    pub transaction_id: i64,
    #[prost(message, optional, tag = "4")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    /// The session_state_changes might be set if the transaction is a snapshot transaction
    /// and the MySQL implementation supports getting a start gtid on snapshot
    #[prost(string, tag = "5")]
    pub session_state_changes: ::prost::alloc::string::String,
}
/// MessageStreamRequest is the request payload for MessageStream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageStreamRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    /// name is the message table name.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
/// MessageStreamResponse is a response for MessageStream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageStreamResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<QueryResult>,
}
/// MessageAckRequest is the request payload for MessageAck.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageAckRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    /// name is the message table name.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub ids: ::prost::alloc::vec::Vec<Value>,
}
/// MessageAckResponse is the response for MessageAck.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageAckResponse {
    /// result contains the result of the ack operation.
    /// Since this acts like a DML, only
    /// RowsAffected is returned in the result.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<QueryResult>,
}
/// ReserveExecuteRequest is the payload to ReserveExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveExecuteRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(message, optional, tag = "4")]
    pub query: ::core::option::Option<BoundQuery>,
    #[prost(int64, tag = "5")]
    pub transaction_id: i64,
    #[prost(message, optional, tag = "6")]
    pub options: ::core::option::Option<ExecuteOptions>,
    #[prost(string, repeated, tag = "7")]
    pub pre_queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ReserveExecuteResponse is the returned value from ReserveExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveExecuteResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<QueryResult>,
    /// The following fields might be non-zero even if an error is present.
    #[prost(int64, tag = "3")]
    pub reserved_id: i64,
    #[prost(message, optional, tag = "4")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
/// ReserveStreamExecuteRequest is the payload to ReserveStreamExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveStreamExecuteRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(message, optional, tag = "4")]
    pub query: ::core::option::Option<BoundQuery>,
    #[prost(message, optional, tag = "5")]
    pub options: ::core::option::Option<ExecuteOptions>,
    #[prost(int64, tag = "6")]
    pub transaction_id: i64,
    #[prost(string, repeated, tag = "7")]
    pub pre_queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ReserveStreamExecuteResponse is the returned value from ReserveStreamExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveStreamExecuteResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<QueryResult>,
    /// The following fields might be non-zero even if an error is present.
    #[prost(int64, tag = "3")]
    pub reserved_id: i64,
    #[prost(message, optional, tag = "4")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
/// ReserveBeginExecuteRequest is the payload to ReserveBeginExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveBeginExecuteRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(message, optional, tag = "4")]
    pub query: ::core::option::Option<BoundQuery>,
    #[prost(message, optional, tag = "5")]
    pub options: ::core::option::Option<ExecuteOptions>,
    #[prost(string, repeated, tag = "6")]
    pub pre_queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub post_begin_queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ReserveBeginExecuteResponse is the returned value from ReserveBeginExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveBeginExecuteResponse {
    /// error contains an application level error if necessary. Note the
    /// transaction_id may be set, even when an error is returned, if the begin
    /// worked but the execute failed.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<QueryResult>,
    /// The following fields might be non-zero even if an error is present.
    #[prost(int64, tag = "3")]
    pub transaction_id: i64,
    #[prost(int64, tag = "4")]
    pub reserved_id: i64,
    #[prost(message, optional, tag = "5")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    /// The session_state_changes might be set if the transaction is a snapshot transaction
    /// and the MySQL implementation supports getting a start gtid on snapshot
    #[prost(string, tag = "6")]
    pub session_state_changes: ::prost::alloc::string::String,
}
/// ReserveBeginStreamExecuteRequest is the payload to ReserveBeginStreamExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveBeginStreamExecuteRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(message, optional, tag = "4")]
    pub query: ::core::option::Option<BoundQuery>,
    #[prost(message, optional, tag = "5")]
    pub options: ::core::option::Option<ExecuteOptions>,
    #[prost(string, repeated, tag = "6")]
    pub pre_queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "7")]
    pub post_begin_queries: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ReserveBeginStreamExecuteResponse is the returned value from ReserveBeginStreamExecute
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveBeginStreamExecuteResponse {
    /// error contains an application level error if necessary. Note the
    /// transaction_id may be set, even when an error is returned, if the begin
    /// worked but the stream execute failed.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::vtrpc::RpcError>,
    #[prost(message, optional, tag = "2")]
    pub result: ::core::option::Option<QueryResult>,
    /// The following fields might be non-zero even if an error is present.
    #[prost(int64, tag = "3")]
    pub transaction_id: i64,
    #[prost(int64, tag = "4")]
    pub reserved_id: i64,
    #[prost(message, optional, tag = "5")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    /// The session_state_changes might be set if the transaction is a snapshot transaction
    /// and the MySQL implementation supports getting a start gtid on snapshot
    #[prost(string, tag = "6")]
    pub session_state_changes: ::prost::alloc::string::String,
}
/// ReleaseRequest is the payload to Release
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    #[prost(int64, tag = "4")]
    pub transaction_id: i64,
    #[prost(int64, tag = "5")]
    pub reserved_id: i64,
}
/// ReleaseResponse is the returned value from Release
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseResponse {}
/// StreamHealthRequest is the payload for StreamHealth
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamHealthRequest {}
/// RealtimeStats contains information about the tablet status.
/// It is only valid for a single tablet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealtimeStats {
    /// health_error is the last error we got from health check,
    /// or empty is the server is healthy. This is used for subset selection,
    /// we do not send queries to servers that are not healthy.
    #[prost(string, tag = "1")]
    pub health_error: ::prost::alloc::string::String,
    /// replication_lag_seconds is populated for replicas only. It indicates
    /// how far behind on (MySQL) replication a replica currently is.  It is used
    /// by clients for subset selection (so we don't try to send traffic
    /// to tablets that are too far behind).
    /// NOTE: This field must not be evaluated if "health_error" is not empty.
    /// TODO(mberlin): Let's switch it to int64 instead?
    #[prost(uint32, tag = "2")]
    pub replication_lag_seconds: u32,
    /// bin_log_players_count is the number of currently running binlog players.
    /// if the value is 0, it means that filtered replication is currently not
    /// running on the tablet. If >0, filtered replication is running.
    /// NOTE: This field must not be evaluated if "health_error" is not empty.
    #[prost(int32, tag = "3")]
    pub binlog_players_count: i32,
    /// filtered_replication_lag_seconds is populated for the receiving
    /// primary of an ongoing filtered replication only.
    /// It specifies how far the receiving primary lags behind the sending primary.
    /// NOTE: This field must not be evaluated if "health_error" is not empty.
    /// NOTE: This field must not be evaluated if "bin_log_players_count" is 0.
    #[prost(int64, tag = "4")]
    pub filtered_replication_lag_seconds: i64,
    /// cpu_usage is used for load-based balancing
    #[prost(double, tag = "5")]
    pub cpu_usage: f64,
    /// qps is the average QPS (queries per second) rate in the last XX seconds
    /// where XX is usually 60 (See query_service_stats.go).
    #[prost(double, tag = "6")]
    pub qps: f64,
    /// table_schema_changed is to provide list of tables that have schema changes detected by the tablet.
    #[prost(string, repeated, tag = "7")]
    pub table_schema_changed: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// view_schema_changed is to provide list of views that have schema changes detected by the tablet.
    #[prost(string, repeated, tag = "8")]
    pub view_schema_changed: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AggregateStats contains information about the health of a group of
/// tablets for a Target.  It is used to propagate stats from a vtgate
/// to another, or from the Gateway layer of a vtgate to the routing
/// layer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregateStats {
    /// healthy_tablet_count is the number of healthy tablets in the group.
    #[prost(int32, tag = "1")]
    pub healthy_tablet_count: i32,
    /// unhealthy_tablet_count is the number of unhealthy tablets in the group.
    #[prost(int32, tag = "2")]
    pub unhealthy_tablet_count: i32,
    /// replication_lag_seconds_min is the minimum of the
    /// replication_lag_seconds values of the healthy tablets. It is unset
    /// if the tablet type is primary.
    #[prost(uint32, tag = "3")]
    pub replication_lag_seconds_min: u32,
    /// replication_lag_seconds_max is the maximum of the
    /// replication_lag_seconds values of the healthy tablets. It is unset
    /// if the tablet type is primary.
    #[prost(uint32, tag = "4")]
    pub replication_lag_seconds_max: u32,
}
/// StreamHealthResponse is streamed by StreamHealth on a regular basis.
/// It is expected to be used between a vtgate and vttablet:
/// - target describes the tablet.
/// - realtime_stats is set.
/// - aggregate_stats is not set (deprecated)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamHealthResponse {
    /// target is the current server type. Only queries with that exact Target
    /// record will be accepted (the cell may not match, however).
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<Target>,
    /// serving is true iff the tablet is serving. A tablet may not be serving
    /// if filtered replication is enabled on a primary for instance,
    /// or if a replica should not be used because the keyspace is being resharded.
    #[prost(bool, tag = "2")]
    pub serving: bool,
    /// tablet_externally_reparented_timestamp can be interpreted as the
    /// last time we knew that this tablet was the PRIMARY of this shard
    /// (if StreamHealthResponse describes a group of tablets, between
    /// two vtgates, only one primary will be present in the group, and
    /// this is this primary's value).
    ///
    /// It is used by vtgate when determining the current PRIMARY of a shard.
    /// If vtgate sees more than one PRIMARY tablet, this timestamp is used
    /// as tiebreaker where the PRIMARY with the highest timestamp wins.
    /// Another usage of this timestamp is in go/vt/vtgate/buffer to detect the end
    /// of a reparent (failover) and stop buffering.
    ///
    /// In practice, this field is set to:
    /// a) the last time the RPC tabletmanager.TabletExternallyReparented was
    ///     called on this tablet (usually done by an external failover tool e.g.
    ///     Orchestrator). The failover tool can call this as long as we are the
    ///     primary i.e. even ages after the last reparent occurred.
    /// OR
    /// b) the last time an active reparent was executed through a vtctl command
    ///     (InitShardPrimary, PlannedReparentShard, EmergencyReparentShard)
    /// OR
    /// c) the last time vttablet was started and it initialized its tablet type
    ///     as PRIMARY because it was recorded as the shard's current primary in the
    ///     topology (see go/vt/vttablet/tabletmanager/init_tablet.go)
    /// OR
    /// d) 0 if the vttablet was never a PRIMARY.
    #[prost(int64, tag = "3")]
    pub tablet_externally_reparented_timestamp: i64,
    /// realtime_stats contains information about the tablet status.
    /// It is only filled in if the information is about a tablet.
    #[prost(message, optional, tag = "4")]
    pub realtime_stats: ::core::option::Option<RealtimeStats>,
    /// tablet_alias is the alias of the sending tablet. The discovery/healthcheck.go
    /// code uses it to verify that it's talking to the correct tablet and that it
    /// hasn't changed in the meantime e.g. due to tablet restarts where ports or
    /// ips have been reused but assigned differently.
    #[prost(message, optional, tag = "5")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
/// TransactionMetadata contains the metadata for a distributed transaction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionMetadata {
    #[prost(string, tag = "1")]
    pub dtid: ::prost::alloc::string::String,
    #[prost(enumeration = "TransactionState", tag = "2")]
    pub state: i32,
    #[prost(int64, tag = "3")]
    pub time_created: i64,
    #[prost(message, repeated, tag = "4")]
    pub participants: ::prost::alloc::vec::Vec<Target>,
}
/// GetSchemaRequest is the payload to GetSchema
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaRequest {
    #[prost(message, optional, tag = "1")]
    pub target: ::core::option::Option<Target>,
    #[prost(enumeration = "SchemaTableType", tag = "2")]
    pub table_type: i32,
    #[prost(string, repeated, tag = "3")]
    pub table_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetSchemaResponse is the returned value from GetSchema
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaResponse {
    /// this is for the schema definition for the requested tables.
    #[prost(map = "string, string", tag = "2")]
    pub table_definition: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Flags sent from the MySQL C API
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MySqlFlag {
    Empty = 0,
    NotNullFlag = 1,
    PriKeyFlag = 2,
    UniqueKeyFlag = 4,
    MultipleKeyFlag = 8,
    BlobFlag = 16,
    UnsignedFlag = 32,
    ZerofillFlag = 64,
    BinaryFlag = 128,
    EnumFlag = 256,
    AutoIncrementFlag = 512,
    TimestampFlag = 1024,
    SetFlag = 2048,
    NoDefaultValueFlag = 4096,
    OnUpdateNowFlag = 8192,
    NumFlag = 32768,
    PartKeyFlag = 16384,
    UniqueFlag = 65536,
    BincmpFlag = 131072,
}
impl MySqlFlag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MySqlFlag::Empty => "EMPTY",
            MySqlFlag::NotNullFlag => "NOT_NULL_FLAG",
            MySqlFlag::PriKeyFlag => "PRI_KEY_FLAG",
            MySqlFlag::UniqueKeyFlag => "UNIQUE_KEY_FLAG",
            MySqlFlag::MultipleKeyFlag => "MULTIPLE_KEY_FLAG",
            MySqlFlag::BlobFlag => "BLOB_FLAG",
            MySqlFlag::UnsignedFlag => "UNSIGNED_FLAG",
            MySqlFlag::ZerofillFlag => "ZEROFILL_FLAG",
            MySqlFlag::BinaryFlag => "BINARY_FLAG",
            MySqlFlag::EnumFlag => "ENUM_FLAG",
            MySqlFlag::AutoIncrementFlag => "AUTO_INCREMENT_FLAG",
            MySqlFlag::TimestampFlag => "TIMESTAMP_FLAG",
            MySqlFlag::SetFlag => "SET_FLAG",
            MySqlFlag::NoDefaultValueFlag => "NO_DEFAULT_VALUE_FLAG",
            MySqlFlag::OnUpdateNowFlag => "ON_UPDATE_NOW_FLAG",
            MySqlFlag::NumFlag => "NUM_FLAG",
            MySqlFlag::PartKeyFlag => "PART_KEY_FLAG",
            MySqlFlag::UniqueFlag => "UNIQUE_FLAG",
            MySqlFlag::BincmpFlag => "BINCMP_FLAG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EMPTY" => Some(Self::Empty),
            "NOT_NULL_FLAG" => Some(Self::NotNullFlag),
            "PRI_KEY_FLAG" => Some(Self::PriKeyFlag),
            "UNIQUE_KEY_FLAG" => Some(Self::UniqueKeyFlag),
            "MULTIPLE_KEY_FLAG" => Some(Self::MultipleKeyFlag),
            "BLOB_FLAG" => Some(Self::BlobFlag),
            "UNSIGNED_FLAG" => Some(Self::UnsignedFlag),
            "ZEROFILL_FLAG" => Some(Self::ZerofillFlag),
            "BINARY_FLAG" => Some(Self::BinaryFlag),
            "ENUM_FLAG" => Some(Self::EnumFlag),
            "AUTO_INCREMENT_FLAG" => Some(Self::AutoIncrementFlag),
            "TIMESTAMP_FLAG" => Some(Self::TimestampFlag),
            "SET_FLAG" => Some(Self::SetFlag),
            "NO_DEFAULT_VALUE_FLAG" => Some(Self::NoDefaultValueFlag),
            "ON_UPDATE_NOW_FLAG" => Some(Self::OnUpdateNowFlag),
            "NUM_FLAG" => Some(Self::NumFlag),
            "PART_KEY_FLAG" => Some(Self::PartKeyFlag),
            "UNIQUE_FLAG" => Some(Self::UniqueFlag),
            "BINCMP_FLAG" => Some(Self::BincmpFlag),
            _ => None,
        }
    }
}
/// Flag allows us to qualify types by their common properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Flag {
    None = 0,
    Isintegral = 256,
    Isunsigned = 512,
    Isfloat = 1024,
    Isquoted = 2048,
    Istext = 4096,
    Isbinary = 8192,
}
impl Flag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Flag::None => "NONE",
            Flag::Isintegral => "ISINTEGRAL",
            Flag::Isunsigned => "ISUNSIGNED",
            Flag::Isfloat => "ISFLOAT",
            Flag::Isquoted => "ISQUOTED",
            Flag::Istext => "ISTEXT",
            Flag::Isbinary => "ISBINARY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "ISINTEGRAL" => Some(Self::Isintegral),
            "ISUNSIGNED" => Some(Self::Isunsigned),
            "ISFLOAT" => Some(Self::Isfloat),
            "ISQUOTED" => Some(Self::Isquoted),
            "ISTEXT" => Some(Self::Istext),
            "ISBINARY" => Some(Self::Isbinary),
            _ => None,
        }
    }
}
/// Type defines the various supported data types in bind vars
/// and query results.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    /// NULL_TYPE specifies a NULL type.
    NullType = 0,
    /// INT8 specifies a TINYINT type.
    /// Properties: 1, IsNumber.
    Int8 = 257,
    /// UINT8 specifies a TINYINT UNSIGNED type.
    /// Properties: 2, IsNumber, IsUnsigned.
    Uint8 = 770,
    /// INT16 specifies a SMALLINT type.
    /// Properties: 3, IsNumber.
    Int16 = 259,
    /// UINT16 specifies a SMALLINT UNSIGNED type.
    /// Properties: 4, IsNumber, IsUnsigned.
    Uint16 = 772,
    /// INT24 specifies a MEDIUMINT type.
    /// Properties: 5, IsNumber.
    Int24 = 261,
    /// UINT24 specifies a MEDIUMINT UNSIGNED type.
    /// Properties: 6, IsNumber, IsUnsigned.
    Uint24 = 774,
    /// INT32 specifies a INTEGER type.
    /// Properties: 7, IsNumber.
    Int32 = 263,
    /// UINT32 specifies a INTEGER UNSIGNED type.
    /// Properties: 8, IsNumber, IsUnsigned.
    Uint32 = 776,
    /// INT64 specifies a BIGINT type.
    /// Properties: 9, IsNumber.
    Int64 = 265,
    /// UINT64 specifies a BIGINT UNSIGNED type.
    /// Properties: 10, IsNumber, IsUnsigned.
    Uint64 = 778,
    /// FLOAT32 specifies a FLOAT type.
    /// Properties: 11, IsFloat.
    Float32 = 1035,
    /// FLOAT64 specifies a DOUBLE or REAL type.
    /// Properties: 12, IsFloat.
    Float64 = 1036,
    /// TIMESTAMP specifies a TIMESTAMP type.
    /// Properties: 13, IsQuoted.
    Timestamp = 2061,
    /// DATE specifies a DATE type.
    /// Properties: 14, IsQuoted.
    Date = 2062,
    /// TIME specifies a TIME type.
    /// Properties: 15, IsQuoted.
    Time = 2063,
    /// DATETIME specifies a DATETIME type.
    /// Properties: 16, IsQuoted.
    Datetime = 2064,
    /// YEAR specifies a YEAR type.
    /// Properties: 17, IsNumber, IsUnsigned.
    Year = 785,
    /// DECIMAL specifies a DECIMAL or NUMERIC type.
    /// Properties: 18, None.
    Decimal = 18,
    /// TEXT specifies a TEXT type.
    /// Properties: 19, IsQuoted, IsText.
    Text = 6163,
    /// BLOB specifies a BLOB type.
    /// Properties: 20, IsQuoted, IsBinary.
    Blob = 10260,
    /// VARCHAR specifies a VARCHAR type.
    /// Properties: 21, IsQuoted, IsText.
    Varchar = 6165,
    /// VARBINARY specifies a VARBINARY type.
    /// Properties: 22, IsQuoted, IsBinary.
    Varbinary = 10262,
    /// CHAR specifies a CHAR type.
    /// Properties: 23, IsQuoted, IsText.
    Char = 6167,
    /// BINARY specifies a BINARY type.
    /// Properties: 24, IsQuoted, IsBinary.
    Binary = 10264,
    /// BIT specifies a BIT type.
    /// Properties: 25, IsQuoted.
    Bit = 2073,
    /// ENUM specifies an ENUM type.
    /// Properties: 26, IsQuoted.
    Enum = 2074,
    /// SET specifies a SET type.
    /// Properties: 27, IsQuoted.
    Set = 2075,
    /// TUPLE specifies a tuple. This cannot
    /// be returned in a QueryResult, but it can
    /// be sent as a bind var.
    /// Properties: 28, None.
    Tuple = 28,
    /// GEOMETRY specifies a GEOMETRY type.
    /// Properties: 29, IsQuoted.
    Geometry = 2077,
    /// JSON specifies a JSON type.
    /// Properties: 30, IsQuoted.
    Json = 2078,
    /// EXPRESSION specifies a SQL expression.
    /// This type is for internal use only.
    /// Properties: 31, None.
    Expression = 31,
    /// HEXNUM specifies a HEXNUM type (unquoted varbinary).
    /// Properties: 32, IsText.
    Hexnum = 4128,
    /// HEXVAL specifies a HEXVAL type (unquoted varbinary).
    /// Properties: 33, IsText.
    Hexval = 4129,
    /// BITNUM specifies a base 2 binary type (unquoted varbinary).
    /// Properties: 34, IsText.
    Bitnum = 4130,
}
impl Type {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Type::NullType => "NULL_TYPE",
            Type::Int8 => "INT8",
            Type::Uint8 => "UINT8",
            Type::Int16 => "INT16",
            Type::Uint16 => "UINT16",
            Type::Int24 => "INT24",
            Type::Uint24 => "UINT24",
            Type::Int32 => "INT32",
            Type::Uint32 => "UINT32",
            Type::Int64 => "INT64",
            Type::Uint64 => "UINT64",
            Type::Float32 => "FLOAT32",
            Type::Float64 => "FLOAT64",
            Type::Timestamp => "TIMESTAMP",
            Type::Date => "DATE",
            Type::Time => "TIME",
            Type::Datetime => "DATETIME",
            Type::Year => "YEAR",
            Type::Decimal => "DECIMAL",
            Type::Text => "TEXT",
            Type::Blob => "BLOB",
            Type::Varchar => "VARCHAR",
            Type::Varbinary => "VARBINARY",
            Type::Char => "CHAR",
            Type::Binary => "BINARY",
            Type::Bit => "BIT",
            Type::Enum => "ENUM",
            Type::Set => "SET",
            Type::Tuple => "TUPLE",
            Type::Geometry => "GEOMETRY",
            Type::Json => "JSON",
            Type::Expression => "EXPRESSION",
            Type::Hexnum => "HEXNUM",
            Type::Hexval => "HEXVAL",
            Type::Bitnum => "BITNUM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NULL_TYPE" => Some(Self::NullType),
            "INT8" => Some(Self::Int8),
            "UINT8" => Some(Self::Uint8),
            "INT16" => Some(Self::Int16),
            "UINT16" => Some(Self::Uint16),
            "INT24" => Some(Self::Int24),
            "UINT24" => Some(Self::Uint24),
            "INT32" => Some(Self::Int32),
            "UINT32" => Some(Self::Uint32),
            "INT64" => Some(Self::Int64),
            "UINT64" => Some(Self::Uint64),
            "FLOAT32" => Some(Self::Float32),
            "FLOAT64" => Some(Self::Float64),
            "TIMESTAMP" => Some(Self::Timestamp),
            "DATE" => Some(Self::Date),
            "TIME" => Some(Self::Time),
            "DATETIME" => Some(Self::Datetime),
            "YEAR" => Some(Self::Year),
            "DECIMAL" => Some(Self::Decimal),
            "TEXT" => Some(Self::Text),
            "BLOB" => Some(Self::Blob),
            "VARCHAR" => Some(Self::Varchar),
            "VARBINARY" => Some(Self::Varbinary),
            "CHAR" => Some(Self::Char),
            "BINARY" => Some(Self::Binary),
            "BIT" => Some(Self::Bit),
            "ENUM" => Some(Self::Enum),
            "SET" => Some(Self::Set),
            "TUPLE" => Some(Self::Tuple),
            "GEOMETRY" => Some(Self::Geometry),
            "JSON" => Some(Self::Json),
            "EXPRESSION" => Some(Self::Expression),
            "HEXNUM" => Some(Self::Hexnum),
            "HEXVAL" => Some(Self::Hexval),
            "BITNUM" => Some(Self::Bitnum),
            _ => None,
        }
    }
}
/// TransactionState represents the state of a distributed transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionState {
    Unknown = 0,
    Prepare = 1,
    Commit = 2,
    Rollback = 3,
}
impl TransactionState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionState::Unknown => "UNKNOWN",
            TransactionState::Prepare => "PREPARE",
            TransactionState::Commit => "COMMIT",
            TransactionState::Rollback => "ROLLBACK",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "PREPARE" => Some(Self::Prepare),
            "COMMIT" => Some(Self::Commit),
            "ROLLBACK" => Some(Self::Rollback),
            _ => None,
        }
    }
}
/// SchemaTableType represents the type of table requested.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SchemaTableType {
    Views = 0,
    Tables = 1,
    All = 2,
}
impl SchemaTableType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SchemaTableType::Views => "VIEWS",
            SchemaTableType::Tables => "TABLES",
            SchemaTableType::All => "ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VIEWS" => Some(Self::Views),
            "TABLES" => Some(Self::Tables),
            "ALL" => Some(Self::All),
            _ => None,
        }
    }
}
