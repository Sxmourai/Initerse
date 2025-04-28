use std::time::{Duration, Instant};

use bevy::{color::palettes::css::BLACK, math::ops::atan2};
use bevy_prototype_lyon::prelude::ShapeBuilderBase as _;
use strum::IntoEnumIterator as _;
use window::PrimaryWindow;

use crate::{camera::screen_to_world_pos, *};

use super::{machines::MachineCommon};

#[derive(Component, Debug)]
pub struct Slot {
    pub inner: MachineType,
}
#[derive(Component, Debug)]
pub struct SelectedMachine {
    pub inner: Option<MachineType>,
}

pub fn spawn_hotbar(mut cmd: Commands, assets: Res<GameAssets>) {
    for (i, ty) in MachineType::iter().enumerate() {
        let x = i as f32 * 100. + 200.;
        cmd.spawn((
            Button,
            Slot { inner: ty },
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                left: Val::Px(x),
                top: Val::Percent(90.),
                ..Default::default()
            },
            ImageNode::new(get_machine_image(&assets, ty)),
            StateScoped(Screen::Game),
        ));
    }

    cmd.spawn((
        SelectedMachine { inner: None },
        Sprite::from_color(Color::linear_rgb(1., 1., 1.), Vec2::splat(64.)),
        Visibility::Hidden,
        Transform::default(),
        StateScoped(Screen::Game),
    ));

    let b = BuildingArrow { points: None };
    cmd.spawn((
        arrow(&[Vec2::ZERO; 2], BLACK, 10.),    
        ForceArrow,
        EditingForceArrow,
        Transform::from_translation(Vec3::new(0., 0., 1.)),
        Visibility::Visible,
    ));
    cmd.insert_resource(b);
}


pub fn change_selected(
    slots: Query<(&Interaction, &Slot), (Changed<Interaction>, With<Button>)>,
    mut selected_machine: Query<(&mut SelectedMachine, &mut Visibility, &mut Sprite)>,
    assets: Res<GameAssets>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (button, slot) in slots.iter() {
        match button {
            Interaction::Pressed => {
                let (mut mac, mut vis, mut material) = selected_machine.single_mut().unwrap();
                mac.inner.replace(slot.inner.clone());
                *vis = Visibility::Visible;
                material.image = get_machine_image(&assets, slot.inner);
                mouse.clear_just_pressed(MouseButton::Left);
            }
            _ => {}
        }
    }
    if keys.just_pressed(KeyCode::Escape) || mouse.just_pressed(MouseButton::Right) {
        let (mut mac, mut vis, mut material) = selected_machine.single_mut().unwrap();
        mac.inner = None;
        *vis = Visibility::Hidden;
        // material.0 = ; Don't change it but shoudln't be an issue
        mouse.clear_just_pressed(MouseButton::Right);
    }
}

pub fn build_placeholder(
    mut selected_machine: Query<(&mut Transform, &SelectedMachine)>,
    mut cmd: Commands,
    window_q: Query<&Window>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut kb: ResMut<ButtonInput<KeyCode>>,
    // mut world: ResMut<game::world::World>,
    assets: Res<GameAssets>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut double_click_timer: Local<InstantDefaultNow>,
    mut building_arrow: ResMut<BuildingArrow>,
    mut building_arrow_q: Query<
        (Entity, &mut bevy_prototype_lyon::entity::Shape, &mut Visibility),
        With<EditingForceArrow>,
    >,
) {
    let mut selected_machine = selected_machine.single_mut().unwrap();
    let w = get_single!(window_q);
    if let Some(pos) = w.cursor_position() {
        let world_pos = screen_to_world_pos(pos, camera_q);
        selected_machine.0.translation.x = world_pos.x;
        selected_machine.0.translation.y = world_pos.y;
        redraw_editing_arrow(world_pos, &mut building_arrow, &mut building_arrow_q, &mut mouse, &mut kb, &mut cmd, double_click_timer);
        if mouse.just_pressed(MouseButton::Left) {
            if let Some(ty) = selected_machine.1.inner {
                let machine = MachineCommon {
                    sprite: get_machine_image(&assets, ty).to_sprite(),
                    vis: Visibility::Visible,
                    transform: Transform::from_translation(world_pos.extend(1.)),
                    _state_scoped: StateScoped(Screen::Game),
                };
                let mut e = cmd.spawn((machine, MachineTag));
                match ty {
                    MachineType::StringCreator => e.insert(game::machines::StringCreator::new()),
                    MachineType::Electron => e.insert(game::machines::Electron {  }),
                    MachineType::Energy => e.insert(game::machines::Energy {  }),
                };
                // world.set_machine(world_pos, e.id());
                mouse.clear_just_pressed(MouseButton::Left);
            }
        }
    }
}

