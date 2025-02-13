use crate::prelude::*;


#[system]
pub fn map_render(
    #[resource] m: &Map,
    #[resource] camera: &mut Camera
) {
    let mut  db = DrawBatch::new();
    db.target(0);
    for y in camera.top_y..camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            if m.in_bounds(Point::new(x, y)){
                let offset = Point::new(x-camera.left_x, y-camera.top_y);
                let idx = map_idx(x, y);
                
                match m.tiles[idx] {
                    TileType::Floor=>{
                        // db = 
                        db.set(offset, ColorPair::new (WHITE, BLACK), to_cp437('.'));
                    },
                    TileType::Wall=>{
                        // db = 
                        db.set(offset,ColorPair::new  (WHITE, BLACK),to_cp437('#'));
                    }
                }
            }
        
        }
    }
    db.submit(0).expect("submit error")
}

#[system]
pub fn fake_render(){
    let mut  db = DrawBatch::new();
    db.target(0);
    for x in 0..DISPLAY_WIDTH{
        for y in 0..DISPLAY_HEIGHT{
            db.set(Point::new(x, y), ColorPair::new(WHITE, BLACK), to_cp437('#'));        
        }
    }
    db.submit(0).expect("Fake render fail");
}