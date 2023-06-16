/// CallerID is passed along RPCs to identify the originating client
/// for a request. It is not meant to be secure, but only
/// informational.  The client can put whatever info they want in these
/// fields, and they will be trusted by the servers. The fields will
/// just be used for logging purposes, and to easily find a client.
/// VtGate propagates it to VtTablet, and VtTablet may use this
/// information for monitoring purposes, to display on dashboards, or
/// for denying access to tables during a migration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallerId {
    /// principal is the effective user identifier. It is usually filled in
    /// with whoever made the request to the appserver, if the request
    /// came from an automated job or another system component.
    /// If the request comes directly from the Internet, or if the Vitess client
    /// takes action on its own accord, it is okay for this field to be absent.
    #[prost(string, tag = "1")]
    pub principal: ::prost::alloc::string::String,
    /// component describes the running process of the effective caller.
    /// It can for instance be the hostname:port of the servlet initiating the
    /// database call, or the container engine ID used by the servlet.
    #[prost(string, tag = "2")]
    pub component: ::prost::alloc::string::String,
    /// subcomponent describes a component inisde the immediate caller which
    /// is responsible for generating is request. Suggested values are a
    /// servlet name or an API endpoint name.
    #[prost(string, tag = "3")]
    pub subcomponent: ::prost::alloc::string::String,
    /// set of security groups that should be assigned to this caller.
    #[prost(string, repeated, tag = "4")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RPCError is an application-level error structure returned by
