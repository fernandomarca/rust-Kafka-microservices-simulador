use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use std::{env, error::Error, time::Duration};
pub fn new_kafka_producer() -> FutureProducer {
    //envs
    let kafka_bootstrap_servers =
        env::var("KafkaBootstrapServers").expect("env kafka_bootstrap_servers error");
    //
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_bootstrap_servers)
        // .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    producer
}
pub async fn publish(
    msg: &str,
    topic: &str,
    producer: &FutureProducer,
) -> Result<(), Box<dyn Error>> {
    let record: FutureRecord<String, str> = FutureRecord::to(topic).payload(msg);
    let produce_future = producer.send(record, Duration::from_secs(0));

    match produce_future.await {
        Ok(delivery) => {
            println!("Sent: {:?} msg: {}", delivery, msg);
            Ok(())
        }
        Err((e, _)) => Err(format!("Error: {:?}", e))?,
    }
}
