// TASK FOR COPILOT:
// I want to build a Rust simulation project called "temporal_god_sim_3d".
// It is a **console-based** simulation (text / ASCII only, no graphics lib required)
// of a **true 3D voxel world** with:
//   - basic physics,
//   - primitive life,
//   - emergent civilizations,
//   - an internal "God" AI inside the simulation,
//   - time manipulation (rewind of the simulation state).
//
// IMPORTANT STYLE / STRUCTURE:
// - Single crate, binary project.
// - Use modules to keep the code organized:
//     - world3d
//     - physics
//     - biology
//     - civilization
//     - time_sim
//     - god
//     - render (for text/ASCII summaries)
// - Prefer simple, explicit Rust (no async, no heavy dependencies).
// - Each major struct or enum can live in its own file if needed, but you can start
//   with modules in src/*.rs and expand later.
//
// ===============================
// === WORLD: TRUE 3D VOXEL MAP ===
// ===============================
//
// The world is a 3D voxel grid (x, y, z):
// - width, height, depth (e.g., 32 x 32 x 32 or 64 x 64 x 32).
// - Each voxel has:
//     - material (enum VoxelMaterial)
//     - temperature (f32)
//     - density (f32)
//
// Define:
//
//   pub enum VoxelMaterial {
//       Air,
//       Rock,
//       Soil,
//       Water,
//       Lava,
//       Ice,
//       Organic(u8), // for living/bio matter if needed
//   }
//
//   pub struct Voxel {
//       pub material: VoxelMaterial,
//       pub temperature: f32,
//       pub density: f32,
//   }
//
//   pub struct World3D {
//       pub width: u32,
//       pub height: u32,
//       pub depth: u32,
//       pub voxels: Vec<Voxel>,
//   }
//
// Implement for World3D:
//
//   impl World3D {
//       pub fn new(width: u32, height: u32, depth: u32) -> Self { ... }
//
//       #[inline]
//       pub fn index(&self, x: u32, y: u32, z: u32) -> usize { ... }
//
//       pub fn get(&self, x: u32, y: u32, z: u32) -> &Voxel { ... }
//
//       pub fn get_mut(&mut self, x: u32, y: u32, z: u32) -> &mut Voxel { ... }
//   }
//
// Also implement a simple terrain generator:
// - bottom layers: Rock
// - intermediate: Soil
// - top: some Soil + maybe Water for oceans
// - initialize temperatures with a simple gradient or noise.
// Provide a World3D::generate_basic_world(width, height, depth) helper.
//
// ===================
// === PHYSICS LAYER ===
// ===================
//
// Basic physics is simple and local; we don't need a real physics engine.
// Define:
//
//   pub struct PhysicsRules {
//       pub gravity_enabled: bool,
//       pub heat_diffusion_rate: f32,
//       pub cooling_rate: f32,
//   }
//
// Implement in module `physics`:
//
//   pub fn apply_physics(world: &mut World3D, rules: &PhysicsRules) {
//       // Very simple rules, for example:
//       // - diffuse temperature between neighboring voxels,
//       // - slowly cool towards an ambient temperature,
//       // - optionally make "loose" materials (Soil, Organic) fall down
//       //   if below them is Air (toy gravity).
//   }
//
// Keep it simple and fast; no need for real stability or complex math.
//
// ==============================
// === BIOLOGY / LIFE LAYER  ===
// ==============================
//
// Introduce primitive species and populations that live in the 3D world.
// They occupy specific (x, y, z) positions or small regions.
//
//   pub struct Species {
//       pub id: u32,
//       pub metabolism: f32,
//       pub reproduction_rate: f32,
//       pub mobility: f32,
//       pub preferred_temperature: f32,
//   }
//
//   pub struct Population {
//       pub species_id: u32,
//       pub x: u32,
//       pub y: u32,
//       pub z: u32,
//       pub size: u32,
//   }
//
// In module `biology`, implement:
//
//   pub fn step_biology(
//       world: &mut World3D,
//       species_list: &Vec<Species>,
//       populations: &mut Vec<Population>
//   ) {
//       // For each population:
//       // - read the voxel at (x, y, z)
//       // - adjust population size based on:
//       //      - material (must have Soil/Organic/Water nearby to thrive)
//       //      - temperature match with preferred_temperature
//       // - optionally move some populations to neighboring voxels
//       //   (simple random walk within world bounds).
//   }
//
// You can keep the logic very simple at first (just enough to see life grow or die).
//
// ==============================
// === CIVILIZATION LAYER     ===
// ==============================
//
// Civilizations emerge from big populations.
//
//   pub struct Civilization {
//       pub id: u32,
//       pub name: String,
//       pub x: u32,
//       pub y: u32,
//       pub z: u32,
//       pub population: u32,
//       pub tech_level: f32,
//       pub aggression: f32,
//       pub spirituality: f32,
//   }
//
// In module `civilization`, implement:
//
//   pub fn maybe_spawn_civilizations(
//       populations: &Vec<Population>,
//       civilizations: &mut Vec<Civilization>
//   ) {
//       // If a population exceeds a size threshold and no civ exists at that location,
//       // create a new Civilization with default values.
//   }
//
//   pub fn step_civilizations(
//       world: &World3D,
//       civilizations: &mut Vec<Civilization>
//   ) {
//       // Basic rules:
//       // - increase tech_level slowly over time,
//       // - maybe reduce population if environment is harsh,
//       // - if two civilizations are close in 3D space and aggressive, simulate simple "war":
//       //     - one wins, one loses population, or one collapses.
//       // Keep everything very toy-level; no need for complex logic.
//   }
//
// ==========================================
// === TIME / SIMULATION STATE / MULTIVERSE ===
// ==========================================
//
// We need to store snapshots of the simulation state to allow rewind.
//
// Define in module `time_sim`:
//
//   use crate::world3d::World3D;
//   use crate::physics::PhysicsRules;
//   use crate::biology::{Species, Population};
//   use crate::civilization::Civilization;
//   use crate::god::GodState;
//
//   pub struct SimulationState {
//       pub world: World3D,
//       pub physics_rules: PhysicsRules,
//       pub species: Vec<Species>,
//       pub populations: Vec<Population>,
//       pub civilizations: Vec<Civilization>,
//       pub god_state: GodState,
//   }
//
//   pub struct Timeline {
//       pub id: u32,
//       pub states: Vec<SimulationState>,
//   }
//
//   pub struct Multiverse {
//       pub timelines: Vec<Timeline>,
//       pub current_timeline: u32,
//       pub current_tick: u64,
//   }
//
// Implement:
//
//   impl Multiverse {
//       pub fn new(initial_state: SimulationState) -> Self { ... }
//
//       pub fn current_timeline_mut(&mut self) -> &mut Timeline { ... }
//
//       pub fn push_state(&mut self, state: SimulationState) { ... }
//
//       pub fn rewind(&mut self, ticks: u64) {
//           // Move current_tick backwards by up to `ticks`,
//           // restore the SimulationState from that previous index.
//           // For now, no branching, just plain rewind on the same Timeline.
//       }
//   }
//
// We will have a separate function `simulate_tick(state: &mut SimulationState)` that applies:
//   - physics::apply_physics,
//   - biology::step_biology,
//   - civilization::maybe_spawn_civilizations,
//   - civilization::step_civilizations,
//   - god::step_god.
//
// ==========================
// === GOD AI (INTERNAL)  ===
// ==========================
//
// God is an AI that lives INSIDE the simulation. It is not the user.
// The user is "above God".
//
// In module `god`, define:
//
//   pub struct GodState {
//       pub curiosity: f32,
//       pub benevolence: f32,
//       pub cruelty: f32,
//       pub boredom: f32,
//   }
//
//   pub struct WorldSummary {
//       pub num_civilizations: u32,
//       pub avg_tech_level: f32,
//       pub total_biomass: u32,
//       pub wars_ongoing: u32,
//       pub climate_stability: f32,
//   }
//
//   pub struct PhysicsRulesDelta {
//       pub heat_diffusion_delta: f32,
//       pub cooling_rate_delta: f32,
//   }
//
//   pub enum GodAction {
//       ChangePhysics(PhysicsRulesDelta),
//       SpawnCatastrophe { x: u32, y: u32, z: u32, intensity: f32 },
//       BlessCivilization { civ_id: u32, tech_boost: f32 },
//       None,
//   }
//
// Implement:
//
//   pub fn build_world_summary(state: &crate::time_sim::SimulationState) -> WorldSummary { ... }
//
//   pub fn choose_action(god: &mut GodState, summary: &WorldSummary) -> GodAction {
//       // Simple rule-based behavior for now, for example:
//       // - If num_civilizations == 0 -> increase benevolence.
//       // - If wars_ongoing is high and cruelty is high -> SpawnCatastrophe.
//       // - If boredom is high and num_civilizations > 0 -> BlessCivilization randomly.
//   }
//
//   pub fn apply_action(state: &mut crate::time_sim::SimulationState, action: GodAction) {
//       // Modify physics_rules or civilizations or world voxels depending on the action.
//   }
//
//   pub fn step_god(state: &mut crate::time_sim::SimulationState) {
//       let summary = build_world_summary(state);
//       // Update GodState based on summary (boredom, curiosity, etc.).
//       let action = choose_action(&mut state.god_state, &summary);
//       apply_action(state, action);
//   }
//
// ==========================
// === RENDERING / OUTPUT ===
// ==========================
//
// In module `render`, implement simple console output:
//
//   pub fn print_summary(tick: u64, state: &crate::time_sim::SimulationState) {
//       // Print a short summary:
//       // - tick number
//       // - number of civilizations
//       // - average tech level
//       // - total biomass
//       // - maybe last known "GodAction" (you can store it in GodState if needed).
//   }
//
// Optionally, implement a function to print a 2D slice of the 3D world (for example at a given z height)
// using ASCII characters depending on VoxelMaterial.
//
// =================
// === MAIN LOOP ===
// =================
//
// In src/main.rs:
//
//   - create an initial World3D with World3D::generate_basic_world(width, height, depth);
//   - create some default PhysicsRules;
//   - create an initial list of Species and a few Populations;
//   - GodState starts with some default values;
//   - wrap everything into an initial SimulationState;
//   - create a Multiverse with a single Timeline and this initial state.
//
//   - then run a loop for N ticks (e.g., 500 or 1000):
//       - take the current SimulationState (clone if necessary),
//       - call simulate_tick(&mut state),
//       - push the cloned state into the Timeline,
//       - update multiverse.current_tick,
//       - every, say, 10 ticks, call render::print_summary(tick, &state).
//
// For now, you can:
//   - implement everything in a minimal way,
//   - leave some functions as simple stubs if needed,
//   - focus on getting the structure compiling and the main loop running.
//
// Please now generate the Rust module structure, stubs, and main.rs according to this specification, so I can iterate on the details afterwards.
