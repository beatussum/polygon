mod point;
pub use point::Point;

mod vector;
pub use vector::Vector;

mod segment;
pub use segment::Segment;

const EPSILON: f32 = 1e-5;
pub type Unit = f32;
