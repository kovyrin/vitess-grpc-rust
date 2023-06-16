use vitess_grpc::vtgateservice::vitess_client::VitessClient;
use vitess_grpc::topodata::TabletType;
use vitess_grpc::vtgate::{VStreamRequest, VStreamFlags};
use vitess_grpc::binlogdata::{VGtid, ShardGtid, VEventType};

#[tokio::test]
async fn integration() {
    // See docker-compose.yml for the details of the vitess deployment
    let vitess_url = "http://127.0.0.1:15301";
    let vitess_keyspace = "commerce".to_string();

    // Connect to Vitess
    let mut client = VitessClient::connect(vitess_url).await.expect("Failed to connect to Vitess");

    // Configure the details of VStream
    let vstream_flags = VStreamFlags {
        stop_on_reshard: true,
        heartbeat_interval: 5,
        ..Default::default()
    };

    // Start from the current position
    let initial_position = VGtid {
        shard_gtids: vec![
            ShardGtid {
                keyspace: vitess_keyspace.clone(),
                shard: "".to_string(),
                gtid: "current".to_string(),
                ..Default::default()
            },
        ],
    };

    // Make the VStream API request to start streaming changes from the cluster
    let request = VStreamRequest {
        vgtid: Some(initial_position),
        tablet_type: TabletType::Primary.into(),
        flags: Some(vstream_flags),
        ..Default::default()
    };

    let vstream = client.v_stream(request).await.expect("Failed to start VStream");

    let mut response_stream = vstream.into_inner();
    let response = response_stream.message().await.unwrap().unwrap();

    assert_eq!(response.events.len(), 2);
    dbg!(&response);

    let message1 = &response.events[0];
    assert_eq!(message1.r#type, VEventType::Vgtid as i32);
    assert_eq!(message1.keyspace, vitess_keyspace);
    assert_eq!(message1.shard, "0");

    let vgtid = message1.vgtid.as_ref().unwrap();
    assert_eq!(vgtid.shard_gtids.len(), 1);
    assert_eq!(vgtid.shard_gtids[0].keyspace, vitess_keyspace);
    assert!(vgtid.shard_gtids[0].gtid.starts_with("MySQL"));

    let message2 = &response.events[1];
    assert_eq!(message2.r#type, VEventType::Other as i32);
    assert_eq!(message2.keyspace, vitess_keyspace);
    assert_eq!(message2.shard, "0");
    assert_eq!(message2.vgtid, None);
}
