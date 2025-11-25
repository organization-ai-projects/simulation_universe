mod biology;
mod civilization;
mod god;
mod physics;
mod render;
mod time_sim;
mod world3d;

use biology::{Population, Species};
use god::{GodAction, GodState};
use physics::PhysicsRules;
use time_sim::{Multiverse, SimulationState};
use world3d::World3D;

fn main() {
    println!("=== TEMPORAL GOD SIMULATION 3D ===\n");

    // Configuration
    const WORLD_WIDTH: u32 = 64;
    const WORLD_HEIGHT: u32 = 64;
    const WORLD_DEPTH: u32 = 32;
    const NUM_TICKS: u64 = 1000;
    const PRINT_INTERVAL: u64 = 50;

    // Initialize world
    println!("Generating 3D voxel world ({}x{}x{})...", WORLD_WIDTH, WORLD_HEIGHT, WORLD_DEPTH);
    let world = World3D::generate_basic_world(WORLD_WIDTH, WORLD_HEIGHT, WORLD_DEPTH);

    // Initialize physics
    let physics_rules = PhysicsRules::default();

    // Initialize species
    println!("Creating initial species...");
    let species = vec![
        Species::new(0),
        Species::new(1),
        Species::new(2),
    ];

    // Initialize populations (seed life in various locations)
    println!("Seeding initial populations...");
    let mut populations = Vec::new();

    for i in 0..5 {
        let x = 10 + i * 10;
        let y = 10 + i * 8;
        let z = WORLD_DEPTH * 6 / 10; // Mid-upper level

        populations.push(Population::new(i % 3, x, y, z, 50 + i * 20));
    }

    // Initialize God
    println!("Awakening the God AI...");
    let god_state = GodState::default();

    // Create initial simulation state
    let initial_state = SimulationState::new(
        world,
        physics_rules,
        species,
        populations,
        god_state,
    );

    // Create multiverse with initial timeline
    println!("Creating the multiverse...\n");
    let mut multiverse = Multiverse::new(initial_state);

    // Print initial state
    if let Some(state) = multiverse.current_state() {
        render::print_summary(0, state, &GodAction::None);
        render::print_world_slice(state, WORLD_DEPTH / 2);
    }

    // Main simulation loop
    println!("Starting simulation for {} ticks...\n", NUM_TICKS);

    for tick in 1..=NUM_TICKS {
        // Get current state and clone it for modification
        let current_state = multiverse.current_state().unwrap().clone();
        let mut new_state = current_state;

        // Simulate one tick
        time_sim::simulate_tick(&mut new_state);

        // Store the new state
        multiverse.push_state(new_state);

        // Print periodic updates
        if tick % PRINT_INTERVAL == 0 {
            if let Some(state) = multiverse.current_state() {
                let last_action = god::step_god(&mut state.clone());
                render::print_summary(tick, state, &last_action);

                // Optionally show a world slice every few intervals
                if tick % (PRINT_INTERVAL * 4) == 0 {
                    render::print_world_slice(state, WORLD_DEPTH / 2);
                }
            }
        }
    }

    // Final report
    println!("\n=== SIMULATION COMPLETE ===\n");
    if let Some(final_state) = multiverse.current_state() {
        render::print_detailed_report(final_state);
    }

    println!("Total ticks simulated: {}", NUM_TICKS);
    println!("Timeline states stored: {}", multiverse.current_timeline().len());
    println!("\nThe simulation has ended. The God AI rests.");
}
