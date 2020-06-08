mod execute;
pub use execute::Execute;

pub trait Command {
    // (name, description)
    fn register(&self) -> clap::App;
    fn run(&self, _args: &clap::ArgMatches) -> Result<(), String> {
        unimplemented!();
    }
}
