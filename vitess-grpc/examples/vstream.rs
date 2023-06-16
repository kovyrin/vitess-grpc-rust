use vitess_grpc::binlogdata::{VGtid, ShardGtid};
use vitess_grpc::topodata::TabletType;
use vitess_grpc::vtgate::{VStreamRequest, VStreamFlags};
use vitess_grpc::vtgateservice::vitess_client::VitessClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                keyspace: vitess_keyspace,
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

    // Stream messages one at a time
    let mut response_stream = vstream.into_inner();
    while let Some(response) = response_stream.message().await? {
        for message in response.events {
            println!("Received Vitess event: {:?}\n", message);
        }
    }

    Ok(())
}
