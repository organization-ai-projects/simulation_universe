# Temporal God Simulation 3D

A console-based 3D voxel universe simulator written in Rust, featuring physics, biology, emergent civilizations, an internal "God" AI, and time manipulation capabilities.

## Overview

This is a complex simulation of a 3D voxel world where:
- **Physics** governs heat diffusion, cooling, and basic gravity
- **Biology** enables primitive life forms to grow, reproduce, and adapt
- **Civilizations** emerge from large populations and develop technology
- **God AI** lives inside the simulation and can influence the world
- **Time manipulation** allows rewinding the simulation to previous states

## Architecture

The project is organized into modules:

- **world3d** - 3D voxel grid with materials (Air, Rock, Soil, Water, Lava, Ice, Organic)
- **physics** - Basic physics rules (heat diffusion, cooling, gravity)
- **biology** - Species and populations that live and evolve
- **civilization** - Emergent civilizations with tech levels, aggression, and spirituality
- **time_sim** - Timeline management and simulation state tracking
- **god** - Internal AI that observes and influences the simulation
- **render** - Console output and world visualization

## Features

### 3D Voxel World
- Fully 3D voxel grid (default: 64x64x32)
- Multiple material types with temperature and density properties
- Procedurally generated terrain with rocks, soil, water, and air

### Physics Simulation
- Heat diffusion between neighboring voxels
- Ambient cooling toward equilibrium
- Simple gravity for loose materials (soil, organic matter)

### Biology Layer
- Species with unique traits (metabolism, reproduction rate, mobility, temperature preference)
- Populations that grow, move, and adapt to their environment
- Environmental factors affect population survival

### Civilization Emergence
- Civilizations spawn from large populations (500+ individuals)
- Dynamic names, tech levels, aggression, and spirituality
- Conflicts and wars between nearby aggressive civilizations
- Environmental challenges affect population growth

### God AI
- Internal AI with emotional states: curiosity, benevolence, cruelty, boredom
- Observes the world and makes decisions based on current state
- Can perform actions:
  - Change physics rules
  - Spawn catastrophes (heat events)
  - Bless civilizations (tech boost, population increase)
- Emotional state evolves based on simulation events

### Time Manipulation
- Full simulation state is stored at each tick
- Support for rewinding to previous states (implemented but not used in main loop)
- Multiverse structure ready for timeline branching

## Building and Running

### Prerequisites
- Rust 1.70 or later
- Cargo

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run --release
```

The simulation will run for 1000 ticks by default, printing summaries every 50 ticks.

## Configuration

You can modify constants in [src/main.rs](src/main.rs):

- `WORLD_WIDTH`, `WORLD_HEIGHT`, `WORLD_DEPTH` - World dimensions
- `NUM_TICKS` - Number of simulation steps
- `PRINT_INTERVAL` - How often to print summaries

## Output

The simulation prints:
- Tick number
- Number and details of civilizations
- Population and biomass statistics
- God AI emotional state and last action
- Physics parameters
- ASCII world slices (showing material distribution)

Example output:
```
========== TICK 50 ==========
Civilizations: 2
  Avg Tech Level: 1.85
  Total Civ Population: 1420
  - Astranians #0 at (10,10,19) pop:850 tech:2.10 agg:0.45 spirit:0.67
  - Zenooks #1 at (30,26,19) pop:570 tech:1.60 agg:0.78 spirit:0.32
Populations: 12 (Total Biomass: 3450)
God State: curiosity:0.65 benevolence:0.50 cruelty:0.15 boredom:0.05
Last God Action: None
Physics: heat_diff:0.100 cooling:0.020
```

## Code Structure

Each module is self-contained:

- [src/world3d.rs](src/world3d.rs) - Voxel data structures and world generation
- [src/physics.rs](src/physics.rs) - Physics simulation functions
- [src/biology.rs](src/biology.rs) - Species and population management
- [src/civilization.rs](src/civilization.rs) - Civilization emergence and evolution
- [src/god.rs](src/god.rs) - God AI decision making and world influence
- [src/time_sim.rs](src/time_sim.rs) - Simulation state and timeline management
- [src/render.rs](src/render.rs) - Console output and visualization
- [src/main.rs](src/main.rs) - Main simulation loop

## Future Enhancements

Potential improvements (from the original specification):
- Interactive timeline branching (multiverse exploration)
- More complex physics (fluid dynamics, erosion)
- Advanced AI behaviors for God and civilizations
- Trade and diplomacy between civilizations
- Better ASCII/text visualization of the 3D world
- User commands to control simulation (pause, rewind, inspect)
- Save/load simulation states

## License

This project was created as an experimental simulation. Feel free to use and modify as needed.
