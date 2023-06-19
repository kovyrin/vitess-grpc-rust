/// Charset is the per-statement charset info from a QUERY_EVENT binlog entry.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Charset {
    /// @@session.character_set_client
    #[prost(int32, tag = "1")]
    pub client: i32,
    /// @@session.collation_connection
    #[prost(int32, tag = "2")]
    pub conn: i32,
    /// @@session.collation_server
    #[prost(int32, tag = "3")]
    pub server: i32,
}
/// BinlogTransaction describes a transaction inside the binlogs.
/// It is streamed by vttablet for filtered replication, used during resharding.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinlogTransaction {
    /// the statements in this transaction
    #[prost(message, repeated, tag = "1")]
    pub statements: ::prost::alloc::vec::Vec<binlog_transaction::Statement>,
    /// The Event Token for this event.
    #[prost(message, optional, tag = "4")]
    pub event_token: ::core::option::Option<super::query::EventToken>,
}
/// Nested message and enum types in `BinlogTransaction`.
pub mod binlog_transaction {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Statement {
        /// what type of statement is this?
        #[prost(enumeration = "statement::Category", tag = "1")]
        pub category: i32,
        /// charset of this statement, if different from pre-negotiated default.
        #[prost(message, optional, tag = "2")]
        pub charset: ::core::option::Option<super::Charset>,
        /// the sql
        #[prost(bytes = "vec", tag = "3")]
        pub sql: ::prost::alloc::vec::Vec<u8>,
    }
    /// Nested message and enum types in `Statement`.
    pub mod statement {
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
            BlUnrecognized = 0,
            BlBegin = 1,
            BlCommit = 2,
            BlRollback = 3,
            /// BL_DML is deprecated.
            BlDmlDeprecated = 4,
            BlDdl = 5,
            BlSet = 6,
            BlInsert = 7,
            BlUpdate = 8,
            BlDelete = 9,
        }
        impl Category {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Category::BlUnrecognized => "BL_UNRECOGNIZED",
                    Category::BlBegin => "BL_BEGIN",
                    Category::BlCommit => "BL_COMMIT",
                    Category::BlRollback => "BL_ROLLBACK",
                    Category::BlDmlDeprecated => "BL_DML_DEPRECATED",
                    Category::BlDdl => "BL_DDL",
                    Category::BlSet => "BL_SET",
                    Category::BlInsert => "BL_INSERT",
                    Category::BlUpdate => "BL_UPDATE",
                    Category::BlDelete => "BL_DELETE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "BL_UNRECOGNIZED" => Some(Self::BlUnrecognized),
                    "BL_BEGIN" => Some(Self::BlBegin),
                    "BL_COMMIT" => Some(Self::BlCommit),
                    "BL_ROLLBACK" => Some(Self::BlRollback),
                    "BL_DML_DEPRECATED" => Some(Self::BlDmlDeprecated),
                    "BL_DDL" => Some(Self::BlDdl),
                    "BL_SET" => Some(Self::BlSet),
                    "BL_INSERT" => Some(Self::BlInsert),
                    "BL_UPDATE" => Some(Self::BlUpdate),
                    "BL_DELETE" => Some(Self::BlDelete),
                    _ => None,
                }
            }
        }
    }
}
/// StreamKeyRangeRequest is the payload to StreamKeyRange
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamKeyRangeRequest {
    /// where to start
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
    /// what to get
    #[prost(message, optional, tag = "2")]
    pub key_range: ::core::option::Option<super::topodata::KeyRange>,
    /// default charset on the player side
    #[prost(message, optional, tag = "3")]
    pub charset: ::core::option::Option<Charset>,
}
/// StreamKeyRangeResponse is the response from StreamKeyRange
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamKeyRangeResponse {
    #[prost(message, optional, tag = "1")]
    pub binlog_transaction: ::core::option::Option<BinlogTransaction>,
}
/// StreamTablesRequest is the payload to StreamTables
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTablesRequest {
    /// where to start
    #[prost(string, tag = "1")]
    pub position: ::prost::alloc::string::String,
    /// what to get
    #[prost(string, repeated, tag = "2")]
    pub tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// default charset on the player side
    #[prost(message, optional, tag = "3")]
    pub charset: ::core::option::Option<Charset>,
}
/// StreamTablesResponse is the response from StreamTables
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamTablesResponse {
    #[prost(message, optional, tag = "1")]
    pub binlog_transaction: ::core::option::Option<BinlogTransaction>,
}
/// CharsetConversion represent a conversion of text from one charset to another
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CharsetConversion {
    /// FromCharset is the charset name from which we convert the text (e.g. latin1)
    #[prost(string, tag = "1")]
    pub from_charset: ::prost::alloc::string::String,
    /// ToCharset is the charset name to which we convert the text (e.g. utf8mb4)
    #[prost(string, tag = "2")]
    pub to_charset: ::prost::alloc::string::String,
}
/// Rule represents one rule in a Filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rule {
    /// Match can be a table name or a regular expression.
    /// If it starts with a '/', it's a regular expression.
    /// For example, "t" matches a table named "t", whereas
    /// "/t.*" matches all tables that begin with 't'.
    #[prost(string, tag = "1")]
    pub r#match: ::prost::alloc::string::String,
    /// Filter: If empty, all columns and rows of the matching tables
    /// are sent. If it's a keyrange like "-80", only rows that
    /// match the keyrange are sent.
    /// If Match is a table name instead of a regular expression,
    /// the Filter can also be a select expression like this:
    /// "select * from t", same as an empty Filter, or
    /// "select * from t where in_keyrange('-80')", same as "-80", or
    /// "select col1, col2 from t where in_keyrange(col1, 'hash', '-80'), or
    /// What is allowed in a select expression depends on whether
    /// it's a vstreamer or vreplication request. For more details,
    /// please refer to the specific package documentation.
    /// On the vreplication side, Filter can also accept a special
    /// "exclude" value, which will cause the matched tables
    /// to be excluded.
    /// TODO(sougou): support this on vstreamer side also.
    ///
    /// ConvertEnumToText: optional, list per enum column name, the list of textual values.
    /// When reading the binary log, all enum values are numeric. But sometimes it
    /// is useful/needed to know what the textual mapping are.
    /// Online DDL provides such use case.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Example: key="color", value="'red','green','blue'"
    #[prost(map = "string, string", tag = "3")]
    pub convert_enum_to_text: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// ConvertCharset: optional mapping, between column name and a CharsetConversion.
    /// This hints to vreplication that columns are encoded from/to non-trivial charsets
    /// The map is only populated when either "from" or "to" charset of a column are non-trivial
    /// trivial charsets are utf8 and ascii variants.
    #[prost(map = "string, message", tag = "4")]
    pub convert_charset: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        CharsetConversion,
    >,
    /// SourceUniqueKeyColumns represents the ordered columns in the index used by rowstreamer to iterate the table
    /// It is comma delimited, as in col1,col2,col3 (tokens are escaped via net/url)
    #[prost(string, tag = "5")]
    pub source_unique_key_columns: ::prost::alloc::string::String,
    /// TargetUniqueKeyColumns represents the ordered columns in that index used by vcopier and vplayer to apply rows
    /// It is comma delimited, as in col1,col2,col3 (tokens are escaped via net/url)
    #[prost(string, tag = "6")]
    pub target_unique_key_columns: ::prost::alloc::string::String,
    /// SourceUniqueKeyTargetColumns represents the names of columns in target table, mapped from the chosen unique
    /// key on source tables (some columns may be renamed from source to target)
    #[prost(string, tag = "7")]
    pub source_unique_key_target_columns: ::prost::alloc::string::String,
    /// ConvertIntToEnum lists any columns that are converted from an integral value into an enum.
    /// such columns need to have special transofrmation of the data, from an integral format into a
    /// string format. e.g. the value 0 needs to be converted to '0'.
    #[prost(map = "string, bool", tag = "8")]
    pub convert_int_to_enum: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        bool,
    >,
}
/// Filter represents a list of ordered rules. The first
/// match wins.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<Rule>,
    /// FieldEventMode specifies the behavior if there is a mismatch
    /// between the current schema and the fields in the binlog. This
    /// can happen if the binlog position is before a DDL that would
    /// cause the fields to change. If vstreamer detects such
    /// an inconsistency, the behavior depends on the FieldEventMode.
    /// If the value is ERR_ON_MISMATCH (default), then it errors out.
    /// If it's BEST_EFFORT, it sends a field event with fake column
    /// names as "@1", "@2", etc.
    #[prost(enumeration = "filter::FieldEventMode", tag = "2")]
    pub field_event_mode: i32,
    #[prost(int64, tag = "3")]
    pub workflow_type: i64,
    #[prost(string, tag = "4")]
    pub workflow_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
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
    pub enum FieldEventMode {
        ErrOnMismatch = 0,
        BestEffort = 1,
    }
    impl FieldEventMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FieldEventMode::ErrOnMismatch => "ERR_ON_MISMATCH",
                FieldEventMode::BestEffort => "BEST_EFFORT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ERR_ON_MISMATCH" => Some(Self::ErrOnMismatch),
                "BEST_EFFORT" => Some(Self::BestEffort),
                _ => None,
            }
        }
    }
}
/// BinlogSource specifies the source  and filter parameters for
/// Filtered Replication. KeyRange and Tables are legacy. Filter
/// is the new way to specify the filtering rules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinlogSource {
    /// the source keyspace
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// the source shard
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    /// the source tablet type
    #[prost(enumeration = "super::topodata::TabletType", tag = "3")]
    pub tablet_type: i32,
    /// KeyRange is set if the request is for a keyrange
    #[prost(message, optional, tag = "4")]
    pub key_range: ::core::option::Option<super::topodata::KeyRange>,
    /// Tables is set if the request is for a list of tables
    #[prost(string, repeated, tag = "5")]
    pub tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Filter is set if we're using the generalized representation
    /// for the filter.
    #[prost(message, optional, tag = "6")]
    pub filter: ::core::option::Option<Filter>,
    /// OnDdl specifies the action to be taken when a DDL is encountered.
    #[prost(enumeration = "OnDdlAction", tag = "7")]
    pub on_ddl: i32,
    /// Source is an external mysql. This attribute should be set to the username
    /// to use in the connection
    #[prost(string, tag = "8")]
    pub external_mysql: ::prost::alloc::string::String,
    /// StopAfterCopy specifies if vreplication should be stopped
    /// after copying is done.
    #[prost(bool, tag = "9")]
    pub stop_after_copy: bool,
    /// ExternalCluster is the name of the mounted cluster which has the source keyspace/db for this workflow
    /// it is of the type <cluster_type.cluster_name>
    #[prost(string, tag = "10")]
    pub external_cluster: ::prost::alloc::string::String,
    /// SourceTimeZone is the time zone in which datetimes on the source were stored, provided as an option in MoveTables
    #[prost(string, tag = "11")]
    pub source_time_zone: ::prost::alloc::string::String,
    /// TargetTimeZone is not currently specifiable by the user, defaults to UTC for the forward workflows
    /// and to the SourceTimeZone in reverse workflows
    #[prost(string, tag = "12")]
    pub target_time_zone: ::prost::alloc::string::String,
}
/// RowChange represents one row change.
/// If Before is set and not After, it's a delete.
/// If After is set and not Before, it's an insert.
/// If both are set, it's an update.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowChange {
    #[prost(message, optional, tag = "1")]
    pub before: ::core::option::Option<super::query::Row>,
    #[prost(message, optional, tag = "2")]
    pub after: ::core::option::Option<super::query::Row>,
}
/// RowEvent represent row events for one table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowEvent {
    #[prost(string, tag = "1")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub row_changes: ::prost::alloc::vec::Vec<RowChange>,
    #[prost(string, tag = "3")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub shard: ::prost::alloc::string::String,
}
/// FieldEvent represents the field info for a table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldEvent {
    #[prost(string, tag = "1")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<super::query::Field>,
    #[prost(string, tag = "3")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub shard: ::prost::alloc::string::String,
}
/// ShardGtid contains the GTID position for one shard.
/// It's used in a request for requesting a starting position.
/// It's used in a response to transmit the current position
/// of a shard. It's also used in a Journal to indicate the
/// list of targets and shard positions to migrate to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardGtid {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub gtid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub table_p_ks: ::prost::alloc::vec::Vec<TableLastPk>,
}
/// A VGtid is a list of ShardGtids.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VGtid {
    #[prost(message, repeated, tag = "1")]
    pub shard_gtids: ::prost::alloc::vec::Vec<ShardGtid>,
}
/// KeyspaceShard represents a keyspace and shard.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyspaceShard {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
}
/// Journal contains the metadata for a journal event.
/// The commit of a journal event indicates the point of no return
/// for a migration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Journal {
    /// Id represents a unique journal id.
    #[prost(int64, tag = "1")]
    pub id: i64,
    #[prost(enumeration = "MigrationType", tag = "2")]
    pub migration_type: i32,
    /// Tables is set if the journal represents a TABLES migration.
    #[prost(string, repeated, tag = "3")]
    pub tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// LocalPosition is the source position at which the migration happened.
    #[prost(string, tag = "4")]
    pub local_position: ::prost::alloc::string::String,
    /// ShardGtids is the list of targets to which the migration took place.
    #[prost(message, repeated, tag = "5")]
    pub shard_gtids: ::prost::alloc::vec::Vec<ShardGtid>,
    /// Participants is the list of source participants for a migration.
    /// Every participant is expected to have an identical journal entry.
    /// While streaming, the client must wait for the journal entry to
    /// be received from all pariticipants, and then replace them with new
    /// streams specified by ShardGtid.
    /// If a stream does not have all participants, a consistent migration
    /// is not possible.
    #[prost(message, repeated, tag = "6")]
    pub participants: ::prost::alloc::vec::Vec<KeyspaceShard>,
    /// SourceWorkflows is the list of workflows in the source shard that
    /// were migrated to the target. If a migration fails after a Journal
    /// is committed, this information is used to start the target streams
    /// that were created prior to the creation of the journal.
    #[prost(string, repeated, tag = "7")]
    pub source_workflows: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// VEvent represents a vstream event.
