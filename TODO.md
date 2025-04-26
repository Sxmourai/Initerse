# TODO

- Following [Bevy's best practices](https://github.com/tbillington/bevy_best_practices?tab=readme-ov-file):
    - All systems needs to be bounded by run conditions on State and SystemSet
    - Co-locate system registration for the same State
    - use Events and learn more about them
    - Write helper utilities for common operations
    - Cleanup:
```rust
fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    q.for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
}
.add_systems(
    OnExit(GameState::InGame),
    cleanup_system::<MarkedToBeCleanedOnGameExit>,)
```
    - Variations to single!(), use tiny_bail
    - Use/define more plugins (dis/enable parts of game, cleaner code)
    - Separate the "release" and "dev" mode better, with features & all (look more how to do nice releases: https://github.com/tbillington/bevy_best_practices?tab=readme-ov-file#release)
    - Look into Tantan's macro for asset loading: https://www.youtube.com/watch?v=ModFC1bhobA


