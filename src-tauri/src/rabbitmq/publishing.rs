use amqprs::{
    channel::{BasicPublishArguments, Channel},
    BasicProperties,
};
use anyhow::Result;
use tokio::time::{sleep, Duration, Instant};

use crate::{
    async_jobs::CancelToken,
    events::{Event::ProgressUpdate, Event::PublishEnd, EventEmitter, ProgressUpdateData},
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
        PublishStrategy::Limited { quantity, speed } => {
            let _ = publish_loop(
                |i| i < quantity,
                speed,
                target,
                channel,
                body,
                routing_key,
                emitter,
                cancel,
            )
            .await;
        }
        PublishStrategy::Infinite { speed } => {
            let _ = publish_loop(
                |_| true,
                speed,
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
    speed: i32,
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
    let mut published_this_sec = 0;
    let mut total_published = 0;

    let mut now = Instant::now();
    while done_callback(published_this_sec) {
        if cancel.should_cancel() {
            println!("got a signal to finish publishing");
            emitter.emit(PublishEnd(Err(String::from(
                "publishing cancelled before finishing",
            ))));
            return;
        }

        let (should_publish, actual_speed) =
            enforce_speed(speed, &mut now, &mut published_this_sec).await;
        if !should_publish {
            emitter.emit(ProgressUpdate(ProgressUpdateData {
                published_count: total_published,
                expected_speed: speed,
                actual_speed: actual_speed,
            }));
            continue;
        }

        if let Err(e) = publish_message(target, channel, body.clone(), routing_key).await {
            emitter.emit(PublishEnd(Err(e.to_string())));
            return;
        }

        published_this_sec += 1;
        total_published += 1;
        println!("published {:?} messages so far", total_published);
    }

    emitter.emit(PublishEnd(Ok(published_this_sec)));
}

async fn enforce_speed(
    publish_per_sec: i32,
    last_seconds_start: &mut Instant,
    published_this_sec: &mut i32,
) -> (bool, i32) {
    let second_passed = last_seconds_start.elapsed().as_millis() >= 1000;

    if *published_this_sec >= publish_per_sec && !second_passed {
        let acutal_speed = published_this_sec.clone();
        sleep(Duration::from_millis(10)).await;
        return (false, acutal_speed);
    }
    if second_passed {
        let acutal_speed = published_this_sec.clone();
        *last_seconds_start = Instant::now();
        *published_this_sec = 0;
        return (false, acutal_speed);
    }

    return (true, -1);
}

#[test]
fn test_enforece_speed() {
    let speed = 100;

    let mut now = Instant::now();
    let mut pub_this_sec = 0;
    loop {
        let (should_publish, acutal_speed) =
            tokio_test::block_on(enforce_speed(speed, &mut now, &mut pub_this_sec));

        if !should_publish {
            println!("actual speed: {:?}", acutal_speed);
            continue;
        }

        std::thread::sleep(Duration::from_millis(50));
        println!("publish");
        pub_this_sec += 1;
        println!(
            "published {:?} messages in {:?} ms",
            pub_this_sec,
            now.elapsed().as_millis()
        );
    }
}
