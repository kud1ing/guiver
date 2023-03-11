use crate::widget_manager::command::Command;
use crate::WidgetError;

pub mod command;
pub mod id_provider;

pub trait WidgetManager<APP_EVENT> {
    ///
    fn handle_command(&mut self, command: Command<APP_EVENT>) -> Result<(), WidgetError> {
        self.handle_commands(vec![command])
    }

    ///
    fn handle_commands(&mut self, commands: Vec<Command<APP_EVENT>>) -> Result<(), WidgetError>;
}
