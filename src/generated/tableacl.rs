/// TableGroupSpec defines ACLs for a group of tables.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableGroupSpec {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// either tables or a table name prefixes (if it ends in a %)
    #[prost(string, repeated, tag = "2")]
    pub table_names_or_prefixes: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    #[prost(string, repeated, tag = "3")]
    pub readers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub writers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub admins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    #[prost(message, repeated, tag = "1")]
    pub table_groups: ::prost::alloc::vec::Vec<TableGroupSpec>,
}
