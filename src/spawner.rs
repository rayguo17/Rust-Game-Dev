use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point){
    ecs.push(
        (
            Player,
            pos,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            }
        )
    );
}

pub fn spawn_monster(ecs: &mut World, pos: Point, rng :&mut RandomNumberGenerator){
    ecs.push((
        Monster,
        pos,
        Render{
            color:ColorPair::new(WHITE, BLACK),
            glyph:match rng.range(0, 3) {
                0=>to_cp437('E'),
                1=>to_cp437('O'),
                2=>to_cp437('g'),
                _=>to_cp437('o'),
            },
        },
        MovingRandomly
    ));
}