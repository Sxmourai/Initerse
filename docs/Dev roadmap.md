# Main ideas

- Have a good storyline, that will improve as the development of the game goes on
- Think about the functionnalities, the experience, building, main goal, graphics
    - Machines: Different sizes (start: planck length, end game: machines that operate at planet scale, or bigger, multiple universes)

## Coding
- 
- Quickly make assets for machines & all, polishing later
- Quickly make ui, for selecting map, multiplayer, game settings ...

- Make the game multiplayer

### Optimisations
Some optimisations that we thought of during dev time
*Probably don't need this now because I don't want it to be tile-based anymore:*
Using some sort of Sparse Quadtree for storing the map, and maybe also an array with all machines.

Try to get the average production of a part of the factory, and stops updating the part when player far away, then get last updated time, and multiply by average production

- Polish assets

# Bevy specific

- Follow [Bevy's best practices](https://github.com/tbillington/bevy_best_practices?tab=readme-ov-file):
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
    - Lottie seems cool for animations https://lottiefiles.com/free-animations/json

- https://github.com/bevyengine/bevy/blob/main/docs/profiling.md
- Make some shaders: https://github.com/alphastrata/shadplay/
- Stop loading all images inside "images" folder, use a typed definition with samplers & all

# Random ideas

Put 2 machines next to each other: buff/nerf => overclocker, ...
