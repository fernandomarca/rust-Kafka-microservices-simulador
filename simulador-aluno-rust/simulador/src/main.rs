use crate::{
    application::kafka_produce::produce,
    infra::kafka::{consumer::KafkaConsumer, producer::new_kafka_producer},
};
use dotenv::dotenv;
use futures::{stream::FuturesUnordered, StreamExt, TryStreamExt};
use log::info;

mod application;
mod infra;

async fn run_async_processor() {
    let consumer = KafkaConsumer::new();
    // Create the outer pipeline on the message stream.
    let stream_processor = consumer
        .stream()
        .try_for_each(|borrowed_message| async move {
            let producer = new_kafka_producer().clone();
            let owned_message = borrowed_message.detach();
            tokio::spawn(async move {
                info!("produce in execution: {:?}", owned_message);
                produce(owned_message, producer).await;
            });
            Ok(())
        });
    info!("Starting event loop");
    stream_processor.await.expect("stream processing failed");
    info!("Stream processing terminated!");
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    const NUM_WORKERS: u32 = 6;
    (0..NUM_WORKERS)
        .map(|_| tokio::spawn(run_async_processor()))
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async { () })
        .await
}

//kafka-console-consumer --bootstrap-server=localhost:90902 --topic=readtest
//kafka-console-producer --bootstrap-server=localhost:9092 --topic=readtest