/// VtTablet (and passed along by VtGate if appropriate).
/// We use this so the clients don't have to parse the error messages,
/// but instead can depend on the value of the code.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpcError {
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    #[prost(enumeration = "Code", tag = "3")]
    pub code: i32,
}
/// Code represents canonical error codes. The names, numbers and comments
/// must match the ones defined by grpc (0-16):
///    <https://godoc.org/google.golang.org/grpc/codes.>
/// 17+ are custom codes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Code {
    /// OK is returned on success.
    Ok = 0,
    /// CANCELED indicates the operation was cancelled (typically by the caller).
    Canceled = 1,
    /// UNKNOWN error. An example of where this error may be returned is
    /// if a Status value received from another address space belongs to
    /// an error-space that is not known in this address space. Also
    /// errors raised by APIs that do not return enough error information
    /// may be converted to this error.
    Unknown = 2,
    /// INVALID_ARGUMENT indicates client specified an invalid argument.
    /// Note that this differs from FAILED_PRECONDITION. It indicates arguments
    /// that are problematic regardless of the state of the system
    /// (e.g., a malformed file name).
    InvalidArgument = 3,
    /// DEADLINE_EXCEEDED means operation expired before completion.
    /// For operations that change the state of the system, this error may be
    /// returned even if the operation has completed successfully. For
    /// example, a successful response from a server could have been delayed
    /// long enough for the deadline to expire.
    DeadlineExceeded = 4,
    /// NOT_FOUND means some requested entity (e.g., file or directory) was
    /// not found.
    NotFound = 5,
    /// ALREADY_EXISTS means an attempt to create an entity failed because one
    /// already exists.
    AlreadyExists = 6,
    /// PERMISSION_DENIED indicates the caller does not have permission to
    /// execute the specified operation. It must not be used for rejections
    /// caused by exhausting some resource (use RESOURCE_EXHAUSTED
    /// instead for those errors).  It must not be
    /// used if the caller cannot be identified (use Unauthenticated
    /// instead for those errors).
    PermissionDenied = 7,
    /// RESOURCE_EXHAUSTED indicates some resource has been exhausted, perhaps
    /// a per-user quota, or perhaps the entire file system is out of space.
    ResourceExhausted = 8,
    /// FAILED_PRECONDITION indicates operation was rejected because the
    /// system is not in a state required for the operation's execution.
    /// For example, directory to be deleted may be non-empty, an rmdir
    /// operation is applied to a non-directory, etc.
    ///
    /// A litmus test that may help a service implementor in deciding
    /// between FAILED_PRECONDITION, ABORTED, and UNAVAILABLE:
    ///   (a) Use UNAVAILABLE if the client can retry just the failing call.
    ///   (b) Use ABORTED if the client should retry at a higher-level
    ///       (e.g., restarting a read-modify-write sequence).
    ///   (c) Use FAILED_PRECONDITION if the client should not retry until
    ///       the system state has been explicitly fixed.  E.g., if an "rmdir"
    ///       fails because the directory is non-empty, FAILED_PRECONDITION
    ///       should be returned since the client should not retry unless
    ///       they have first fixed up the directory by deleting files from it.
    ///   (d) Use FAILED_PRECONDITION if the client performs conditional
    ///       REST Get/Update/Delete on a resource and the resource on the
    ///       server does not match the condition. E.g., conflicting
    ///       read-modify-write on the same resource.
    FailedPrecondition = 9,
    /// ABORTED indicates the operation was aborted, typically due to a
    /// concurrency issue like sequencer check failures, transaction aborts,
    /// etc.
    ///
    /// See litmus test above for deciding between FAILED_PRECONDITION,
    /// ABORTED, and UNAVAILABLE.
    Aborted = 10,
    /// OUT_OF_RANGE means operation was attempted past the valid range.
    /// E.g., seeking or reading past end of file.
    ///
    /// Unlike INVALID_ARGUMENT, this error indicates a problem that may
    /// be fixed if the system state changes. For example, a 32-bit file
    /// system will generate INVALID_ARGUMENT if asked to read at an
    /// offset that is not in the range \[0,2^32-1\], but it will generate
    /// OUT_OF_RANGE if asked to read from an offset past the current
    /// file size.
    ///
    /// There is a fair bit of overlap between FAILED_PRECONDITION and
    /// OUT_OF_RANGE.  We recommend using OUT_OF_RANGE (the more specific
    /// error) when it applies so that callers who are iterating through
    /// a space can easily look for an OUT_OF_RANGE error to detect when
    /// they are done.
    OutOfRange = 11,
    /// UNIMPLEMENTED indicates operation is not implemented or not
    /// supported/enabled in this service.
    Unimplemented = 12,
    /// INTERNAL errors. Means some invariants expected by underlying
    /// system has been broken.  If you see one of these errors,
    /// something is very broken.
    Internal = 13,
    /// UNAVAILABLE indicates the service is currently unavailable.
    /// This is a most likely a transient condition and may be corrected
    /// by retrying with a backoff.
    ///
    /// See litmus test above for deciding between FAILED_PRECONDITION,
    /// ABORTED, and UNAVAILABLE.
    Unavailable = 14,
    /// DATA_LOSS indicates unrecoverable data loss or corruption.
    DataLoss = 15,
    /// UNAUTHENTICATED indicates the request does not have valid
    /// authentication credentials for the operation.
    Unauthenticated = 16,
    /// CLUSTER_EVENT indicates that a cluster operation might be in effect
    ClusterEvent = 17,
    /// Topo server connection is read-only
    ReadOnly = 18,
}
impl Code {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Code::Ok => "OK",
            Code::Canceled => "CANCELED",
            Code::Unknown => "UNKNOWN",
            Code::InvalidArgument => "INVALID_ARGUMENT",
            Code::DeadlineExceeded => "DEADLINE_EXCEEDED",
            Code::NotFound => "NOT_FOUND",
            Code::AlreadyExists => "ALREADY_EXISTS",
            Code::PermissionDenied => "PERMISSION_DENIED",
            Code::ResourceExhausted => "RESOURCE_EXHAUSTED",
            Code::FailedPrecondition => "FAILED_PRECONDITION",
            Code::Aborted => "ABORTED",
            Code::OutOfRange => "OUT_OF_RANGE",
            Code::Unimplemented => "UNIMPLEMENTED",
            Code::Internal => "INTERNAL",
            Code::Unavailable => "UNAVAILABLE",
            Code::DataLoss => "DATA_LOSS",
            Code::Unauthenticated => "UNAUTHENTICATED",
            Code::ClusterEvent => "CLUSTER_EVENT",
            Code::ReadOnly => "READ_ONLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OK" => Some(Self::Ok),
            "CANCELED" => Some(Self::Canceled),
            "UNKNOWN" => Some(Self::Unknown),
            "INVALID_ARGUMENT" => Some(Self::InvalidArgument),
            "DEADLINE_EXCEEDED" => Some(Self::DeadlineExceeded),
            "NOT_FOUND" => Some(Self::NotFound),
            "ALREADY_EXISTS" => Some(Self::AlreadyExists),
            "PERMISSION_DENIED" => Some(Self::PermissionDenied),
            "RESOURCE_EXHAUSTED" => Some(Self::ResourceExhausted),
            "FAILED_PRECONDITION" => Some(Self::FailedPrecondition),
            "ABORTED" => Some(Self::Aborted),
            "OUT_OF_RANGE" => Some(Self::OutOfRange),
            "UNIMPLEMENTED" => Some(Self::Unimplemented),
            "INTERNAL" => Some(Self::Internal),
            "UNAVAILABLE" => Some(Self::Unavailable),
            "DATA_LOSS" => Some(Self::DataLoss),
            "UNAUTHENTICATED" => Some(Self::Unauthenticated),
            "CLUSTER_EVENT" => Some(Self::ClusterEvent),
            "READ_ONLY" => Some(Self::ReadOnly),
            _ => None,
        }
    }
}
