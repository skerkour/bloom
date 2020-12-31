use kernel::Error;

pub fn run() -> Result<(), Error> {
    // This is the number of concurrent tasks per worker thread.
    let concurrency = 500;
    println!("running worker with concurrency = {}", concurrency);

    let mut runtime = tokio::runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(worker::run(concurrency))
}
