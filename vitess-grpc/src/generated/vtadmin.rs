/// Cluster represents information about a Vitess cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterBackup {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(message, optional, tag = "2")]
    pub backup: ::core::option::Option<super::mysqlctl::BackupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterCellsAliases {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(map = "string, message", tag = "2")]
    pub aliases: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::topodata::CellsAlias,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterCellInfo {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// CellInfo contains the data for the cell.
    ///
    /// It may be nil if the GetCellsInfosRequest specified NamesOnly.
    #[prost(message, optional, tag = "3")]
    pub cell_info: ::core::option::Option<super::topodata::CellInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterShardReplicationPosition {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub position_info: ::core::option::Option<
        super::vtctldata::ShardReplicationPositionsResponse,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterWorkflows {
    #[prost(message, repeated, tag = "1")]
    pub workflows: ::prost::alloc::vec::Vec<Workflow>,
    /// Warnings is a list of non-fatal errors encountered when fetching
    /// workflows for a particular cluster.
    #[prost(string, repeated, tag = "2")]
    pub warnings: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Keyspace represents information about a keyspace in a particular Vitess
/// cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keyspace {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(message, optional, tag = "2")]
    pub keyspace: ::core::option::Option<super::vtctldata::Keyspace>,
    #[prost(map = "string, message", tag = "3")]
    pub shards: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::vtctldata::Shard,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub table_definitions: ::prost::alloc::vec::Vec<
        super::tabletmanagerdata::TableDefinition,
    >,
    /// TableSizes is a mapping of table name to TableSize information.
    #[prost(map = "string, message", tag = "4")]
    pub table_sizes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        schema::TableSize,
    >,
}
/// Nested message and enum types in `Schema`.
pub mod schema {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShardTableSize {
        #[prost(uint64, tag = "1")]
        pub row_count: u64,
        #[prost(uint64, tag = "2")]
        pub data_length: u64,
    }
    /// TableSize aggregates table size information across all shards containing
    /// in the given keyspace and cluster, as well as per-shard size information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TableSize {
        #[prost(uint64, tag = "1")]
        pub row_count: u64,
        #[prost(uint64, tag = "2")]
        pub data_length: u64,
        #[prost(map = "string, message", tag = "3")]
        pub by_shard: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ShardTableSize,
        >,
    }
}
/// Shard groups the vtctldata information about a shard record together with
/// the Vitess cluster it belongs to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shard {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(message, optional, tag = "2")]
    pub shard: ::core::option::Option<super::vtctldata::Shard>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SrvVSchema {
    #[prost(string, tag = "1")]
    pub cell: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(message, optional, tag = "3")]
    pub srv_v_schema: ::core::option::Option<super::vschema::SrvVSchema>,
}
/// Tablet groups the topo information of a tablet together with the Vitess
/// cluster it belongs to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tablet {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(message, optional, tag = "2")]
    pub tablet: ::core::option::Option<super::topodata::Tablet>,
    #[prost(enumeration = "tablet::ServingState", tag = "3")]
    pub state: i32,
    #[prost(string, tag = "4")]
    pub fqdn: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Tablet`.
pub mod tablet {
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
    pub enum ServingState {
        Unknown = 0,
        Serving = 1,
        NotServing = 2,
    }
    impl ServingState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServingState::Unknown => "UNKNOWN",
                ServingState::Serving => "SERVING",
                ServingState::NotServing => "NOT_SERVING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNKNOWN" => Some(Self::Unknown),
                "SERVING" => Some(Self::Serving),
                "NOT_SERVING" => Some(Self::NotServing),
                _ => None,
            }
        }
    }
}
/// VSchema represents the vschema for a keyspace in the cluster it belongs to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VSchema {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Name is the name of the keyspace this VSchema is for.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub v_schema: ::core::option::Option<super::vschema::Keyspace>,
}
/// Vtctld represents information about a single Vtctld host.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vtctld {
    #[prost(string, tag = "1")]
    pub hostname: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(string, tag = "3")]
    pub fqdn: ::prost::alloc::string::String,
}
/// VTGate represents information about a single VTGate host.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VtGate {
    /// Hostname is the shortname of the VTGate.
    #[prost(string, tag = "1")]
    pub hostname: ::prost::alloc::string::String,
    /// Pool is group the VTGate serves queries for. Some deployments segment
    /// VTGates into groups or pools, based on the workloads they serve queries
    /// for. Use of this field is optional.
    #[prost(string, tag = "2")]
    pub pool: ::prost::alloc::string::String,
    /// Cell is the topology cell the VTGate is in.
    #[prost(string, tag = "3")]
    pub cell: ::prost::alloc::string::String,
    /// Cluster is the cluster the VTGate serves.
    #[prost(message, optional, tag = "4")]
    pub cluster: ::core::option::Option<Cluster>,
    /// Keyspaces is the list of keyspaces-to-watch for the VTGate.
    #[prost(string, repeated, tag = "5")]
    pub keyspaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "6")]
    pub fqdn: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workflow {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub workflow: ::core::option::Option<super::vtctldata::Workflow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<super::vtctldata::CreateKeyspaceRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyspaceResponse {
    #[prost(message, optional, tag = "1")]
    pub keyspace: ::core::option::Option<Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShardRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<super::vtctldata::CreateShardRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<super::vtctldata::DeleteKeyspaceRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShardsRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<super::vtctldata::DeleteShardsRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTabletRequest {
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "3")]
    pub allow_primary: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTabletResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmergencyFailoverShardRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<super::vtctldata::EmergencyReparentShardRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmergencyFailoverShardResponse {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shard: ::prost::alloc::string::String,
    /// PromotedPrimary is the tablet alias that was promoted to shard primary.
    /// If NewPrimary was set in the request options, then this will be the
    /// same tablet alias. Otherwise, it will be the alias of the tablet found
    /// to be most up-to-date in the shard.
    #[prost(message, optional, tag = "4")]
    pub promoted_primary: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(message, repeated, tag = "5")]
    pub events: ::prost::alloc::vec::Vec<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindSchemaRequest {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub table_size_options: ::core::option::Option<GetSchemaTableSizeOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupsRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Keyspaces, if set, limits backups to just the specified keyspaces.
    /// Applies to all clusters in the request.
    #[prost(string, repeated, tag = "2")]
    pub keyspaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// KeyspaceShards, if set, limits backups to just the specified
    /// keyspace/shards. Applies to all clusters in the request.
    ///
    /// This field takes precedence over Keyspaces. If KeyspaceShards is set,
    /// Keyspaces is ignored.
    #[prost(string, repeated, tag = "3")]
    pub keyspace_shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// RequestOptions controls the per-shard request options when making
    /// GetBackups requests to vtctlds. Note that the Keyspace and Shard fields
    /// of this field are ignored; it is used only to specify Limit and Detailed
    /// fields.
    #[prost(message, optional, tag = "4")]
    pub request_options: ::core::option::Option<super::vtctldata::GetBackupsRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupsResponse {
    #[prost(message, repeated, tag = "1")]
    pub backups: ::prost::alloc::vec::Vec<ClusterBackup>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCellInfosRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Cells, if specified, limits the response to include only CellInfo objects
    /// with those names. If omitted, all CellInfo objects in each cluster are
    /// returned.
    ///
    /// Mutually-exclusive with NamesOnly. If both are set, this field takes
    /// precedence.
    #[prost(string, repeated, tag = "2")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Return only the cell names in each cluster; the actual CellInfo objects
    /// will be empty.
    #[prost(bool, tag = "3")]
    pub names_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCellInfosResponse {
    #[prost(message, repeated, tag = "1")]
    pub cell_infos: ::prost::alloc::vec::Vec<ClusterCellInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCellsAliasesRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCellsAliasesResponse {
    #[prost(message, repeated, tag = "1")]
    pub aliases: ::prost::alloc::vec::Vec<ClusterCellsAliases>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClustersRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClustersResponse {
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFullStatusRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGatesRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub gates: ::prost::alloc::vec::Vec<VtGate>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyspacesRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyspacesResponse {
    #[prost(message, repeated, tag = "1")]
    pub keyspaces: ::prost::alloc::vec::Vec<Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub table_size_options: ::core::option::Option<GetSchemaTableSizeOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemasRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub table_size_options: ::core::option::Option<GetSchemaTableSizeOptions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemasResponse {
    #[prost(message, repeated, tag = "1")]
    pub schemas: ::prost::alloc::vec::Vec<Schema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShardReplicationPositionsRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Keyspaces, if set, limits replication positions to just the specified
    /// keyspaces. Applies to all clusters in the request.
    #[prost(string, repeated, tag = "2")]
    pub keyspaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// KeyspaceShards, if set, limits replicatin positions to just the specified
    /// keyspace/shards. Applies to all clusters in the request.
    ///
    /// This field takes precedence over Keyspaces. If KeyspaceShards is set,
    /// Keyspaces is ignored.
    #[prost(string, repeated, tag = "3")]
    pub keyspace_shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShardReplicationPositionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub replication_positions: ::prost::alloc::vec::Vec<ClusterShardReplicationPosition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    /// Cells is a list of cells to lookup a SrvKeyspace for. Leaving this empty is
    /// equivalent to specifying all cells in the topo.
    #[prost(string, repeated, tag = "3")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvKeyspacesRequest {
    /// An optional list of cluster IDs to filter specific clusters
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Cells is a list of cells to lookup a SrvKeyspace for. Leaving this empty is
    /// equivalent to specifying all cells in the topo.
    #[prost(string, repeated, tag = "2")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvKeyspacesResponse {
    /// GetSrvKeyspaces responses for each keyspace
    #[prost(map = "string, message", tag = "1")]
    pub srv_keyspaces: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::vtctldata::GetSrvKeyspacesResponse,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvVSchemaRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cell: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvVSchemasRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvVSchemasResponse {
    #[prost(message, repeated, tag = "1")]
    pub srv_v_schemas: ::prost::alloc::vec::Vec<SrvVSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaTableSizeOptions {
    #[prost(bool, tag = "1")]
    pub aggregate_sizes: bool,
    #[prost(bool, tag = "2")]
    pub include_non_serving_shards: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTabletRequest {
    /// Unique (per cluster) tablet alias.
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    /// ClusterIDs is an optional parameter to narrow the scope of the search, if
    /// the caller knows which cluster the tablet may be in, or, to disambiguate
    /// if multiple clusters have a tablet with the same hostname.
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTabletsRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTabletsResponse {
    #[prost(message, repeated, tag = "1")]
    pub tablets: ::prost::alloc::vec::Vec<Tablet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopologyPathRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVSchemaRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVSchemasRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVSchemasResponse {
    #[prost(message, repeated, tag = "1")]
    pub v_schemas: ::prost::alloc::vec::Vec<VSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVtctldsRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVtctldsResponse {
    #[prost(message, repeated, tag = "1")]
    pub vtctlds: ::prost::alloc::vec::Vec<Vtctld>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkflowRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub active_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkflowsRequest {
    #[prost(string, repeated, tag = "1")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ActiveOnly specifies whether to return workflows that are currently
    /// active (running or paused) instead of all workflows.
    #[prost(bool, tag = "2")]
    pub active_only: bool,
    /// Keyspaces is a list of keyspaces to restrict the workflow search to. Note
    /// that the keyspaces list applies across all cluster IDs in the request.
    ///
    /// If, for example, you have two clusters, each with a keyspace called "foo"
    /// and want the workflows from "foo" in cluster1 but not from cluster2, you
    /// must make two requests.
    ///
    /// Keyspaces and IgnoreKeyspaces are mutually-exclusive, and Keyspaces takes
    /// precedence; if Keyspaces is a non-empty list, then IgnoreKeyspaces is
    /// ignored completely.
    #[prost(string, repeated, tag = "3")]
    pub keyspaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// IgnoreKeyspaces is a list of keyspaces to skip during the workflow
    /// search. It has the same semantics as the Keyspaces parameter, so refer to
    /// that documentation for more details.
    #[prost(string, repeated, tag = "4")]
    pub ignore_keyspaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkflowsResponse {
    #[prost(map = "string, message", tag = "1")]
    pub workflows_by_cluster: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ClusterWorkflows,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingTabletRequest {
    /// Unique (per cluster) tablet alias of the standard form: "$cell-$uid"
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    /// ClusterIDs is an optional parameter to narrow the scope of the search, if
    /// the caller knows which cluster the tablet may be in, or, to disambiguate
    /// if multiple clusters have a tablet with the same hostname.
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingTabletResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannedFailoverShardRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub options: ::core::option::Option<super::vtctldata::PlannedReparentShardRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannedFailoverShardResponse {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shard: ::prost::alloc::string::String,
    /// PromotedPrimary is the tablet alias that was promoted to shard primary.
    /// If NewPrimary was set in the request options, then this will be the
    /// same tablet alias. Otherwise, it will be the alias of the tablet found
    /// to be most up-to-date in the shard.
    #[prost(message, optional, tag = "4")]
    pub promoted_primary: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(message, repeated, tag = "5")]
    pub events: ::prost::alloc::vec::Vec<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebuildKeyspaceGraphRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "4")]
    pub allow_partial: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebuildKeyspaceGraphResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshStateRequest {
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshStateResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemasRequest {
    /// Keyspaces, if set, will reload schemas across one or more keyspaces. A
    /// keyspace not existing in a cluster will not fail the overall request.
    ///
    /// Superceded by KeyspaceShards and Tablets, in that order.
    #[prost(string, repeated, tag = "1")]
    pub keyspaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// KeyspaceShards, if set, will reload schemas across one or more shards.
    /// Each element must be a valid keyspace/shard according to
    /// topoproto.ParseKeyspaceShard. A shard not existing in a cluster will not
    /// fail the overall request.
    ///
    /// Supercedes Keyspaces, and is superceded by Tablets.
    #[prost(string, repeated, tag = "2")]
    pub keyspace_shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Tablets, if set will reload schemas across one or more tablets.
    /// Supercedes both Keyspaces and KeyspaceShards.
    #[prost(message, repeated, tag = "3")]
    pub tablets: ::prost::alloc::vec::Vec<super::topodata::TabletAlias>,
    /// ClusterIds optionally restricts the reload operation to clusters with
    /// the specified IDs. An empty list of ClusterIds will operate on all
    /// clusters.
    #[prost(string, repeated, tag = "4")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Concurrency controls the number of tablets to reload at any given time.
    /// Its semantics depend on whether the request is for keyspace, shard, or
    /// tablet mode.
    ///
    /// In Keyspaces mode, Concurrency is the number of tablets to reload at once
    /// *per keyspace*.
    ///
    /// In KeyspaceShards mode, Concurrency is the number of tablets to reload at
    /// once *per shard*.
    ///
    /// In Tablets mode, Concurrency is the number of tablets to reload at once
    /// *per cluster*.
    #[prost(uint32, tag = "5")]
    pub concurrency: u32,
    /// WaitPosition is the replication position that replicating tablets should
    /// reach prior to reloading their schemas.
    ///
    /// Does not apply in Tablets mode.
    #[prost(string, tag = "6")]
    pub wait_position: ::prost::alloc::string::String,
    /// IncludePrimary, if set, will reload the schemas on PRIMARY tablets as
    /// well as REPLICA and RDONLY.
    ///
    /// Does not apply in Tablets mode.
    #[prost(bool, tag = "7")]
    pub include_primary: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemasResponse {
    /// KeyspaceResults is the list of KeyspaceResult objects for a ReloadSchemas
    /// operation. It is only set when the request mandates Keyspaces mode (see
    /// ReloadSchemasRequest).
    #[prost(message, repeated, tag = "1")]
    pub keyspace_results: ::prost::alloc::vec::Vec<
        reload_schemas_response::KeyspaceResult,
    >,
    /// ShardResults is the list of ShardResult objects for a ReloadSchemas
    /// operation. It is only set when the request mandates KeyspaceShards mode
    /// (see ReloadSchemasRequest).
    #[prost(message, repeated, tag = "2")]
    pub shard_results: ::prost::alloc::vec::Vec<reload_schemas_response::ShardResult>,
    /// TabletResults is the list of TabletResult objects for a ReloadSchemas
    /// operation. It is only set when the request mandates Tablets mode (see
    /// ReloadSchemasRequest).
    #[prost(message, repeated, tag = "3")]
    pub tablet_results: ::prost::alloc::vec::Vec<reload_schemas_response::TabletResult>,
}
/// Nested message and enum types in `ReloadSchemasResponse`.
pub mod reload_schemas_response {
    /// KeyspaceResult is a grouping of a Keyspace and any log events that
    /// occurred in that keyspace during a schema reload (usually associated with
    /// partial errors - ReloadSchemas requests are best-effort).
    ///
    /// It is only set when a ReloadSchemas request mandates Keyspaces mode
    /// (see ReloadSchemasRequest).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeyspaceResult {
        #[prost(message, optional, tag = "1")]
        pub keyspace: ::core::option::Option<super::Keyspace>,
        #[prost(message, repeated, tag = "2")]
        pub events: ::prost::alloc::vec::Vec<super::super::logutil::Event>,
    }
    /// ShardResult is a grouping of a Shard and any log events that occurred in
    /// that shard during a schema reload (usually associated with partial
    /// errors - ReloadSchemas requests are best-effort).
    ///
    /// It is only set when a ReloadSchemas request mandates KeyspaceShards mode
    /// (see ReloadSchemasRequest).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShardResult {
        #[prost(message, optional, tag = "1")]
        pub shard: ::core::option::Option<super::Shard>,
        #[prost(message, repeated, tag = "2")]
        pub events: ::prost::alloc::vec::Vec<super::super::logutil::Event>,
    }
    /// TabletResult is a grouping of a Tablet and the result of reloading that
    /// Tablet's schema. Result will either be the string "ok", or the error
    /// message from that tablet. Note ReloadSchemas is best-effort, so tablet's
    /// failing to reload is not treated as an overall failure.
    ///
    /// It is only set when a ReloadSchemas request mandates Tablets mode (see
    /// ReloadSchemasRequest).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TabletResult {
        #[prost(message, optional, tag = "1")]
        pub tablet: ::core::option::Option<super::Tablet>,
        #[prost(string, tag = "2")]
        pub result: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemaShardRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shard: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub wait_position: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub include_primary: bool,
    #[prost(uint32, tag = "6")]
    pub concurrency: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemaShardResponse {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshTabletReplicationSourceRequest {
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshTabletReplicationSourceResponse {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub primary: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(message, optional, tag = "4")]
    pub cluster: ::core::option::Option<Cluster>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveKeyspaceCellRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cell: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub force: bool,
    #[prost(bool, tag = "5")]
    pub recursive: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveKeyspaceCellResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunHealthCheckRequest {
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunHealthCheckResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReadOnlyRequest {
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReadOnlyResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReadWriteRequest {
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReadWriteResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartReplicationRequest {
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartReplicationResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationRequest {
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cluster: ::core::option::Option<Cluster>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabletExternallyPromotedRequest {
    /// Tablet is the alias of the tablet that was promoted externally and should
    /// be updated to the shard primary in the topo.
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabletExternallyPromotedResponse {
    #[prost(message, optional, tag = "1")]
    pub cluster: ::core::option::Option<Cluster>,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub new_primary: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(message, optional, tag = "5")]
    pub old_primary: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabletExternallyReparentedRequest {
    #[prost(message, optional, tag = "1")]
    pub alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, repeated, tag = "2")]
    pub cluster_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub ping_tablets: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub ping_tablets: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateSchemaKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateShardRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shard: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub ping_tablets: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateVersionKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateVersionShardRequest {
    #[prost(string, tag = "1")]
    pub cluster_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shard: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VtExplainRequest {
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sql: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VtExplainResponse {
    #[prost(string, tag = "1")]
    pub response: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod vt_admin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// VTAdmin is the Vitess Admin API service. It provides RPCs that operate on
    /// across a range of Vitess clusters.
    #[derive(Debug, Clone)]
    pub struct VtAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VtAdminClient<tonic::transport::Channel> {
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
    impl<T> VtAdminClient<T>
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
        ) -> VtAdminClient<InterceptedService<T, F>>
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
            VtAdminClient::new(InterceptedService::new(inner, interceptor))
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
        /// CreateKeyspace creates a new keyspace in the given cluster.
        pub async fn create_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKeyspaceRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateKeyspaceResponse>,
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
                "/vtadmin.VTAdmin/CreateKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "CreateKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// CreateShard creates a new shard in the given cluster and keyspace.
        pub async fn create_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateShardRequest>,
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
                "/vtadmin.VTAdmin/CreateShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "CreateShard"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteKeyspace deletes a keyspace in the given cluster.
        pub async fn delete_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteKeyspaceRequest>,
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
                "/vtadmin.VTAdmin/DeleteKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "DeleteKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteShard deletes one or more shards in the given cluster and keyspace.
        pub async fn delete_shards(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShardsRequest>,
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
                "/vtadmin.VTAdmin/DeleteShards",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "DeleteShards"));
            self.inner.unary(req, path, codec).await
        }
        /// DeleteTablet deletes a tablet from the topology
        pub async fn delete_tablet(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTabletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTabletResponse>,
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
                "/vtadmin.VTAdmin/DeleteTablet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "DeleteTablet"));
            self.inner.unary(req, path, codec).await
        }
        /// EmergencyFailoverShard fails over a shard to a new primary. It assumes
        /// the old primary is dead or otherwise not responding.
        pub async fn emergency_failover_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::EmergencyFailoverShardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EmergencyFailoverShardResponse>,
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
                "/vtadmin.VTAdmin/EmergencyFailoverShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "EmergencyFailoverShard"));
            self.inner.unary(req, path, codec).await
        }
        /// FindSchema returns a single Schema that matches the provided table name
        /// across all specified clusters IDs. Not specifying a set of cluster IDs
        /// causes the search to span all configured clusters.
        ///
        /// An error occurs if either no table exists across any of the clusters with
        /// the specified table name, or if multiple tables exist with that name.
        pub async fn find_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::FindSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::Schema>, tonic::Status> {
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
                "/vtadmin.VTAdmin/FindSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "FindSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// GetBackups returns backups grouped by cluster.
        pub async fn get_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBackupsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBackupsResponse>,
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
                "/vtadmin.VTAdmin/GetBackups",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetBackups"));
            self.inner.unary(req, path, codec).await
        }
        /// GetCellInfos returns the CellInfo objects for the specified clusters.
        ///
        /// Callers may optionally restrict the set of CellInfos, or restrict the
        /// response to include only cell names.
        pub async fn get_cell_infos(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCellInfosRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCellInfosResponse>,
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
                "/vtadmin.VTAdmin/GetCellInfos",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetCellInfos"));
            self.inner.unary(req, path, codec).await
        }
        /// GetCellsAliases returns the CellsAliases data for the specified clusters.
        pub async fn get_cells_aliases(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCellsAliasesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCellsAliasesResponse>,
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
                "/vtadmin.VTAdmin/GetCellsAliases",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetCellsAliases"));
            self.inner.unary(req, path, codec).await
        }
        /// GetClusters returns all configured clusters.
        pub async fn get_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClustersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetClustersResponse>,
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
                "/vtadmin.VTAdmin/GetClusters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetClusters"));
            self.inner.unary(req, path, codec).await
        }
        /// GetFullStatus returns the full status of MySQL including the replication information, semi-sync information, GTID information among others
        pub async fn get_full_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFullStatusRequest>,
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
                "/vtadmin.VTAdmin/GetFullStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetFullStatus"));
            self.inner.unary(req, path, codec).await
        }
        /// GetGates returns all gates across all the specified clusters.
        pub async fn get_gates(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetGatesResponse>,
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
            let path = http::uri::PathAndQuery::from_static("/vtadmin.VTAdmin/GetGates");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("vtadmin.VTAdmin", "GetGates"));
            self.inner.unary(req, path, codec).await
        }
        /// GetKeyspace returns a keyspace by name in the specified cluster.
        pub async fn get_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeyspaceRequest>,
        ) -> std::result::Result<tonic::Response<super::Keyspace>, tonic::Status> {
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
                "/vtadmin.VTAdmin/GetKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// GetKeyspaces returns all keyspaces across the specified clusters.
        pub async fn get_keyspaces(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeyspacesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetKeyspacesResponse>,
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
                "/vtadmin.VTAdmin/GetKeyspaces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetKeyspaces"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSchema returns the schema for the specified (cluster, keyspace, table)
        /// tuple.
        pub async fn get_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::Schema>, tonic::Status> {
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
                "/vtadmin.VTAdmin/GetSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("vtadmin.VTAdmin", "GetSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSchemas returns all schemas across the specified clusters.
        pub async fn get_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSchemasResponse>,
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
                "/vtadmin.VTAdmin/GetSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetSchemas"));
            self.inner.unary(req, path, codec).await
        }
        /// GetShardReplicationPositions returns shard replication positions grouped
        /// by cluster.
        pub async fn get_shard_replication_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShardReplicationPositionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShardReplicationPositionsResponse>,
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
                "/vtadmin.VTAdmin/GetShardReplicationPositions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtadmin.VTAdmin", "GetShardReplicationPositions"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GetSrvKeyspace returns the SrvKeyspace for a keyspace in one or more cells.
        pub async fn get_srv_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSrvKeyspaceRequest>,
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
                "/vtadmin.VTAdmin/GetSrvKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetSrvKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSrvKeyspaces returns the SrvKeyspaces for all keyspaces across all the specified clusters.
        pub async fn get_srv_keyspaces(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSrvKeyspacesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSrvKeyspacesResponse>,
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
                "/vtadmin.VTAdmin/GetSrvKeyspaces",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetSrvKeyspaces"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSrvVSchema returns the SrvVSchema for the given cluster and cell.
        pub async fn get_srv_v_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSrvVSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SrvVSchema>, tonic::Status> {
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
                "/vtadmin.VTAdmin/GetSrvVSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetSrvVSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// GetSrvVSchemas returns all SrvVSchemas across all (or specified) clusters
        /// and cells.
        pub async fn get_srv_v_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSrvVSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSrvVSchemasResponse>,
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
                "/vtadmin.VTAdmin/GetSrvVSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetSrvVSchemas"));
            self.inner.unary(req, path, codec).await
        }
        /// GetTablet looks up a tablet by hostname across all clusters and returns
        /// the result.
        pub async fn get_tablet(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTabletRequest>,
        ) -> std::result::Result<tonic::Response<super::Tablet>, tonic::Status> {
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
                "/vtadmin.VTAdmin/GetTablet",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("vtadmin.VTAdmin", "GetTablet"));
            self.inner.unary(req, path, codec).await
        }
        /// GetTablets returns all tablets across all the specified clusters.
        pub async fn get_tablets(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTabletsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTabletsResponse>,
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
                "/vtadmin.VTAdmin/GetTablets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetTablets"));
            self.inner.unary(req, path, codec).await
        }
        /// GetTopologyPath returns the cell located at the specified path in the topology server.
        pub async fn get_topology_path(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTopologyPathRequest>,
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
                "/vtadmin.VTAdmin/GetTopologyPath",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetTopologyPath"));
            self.inner.unary(req, path, codec).await
        }
        /// GetVSchema returns a VSchema for the specified keyspace in the specified
        /// cluster.
        pub async fn get_v_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::VSchema>, tonic::Status> {
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
                "/vtadmin.VTAdmin/GetVSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetVSchema"));
            self.inner.unary(req, path, codec).await
        }
        /// GetVSchemas returns the VSchemas for all specified clusters.
        pub async fn get_v_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVSchemasResponse>,
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
                "/vtadmin.VTAdmin/GetVSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetVSchemas"));
            self.inner.unary(req, path, codec).await
        }
        /// GetVtctlds returns the Vtctlds for all specified clusters.
        pub async fn get_vtctlds(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVtctldsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetVtctldsResponse>,
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
                "/vtadmin.VTAdmin/GetVtctlds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetVtctlds"));
            self.inner.unary(req, path, codec).await
        }
        /// GetWorkflow returns a single Workflow for a given cluster, keyspace, and
        /// workflow name.
        pub async fn get_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkflowRequest>,
        ) -> std::result::Result<tonic::Response<super::Workflow>, tonic::Status> {
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
                "/vtadmin.VTAdmin/GetWorkflow",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetWorkflow"));
            self.inner.unary(req, path, codec).await
        }
        /// GetWorkflows returns the Workflows for all specified clusters.
        pub async fn get_workflows(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkflowsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetWorkflowsResponse>,
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
                "/vtadmin.VTAdmin/GetWorkflows",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "GetWorkflows"));
            self.inner.unary(req, path, codec).await
        }
        /// PingTablet checks that the specified tablet is awake and responding to
        /// RPCs. This command can be blocked by other in-flight operations.
        pub async fn ping_tablet(
            &mut self,
            request: impl tonic::IntoRequest<super::PingTabletRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PingTabletResponse>,
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
                "/vtadmin.VTAdmin/PingTablet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "PingTablet"));
            self.inner.unary(req, path, codec).await
        }
        /// PlannedFailoverShard fails over the shard to a new primary, or away from
        /// an old primary. Both the old and new primaries must be reachable and
        /// running.
        ///
        /// NOTE: A planned failover will not consider replicas outside the current
        /// shard primary's cell as promotion candidates unless NewPrimary is
        /// explicitly provided in the request.
        pub async fn planned_failover_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::PlannedFailoverShardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PlannedFailoverShardResponse>,
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
                "/vtadmin.VTAdmin/PlannedFailoverShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "PlannedFailoverShard"));
            self.inner.unary(req, path, codec).await
        }
        /// RebuildKeyspaceGraph rebuilds the serving data for a keyspace.
        pub async fn rebuild_keyspace_graph(
            &mut self,
            request: impl tonic::IntoRequest<super::RebuildKeyspaceGraphRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RebuildKeyspaceGraphResponse>,
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
                "/vtadmin.VTAdmin/RebuildKeyspaceGraph",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "RebuildKeyspaceGraph"));
            self.inner.unary(req, path, codec).await
        }
        /// RefreshState reloads the tablet record on the specified tablet.
        pub async fn refresh_state(
            &mut self,
            request: impl tonic::IntoRequest<super::RefreshStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RefreshStateResponse>,
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
                "/vtadmin.VTAdmin/RefreshState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "RefreshState"));
            self.inner.unary(req, path, codec).await
        }
        /// RefreshTabletReplicationSource performs a `CHANGE REPLICATION SOURCE TO`
        /// on a tablet to replicate from the current primary in the shard.
        pub async fn refresh_tablet_replication_source(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RefreshTabletReplicationSourceRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RefreshTabletReplicationSourceResponse>,
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
                "/vtadmin.VTAdmin/RefreshTabletReplicationSource",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("vtadmin.VTAdmin", "RefreshTabletReplicationSource"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// ReloadSchemas reloads the schema definition across keyspaces, shards, or
        /// tablets in one or more clusters, depending on the request fields (see
        /// ReloadSchemasRequest for details).
        pub async fn reload_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ReloadSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReloadSchemasResponse>,
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
                "/vtadmin.VTAdmin/ReloadSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "ReloadSchemas"));
            self.inner.unary(req, path, codec).await
        }
        /// ReloadSchemaShard reloads the schema on all tablets in a shard. This is done on a best-effort basis.
        pub async fn reload_schema_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::ReloadSchemaShardRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReloadSchemaShardResponse>,
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
                "/vtadmin.VTAdmin/ReloadSchemaShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "ReloadSchemaShard"));
            self.inner.unary(req, path, codec).await
        }
        /// RemoveKeyspaceCell removes the cell from the Cells list for all shards in the keyspace, and the SrvKeyspace for that keyspace in that cell.
        pub async fn remove_keyspace_cell(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveKeyspaceCellRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveKeyspaceCellResponse>,
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
                "/vtadmin.VTAdmin/RemoveKeyspaceCell",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "RemoveKeyspaceCell"));
            self.inner.unary(req, path, codec).await
        }
        /// RunHealthCheck runs a healthcheck on the tablet.
        pub async fn run_health_check(
            &mut self,
            request: impl tonic::IntoRequest<super::RunHealthCheckRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RunHealthCheckResponse>,
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
                "/vtadmin.VTAdmin/RunHealthCheck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "RunHealthCheck"));
            self.inner.unary(req, path, codec).await
        }
        /// SetReadOnly sets the tablet to read-only mode.
        pub async fn set_read_only(
            &mut self,
            request: impl tonic::IntoRequest<super::SetReadOnlyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetReadOnlyResponse>,
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
                "/vtadmin.VTAdmin/SetReadOnly",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "SetReadOnly"));
            self.inner.unary(req, path, codec).await
        }
        /// SetReadWrite sets the tablet to read-write mode.
        pub async fn set_read_write(
            &mut self,
            request: impl tonic::IntoRequest<super::SetReadWriteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SetReadWriteResponse>,
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
                "/vtadmin.VTAdmin/SetReadWrite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "SetReadWrite"));
            self.inner.unary(req, path, codec).await
        }
        /// StartReplication runs the underlying database command to start
        /// replication on a tablet.
        pub async fn start_replication(
            &mut self,
            request: impl tonic::IntoRequest<super::StartReplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartReplicationResponse>,
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
                "/vtadmin.VTAdmin/StartReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "StartReplication"));
            self.inner.unary(req, path, codec).await
        }
        /// StopReplication runs the underlying database command to stop replication
        /// on a tablet
        pub async fn stop_replication(
            &mut self,
            request: impl tonic::IntoRequest<super::StopReplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopReplicationResponse>,
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
                "/vtadmin.VTAdmin/StopReplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "StopReplication"));
            self.inner.unary(req, path, codec).await
        }
        /// TabletExternallyPromoted updates the metadata in a cluster's topology
        /// to acknowledge a shard primary change performed by an external tool
        /// (e.g. orchestrator*).
        ///
        /// See the Reparenting guide for more information:
        /// https://vitess.io/docs/user-guides/configuration-advanced/reparenting/#external-reparenting.
        ///
        /// * "orchestrator" here refers to external orchestrator, not the newer,
        /// Vitess-aware orchestrator, VTOrc.
        pub async fn tablet_externally_promoted(
            &mut self,
            request: impl tonic::IntoRequest<super::TabletExternallyPromotedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TabletExternallyPromotedResponse>,
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
                "/vtadmin.VTAdmin/TabletExternallyPromoted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "TabletExternallyPromoted"));
            self.inner.unary(req, path, codec).await
        }
        /// Validate validates all nodes in a cluster that are reachable from the global replication graph,
        /// as well as all tablets in discoverable cells, are consistent
        pub async fn validate(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateRequest>,
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
            let path = http::uri::PathAndQuery::from_static("/vtadmin.VTAdmin/Validate");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("vtadmin.VTAdmin", "Validate"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidateKeyspace validates that all nodes reachable from the specified
        /// keyspace are consistent.
        pub async fn validate_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateKeyspaceRequest>,
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
                "/vtadmin.VTAdmin/ValidateKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "ValidateKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidateSchemaKeyspace validates that the schema on the primary tablet
        /// for shard 0 matches the schema on all of the other tablets in the
        /// keyspace.
        pub async fn validate_schema_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateSchemaKeyspaceRequest>,
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
                "/vtadmin.VTAdmin/ValidateSchemaKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "ValidateSchemaKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidateShard validates that that all nodes reachable from the specified shard are consistent.
        pub async fn validate_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateShardRequest>,
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
                "/vtadmin.VTAdmin/ValidateShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "ValidateShard"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidateVersionKeyspace validates that the version on the primary of
        /// shard 0 matches all of the other tablets in the keyspace.
        pub async fn validate_version_keyspace(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateVersionKeyspaceRequest>,
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
                "/vtadmin.VTAdmin/ValidateVersionKeyspace",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "ValidateVersionKeyspace"));
            self.inner.unary(req, path, codec).await
        }
        /// ValidateVersionShard validates that the version on the primary matches all of the replicas.
        pub async fn validate_version_shard(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateVersionShardRequest>,
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
                "/vtadmin.VTAdmin/ValidateVersionShard",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("vtadmin.VTAdmin", "ValidateVersionShard"));
            self.inner.unary(req, path, codec).await
        }
        /// VTExplain provides information on how Vitess plans to execute a
        /// particular query.
        pub async fn vt_explain(
            &mut self,
            request: impl tonic::IntoRequest<super::VtExplainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::VtExplainResponse>,
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
                "/vtadmin.VTAdmin/VTExplain",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("vtadmin.VTAdmin", "VTExplain"));
            self.inner.unary(req, path, codec).await
        }
    }
}
