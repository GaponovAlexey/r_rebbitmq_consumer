use std::result::Result;
use futures::stream::StreamExt;
use lapin::{ Connection, ConnectionProperties, options::*, types::FieldTable };
use std::str::from_utf8;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default()
    ).await?;
    let channel = conn.create_channel().await?;

    let queue = "dev-queue";
    channel.queue_declare(
        queue,
        QueueDeclareOptions { durable: true, ..Default::default() },
        Default::default()
    ).await?;

    let mut ch = channel.basic_consume(
        queue,
        queue,
        BasicConsumeOptions::default(),
        FieldTable::default()
    ).await?;

    while let Some(delivery) = ch.next().await {
        println!("SUBSCRIBE");
        let delivery = delivery.expect("error in consumer");
        let data_str = from_utf8(&delivery.data).expect("Error decoding utf8");
        println!("Data: {}", data_str);
        delivery.ack(BasicAckOptions::default()).await.expect("ack");
    }
    Ok(())
}
