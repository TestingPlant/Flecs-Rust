mod common;
use common::*;

// Type for Platoon relationship
#[derive(Clone, Component, Debug, Default)]
struct Platoon;

fn main() {
    let world = World::new();

    // Register Platoon as exclusive relationship. This ensures that an entity
    // can only belong to a single Platoon.
    world.component::<Platoon>().add_id(ECS_EXCLUSIVE);

    // Create two platoons
    let platoon_1 = world.new_entity();
    let platoon_2 = world.new_entity();

    // Create a unit
    let unit = world.new_entity();

    // Add unit to platoon 1
    unit.add_pair_second_id::<Platoon>(platoon_1.into());

    // Log platoon of unit
    println!(
        "Unit in platoon 1: {}",
        unit.has_pair_first::<Platoon>(platoon_1.into())
    ); // true
    println!(
        "Unit in platoon 2: {}",
        unit.has_pair_first::<Platoon>(platoon_2.into())
    ); // false

    println!();

    // Add unit to platoon 2. Because Platoon is an exclusive relationship, this
    // both removes (Platoon, platoon_1) and adds (Platoon, platoon_2) in a
    // single operation.
    unit.add_pair_second_id::<Platoon>(platoon_2.into());

    // Log platoon of unit
    println!(
        "Unit in platoon 1: {}",
        unit.has_pair_first::<Platoon>(platoon_1.into())
    ); // false
    println!(
        "Unit in platoon 2: {}",
        unit.has_pair_first::<Platoon>(platoon_2.into())
    ); // true

    // Output:
    //  Unit in platoon 1: true
    //  Unit in platoon 2: false
    //
    //  Unit in platoon 1: false
    //  Unit in platoon 2: true
}