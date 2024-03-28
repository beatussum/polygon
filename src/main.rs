use polygon::geo::SVG;
use polygon::geo::generate_tree_from_polygons;

use polygon::parse_from_file;

use clap::Parser;

use std::path::Path;

#[derive(Debug, Parser)]
#[command(about, version)]
struct Args
{
    #[arg(help = "The path of the input file")]
    path: String,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Print the polygons in SVG format"
    )]
    show: bool
}

fn main()
{
    let args = Args::parse();

    let path = Path::new(args.path.as_str());
    let nodes = parse_from_file(path);

    if args.show {
        println!("<svg>");

        for node in nodes {
            println!("\t{}", node.value().1.to_svg());
        }

        println!("</svg>");
    } else {
        let _root = generate_tree_from_polygons(&nodes);

        for node in nodes {
            print!("{} ", node.parent().unwrap().value().0);
        }
    }

    println!();
}
