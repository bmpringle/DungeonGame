use crate::tile;

pub struct DrawQue{
    que_array :Vec<tile::Tile>
}

impl DrawQue{
    pub fn new() -> DrawQue{
        let q_a = Vec::new();
        let draw_que = DrawQue{
        que_array: q_a
        };
        return draw_que;
    }
}


