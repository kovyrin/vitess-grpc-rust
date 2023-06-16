/// Event is a single logging event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<super::vttime::Time>,
    #[prost(enumeration = "Level", tag = "2")]
    pub level: i32,
    #[prost(string, tag = "3")]
    pub file: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub line: i64,
    #[prost(string, tag = "5")]
    pub value: ::prost::alloc::string::String,
}
/// Level is the level of the log messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Level {
    /// The usual logging levels.
    /// Should be logged using logging facility.
    Info = 0,
    Warning = 1,
    Error = 2,
    /// For messages that may contains non-logging events.
    /// Should be logged to console directly.
    Console = 3,
}
impl Level {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Level::Info => "INFO",
            Level::Warning => "WARNING",
            Level::Error => "ERROR",
            Level::Console => "CONSOLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INFO" => Some(Self::Info),
            "WARNING" => Some(Self::Warning),
            "ERROR" => Some(Self::Error),
            "CONSOLE" => Some(Self::Console),
            _ => None,
        }
    }
}
