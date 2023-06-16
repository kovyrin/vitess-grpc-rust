/// ExecuteVtctlCommandRequest is the payload for ExecuteVtctlCommand.
/// timeouts are in nanoseconds.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteVtctlCommandRequest {
    #[prost(string, repeated, tag = "1")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "2")]
    pub action_timeout: i64,
}
/// ExecuteVtctlCommandResponse is streamed back by ExecuteVtctlCommand.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteVtctlCommandResponse {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<super::logutil::Event>,
}
/// TableMaterializeSttings contains the settings for one table.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableMaterializeSettings {
    #[prost(string, tag = "1")]
    pub target_table: ::prost::alloc::string::String,
    /// source_expression is a select statement.
    #[prost(string, tag = "2")]
    pub source_expression: ::prost::alloc::string::String,
    /// create_ddl contains the DDL to create the target table.
    /// If empty, the target table must already exist.
    /// if "copy", the target table DDL is the same as the source table.
    #[prost(string, tag = "3")]
    pub create_ddl: ::prost::alloc::string::String,
}
/// MaterializeSettings contains the settings for the Materialize command.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaterializeSettings {
    /// workflow is the name of the workflow.
    #[prost(string, tag = "1")]
    pub workflow: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source_keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub target_keyspace: ::prost::alloc::string::String,
    /// stop_after_copy specifies if vreplication should be stopped after copying.
    #[prost(bool, tag = "4")]
    pub stop_after_copy: bool,
    #[prost(message, repeated, tag = "5")]
    pub table_settings: ::prost::alloc::vec::Vec<TableMaterializeSettings>,
    /// optional parameters.
    #[prost(string, tag = "6")]
    pub cell: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub tablet_types: ::prost::alloc::string::String,
    /// ExternalCluster is the name of the mounted cluster which has the source keyspace/db for this workflow
    /// it is of the type <cluster_type.cluster_name>
    #[prost(string, tag = "8")]
    pub external_cluster: ::prost::alloc::string::String,
    /// MaterializationIntent is used to identify the reason behind the materialization workflow: eg. MoveTables, CreateLookupVindex
    #[prost(enumeration = "MaterializationIntent", tag = "9")]
    pub materialization_intent: i32,
    /// SourceTimeZone is the time zone in which datetimes on the source were stored, provided as an option in MoveTable
    #[prost(string, tag = "10")]
    pub source_time_zone: ::prost::alloc::string::String,
    /// TargetTimeZone is not currently specifiable by the user, defaults to UTC for the forward workflows
    /// and to the SourceTimeZone in reverse workflows
    #[prost(string, tag = "11")]
    pub target_time_zone: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "12")]
    pub source_shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keyspace {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub keyspace: ::core::option::Option<super::topodata::Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shard {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub shard: ::core::option::Option<super::topodata::Shard>,
}
/// TODO: comment the hell out of this.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workflow {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub source: ::core::option::Option<workflow::ReplicationLocation>,
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<workflow::ReplicationLocation>,
    #[prost(int64, tag = "4")]
    pub max_v_replication_lag: i64,
    #[prost(map = "string, message", tag = "5")]
    pub shard_streams: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        workflow::ShardStream,
    >,
}
/// Nested message and enum types in `Workflow`.
pub mod workflow {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReplicationLocation {
        #[prost(string, tag = "1")]
        pub keyspace: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "2")]
        pub shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShardStream {
        #[prost(message, repeated, tag = "1")]
        pub streams: ::prost::alloc::vec::Vec<Stream>,
        #[prost(message, repeated, tag = "2")]
        pub tablet_controls: ::prost::alloc::vec::Vec<
            super::super::topodata::shard::TabletControl,
        >,
        #[prost(bool, tag = "3")]
        pub is_primary_serving: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Stream {
        #[prost(int64, tag = "1")]
        pub id: i64,
        #[prost(string, tag = "2")]
        pub shard: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "3")]
        pub tablet: ::core::option::Option<super::super::topodata::TabletAlias>,
        #[prost(message, optional, tag = "4")]
        pub binlog_source: ::core::option::Option<
            super::super::binlogdata::BinlogSource,
        >,
        #[prost(string, tag = "5")]
        pub position: ::prost::alloc::string::String,
        #[prost(string, tag = "6")]
        pub stop_position: ::prost::alloc::string::String,
        #[prost(string, tag = "7")]
        pub state: ::prost::alloc::string::String,
        #[prost(string, tag = "8")]
        pub db_name: ::prost::alloc::string::String,
        #[prost(message, optional, tag = "9")]
        pub transaction_timestamp: ::core::option::Option<super::super::vttime::Time>,
        #[prost(message, optional, tag = "10")]
        pub time_updated: ::core::option::Option<super::super::vttime::Time>,
        #[prost(string, tag = "11")]
        pub message: ::prost::alloc::string::String,
        #[prost(message, repeated, tag = "12")]
        pub copy_states: ::prost::alloc::vec::Vec<stream::CopyState>,
        #[prost(message, repeated, tag = "13")]
        pub logs: ::prost::alloc::vec::Vec<stream::Log>,
        /// LogFetchError is set if we fail to fetch some logs for this stream. We
        /// will never fail to fetch workflows because we cannot fetch the logs, but
        /// we will still forward log-fetch errors to the caller, should that be
        /// relevant to the context in which they are fetching workflows.
        ///
        /// Note that this field being set does not necessarily mean that Logs is nil;
        /// if there are N logs that exist for the stream, and we fail to fetch the
        /// ith log, we will still return logs in [0, i) + (i, N].
        #[prost(string, tag = "14")]
        pub log_fetch_error: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "15")]
        pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Nested message and enum types in `Stream`.
    pub mod stream {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CopyState {
            #[prost(string, tag = "1")]
            pub table: ::prost::alloc::string::String,
            #[prost(string, tag = "2")]
            pub last_pk: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Log {
            #[prost(int64, tag = "1")]
            pub id: i64,
            #[prost(int64, tag = "2")]
            pub stream_id: i64,
            #[prost(string, tag = "3")]
            pub r#type: ::prost::alloc::string::String,
            #[prost(string, tag = "4")]
            pub state: ::prost::alloc::string::String,
            #[prost(message, optional, tag = "5")]
            pub created_at: ::core::option::Option<super::super::super::vttime::Time>,
            #[prost(message, optional, tag = "6")]
            pub updated_at: ::core::option::Option<super::super::super::vttime::Time>,
            #[prost(string, tag = "7")]
            pub message: ::prost::alloc::string::String,
            #[prost(int64, tag = "8")]
            pub count: i64,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCellInfoRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cell_info: ::core::option::Option<super::topodata::CellInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCellInfoResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCellsAliasRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddCellsAliasResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRoutingRulesRequest {
    #[prost(message, optional, tag = "1")]
    pub routing_rules: ::core::option::Option<super::vschema::RoutingRules>,
    /// SkipRebuild, if set, will cause ApplyRoutingRules to skip rebuilding the
    /// SrvVSchema objects in each cell in RebuildCells.
    #[prost(bool, tag = "2")]
    pub skip_rebuild: bool,
    /// RebuildCells limits the SrvVSchema rebuild to the specified cells. If not
    /// provided the SrvVSchema will be rebuilt in every cell in the topology.
    ///
    /// Ignored if SkipRebuild is set.
    #[prost(string, repeated, tag = "3")]
    pub rebuild_cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRoutingRulesResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyShardRoutingRulesRequest {
    #[prost(message, optional, tag = "1")]
    pub shard_routing_rules: ::core::option::Option<super::vschema::ShardRoutingRules>,
    /// SkipRebuild, if set, will cause ApplyShardRoutingRules to skip rebuilding the
    /// SrvVSchema objects in each cell in RebuildCells.
    #[prost(bool, tag = "2")]
    pub skip_rebuild: bool,
    /// RebuildCells limits the SrvVSchema rebuild to the specified cells. If not
    /// provided the SrvVSchema will be rebuilt in every cell in the topology.
    ///
    /// Ignored if SkipRebuild is set.
    #[prost(string, repeated, tag = "3")]
    pub rebuild_cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyShardRoutingRulesResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplySchemaRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// Allow large schema changes which incur a longer unavailability of the database.
    #[prost(bool, tag = "2")]
    pub allow_long_unavailability: bool,
    /// SQL commands to run.
    #[prost(string, repeated, tag = "3")]
    pub sql: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Online DDL strategy, compatible with @@ddl_strategy session variable (examples: 'gh-ost', 'pt-osc', 'gh-ost --max-load=Threads_running=100'")
    #[prost(string, tag = "4")]
    pub ddl_strategy: ::prost::alloc::string::String,
    /// Optional: explicit UUIDs for migration.
    /// If given, must match number of DDL changes
    #[prost(string, repeated, tag = "5")]
    pub uuid_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// For Online DDL, optionally supply a custom unique string used as context for the migration(s) in this command.
    /// By default a unique context is auto-generated by Vitess
    #[prost(string, tag = "6")]
    pub migration_context: ::prost::alloc::string::String,
    /// WaitReplicasTimeout is the duration of time to wait for replicas to catch
    /// up in reparenting.
    #[prost(message, optional, tag = "7")]
    pub wait_replicas_timeout: ::core::option::Option<super::vttime::Duration>,
    /// Skip pre-apply schema checks, and directly forward schema change query to shards
    #[prost(bool, tag = "8")]
    pub skip_preflight: bool,
    /// caller_id identifies the caller. This is the effective caller ID,
    /// set by the application to further identify the caller.
    #[prost(message, optional, tag = "9")]
    pub caller_id: ::core::option::Option<super::vtrpc::CallerId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplySchemaResponse {
    #[prost(string, repeated, tag = "1")]
    pub uuid_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyVSchemaRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub skip_rebuild: bool,
    #[prost(bool, tag = "3")]
    pub dry_run: bool,
    #[prost(string, repeated, tag = "4")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "5")]
    pub v_schema: ::core::option::Option<super::vschema::Keyspace>,
    #[prost(string, tag = "6")]
    pub sql: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyVSchemaResponse {
    #[prost(message, optional, tag = "1")]
    pub v_schema: ::core::option::Option<super::vschema::Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    /// AllowPrimary allows the backup to proceed if TabletAlias is a PRIMARY.
    ///
    /// WARNING: If using the builtin backup engine, this will shutdown mysqld on
    /// the primary for the duration of the backup, and no writes will be possible.
    #[prost(bool, tag = "2")]
    pub allow_primary: bool,
    /// Concurrency specifies the number of compression/checksum jobs to run
    /// simultaneously.
    #[prost(uint64, tag = "3")]
    pub concurrency: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupResponse {
    /// TabletAlias is the alias being used for the backup.
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub event: ::core::option::Option<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupShardRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    /// AllowPrimary allows the backup to occur on a PRIMARY tablet. See
    /// BackupRequest.AllowPrimary for warnings and caveats.
    #[prost(bool, tag = "3")]
    pub allow_primary: bool,
    /// Concurrency specifies the number of compression/checksum jobs to run
    /// simultaneously.
    #[prost(uint64, tag = "4")]
    pub concurrency: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeTabletTypeRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(enumeration = "super::topodata::TabletType", tag = "2")]
    pub db_type: i32,
    #[prost(bool, tag = "3")]
    pub dry_run: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeTabletTypeResponse {
    #[prost(message, optional, tag = "1")]
    pub before_tablet: ::core::option::Option<super::topodata::Tablet>,
    #[prost(message, optional, tag = "2")]
    pub after_tablet: ::core::option::Option<super::topodata::Tablet>,
    #[prost(bool, tag = "3")]
    pub was_dry_run: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyspaceRequest {
    /// Name is the name of the keyspace.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Force proceeds with the request even if the keyspace already exists.
    #[prost(bool, tag = "2")]
    pub force: bool,
    /// AllowEmptyVSchema allows a keyspace to be created with no vschema.
    #[prost(bool, tag = "3")]
    pub allow_empty_v_schema: bool,
    /// ServedFroms specifies a set of db_type:keyspace pairs used to serve
    /// traffic for the keyspace.
    #[prost(message, repeated, tag = "6")]
    pub served_froms: ::prost::alloc::vec::Vec<super::topodata::keyspace::ServedFrom>,
    /// Type is the type of the keyspace to create.
    #[prost(enumeration = "super::topodata::KeyspaceType", tag = "7")]
    pub r#type: i32,
    /// BaseKeyspace specifies the base keyspace for SNAPSHOT keyspaces. It is
    /// required to create a SNAPSHOT keyspace.
    #[prost(string, tag = "8")]
    pub base_keyspace: ::prost::alloc::string::String,
    /// SnapshotTime specifies the snapshot time for this keyspace. It is required
    /// to create a SNAPSHOT keyspace.
    #[prost(message, optional, tag = "9")]
    pub snapshot_time: ::core::option::Option<super::vttime::Time>,
    /// DurabilityPolicy is the durability policy to be
    /// used for this keyspace.
    #[prost(string, tag = "10")]
    pub durability_policy: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyspaceResponse {
    /// Keyspace is the newly-created keyspace.
    #[prost(message, optional, tag = "1")]
    pub keyspace: ::core::option::Option<Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShardRequest {
    /// Keyspace is the name of the keyspace to create the shard in.
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// ShardName is the name of the shard to create. E.g. "-" or "-80".
    #[prost(string, tag = "2")]
    pub shard_name: ::prost::alloc::string::String,
    /// Force treats an attempt to create a shard that already exists as a
    /// non-error.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// IncludeParent creates the parent keyspace as an empty BASE keyspace, if it
    /// doesn't already exist.
    #[prost(bool, tag = "4")]
    pub include_parent: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateShardResponse {
    /// Keyspace is the created keyspace. It is set only if IncludeParent was
    /// specified in the request and the parent keyspace needed to be created.
    #[prost(message, optional, tag = "1")]
    pub keyspace: ::core::option::Option<Keyspace>,
    /// Shard is the newly-created shard object.
    #[prost(message, optional, tag = "2")]
    pub shard: ::core::option::Option<Shard>,
    /// ShardAlreadyExists is set if Force was specified in the request and the
    /// shard already existed.
    #[prost(bool, tag = "3")]
    pub shard_already_exists: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCellInfoRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub force: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCellInfoResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCellsAliasRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCellsAliasResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeyspaceRequest {
    /// Keyspace is the name of the keyspace to delete.
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// Recursive causes all shards in the keyspace to be recursively deleted
    /// before deleting the keyspace. It is an error to call DeleteKeyspace on a
    /// non-empty keyspace without also specifying Recursive.
    #[prost(bool, tag = "2")]
    pub recursive: bool,
    /// Force allows a keyspace to be deleted even if the keyspace lock cannot be
    /// obtained. This should only be used to force-clean a keyspace.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteKeyspaceResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShardsRequest {
    /// Shards is the list of shards to delete. The nested topodatapb.Shard field
    /// is not required for DeleteShard, but the Keyspace and Shard fields are.
    #[prost(message, repeated, tag = "1")]
    pub shards: ::prost::alloc::vec::Vec<Shard>,
    /// Recursive also deletes all tablets belonging to the shard(s). It is an
    /// error to call DeleteShard on a non-empty shard without also specificying
    /// Recursive.
    #[prost(bool, tag = "2")]
    pub recursive: bool,
    /// EvenIfServing allows a shard to be deleted even if it is serving, which is
    /// normally an error. Use with caution.
    #[prost(bool, tag = "4")]
    pub even_if_serving: bool,
    /// Force allows a shard to be deleted even if the shard lock cannot be
    /// obtained. This should only be used to force-clean a shard.
    #[prost(bool, tag = "5")]
    pub force: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShardsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSrvVSchemaRequest {
    #[prost(string, tag = "1")]
    pub cell: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSrvVSchemaResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTabletsRequest {
    /// TabletAliases is the list of tablets to delete.
    #[prost(message, repeated, tag = "1")]
    pub tablet_aliases: ::prost::alloc::vec::Vec<super::topodata::TabletAlias>,
    /// AllowPrimary allows for the primary tablet of a shard to be deleted.
    /// Use with caution.
    #[prost(bool, tag = "2")]
    pub allow_primary: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTabletsResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmergencyReparentShardRequest {
    /// Keyspace is the name of the keyspace to perform the Emergency Reparent in.
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// Shard is the name of the shard to perform the Emergency Reparent in.
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    /// Optional alias of a tablet that should become the new shard primary. If not
    /// not specified, the vtctld will select the most up-to-date canditate to
    /// promote.
    #[prost(message, optional, tag = "3")]
    pub new_primary: ::core::option::Option<super::topodata::TabletAlias>,
    /// List of replica aliases to ignore during the Emergency Reparent. The vtctld
    /// will not attempt to stop replication on these tablets, nor attempt to
    /// demote any that may think they are the shard primary.
    #[prost(message, repeated, tag = "4")]
    pub ignore_replicas: ::prost::alloc::vec::Vec<super::topodata::TabletAlias>,
    /// WaitReplicasTimeout is the duration of time to wait for replicas to catch
    /// up in reparenting.
    #[prost(message, optional, tag = "5")]
    pub wait_replicas_timeout: ::core::option::Option<super::vttime::Duration>,
    /// PreventCrossCellPromotion is used to only promote the new primary from the same cell
    /// as the failed primary.
    #[prost(bool, tag = "6")]
    pub prevent_cross_cell_promotion: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmergencyReparentShardResponse {
    /// Keyspace is the name of the keyspace the Emergency Reparent took place in.
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// Shard is the name of the shard the Emergency Reparent took place in.
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    /// PromotedPrimary is the alias of the tablet that was promoted to shard
    /// primary. If NewPrimary was set in the request, then this will be the same
    /// alias. Otherwise, it will be the alias of the tablet found to be most
    /// up-to-date.
    #[prost(message, optional, tag = "3")]
    pub promoted_primary: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(message, repeated, tag = "4")]
    pub events: ::prost::alloc::vec::Vec<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFetchAsAppRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// MaxRows is an optional parameter to limit the number of rows read into the
    /// QueryResult. Note that this does not apply a LIMIT to the query, just how
    /// many rows are read from the MySQL server on the tablet side.
    ///
    /// This field is optional. Specifying a non-positive value will use whatever
    /// default is configured in the VtctldService.
    #[prost(int64, tag = "3")]
    pub max_rows: i64,
    /// UsePool causes the query to be run with a pooled connection to the tablet.
    #[prost(bool, tag = "4")]
    pub use_pool: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFetchAsAppResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<super::query::QueryResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteFetchAsDbaRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// MaxRows is an optional parameter to limit the number of rows read into the
    /// QueryResult. Note that this does not apply a LIMIT to the query, just how
    /// many rows are read from the MySQL server on the tablet side.
    ///
    /// This field is optional. Specifying a non-positive value will use whatever
    /// default is configured in the VtctldService.
    #[prost(int64, tag = "3")]
    pub max_rows: i64,
    /// DisableBinlogs instructs the tablet not to use binary logging when
    /// executing the query.
    #[prost(bool, tag = "4")]
    pub disable_binlogs: bool,
    /// ReloadSchema instructs the tablet to reload its schema after executing the
    /// query.
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
pub struct ExecuteHookRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(message, optional, tag = "2")]
    pub tablet_hook_request: ::core::option::Option<
        super::tabletmanagerdata::ExecuteHookRequest,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteHookResponse {
    #[prost(message, optional, tag = "1")]
    pub hook_result: ::core::option::Option<
        super::tabletmanagerdata::ExecuteHookResponse,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindAllShardsInKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindAllShardsInKeyspaceResponse {
    #[prost(map = "string, message", tag = "1")]
    pub shards: ::std::collections::HashMap<::prost::alloc::string::String, Shard>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupsRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    /// Limit, if nonzero, will return only the most N recent backups.
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    /// Detailed indicates whether to use the backupengine, if supported, to
    /// populate additional fields, such as Engine and Status, on BackupInfo
    /// objects in the response. If not set, or if the backupengine does not
    /// support populating these fields, Engine will always be empty, and Status
    /// will always be UNKNOWN.
    #[prost(bool, tag = "4")]
    pub detailed: bool,
    /// DetailedLimit, if nonzero, will only populate additional fields (see Detailed)
    /// on the N most recent backups. The Limit field still dictates the total
    /// number of backup info objects returned, so, in reality, min(Limit, DetailedLimit)
    /// backup infos will have additional fields set, and any remaining backups
    /// will not.
    #[prost(uint32, tag = "5")]
    pub detailed_limit: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupsResponse {
    #[prost(message, repeated, tag = "1")]
    pub backups: ::prost::alloc::vec::Vec<super::mysqlctl::BackupInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCellInfoRequest {
    #[prost(string, tag = "1")]
    pub cell: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCellInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub cell_info: ::core::option::Option<super::topodata::CellInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCellInfoNamesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCellInfoNamesResponse {
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCellsAliasesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCellsAliasesResponse {
    #[prost(map = "string, message", tag = "1")]
    pub aliases: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::topodata::CellsAlias,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFullStatusRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFullStatusResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::replicationdata::FullStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyspacesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyspacesResponse {
    #[prost(message, repeated, tag = "1")]
    pub keyspaces: ::prost::alloc::vec::Vec<Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyspaceResponse {
    #[prost(message, optional, tag = "1")]
    pub keyspace: ::core::option::Option<Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermissionsRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermissionsResponse {
    #[prost(message, optional, tag = "1")]
    pub permissions: ::core::option::Option<super::tabletmanagerdata::Permissions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoutingRulesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoutingRulesResponse {
    #[prost(message, optional, tag = "1")]
    pub routing_rules: ::core::option::Option<super::vschema::RoutingRules>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    /// Tables is a list of tables for which we should gather information. Each is
    /// either an exact match, or a regular expression of the form /regexp/.
    #[prost(string, repeated, tag = "2")]
    pub tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ExcludeTables is a list of tables to exclude from the result. Each is
    /// either an exact match, or a regular expression of the form /regexp/.
    #[prost(string, repeated, tag = "3")]
    pub exclude_tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// IncludeViews specifies whether to include views in the result.
    #[prost(bool, tag = "4")]
    pub include_views: bool,
    /// TableNamesOnly specifies whether to limit the results to just table names,
    /// rather than full schema information for each table.
    #[prost(bool, tag = "5")]
    pub table_names_only: bool,
    /// TableSizesOnly specifies whether to limit the results to just table sizes,
    /// rather than full schema information for each table. It is ignored if
    /// TableNamesOnly is set to true.
    #[prost(bool, tag = "6")]
    pub table_sizes_only: bool,
    /// TableSchemaOnly specifies whether to limit the results to just table/view
    /// schema definition (CREATE TABLE/VIEW statements) and skip column/field information
    #[prost(bool, tag = "7")]
    pub table_schema_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSchemaResponse {
    #[prost(message, optional, tag = "1")]
    pub schema: ::core::option::Option<super::tabletmanagerdata::SchemaDefinition>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShardRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShardResponse {
    #[prost(message, optional, tag = "1")]
    pub shard: ::core::option::Option<Shard>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShardRoutingRulesRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShardRoutingRulesResponse {
    #[prost(message, optional, tag = "1")]
    pub shard_routing_rules: ::core::option::Option<super::vschema::ShardRoutingRules>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvKeyspaceNamesRequest {
    #[prost(string, repeated, tag = "1")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvKeyspaceNamesResponse {
    /// Names is a mapping of cell name to a list of SrvKeyspace names.
    #[prost(map = "string, message", tag = "1")]
    pub names: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        get_srv_keyspace_names_response::NameList,
    >,
}
/// Nested message and enum types in `GetSrvKeyspaceNamesResponse`.
pub mod get_srv_keyspace_names_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NameList {
        #[prost(string, repeated, tag = "1")]
        pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvKeyspacesRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// Cells is a list of cells to lookup a SrvKeyspace for. Leaving this empty is
    /// equivalent to specifying all cells in the topo.
    #[prost(string, repeated, tag = "2")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvKeyspacesResponse {
    /// SrvKeyspaces is a mapping of cell name to SrvKeyspace.
    #[prost(map = "string, message", tag = "1")]
    pub srv_keyspaces: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::topodata::SrvKeyspace,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvVSchemaRequest {
    #[prost(string, tag = "1")]
    pub cell: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvVSchemaResponse {
    #[prost(message, optional, tag = "1")]
    pub srv_v_schema: ::core::option::Option<super::vschema::SrvVSchema>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvVSchemasRequest {
    #[prost(string, repeated, tag = "2")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSrvVSchemasResponse {
    /// SrvVSchemas is a mapping of cell name to SrvVSchema
    #[prost(map = "string, message", tag = "1")]
    pub srv_v_schemas: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::vschema::SrvVSchema,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTabletRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTabletResponse {
    #[prost(message, optional, tag = "1")]
    pub tablet: ::core::option::Option<super::topodata::Tablet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTabletsRequest {
    /// Keyspace is the name of the keyspace to return tablets for. Omit to return
    /// tablets from all keyspaces.
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// Shard is the name of the shard to return tablets for. This field is ignored
    /// if Keyspace is not set.
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    /// Cells is an optional set of cells to return tablets for.
    #[prost(string, repeated, tag = "3")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Strict specifies how the server should treat failures from individual
    /// cells.
    ///
    /// When false (the default), GetTablets will return data from any cells that
    /// return successfully, but will fail the request if all cells fail. When
    /// true, any individual cell can fail the full request.
    #[prost(bool, tag = "4")]
    pub strict: bool,
    /// TabletAliases is an optional list of tablet aliases to fetch Tablet objects
    /// for. If specified, Keyspace, Shard, and Cells are ignored, and tablets are
    /// looked up by their respective aliases' Cells directly.
    #[prost(message, repeated, tag = "5")]
    pub tablet_aliases: ::prost::alloc::vec::Vec<super::topodata::TabletAlias>,
    /// tablet_type specifies the type of tablets to return. Omit to return all
    /// tablet types.
    #[prost(enumeration = "super::topodata::TabletType", tag = "6")]
    pub tablet_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTabletsResponse {
    #[prost(message, repeated, tag = "1")]
    pub tablets: ::prost::alloc::vec::Vec<super::topodata::Tablet>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopologyPathRequest {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopologyPathResponse {
    #[prost(message, optional, tag = "1")]
    pub cell: ::core::option::Option<TopologyCell>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopologyCell {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// Data is the file contents of the cell located at path.
    /// It is only populated if the cell is a terminal node.
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub children: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVSchemaRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionResponse {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVSchemaResponse {
    #[prost(message, optional, tag = "1")]
    pub v_schema: ::core::option::Option<super::vschema::Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkflowsRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub active_only: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkflowsResponse {
    #[prost(message, repeated, tag = "1")]
    pub workflows: ::prost::alloc::vec::Vec<Workflow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitShardPrimaryRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub primary_elect_tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(bool, tag = "4")]
    pub force: bool,
    #[prost(message, optional, tag = "5")]
    pub wait_replicas_timeout: ::core::option::Option<super::vttime::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitShardPrimaryResponse {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingTabletRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingTabletResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannedReparentShardRequest {
    /// Keyspace is the name of the keyspace to perform the Planned Reparent in.
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// Shard is the name of the shard to perform teh Planned Reparent in.
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    /// NewPrimary is the alias of the tablet to promote to shard primary. If not
    /// specified, the vtctld will select the most up-to-date candidate to promote.
    ///
    /// It is an error to set NewPrimary and AvoidPrimary to the same alias.
    #[prost(message, optional, tag = "3")]
    pub new_primary: ::core::option::Option<super::topodata::TabletAlias>,
    /// AvoidPrimary is the alias of the tablet to demote. In other words,
    /// specifying an AvoidPrimary alias tells the vtctld to promote any replica
    /// other than this one. A shard whose current primary is not this one is then
    /// a no-op.
    ///
    /// It is an error to set NewPrimary and AvoidPrimary to the same alias.
    #[prost(message, optional, tag = "4")]
    pub avoid_primary: ::core::option::Option<super::topodata::TabletAlias>,
    /// WaitReplicasTimeout is the duration of time to wait for replicas to catch
    /// up in replication both before and after the reparent. The timeout is not
    /// cumulative across both wait periods, meaning that the replicas have
    /// WaitReplicasTimeout time to catch up before the reparent, and an additional
    /// WaitReplicasTimeout time to catch up after the reparent.
    #[prost(message, optional, tag = "5")]
    pub wait_replicas_timeout: ::core::option::Option<super::vttime::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannedReparentShardResponse {
    /// Keyspace is the name of the keyspace the Planned Reparent took place in.
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// Shard is the name of the shard the Planned Reparent took place in.
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    /// PromotedPrimary is the alias of the tablet that was promoted to shard
    /// primary. If NewPrimary was set in the request, then this will be the same
    /// alias. Otherwise, it will be the alias of the tablet found to be most
    /// up-to-date.
    #[prost(message, optional, tag = "3")]
    pub promoted_primary: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(message, repeated, tag = "4")]
    pub events: ::prost::alloc::vec::Vec<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebuildKeyspaceGraphRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// AllowPartial, when set, allows a SNAPSHOT keyspace to serve with an
    /// incomplete set of shards. It is ignored for all other keyspace types.
    #[prost(bool, tag = "3")]
    pub allow_partial: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebuildKeyspaceGraphResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebuildVSchemaGraphRequest {
    /// Cells specifies the cells to rebuild the SrvVSchema objects for. If empty,
    /// RebuildVSchemaGraph rebuilds the SrvVSchema for every cell in the topo.
    #[prost(string, repeated, tag = "1")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RebuildVSchemaGraphResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshStateRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshStateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshStateByShardRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RefreshStateByShardResponse {
    #[prost(bool, tag = "1")]
    pub is_partial_refresh: bool,
    /// This explains why we had a partial refresh (if we did)
    #[prost(string, tag = "2")]
    pub partial_refresh_details: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemaRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemaResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemaKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub wait_position: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub include_primary: bool,
    /// Concurrency is the global concurrency across all shards in the keyspace
    /// (so, at most this many tablets will be reloaded across the keyspace at any
    /// given point).
    #[prost(uint32, tag = "4")]
    pub concurrency: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemaKeyspaceResponse {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemaShardRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub wait_position: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub include_primary: bool,
    /// Concurrency is the maximum number of tablets to reload at one time.
    #[prost(uint32, tag = "5")]
    pub concurrency: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadSchemaShardResponse {
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBackupRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBackupResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveKeyspaceCellRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub cell: ::prost::alloc::string::String,
    /// Force proceeds even if the cell's topology server cannot be reached. This
    /// should only be set if a cell has been shut down entirely, and the global
    /// topology data just needs to be updated.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// Recursive also deletes all tablets in that cell belonging to the specified
    /// keyspace.
    #[prost(bool, tag = "4")]
    pub recursive: bool,
}
/// (TODO:@amason) Consider including the deleted SrvKeyspace object and any
/// deleted Tablet objects here.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveKeyspaceCellResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveShardCellRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cell: ::prost::alloc::string::String,
    /// Force proceeds even if the cell's topology server cannot be reached. This
    /// should only be set if a cell has been shut down entirely, and the global
    /// topology data just needs to be updated.
    #[prost(bool, tag = "4")]
    pub force: bool,
    /// Recursive also deletes all tablets in that cell belonging to the specified
    /// keyspace and shard.
    #[prost(bool, tag = "5")]
    pub recursive: bool,
}
/// (TODO:@amason) Consider including the deleted SrvKeyspacePartitions objects
/// and any deleted Tablet objects here.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveShardCellResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReparentTabletRequest {
    /// Tablet is the alias of the tablet that should be reparented under the
    /// current shard primary.
    #[prost(message, optional, tag = "1")]
    pub tablet: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReparentTabletResponse {
    /// Keyspace is the name of the keyspace the tablet was reparented in.
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    /// Shard is the name of the shard the tablet was reparented in.
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    /// Primary is the alias of the tablet that the tablet was reparented under.
    #[prost(message, optional, tag = "3")]
    pub primary: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreFromBackupRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    /// BackupTime, if set, will use the backup taken most closely at or before
    /// this time. If nil, the latest backup will be restored on the tablet.
    #[prost(message, optional, tag = "2")]
    pub backup_time: ::core::option::Option<super::vttime::Time>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreFromBackupResponse {
    /// TabletAlias is the alias of the tablet doing the restore.
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(string, tag = "2")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub shard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub event: ::core::option::Option<super::logutil::Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunHealthCheckRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunHealthCheckResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetKeyspaceDurabilityPolicyRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub durability_policy: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetKeyspaceDurabilityPolicyResponse {
    /// Keyspace is the updated keyspace record.
    #[prost(message, optional, tag = "1")]
    pub keyspace: ::core::option::Option<super::topodata::Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetKeyspaceServedFromRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(enumeration = "super::topodata::TabletType", tag = "2")]
    pub tablet_type: i32,
    #[prost(string, repeated, tag = "3")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "4")]
    pub remove: bool,
    #[prost(string, tag = "5")]
    pub source_keyspace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetKeyspaceServedFromResponse {
    /// Keyspace is the updated keyspace record.
    #[prost(message, optional, tag = "1")]
    pub keyspace: ::core::option::Option<super::topodata::Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetKeyspaceShardingInfoRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub force: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetKeyspaceShardingInfoResponse {
    /// Keyspace is the updated keyspace record.
    #[prost(message, optional, tag = "1")]
    pub keyspace: ::core::option::Option<super::topodata::Keyspace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetShardIsPrimaryServingRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub is_serving: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetShardIsPrimaryServingResponse {
    /// Shard is the updated shard record.
    #[prost(message, optional, tag = "1")]
    pub shard: ::core::option::Option<super::topodata::Shard>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetShardTabletControlRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(enumeration = "super::topodata::TabletType", tag = "3")]
    pub tablet_type: i32,
    #[prost(string, repeated, tag = "4")]
    pub cells: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// DeniedTables updates the list of denied tables the shard will serve for
    /// the given tablet type. This is useful to fix tables that are being blocked
    /// after a MoveTables operation.
    ///
    /// NOTE: Setting this field will cause DisableQueryService to be ignored.
    #[prost(string, repeated, tag = "5")]
    pub denied_tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// DisableQueryService instructs whether to enable the query service on
    /// tablets of the given type in the shard. This is useful to fix Reshard
    /// operations gone awry.
    ///
    /// NOTE: this is ignored if DeniedTables is not empty.
    #[prost(bool, tag = "6")]
    pub disable_query_service: bool,
    /// Remove removes the ShardTabletControl record entirely. If set, this takes
    /// precedence over DeniedTables and DisableQueryService fields, and is useful
    /// to manually remove serving restrictions after a completed MoveTables
    /// operation.
    #[prost(bool, tag = "7")]
    pub remove: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetShardTabletControlResponse {
    /// Shard is the updated shard record.
    #[prost(message, optional, tag = "1")]
    pub shard: ::core::option::Option<super::topodata::Shard>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetWritableRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(bool, tag = "2")]
    pub writable: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetWritableResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReplicationAddRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReplicationAddResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReplicationFixRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub cell: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReplicationFixResponse {
    /// Error contains information about the error fixed by a
    /// ShardReplicationFix RPC. If there were no errors to fix (i.e. all nodes
    /// in the replication graph are valid), this field is nil.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::topodata::ShardReplicationError>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReplicationPositionsRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReplicationPositionsResponse {
    /// ReplicationStatuses is a mapping of tablet alias string to replication
    /// status for that tablet.
    #[prost(map = "string, message", tag = "1")]
    pub replication_statuses: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::replicationdata::Status,
    >,
    /// TabletMap is the set of tablets whose replication statuses were queried,
    /// keyed by tablet alias.
    #[prost(map = "string, message", tag = "2")]
    pub tablet_map: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::topodata::Tablet,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReplicationRemoveRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShardReplicationRemoveResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SleepTabletRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(message, optional, tag = "2")]
    pub duration: ::core::option::Option<super::vttime::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SleepTabletResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceShardAddRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub uid: u32,
    #[prost(string, tag = "4")]
    pub source_keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub source_shard: ::prost::alloc::string::String,
    /// KeyRange identifies the key range to use for the SourceShard. This field is
    /// optional.
    #[prost(message, optional, tag = "6")]
    pub key_range: ::core::option::Option<super::topodata::KeyRange>,
    /// Tables is a list of tables replicate (for MoveTables). Each "table" can be
    /// either an exact match or a regular expression of the form "/regexp/".
    #[prost(string, repeated, tag = "7")]
    pub tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceShardAddResponse {
    /// Shard is the updated shard record.
    #[prost(message, optional, tag = "1")]
    pub shard: ::core::option::Option<super::topodata::Shard>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceShardDeleteRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub uid: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceShardDeleteResponse {
    /// Shard is the updated shard record.
    #[prost(message, optional, tag = "1")]
    pub shard: ::core::option::Option<super::topodata::Shard>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartReplicationRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartReplicationResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationRequest {
    #[prost(message, optional, tag = "1")]
    pub tablet_alias: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReplicationResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabletExternallyReparentedRequest {
    /// Tablet is the alias of the tablet that was promoted externally and should
    /// be updated to the shard primary in the topo.
    #[prost(message, optional, tag = "1")]
    pub tablet: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabletExternallyReparentedResponse {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub new_primary: ::core::option::Option<super::topodata::TabletAlias>,
    #[prost(message, optional, tag = "4")]
    pub old_primary: ::core::option::Option<super::topodata::TabletAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCellInfoRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cell_info: ::core::option::Option<super::topodata::CellInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCellInfoResponse {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cell_info: ::core::option::Option<super::topodata::CellInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCellsAliasRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cells_alias: ::core::option::Option<super::topodata::CellsAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCellsAliasResponse {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub cells_alias: ::core::option::Option<super::topodata::CellsAlias>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateRequest {
    #[prost(bool, tag = "1")]
    pub ping_tablets: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateResponse {
    #[prost(string, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, message", tag = "2")]
    pub results_by_keyspace: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ValidateKeyspaceResponse,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub ping_tablets: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateKeyspaceResponse {
    #[prost(string, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, message", tag = "2")]
    pub results_by_shard: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ValidateShardResponse,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateSchemaKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub exclude_tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "3")]
    pub include_views: bool,
    #[prost(bool, tag = "4")]
    pub skip_no_primary: bool,
    #[prost(bool, tag = "5")]
    pub include_vschema: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateSchemaKeyspaceResponse {
    #[prost(string, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, message", tag = "2")]
    pub results_by_shard: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ValidateShardResponse,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateShardRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub ping_tablets: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateShardResponse {
    #[prost(string, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateVersionKeyspaceRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateVersionKeyspaceResponse {
    #[prost(string, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, message", tag = "2")]
    pub results_by_shard: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ValidateShardResponse,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateVersionShardRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub shard: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateVersionShardResponse {
    #[prost(string, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateVSchemaRequest {
    #[prost(string, tag = "1")]
    pub keyspace: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub shards: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub exclude_tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "4")]
    pub include_views: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateVSchemaResponse {
    #[prost(string, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, message", tag = "2")]
    pub results_by_shard: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ValidateShardResponse,
    >,
}
/// MaterializationIntent describes the reason for creating the Materialize flow
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MaterializationIntent {
    /// CUSTOM is the default value
    Custom = 0,
    /// MOVETABLES is when we are creating a MoveTables flow
    Movetables = 1,
    /// CREATELOOKUPINDEX is when we are creating a CreateLookupIndex flow
    Createlookupindex = 2,
}
impl MaterializationIntent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MaterializationIntent::Custom => "CUSTOM",
            MaterializationIntent::Movetables => "MOVETABLES",
            MaterializationIntent::Createlookupindex => "CREATELOOKUPINDEX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CUSTOM" => Some(Self::Custom),
            "MOVETABLES" => Some(Self::Movetables),
            "CREATELOOKUPINDEX" => Some(Self::Createlookupindex),
            _ => None,
        }
    }
}
