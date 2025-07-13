use bson::doc;
use dotenv_codegen::dotenv;
use mongodb::Client;
use mongodb::options::ClientOptions;


#[allow(dead_code)]
async fn init() {
    let client_options =
        ClientOptions::parse(dotenv!("MONGODB_URI")).await.expect("Some error message");
    
    // Get a handle to the cluster
    let client = Client::with_options(client_options).expect("Some error message");
    
    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": "1"})
        .await.expect("Some error message");
    println!("Connected successfully.");
    // List the names of the databases in that cluster
    for db_name in client.list_database_names().await.expect("Some error message") {
        println!("{}", db_name);
    }
}