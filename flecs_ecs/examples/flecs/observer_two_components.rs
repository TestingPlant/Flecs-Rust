use crate::z_snapshot_test::*;
snapshot_test!();
use flecs_ecs::prelude::*;

#[derive(Debug, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// An observer can match multiple components/tags. Only entities that match the
// entire observer filter will be forwarded to the callback. For example, an
// observer for Position,Velocity won't match an entity that only has Position.

#[test]
fn main() {
    let world = World::new();

    //ignore snap in example, it's for snapshot testing
    world.import::<Snap>();

    // Create observer for custom event
    world
        .observer::<flecs::OnSet, (&Position, &Velocity)>()
        .each_iter(|it, index, (pos, vel)| {
            fprintln!(
                it,
                " - {}: {}: {}: p: {{ {}, {} }}, v: {{ {}, {} }}",
                it.event().name(),
                it.event_id().to_str(),
                it.entity(index).name(),
                pos.x,
                pos.y,
                vel.x,
                vel.y
            );
        });

    // Create entity, set Position (emits EcsOnSet, does not yet match observer)
    let entity = world.entity_named(c"e").set(Position { x: 10.0, y: 20.0 });

    // Set Velocity (emits EcsOnSet, matches observer)
    entity.set(Velocity { x: 1.0, y: 2.0 });

    world
        .get::<Snap>()
        .test("observer_two_components".to_string());

    // Output:
    //  - OnSet: Velocity: e: p: { 10, 20 }, v: { 1, 2 }
}