/// Shard describes a single shard in a keyspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shard {
    /// name has to be unique in a keyspace. For unsharded keyspaces, it
    /// should be '0'. For sharded keyspace, it should be derived from
    /// the keyrange, like '-80' or '40-80'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// db_name_override is the mysql db name for this shard. Has to be
    /// globally unique. If not specified, we will by default use
    /// 'vt_<keyspace>_<shard>'.
    #[prost(string, tag = "2")]
    pub db_name_override: ::prost::alloc::string::String,
}
/// Keyspace describes a single keyspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keyspace {
    /// name has to be unique in a VTTestTopology.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// shards inside this keyspace. Ignored if redirect is set.
    #[prost(message, repeated, tag = "2")]
    pub shards: ::prost::alloc::vec::Vec<Shard>,
    /// redirects all traffic to another keyspace. If set, shards is ignored.
    #[prost(string, tag = "5")]
    pub served_from: ::prost::alloc::string::String,
    /// number of replica tablets to instantiate. This includes the primary tablet.
    #[prost(int32, tag = "6")]
    pub replica_count: i32,
    /// number of rdonly tablets to instantiate.
    #[prost(int32, tag = "7")]
    pub rdonly_count: i32,
}
/// VTTestTopology describes the keyspaces in the topology.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VtTestTopology {
    /// all keyspaces in the topology.
    #[prost(message, repeated, tag = "1")]
    pub keyspaces: ::prost::alloc::vec::Vec<Keyspace>,
    /// list of cells the keyspaces reside in. Vtgate is started in only the first cell.
    #[prost(string, repeated, tag = "2")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// routing rules for the topology.
    #[prost(message, optional, tag = "3")]
    pub routing_rules: ::core::option::Option<super::vschema::RoutingRules>,
}