/// A FieldEvent is sent once for every table, just before
/// the first event for that table. The client is expected
/// to cache this information and match it against the RowEvent
/// which contains the table name.
/// A GTID event always precedes a commitable event, which can be
/// COMMIT, DDL or OTHER.
/// OTHER events are non-material events that have no additional metadata.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VEvent {
    #[prost(enumeration = "VEventType", tag = "1")]
    pub r#type: i32,
    /// Timestamp is the binlog timestamp in seconds.
    /// The value should be ignored if 0.
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
    /// Gtid is set if the event type is GTID.
    #[prost(string, tag = "3")]
    pub gtid: ::prost::alloc::string::String,
    /// Statement is set if the event type is DDL, DML or SAVEPOINT.
    #[prost(string, tag = "4")]
    pub statement: ::prost::alloc::string::String,
    /// RowEvent is set if the event type is ROW.
    #[prost(message, optional, tag = "5")]
    pub row_event: ::core::option::Option<RowEvent>,
    /// FieldEvent is set if the event type is FIELD.
    #[prost(message, optional, tag = "6")]
    pub field_event: ::core::option::Option<FieldEvent>,
    /// Vgtid is set if the event type is VGTID.
    /// This event is only generated by VTGate's VStream function.
    #[prost(message, optional, tag = "7")]
    pub vgtid: ::core::option::Option<VGtid>,
    /// Journal is set if the event type is JOURNAL.
    #[prost(message, optional, tag = "8")]
    pub journal: ::core::option::Option<Journal>,
    /// Dml is set if the event type is INSERT, REPLACE, UPDATE or DELETE.
    #[prost(string, tag = "9")]
    pub dml: ::prost::alloc::string::String,
    /// CurrentTime specifies the current time when the message was sent.
    /// This can be used to compenssate for clock skew.
    #[prost(int64, tag = "20")]
    pub current_time: i64,
    /// LastPK is the last PK for a table
    #[prost(message, optional, tag = "21")]
    pub last_p_k_event: ::core::option::Option<LastPkEvent>,
    /// the source keyspace
    #[prost(string, tag = "22")]
    pub keyspace: ::prost::alloc::string::String,
    /// the source shard
    #[prost(string, tag = "23")]
    pub shard: ::prost::alloc::string::String,
    /// indicate that we are being throttled right now
    #[prost(bool, tag = "24")]
    pub throttled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinimalTable {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<super::query::Field>,
    #[prost(int64, repeated, tag = "3")]
    pub p_k_columns: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinimalSchema {
    #[prost(message, repeated, tag = "1")]
    pub tables: ::prost::alloc::vec::Vec<MinimalTable>,
}
/// VStreamRequest is the payload for VStreamer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VStreamRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<super::query::VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<super::query::Target>,
    #[prost(string, tag = "4")]
    pub position: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub filter: ::core::option::Option<Filter>,
    #[prost(message, repeated, tag = "6")]
    pub table_last_p_ks: ::prost::alloc::vec::Vec<TableLastPk>,
}
/// VStreamResponse is the response from VStreamer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VStreamResponse {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<VEvent>,
}
/// VStreamRowsRequest is the payload for VStreamRows
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VStreamRowsRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<super::query::VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<super::query::Target>,
    #[prost(string, tag = "4")]
    pub query: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub lastpk: ::core::option::Option<super::query::QueryResult>,
}
/// VStreamRowsResponse is the response from VStreamRows
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VStreamRowsResponse {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<super::query::Field>,
    #[prost(message, repeated, tag = "2")]
    pub pkfields: ::prost::alloc::vec::Vec<super::query::Field>,
    #[prost(string, tag = "3")]
    pub gtid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub rows: ::prost::alloc::vec::Vec<super::query::Row>,
    #[prost(message, optional, tag = "5")]
    pub lastpk: ::core::option::Option<super::query::Row>,
    /// Throttled indicates that rowstreamer is being throttled right now
    #[prost(bool, tag = "6")]
    pub throttled: bool,
    /// Heartbeat indicates that this is a heartbeat message
    #[prost(bool, tag = "7")]
    pub heartbeat: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPkEvent {
    #[prost(message, optional, tag = "1")]
    pub table_last_p_k: ::core::option::Option<TableLastPk>,
    #[prost(bool, tag = "2")]
    pub completed: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableLastPk {
    #[prost(string, tag = "1")]
    pub table_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub lastpk: ::core::option::Option<super::query::QueryResult>,
}
/// VStreamResultsRequest is the payload for VStreamResults
/// The ids match VStreamRows, in case we decide to merge the two.
/// The ids match VStreamRows, in case we decide to merge the two.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VStreamResultsRequest {
    #[prost(message, optional, tag = "1")]
    pub effective_caller_id: ::core::option::Option<super::vtrpc::CallerId>,
    #[prost(message, optional, tag = "2")]
    pub immediate_caller_id: ::core::option::Option<super::query::VtGateCallerId>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<super::query::Target>,
    #[prost(string, tag = "4")]
    pub query: ::prost::alloc::string::String,
}
/// VStreamResultsResponse is the response from VStreamResults
/// The ids match VStreamRows, in case we decide to merge the two.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VStreamResultsResponse {
    #[prost(message, repeated, tag = "1")]
    pub fields: ::prost::alloc::vec::Vec<super::query::Field>,
    #[prost(string, tag = "3")]
    pub gtid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub rows: ::prost::alloc::vec::Vec<super::query::Row>,
}
/// OnDDLAction lists the possible actions for DDLs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OnDdlAction {
    Ignore = 0,
    Stop = 1,
    Exec = 2,
    ExecIgnore = 3,
}
impl OnDdlAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OnDdlAction::Ignore => "IGNORE",
            OnDdlAction::Stop => "STOP",
            OnDdlAction::Exec => "EXEC",
            OnDdlAction::ExecIgnore => "EXEC_IGNORE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IGNORE" => Some(Self::Ignore),
            "STOP" => Some(Self::Stop),
            "EXEC" => Some(Self::Exec),
            "EXEC_IGNORE" => Some(Self::ExecIgnore),
            _ => None,
        }
    }
}
/// VReplicationWorkflowType define types of vreplication workflows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VReplicationWorkflowType {
    Materialize = 0,
    MoveTables = 1,
    CreateLookupIndex = 2,
    Migrate = 3,
    Reshard = 4,
    OnlineDdl = 5,
}
impl VReplicationWorkflowType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VReplicationWorkflowType::Materialize => "Materialize",
            VReplicationWorkflowType::MoveTables => "MoveTables",
            VReplicationWorkflowType::CreateLookupIndex => "CreateLookupIndex",
            VReplicationWorkflowType::Migrate => "Migrate",
            VReplicationWorkflowType::Reshard => "Reshard",
            VReplicationWorkflowType::OnlineDdl => "OnlineDDL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Materialize" => Some(Self::Materialize),
            "MoveTables" => Some(Self::MoveTables),
            "CreateLookupIndex" => Some(Self::CreateLookupIndex),
            "Migrate" => Some(Self::Migrate),
            "Reshard" => Some(Self::Reshard),
            "OnlineDDL" => Some(Self::OnlineDdl),
            _ => None,
        }
    }
}
/// VReplicationWorkflowSubType define types of vreplication workflows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VReplicationWorkflowSubType {
    None = 0,
    Partial = 1,
}
impl VReplicationWorkflowSubType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VReplicationWorkflowSubType::None => "None",
            VReplicationWorkflowSubType::Partial => "Partial",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "None" => Some(Self::None),
            "Partial" => Some(Self::Partial),
            _ => None,
        }
    }
}
/// VEventType enumerates the event types. Many of these types
/// will not be encountered in RBR mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VEventType {
    Unknown = 0,
    Gtid = 1,
    Begin = 2,
    Commit = 3,
    Rollback = 4,
    Ddl = 5,
    /// INSERT, REPLACE, UPDATE, DELETE and SET will not be seen in RBR mode.
    Insert = 6,
    Replace = 7,
    Update = 8,
    Delete = 9,
    Set = 10,
    /// OTHER is a dummy event. If encountered, the current GTID must be
    /// recorded by the client to be able to resume.
    Other = 11,
    Row = 12,
    Field = 13,
    /// HEARTBEAT is sent if there is inactivity. If a client does not
    /// receive events beyond the hearbeat interval, it can assume that it's
    /// lost connection to the vstreamer.
    Heartbeat = 14,
    /// VGTID is generated by VTGate's VStream that combines multiple
    /// GTIDs.
    Vgtid = 15,
    Journal = 16,
    Version = 17,
    Lastpk = 18,
    Savepoint = 19,
    /// COPY_COMPLETED is sent when VTGate's VStream copy operation is done.
    /// If a client experiences some disruptions before receiving the event,
    /// the client should restart the copy operation.
    CopyCompleted = 20,
}
impl VEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VEventType::Unknown => "UNKNOWN",
            VEventType::Gtid => "GTID",
            VEventType::Begin => "BEGIN",
            VEventType::Commit => "COMMIT",
            VEventType::Rollback => "ROLLBACK",
            VEventType::Ddl => "DDL",
            VEventType::Insert => "INSERT",
            VEventType::Replace => "REPLACE",
            VEventType::Update => "UPDATE",
            VEventType::Delete => "DELETE",
            VEventType::Set => "SET",
            VEventType::Other => "OTHER",
            VEventType::Row => "ROW",
            VEventType::Field => "FIELD",
            VEventType::Heartbeat => "HEARTBEAT",
            VEventType::Vgtid => "VGTID",
            VEventType::Journal => "JOURNAL",
            VEventType::Version => "VERSION",
            VEventType::Lastpk => "LASTPK",
            VEventType::Savepoint => "SAVEPOINT",
            VEventType::CopyCompleted => "COPY_COMPLETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "GTID" => Some(Self::Gtid),
            "BEGIN" => Some(Self::Begin),
            "COMMIT" => Some(Self::Commit),
            "ROLLBACK" => Some(Self::Rollback),
            "DDL" => Some(Self::Ddl),
            "INSERT" => Some(Self::Insert),
            "REPLACE" => Some(Self::Replace),
            "UPDATE" => Some(Self::Update),
            "DELETE" => Some(Self::Delete),
            "SET" => Some(Self::Set),
            "OTHER" => Some(Self::Other),
            "ROW" => Some(Self::Row),
            "FIELD" => Some(Self::Field),
            "HEARTBEAT" => Some(Self::Heartbeat),
            "VGTID" => Some(Self::Vgtid),
            "JOURNAL" => Some(Self::Journal),
            "VERSION" => Some(Self::Version),
            "LASTPK" => Some(Self::Lastpk),
            "SAVEPOINT" => Some(Self::Savepoint),
            "COPY_COMPLETED" => Some(Self::CopyCompleted),
            _ => None,
        }
    }
}
/// MigrationType specifies the type of migration for the Journal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MigrationType {
    Tables = 0,
    Shards = 1,
}
impl MigrationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MigrationType::Tables => "TABLES",
            MigrationType::Shards => "SHARDS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TABLES" => Some(Self::Tables),
            "SHARDS" => Some(Self::Shards),
            _ => None,
        }
    }
}
