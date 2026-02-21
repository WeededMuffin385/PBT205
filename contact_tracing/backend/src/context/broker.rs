use lapin::{Channel, Connection, ConnectionProperties, ExchangeKind};
use lapin::options::ExchangeDeclareOptions;
use lapin::types::FieldTable;
use tracing::info;

pub const POSITION_EXCHANGE: &str = "position";

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

        channel.exchange_declare(
            POSITION_EXCHANGE.into(),
            ExchangeKind::Topic,
            ExchangeDeclareOptions{
                durable: true,
                .. Default::default()
            },
            FieldTable::default(),
        ).await.unwrap();

        Self {
            conn,
            channel,
        }
    }
}