use kernel::{drivers::Queue, Error};
use std::{sync::Arc, time::Duration};
use stdx::futures::stream::StreamExt;
use stdx::log::{error, info};
use tokio::{sync::mpsc, time::delay_for};
use worker::Worker;

mod worker;

pub async fn run(
    kernel_service: Arc<kernel::Service>,
    analytics_service: Arc<analytics::Service>,
    inbox_service: Arc<inbox::Service>,
    queue: Arc<dyn Queue>,
) -> Result<(), Error> {
    let ten_ms = Duration::from_millis(10);
    let one_hundred_ms = Duration::from_millis(100);

    let config = kernel_service.config();
    let concurrency = config.worker.concurrency;
    let worker = Worker::new(kernel_service, analytics_service, inbox_service);
    let (mut tx, rx) = mpsc::channel(concurrency);
    let queue_tx = queue.clone();

    info!("worker.run: Starting worker. concurrency={}", concurrency);

    tokio::spawn(async move {
        loop {
            let jobs = match queue_tx.pull(50).await {
                Ok(jobs) => jobs,
                Err(err) => {
                    error!("worker.run: pulling jobs: {}", err);
                    delay_for(one_hundred_ms).await;
                    Vec::new()
                }
            };

            for job in jobs {
                let job_id = job.id.clone();
                match tx.send(job).await {
                    Ok(_) => {}
                    Err(err) => {
                        error!("worker.run: sending job: {}", err);
                        let _ = queue_tx.fail_job(job_id).await; // TODO: handle error?
                    }
                }
            }
            delay_for(ten_ms).await;
        }
        // drop(tx);
    });

    rx.for_each_concurrent(concurrency, |job| async {
        let job_id = job.id.clone();

        // TODO: handle error?
        let _ = match worker.handle_job(job).await {
            Ok(_) => queue.delete_job(job_id).await,
            Err(err) => {
                error!("worker.run: handling job: {}", &err);
                queue.fail_job(job_id).await
            }
        };
    })
    .await;

    Ok(())
}
