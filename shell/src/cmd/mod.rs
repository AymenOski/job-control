pub mod rm;
pub mod mkdir;
pub mod mv;
pub mod cp;
pub mod ls;
pub mod cat;
pub mod help;


pub trait Command {
    fn execute(&self, args: Vec<String>) -> Result<(), crate::error::ShellError>;
}