/// Workflow is the persisted state of a long-running workflow.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workflow {
    /// uuid is set when the workflow is created, and immutable after
    /// that.
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    /// factory_name is set with the name of the factory that created the
    /// job (and can also restart it). It is set at creation time, and
    /// immutable after that.
    #[prost(string, tag = "2")]
    pub factory_name: ::prost::alloc::string::String,
    /// name is the display name of the workflow.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// state describes the state of the job. A job is created as
    /// NotStarted, then the Workflow Manager picks it up and starts it,
    /// switching it to Running (and populating 'start_time').  The
    /// workflow can then fail over to a new Workflow Manager is
    /// necessary, and still be in Running state.  When done, it goes to
    /// Done, 'end_time' is populated, and 'error' is set if there was an
    /// error.
    #[prost(enumeration = "WorkflowState", tag = "4")]
    pub state: i32,
    /// data is workflow-specific stored data. It is usually a binary
    /// proto-encoded data structure. It can vary throughout the
    /// execution of the workflow.  It will not change after the workflow
    /// is Done.
    #[prost(bytes = "vec", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// error is set if the job finished with an error. This field only
    /// makes sense if 'state' is Done.
    #[prost(string, tag = "6")]
    pub error: ::prost::alloc::string::String,
    /// start_time is set when the workflow manager starts a workflow for
    /// the first time. This field only makes sense if 'state' is Running
    /// or Done.
    #[prost(int64, tag = "7")]
    pub start_time: i64,
    /// end_time is set when the workflow is finished.
    /// This field only makes sense if 'state' is Done.
    #[prost(int64, tag = "8")]
    pub end_time: i64,
    /// create_time is set when the workflow is created.
    #[prost(int64, tag = "9")]
    pub create_time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkflowCheckpoint {
    /// code_version is used to detect incompabilities between the version of the
    /// running workflow and the one which wrote the checkpoint. If they don't
    /// match, the workflow must not continue. The author of workflow must update
    /// this variable in their implementation when incompabilities are introduced.
    ///
    /// tasks stores all tasks of the workflow in a map. The key is a unique name
    /// to identify the task, e.g. clone/-80.
    #[prost(int32, tag = "1")]
    pub code_version: i32,
    /// Task is the data structure that stores the execution status and the
    /// attributes of a task.
    #[prost(map = "string, message", tag = "2")]
    pub tasks: ::std::collections::HashMap<::prost::alloc::string::String, Task>,
    /// settings includes workflow specific data, e.g. the resharding workflow
    /// would store the source shards and destination shards.
    #[prost(map = "string, string", tag = "3")]
    pub settings: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration = "TaskState", tag = "2")]
    pub state: i32,
    /// attributes includes the parameters the task needs.
    #[prost(map = "string, string", tag = "3")]
    pub attributes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "4")]
    pub error: ::prost::alloc::string::String,
}
/// WorkflowState describes the state of a workflow.
/// This constant should match the Node object described in
/// web/vtctld2/src/app/workflows/node.ts as it is exposed as JSON to
/// the Angular 2 web app.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WorkflowState {
    NotStarted = 0,
    Running = 1,
    Done = 2,
}
impl WorkflowState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WorkflowState::NotStarted => "NotStarted",
            WorkflowState::Running => "Running",
            WorkflowState::Done => "Done",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NotStarted" => Some(Self::NotStarted),
            "Running" => Some(Self::Running),
            "Done" => Some(Self::Done),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskState {
    TaskNotStarted = 0,
    TaskRunning = 1,
    TaskDone = 2,
}
impl TaskState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaskState::TaskNotStarted => "TaskNotStarted",
            TaskState::TaskRunning => "TaskRunning",
            TaskState::TaskDone => "TaskDone",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TaskNotStarted" => Some(Self::TaskNotStarted),
            "TaskRunning" => Some(Self::TaskRunning),
            "TaskDone" => Some(Self::TaskDone),
            _ => None,
        }
    }
}
