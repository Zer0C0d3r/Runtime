use colored::*;
use runtime::{OutputFormat, Runtime};
use std::thread;
use std::time::Duration;

mod cli;

fn main() {
    let args = cli::parse_args();

    // Show minimal loading animation only for interactive mode
    if args.format == OutputFormat::Interactive {
        show_fast_loading();
    }

    // Create runtime and collect metrics
    let mut runtime = Runtime::new(args);
    runtime.refresh();

    // Print the result
    println!("{}", runtime);
}

fn show_fast_loading() {
    let frames = ["|", "/", "-", "\\"];

    print!("{}", "Loading system metrics".bright_cyan().bold());

    // Much faster animation - only 200ms total
    for _ in 0..8 {
        for frame in &frames {
            print!(
                "\r{} {}",
                "Loading system metrics".bright_cyan().bold(),
                frame.bright_yellow().bold()
            );
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            thread::sleep(Duration::from_millis(25)); // Much faster!
        }
    }

    println!(
        "\r{} {}",
        "Loading system metrics".bright_cyan().bold(),
        "Done!".bright_green().bold()
    );

    thread::sleep(Duration::from_millis(50)); // Minimal delay
}
