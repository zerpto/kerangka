use besi::{Command, Context, Result};
use clap::Args;

use crate::config::HelloConfig;

#[derive(Debug, Args)]
pub struct HelloCommand {
    /// Who to greet
    #[arg(default_value = "world")]
    name: String,
}

impl Command for HelloCommand {
    async fn run(&self, ctx: &mut Context<'_>) -> Result<()> {
        let config: HelloConfig = ctx.config().section()?;
        writeln!(ctx.out(), "{}, {}!", config.greeting, self.name)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use besi::Config;

    use super::*;

    async fn run(name: &str) -> String {
        let mut buf = Vec::new();
        let config = Config::default();
        let cmd = HelloCommand {
            name: name.to_string(),
        };
        cmd.run(&mut Context::new(&mut buf, &config)).await.unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[tokio::test]
    async fn greets_by_name() {
        assert_eq!(run("Akung").await, "Hello, Akung!\n");
    }
}
