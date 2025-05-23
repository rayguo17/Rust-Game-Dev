use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point){
    ecs.push(
        (
            Player,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            },
            Health{
                current: 20,
                max: 20,
            }
        )
    );
}

pub fn spawn_monster(ecs: &mut World, pos: Point, rng :&mut RandomNumberGenerator){
    let (hp, name, glyph) = match rng.range(0, 1){
        0 => goblin(),
        _=> orc(),
    };
    
    ecs.push((
        Monster,
        pos,
        Render{
            color:ColorPair::new(WHITE, BLACK),
            glyph,
        },
        Health{
            current:hp,
            max: hp
        },
        Name(name),
        MovingRandomly
    ));
}

fn goblin() -> (i32,String, FontCharType){
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType){
    (2, "Orc".to_string(), to_cp437('o'))
}
