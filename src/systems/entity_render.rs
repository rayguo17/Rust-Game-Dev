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
    db.target(1); // select the console layer 1.
    <(&Point,&Render)>::query().iter(ecs).for_each(|(pos,render)|{
        // only render those can be seen in camera

        let offset = Point::new(camera.left_x, camera.top_y);
        let camera_pos = *pos-offset;
        if camera_pos.x>0 && camera_pos.y>0{
            db.set(camera_pos, render.color, render.glyph);
        }
       
    });

    db.submit(5000).expect("Draw entity error");
}