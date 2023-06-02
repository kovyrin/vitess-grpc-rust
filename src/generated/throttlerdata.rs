/// MaxRatesRequest is the payload for the MaxRates RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxRatesRequest {}
/// MaxRatesResponse is returned by the MaxRates RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxRatesResponse {
    /// max_rates returns the max rate for each throttler. It's keyed by the
    /// throttler name.
    #[prost(map = "string, int64", tag = "1")]
    pub rates: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
}
/// SetMaxRateRequest is the payload for the SetMaxRate RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMaxRateRequest {
    #[prost(int64, tag = "1")]
    pub rate: i64,
}
/// SetMaxRateResponse is returned by the SetMaxRate RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMaxRateResponse {
    /// names is the list of throttler names which were updated.
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Configuration holds the configuration parameters for the
/// MaxReplicationLagModule which adaptively adjusts the throttling rate based on
/// the observed replication lag across all replicas.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Configuration {
    /// target_replication_lag_sec is the replication lag (in seconds) the
    /// MaxReplicationLagModule tries to aim for.
    /// If it is within the target, it tries to increase the throttler
    /// rate, otherwise it will lower it based on an educated guess of the
    /// replica's throughput.
    #[prost(int64, tag = "1")]
    pub target_replication_lag_sec: i64,
    /// max_replication_lag_sec is meant as a last resort.
    /// By default, the module tries to find out the system maximum capacity while
    /// trying to keep the replication lag around "target_replication_lag_sec".
    /// Usually, we'll wait min_duration_between_(increases|decreases)_sec to see
    /// the effect of a throttler rate change on the replication lag.
    /// But if the lag goes above this field's value we will go into an "emergency"
    /// state and throttle more aggressively (see "emergency_decrease" below).
    /// This is the only way to ensure that the system will recover.
    #[prost(int64, tag = "2")]
    pub max_replication_lag_sec: i64,
    /// initial_rate is the rate at which the module will start.
    #[prost(int64, tag = "3")]
    pub initial_rate: i64,
    /// max_increase defines by how much we will increase the rate
    /// e.g. 0.05 increases the rate by 5% while 1.0 by 100%.
    /// Note that any increase will let the system wait for at least
    /// (1 / MaxIncrease) seconds. If we wait for shorter periods of time, we
    /// won't notice if the rate increase also increases the replication lag.
    /// (If the system was already at its maximum capacity (e.g. 1k QPS) and we
    /// increase the rate by e.g. 5% to 1050 QPS, it will take 20 seconds until
    /// 1000 extra queries are buffered and the lag increases by 1 second.)
    #[prost(double, tag = "4")]
    pub max_increase: f64,
    /// emergency_decrease defines by how much we will decrease the current rate
    /// if the observed replication lag is above "max_replication_lag_sec".
    /// E.g. 0.50 decreases the current rate by 50%.
    #[prost(double, tag = "5")]
    pub emergency_decrease: f64,
    /// min_duration_between_increases_sec specifies how long we'll wait at least
    /// for the last rate increase to have an effect on the system.
    #[prost(int64, tag = "6")]
    pub min_duration_between_increases_sec: i64,
    /// max_duration_between_increases_sec specifies how long we'll wait at most
    /// for the last rate increase to have an effect on the system.
    #[prost(int64, tag = "7")]
    pub max_duration_between_increases_sec: i64,
    /// min_duration_between_decreases_sec specifies how long we'll wait at least
    /// for the last rate decrease to have an effect on the system.
    #[prost(int64, tag = "8")]
    pub min_duration_between_decreases_sec: i64,
    /// spread_backlog_across_sec is used when we set the throttler rate after
    /// we guessed the rate of a replica and determined its backlog.
    /// For example, at a guessed rate of 100 QPS and a lag of 10s, the replica has
    /// a backlog of 1000 queries.
    /// When we set the new, decreased throttler rate, we factor in how long it
    /// will take the replica to go through the backlog (in addition to new
    /// requests). This field specifies over which timespan we plan to spread this.
    /// For example, for a backlog of 1000 queries spread over 5s means that we
    /// have to further reduce the rate by 200 QPS or the backlog will not be
    /// processed within the 5 seconds.
    #[prost(int64, tag = "9")]
    pub spread_backlog_across_sec: i64,
    /// ignore_n_slowest_replicas will ignore replication lag updates from the
    /// N slowest REPLICA tablets. Under certain circumstances, replicas are still
    /// considered e.g. a) if the lag is at most max_replication_lag_sec, b) there
    /// are less than N+1 replicas or c) the lag increased on each replica such
    /// that all replicas were ignored in a row.
    #[prost(int32, tag = "10")]
    pub ignore_n_slowest_replicas: i32,
    /// ignore_n_slowest_rdonlys does the same thing as ignore_n_slowest_replicas
    /// but for RDONLY tablets. Note that these two settings are independent.
    #[prost(int32, tag = "11")]
    pub ignore_n_slowest_rdonlys: i32,
    /// age_bad_rate_after_sec is the duration after which an unchanged bad rate
    /// will "age out" and increase by "bad_rate_increase".
    /// Bad rates are tracked by the code in memory.go and serve as an upper bound
    /// for future rate changes. This ensures that the adaptive throttler does not
    /// try known too high (bad) rates over and over again.
    /// To avoid that temporary degradations permanently reduce the maximum rate,
    /// a stable bad rate "ages out" after "age_bad_rate_after_sec".
    #[prost(int64, tag = "12")]
    pub age_bad_rate_after_sec: i64,
    /// bad_rate_increase defines the percentage by which a bad rate will be
    /// increased when it's aging out.
    #[prost(double, tag = "13")]
    pub bad_rate_increase: f64,
    /// max_rate_approach_threshold is the fraction of the current rate limit that the actual
    /// rate must exceed for the throttler to increase the limit when the replication lag
    /// is below target_replication_lag_sec. For example, assuming the actual replication lag
    /// is below target_replication_lag_sec, if the current rate limit is 100, then the actual
    /// rate must exceed 100*max_rate_approach_threshold for the throttler to increase the current
    /// limit.
    #[prost(double, tag = "14")]
    pub max_rate_approach_threshold: f64,
}
/// GetConfigurationRequest is the payload for the GetConfiguration RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigurationRequest {
    /// throttler_name specifies which throttler to select. If empty, all active
    /// throttlers will be selected.
    #[prost(string, tag = "1")]
    pub throttler_name: ::prost::alloc::string::String,
}
/// GetConfigurationResponse is returned by the GetConfiguration RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigurationResponse {
    /// max_rates returns the configurations for each throttler.
    /// It's keyed by the throttler name.
    #[prost(map = "string, message", tag = "1")]
    pub configurations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        Configuration,
    >,
}
/// UpdateConfigurationRequest is the payload for the UpdateConfiguration RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConfigurationRequest {
    /// throttler_name specifies which throttler to update. If empty, all active
    /// throttlers will be updated.
    #[prost(string, tag = "1")]
    pub throttler_name: ::prost::alloc::string::String,
    /// configuration is the new (partial) configuration.
    #[prost(message, optional, tag = "2")]
    pub configuration: ::core::option::Option<Configuration>,
    /// copy_zero_values specifies whether fields with zero values should be copied
    /// as well.
    #[prost(bool, tag = "3")]
    pub copy_zero_values: bool,
}
/// UpdateConfigurationResponse is returned by the UpdateConfiguration RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConfigurationResponse {
    /// names is the list of throttler names which were updated.
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ResetConfigurationRequest is the payload for the ResetConfiguration RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetConfigurationRequest {
    /// throttler_name specifies which throttler to reset. If empty, all active
    /// throttlers will be reset.
    #[prost(string, tag = "1")]
    pub throttler_name: ::prost::alloc::string::String,
}
/// ResetConfigurationResponse is returned by the ResetConfiguration RPC.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetConfigurationResponse {
    /// names is the list of throttler names which were updated.
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
