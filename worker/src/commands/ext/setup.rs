use flarecord::{error::{Error, BotResult}, models::{command::{CommandOptions, Subcommand, context::CommandContext, interaction::CommandInteraction, option::CommandOption, response::CommandResponse}}};


pub struct SetupCommand;

impl Subcommand for SetupCommand {
    fn name(&self) -> String {
        "setup".into()
    }

    fn description(&self) -> String {
        "Setup an extension on this server!".into()
    }

    fn options(&self) -> BotResult<CommandOptions> {
        let extension = CommandOption::integer("extension", "The extension to setup")?;
        Ok(Some(vec![extension]))
    }

    async fn execute(
        &self,
        interaction: CommandInteraction,
        ctx: CommandContext
    ) -> BotResult<CommandResponse> {
        interaction.defer(true).await;


        let response = CommandResponse::empty();

        interaction.edit(response).await
    }
}