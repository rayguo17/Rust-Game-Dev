use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
    commands: &mut CommandBuffer
){
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0,1),
            _=>Point::new(0,0)
        };
        if delta.x !=0 || delta.y !=0 {
            let mut players = <(Entity,&Point)>::query()
            .filter(component::<Player>());
            players.iter(ecs).for_each(|(entity,pos)|{
                let destination = *pos + delta;
                commands.push(((),
                    MoveIntent{
                        entity:*entity,
                        destination
                    }
                ));
            });
        }
        *turn_state = TurnState::PlayerMove;
    }
}