use polygon::cmd::generate;

#[cfg(feature = "dac")] use polygon::cmd::process_dac;
#[cfg(feature = "naive")] use polygon::cmd::process_naive;
#[cfg(feature = "frames")] use polygon::cmd::process_frames;

use polygon::geo::SVG;
use polygon::geo::Unit;

use polygon::parse_from_file;

use clap::{Parser, Subcommand, ValueEnum};

use std::path::Path;

/**************/
/* STRUCTURES */
/**************/

#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
#[derive(Debug, Default, ValueEnum)]
enum Algorithm
{
    #[cfg(feature = "dac")]
    DAC,

    #[cfg(feature = "frames")]
    #[default]
    Frames,

    #[cfg(feature = "naive")]
    Naive
}

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
        #[arg(long, short, help = "The algorithm used")]
        algorithm: Algorithm,

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

        Command::Process { algorithm, path } => {
            let nodes = parse_from_file(Path::new(path.as_str()));

            let _root =
                match algorithm {
                    #[cfg(feature = "dac")]
                    Algorithm::DAC => process_dac(&nodes),

                    #[cfg(feature = "frames")]
                    Algorithm::Frames => process_frames(&nodes),

                    #[cfg(feature = "naive")]
                    Algorithm::Naive => process_naive(&nodes)
                };

            for node in nodes {
                print!("{} ", node.parent().unwrap().value().0);
            }
        }
    }

    println!();
}
