use crate::world3d::{World3D, VoxelMaterial};

#[derive(Clone)]
pub struct PhysicsRules {
    pub gravity_enabled: bool,
    pub heat_diffusion_rate: f32,
    pub cooling_rate: f32,
}

impl Default for PhysicsRules {
    fn default() -> Self {
        Self {
            gravity_enabled: true,
            heat_diffusion_rate: 0.1,
            cooling_rate: 0.02,
        }
    }
}

pub fn apply_physics(world: &mut World3D, rules: &PhysicsRules) {
    apply_heat_diffusion(world, rules);
    apply_cooling(world, rules);

    if rules.gravity_enabled {
        apply_simple_gravity(world);
    }
}

fn apply_heat_diffusion(world: &mut World3D, rules: &PhysicsRules) {
    let mut temp_buffer = vec![0.0; world.voxels.len()];

    // Copy current temperatures
    for (i, voxel) in world.voxels.iter().enumerate() {
        temp_buffer[i] = voxel.temperature;
    }

    // Diffuse heat to neighbors
    for z in 0..world.depth {
        for y in 0..world.height {
            for x in 0..world.width {
                let idx = world.index(x, y, z);
                let current_temp = temp_buffer[idx];

                let mut neighbor_count = 0;
                let mut neighbor_temp_sum = 0.0;

                // Check 6 neighbors (up, down, left, right, front, back)
                let neighbors = [
                    (x as i32 - 1, y as i32, z as i32),
                    (x as i32 + 1, y as i32, z as i32),
                    (x as i32, y as i32 - 1, z as i32),
                    (x as i32, y as i32 + 1, z as i32),
                    (x as i32, y as i32, z as i32 - 1),
                    (x as i32, y as i32, z as i32 + 1),
                ];

                for (nx, ny, nz) in neighbors.iter() {
                    if world.is_valid(*nx, *ny, *nz) {
                        let n_idx = world.index(*nx as u32, *ny as u32, *nz as u32);
                        neighbor_temp_sum += temp_buffer[n_idx];
                        neighbor_count += 1;
                    }
                }

                if neighbor_count > 0 {
                    let avg_neighbor_temp = neighbor_temp_sum / neighbor_count as f32;
                    let new_temp = current_temp +
                        (avg_neighbor_temp - current_temp) * rules.heat_diffusion_rate;
                    world.voxels[idx].temperature = new_temp;
                }
            }
        }
    }
}

fn apply_cooling(world: &mut World3D, rules: &PhysicsRules) {
    const AMBIENT_TEMP: f32 = 20.0;

    for voxel in world.voxels.iter_mut() {
        let diff = AMBIENT_TEMP - voxel.temperature;
        voxel.temperature += diff * rules.cooling_rate;
    }
}

fn apply_simple_gravity(world: &mut World3D) {
    // Very simple: if a loose material (Soil, Organic) has Air below it, swap them
    for z in (1..world.depth).rev() {
        for y in 0..world.height {
            for x in 0..world.width {
                let current = world.get(x, y, z).material;
                let below = world.get(x, y, z - 1).material;

                let is_loose = matches!(current, VoxelMaterial::Soil | VoxelMaterial::Organic(_));
                let is_air_below = matches!(below, VoxelMaterial::Air);

                if is_loose && is_air_below {
                    // Swap the two voxels
                    let current_idx = world.index(x, y, z);
                    let below_idx = world.index(x, y, z - 1);
                    world.voxels.swap(current_idx, below_idx);
                }
            }
        }
    }
}
