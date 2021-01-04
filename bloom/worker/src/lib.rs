use futures::stream::StreamExt;
use kernel::{domain::messages::Message, drivers::queue::Job, Error};
use tokio::sync::mpsc;
use worker::Worker;

mod worker;

pub async fn run(concurrency: usize) -> Result<(), Error> {
    let (mut tx, rx) = mpsc::channel(concurrency);

    let worker = Worker::new();

    tokio::spawn(async move {
        loop {
            tx.send(Job {
                id: "TODO".to_string(),
                message: Message::KenrnelSendRegisterEmail {
                    email: "TODO".to_string(),
                    username: "TODO".to_string(),
                    code: "TODO".to_string(),
                },
            })
            .await
            .unwrap();
        }
        // drop(tx);
    });

    rx.for_each_concurrent(concurrency, |job| async {
        let _ = worker.handle_job(job).await;
    })
    .await;

    Ok(())
}
