use flarecord::models::InteractionContextType;
use flarecord::models::command::SubcommandType;
use flarecord::prelude::*;
use flarecord::command;

pub mod setup;
pub mod teardown;
pub mod enable;
pub mod disable;
pub mod show;


#[command]
impl Command for Ext {
    fn name(&self) -> String {
        "ext".into()
    }

    fn description(&self) -> String {
        "Set of commands to manage extensions".into()
    }

    fn interaction_contexts(&self) -> Vec<InteractionContextType> {
        vec![InteractionContextType::Guild]
    }

    fn subcommands(&self) -> Vec<SubcommandType> {
        vec![
            Arc::new(setup::SetupCommand)
        ]
    }
}