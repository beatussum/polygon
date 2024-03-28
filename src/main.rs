use polygon::geo::SVG;
use polygon::geo::generate_tree_from_polygons;

use polygon::parse_from_file;

use std::env::args;
use std::path::Path;
use std::process::ExitCode;

fn main() -> Result<(), ExitCode>
{
    if args().len() > 3 {
        println!("USAGE: {} [-s] <input file>", args().nth(0).unwrap());

        return Err(ExitCode::FAILURE);
    }

    let second = args().nth(1).unwrap();
    let show = second == "-s";
    let path = if show { args().nth(2).unwrap() } else { second };
    let path = Path::new(path.as_str());
    let nodes = parse_from_file(path);

    if show {
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

    Ok(())
}
