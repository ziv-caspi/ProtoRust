use amqprs::{
    channel::{BasicPublishArguments, Channel},
    BasicProperties,
};
use anyhow::Result;

use crate::{
    async_jobs::CancelToken,
    events::{self, Event::PublishEnd, EventEmitter},
};

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
    cancel: &mut CancelToken,
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
        Ok(_) => emitter.emit(PublishEnd(Ok(1))),
        Err(e) => emitter.emit(PublishEnd(Err(e.to_string()))),
    }
}

async fn publish_loop<F>(
    done_callback: F,
    target: &str,
    channel: &Channel,
    body: Vec<u8>,
    routing_key: &str,
    emitter: impl EventEmitter,
    cancel: &mut CancelToken,
) where
    F: Fn(i32) -> bool,
{
    cancel.reset();
    let mut i = 0;
    while done_callback(i) {
        if cancel.should_cancel() {
            println!("got a signal to finish publishing");
            emitter.emit(PublishEnd(Err(String::from(
                "publishing cancelled before finishing",
            ))));
            return;
        }

        if let Err(e) = publish_message(target, channel, body.clone(), routing_key).await {
            emitter.emit(PublishEnd(Err(e.to_string())));
            return;
        }

        i += 1;
        println!("published {:?} messages so far", i);
    }

    emitter.emit(PublishEnd(Ok(i)));
}
