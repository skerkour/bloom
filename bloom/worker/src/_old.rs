
// async fn handle_job_ref(client: &Client, job: Job) {
//     info!("received job({:?})", &job.payload);
//     let resp = client.get(&job.event).send().await.unwrap();
//     let headers = resp.headers();
//     let x_cache_header = headers.get("x-cache");
//     info!("completed job({:?}): {:?}", &job.payload, x_cache_header);
// }

// async fn handle_job_arc(client: Arc<Client>, job: Job) {
//     info!("received job({:?})", &job.payload);
//     let resp = client.get(&job.event).send().await.unwrap();
//     let headers = resp.headers();
//     let x_cache_header = headers.get("x-cache");
//     info!("completed job({:?}): {:?}", &job.payload, x_cache_header);
// }


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
//         Ok(task) => info!("completed request {:?}", &task.payload),
//                 Err(err) => einfo!("Got an error: {}", err),
//     }
// })
// .await;
// while let Some(task) = consumer_stream.next().await {
//     match task {
//         Ok(task) => info!("completed request {:?}", &task.unwrap().payload),
//         Err(err) => einfo!("Error: {}", err),
//     }
//     // info!("completed request {:?}", &task.payload);
// }

// producer_task.await.unwrap();
// producer_task.await.unwrap();


// let consumer_task = tokio::spawn(async move {
//     rx.for_each_concurrent(concurrency, |job| handle_job_ref(&client, job))
//         .await;
// });
// consumer_task.await.unwrap();
