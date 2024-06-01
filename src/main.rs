use std::env;
use amqprs::{
    callbacks::{ DefaultChannelCallback, DefaultConnectionCallback },
    channel::{ Channel, BasicConsumeArguments, QueueBindArguments, QueueDeclareArguments },
    connection::{ Connection, OpenConnectionArguments }
};
use tokio::{ sync::Notify, fs };
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing_subscriber::{fmt, prelude::*};
use crate::consumer::temporal_videos_consumer::TemporalVideosConsumer;
use crate::camera::camera::Camera;
use crate::node::node::Node;
use crate::node::node_pool::NodePool;

mod consumer;
mod api;
mod node;
mod camera;


async fn login_or_register() -> Node {
    let node: Node;

    if !fs::try_exists("node.json").await.unwrap() {
        node = node::api::register().await.unwrap();

        let mut file = fs::File::create("node.json").await.unwrap();

        let node_as_string = serde_json::to_string(&node).unwrap();
        let bytes: Vec<u8> = node_as_string.bytes().collect();

        file.write(&bytes).await.unwrap();
    } else {
        let mut node_file = fs::File::open("node.json").await.unwrap();
        let mut node_data: String = String::new();
        node_file.read_to_string(&mut node_data).await.unwrap();

        node = serde_json::from_str(&node_data).unwrap();
    }

    return node;
}


#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let node: Node = login_or_register().await;

    let rabbit_host = env::var("RABBIT_HOST").unwrap();
    let rabbit_user = env::var("RABBIT_USER").unwrap();
    let rabbit_password = env::var("RABBIT_PASSWORD").unwrap();
    let exchange_name = env::var("EXCHANGE_NAME").unwrap();

    let cameras: Vec<Camera> = camera::api::get_all_cameras_in_node(&node.id).await.unwrap();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .try_init()
        .ok();

    let connection = Connection::open(
        &OpenConnectionArguments::new(&rabbit_host, 5672, &rabbit_user, &rabbit_password)
    ).await.unwrap();

    connection.register_callback(DefaultConnectionCallback).await.unwrap();

    let channel: Channel = connection.open_channel(None).await.unwrap();
    channel.register_callback(DefaultChannelCallback).await.unwrap();

    let mut queue: QueueDeclareArguments = QueueDeclareArguments::new("testing_queue");
    queue.durable(true);

    let (queue_name, _, _) = channel.queue_declare(queue).await.unwrap().unwrap();

    for camera in cameras {
        println!("Binding camera '{}' with id: {}", camera.name, camera.id);
        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name,
                &exchange_name,
                &camera.id.to_string(),
            ))
            .await
            .unwrap();
    }

    let args: BasicConsumeArguments =
        BasicConsumeArguments::new(&queue_name, "basic_consumer")
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
