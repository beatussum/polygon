use polygon::geo::SVG;
use polygon::geo::generate_tree_from_polygons;

use polygon::parse_from_file;

use clap::{Parser, Subcommand};

use std::path::Path;

#[derive(Debug, Subcommand)]
enum Command
{
    #[command(about = "Process the hierarchy generation")]
    Process {
        #[arg(help = "The path of the input file")]
        path: String
    },

    #[command(about = "Print the polygons in SVG format")]
    Show {
        #[arg(help = "The path of the input file")]
        path: String
    }
}

#[derive(Debug, Parser)]
#[command(about, version)]
struct Args
{
    #[clap(subcommand)]
    command: Command
}

fn main()
{
    let args = Args::parse();

    match args.command {
        Command::Show { path } => {
            let nodes = parse_from_file(Path::new(path.as_str()));

            println!("<svg>");

            for node in nodes {
                println!("\t{}", node.value().1.to_svg());
            }

            println!("</svg>");
        },

        Command::Process { path } => {
            let nodes = parse_from_file(Path::new(path.as_str()));
            let _root = generate_tree_from_polygons(&nodes);

            for node in nodes {
                print!("{} ", node.parent().unwrap().value().0);
            }
        }
    }

    println!();
}
