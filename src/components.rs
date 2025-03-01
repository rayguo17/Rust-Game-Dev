use crate::prelude::*;

#[derive(Debug,Clone, Copy,PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType 
}

#[derive(Debug,Clone, Copy,PartialEq)]
pub struct Player;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Monster;

#[derive(Clone, Copy, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy,PartialEq)]
pub struct MoveIntent{
    pub entity: Entity,
    pub destination: Point
}