use cgmath::Point2;

pub type Color = [f32; 3];
pub type Point = Point2<f32>;
pub type Block = Vec<Point>;

#[derive(Clone,Debug)]
pub struct EmbOp {
    pub color: Color,
    pub blocks: Vec<Block>,
}

impl EmbOp {
    pub fn new(col: Color) -> Self {
        EmbOp {
            color: col,
            blocks: Vec::new(),
        }
    }
}
