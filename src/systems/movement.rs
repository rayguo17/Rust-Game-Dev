use crate::prelude::*;


#[system(for_each)]
#[read_component(Player)]
pub fn movement(
    entity: &Entity,
    next_move: &MoveIntent,
    #[resource] m: &Map,
    #[resource] cam: &mut Camera,
    ecs: &mut SubWorld,
    command_buffer: &mut CommandBuffer
){
    if m.can_enter_tile(next_move.destination){
        command_buffer.add_component(next_move.entity,next_move.destination); // if already exist, will be override.
        if ecs.entry_ref(next_move.entity).unwrap().get_component::<Player>().is_ok(){
            cam.on_player_move(next_move.destination);
        }
    }
    command_buffer.remove(*entity);
}