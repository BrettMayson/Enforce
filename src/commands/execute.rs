use std::io::Read;
use std::path::PathBuf;

use crate::Command;

pub struct Execute {}
impl Command for Execute {
    fn register(&self) -> clap::App {
        clap::SubCommand::with_name("execute")
            .version(*crate::VERSION)
            .about("Execute a file")
            .arg(clap::Arg::with_name("file").help("File to lint").required(true))
    }

    fn run(&self, args: &clap::ArgMatches) -> Result<(), String> {
        let path = PathBuf::from(args.value_of("file").unwrap());
        let mut f = std::fs::File::open(path).unwrap();
        let mut source = String::new();
        f.read_to_string(&mut source).unwrap();
        println!("=============");
        println!("{}", source);
        println!("=============");
        println!("{:#?}", crate::parser::parse(&source).unwrap());
        Ok(())
    }
}
