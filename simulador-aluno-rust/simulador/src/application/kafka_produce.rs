use super::route::Route;
use crate::infra::kafka::producer::publish;
use rdkafka::{message::OwnedMessage, producer::FutureProducer, Message};
use std::{env, thread, time::Duration};
pub async fn produce(msg: OwnedMessage, producer: FutureProducer) {
    let m = msg.payload().unwrap();
    let mut route: Route = serde_json::from_slice(m).unwrap();
    route
        .load_positions()
        .expect("load_positions error in produce");
    let positions = route
        .export_json_positions()
        .expect("produce error positons!");
    //
    let kafka_produce_topic = env::var("KafkaProduceTopic").expect("env KafkaProduceTopic error");
    for p in positions {
        let _r = publish(&p, &kafka_produce_topic, &producer).await;
        thread::sleep(Duration::from_millis(500));
    }
}
