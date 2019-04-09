mod tile;
mod DrawQue;

fn main() {
let tile = tile::tile::Tile::new(3, 4);
let drawQue = DrawQue::DrawQue::DrawQue::new();
    tile.display();
}
