use chrono::{Local, Utc};
use tokio_schedule::{every, Job};

pub async fn start_scheduler() {
    let every_second = every(1)
        .seconds()
        .in_timezone(&Utc)
        .perform(|| async { println!("schedule_task event - {:?}", Local::now()) });
    every_second.await;
}
