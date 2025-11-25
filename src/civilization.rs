use crate::biology::Population;
use crate::world3d::World3D;
use rand::Rng;

#[derive(Clone)]
pub struct Civilization {
    pub id: u32,
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub population: u32,
    pub tech_level: f32,
    pub aggression: f32,
    pub spirituality: f32,
}

impl Civilization {
    pub fn new(id: u32, x: u32, y: u32, z: u32, population: u32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            name: generate_civ_name(id),
            x,
            y,
            z,
            population,
            tech_level: 1.0,
            aggression: rng.gen_range(0.0..1.0),
            spirituality: rng.gen_range(0.0..1.0),
        }
    }

    pub fn distance_to(&self, other: &Civilization) -> f32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        let dz = self.z as f32 - other.z as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn generate_civ_name(id: u32) -> String {
    let prefixes = ["Astra", "Terra", "Zeno", "Kryth", "Luma", "Vexis", "Orin", "Drak"];
    let suffixes = ["nians", "ites", "oks", "ans", "ari", "oni", "ian", "eth"];

    let mut rng = rand::thread_rng();
    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let suffix = suffixes[rng.gen_range(0..suffixes.len())];

    format!("{}{} #{}", prefix, suffix, id)
}

pub fn maybe_spawn_civilizations(
    populations: &[Population],
    civilizations: &mut Vec<Civilization>,
) {
    const CIVILIZATION_THRESHOLD: u32 = 500;

    for pop in populations {
        if pop.size < CIVILIZATION_THRESHOLD {
            continue;
        }

        // Check if a civilization already exists at this location
        let already_exists = civilizations.iter().any(|civ| {
            civ.x == pop.x && civ.y == pop.y && civ.z == pop.z
        });

        if !already_exists {
            let new_id = civilizations.len() as u32;
            let civ = Civilization::new(new_id, pop.x, pop.y, pop.z, pop.size);
            civilizations.push(civ);
        }
    }
}

pub fn step_civilizations(world: &World3D, civilizations: &mut Vec<Civilization>) {
    let mut rng = rand::thread_rng();

    // Update each civilization
    for civ in civilizations.iter_mut() {
        // Slowly increase tech level
        civ.tech_level += 0.01 + rng.gen::<f32>() * 0.02;

        // Check environment harshness
        if civ.x < world.width && civ.y < world.height && civ.z < world.depth {
            let voxel = world.get(civ.x, civ.y, civ.z);
            let harsh = voxel.temperature < 10.0 || voxel.temperature > 30.0;

            if harsh {
                let loss = (civ.population as f32 * 0.05) as u32;
                civ.population = civ.population.saturating_sub(loss);
            } else {
                // Grow population slightly
                let growth = (civ.population as f32 * 0.02) as u32;
                civ.population += growth;
            }
        }

        // Adapt spirituality and aggression over time
        civ.spirituality += (rng.gen::<f32>() - 0.5) * 0.01;
        civ.spirituality = civ.spirituality.clamp(0.0, 1.0);

        civ.aggression += (rng.gen::<f32>() - 0.5) * 0.01;
        civ.aggression = civ.aggression.clamp(0.0, 1.0);
    }

    // Check for conflicts between nearby civilizations
    let civ_count = civilizations.len();
    for i in 0..civ_count {
        for j in (i + 1)..civ_count {
            let distance = {
                let civ_i = &civilizations[i];
                let civ_j = &civilizations[j];
                civ_i.distance_to(civ_j)
            };

            if distance < 10.0 {
                let aggression_sum = civilizations[i].aggression + civilizations[j].aggression;

                if aggression_sum > 1.2 && rng.gen::<f32>() < 0.1 {
                    // War! The stronger one wins
                    let (winner_idx, loser_idx) = if civilizations[i].tech_level
                        + civilizations[i].population as f32 * 0.001
                        > civilizations[j].tech_level + civilizations[j].population as f32 * 0.001
                    {
                        (i, j)
                    } else {
                        (j, i)
                    };

                    // Winner gains population, loser loses heavily
                    let spoils = civilizations[loser_idx].population / 3;
                    civilizations[winner_idx].population += spoils;
                    civilizations[loser_idx].population =
                        civilizations[loser_idx].population.saturating_sub(spoils * 2);

                    civilizations[winner_idx].tech_level += 0.1;
                }
            }
        }
    }

    // Remove collapsed civilizations
    civilizations.retain(|civ| civ.population > 50);
}
