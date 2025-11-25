use crate::world3d::{World3D, VoxelMaterial};
use rand::Rng;

#[derive(Clone)]
pub struct Species {
    pub id: u32,
    pub metabolism: f32,
    pub reproduction_rate: f32,
    pub mobility: f32,
    pub preferred_temperature: f32,
}

impl Species {
    pub fn new(id: u32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            metabolism: rng.gen_range(0.5..2.0),
            reproduction_rate: rng.gen_range(0.01..0.1),
            mobility: rng.gen_range(0.1..1.0),
            preferred_temperature: rng.gen_range(15.0..25.0),
        }
    }
}

#[derive(Clone)]
pub struct Population {
    pub species_id: u32,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub size: u32,
}

impl Population {
    pub fn new(species_id: u32, x: u32, y: u32, z: u32, size: u32) -> Self {
        Self {
            species_id,
            x,
            y,
            z,
            size,
        }
    }
}

pub fn step_biology(
    world: &mut World3D,
    species_list: &[Species],
    populations: &mut Vec<Population>,
) {
    let mut rng = rand::thread_rng();
    let mut new_populations = Vec::new();

    populations.retain_mut(|pop| {
        // Find the species
        let species = species_list.iter().find(|s| s.id == pop.species_id);
        if species.is_none() {
            return false;
        }
        let species = species.unwrap();

        // Check if position is valid
        if pop.x >= world.width || pop.y >= world.height || pop.z >= world.depth {
            return false;
        }

        // Get the voxel at this population's location
        let voxel = world.get(pop.x, pop.y, pop.z);

        // Check if the material is suitable for life
        let suitable_material = matches!(
            voxel.material,
            VoxelMaterial::Soil | VoxelMaterial::Water | VoxelMaterial::Organic(_)
        );

        if !suitable_material {
            pop.size = pop.size.saturating_sub(5);
            return pop.size > 0;
        }

        // Temperature compatibility
        let temp_diff = (voxel.temperature - species.preferred_temperature).abs();
        let temp_factor = if temp_diff < 5.0 {
            1.2 // Good conditions
        } else if temp_diff < 10.0 {
            1.0 // Neutral
        } else {
            0.8 // Harsh conditions
        };

        // Update population size
        let growth_rate = species.reproduction_rate * temp_factor;
        let growth = (pop.size as f32 * growth_rate) as i32;
        pop.size = (pop.size as i32 + growth).max(0) as u32;

        // Apply metabolism cost
        let metabolic_cost = (pop.size as f32 * species.metabolism * 0.01) as u32;
        pop.size = pop.size.saturating_sub(metabolic_cost);

        // Maybe move to a neighboring voxel
        if rng.gen::<f32>() < species.mobility * 0.1 {
            let directions = [
                (-1i32, 0i32, 0i32),
                (1, 0, 0),
                (0, -1, 0),
                (0, 1, 0),
                (0, 0, -1),
                (0, 0, 1),
            ];

            let (dx, dy, dz) = directions[rng.gen_range(0..directions.len())];
            let new_x = pop.x as i32 + dx;
            let new_y = pop.y as i32 + dy;
            let new_z = pop.z as i32 + dz;

            if world.is_valid(new_x, new_y, new_z) {
                // Split population: half stays, half moves
                let moving_size = pop.size / 2;
                if moving_size > 0 {
                    pop.size -= moving_size;
                    new_populations.push(Population::new(
                        pop.species_id,
                        new_x as u32,
                        new_y as u32,
                        new_z as u32,
                        moving_size,
                    ));
                }
            }
        }

        // Mark voxels with large populations as Organic
        if pop.size > 100 {
            let voxel = world.get_mut(pop.x, pop.y, pop.z);
            voxel.material = VoxelMaterial::Organic((pop.size / 100).min(255) as u8);
        }

        pop.size > 0
    });

    populations.extend(new_populations);
}
