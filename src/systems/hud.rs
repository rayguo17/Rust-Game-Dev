use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
pub fn hud(
    ecs: &SubWorld
){
    let mut db = DrawBatch::new();
    db.target(2);
    let mut query = <&Health>::query().filter(component::<Player>());
    let health = query.iter(ecs).nth(0).unwrap();
    
    db.print_centered(1, "Explore the dungeon, use cursors and key to move the player".to_string());
    db.bar_horizontal(Point::zero(), SCREEN_WIDTH*2, health.current, health.max, ColorPair::new(RED, RED));
    db.print_color_centered(0,
        format!("Health: {}/{}",health.current,health.max), ColorPair::new(WHITE, RED));
    db.submit(100000).expect("batch submit error!");
}