use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Render)]
pub fn entity_render(
    ecs:&SubWorld,
    #[resource] camera: &Camera,
){
    let mut db = DrawBatch::new();
    db.target(1);
    let mut  query = <(&Point,&Render)>::query().filter(component::<Player>());
    
    for  (pos,rend) in query.iter(ecs){
        let offset = Point::new(camera.left_x, camera.top_y);
        db.set(*pos-offset, rend.color, rend.glyph);
    }
    db.submit(5000).expect("Draw entity error");
}