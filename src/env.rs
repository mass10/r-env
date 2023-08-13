//!
//! Controls environment variables
//!

use std::io::Read;

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

/// .env data structure (simple string map)
pub struct DotenvFile {
	/// Map of String
	pub map: std::collections::HashMap<String, String>,
}

impl DotenvFile {
	/// Configure with specified .env file.
	pub fn configure(use_stdin: bool, file: Option<String>) -> Result<DotenvFile, Box<dyn std::error::Error>> {
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
