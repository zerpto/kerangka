use besi::{CronJobs, Job};

/// Marker type naming this app's scheduled jobs: `besi::CronCommand<AppJobs>`.
#[derive(Debug)]
pub struct AppJobs;

impl CronJobs for AppJobs {
    fn jobs() -> Vec<Job> {
        vec![]
    }
}
