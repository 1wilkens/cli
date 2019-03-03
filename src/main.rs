#![warn(clippy::pedantic)]

mod api;
mod cmd;
mod config;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::config::Config;

const SUBCOMMAND_REGISTER: &str = "register";
const SUBCOMMAND_LOGIN: &str = "login";
const SUBCOMMAND_WALLET: &str = "wallet";
const SUBCOMMAND_TRANSACTION: &str = "transaction";

fn main() {
    let args = build_argparser().get_matches();
    let cfg = Config::from_args(&args);
    let log = init_logging(&cfg);

    if let Some(cmd) = args.subcommand_matches(SUBCOMMAND_REGISTER) {
        cmd::register(log, &cfg, &cmd);
    } else if let Some(cmd) = args.subcommand_matches(SUBCOMMAND_WALLET) {
        // XXX: Add subcommands to wallet
    }
}

fn build_argparser<'a, 'b>() -> App<'a, 'b> {
    App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .args(&[
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Print more verbose output")
                .takes_value(false),
            Arg::with_name("debug")
                .long("debug")
                .help("Print debug output (implies --verbose)"),
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Config file to use")
                .takes_value(true),
        ])
        .subcommands(vec![
            SubCommand::with_name(SUBCOMMAND_REGISTER)
                .about("Register a new user account")
                .args(&[Arg::with_name("email")
                    .help("Email address of the new user")
                    .required(true)]),
            SubCommand::with_name(SUBCOMMAND_LOGIN)
                .about("Login into existing user account")
                .args(&[Arg::with_name("email")
                    .help("Email address of the account to login to")
                    .required(true)]),
            SubCommand::with_name(SUBCOMMAND_WALLET).about("Manage wallets"),
            SubCommand::with_name(SUBCOMMAND_TRANSACTION).about("Manage transactions"),
        ])
}

fn init_logging(cfg: &Config) -> slog::Logger {
    use slog::Drain;

    let log_level = if cfg.debug {
        slog::Level::Debug
    } else if cfg.verbose {
        slog::Level::Info
    } else {
        slog::Level::Error
    };

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build();
    let drain = slog::LevelFilter::new(drain, log_level).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    slog::Logger::root(drain, slog::o!())
}
