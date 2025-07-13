use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::error::Error;
use std::env;

pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init() -> Result<Self, Error> {
        // Parse your connection string into an options struct
        // Attempt to retrieve the Google Client ID from the environment variable
        // let client_options = match env::var("MONGODB_URI") {
        //     Ok(value) => ClientOptions::parse(value),
        //     Err(_) => {
        //         eprintln!("Error: MONGODB_URI environment variable not set or is empty.");
        //         // Handle the error gracefully, such as returning a default value or terminating the application.
        //         std::process::exit(1);
        //     }
        // };

        // Manually set an option
        // client_options.app_name = Some(env::var("APP_NAME").expect("").to_string());

        // Get a handle to the cluster
        // let client = Client::with_options(client_options)?;

        // Ping the server to see if you can connect to the cluster
        // client
        //     .database(dotenv!("MONGODB_DATABASE"))
        //     .run_command(doc! {"ping": "1"}, None)
        //     .await?;

        // println!("MongoDB connected successfully.");
        // for db_name in client.list_database_names(None, None).await {
        //     println!("{:?}", db_name);
        // }

        // let database = client.database("rust");
        // let collection = database.collection("products");
        // let my_document = doc! {"item":"motorcycle", "qty": "50"};
        // collection.insert_many(my_document, None).await?;
        // let bson_document = to_document(&my_document)?;
        // collection.insert_one(bson_document, None).await?;

        // Retrieve the value from the environment variable
        // let app_name = env::var("APP_NAME").expect("APP_NAME environment variable not set.");
        let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI environment variable not set.");

        // Build the ClientOptions outside of the future
        let client_options = ClientOptions::parse(mongodb_uri).await.expect("Failed to parse ClientOptions.");

        // Set the app_name field on the ClientOptions
        // client_options.app_name = Some(app_name);

        // Create the MongoDB client using the resolved client_options
        let client = Client::with_options(client_options).expect("Failed to create MongoDB client.");

        Ok(Self{
            client: client,
        })
    }
}
