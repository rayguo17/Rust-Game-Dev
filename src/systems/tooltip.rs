use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Monster)]
#[read_component(Name)]
pub fn tooltip(ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] map: &Map,
    #[resource] camera: &Camera,
){
    let mut db = DrawBatch::new();
    db.target(2);
    let mut query = <(&Point, &Name)>::query().filter(component::<Monster>());
    let _ = query.iter(ecs).filter(|(&monster_pos,_)|{
        let offset = Point::new(camera.left_x, camera.top_y);
        let screen_pos = monster_pos - offset;
        let mouse_pos = *mouse_pos/4;
        println!("screen_pos: {:?}", screen_pos);
        println!("mouse_pos: {:?}", mouse_pos);
        println!("monster_pos: {:?}", monster_pos);
        screen_pos == mouse_pos
    }).map(|(monster_pos,name)| {
        let offset = Point::new(camera.left_x, camera.top_y);
        let screen_pos = *monster_pos - offset;

        db.print_color_centered_at(screen_pos*4,
            format!("{}",name.0)
            , ColorPair::new(WHITE, RED));
    }).collect::<Vec<_>>();
    
    db.submit(10000).expect("Draw tooltip error");
}