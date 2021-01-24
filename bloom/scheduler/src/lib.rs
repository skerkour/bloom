use kernel::{Error, Service};
use std::sync::Arc;
use stdx::{
    job_scheduler::{self, scheduler::TryToScheduler},
    log::{error, info},
};

pub async fn run(kernel_service: Arc<Service>) -> Result<(), Error> {
    let scheduler = job_scheduler::new_utc_executor();

    // every day at 04:00
    let job = job_scheduler::job::Job::new("kernel", "dispatch_delete_old_data", Some(3), move || {
        let kernel_service_inner = kernel_service.clone();
        Box::pin(async move {
            kernel_service_inner.dispatch_delete_old_data().await?;
            Ok(())
        })
    });
    scheduler
        .add_job_with_scheduler(
            "* 0 4 * * *"
                .to_scheduler()
                .expect("scheduler.run: parsing kernel.delete_old_data cron expression"),
            job,
        )
        .await;

    info!("scheduler.run: Starting scheduler.");
    let join_handle_res = match scheduler.run().await {
        Ok(join_handle) => join_handle.await,
        Err(err) => {
            error!("scheduler.run: starting scheduler: {}", err);
            return Err(Error::Internal(err.to_string()));
        }
    };

    match join_handle_res {
        Ok(()) => {}
        Err(err) => {
            error!("scheduler.run: running scheduler: {}", err);
            return Err(Error::Internal(err.to_string()));
        }
    }
    info!("scheduler.run: scheduler stopped.");

    Ok(())
}
