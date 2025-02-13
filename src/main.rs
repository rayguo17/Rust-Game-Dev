mod systems;
mod spawner;
mod components;
mod map_builder;
mod map;
mod camera;

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
}
use prelude::*;

struct State {
    ecs: World,
    resources: Resources, // Could we allow <T> Cell and Ref Cell is for lock access. Correct?
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mb = MapBuilder::new(&mut rng);
        resources.insert(mb.map);
        resources.insert(Camera::new(mb.player_start));
        spawn_player(&mut ecs, mb.player_start);
        Self {
           ecs,
           resources,
           systems: build_scheduler()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        render_draw_buffer(ctx).expect("Render draw failed");
        self.systems.execute(&mut self.ecs, &mut self.resources);
    }
}

fn main() -> BError{
    let context = BTermBuilder::new()
    .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
    .with_tile_dimensions(32, 32)
    .with_resource_path("resources/")
    .with_font("dungeonfont.png", 32, 32)
    .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
    .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
    .with_fps_cap(30.0)
    .build()?;
    
    let gs = State::new();
    main_loop(context, gs)?;
    
    Ok(())

}
