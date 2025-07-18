// SPDX-License-Identifier: MIT

mod link;

#[cfg(test)]
mod tests;

use std::io::IsTerminal;

use iproute_rs::{CliColor, CliError, OutputFormat, print_result_and_exit};

use self::link::LinkCommand;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), CliError> {
    let mut app = clap::Command::new("iproute-rs")
        .version(clap::crate_version!())
        .author("Gris Ge <fge@redhat.com>")
        .about("Command line of rust-netlink")
        .arg(
            clap::Arg::new("VERSION")
                .long("Version")
                .help("Print Version")
                .action(clap::ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            clap::Arg::new("JSON")
                .short('j')
                .help("JSON output")
                .action(clap::ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            clap::Arg::new("COLOR")
                .short('c')
                .help("Colorful output")
                .action(clap::ArgAction::Set)
                .value_parser(["always", "auto", "never"])
                .default_value("auto")
                .global(true),
        )
        .arg(
            clap::Arg::new("YAML")
                .short('y')
                .help("YAML output")
                .action(clap::ArgAction::SetTrue)
                .global(true),
        )
        .subcommand_required(true)
        .subcommand(LinkCommand::gen_command());

    let matches = app.get_matches_mut();

    let fmt = if matches.get_flag("JSON") {
        OutputFormat::Json
    } else if matches.get_flag("YAML") {
        OutputFormat::Yaml
    } else {
        OutputFormat::default()
    };

    if let Some(color_str) = matches.get_one::<String>("COLOR")
        && (color_str == "always"
            || (color_str == "auto" && std::io::stdout().is_terminal()))
    {
        CliColor::enable();
    }

    if matches.get_flag("VERSION") {
        print_result_and_exit(Ok(app.render_version().to_string()), fmt);
    } else if let Some(matches) = matches.subcommand_matches(LinkCommand::CMD) {
        print_result_and_exit(LinkCommand::handle(matches).await, fmt);
    }

    Ok(())
}
