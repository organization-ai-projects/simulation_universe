use crate::god::GodAction;
use crate::time_sim::SimulationState;
use crate::world3d::VoxelMaterial;

pub fn print_summary(tick: u64, state: &SimulationState, last_god_action: &GodAction) {
    println!("\n========== TICK {} ==========", tick);

    // Civilizations
    let num_civs = state.civilizations.len();
    println!("Civilizations: {}", num_civs);

    if num_civs > 0 {
        let avg_tech: f32 =
            state.civilizations.iter().map(|c| c.tech_level).sum::<f32>() / num_civs as f32;
        let total_civ_pop: u32 = state.civilizations.iter().map(|c| c.population).sum();
        println!("  Avg Tech Level: {:.2}", avg_tech);
        println!("  Total Civ Population: {}", total_civ_pop);

        for civ in state.civilizations.iter().take(3) {
            println!(
                "  - {} at ({},{},{}) pop:{} tech:{:.2} agg:{:.2} spirit:{:.2}",
                civ.name, civ.x, civ.y, civ.z, civ.population, civ.tech_level, civ.aggression, civ.spirituality
            );
        }
        if num_civs > 3 {
            println!("  ... and {} more", num_civs - 3);
        }
    }

    // Biology
    let num_pops = state.populations.len();
    let total_biomass: u32 = state.populations.iter().map(|p| p.size).sum();
    println!("Populations: {} (Total Biomass: {})", num_pops, total_biomass);

    // God
    println!(
        "God State: curiosity:{:.2} benevolence:{:.2} cruelty:{:.2} boredom:{:.2}",
        state.god_state.curiosity,
        state.god_state.benevolence,
        state.god_state.cruelty,
        state.god_state.boredom
    );
    println!("Last God Action: {:?}", last_god_action);

    // Physics
    println!(
        "Physics: heat_diff:{:.3} cooling:{:.3}",
        state.physics_rules.heat_diffusion_rate, state.physics_rules.cooling_rate
    );

    println!("==============================\n");
}

pub fn print_world_slice(state: &SimulationState, z_level: u32) {
    if z_level >= state.world.depth {
        println!("Invalid z level: {}", z_level);
        return;
    }

    println!("\n--- World Slice at Z={} ---", z_level);

    for y in (0..state.world.height).rev() {
        for x in 0..state.world.width {
            let voxel = state.world.get(x, y, z_level);
            let char = match voxel.material {
                VoxelMaterial::Air => '.',
                VoxelMaterial::Rock => '#',
                VoxelMaterial::Soil => ':',
                VoxelMaterial::Water => '~',
                VoxelMaterial::Lava => '*',
                VoxelMaterial::Ice => 'i',
                VoxelMaterial::Organic(_) => 'o',
            };
            print!("{}", char);
        }
        println!();
    }
    println!("----------------------------\n");
}

pub fn print_detailed_report(state: &SimulationState) {
    println!("\n========== DETAILED REPORT ==========");

    // World statistics
    let mut material_counts = std::collections::HashMap::new();
    let mut temp_sum = 0.0;

    for voxel in &state.world.voxels {
        *material_counts
            .entry(format!("{:?}", voxel.material))
            .or_insert(0) += 1;
        temp_sum += voxel.temperature;
    }

    let avg_temp = temp_sum / state.world.voxels.len() as f32;

    println!("World: {}x{}x{}", state.world.width, state.world.height, state.world.depth);
    println!("Average Temperature: {:.2}°C", avg_temp);
    println!("Material Distribution:");
    for (material, count) in material_counts {
        println!("  {}: {}", material, count);
    }

    // Species info
    println!("\nSpecies: {}", state.species.len());
    for species in &state.species {
        println!(
            "  Species #{}: metabolism:{:.2} repro:{:.2} mobility:{:.2} pref_temp:{:.2}",
            species.id, species.metabolism, species.reproduction_rate, species.mobility, species.preferred_temperature
        );
    }

    // Civilizations
    println!("\nCivilizations: {}", state.civilizations.len());
    for civ in &state.civilizations {
        println!(
            "  {}: pop:{} tech:{:.2} aggression:{:.2} spirituality:{:.2} at ({},{},{})",
            civ.name, civ.population, civ.tech_level, civ.aggression, civ.spirituality, civ.x, civ.y, civ.z
        );
    }

    println!("=====================================\n");
}
