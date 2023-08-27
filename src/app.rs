//!
//! Application
//!

use crate::dotenv;

/// Launch a new process.
///
/// # Arguments
/// * `env` - Environment variables.
/// * `commands` - Command line arguments.
fn launch_command(env: Box<dyn dotenv::DotenvFile>, commands: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
	if commands.len() == 0 {
		return Err("ERROR: command is empty.".into());
	}

	let (command_str, args) = commands.split_first().unwrap();

	let mut command = std::process::Command::new(&command_str);
	command.args(args);

	let map = env.as_map();

	for (k, v) in map {
		command.env(k, v);
	}

	let result = command.spawn()?.wait()?;
	if !result.success() {
		let exit_code = result.code().unwrap();
		return Ok(exit_code);
	}

	return Ok(0);
}

/// Application trait.
pub trait Application {
	/// Execute command.
	///
	/// # Arguments
	/// * `use_stdin` - Whether to use stdin as .env.
	/// * `env_file` - File path to parse. (DEFAULT: .env)
	/// * `commands` - Command and arguments.
	fn execute_command(&self, use_stdin: bool, env_file: Option<String>, commands: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>>;

	/// Dump variables
	///
	/// # Arguments
	/// * `use_stdin` - Whether to use stdin as .env.
	/// * `env_file` - File path to parse. (DEFAULT: .env)
	fn dump_variables(&self, use_stdin: bool, env_file: Option<String>) -> Result<(), Box<dyn std::error::Error>>;
}

/// Application implementation.
struct ApplicationImpl;

impl Application for ApplicationImpl {
	/// Execute command.
	fn execute_command(&self, use_stdin: bool, env_file: Option<String>, commands: &Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
		// Try to configure.
		let dotenv = dotenv::configure(use_stdin, env_file)?;

		// Launch a new process.
		return launch_command(dotenv, &commands);
	}

	/// Dump variables
	fn dump_variables(&self, use_stdin: bool, env_file: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
		// Try to configure.
		let dotenv = dotenv::configure(use_stdin, env_file)?;

		// internal map.
		let map = dotenv.as_map();

		// dump.
		for (key, value) in map {
			println!("{}={}", key, value);
		}

		return Ok(());
	}
}

/// Create a new instance of Application.
pub fn new() -> Box<dyn Application> {
	return Box::new(ApplicationImpl {});
}
