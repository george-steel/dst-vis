use cgmath::{point2, Vector2, vec2};
use crate::model::*;

struct DSTStitch {
    dp: Vector2<i32>,
    jump: bool,
    stop: bool,
}


fn read_dst_byte(b: u8) -> Vector2<i32> {
    let mut dp = vec2(0,0);
    if b & 0x01 != 0 {
        dp.x += 1;
    }
    if b & 0x02 != 0 {
        dp.x -= 1
    }
    if b & 0x04 != 0 {
        dp.x += 9
    }
    if b & 0x08 != 0 {
        dp.x -= 9
    }
    if b & 0x80 != 0 {
        dp.y += 1;
    }
    if b & 0x40 != 0 {
        dp.y -= 1
    }
    if b & 0x20 != 0 {
        dp.y += 9
    }
    if b & 0x10 != 0 {
        dp.y -= 9
    }
    dp
}


impl DSTStitch {
    fn parse(bin: [u8; 3]) -> Self{
        let mut dp = vec2(0,0);
        dp += read_dst_byte(bin[0]);
        dp += read_dst_byte(bin[1]) * 3;
        dp += read_dst_byte(bin[2] & 0x3c) * 9;
        
        DSTStitch {
            dp: dp,
            jump: bin[2] & 0x80 != 0,
            stop: bin[2] & 0x40 != 0,
        }
    }
}

pub fn decode_dst(buf: &[u8], colors: &[Color]) -> Vec<EmbOp> {
    if buf.len() < 512 {
        return Vec::new();
    }
    if colors.len() == 0{
        return Vec::new();
    }

    let mut colidx = 0;
    let mut pos = vec2(0,0);
    let mut ops = Vec::new();
    let mut current_op = EmbOp::new(colors[0]);
    let mut current_block: Block = Vec::new();
    let mut njumps = 0;


    for rec in buf[512..].chunks_exact(3) {
        let stitch = DSTStitch::parse(<[u8;3]>::try_from(rec).unwrap());
        pos += stitch.dp;

        if stitch.jump {
            njumps += 1;
            if njumps == 5 && current_block.len() != 0{
                current_op.blocks.push(current_block);
                current_block = Vec::new();
            }
            if stitch.stop {
                if current_block.len() != 0{
                    current_op.blocks.push(current_block);
                    current_block = Vec::new();
                }
                if current_op.blocks.len() != 0 {
                    ops.push(current_op);
                    colidx = (colidx + 1) % colors.len();
                    current_op = EmbOp::new(colors[colidx])
                }
            }
        } else {
            njumps = 0;
            current_block.push(point2(pos.x as f32 * 0.1, pos.y as f32 * 0.1));
        }
    }

    ops
}