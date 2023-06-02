pub mod vitess_grpc {
    pub mod automation { include!(concat!("generated/automation.rs")); }
    pub mod automationservice { include!(concat!("generated/automationservice.rs")); }
    pub mod binlogdata { include!(concat!("generated/binlogdata.rs")); }
    pub mod binlogservice { include!(concat!("generated/binlogservice.rs")); }
    pub mod logutil { include!(concat!("generated/logutil.rs")); }
    pub mod mysqlctl { include!(concat!("generated/mysqlctl.rs")); }
    pub mod query { include!(concat!("generated/query.rs")); }
    pub mod queryservice { include!(concat!("generated/queryservice.rs")); }
    pub mod replicationdata { include!(concat!("generated/replicationdata.rs")); }
    pub mod tableacl { include!(concat!("generated/tableacl.rs")); }
    pub mod tabletmanagerdata { include!(concat!("generated/tabletmanagerdata.rs")); }
    pub mod tabletmanagerservice { include!(concat!("generated/tabletmanagerservice.rs")); }
    pub mod throttlerdata { include!(concat!("generated/throttlerdata.rs")); }
    pub mod throttlerservice { include!(concat!("generated/throttlerservice.rs")); }
    pub mod topodata { include!(concat!("generated/topodata.rs")); }
    pub mod vschema { include!(concat!("generated/vschema.rs")); }
    pub mod vtadmin { include!(concat!("generated/vtadmin.rs")); }
    pub mod vtctldata { include!(concat!("generated/vtctldata.rs")); }
    pub mod vtctlservice { include!(concat!("generated/vtctlservice.rs")); }
    pub mod vtgate { include!(concat!("generated/vtgate.rs")); }
    pub mod vtgateservice { include!(concat!("generated/vtgateservice.rs")); }
    pub mod vtrpc { include!(concat!("generated/vtrpc.rs")); }
    pub mod vttest { include!(concat!("generated/vttest.rs")); }
    pub mod vttime { include!(concat!("generated/vttime.rs")); }
    pub mod workflow { include!(concat!("generated/workflow.rs")); }
}
