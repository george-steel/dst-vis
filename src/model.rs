use cgmath::Point2;
use crate::util::RGBA32float;

pub type Point = Point2<f32>;
pub type Block = Vec<Point>;

#[derive(Clone,Debug)]
pub struct EmbOp {
    pub color: RGBA32float,
    pub blocks: Vec<Block>,
}

impl EmbOp {
    pub fn new(col: RGBA32float) -> Self {
        EmbOp {
            color: col,
            blocks: Vec::new(),
        }
    }
}
