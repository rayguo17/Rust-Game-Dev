
use crate::prelude::*;

mod player_input;
mod map_render;
mod entity_render;
mod collisions;
mod random_move;
mod end_turn;
mod movement;
mod hud;
mod tooltip;


pub fn build_player_move_scheduler()->Schedule{
    Schedule::builder()
    .add_system(movement::movement_system())
    .flush()
    .add_system(collisions::collisions_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(end_turn::end_turn_system())
    .build()
}

pub fn build_player_input_scheduler()->Schedule{
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(hud::hud_system())
    .add_system(tooltip::tooltip_system())
    .build()
}
//How to organize the order of different systems?

pub fn build_monster_move_scheduler()->Schedule{
    Schedule::builder()
    .add_system(random_move::random_move_system())
    .flush()//some effect system can run before movement.
    .add_system(movement::movement_system())
    .flush()
    .add_system(collisions::collisions_system())
    .flush()
    .add_system(map_render::map_render_system())
    .add_system(entity_render::entity_render_system())
    .add_system(end_turn::end_turn_system())
    .build()
}