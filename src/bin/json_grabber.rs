use anyhow::Result;
use clap::Parser;
use colored::*;
use json_grabber::{
    config::{Config, Operation},
    json_grabber::Projector,
    opts::Opts,
};
fn main() -> Result<()> {
    println!(
        "{}",
        "
       __                       ______              __     __               
      ╱ ╱_____ ____   ____     ╱ ____╱_____ ______ ╱ ╱_   / ╱_   ____  _____
 __  ╱ ╱╱ ___╱╱ __ ╲ ╱ __ ╲   ╱ ╱ __ ╱ ___╱╱ __ `╱╱ __ ╲ ╱ __ ╲ ╱ __/ / ___╱
╱ ╱_╱ ╱(__  )╱ ╱_╱ ╱╱ ╱ ╱ ╱  ╱ ╱_╱ ╱╱ ╱   ╱ ╱_╱ ╱╱ ╱_╱ ╱╱ ╱_╱ ╱╱  __╱╱ ╱    
╲____╱╱____╱ ╲____╱╱_╱ ╱_╱   ╲____╱╱_╱    ╲____╱╱_____╱╱_____╱ ╲___╱╱_╱
"
        .bright_cyan()
    );
    println!(
        "{} {} {}",
        "Json grabber".bright_green().bold(),
        "v0.1.0".bright_yellow(),
        "Remember important variables from your terminal 🦀\n".bright_white(),
    );
    let config: Config = Opts::parse().try_into()?;

    let mut proj = Projector::from_config(config.config, config.pwd);

    match config.operation {
        Operation::Print(None) => {
            let value = proj.get_value_all();
            let value = serde_json::to_string(&value)?;
            println!("{}", value);
        }
        Operation::Print(Some(k)) => {
            proj.get_value(&k).map(|x| {
                println!("{}", x);
            });
        }
        Operation::Add(k, v) => {
            proj.set_value(k, v);
            proj.save()?;
        }
        Operation::Remove(k) => {
            proj.remove_value(&k);
            proj.save()?;
        }
    }

    return Ok(());
}
