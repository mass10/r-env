//!
//! r-env: CLI utility for .env
//!  

mod app;
mod env;

/// Shows usage.
fn usage() {
	let pkg_name = env!("CARGO_PKG_NAME");
	eprintln!("Usage: {} [OPTIONS] [COMMAND]", pkg_name);
	eprintln!();
	eprintln!("Options:");
	eprintln!("        --dump              Dump variables.");
	eprintln!("    -h, --help              usage");
	eprintln!("        --use-stdin         Use command stdin as .env");
	eprintln!("        --file STRING       .env");
}

/// Configuration class
#[derive(Debug)]
struct Configuration {
	/// Command to execute.
	command: Vec<String>,

	/// Dump variables.
	exec_dump: bool,

	/// .env file path.
	file: Option<String>,

	/// Show help.
	show_help: bool,

	/// Use stdin as .env
	use_stdin: bool,
}

impl Configuration {
	/// Create a new instance.
	pub fn new(args: &Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
		let mut conf = Self {
			command: vec![],
			exec_dump: false,
			file: None,
			show_help: false,
			use_stdin: false,
		};

		// ========== PARSE ARGUMENTS ==========
		let mut current_section = "";

		for arg in args {
			if 0 < conf.command.len() {
				conf.command.push(arg.to_string());
			} else if arg == "--" {
				// end of section
				current_section = "";
				conf.command.push(arg.to_string());
			} else if arg == "--dump" {
				conf.exec_dump = true;
				current_section = "";
			} else if arg == "--file" {
				conf.file = Some("".to_string());
				current_section = "--file";
			} else if arg == "--help" {
				conf.show_help = true;
				current_section = "";
			} else if arg == "-h" {
				conf.show_help = true;
				current_section = "";
			} else if arg == "--use-stdin" {
				conf.use_stdin = true;
				current_section = ""
			} else if arg.starts_with("-") {
				let err = format!("Unknown option {}.", arg);
				return Err(err.into());
			} else if current_section == "--file" {
				conf.file = Some(arg.to_string());
				current_section = "";
			} else {
				conf.command.push(arg.to_string());
			}
		}

		// ========== VALIDATE ARGUMENTS ==========

		if conf.file.is_some() {
			let file = conf.file.as_ref().unwrap();
			if file == "" {
				let error = "--file option requires a file path.";
				return Err(error.into());
			}
		}

		return Ok(conf);
	}
}

/// Entrypoint of a Rust application.
fn main() {
	// ========== PARSE ARGUMENTS ==========
	let args: Vec<String> = std::env::args().skip(1).collect();
	let result = Configuration::new(&args);
	if result.is_err() {
		let pkg_name = env!("CARGO_PKG_NAME");
		eprintln!("{}: {}", pkg_name, result.err().unwrap());
		eprintln!();
		usage();
		std::process::exit(1);
	}
	let input = result.unwrap();

	if input.show_help {
		// ========== SHOW HELP ==========
		usage();
	} else if input.exec_dump {
		// ========== DUMP VARIABLES ==========
		let app = app::Application;
		let result = app.dump_variables(input.use_stdin, input.file);
		if result.is_err() {
			let pkg_name = env!("CARGO_PKG_NAME");
			eprintln!("{}: {}", pkg_name, result.err().unwrap());
			eprintln!();
			usage();
			std::process::exit(1);
		}
	} else {
		// ========== EXECUTE READ ENV AND PASS TO NEXT COMMAND ==========
		let app = app::Application;
		let result = app.execute(input.use_stdin, input.file, &input.command);
		if result.is_err() {
			eprintln!("ERROR: {}", result.err().unwrap());
			std::process::exit(1);
		}
		let exit_code = result.unwrap();
		if exit_code != 0 {
			std::process::exit(exit_code);
		}
	};
}
