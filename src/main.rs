mod cmd;
mod config;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::config::Config;

const SUBCOMMAND_REGISTER: &str = "register";

fn main() {
    let args = build_argparser().get_matches();
    let log = init_logging(&args);

    let cfg = Config::from_args(&args);

    if let Some(cmd) = args.subcommand_matches(SUBCOMMAND_REGISTER) {
        cmd::register(log, &cfg, &cmd);
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
        .subcommands(vec![SubCommand::with_name(SUBCOMMAND_REGISTER)
            .about("Register a new user")
            .args(&[Arg::with_name("user").required(true)])])
}

fn init_logging(args: &ArgMatches) -> slog::Logger {
    use slog::Drain;

    let log_level = if args.is_present("debug") {
        slog::Level::Debug
    } else if args.is_present("verbose") {
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
