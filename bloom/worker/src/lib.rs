use futures::stream::StreamExt;
use kernel::Error;
use reqwest::Client;
use std::time::Instant;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct Job {
    pub event: String,
    pub payload: String,
}

pub async fn run(concurrency: usize) -> Result<(), Error> {
    let (mut tx, rx) = mpsc::channel(concurrency);
    let client = Client::new();

    let start = Instant::now();

    let concurrency = 20;

    tokio::spawn(async move {
        for i in 0..200 {
            println!("sending job: {}", i);
            tx.send(Job {
                event: "https://kerkour.fr".into(),
                payload: i.to_string(),
            })
            .await
            .unwrap();
        }
        drop(tx);
    });

    // let consumer_task = tokio::spawn(async move {
    //     rx.for_each_concurrent(concurrency, |job| handle_job_ref(&client, job))
    //         .await;
    // });
    // consumer_task.await.unwrap();

    rx.for_each_concurrent(concurrency, |job| handle_job_ref(&client, job))
        .await;

    // let consumer_task = tokio::spawn(async move {
    //     let client = Arc::new(client);
    //     let mut consumer_stream = rx
    //         .map(|job| tokio::spawn(handle_job_arc(Arc::clone(&client), job)))
    //         .buffer_unordered(concurrency);
    //     while let Some(_) = consumer_stream.next().await {}
    // });

    // let client = Arc::new(client);
    //     let mut consumer_stream = rx
    //         .map(|job| tokio::spawn(handle_job_arc(Arc::clone(&client), job)))
    //         .buffer_unordered(concurrency);
    //     while let Some(_) = consumer_stream.next().await {}

    // consumer_stream
    // .for_each(|task| async {
    //     match task {
    //         Ok(task) => println!("completed request {:?}", &task.payload),
    //                 Err(err) => eprintln!("Got an error: {}", err),
    //     }
    // })
    // .await;
    // while let Some(task) = consumer_stream.next().await {
    //     match task {
    //         Ok(task) => println!("completed request {:?}", &task.unwrap().payload),
    //         Err(err) => eprintln!("Error: {}", err),
    //     }
    //     // println!("completed request {:?}", &task.payload);
    // }

    // producer_task.await.unwrap();

    let duration = start.elapsed();
    println!("Time elapsed in worker() is: {:?}", duration);

    // producer_task.await.unwrap();
    Ok(())
}

async fn handle_job_ref(client: &Client, job: Job) {
    println!("received job({:?})", &job.payload);
    let resp = client.get(&job.event).send().await.unwrap();
    let headers = resp.headers();
    let x_cache_header = headers.get("x-cache");
    println!("completed job({:?}): {:?}", &job.payload, x_cache_header);
}

// async fn handle_job_arc(client: Arc<Client>, job: Job) {
//     println!("received job({:?})", &job.payload);
//     let resp = client.get(&job.event).send().await.unwrap();
//     let headers = resp.headers();
//     let x_cache_header = headers.get("x-cache");
//     println!("completed job({:?}): {:?}", &job.payload, x_cache_header);
// }
