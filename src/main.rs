use polygon::geo::generate_tree_from_polygons;
use polygon::parse_from_file;

use std::env::args;
use std::path::Path;
use std::process::ExitCode;

fn main() -> Result<(), ExitCode>
{
    if args().len() != 2 {
        println!("USAGE: {} <input file>", args().next().unwrap());

        return Err(ExitCode::FAILURE);
    }

    let path = args().nth(1).unwrap();
    let path = Path::new(path.as_str());
    let nodes = parse_from_file(path);
    let _root = generate_tree_from_polygons(&nodes);

    for node in nodes {
        print!("{} ", node.parent().unwrap().value().0);
    }

    println!();

    Ok(())
}
