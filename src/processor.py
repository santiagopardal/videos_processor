from pika import BlockingConnection, ConnectionParameters, PlainCredentials
import os

# TODO fetch the cameras I am going to handle
CAMERAS_TO_HANDLE = [1, 2, 3, 4]


def main():
    credentials = PlainCredentials(username=os.environ.get('RABBIT_USER'), password=os.environ.get('RABBIT_PASSWORD'))
    connection = BlockingConnection(ConnectionParameters(host=os.environ.get('RABBIT_HOST'), credentials=credentials))
    channel = connection.channel()
    channel.queue_declare(queue='videos_to_merge')

    for camera_id in CAMERAS_TO_HANDLE:
        channel.queue_bind(exchange='camerai', queue='videos_to_merge', routing_key=f"{camera_id}")

    def callback(ch, method, properties, body):
        print(f" [x] Received {body}")

    channel.basic_consume(queue='videos_to_merge', on_message_callback=callback, auto_ack=True)

    print(' [*] Waiting for messages. To exit press CTRL+C')
    channel.start_consuming()


if __name__ == '__main__':
    main()
