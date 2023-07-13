//!
//! r-env: CLI utility for .env
//!  

use std::io::Read;

/// Check if the specified file exists.
///
/// # Arguments
/// * `file` - File path.
///
/// # Returns
/// true if the file exists.
fn is_file_existing(file: &str) -> bool {
	let path = std::path::Path::new(file);
	return path.exists();
}

/// Read the whole lines from file.
///
/// # Arguments
/// * `file` - File path.
///
/// # Returns
/// File content.
fn read_text_file(file: &str) -> Result<String, Box<dyn std::error::Error>> {
	let mut file = std::fs::File::open(file)?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;
	return Ok(buffer);
}

/// Try to parse the content as .env format.
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

/// Try to parse the specified file as .env format.
///
/// # Arguments
/// * `path` - File path.
///
/// # Returns
/// File content as string map.
fn read_dotenv_file(path: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
	// Read the whole file
	let content = read_text_file(path)?;

	// Try to parse as .env.
	let map = parse_dotenv_string(&content)?;

	return Ok(map);
}

/// Read file if exists.
///
/// # Arguments
/// * `file` - File path.
///
/// # Returns
/// File content.
fn read_file_if_exists(file: &str) -> Result<String, Box<dyn std::error::Error>> {
	if !is_file_existing(file) {
		return Ok("".into());
	}

	let content = read_text_file(file)?;

	return Ok(content);
}

/// Try to parse stdin as .env format.
///
/// # Returns
/// File content as string map.
fn read_dotenv_file_from_stdin() -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
	// Read stdin all.
	let content = read_whole_lines_from_stdin()?;

	// Try to parse as .env.
	let map = parse_dotenv_string(&content)?;

	return Ok(map);
}

/// Read .env file if exists.
///
/// # Arguments
/// * `path` - File path.
///
/// # Returns
/// File content as string map.
fn read_dotenv_file_if_exists(path: &str) -> Result<std::collections::HashMap<String, String>, Box<dyn std::error::Error>> {
	// Read the whole file (if exists).
	let content = read_file_if_exists(path)?;

	// Try to parse as .env.
	let map = parse_dotenv_string(&content)?;

	return Ok(map);
}

/// .env data structure (simple string map)
struct DotenvFile {
	/// Map of String
	map: std::collections::HashMap<String, String>,
}

impl DotenvFile {
	/// Configure with specified .env file.
	fn configure(use_stdin: bool, file: Option<String>) -> Result<DotenvFile, Box<dyn std::error::Error>> {
		if use_stdin {
			// Try to parse stdin as .env.
			let vars = read_dotenv_file_from_stdin()?;

			// Initialize object.
			let instance = Self { map: vars };

			return Ok(instance);
		} else if file.is_none() {
			// Try to read default file. (IGNORE MISSING)
			let vars = read_dotenv_file_if_exists(".env")?;

			// Initialize object.
			let instance = Self { map: vars };

			return Ok(instance);
		} else {
			// Try to parse specified file as .env.
			let file = file.unwrap();
			let vars = read_dotenv_file(&file)?;

			// Initialize object.
			let instance = Self { map: vars };

			return Ok(instance);
		}
	}

	/// Get reference to the internal map.
	pub fn get_inner_map(&self) -> &std::collections::HashMap<String, String> {
		return &self.map;
	}
}

/// Read to end from stdin.
///
/// # Returns
/// Read content.
fn read_whole_lines_from_stdin() -> Result<String, Box<dyn std::error::Error>> {
	let stdin = std::io::stdin();
	let mut handle = stdin.lock();
	let mut buffer = String::new();
	let _size_read = handle.read_to_string(&mut buffer)?;
	return Ok(buffer);
}

/// Launch a new process.
///
/// # Arguments
/// * `commands` - Command line arguments.
/// * `env` - Environment variables.
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

	let _result = command.spawn()?.wait()?;

	return Ok(());
}

/// Execute command.
fn execute(use_stdin: bool, env_file: Option<String>, commands: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
	// Try to configure.
	let result = DotenvFile::configure(use_stdin, env_file)?;

	// Launch a new process.
	launch_command(&commands, &result)?;

	return Ok(());
}

/// Dump variables
fn dump_variables(use_stdin: bool, path: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
	// Try to configure.
	let dotenv = DotenvFile::configure(use_stdin, path)?;

	// internal map.
	let map = dotenv.get_inner_map();

	// dump.
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
		// exit with 1
		std::process::exit(1);
	}
	let input = result.unwrap();

	// Whether to use stdin as .env.
	let use_stdin = input.opt_present("use-stdin");

	// File path to parse. (DEFAULT: .env)
	let option_file = input.opt_str("file");

	if input.opt_present("help") {
		// ========== SHOW HELP ==========
		eprintln!("{}", options.usage(""));
	} else if input.opt_present("dump") {
		// ========== DUMP VARIABLES ==========
		let result = dump_variables(use_stdin, option_file);
		if result.is_err() {
			eprintln!("ERROR: {}", result.err().unwrap());
			std::process::exit(1);
		}
	} else {
		// ========== EXECUTE READ ENV AND PASS TO NEXT COMMAND ==========
		let result = execute(use_stdin, option_file, &input.free);
		if result.is_err() {
			eprintln!("ERROR: {}", result.err().unwrap());
			std::process::exit(1);
		}
	};
}
