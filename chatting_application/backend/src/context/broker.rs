use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use tracing::info;

pub struct Broker {
	pub conn: Connection,
	pub channel: Channel,
}

impl Broker {
	pub async fn new() -> Self {
		let conn = Connection::connect(
			"amqp://admin:secret@rabbit:5672",
			ConnectionProperties::default(),
		).await.unwrap();

		info!("Connected to RabbitMQ");

		let channel = conn.create_channel().await.unwrap();
/*		channel.queue_declare("channel.a".into(), QueueDeclareOptions::default(), FieldTable::default()).await.unwrap();

		let payload = b"message";
		channel.basic_publish("".into(), "channel.a".into(), BasicPublishOptions::default(), payload, BasicProperties::default()).await.unwrap();
*/
		Self {
			conn,
			channel,
		}
	}
}