#![feature(crate_visibility_modifier)]
#![feature(proc_macro, generators)]

extern crate catslapp;

extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate linefeed;
extern crate serde_json;
extern crate structopt;
extern crate tempdir;
extern crate tokio_core;

use futures::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Cmd {
    AddMedium,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "repl")]
    Repl,
    #[structopt(name = "cmd")]
    Cmd(Cmd),
    #[structopt(name = "server")]
    Server,
}

#[derive(Fail, Debug)]
enum ReplError {
    #[fail(display = "{}", msg)]
    ArgumentsError { msg: &'static str },
}

fn start_repl() -> Result<(), failure::Error> {
    unimplemented!()
    // let mut reader = linefeed::Reader::new("catslapp");
    // reader.set_prompt("catslapp > ");
    // let mut core = tokio_core::reactor::Core::new().unwrap();
    // let handle = core.handle();

    // while let linefeed::ReadResult::Input(line) = reader.read_line().unwrap() {
    //     let line = line.trim();
    //     let command = read_command(line);

    //     match command {
    //         Some(("AddMedium", _)) => {
    //             unimplemented!();
    //             // let command = catslapp::add_medium_proto();
    //             // println!("{}", serde_json::to_string_pretty(&command)?);
    //         }
    //         Some(("add-medium", args)) => {
    //             if args.len() != 1 {
    //                 Err(ReplError::ArgumentsError {
    //                     msg: "add-medium takes one argument",
    //                 })?;
    //             }

    //             unimplemented!();

    //             // let uri = args[0];

    //             // println!(
    //             //     "{}",
    //             //     core.run(catslapp::add_medium(uri.parse()?, handle.clone()))?
    //             // );
    //         }
    //         _ => (),
    //     }
    // }

    // Ok(())
}

fn read_command(line: &str) -> Option<(&str, Vec<&str>)> {
    let mut words = line.split_whitespace();
    let command = words.next();
    command.map(|command| (command, words.collect()))
}

fn main() -> Result<(), failure::Error> {
    use std::io::prelude::*;
    env_logger::init();
    info!("SLAPPP!!!");
    match Command::from_args() {
        Command::Server => catslapp::start_server(),
        Command::Repl => start_repl(),
        Command::Cmd(cmd) => match cmd {
            Cmd::AddMedium => {
                let tmp_dir = tempdir::TempDir::new("catslapp")?;
                let tmp_file_path = tmp_dir.path().join("command.json");
                let mut tmp_file = ::std::fs::File::create(&tmp_file_path)?;
                unimplemented!();
                // let proto = catslapp::add_medium_proto();
                // serde_json::to_writer_pretty(&mut tmp_file, &proto)?;
                // tmp_file.sync_all()?;
                // let outcome = ::std::process::Command::new("code")
                //     .arg("--wait")
                //     .arg(tmp_file_path)
                //     .output()?;
                // if outcome.status.success() {
                //     let reader = ::std::fs::File::open(&tmp_file_path)?;
                //     let msg: ::catslapp::app::catsl::media::AddMedium =
                //         serde_json::from_reader(&reader)?;
                //     unimplemented!();
                //     Ok(())
                // } else {
                //     Ok(())
                // }
            }
        },
    }
}
