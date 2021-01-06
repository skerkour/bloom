use thiserror::Error;

#[derive(Error, Debug)]
pub enum SchedulerError {
    #[error("ScheduleDefinitionError: [{message}]")]
    ScheduleDefinitionError { message: String },
    #[error("JobLockError: [{message}]")]
    JobLockError { message: String },
    #[error("JobExecutionStateError: [{message}]")]
    JobExecutionStateError { message: String },
    #[error("JobExecutionError: [{cause}]")]
    JobExecutionError {
        cause: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("JobExecutionPanic: [{cause}]")]
    JobExecutionPanic { cause: String },
}
