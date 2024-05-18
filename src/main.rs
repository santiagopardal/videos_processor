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
use consumer::temporal_videos_consumer::TemporalVideosConsumer;
use camera::camera::Camera;
use crate::node::node_pool::NodePool;

mod consumer;
mod api;
mod node;
mod camera;


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let rabbit_host = env::var("RABBIT_HOST").unwrap();
    let rabbit_user = env::var("RABBIT_USER").unwrap();
    let rabbit_password = env::var("RABBIT_PASSWORD").unwrap();
    let exchange_name = env::var("EXCHANGE_NAME").unwrap();

    let cameras: Vec<Camera> = camera::api::get_all_cameras().await.unwrap();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

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

    let mut queue = QueueDeclareArguments::new("testing_queue");
    queue.durable(true);

    let (queue_name, _, _) = channel
        .queue_declare(queue)
        .await
        .unwrap()
        .unwrap();

    for camera in cameras {
        println!("Binding camera '{}' with id: {}", camera.name, camera.id);
        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name,
                exchange_name.as_str(),
                &camera.id.to_string(),
            ))
            .await
            .unwrap();
    }

    let args = BasicConsumeArguments::new(&queue_name, "basic_consumer")
        .manual_ack(true)
        .finish();

    let node_pool = NodePool::new();
    let consumer = TemporalVideosConsumer::new(node_pool);

    channel
        .basic_consume(consumer, args)
        .await
        .unwrap();

    let guard = Notify::new();
    guard.notified().await;
}
