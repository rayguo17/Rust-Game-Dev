mod systems;
mod spawner;
mod components;
mod map_builder;
mod map;
mod camera;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}
use std::io::{self, Write};

use prelude::*;

struct State {
    ecs: World,
    resources: Resources, // Could we allow <T> Cell and Ref Cell is for lock access. Correct?
    player_move_systems: Schedule,
    player_input_systems: Schedule,
    monster_move_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        resources.insert(mb.map);
        resources.insert(Camera::new(mb.player_start));
        resources.insert(TurnState::AwaitPlayerInput);// initial state.
        spawn_player(&mut ecs, mb.player_start);
        mb.rooms.iter().skip(mb.rooms.len()-1).map(|r|r.center()).for_each(|pos|{
            spawn_monster(&mut ecs, pos, &mut rng);
        });
        

        Self {
           ecs,
           resources,
           player_input_systems: build_player_input_scheduler(),
           player_move_systems: build_player_move_scheduler(),
           monster_move_systems: build_monster_move_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.mouse_point());
        
        self.resources.insert(ctx.key);
        let current_turn = self.resources.get::<TurnState>().unwrap().clone();
        match current_turn {
            TurnState::AwaitPlayerInput=>{
                self.player_input_systems.execute(&mut self.ecs, &mut self.resources);
            },
            TurnState::MonsterMove=>{
                self.monster_move_systems.execute(&mut self.ecs, &mut self.resources);
            },
            TurnState::PlayerMove=>{
                self.player_move_systems.execute(&mut self.ecs, &mut self.resources);
            }
        }
        let turn_after_execution = self.resources.get::<TurnState>().unwrap().clone();
        if turn_after_execution != current_turn {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            
            let msg = format!("turn_after_execution: {:?}\n",turn_after_execution);
            handle.write_all(msg.as_bytes()).unwrap();
            handle.flush().unwrap();
            
        }
        render_draw_buffer(ctx).expect("Render draw failed");
    }
}

fn main() -> BError{
    let context = BTermBuilder::new()
    .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    .with_tile_dimensions(32, 32)
    .with_resource_path("resources/")
    .with_font("dungeonfont.png", 32, 32)
    .with_font("terminal8x8.png", 8, 8)
    .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
    .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
    .with_simple_console_no_bg(SCREEN_WIDTH*2, SCREEN_HEIGHT*2, "terminal8x8.png")
    .with_fps_cap(30.0)
    .build()?;
    
    let gs = State::new();
    main_loop(context, gs)?;
    
    Ok(())

}
