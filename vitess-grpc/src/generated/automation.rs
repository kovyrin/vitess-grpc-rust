#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterOperation {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// TaskContainer are processed sequentially, one at a time.
    #[prost(message, repeated, tag = "2")]
    pub serial_tasks: ::prost::alloc::vec::Vec<TaskContainer>,
    /// Cached value. This has to be re-evaluated e.g. after a checkpoint load because running tasks may have already finished.
    #[prost(enumeration = "ClusterOperationState", tag = "3")]
    pub state: i32,
    /// Error of the first task which failed. Set after state advanced to CLUSTER_OPERATION_DONE. If empty, all tasks succeeded. Cached value, see state above.
    #[prost(string, tag = "4")]
    pub error: ::prost::alloc::string::String,
}
/// TaskContainer holds one or more task which may be executed in parallel.
/// "concurrency", if > 0, limits the amount of concurrently executed tasks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskContainer {
    #[prost(message, repeated, tag = "1")]
    pub parallel_tasks: ::prost::alloc::vec::Vec<Task>,
    #[prost(int32, tag = "2")]
    pub concurrency: i32,
}
/// Task represents a specific task which should be automatically executed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Task specification.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Runtime data.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "TaskState", tag = "4")]
    pub state: i32,
    /// Set after state advanced to DONE.
    #[prost(string, tag = "5")]
    pub output: ::prost::alloc::string::String,
    /// Set after state advanced to DONE. If empty, the task did succeed.
    #[prost(string, tag = "6")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnqueueClusterOperationRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "2")]
    pub parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnqueueClusterOperationResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterOperationStateRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterOperationStateResponse {
    #[prost(enumeration = "ClusterOperationState", tag = "1")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterOperationDetailsRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterOperationDetailsResponse {
    /// Full snapshot of the execution e.g. including output of each task.
    #[prost(message, optional, tag = "2")]
    pub cluster_op: ::core::option::Option<ClusterOperation>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClusterOperationState {
    UnknownClusterOperationState = 0,
    ClusterOperationNotStarted = 1,
    ClusterOperationRunning = 2,
    ClusterOperationDone = 3,
}
impl ClusterOperationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ClusterOperationState::UnknownClusterOperationState => {
                "UNKNOWN_CLUSTER_OPERATION_STATE"
            }
            ClusterOperationState::ClusterOperationNotStarted => {
                "CLUSTER_OPERATION_NOT_STARTED"
            }
            ClusterOperationState::ClusterOperationRunning => "CLUSTER_OPERATION_RUNNING",
            ClusterOperationState::ClusterOperationDone => "CLUSTER_OPERATION_DONE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_CLUSTER_OPERATION_STATE" => Some(Self::UnknownClusterOperationState),
            "CLUSTER_OPERATION_NOT_STARTED" => Some(Self::ClusterOperationNotStarted),
            "CLUSTER_OPERATION_RUNNING" => Some(Self::ClusterOperationRunning),
            "CLUSTER_OPERATION_DONE" => Some(Self::ClusterOperationDone),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskState {
    UnknownTaskState = 0,
    NotStarted = 1,
    Running = 2,
    Done = 3,
}
impl TaskState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaskState::UnknownTaskState => "UNKNOWN_TASK_STATE",
            TaskState::NotStarted => "NOT_STARTED",
            TaskState::Running => "RUNNING",
            TaskState::Done => "DONE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_TASK_STATE" => Some(Self::UnknownTaskState),
            "NOT_STARTED" => Some(Self::NotStarted),
            "RUNNING" => Some(Self::Running),
            "DONE" => Some(Self::Done),
            _ => None,
        }
    }
}
