use anyhow::Result;
use vitess_grpc::query::BoundQuery;
use vitess_grpc::vtgateservice::vitess_client::VitessClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // See docker-compose.yml for the details of the vitess deployment
    let vitess_url = "http://127.0.0.1:15301";

    // Connect to Vitess
    let mut client = VitessClient::connect(vitess_url)
        .await
        .expect("Failed to connect to Vitess");

    // Prepare the query to be executed
    let exec_request = vitess_grpc::vtgate::ExecuteRequest {
        query: Some(BoundQuery {
            sql: "SHOW TABLES FROM commerce".into(),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Execute the query
    let res = client.execute(exec_request).await?;

    // Print the results
    if let Some(result) = res.into_inner().result {
        let fields = result.fields;
        println!("fields: {:?}", fields);

        for row in result.rows {
            println!("row: {:?}", row);
        }
    } else {
        println!("no result");
    }

    Ok(())
}
