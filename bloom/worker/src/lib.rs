use std::{sync::Arc, time::Duration};

use futures::stream::StreamExt;
use kernel::{drivers::Queue, Error};
use tokio::{sync::mpsc, time::delay_for};
use worker::Worker;

mod worker;

pub async fn run(queue: Arc<dyn Queue>, concurrency: usize) -> Result<(), Error> {
    let ten_ms = Duration::from_millis(10);
    let one_hundred_ms = Duration::from_millis(100);

    let worker = Worker::new();
    let (mut tx, rx) = mpsc::channel(concurrency);
    let queue_tx = queue.clone();

    tokio::spawn(async move {
        loop {
            let jobs = match queue_tx.pull(50).await {
                Ok(jobs) => jobs,
                Err(err) => {
                    println!("worker.run: pulling jobs: {}", err);
                    delay_for(one_hundred_ms).await;
                    Vec::new()
                }
            };

            for job in jobs {
                match tx.send(job).await {
                    Ok(_) => {}
                    Err(err) => println!("worker.run: sending job: {}", err),
                }
            }
            delay_for(ten_ms).await;
        }
        // drop(tx);
    });

    rx.for_each_concurrent(concurrency, |job| async {
        let job_id = job.id.clone();
        let _ = match worker.handle_job(job).await {
            Ok(_) => queue.delete_job(job_id).await,
            Err(err) => {
                println!("worker.run: handling job: {}", &err);
                queue.fail_job(job_id).await
            }
        };
    })
    .await;

    Ok(())
}
