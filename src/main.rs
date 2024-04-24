use std::env;
use amqprs::{
    callbacks::{
        DefaultChannelCallback,
        DefaultConnectionCallback
    },
    channel::{
        BasicConsumeArguments,
        QueueBindArguments,
        QueueDeclareArguments,
    },
    connection::{
        Connection,
        OpenConnectionArguments
    }
};
use tokio::sync::Notify;
use tracing_subscriber::{fmt, prelude::*};
pub mod consumer;
use consumer::temporal_videos_consumer::TemporalVideosConsumer;


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let rabbit_host = env::var("RABBIT_HOST").unwrap();
    let rabbit_user = env::var("RABBIT_USER").unwrap();
    let rabbit_password = env::var("RABBIT_PASSWORD").unwrap();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .try_init()
        .ok();

    let connection = Connection::open(
        &OpenConnectionArguments::new(
            rabbit_host.as_str(),
            5672,
            rabbit_user.as_str(),
            rabbit_password.as_str(),
        )
    )
        .await
        .unwrap();

    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();

    let channel = connection.open_channel(None).await.unwrap();
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .unwrap();

    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::new("testing_queue"))
        .await
        .unwrap()
        .unwrap();

    let routing_key = "amqprs.example";
    let exchange_name = "test_exchange";
    channel
        .queue_bind(QueueBindArguments::new(
            &queue_name,
            exchange_name,
            routing_key,
        ))
        .await
        .unwrap();

    let args = BasicConsumeArguments::new(&queue_name, "basic_consumer")
        .manual_ack(true)
        .finish();

    channel
        .basic_consume(TemporalVideosConsumer::new(), args)
        .await
        .unwrap();

    let guard = Notify::new();
    guard.notified().await;
}