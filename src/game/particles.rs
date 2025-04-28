use crate::*;


#[derive(Bundle)]
pub struct ParticleComponent {
    pub vel: Velocity,
    pub transform: Transform,
    pub sprite: Sprite,
    pub vis: Visibility,
    pub _tag: ParticleTag,
}
impl ParticleComponent {
    pub(crate) fn new(pos: Vec2, sprite: Sprite) -> Self {
        Self { vel: Velocity(Vec2::ZERO), transform: Transform::from_translation(pos.extend(1.)), sprite, vis: Visibility::Visible, _tag: ParticleTag }
    }
    pub(crate) fn update(&mut self) {
        self.transform.translation = (self.pos()+self.vel.0).extend(1.);
    }
    pub fn pos(&self) -> Vec2 {
        self.transform.translation.xy()
    }
    
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Copy, PartialEq)]
pub struct ParticleTag;


