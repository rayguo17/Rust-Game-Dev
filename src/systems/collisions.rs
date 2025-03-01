use crate::prelude::*;


#[system]
#[read_component(Point)]
#[read_component(Monster)]
#[read_component(Player)]
pub fn collisions(
    ecs:&mut SubWorld,
    commands:&mut CommandBuffer
){
    let mut player_pos = Point::zero();
    <&Point>::query().filter(component::<Player>()).iter(ecs).for_each(|pos|{
        player_pos = *pos;
    });
    let mut  entity_to_be_remove = Vec::<Entity>::new();
    <(Entity,&Point)>::query().filter(component::<Monster>()).iter(ecs).for_each(|(entity,pos)|{
        if *pos == player_pos {
            entity_to_be_remove.push(*entity);
        }
    });
    entity_to_be_remove.iter().for_each(|entity|{
        commands.remove(*entity);
    });
    
}