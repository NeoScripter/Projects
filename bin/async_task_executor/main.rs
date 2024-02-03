use std::error::Error;
use std::fmt;
use std::time::Duration;
use tokio::time::{error::Elapsed, timeout};
use tokio::task::JoinError;

#[derive(Debug)]
enum TaskError {
    TimeoutError(Elapsed),
    ExecutionError(JoinError),
    TaskFailed(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::TimeoutError(_) => write!(f, "Task execution exceeded the timeout period."),
            TaskError::ExecutionError(_) => write!(f, "The task executor failed to start or manage the task."),
            TaskError::TaskFailed(msg) => write!(f, "Task failed: {}", msg),
        }
    }
}

impl Error for TaskError {}

impl From<Elapsed> for TaskError {
    fn from(err: Elapsed) -> Self {
        TaskError::TimeoutError(err)
    }
}

impl From<JoinError> for TaskError {
    fn from(err: JoinError) -> Self {
        TaskError::ExecutionError(err)
    }
}

async fn execute_task<F, T>(task: F, duration: Duration) -> Result<T, TaskError>
where
    F: std::future::Future<Output = Result<T, String>> + Send,
    T: Send + 'static,
{
    match timeout(duration, task).await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err_msg)) => Err(TaskError::TaskFailed(err_msg)),
        Err(timeout_err) => Err(timeout_err.into()),
    }
}

#[tokio::main]
async fn main() {
    let duration = Duration::from_secs(3);

    let task = async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        if true {
            Ok("Task completed successfully.")
        } else {
            Err("Task encountered an error.".to_string())
        }
    };

    match execute_task(task, duration).await {
        Ok(result) => println!("Success: {}", result),
        Err(e) => println!("{}", e),
    }
}
