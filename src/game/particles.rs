use crate::*;

pub fn particle_bundle(pos: Vec2,scale: Vec2, sprite: Sprite) -> (Velocity, Transform, Sprite, ParticleTag, StateScoped<Screen>) {
    (
        Velocity(Vec2::ZERO), 
        Transform::from_translation(pos.extend(1.)).with_scale(scale.extend(1.)), 
        sprite, 
        ParticleTag,
        StateScoped(Screen::Game)
    )
}

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Velocity(pub Vec2);

#[derive(Component, Clone, Copy, PartialEq)]
pub struct ParticleTag;


#[derive(Component, Clone, Copy, PartialEq)]
pub struct EnemyParticle;

pub fn enemy_particle(assets: &Res<GameAssets>, pos: Vec2, rad: f32) -> impl Bundle {
    let mut p = particle_bundle(pos, Vec2::splat(rad), get_image(assets, "enemy_particle.png").unwrap().to_sprite());
    p.0.0.x = rand::random();
    p.0.0.y = rand::random();
    (
        p,
        EnemyParticle,
    )
}
#[derive(Resource)]
pub struct PerlinResource(pub noise::Perlin);

pub fn enemy_particle_random_motion_setup(
    mut cmd: Commands,
    assets: Res<GameAssets>,
) {
    cmd.insert_resource(PerlinResource(noise::Perlin::new(1)));
    cmd.spawn(enemy_particle(&assets, Vec2::new(100., -100.), 1.));
}

pub fn enemy_particle_random_motion(
    perlin: ResMut<PerlinResource>,
    mut enemy_particles: Query<(&Transform, &mut Velocity), With<EnemyParticle>>,
) {
    for (trans, mut vel) in &mut enemy_particles {
        let pos = trans.translation;
        let angle = noise::NoiseFn::get(&perlin.0, [(pos.x/2.) as f64, (pos.y/2.) as f64]) as f32*2.0*std::f32::consts::PI;
        let r = 1.;
        // Polar to cartesian coordinates
        vel.0 *= 0.9;
        vel.0 += Vec2::new(r*angle.cos(), r*angle.sin());
    }
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BackgroundMaterial {
    #[uniform(0)]
    pub seed: u32,
}


impl sprite::Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}
