use tokio::sync::mpsc::Receiver;

use amqprs::{
    channel::{BasicPublishArguments, Channel},
    BasicProperties,
};
use anyhow::Result;

use crate::events::{self, EventEmitter};

#[derive(Clone, serde::Deserialize, Debug)]
pub enum PublishStrategy {
    Once,
    Limited { quantity: i32, speed: i32 },
    Infinite { speed: i32 },
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

pub async fn publish_by_strategy(
    strategy: PublishStrategy,
    target: &str,
    channel: &Channel,
    body: Vec<u8>,
    routing_key: &str,
    emitter: impl EventEmitter,
    cancel: &mut Receiver<bool>,
) -> Result<()> {
    println!("choosing strat: {:?}", strategy);
    match strategy {
        PublishStrategy::Once => {
            let _ = publish_once(target, channel, body, routing_key, emitter).await;
        }
        PublishStrategy::Limited { quantity, speed: _ } => {
            let _ = publish_loop(
                |i| i < quantity,
                target,
                channel,
                body,
                routing_key,
                emitter,
                cancel,
            )
            .await;
        }
        PublishStrategy::Infinite { speed: _ } => {
            let _ = publish_loop(
                |_| true,
                target,
                channel,
                body,
                routing_key,
                emitter,
                cancel,
            )
            .await;
        }
    };

    Ok(())
}

async fn publish_once(
    target: &str,
    channel: &Channel,
    body: Vec<u8>,
    routing_key: &str,
    emitter: impl EventEmitter,
) {
    println!("publishing once");
    match publish_message(target, channel, body, routing_key).await {
        Ok(_) => events::emit_publish_end_event(emitter, events::PublishEnd(Ok(1))),
        Err(e) => events::emit_publish_end_event(
            emitter,
            events::PublishEnd(Err(String::from(e.to_string()))),
        ),
    }
}

async fn publish_loop<F>(
    done_callback: F,
    target: &str,
    channel: &Channel,
    body: Vec<u8>,
    routing_key: &str,
    emitter: impl EventEmitter,
    cancel: &mut Receiver<bool>,
) where
    F: Fn(i32) -> bool,
{
    let mut i = 0;
    while done_callback(i) {
        if let Ok(_) = cancel.try_recv() {
            println!("got a signal to finish publishing");
            events::emit_publish_end_event(
                emitter,
                events::PublishEnd(Err(String::from("publishing cancelled before finishing"))),
            );
            return;
        }

        if let Err(e) = publish_message(target, channel, body.clone(), routing_key).await {
            events::emit_publish_end_event(
                emitter,
                events::PublishEnd(Err(String::from(e.to_string()))),
            );
            return;
        }

        i += 1;
        println!("published {:?} messages so far", i);
    }

    events::emit_publish_end_event(emitter, events::PublishEnd(Ok(i)));
}
