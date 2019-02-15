use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

fn main() {
    let args = build_argparser().get_matches();
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
                .long("config")
                .help("Config file to use")
                .takes_value(true),
        ])
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
