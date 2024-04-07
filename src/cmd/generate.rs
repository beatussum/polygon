use super::super::geo::polygon::Any;
use super::super::geo::{Point, Unit};
use super::super::geo::{Intersecter, Polygon};

use rand::Rng;
use rand::thread_rng;

use std::f64::consts::PI;

/*************/
/* FUNCTIONS */
/*************/

pub fn generate(
    corner_count: usize,
    dimension: Unit,
    polygon_count: usize,
    radius: Unit
) -> Vec<Any>
{
    let mut ret = Vec::new();
    let mut rng = thread_rng();

    ret.reserve(polygon_count);

    for i in 0..polygon_count {
        let mut polygon;

        'polygons : loop {
            let corner_count = rng.gen_range(3..=corner_count);
            let radius = rng.gen_range((1.)..=radius);

            let center = {
                let x = rng.gen_range(radius..=(dimension - radius));
                let y = rng.gen_range(radius..=(dimension - radius));

                Point { x , y }
            };

            polygon = generate_polygon(center, corner_count, radius);

            if i == 0 {
                break;
            } else {
                for j in &ret[..i] {
                    if polygon.intersects(j) {
                        continue 'polygons;
                    }
                }

                break 'polygons;
            }
        }

        ret.push(polygon);
    }

    ret
}

fn generate_polygon(center: Point, corner_count: usize, radius: Unit) -> Any
{
    let mut polygon = Any::default();
    let mut rng = thread_rng();

    polygon.points.reserve(corner_count);

    loop {
        let mut angles =
            (0..corner_count)
                .map(|_| rng.gen_range((0.)..(2. * PI)))
                .collect::<Vec<_>>();

        angles.sort_by(Unit::total_cmp);

        for a in angles.iter() {
            let point = {
                let distance_to_center = rng.gen_range((0.)..=radius);
                let x = center.x + distance_to_center * a.cos();
                let y = center.y + distance_to_center * a.sin();

                Point { x, y }
            };

            polygon.points.push(point);
        }

        if polygon.is_valid() {
            break;
        } else {
            polygon.points.clear();
        }
    }

    return polygon;
}
