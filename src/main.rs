mod tile;
mod DrawQue;

fn main() {
    let tile = tile::Tile::new(3, 4);
    let drawQue = DrawQue::DrawQue::new();
    tile.display();
}
