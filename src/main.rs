//!
//! r-env: CLI utility for .env
//!  

mod app;
mod conf;
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

/// Entrypoint of a Rust application.
fn main() {
	// ========== PARSE ARGUMENTS ==========
	let args: Vec<String> = std::env::args().skip(1).collect();
	let result = conf::Configuration::new(&args);
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
