use polygon::geo::generate_tree_from_polygons;
use polygon::parse_from_file;

use std::env::args;
use std::path::Path;

fn main()
{
    let path = args().nth(1).unwrap();
    let path = Path::new(path.as_str());
    let polygons = parse_from_file(path);
    let tree = generate_tree_from_polygons(polygons);

    for parent in tree.bfs_iter().map(|node| node.parent()) {
        match parent {
            Some(parent) => print!("{} ", parent.value().0),
            None => ()
        }
    }

    println!();
}
