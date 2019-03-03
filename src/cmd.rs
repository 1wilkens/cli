use clap::ArgMatches;
use rpassword::prompt_password_stdout;
use slog::Logger;

use crate::api::Api;
use crate::config::Config;

static PROMPT_PASSWORD: &str = "Password: ";
static PROMPT_RETYPE_PASSWORD: &str = "Retype password: ";

pub(crate) fn register(log: Logger, cfg: &Config, args: &ArgMatches) {
    // unwrap is safe here, because clap validates arguments beforehand
    let user = args.value_of("email").unwrap();
    println!("Please enter a password for {}", user);
    let passwd = prompt_password_stdout(PROMPT_PASSWORD).expect("Failed to read password");
    let copy = prompt_password_stdout(PROMPT_RETYPE_PASSWORD).expect("Failed to read password");
    if passwd != copy {
        println!("Passwords do not match!");
        return;
    }
    // TODO: Validate config beforehand so unwrap here is safe
    let mut api = Api::new(cfg.base_url.as_ref()).unwrap();
    api.register(user.into(), passwd);
}
