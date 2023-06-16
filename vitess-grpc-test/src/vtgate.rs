use vitess_grpc::vtgateservice::vitess_client::VitessClient;
use vitess_grpc::vtgate::VStreamRequest;
use vitess_grpc::binlogdata::{VGtid, ShardGtid, VEventType};

use mysql::*;
use mysql::prelude::*;

// See docker-compose.yml for the details of the vitess deployment
const VITESS_HOST: &str = "127.0.0.1";
const VITESS_PORT: u16 = 15301;
const MYSQL_PORT: u16 = 15303;
const VITESS_KEYSPACE: &str = "commerce";

#[tokio::test]
async fn vstream_integration() {
    // Connect to Vitess via the MySQL port
    let mysql_url = format!("mysql://root:@{}:{}/{}", VITESS_HOST, MYSQL_PORT, VITESS_KEYSPACE);
    let mysql_opts = Opts::from_url(&mysql_url.as_str()).expect("Failed to parse MySQL URL");
    let pool = Pool::new(mysql_opts).expect("Failed to connect to MySQL");
    let mut conn = pool.get_conn().expect("Failed to get MySQL connection");

    // TRUNCATE the table to ensure we have a clean slate
    let _ = conn.query_drop("TRUNCATE TABLE product");

    // Connect to Vitess via gRPC
    let vitess_url = format!("http://{}:{}", VITESS_HOST, VITESS_PORT);
    let mut client = VitessClient::connect(vitess_url).await.expect("Failed to connect to Vitess");

    // Start from the current position
    let vitess_keyspace = VITESS_KEYSPACE.to_string();
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
    let request = VStreamRequest { vgtid: Some(initial_position), ..Default::default() };
    let vstream = client.v_stream(request).await.expect("Failed to start VStream");
    let mut response_stream = vstream.into_inner();

    // The VStream should send us a set of messages describing the current position
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
    assert_eq!(message2.keyspace, VITESS_KEYSPACE.to_string());
    assert_eq!(message2.shard, "0");
    assert_eq!(message2.vgtid, None);

    // Insert a row into the table
    let _ = conn.query_drop("INSERT INTO product SET sku='sku-1', description='Product 1', price=42");

    // The VStream should send us a set of messages describing the change
    let response = response_stream.message().await.unwrap().unwrap();
    assert_eq!(response.events.len(), 5); // BEGIN, FIELD, ROW, VGTID, COMMIT
    dbg!(&response);

    // BEGIN event
    let begin = &response.events[0];
    assert_eq!(begin.r#type, VEventType::Begin as i32);

    // FIELD event describing the schema for the table (since this is the first change we see)
    let field = &response.events[1];
    assert_eq!(field.r#type, VEventType::Field as i32);
    let field_event = field.field_event.as_ref().unwrap();
    assert_eq!(field_event.table_name, "commerce.product");
    assert_eq!(field_event.fields.len(), 3);

    // ROW event describing the inserted row
    let row = &response.events[2];
    assert_eq!(row.r#type, VEventType::Row as i32);

    let row_event = row.row_event.as_ref().unwrap();
    assert_eq!(row_event.table_name, "commerce.product");
    assert_eq!(row_event.row_changes.len(), 1);

    // The row change should be an INSERT (no before state, after state is present)
    let row_change = &row_event.row_changes[0];
    assert_eq!(row_change.before, None);
    assert!(row_change.after.is_some());

    // VGTID event describing the current position after the changes
    let vgtid = &response.events[3];
    assert_eq!(vgtid.r#type, VEventType::Vgtid as i32);

    // COMMIT event
    let commit = &response.events[4];
    assert_eq!(commit.r#type, VEventType::Commit as i32);
}
