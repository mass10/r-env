//!
//! r-env: CLI utility for .env
//!  

mod app;
mod env;

/// Initialize commandline options parser.
///
/// # Returns
/// parser.
fn create_options() -> getopts::Options {
	let mut options = getopts::Options::new();
	options.optflag("", "dump", "Dump variables.");
	options.optflag("h", "help", "usage");
	options.optflag("", "use-stdin", "Use command stdin as .env");
	options.opt("", "file", ".env", "STRING", getopts::HasArg::Yes, getopts::Occur::Optional);
	return options;
}

/// Entrypoint of a Rust application.
fn main() {
	// Create options parser.
	let options = create_options();

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
		let app = app::Application;
		let result = app.dump_variables(use_stdin, option_file);
		if result.is_err() {
			eprintln!("ERROR: {}", result.err().unwrap());
			std::process::exit(1);
		}
	} else {
		// ========== EXECUTE READ ENV AND PASS TO NEXT COMMAND ==========
		let app = app::Application;
		let result = app.execute(use_stdin, option_file, &input.free);
		if result.is_err() {
			eprintln!("ERROR: {}", result.err().unwrap());
			std::process::exit(1);
		}
	};
}
