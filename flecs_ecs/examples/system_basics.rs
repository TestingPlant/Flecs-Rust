include!("common");

#[allow(dead_code)]
pub fn main() -> Result<Snap, String> {
    //ignore snap in example, it's for snapshot testing
    let mut snap = Snap::setup_snapshot_test();

    let world = World::new();

    // Create a system for Position, Velocity. Systems are like queries (see
    // queries) with a function that can be ran or scheduled (see pipeline).

    let s = world
        .system::<(&mut Position, &Velocity)>()
        .each_entity(|e, (p, v)| {
            p.x += v.x;
            p.y += v.y;
            fprintln!(snap, "{}: {{ {}, {} }}", e.name(), p.x, p.y);
        });

    // Create a few test entities for a Position, Velocity query
    world
        .entity_named(c"e1")
        .set(Position { x: 10.0, y: 20.0 })
        .set(Velocity { x: 1.0, y: 2.0 });

    world
        .entity_named(c"e2")
        .set(Position { x: 10.0, y: 20.0 })
        .set(Velocity { x: 3.0, y: 4.0 });

    // This entity will not match as it does not have Position, Velocity
    world.entity_named(c"e3").set(Position { x: 10.0, y: 20.0 });

    // Run the system
    s.run();

    Ok(snap)

    // Output:
    //  e1: { 11, 22 }
    //  e2: { 13, 24 }
}
