use crate::biology::{Population, Species};
use crate::civilization::Civilization;
use crate::god::GodState;
use crate::physics::PhysicsRules;
use crate::world3d::World3D;

#[derive(Clone)]
pub struct SimulationState {
    pub world: World3D,
    pub physics_rules: PhysicsRules,
    pub species: Vec<Species>,
    pub populations: Vec<Population>,
    pub civilizations: Vec<Civilization>,
    pub god_state: GodState,
}

impl SimulationState {
    pub fn new(
        world: World3D,
        physics_rules: PhysicsRules,
        species: Vec<Species>,
        populations: Vec<Population>,
        god_state: GodState,
    ) -> Self {
        Self {
            world,
            physics_rules,
            species,
            populations,
            civilizations: Vec::new(),
            god_state,
        }
    }
}

pub struct Timeline {
    pub id: u32,
    pub states: Vec<SimulationState>,
}

impl Timeline {
    pub fn new(id: u32, initial_state: SimulationState) -> Self {
        Self {
            id,
            states: vec![initial_state],
        }
    }

    pub fn push_state(&mut self, state: SimulationState) {
        self.states.push(state);
    }

    pub fn get_state(&self, index: usize) -> Option<&SimulationState> {
        self.states.get(index)
    }

    pub fn get_state_mut(&mut self, index: usize) -> Option<&mut SimulationState> {
        self.states.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }
}

pub struct Multiverse {
    pub timelines: Vec<Timeline>,
    pub current_timeline: u32,
    pub current_tick: u64,
}

impl Multiverse {
    pub fn new(initial_state: SimulationState) -> Self {
        let timeline = Timeline::new(0, initial_state);
        Self {
            timelines: vec![timeline],
            current_timeline: 0,
            current_tick: 0,
        }
    }

    pub fn current_timeline_mut(&mut self) -> &mut Timeline {
        &mut self.timelines[self.current_timeline as usize]
    }

    pub fn current_timeline(&self) -> &Timeline {
        &self.timelines[self.current_timeline as usize]
    }

    pub fn push_state(&mut self, state: SimulationState) {
        self.current_timeline_mut().push_state(state);
        self.current_tick += 1;
    }

    pub fn current_state(&self) -> Option<&SimulationState> {
        let timeline = self.current_timeline();
        timeline.get_state(self.current_tick as usize)
    }

    pub fn current_state_mut(&mut self) -> Option<&mut SimulationState> {
        let tick = self.current_tick as usize;
        let timeline = self.current_timeline_mut();
        timeline.get_state_mut(tick)
    }

    pub fn rewind(&mut self, ticks: u64) {
        if ticks > self.current_tick {
            self.current_tick = 0;
        } else {
            self.current_tick -= ticks;
        }
    }

    pub fn get_tick(&self) -> u64 {
        self.current_tick
    }
}

pub fn simulate_tick(state: &mut SimulationState) {
    // Apply physics
    crate::physics::apply_physics(&mut state.world, &state.physics_rules);

    // Step biology
    crate::biology::step_biology(&mut state.world, &state.species, &mut state.populations);

    // Maybe spawn new civilizations
    crate::civilization::maybe_spawn_civilizations(&state.populations, &mut state.civilizations);

    // Step civilizations
    crate::civilization::step_civilizations(&state.world, &mut state.civilizations);

    // Step god (returns the action taken)
    let _god_action = crate::god::step_god(state);
}
