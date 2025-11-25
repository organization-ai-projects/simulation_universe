use crate::civilization::Civilization;
use crate::time_sim::SimulationState;
use rand::Rng;

#[derive(Clone)]
pub struct GodState {
    pub curiosity: f32,
    pub benevolence: f32,
    pub cruelty: f32,
    pub boredom: f32,
}

impl Default for GodState {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            curiosity: rng.gen_range(0.3..0.8),
            benevolence: rng.gen_range(0.4..0.7),
            cruelty: rng.gen_range(0.1..0.4),
            boredom: 0.0,
        }
    }
}

pub struct WorldSummary {
    pub num_civilizations: u32,
    pub avg_tech_level: f32,
    pub total_biomass: u32,
    pub wars_ongoing: u32,
    pub climate_stability: f32,
}

#[derive(Debug, Clone)]
pub struct PhysicsRulesDelta {
    pub heat_diffusion_delta: f32,
    pub cooling_rate_delta: f32,
}

#[derive(Debug, Clone)]
pub enum GodAction {
    ChangePhysics(PhysicsRulesDelta),
    SpawnCatastrophe { x: u32, y: u32, z: u32, intensity: f32 },
    BlessCivilization { civ_id: u32, tech_boost: f32 },
    None,
}

pub fn build_world_summary(state: &SimulationState) -> WorldSummary {
    let num_civilizations = state.civilizations.len() as u32;

    let avg_tech_level = if num_civilizations > 0 {
        state
            .civilizations
            .iter()
            .map(|c| c.tech_level)
            .sum::<f32>()
            / num_civilizations as f32
    } else {
        0.0
    };

    let total_biomass: u32 = state.populations.iter().map(|p| p.size).sum();

    // Count "wars" as pairs of nearby aggressive civilizations
    let mut wars_ongoing = 0;
    for i in 0..state.civilizations.len() {
        for j in (i + 1)..state.civilizations.len() {
            let distance = state.civilizations[i].distance_to(&state.civilizations[j]);
            if distance < 10.0
                && state.civilizations[i].aggression > 0.6
                && state.civilizations[j].aggression > 0.6
            {
                wars_ongoing += 1;
            }
        }
    }

    // Simple climate stability: average temperature deviation
    let temps: Vec<f32> = state
        .world
        .voxels
        .iter()
        .map(|v| v.temperature)
        .collect();
    let avg_temp = temps.iter().sum::<f32>() / temps.len() as f32;
    let variance = temps
        .iter()
        .map(|t| (t - avg_temp).powi(2))
        .sum::<f32>()
        / temps.len() as f32;
    let climate_stability = 1.0 / (1.0 + variance / 100.0);

    WorldSummary {
        num_civilizations,
        avg_tech_level,
        total_biomass,
        wars_ongoing,
        climate_stability,
    }
}

pub fn choose_action(god: &mut GodState, summary: &WorldSummary) -> GodAction {
    let mut rng = rand::thread_rng();

    // Update god's emotional state based on world summary
    if summary.num_civilizations == 0 {
        god.boredom += 0.1;
        god.benevolence += 0.05;
    } else {
        god.boredom = (god.boredom - 0.02).max(0.0);
    }

    if summary.wars_ongoing > 2 {
        god.curiosity += 0.03;
    }

    if summary.total_biomass < 100 {
        god.benevolence += 0.02;
    }

    // Clamp values
    god.curiosity = god.curiosity.clamp(0.0, 1.0);
    god.benevolence = god.benevolence.clamp(0.0, 1.0);
    god.cruelty = god.cruelty.clamp(0.0, 1.0);
    god.boredom = god.boredom.clamp(0.0, 1.0);

    // Decide action based on emotional state
    let roll = rng.gen::<f32>();

    if god.boredom > 0.7 && summary.num_civilizations > 0 {
        // Bored? Do something interesting
        if rng.gen::<f32>() < 0.5 {
            GodAction::BlessCivilization {
                civ_id: rng.gen_range(0..summary.num_civilizations),
                tech_boost: rng.gen_range(0.5..2.0),
            }
        } else {
            GodAction::SpawnCatastrophe {
                x: rng.gen_range(0..64),
                y: rng.gen_range(0..64),
                z: rng.gen_range(0..32),
                intensity: rng.gen_range(5.0..20.0),
            }
        }
    } else if god.cruelty > 0.6 && summary.wars_ongoing > 1 && roll < 0.15 {
        // Cruel and wars are happening? Make it worse
        GodAction::SpawnCatastrophe {
            x: rng.gen_range(0..64),
            y: rng.gen_range(0..64),
            z: rng.gen_range(0..32),
            intensity: rng.gen_range(10.0..30.0),
        }
    } else if god.benevolence > 0.7 && summary.num_civilizations > 0 && roll < 0.1 {
        // Benevolent? Help a civilization
        GodAction::BlessCivilization {
            civ_id: rng.gen_range(0..summary.num_civilizations),
            tech_boost: rng.gen_range(1.0..3.0),
        }
    } else if god.curiosity > 0.8 && roll < 0.05 {
        // Curious? Tweak the physics
        GodAction::ChangePhysics(PhysicsRulesDelta {
            heat_diffusion_delta: rng.gen_range(-0.05..0.05),
            cooling_rate_delta: rng.gen_range(-0.01..0.01),
        })
    } else {
        GodAction::None
    }
}

pub fn apply_action(state: &mut SimulationState, action: GodAction) {
    match action {
        GodAction::ChangePhysics(delta) => {
            state.physics_rules.heat_diffusion_rate =
                (state.physics_rules.heat_diffusion_rate + delta.heat_diffusion_delta)
                    .clamp(0.0, 1.0);
            state.physics_rules.cooling_rate =
                (state.physics_rules.cooling_rate + delta.cooling_rate_delta).clamp(0.0, 0.1);
        }
        GodAction::SpawnCatastrophe { x, y, z, intensity } => {
            // Raise temperature in a region
            for dz in 0..3 {
                for dy in 0..3 {
                    for dx in 0..3 {
                        let nx = x + dx;
                        let ny = y + dy;
                        let nz = z + dz;

                        if nx < state.world.width
                            && ny < state.world.height
                            && nz < state.world.depth
                        {
                            let voxel = state.world.get_mut(nx, ny, nz);
                            voxel.temperature += intensity;
                        }
                    }
                }
            }

            // Kill nearby populations
            state.populations.retain_mut(|pop| {
                let dist = ((pop.x as i32 - x as i32).pow(2)
                    + (pop.y as i32 - y as i32).pow(2)
                    + (pop.z as i32 - z as i32).pow(2)) as f32;
                let dist = dist.sqrt();

                if dist < 5.0 {
                    pop.size = pop.size.saturating_sub((intensity * 10.0) as u32);
                }
                pop.size > 0
            });
        }
        GodAction::BlessCivilization { civ_id, tech_boost } => {
            if let Some(civ) = state.civilizations.iter_mut().find(|c| c.id == civ_id) {
                civ.tech_level += tech_boost;
                civ.population = (civ.population as f32 * 1.2) as u32;
            }
        }
        GodAction::None => {}
    }
}

pub fn step_god(state: &mut SimulationState) -> GodAction {
    let summary = build_world_summary(state);
    let action = choose_action(&mut state.god_state, &summary);
    apply_action(state, action.clone());
    action
}
