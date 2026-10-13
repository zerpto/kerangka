

use besi::{Command, Context, Result};
use clap::Subcommand;

use super::commands::{hello};
use crate::api::AppRoutes;
use crate::cron::AppJobs;


#[derive(Debug, Subcommand)]
pub enum Commands {
    Hello(hello::HelloCommand),
    Api(besi::ApiCommand<AppRoutes>),
    Cron(besi::CronCommand<AppJobs>),
    Migrate(besi::MigrateCommand<migration::Migrator>)
} 

impl Command for Commands {
    async fn run(&self, ctx: &mut Context<'_>) -> Result<()> {
        match self {
            Self::Hello(cmd) => cmd.run(ctx).await,
            Self::Api(cmd) => cmd.run(ctx).await,
            Self::Cron(cmd) => cmd.run(ctx).await,
            Self::Migrate(cmd) => cmd.run(ctx).await,
        }
    }
}
