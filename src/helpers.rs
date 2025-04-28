use crate::*;

#[macro_export]
macro_rules! get_single {
    ($q:expr) => {
        match $q.single() {
            Ok(m) => m,
            _ => return,
        }
    };
}


#[macro_export]
macro_rules! get_single_mut {
    ($q:expr) => {
        match $q.single_mut() {
            Ok(m) => m,
            _ => return,
        }
    };
}


#[derive(Component)]
pub struct MarkedForDeletion;

pub fn cleanup_system<T: Component>(mut cmd: Commands, q: Query<Entity, With<T>>) {
    for e in q {
        cmd.entity(e).despawn();
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "images/", collection(typed, mapped))]
    images: bevy::platform::collections::HashMap<String, Handle<Image>>,
}

pub fn get_machine_image(assets: &Res<GameAssets>, ty: MachineType) -> Handle<Image> {
    // We assume that ty.path() can only return valid paths
    // TODO! Check at startup when loading all images
    get_image(assets, ty.path()).unwrap()
}
pub fn get_image(assets: &Res<GameAssets>, path: &str) -> Option<Handle<Image>> {
    let mut p = "images/".to_string();
    p.push_str(path);
    assets.images.get(&p).map(|handle| handle.clone_weak())
}

pub trait ToSprite {
    fn to_sprite(self) -> Sprite;
}
impl ToSprite for Handle<Image> {
    fn to_sprite(self) -> Sprite {
        Sprite::from_image(self)
    }
}

