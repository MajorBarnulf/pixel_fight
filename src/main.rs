use std::fs;

use clap::{Parser, Subcommand};
use ron::ser::PrettyConfig;
use speedy2d::Window;

mod sim;
pub use sim::{Simulation, Team};

mod view;
pub use view::View;

mod builder;
pub use builder::{build, example_descriptor};

mod app;
pub use app::App;

#[derive(Parser, Debug)]
/// Simple program for simulating pixel fights
pub struct Params {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Dumps an example fight configuration to stdout.
    Example,

    /// Runs a fight from a configuration file.
    Run {
        /// Path to an example fight configuration file.
        path: String,
    },
}

fn main() {
    let args: Params = Params::parse();
    match args.command {
        Command::Example => {
            let example = example_descriptor();
            let serialized = ron::ser::to_string_pretty(&example, PrettyConfig::default()).unwrap();
            println!("{serialized}");
        }

        Command::Run { path } => {
            let content = fs::read_to_string(path).unwrap();
            let deserialized = ron::from_str(&content).unwrap();

            let sim = build(deserialized);
            let window = Window::new_centered("Pixel fight /rs", (800, 600)).unwrap();
            window.run_loop(App::new(sim));
        }
    }
}
