extern crate machine;

use clap::Parser;
use machine::cli::{compile_to_file, run_from_binary_file, run_from_source, Args, Commands};

fn main() {
    // Parse the command line arguments
    let args = Args::parse();

    // If no command is specified, print a help message and exit
    if args.command.is_none() {
        println!("No command specified. Use --help to see the list of commands.");
        return;
    }

    // Execute the specified command
    let result = match args.command.unwrap() {
        // If the command is "Compile", compile the source file to the output file
        Commands::Compile { src, out } => compile_to_file(&src, &out),
        // If the command is "Run", run the program from the source or binary file
        Commands::Run {
            path,
            from_source,
            debug,
        } => {
            if from_source {
                // If the "from_source" flag is set, run the program from the source file
                run_from_source(&path, debug)
            } else {
                // Otherwise, run the program from the binary file
                run_from_binary_file(&path, debug)
            }
        }
    };

    // If the command resulted in an error, print the error
    if let Err(error) = result {
        println!("Command line error: {:?}", error);
    }
}
