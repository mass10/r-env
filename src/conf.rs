/// Configuration class
#[derive(Debug)]
pub struct Configuration {
	/// Command to execute.
	pub command: Vec<String>,

	/// Dump variables.
	pub exec_dump: bool,

	/// .env file path.
	pub file: Option<String>,

	/// Show help.
	pub show_help: bool,

	/// Use stdin as .env
	pub use_stdin: bool,
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
