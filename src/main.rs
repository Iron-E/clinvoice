#![allow(clippy::suspicious_else_formatting)]

mod app;
mod dyn_result;
mod input;

use
{
	std::{error::Error, fs, process},

	app::App,
	dyn_result::DynResult,

	clinvoice_config::Config,

	structopt::StructOpt,
};

/// # Summary
///
/// Exit `clinvoice` with status code 1, printing some `error`.
fn exit_with_err(error: impl Error) -> !
{
	if cfg!(debug_assertions) { panic!("{:?}", error) }

	eprintln!("\nERROR: {}", error);
	process::exit(1)
}

/// # Summary
///
/// The main method.
fn main()
{
	// Create a default user configuration if not already present.
	Config::init().unwrap_or_else(|e| exit_with_err(e));

	// Get the user configuration.
	let config_bytes = fs::read(Config::path()).unwrap_or_else(|e| exit_with_err(e));
	let config: Config = toml::from_slice(&config_bytes).unwrap_or_else(|e| exit_with_err(e));

	// Run the CLInvoice application.
	App::from_args().run(&config).unwrap_or_else(|e| exit_with_err(e.as_ref()));
}
