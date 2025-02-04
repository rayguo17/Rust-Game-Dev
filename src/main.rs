use bracket_lib::prelude::*;
/*
    Notice the different between 
    1. The Interval of time the tick function being called (actual rendering)
    2. the Game State Change Frame Time (Trigger Game State Change, the game time).
*/
enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 80;
const FRAME_DURATION: f32 = 75.0;
struct Player {
    x: i32,
    y: i32,
    velocity: f32
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x:i32, score: i32)-> Self{
        let mut random = RandomNumberGenerator::new();
        Obstacle { x: x, gap_y: random.range(10, 40), size: i32::max(2, 20-score) }
    }
    
    fn render(&mut self, ctx: &mut BTerm, player_x:i32){
        let screen_x = self.x-player_x;
        let half_size = self.gap_y / 2;
        for y in 0..self.gap_y-half_size{
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y+half_size..SCREEN_HEIGHT{
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }
    fn hit_obstacle(&self, player: &Player)->bool{
        let half_size = self.size / 2;
        let x_match = player.x == self.x;
        let player_above_gap = player.y <=self.gap_y-half_size;
        let player_below_gap = player.y>=self.gap_y+half_size;
        x_match && (player_above_gap || player_below_gap)
    }
}

impl Player {
    fn new(x:i32,y:i32)->Self{
        Self { x: x, y: y, velocity: 0.0 }
    }
    fn render(&mut self, ctx: &mut BTerm){
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }
    fn gravity_and_move(&mut self){
        if self.velocity < 2.0 {
            self.velocity += 0.2; // let the increase seperate?
        }
        self.x += 1;
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self){
        self.velocity = -2.0;

    }
}
struct State {
    mode:GameMode,
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
}

impl State {
    fn new()->Self{
        Self { 
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time:0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
         }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu=>self.main_menu(ctx),
            GameMode::Playing=>self.play(ctx),
            GameMode::End=>self.dead(ctx)
        }
        // ctx.cls();
        // ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
    
}
impl State {
    fn main_menu(&mut self,ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        
        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::P=>self.restart(),
                VirtualKeyCode::Q=>ctx.quitting = true,
                _=>{}
            }
        }
    }
    fn dead(&mut self,ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "You are Dead");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key{
            match key {
                VirtualKeyCode::P=>self.restart(),
                VirtualKeyCode::Q=>ctx.quitting=true,
                _=>{}
            }
        }

    }
    fn play(&mut self,ctx: &mut BTerm){
        //Todo: Fill in this stub later
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        self.player.render(ctx);
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        ctx.print(0, 0, "Press Space to Flap");
        ctx.print(0, 1, &format!("Score: {}",self.score));
        self.obstacle.render(ctx, self.player.x);
        
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x+SCREEN_WIDTH, self.score);
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }
    }
    fn restart(&mut self){
        //TODO: 
        // 1. clear state
        // 2. start the game;
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.score = 0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
    }
}

fn main() ->BError{
    let context = BTermBuilder::simple80x50()
    .with_title("Flappy Dragon")
    .build()?;
    main_loop(context, State::new())
}

