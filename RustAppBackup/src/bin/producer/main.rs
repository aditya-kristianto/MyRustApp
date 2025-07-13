use futures_util::stream::StreamExt;
use producer::dotenv;
use rdkafka::ClientConfig;
use rdkafka::consumer::Consumer;
// use kafka::consumer::FetchOffset;
// use kafka::consumer::GroupOffsetStorage;
use rdkafka::consumer::StreamConsumer;
// use rdkafka::error::KafkaError;
use rdkafka::Message;


#[path = "../../modules/api/producer/mod.rs"]
pub mod producer;

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    // Initialize the logger
    env_logger::init();
    dotenv::init().await;

    let (_app_host, _app_port) = dotenv::get_http_host_and_port("producer");
    let _app_version = dotenv::get_app_version();

    // Replace with your actual Kafka bootstrap servers
    let bootstrap_servers = "localhost:9092";
    let group_id = "test-group";
    let _brokers = vec!["localhost:9092".to_string()];

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("group.id", group_id)
        .create()
        .expect("Failed to create consumer");
    println!("Topic Name");
    let topics = vec!["your_topic_name"];
    println!("Consume Message Init");
    // consume_messages(consumer, topics, brokers);
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

// fn consume_messages(group: String, topic: String, brokers: Vec<String>) -> Result<(), KafkaError> {
//     // Replace with your actual Kafka bootstrap servers
//     let bootstrap_servers = "localhost:9092";
//     let group_id = "test-group";

//     let mut con = <dyn Consumer>::from_hosts(brokers)
//         .with_topic(topic)
//         .with_group(group)
//         .with_fallback_offset(FetchOffset::Earliest)
//         .with_offset_storage(GroupOffsetStorage::Kafka)
//         .set("bootstrap.servers", &bootstrap_servers)
//         .set("group.id", &group_id)
//         .set("enable.auto.commit", "true")
//         .create()?;

//     loop {
//         let mss = con.poll()?;
//         if mss.is_empty() {
//             println!("No messages available right now.");
//             return Ok(());
//         }

//         for ms in mss.iter() {
//             for m in ms.messages() {
//                 println!(
//                     "{}:{}@{}: {:?}",
//                     ms.topic(),
//                     ms.partition(),
//                     m.offset,
//                     m.value
//                 );
//             }
//             let _ = con.consume_messageset(ms);
//         }
//         con.commit_consumed()?;
//     }
// }