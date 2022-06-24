use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig,
};
use std::env;
// A type alias with your custom consumer can be created for convenience.
type CustomConsumer = StreamConsumer;
pub struct KafkaConsumer;
impl KafkaConsumer {
    pub fn new() -> StreamConsumer {
        //envs
        let kafka_bootstrap_servers =
            env::var("KafkaBootstrapServers").expect("env kafka_bootstrap_servers error");
        let kafka_consumer_group_id =
            env::var("KafkaConsumerGroupId").expect("env KafkaConsumerGroupId error");
        let kafka_read_topic = env::var("KafkaReadTopic").expect("env KafkaReadTopic error");
        //
        let consumer: CustomConsumer = ClientConfig::new()
            .set("bootstrap.servers", kafka_bootstrap_servers)
            .set("group.id", kafka_consumer_group_id)
            // .set("security.protocol", value)
            // .set("sasl.mechanisms", value)
            // .set("sasl.username", value)
            // .set("sasl.passwor", value)
            // .set("enable.partition.eof", "false")
            // .set("session.timeout.ms", "6000")
            // .set("enable.auto.commit", "false")
            .create()
            .expect("Consumer creation failed");

        let topics = vec![kafka_read_topic.as_str()];
        _ = consumer.subscribe(&topics);
        consumer
    }
}
