/// RoutingRules specify the high level routing rules for the VSchema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingRules {
    /// rules should ideally be a map. However protos dont't allow
    /// repeated fields as elements of a map. So, we use a list
    /// instead.
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<RoutingRule>,
}
/// RoutingRule specifies a routing rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoutingRule {
    #[prost(string, tag = "1")]
    pub from_table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub to_tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Keyspace is the vschema for a keyspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keyspace {
    /// If sharded is false, vindexes and tables are ignored.
    #[prost(bool, tag = "1")]
    pub sharded: bool,
    #[prost(map = "string, message", tag = "2")]
    pub vindexes: ::std::collections::HashMap<::prost::alloc::string::String, Vindex>,
    #[prost(map = "string, message", tag = "3")]
    pub tables: ::std::collections::HashMap<::prost::alloc::string::String, Table>,
    /// If require_explicit_routing is true, vindexes and tables are not added to global routing
    #[prost(bool, tag = "4")]
    pub require_explicit_routing: bool,
}
/// Vindex is the vindex info for a Keyspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vindex {
    /// The type must match one of the predefined
    /// (or plugged in) vindex names.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// params is a map of attribute value pairs
    /// that must be defined as required by the
    /// vindex constructors. The values can only
    /// be strings.
    #[prost(map = "string, string", tag = "2")]
    pub params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// A lookup vindex can have an owner table defined.
    /// If so, rows in the lookup table are created or
    /// deleted in sync with corresponding rows in the
    /// owner table.
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// Table is the table info for a Keyspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    /// If the table is a sequence, type must be
    /// "sequence".
    ///
    /// If the table is a reference, type must be
    /// "reference".
    /// See <https://vitess.io/docs/reference/features/vschema/#reference-tables.>
    ///
    /// Otherwise, it should be empty.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// column_vindexes associates columns to vindexes.
    #[prost(message, repeated, tag = "2")]
    pub column_vindexes: ::prost::alloc::vec::Vec<ColumnVindex>,
    /// auto_increment is specified if a column needs
    /// to be associated with a sequence.
    #[prost(message, optional, tag = "3")]
    pub auto_increment: ::core::option::Option<AutoIncrement>,
    /// columns lists the columns for the table.
    #[prost(message, repeated, tag = "4")]
    pub columns: ::prost::alloc::vec::Vec<Column>,
    /// pinned pins an unsharded table to a specific
    /// shard, as dictated by the keyspace id.
    /// The keyspace id is represented in hex form
    /// like in keyranges.
    #[prost(string, tag = "5")]
    pub pinned: ::prost::alloc::string::String,
    /// column_list_authoritative is set to true if columns is
    /// an authoritative list for the table. This allows
    /// us to expand 'select *' expressions.
    #[prost(bool, tag = "6")]
    pub column_list_authoritative: bool,
}
/// ColumnVindex is used to associate a column to a vindex.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnVindex {
    /// Legacy implementation, moving forward all vindexes should define a list of columns.
    #[prost(string, tag = "1")]
    pub column: ::prost::alloc::string::String,
    /// The name must match a vindex defined in Keyspace.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// List of columns that define this Vindex
    #[prost(string, repeated, tag = "3")]
    pub columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Autoincrement is used to designate a column as auto-inc.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoIncrement {
    #[prost(string, tag = "1")]
    pub column: ::prost::alloc::string::String,
    /// The sequence must match a table of type SEQUENCE.
    #[prost(string, tag = "2")]
    pub sequence: ::prost::alloc::string::String,
}
/// Column describes a column.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Column {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "super::query::Type", tag = "2")]
    pub r#type: i32,
}
/// SrvVSchema is the roll-up of all the Keyspace schema for a cell.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SrvVSchema {
    /// keyspaces is a map of keyspace name -> Keyspace object.
    #[prost(map = "string, message", tag = "1")]
    pub keyspaces: ::std::collections::HashMap<::prost::alloc::string::String, Keyspace>,
    /// table routing rules
    #[prost(message, optional, tag = "2")]
    pub routing_rules: ::core::option::Option<RoutingRules>,
    #[prost(message, optional, tag = "3")]
    pub shard_routing_rules: ::core::option::Option<ShardRoutingRules>,
}
/// ShardRoutingRules specify the shard routing rules for the VSchema.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardRoutingRules {
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<ShardRoutingRule>,
}
/// RoutingRule specifies a routing rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardRoutingRule {
    #[prost(string, tag = "1")]
    pub from_keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shard: ::prost::alloc::string::String,
}
