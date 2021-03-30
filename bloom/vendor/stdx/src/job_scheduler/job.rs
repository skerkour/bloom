use super::error::SchedulerError;
use super::scheduler::Scheduler;
use chrono::{DateTime, Utc};
use futures::future::FutureExt;
use log::*;
use std::panic::AssertUnwindSafe;
use tokio::macros::support::{Future, Pin};
use tokio::sync::{Mutex, RwLock};

pub struct JobScheduler {
    pub job: Job,
    schedule: Mutex<Scheduler>,
    next_run_at: Mutex<Option<DateTime<Utc>>>,
    last_run_at: Mutex<Option<DateTime<Utc>>>,
}

impl JobScheduler {
    pub fn new(mut schedule: Scheduler, job: Job) -> Self {
        // Determine the next time it should run
        let next_run_at = schedule.next(&Utc::now());
        JobScheduler {
            job,
            schedule: Mutex::new(schedule),
            next_run_at: Mutex::new(next_run_at),
            last_run_at: Mutex::new(None),
        }
    }

    /// Returns true if this job is pending execution.
    pub async fn is_pending(&self) -> bool {
        // Check if paused
        if !self.job.is_active {
            return false;
        }

        // Check if NOW is on or after next_run_at
        if let Some(next_run_at) = self.next_run_at.lock().await.as_ref() {
            *next_run_at < Utc::now()
        } else {
            false
        }
    }

    /// Run the job immediately and re-schedule it.
    pub async fn run(&self) -> Result<(), SchedulerError> {
        // Execute the job function
        let run_result = self.job.run().await;

        let now = Utc::now();

        let mut schedule = self.schedule.lock().await;

        // Determine the next time it should run
        let mut next_run_at = self.next_run_at.lock().await;
        *next_run_at = schedule.next(&now);

        // Save the last time this ran
        let mut last_run_at = self.last_run_at.lock().await;
        *last_run_at = Some(now);

        run_result
    }
}

pub type JobFn = dyn 'static
    + Send
    + Fn() -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>>;

pub struct Job {
    function: Mutex<Box<JobFn>>,
    group: String,
    name: String,
    is_active: bool,
    is_running: RwLock<bool>,
    retries_after_failure: Option<u64>,
}

impl Job {
    pub fn new<
        G: Into<String>,
        N: Into<String>,
        F: 'static
            + Send
            + Fn() -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>>,
    >(
        group: G,
        name: N,
        retries_after_failure: Option<u64>,
        function: F,
    ) -> Self {
        Job {
            function: Mutex::new(Box::new(function)),
            name: name.into(),
            group: group.into(),
            retries_after_failure,
            is_running: RwLock::new(false),
            is_active: true,
        }
    }

    /// Returns true if this job is currently running.
    pub async fn is_running(&self) -> bool {
        let read = self.is_running.read().await;
        *read
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn group(&self) -> &str {
        &self.group
    }

    /// Run the job immediately and re-schedule it.
    pub async fn run(&self) -> Result<(), SchedulerError> {
        self.set_running(true).await?;

        // Execute the job function
        let mut run_result = self.exec().await;

        if let Some(retries) = self.retries_after_failure {
            for attempt in 1..=retries {
                if let Err(e) = run_result {
                    warn!(
                        "Execution failed for job [{}/{}] - Retry execution, attempt {}/{}. Previous err: {}",
                        self.group, self.name, attempt, retries, e
                    );
                    run_result = self.exec().await;
                } else {
                    break;
                }
            }
        }

        self.set_running(false).await?;

        run_result.map_err(|err| SchedulerError::JobExecutionError {
            cause: err,
        })
    }

    async fn exec(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let wrapped = AssertUnwindSafe(async {
            let result = {
                let function = self.function.lock().await;
                (function)()
            };
            result.await
        })
        .catch_unwind();

        match wrapped.await {
            // Here we unwrap just the outer panic-induced `Result`, returning
            // the inner `Result`
            Ok(response) => response,
            Err(panic) => {
                error!("stdx/job_scheduler: A panic happened. Err: {:?}", panic);
                Err(SchedulerError::JobExecutionPanic {
                    cause: format!("{:?}", panic),
                }
                .into())
            }
        }
    }

    async fn set_running(&self, is_running: bool) -> Result<(), SchedulerError> {
        let mut write = self.is_running.write().await;

        if is_running.eq(&*write) {
            return Err(SchedulerError::JobLockError {
                message: format!(
                    "Wrong Job status found for job [{}/{}]. Expected: {}",
                    self.group, self.name, !is_running
                ),
            });
        }

        *write = is_running;
        Ok(())
    }
}

#[cfg(test)]
pub mod test {

