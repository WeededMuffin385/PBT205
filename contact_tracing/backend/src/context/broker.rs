use lapin::options::ExchangeDeclareOptions;
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ConnectionProperties, ExchangeKind};
use tracing::info;
use crate::common::{POSITION_EXCHANGE, QUERY_REQUEST_EXCHANGE, QUERY_RESPONSE_EXCHANGE};

pub struct Broker {
    pub conn: Connection,
    pub channel: Channel,
}

impl Broker {
    pub async fn new() -> Self {
        let conn = Connection::connect(
            "amqp://admin:secret@rabbitmq:5672",
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

        channel.exchange_declare(
            QUERY_REQUEST_EXCHANGE.into(),
            ExchangeKind::Topic,
            ExchangeDeclareOptions{
                durable: true,
                .. Default::default()
            },
            FieldTable::default(),
        ).await.unwrap();

        channel.exchange_declare(
            QUERY_RESPONSE_EXCHANGE.into(),
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