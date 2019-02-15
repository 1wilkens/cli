use clap::ArgMatches;
use slog::Logger;

use crate::config::Config;

pub fn register(log: Logger, cfg: &Config, args: &ArgMatches) {
    if let Some(user) = args.value_of("user") {
        dbg!(user);
    }
}