fn redraw_editing_arrow(
    t: Vec2,
    mut building_arrow: &mut ResMut<BuildingArrow>,
    mut building_arrow_q: &mut Query<
        (Entity, &mut bevy_prototype_lyon::entity::Shape, &mut Visibility),
        With<EditingForceArrow>,
    >,
    mouse: &mut ResMut<ButtonInput<MouseButton>>,
    kb: &mut ResMut<ButtonInput<KeyCode>>,
    cmd: &mut Commands,
    mut double_click_timer: Local<InstantDefaultNow>,
) {
    if let Some(mut p) = building_arrow.points.as_mut() {
        let l = p.len();
        if p[l-2].distance_squared(t) > 2. {
            p[l-1] = t;
        } else {return}
        if mouse.just_pressed(MouseButton::Left) {
            mouse.clear_just_pressed(MouseButton::Left);
            p.push(t+Vec2::ONE);
            let this_click = InstantDefaultNow::default();
            let delay = this_click.0.duration_since(double_click_timer.0);
            *double_click_timer = this_click;
            if delay < Duration::new(0, 500000000) {
                // 500ms
                building_arrow.points = Some(vec![t, Vec2::ZERO]); // Vec2::ZERO gets re-written
                let (_, shape, mut vis) = building_arrow_q.single_mut().unwrap();
                *vis = Visibility::Visible;
            }
        }
        let mut b = building_arrow_q.single_mut().unwrap();
        *b.1 = building_arrow.shape_object().unwrap();
        if kb.just_pressed(KeyCode::Enter) {
            *b.2 = Visibility::Hidden;
            building_arrow.points = None;
            cmd.spawn((
                b.1.clone(),
                // bevy_vello::svg(asset_server.load("embedded://svg/assets/fountain.svg")),
                ForceArrow,
                Transform::from_translation(Vec3::new(0., 0., 1.)).with_scale(Vec3::splat(5.)),
                Visibility::Visible,
            ));
        }
    }
}

#[derive(Component)]
pub struct ForceArrow;
#[derive(Component)]
pub struct EditingForceArrow;
#[derive(Resource)]
pub struct BuildingArrow {
    points: Option<Vec<Vec2>>,
}
impl BuildingArrow {
    pub fn shape_object(&self) -> Option<bevy_prototype_lyon::prelude::Shape> {
        Some(arrow(&self.points.as_ref()?, BLACK, 10.))
    }
}

pub fn arrow(
    points: &[Vec2],
    color: Srgba,
    stroke_width: f32,
) -> bevy_prototype_lyon::entity::Shape {
    use bevy::{color::palettes::css::*, prelude::*};
    use bevy_prototype_lyon::prelude::*;
    assert!(
        points.len() >= 2,
        "Need at least 2 points for a curved arrow"
    );
    if points[0].distance_squared(points[1]) <= 0.1 {return bevy_prototype_lyon::prelude::ShapeBuilder::with(&bevy_prototype_lyon::shapes::Circle {
            radius: 1.,
            center: Vec2::ZERO,
        })
        .fill(color)
        .build()
    }
    // Create a polyline path from the points
    let mut p = ShapePath::new();
    p = p.move_to(points[0]);
    // if points.len()>2 {
    //     // for i in (0..points.len() - 2).step_by(3) {
    //     //     let a = points[i];
    //     //     let b = points[i + 1];
    //     //     let c = points[i + 2];
    
    //     //     // Midpoints for smoother curve
    //     //     // let ctrl1 = a.lerp(b, 0.5);
    //     //     // let ctrl2 = b.lerp(c, 0.5);
    //     //     p = p.cubic_bezier_to(a, b, c);
    //     // }
    
    //     // // Optional: line to last point to ensure the arrow ends where it should
    //     // p = p.line_to(*points.last().unwrap());
    // } else {
    //     p = p.line_to(points[1]);
    // }
    for p2 in &points[1..] {
        p = p.line_to(*p2);
    }
    let path = p;

    // Compute arrowhead
    let tip = *points.last().unwrap();
    let before_tip = points[points.len() - 2];
    let dir = (tip - before_tip).normalize();
    let side = dir.perp() * stroke_width * 1.5;
    let back = dir * stroke_width * 4.0;

    let a = tip;
    let b = tip - back + side;
    let c = tip - back - side;

    let arrowhead = ShapePath::new().move_to(a).line_to(b).move_to(a).line_to(c);

    // Spawn both path and arrowhead in one bundle-like tuple
    bevy_prototype_lyon::prelude::ShapeBuilder::new().add(&path)
        .add(&arrowhead)
        .stroke((color, stroke_width))
        .build()
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstantDefaultNow(pub std::time::Instant);

impl Default for InstantDefaultNow {
    fn default() -> Self {
        Self(std::time::Instant::now())
    }
}
