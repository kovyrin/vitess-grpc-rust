/// KeyRange describes a range of sharding keys, when range-based
/// sharding is used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyRange {
    #[prost(bytes = "vec", tag = "1")]
    pub start: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub end: ::prost::alloc::vec::Vec<u8>,
}
/// TabletAlias is a globally unique tablet identifier.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabletAlias {
    /// cell is the cell (or datacenter) the tablet is in
    #[prost(string, tag = "1")]
    pub cell: ::prost::alloc::string::String,
    /// uid is a unique id for this tablet within the shard
    /// (this is the MySQL server id as well).
    #[prost(uint32, tag = "2")]
    pub uid: u32,
}
/// Tablet represents information about a running instance of vttablet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tablet {
    /// alias is the unique name of the tablet.
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<TabletAlias>,
    /// Fully qualified domain name of the host.
    #[prost(string, tag = "2")]
    pub hostname: ::prost::alloc::string::String,
    /// Map of named ports. Normally this should include vt and grpc.
    /// Going forward, the mysql port will be stored in mysql_port
    /// instead of here.
    /// For accessing mysql port, use topoproto.MysqlPort to fetch, and
    /// topoproto.SetMysqlPort to set. These wrappers will ensure
    /// legacy behavior is supported.
    #[prost(map = "string, int32", tag = "4")]
    pub port_map: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
    /// Keyspace name.
    #[prost(string, tag = "5")]
    pub keyspace: ::prost::alloc::string::String,
    /// Shard name. If range based sharding is used, it should match
    /// key_range.
    #[prost(string, tag = "6")]
    pub shard: ::prost::alloc::string::String,
    /// If range based sharding is used, range for the tablet's shard.
    #[prost(message, optional, tag = "7")]
    pub key_range: ::core::option::Option<KeyRange>,
    /// type is the current type of the tablet.
    #[prost(enumeration = "TabletType", tag = "8")]
    pub r#type: i32,
    /// It this is set, it is used as the database name instead of the
    /// normal "vt_" + keyspace.
    #[prost(string, tag = "9")]
    pub db_name_override: ::prost::alloc::string::String,
    /// tablet tags
    #[prost(map = "string, string", tag = "10")]
    pub tags: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// MySQL hostname.
    #[prost(string, tag = "12")]
    pub mysql_hostname: ::prost::alloc::string::String,
    /// MySQL port. Use topoproto.MysqlPort and topoproto.SetMysqlPort
    /// to access this variable. The functions provide support
    /// for legacy behavior.
    #[prost(int32, tag = "13")]
    pub mysql_port: i32,
    /// primary_term_start_time is the time (in UTC) at which the current term of
    /// the current tablet began as primary. If this tablet is not currently the
    /// primary, this value is ignored.
    ///
    /// A new primary term begins any time an authoritative decision is communicated
    /// about which tablet should be the primary, such as via Vitess
    /// replication-management commands like PlannedReparentShard,
    /// EmergencyReparentShard, and TabletExternallyReparented.
    ///
    #[prost(message, optional, tag = "14")]
    pub primary_term_start_time: ::core::option::Option<super::vttime::Time>,
    /// db_server_version represents the database version used by the tablet.
    #[prost(string, tag = "15")]
    pub db_server_version: ::prost::alloc::string::String,
    /// default_conn_collation is the default connection collation used by this tablet.
    #[prost(uint32, tag = "16")]
    pub default_conn_collation: u32,
}
/// A Shard contains data about a subset of the data whithin a keyspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shard {
    /// primary_alias is the tablet alias of the primary for the shard.
    /// If it is unset, then there is no primary in this shard yet.
    /// No lock is necessary to update this field, when for instance
    /// TabletExternallyReparented updates this. However, we lock the
    /// shard for reparenting operations (InitShardPrimary,
    /// PlannedReparentShard,EmergencyReparentShard), to guarantee
    /// exclusive operation.
    #[prost(message, optional, tag = "1")]
    pub primary_alias: ::core::option::Option<TabletAlias>,
    /// primary_term_start_time is the time (in UTC) at which the current term of
    /// the primary specified in primary_alias began.
    ///
    /// A new primary term begins any time an authoritative decision is communicated
    /// about which tablet should be the primary, such as via Vitess
    /// replication-management commands like PlannedReparentShard,
    /// EmergencyReparentShard, and TabletExternallyReparented.
    ///
    /// The primary_alias should only ever be changed if the new primary's term began
    /// at a later time than this. Note that a new term can start for the tablet
    /// that is already the primary. In that case, the primary_term_start_time would
    /// be increased without changing the primary_alias.
    #[prost(message, optional, tag = "8")]
    pub primary_term_start_time: ::core::option::Option<super::vttime::Time>,
    /// key_range is the KeyRange for this shard. It can be unset if:
    /// - we are not using range-based sharding in this shard.
    /// - the shard covers the entire keyrange.
    /// This must match the shard name based on our other conventions, but
    /// helpful to have it decomposed here.
    /// Once set at creation time, it is never changed.
    #[prost(message, optional, tag = "2")]
    pub key_range: ::core::option::Option<KeyRange>,
    /// SourceShards is the list of shards we're replicating from,
    /// using filtered replication.
    /// The keyspace lock is always taken when changing this.
    #[prost(message, repeated, tag = "4")]
    pub source_shards: ::prost::alloc::vec::Vec<shard::SourceShard>,
    /// tablet_controls has at most one entry per TabletType.
    /// The keyspace lock is always taken when changing this.
    #[prost(message, repeated, tag = "6")]
    pub tablet_controls: ::prost::alloc::vec::Vec<shard::TabletControl>,
    /// is_primary_serving sets whether this shard primary is serving traffic or not.
    /// The keyspace lock is always taken when changing this.
    #[prost(bool, tag = "7")]
    pub is_primary_serving: bool,
}
/// Nested message and enum types in `Shard`.
pub mod shard {
    /// SourceShard represents a data source for filtered replication
    /// across shards. When this is used in a destination shard, the primary
    /// of that shard will run filtered replication.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourceShard {
        /// Uid is the unique ID for this SourceShard object.
        #[prost(uint32, tag = "1")]
        pub uid: u32,
        /// the source keyspace
        #[prost(string, tag = "2")]
        pub keyspace: ::prost::alloc::string::String,
        /// the source shard
        #[prost(string, tag = "3")]
        pub shard: ::prost::alloc::string::String,
        /// the source shard keyrange
        #[prost(message, optional, tag = "4")]
        pub key_range: ::core::option::Option<super::KeyRange>,
        /// the source table list to replicate
        #[prost(string, repeated, tag = "5")]
        pub tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// TabletControl controls tablet's behavior
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TabletControl {
        /// which tablet type is affected
        #[prost(enumeration = "super::TabletType", tag = "1")]
        pub tablet_type: i32,
        #[prost(string, repeated, tag = "2")]
        pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(string, repeated, tag = "4")]
        pub denied_tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// frozen is set if we've started failing over traffic for
        /// the primary. If set, this record should not be removed.
        #[prost(bool, tag = "5")]
        pub frozen: bool,
    }
}
/// A Keyspace contains data about a keyspace.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keyspace {
    /// ServedFrom will redirect the appropriate traffic to
    /// another keyspace.
    #[prost(message, repeated, tag = "4")]
    pub served_froms: ::prost::alloc::vec::Vec<keyspace::ServedFrom>,
    /// keyspace_type will determine how this keyspace is treated by
    /// vtgate / vschema. Normal keyspaces are routable by
    /// any query. Snapshot keyspaces are only accessible
    /// by explicit addresssing or by calling "use keyspace" first
    #[prost(enumeration = "KeyspaceType", tag = "5")]
    pub keyspace_type: i32,
    /// base_keyspace is the base keyspace from which a snapshot
    /// keyspace is created. empty for normal keyspaces
    #[prost(string, tag = "6")]
    pub base_keyspace: ::prost::alloc::string::String,
    /// snapshot_time (in UTC) is a property of snapshot
    /// keyspaces which tells us what point in time
    /// the snapshot is of
    #[prost(message, optional, tag = "7")]
    pub snapshot_time: ::core::option::Option<super::vttime::Time>,
    /// DurabilityPolicy is the durability policy to be
    /// used for the keyspace.
    #[prost(string, tag = "8")]
    pub durability_policy: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Keyspace`.
pub mod keyspace {
    /// ServedFrom indicates a relationship between a TabletType and the
    /// keyspace name that's serving it.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServedFrom {
        /// the tablet type (key for the map)
        #[prost(enumeration = "super::TabletType", tag = "1")]
        pub tablet_type: i32,
        /// the cells to limit this to
        #[prost(string, repeated, tag = "2")]
        pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// the keyspace name that's serving it
        #[prost(string, tag = "3")]
        pub keyspace: ::prost::alloc::string::String,
    }
}
/// ShardReplication describes the MySQL replication relationships
/// whithin a cell.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReplication {
    /// Note there can be only one Node in this array
    /// for a given tablet.
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<shard_replication::Node>,
}
/// Nested message and enum types in `ShardReplication`.
pub mod shard_replication {
    /// Node describes a tablet instance within the cell
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Node {
        #[prost(message, optional, tag = "1")]
        pub tablet_alias: ::core::option::Option<super::TabletAlias>,
    }
}
/// ShardReplicationError describes the error being fixed by
/// ShardReplicationFix.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReplicationError {
    /// Type is the category of problem being fixed.
    #[prost(enumeration = "shard_replication_error::Type", tag = "1")]
    pub r#type: i32,
    /// TabletAlias is the tablet record that has the problem.
    #[prost(message, optional, tag = "2")]
    pub tablet_alias: ::core::option::Option<TabletAlias>,
}
/// Nested message and enum types in `ShardReplicationError`.
pub mod shard_replication_error {
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
    pub enum Type {
        /// UNKNOWN is not a valid value.
        Unknown = 0,
        /// NOT_FOUND occurs when a tablet is in the ShardReplication record
        /// but does not exist in the topology.
        NotFound = 1,
        /// TOPOLOGY_MISMATCH occurs when a tablet is in the replication graph and
        /// exists in the topology, but at least one of the Keyspace, Shard, or Cell
        /// fields for that tablet does not match the ShardReplication record.
        TopologyMismatch = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unknown => "UNKNOWN",
                Type::NotFound => "NOT_FOUND",
                Type::TopologyMismatch => "TOPOLOGY_MISMATCH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "NOT_FOUND" => Some(Self::NotFound),
                "TOPOLOGY_MISMATCH" => Some(Self::TopologyMismatch),
                _ => None,
            }
        }
    }
}
/// ShardReference is used as a pointer from a SrvKeyspace to a Shard
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReference {
    /// Copied from Shard.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Disable query serving in this shard
    #[prost(message, optional, tag = "2")]
    pub key_range: ::core::option::Option<KeyRange>,
}
/// ShardTabletControl is used as a pointer from a SrvKeyspace to a Shard
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardTabletControl {
    /// Copied from Shard.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub key_range: ::core::option::Option<KeyRange>,
    /// Disable query serving in this shard
    #[prost(bool, tag = "3")]
    pub query_service_disabled: bool,
}
/// SrvKeyspace is a rollup node for the keyspace itself.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SrvKeyspace {
    /// The partitions this keyspace is serving, per tablet type.
    #[prost(message, repeated, tag = "1")]
    pub partitions: ::prost::alloc::vec::Vec<srv_keyspace::KeyspacePartition>,
    #[prost(message, repeated, tag = "4")]
    pub served_from: ::prost::alloc::vec::Vec<srv_keyspace::ServedFrom>,
}
/// Nested message and enum types in `SrvKeyspace`.
pub mod srv_keyspace {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyspacePartition {
        /// The type this partition applies to.
        #[prost(enumeration = "super::TabletType", tag = "1")]
        pub served_type: i32,
        /// List of non-overlapping continuous shards sorted by range.
        #[prost(message, repeated, tag = "2")]
        pub shard_references: ::prost::alloc::vec::Vec<super::ShardReference>,
        /// List of shard tablet controls
        #[prost(message, repeated, tag = "3")]
        pub shard_tablet_controls: ::prost::alloc::vec::Vec<super::ShardTabletControl>,
    }
    /// ServedFrom indicates a relationship between a TabletType and the
    /// keyspace name that's serving it.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ServedFrom {
        /// the tablet type
        #[prost(enumeration = "super::TabletType", tag = "1")]
        pub tablet_type: i32,
        /// the keyspace name that's serving it
        #[prost(string, tag = "2")]
        pub keyspace: ::prost::alloc::string::String,
    }
}
/// CellInfo contains information about a cell. CellInfo objects are
/// stored in the global topology server, and describe how to reach
/// local topology servers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CellInfo {
    /// ServerAddress contains the address of the server for the cell.
    /// The syntax of this field is topology implementation specific.
    /// For instance, for Zookeeper, it is a comma-separated list of
    /// server addresses.
    #[prost(string, tag = "1")]
    pub server_address: ::prost::alloc::string::String,
    /// Root is the path to store data in. It is only used when talking
    /// to server_address.
    #[prost(string, tag = "2")]
    pub root: ::prost::alloc::string::String,
}
/// CellsAlias
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CellsAlias {
    /// Cells that map to this alias
    #[prost(string, repeated, tag = "2")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopoConfig {
    #[prost(string, tag = "1")]
    pub topo_type: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub server: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub root: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalVitessCluster {
    #[prost(message, optional, tag = "1")]
    pub topo_config: ::core::option::Option<TopoConfig>,
}
/// ExternalClusters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalClusters {
    #[prost(message, repeated, tag = "1")]
    pub vitess_cluster: ::prost::alloc::vec::Vec<ExternalVitessCluster>,
}
/// KeyspaceType describes the type of the keyspace
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyspaceType {
    /// NORMAL is the default value
    Normal = 0,
    /// SNAPSHOT is when we are creating a snapshot keyspace
    Snapshot = 1,
}
impl KeyspaceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyspaceType::Normal => "NORMAL",
            KeyspaceType::Snapshot => "SNAPSHOT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL" => Some(Self::Normal),
            "SNAPSHOT" => Some(Self::Snapshot),
            _ => None,
        }
    }
}
/// TabletType represents the type of a given tablet.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TabletType {
    /// UNKNOWN is not a valid value.
    Unknown = 0,
    /// PRIMARY is the primary server for the shard. Only PRIMARY allows DMLs.
    Primary = 1,
    /// REPLICA replicates from primary. It is used to serve live traffic.
    /// A REPLICA can be promoted to PRIMARY. A demoted PRIMARY will go to REPLICA.
    Replica = 2,
    /// RDONLY (old name) / BATCH (new name) is used to serve traffic for
    /// long-running jobs. It is a separate type from REPLICA so
    /// long-running queries don't affect web-like traffic.
    Rdonly = 3,
    /// SPARE is a type of servers that cannot serve queries, but is available
    /// in case an extra server is needed.
    Spare = 4,
    /// EXPERIMENTAL is like SPARE, except it can serve queries. This
    /// type can be used for usages not planned by Vitess, like online
    /// export to another storage engine.
    Experimental = 5,
    /// BACKUP is the type a server goes to when taking a backup. No queries
    /// can be served in BACKUP mode.
    Backup = 6,
    /// RESTORE is the type a server uses when restoring a backup, at
    /// startup time.  No queries can be served in RESTORE mode.
    Restore = 7,
    /// DRAINED is the type a server goes into when used by Vitess tools
    /// to perform an offline action. It is a serving type (as
    /// the tools processes may need to run queries), but it's not used
    /// to route queries from Vitess users. In this state,
    /// this tablet is dedicated to the process that uses it.
    Drained = 8,
}
impl TabletType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TabletType::Unknown => "UNKNOWN",
            TabletType::Primary => "PRIMARY",
            TabletType::Replica => "REPLICA",
            TabletType::Rdonly => "RDONLY",
            TabletType::Spare => "SPARE",
            TabletType::Experimental => "EXPERIMENTAL",
            TabletType::Backup => "BACKUP",
            TabletType::Restore => "RESTORE",
            TabletType::Drained => "DRAINED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "PRIMARY" => Some(Self::Primary),
            "REPLICA" => Some(Self::Replica),
            "RDONLY" => Some(Self::Rdonly),
            "SPARE" => Some(Self::Spare),
            "EXPERIMENTAL" => Some(Self::Experimental),
            "BACKUP" => Some(Self::Backup),
            "RESTORE" => Some(Self::Restore),
            "DRAINED" => Some(Self::Drained),
            _ => None,
        }
    }
}
