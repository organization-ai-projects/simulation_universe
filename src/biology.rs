use crate::world3d::{VoxelMaterial, World3D};
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
    let mut new_populations: Vec<Population> = Vec::new();

    // Fusionner les populations proches sur le même voxel
    let mut population_map = std::collections::HashMap::new();

    // Ajouter les populations existantes au map
    for pop in populations.iter() {
        let key = (pop.x, pop.y, pop.z, pop.species_id);
        population_map
            .entry(key)
            .and_modify(|existing_size| *existing_size += pop.size)
            .or_insert(pop.size);
    }

    // Ajouter les nouvelles populations au map
    for pop in new_populations.iter() {
        let key = (pop.x, pop.y, pop.z, pop.species_id);
        population_map
            .entry(key)
            .and_modify(|existing_size| *existing_size += pop.size)
            .or_insert(pop.size);
    }

    // Reconstruire la liste des populations
    populations.clear();
    for ((x, y, z, species_id), size) in population_map {
        populations.push(Population::new(species_id, x, y, z, size));
    }

    populations.retain_mut(|pop| {
        // Trouver l'espèce correspondant à cette population
        let species = species_list.iter().find(|s| s.id == pop.species_id);
        if species.is_none() {
            return false;
        }
        let species = species.unwrap();

        // Vérifier si la position est valide dans le monde
        if pop.x >= world.width || pop.y >= world.height || pop.z >= world.depth {
            return false;
        }

        // Récupérer le voxel correspondant à la position de la population
        let voxel_index = world.index(pop.x, pop.y, pop.z);
        let voxel = &mut world.voxels[voxel_index];

        // Vérifier si le matériau du voxel est adapté à la vie
        let suitable_material = matches!(
            voxel.material,
            VoxelMaterial::Soil | VoxelMaterial::Water | VoxelMaterial::Organic(_)
        );

        if !suitable_material {
            // Réduire la taille de la population si le matériau est inadapté
            pop.size = pop.size.saturating_sub(5);
            return pop.size > 0;
        }

        // Calculer la compatibilité de la température avec l'espèce
        let temp_diff = (voxel.temperature - species.preferred_temperature).abs();
        let temp_factor = if temp_diff < 5.0 {
            1.2 // Conditions idéales
        } else if temp_diff < 10.0 {
            1.0 // Conditions neutres
        } else {
            0.8 // Conditions difficiles
        };

        // Limiter la croissance en fonction de la capacité de charge locale
        let carrying_capacity = (voxel.nutrients * 10.0) as u32;
        if pop.size > carrying_capacity {
            pop.size = pop.size.saturating_sub((pop.size - carrying_capacity) / 10);
        }

        // Calculer la croissance de la population
        let growth_rate = species.reproduction_rate * temp_factor;
        let growth = (pop.size as f32 * growth_rate) as i32;
        pop.size = (pop.size as i32 + growth).max(0) as u32;

        // Appliquer le coût métabolique
        let metabolic_cost = (pop.size as f32 * species.metabolism * 0.01) as u32;
        pop.size = pop.size.saturating_sub(metabolic_cost);

        // Consommer les nutriments du voxel
        let nutrient_consumption = pop.size as f32 * 0.1;
        voxel.nutrients = (voxel.nutrients - nutrient_consumption).max(0.0);

        // Déplacer la population vers un voxel voisin avec une certaine probabilité
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

            // Vérifier si la nouvelle position est valide
            if new_x >= 0
                && new_y >= 0
                && new_z >= 0
                && new_x < world.width as i32
                && new_y < world.height as i32
                && new_z < world.depth as i32
            {
                // Diviser la population : une partie reste, l'autre se déplace
                let moving_size = pop.size / 2;
                if moving_size > 10 {
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

        // Marquer les voxels avec de grandes populations comme Organic
        if pop.size > 100 {
            voxel.material = VoxelMaterial::Organic((pop.size / 100).min(255) as u8);
        }

        pop.size > 0
    });

    // Ajouter les nouvelles populations générées
    populations.extend(new_populations);
}
