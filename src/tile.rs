

pub struct Tile {
    x_pos :u32,
    y_pos :u32
}

pub trait Displaytile {
    fn display(&self);
}

impl Tile {
    pub fn new(x_posin :u32, y_posin :u32) -> Tile{
        let tile = Tile{
            x_pos: x_posin,
            y_pos: y_posin
        };
        
        return tile;
    }

    pub fn display(&self){
        println!("hi!");
    }
}

impl Displaytile for Tile {
    fn display(&self){
        print!("hi!");
    }
}


