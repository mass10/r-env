//!
//! r-regex
//!  

use std::io::Read;

fn is_file_existing(file: &str) -> bool {
	let path = std::path::Path::new(file);
	return path.exists();
}

/// Read the whole lines from file.
fn read_from_file(file: &str) -> Result<String, Box<dyn std::error::Error>> {
	let mut file = std::fs::File::open(file)?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;
	return Ok(buffer);
}

fn parse_dotenv_string(content: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
	let mut dotenv = std::collections::HashMap::new();

	// Split into lines
	let lines = content.split("\n");
	for line in lines {
		let line = line.trim();
		if line.len() == 0 {
			continue;
		}
		if line.starts_with("#") {
			continue;
		}
		let tokens: Vec<&str> = line.split("=").collect();
		if tokens.len() != 2 {
			continue;
		}
		let key = tokens[0].trim().to_string();
		let value = tokens[1].trim().to_string();
		dotenv.insert(key, value);
	}
	return Ok(dotenv);
}

fn read_dotenv_file(file: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
	// Read whole file
	let buffer = read_from_file(file)?;

	// parse
	let map = parse_dotenv_string(&buffer)?;

	return Ok(map);
}

struct DotenvFile {
	/// Map of String
	map: std::collections::HashMap<String, String>,
}

impl DotenvFile {
	/// Configure with specified .env file.
	fn configure(use_stdin: bool, file: &Option<String>) -> Result<DotenvFile, Box<dyn std::error::Error>> {
		if use_stdin {
			// Read stdin all.
			let buffer = read_whole_lines_from_stdin()?;

			// Parse as .env.
			let string_map = parse_dotenv_string(&buffer)?;

			// Initialize object.
			let instance = Self { map: string_map };

			return Ok(instance);
		} else if file.is_none() {
			// IGNORE MISSING default .env file.
			let file = ".env";

			if !is_file_existing(file) {
				return Ok(DotenvFile {
					map: std::collections::HashMap::new(),
				});
			}

			// Read .env file.
			let string_map = read_dotenv_file(file)?;

			// Initialize object.
			let instance = Self { map: string_map };

			return Ok(instance);
		} else {
			// Read from specified file.
			let file = file.clone().unwrap();

			let string_map = read_dotenv_file(&file)?;

			// Initialize object.
			let instance = Self { map: string_map };

			return Ok(instance);
		}
	}

	pub fn get_inner_map(&self) -> &std::collections::HashMap<String, String> {
		return &self.map;
	}
}

/// Helpers for container.
trait MatchesHelper {
	fn opt_string(&self, key: &str) -> String;
}

impl MatchesHelper for getopts::Matches {
	fn opt_string(&self, key: &str) -> String {
		if !self.opt_present(key) {
			return "".to_string();
		}
		let value = self.opt_str(key).unwrap();
		return value;
	}
}

/// Retrieve the result of the regular expression.
#[allow(unused)]
fn get_regex_result(string_value: &str, expression_string: &str) -> Result<String, String> {
	let expression = regex::Regex::new(&expression_string);
	if expression.is_err() {
		eprintln!("ERROR: regex compile error. {}", expression.err().unwrap());
		return Err("".into());
	}
	let expression = expression.unwrap();

	// try to capture by "(...)".
	let capture_result = expression.captures_at(&string_value, 0);
	if capture_result.is_none() {
		eprintln!("not match.");
		return Err("".into());
	}

	// result
	let capture_result = capture_result.unwrap();
	if capture_result.len() <= 1 {
		eprintln!("not match.");
		return Err("".into());
	}

	let captured = capture_result.get(1).unwrap();
	let result = captured.as_str();
	return Ok(result.into());
}

/// Read to end from stdin.
fn read_whole_lines_from_stdin() -> Result<String, Box<dyn std::error::Error>> {
	let stdin = std::io::stdin();
	let mut handle = stdin.lock();
	let mut buffer = String::new();
	let _size_read = handle.read_to_string(&mut buffer)?;
	return Ok(buffer);
}

/// Launch a new process.
fn launch_command(commands: &Vec<String>, env: &DotenvFile) -> Result<(), Box<dyn std::error::Error>> {
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

	let mut result = command.spawn()?;
	let status = result.wait()?;
	eprintln!("Process exited with status: {}", status);

	return Ok(());
}

/// EXECUTE READ ENV
fn execute(use_stdin: bool, env_file: Option<String>, commands: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
	let result = DotenvFile::configure(use_stdin, &env_file);
	if result.is_err() {
		eprintln!("ERROR: {}", result.err().unwrap());
		return Ok(());
	}

	// Launch a new process.
	launch_command(&commands, &result.unwrap())?;

	return Ok(());
}

/// Dump variables
fn dump_variables(use_stdin: bool, path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
	let dotenv = DotenvFile::configure(use_stdin, &path)?;
	let map = dotenv.get_inner_map();
	for (key, value) in map {
		println!("{}={}", key, value);
	}
	return Ok(());
}

/// Entrypoint of a Rust application.
fn main() {
	let mut options = getopts::Options::new();
	options.optflag("", "dump", "Dump variables.");
	options.optflag("h", "help", "usage");
	options.optflag("", "use-stdin", "Use command stdin as .env");
	options.opt("", "file", ".env", "STRING", getopts::HasArg::Yes, getopts::Occur::Optional);

	// Analyzing command line arguments.
	let result = options.parse(std::env::args().skip(1));
	if result.is_err() {
		eprintln!("{}", options.usage(""));
		return;
	}
	let input = result.unwrap();

	if input.opt_present("help") {
		// SHOW HELP.
		eprintln!("{}", options.usage(""));
		return;
	} else if input.opt_present("dump") {
		// DUMP VARIABLES.
		let result = dump_variables(input.opt_present("use-stdin"), input.opt_str("file"));
		if result.is_err() {
			eprintln!("ERROR: {}", result.err().unwrap());
			return;
		}
	} else {
		// EXECUTE READ ENV AND LAUNCH COMMAND.
		let result = execute(input.opt_present("use-stdin"), input.opt_str("file"), &input.free);
		if result.is_err() {
			eprintln!("ERROR: {}", result.err().unwrap());
			return;
		}
	};
}
