pub mod scan;
pub mod scan_result;

use clap::{arg, value_parser, ArgMatches, Command, ArgAction};
use log::info;

use crate::cmd;
use crate::cmd_help;

use scan::Scan;

pub fn yara_cmd() -> Command {
    cmd::command("yara")
        .about("Scan a file or directory with Yara-x")
        .long_about(cmd_help::YARA_SCAN_LONG_HELP)
        .arg(
            arg!(-p --"path" <PATH>)
                .help("Scan the specific PATH")
                .value_parser(value_parser!(String))
                .required(true),
        )
        .arg(
            arg!(-r --"rule-path" <RULE_PATH>)
                .help("Use a specific rule PATH")
                .value_parser(value_parser!(String))
                .required(true),
        )
        .arg(
            arg!(-m --"max-depth")
                .help("Maximum recursive depth")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
                .default_value("3"),
        )
}

pub fn exec(args: &ArgMatches) -> anyhow::Result<()> {
    info!("[collector][yara]");

    let scan = Scan::new(
        args.get_one::<String>("path").unwrap(),
        args.get_one::<String>("rule-path").unwrap(),
        None,
    )?
    .dir_exclude(Some(vec![
        "/proc/**".to_string(),
        "/sys/**".to_string(),
        "/system/**".to_string(),
    ]))
    .max_depth(*args.get_one::<usize>("max-depth").unwrap())
    .follow_links(false)
    .collect()?;

    for file in scan.results {
        println!("{:?}", file);
    }

    Ok(())
}
