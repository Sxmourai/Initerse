use crate::*;


#[derive(Component)]
pub struct ParticleComponent {
    vel: Vec2,
    transform: Transform,
    texture: Sprite,
    vis: Visibility,
}
impl ParticleComponent {
    pub(crate) fn update(&mut self) {
        let v = self.vel.clone();
        self.transform.translation = (self.pos()+v).extend(1.);
    }
    pub fn pos(&self) -> Vec2 {
        self.transform.translation.xy()
    }
}