    use super::*;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::mpsc::channel;

    #[tokio::test]
    async fn should_be_running() {
        let lock = Arc::new(Mutex::new(true));
        let lock_clone = lock.clone();
        let (mut tx, mut rx) = channel(10000);
        let tx_clone = tx.clone();

        let job_scheduler = Arc::new(JobScheduler::new(
            Scheduler::Interval {
                interval_duration: Duration::new(1, 0),
                execute_at_startup: false,
            },
            Job::new("g", "n", None, move || {
                let lock_clone = lock_clone.clone();
                let mut tx_clone = tx_clone.clone();
                Box::pin(async move {
                    println!("job - started");
                    tx_clone.send("").await.unwrap();
                    println!("job - Trying to get the lock");
                    let _lock = lock_clone.lock().await;
                    println!("job - lock acquired");
                    Ok(())
                })
            }),
        ));

        assert!(!job_scheduler.job.is_running().await);

        {
            let _lock = lock.lock().await;
            let job_clone = job_scheduler.clone();
            tokio::spawn(async move {
                println!("starting job");
                job_clone.run().await.unwrap();
                println!("end job execution");
                tx.send("").await.unwrap();
            });
            rx.recv().await.unwrap();
            assert!(job_scheduler.job.is_running().await);
        }

        rx.recv().await.unwrap();
        assert!(!job_scheduler.job.is_running().await);
    }

    #[tokio::test]
    async fn job_should_not_retry_run_if_ok() {
        let lock = Arc::new(Mutex::new(0));
        let lock_clone = lock.clone();

        let max_retries = 12;

        let job = Job::new("g", "n", Some(max_retries), move || {
            let lock_clone = lock_clone.clone();
            Box::pin(async move {
                println!("job - started");
                println!("job - Trying to get the lock");
                let mut lock = lock_clone.lock().await;
                let count = *lock;
                *lock = count + 1;
                println!("job - count {}", count);
                Ok(())
            })
        });

        let result = job.run().await;

        assert!(result.is_ok());

        let lock = lock.lock().await;
        let count = *lock;
        assert_eq!(1, count);
    }

    #[tokio::test]
    async fn job_should_retry_run_if_error() {
        let lock = Arc::new(Mutex::new(0));
        let lock_clone = lock.clone();

        let max_retries = 12;

        let job = Job::new("g", "n", Some(max_retries), move || {
            let lock_clone = lock_clone.clone();
            Box::pin(async move {
                println!("job - started");
                println!("job - Trying to get the lock");
                let mut lock = lock_clone.lock().await;
                let count = *lock;
                *lock = count + 1;
                println!("job - count {}", count);
                Err(SchedulerError::JobLockError {
                    message: "".to_owned(),
                })?
            })
        });

        let result = job.run().await;

        assert!(result.is_err());

        let lock = lock.lock().await;
        let count = *lock;
        assert_eq!(max_retries + 1, count);
    }

    #[tokio::test]
    async fn job_should_stop_retrying_run_if_attempt_succeed() {
        let lock = Arc::new(Mutex::new(0));
        let lock_clone = lock.clone();

        let succeed_at = 7;
        let max_retries = 12;

        let job = Job::new("g", "n", Some(max_retries), move || {
            let lock_clone = lock_clone.clone();
            Box::pin(async move {
                println!("job - started");
                println!("job - Trying to get the lock");
                let mut lock = lock_clone.lock().await;
                let count = *lock;
                *lock = count + 1;
                println!("job - count {}", count);

                if count == succeed_at {
                    Ok(())
                } else {
                    Err(SchedulerError::JobLockError {
                        message: "".to_owned(),
                    })?
                }
            })
        });

        let result = job.run().await;

        assert!(result.is_ok());

        let lock = lock.lock().await;
        let count = *lock;
        assert_eq!(succeed_at + 1, count);
    }

    #[tokio::test]
    async fn job_run_should_gracefully_catch_panics() {
        let lock = Arc::new(Mutex::new(0));
        let lock_clone = lock.clone();

        let max_retries = 12;

        let job = Job::new("g", "n", Some(max_retries), move || {
            let lock_clone = lock_clone.clone();
            Box::pin(async move {
                println!("job - started");
                println!("job - Trying to get the lock");
                let mut lock = lock_clone.lock().await;
                let count = *lock;
                *lock = count + 1;
                println!("job - count {}", count);
                panic!("Manual panic for test")
            })
        });

        let result = job.run().await;

        assert!(result.is_err());

        let lock = lock.lock().await;
        let count = *lock;
        assert_eq!(max_retries + 1, count);
    }
}
