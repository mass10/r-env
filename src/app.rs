//!
//! Application
//!

use crate::env;

/// Launch a new process.
///
/// # Arguments
/// * `env` - Environment variables.
/// * `commands` - Command line arguments.
fn launch_command(env: &env::DotenvFile, commands: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
	if commands.len() == 0 {
		eprintln!("ERROR: command is empty.");
		return Ok(());
	}

	let (command_str, args) = commands.split_first().unwrap();

	let mut command = std::process::Command::new(&command_str);
	command.args(args);

	for (k, v) in &env.map {
		command.env(k, v);
	}

	let _result = command.spawn()?.wait()?;

	return Ok(());
}

pub struct Application;

impl Application {
	/// Execute command.
	///
	/// # Arguments
	/// * `use_stdin` - Whether to use stdin as .env.
	/// * `env_file` - File path to parse. (DEFAULT: .env)
	pub fn execute(&self, use_stdin: bool, env_file: Option<String>, commands: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
		// Try to configure.
		let dotenv = env::DotenvFile::configure(use_stdin, env_file)?;

		// Launch a new process.
		launch_command(&dotenv, &commands)?;

		return Ok(());
	}

	/// Dump variables
	pub fn dump_variables(&self, use_stdin: bool, env_file: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
		// Try to configure.
		let dotenv = env::DotenvFile::configure(use_stdin, env_file)?;

		// internal map.
		let map = dotenv.get_inner_map();

		// dump.
		for (key, value) in map {
			println!("{}={}", key, value);
		}

		return Ok(());
	}
}
