use polygon::cmd::{generate, process};

use polygon::geo::SVG;
use polygon::geo::Unit;

use polygon::parse_from_file;

use clap::{Parser, Subcommand};

use std::path::Path;

/**************/
/* STRUCTURES */
/**************/

#[derive(Debug, Subcommand)]
enum Command
{
    #[command(about = "Generate a `.poly` file")]
    Generate {
        #[arg(
            long,
            short,
            help = "The maximum number of corners for each polygon"
        )]

        corner_count: usize,

        #[arg(long, short, help = "The frame width and height")]
        dimension: Unit,

        #[arg(
            long,
            short,
            help = "Half of the maximum distance between two corners"
        )]

        radius: Unit,

        #[arg(long, short, help = "The polygon count")]
        polygon_count: usize
    },

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

/*************/
/* FUNCTIONS */
/*************/

fn main()
{
    let args = Args::parse();

    match args.command {
        Command::Generate {
            corner_count,
            dimension,
            radius,
            polygon_count
        } => {
            let polygons = generate(
                corner_count,
                dimension,
                polygon_count,
                radius
            );

            for (index, polygon) in polygons.into_iter().enumerate() {
                for point in polygon.points() {
                    println!("{} {} {}", index, point.x, point.y);
                }
            }
        }

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
            let _root = process(&nodes);

            for node in nodes {
                print!("{} ", node.parent().unwrap().value().0);
            }
        }
    }

    println!();
}
