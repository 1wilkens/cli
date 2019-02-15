use clap::ArgMatches;

pub struct Config();

impl Config {
    pub fn from_args(args: &ArgMatches) -> Config {
        Config()
    }
}
