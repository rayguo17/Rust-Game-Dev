use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Monster)]
pub fn random_move(
    ecs: &SubWorld,
    commands: &mut CommandBuffer
){
    let mut  rng = RandomNumberGenerator::new();
    <(Entity,&Point, &MovingRandomly)>::query().filter(component::<Monster>()).iter(ecs).for_each(|(entity,pos, _)|{
        let delta = match rng.range(0, 4){
            0=>Point::new(-1, 0), //Left
            1=>Point::new(1,0), //Right
            2=>Point::new(0, 1), //down
            _=>Point::new(0, -1) //Up
        };
        let new_pos = *pos + delta;
        commands.push((
            (),
            MoveIntent{
                entity: *entity,
                destination:new_pos
            }
        ));
    });
}