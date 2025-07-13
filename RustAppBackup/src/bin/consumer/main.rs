use futures::StreamExt;
use rdkafka::ClientConfig;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use rdkafka::Message;

#[path = "../../../pkg/dotenv/dotenv.rs"] mod dotenv;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    dotenv::init().await;

    let (_app_host, _app_port) = dotenv::get_http_host_and_port("consumer");
    let _app_version = dotenv::get_app_version();

    // Replace with your actual Kafka bootstrap servers
    let bootstrap_servers = "localhost:9092";
    let group_id = "test-group";

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("group.id", group_id)
        .create()
        .expect("Failed to create consumer");
    println!("Topic Name");
    let topics = vec!["your_topic_name"];
    println!("Consume Message Init");
    consume_messages(&consumer, topics).await;
    println!("Consume Message Finish");

    Ok(())
}

async fn consume_messages(consumer: &StreamConsumer, topics: Vec<&str>) {
    println!("Consume Message Start");
    // Subscribe to topics
    consumer.subscribe(&topics).expect("Error subscribing to topics");
    println!("Consume Message Subscribe Success");
    // Start consuming messages
    let mut stream = consumer.stream();
    println!("Start stream");
    while let Some(message) = stream.next().await {
        println!("message {:?}", message);
        match message {
            Ok(msg) => {
                if let Some(payload) = msg.payload_view::<str>() {
                    println!("Received message: {:?}", payload);
                }
            }
            Err(err) => eprintln!("Error receiving message: {:?}", err),
        }
    }
    println!("Consume Message End");
}