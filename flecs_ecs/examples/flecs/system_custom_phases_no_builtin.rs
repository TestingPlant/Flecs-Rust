use crate::z_ignore_test_common::*;

use flecs_ecs::prelude::*;
// This application demonstrates how to use custom phases for systems. The
// default pipeline will automatically run systems for custom phases as long as
// they have the flecs::Phase tag.

// Dummy system
fn sys(it: Iter) {
    println!("system {}", it.system().name());
}

fn main() {
    let world = World::new();

    // Create three custom phases. Note that the phases have the Phase tag,
    // which is necessary for the builtin pipeline to discover which systems it
    // should run.

    let update = world.entity().add::<flecs::pipeline::Phase>();

    let physics = world
        .entity()
        .add::<flecs::pipeline::Phase>()
        .depends_on_id(update);

    let collisions = world
        .entity()
        .add::<flecs::pipeline::Phase>()
        .depends_on_id(physics);

    // Create 3 dummy systems.
    world
        .system_named::<()>(c"CollisionSystem")
        .kind_id(collisions)
        .iter_only(sys);

    world
        .system_named::<()>(c"PhysicsSystem")
        .kind_id(physics)
        .iter_only(sys);

    world
        .system_named::<()>(c"GameSystem")
        .kind_id(update)
        .iter_only(sys);

    // Run pipeline
    world.progress();

    // Output:
    //   system GameSystem
    //   system PhysicsSystem
    //   system CollisionSystem
}

#[test]
fn test() {
    let output_capture = OutputCapture::capture().unwrap();
    main();
    output_capture.test("system_custom_phases_no_builtin".to_string());
}
