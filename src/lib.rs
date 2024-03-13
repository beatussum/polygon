use itertools::Itertools;

use std::fs;
use std::path::Path;

pub mod geo;
pub mod tree;

pub fn parse_from_string(str: &str) -> Vec<geo::polygon::Any>
{
    str
        .lines()
        .map(
            |x| {
                x
                    .split_ascii_whitespace()
                    .collect_tuple()
                    .unwrap()
            }
        )
        .map(
            |(p, x, y)| {
                (
                    p.parse::<u8>().unwrap(),
                    x.parse::<geo::Unit>().unwrap(),
                    y.parse::<geo::Unit>().unwrap()
                )
            }
        )
        .group_by(|(p, _, _)| *p)
        .into_iter()
        .map(
            |(_, group)| {
                group.map(|(_, x, y)| (x, y).into()).collect::<Vec<_>>()
            }
        )
        .map(|points| geo::polygon::Any { points })
        .collect()
}

pub fn parse_from_file(path: &Path) -> Vec<geo::polygon::Any>
{
    parse_from_string(&fs::read_to_string(path).unwrap())
}

#[cfg(test)]
mod tests
{
    use super::*;

    use geo::Point;
    use geo::polygon::Any;
    use indoc::indoc;

    #[test]
    fn test_parse_from_string()
    {
        let testing = indoc! {"
            0 1 1
            0 5 1
            0 5 5
            0 1 5
            1 0 0
            1 6 0
            1 6 6
            1 0 6
            2 2 2
            2 2 3
            2 3 2
            3 4 4
            3 4 3
            3 3 4
        "};

        let expected = vec! [
            Any {
                points: vec! [
                    Point { x: 1.0, y: 1.0 },
                    Point { x: 5.0, y: 1.0 },
                    Point { x: 5.0, y: 5.0 },
                    Point { x: 1.0, y: 5.0 }
                ]
            },
            Any {
                points: vec! [
                    Point { x: 0.0, y: 0.0 },
                    Point { x: 6.0, y: 0.0 },
                    Point { x: 6.0, y: 6.0 },
                    Point { x: 0.0, y: 6.0 }
                ]
            },
            Any {
                points: vec! [
                    Point { x: 2.0, y: 2.0 },
                    Point { x: 2.0, y: 3.0 },
                    Point { x: 3.0, y: 2.0 }
                ]
            },
            Any {
                points: vec! [
                    Point { x: 4.0, y: 4.0 },
                    Point { x: 4.0, y: 3.0 },
                    Point { x: 3.0, y: 4.0 }
                ]
            }
        ];

        assert_eq!(parse_from_string(testing), expected);
    }
}
