use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicPublishArguments, Channel, ExchangeDeclareArguments, ExchangeDeleteArguments,
        QueueBindArguments, QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RabbitMqParamaters {
    pub target_name: String,
    pub is_queue: bool,
    pub routing_key: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

pub struct RabbitMqConnection {
    pub connection: Connection,
    pub channel: Channel,
    pub target: String,
}

const TMP_EXCHANGE: &str = "PROTO_RUST_TMP";
const TMP_EXCHANGE_TYPE: &str = "fanout";

pub async fn create_channel(configuration: &RabbitMqParamaters) -> Result<RabbitMqConnection> {
    let connection = Connection::open(&OpenConnectionArguments::new(
        &configuration.host,
        configuration.port,
        &configuration.username,
        &configuration.password,
    ))
    .await?;

    connection
        .register_callback(DefaultConnectionCallback)
        .await?;

    // open a channel on the connection
    let channel = connection.open_channel(None).await?;
    channel.register_callback(DefaultChannelCallback).await?;

    // declare a queue
    let target: &str;
    if configuration.is_queue {
        let (queue_name, _, _) = channel
            .queue_declare(
                QueueDeclareArguments::new("MY_QUEUE")
                    .passive(true)
                    .to_owned(),
            )
            .await?
            .ok_or(anyhow!("could not declare queue"))?;

        channel
            .exchange_declare(ExchangeDeclareArguments::new(
                TMP_EXCHANGE,
                TMP_EXCHANGE_TYPE,
            ))
            .await?;

        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name,
                TMP_EXCHANGE,
                TMP_EXCHANGE_TYPE,
            ))
            .await?;
        let (queue_name, _, _) = channel
            .queue_declare(
                QueueDeclareArguments::new("MY_QUEUE")
                    .passive(true)
                    .to_owned(),
            )
            .await?
            .ok_or(anyhow!("could not declare queue"))?;

        channel
            .exchange_declare(ExchangeDeclareArguments::new(
                TMP_EXCHANGE,
                TMP_EXCHANGE_TYPE,
            ))
            .await?;

        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name,
                TMP_EXCHANGE,
                TMP_EXCHANGE_TYPE,
            ))
            .await?;
        target = TMP_EXCHANGE;
    } else {
        target = &configuration.target_name;
    }

    return Ok(RabbitMqConnection {
        connection,
        channel,
        target: String::from(target),
    });
}

pub async fn publish_message(
    target: &str,
    channel: &Channel,
    body: Vec<u8>,
    routing_key: &str,
) -> Result<()> {
    // create arguments for basic_publish
    let args = BasicPublishArguments::new(target, routing_key);

    channel
        .basic_publish(BasicProperties::default(), body, args)
        .await?;

    Ok(())
}

pub async fn close_channel(channel: Channel, connection: Connection) -> Result<()> {
    channel
        .exchange_delete(ExchangeDeleteArguments::new(TMP_EXCHANGE))
        .await?;
    channel.close().await?;
    connection.close().await?;

    return Ok(());
}

pub async fn basic_publish_once(configuration: RabbitMqParamaters, body: Vec<u8>) -> Result<()> {
    let conn = create_channel(&configuration).await?;
    publish_message(
        &conn.target,
        &conn.channel,
        body,
        &configuration.routing_key,
    )
    .await?;
    close_channel(conn.channel, conn.connection).await?;
    Ok(())
}
