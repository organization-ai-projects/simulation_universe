use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoxelMaterial {
    Air,
    Rock,
    Soil,
    Water,
    Lava,
    Ice,
    Organic(u8),
}

#[derive(Debug, Clone)]
pub struct Voxel {
    pub material: VoxelMaterial,
    pub temperature: f32,
    pub density: f32,
}

impl Voxel {
    pub fn new(material: VoxelMaterial, temperature: f32, density: f32) -> Self {
        Self {
            material,
            temperature,
            density,
        }
    }

    pub fn air() -> Self {
        Self::new(VoxelMaterial::Air, 20.0, 0.0)
    }

    pub fn rock() -> Self {
        Self::new(VoxelMaterial::Rock, 15.0, 2.5)
    }

    pub fn soil() -> Self {
        Self::new(VoxelMaterial::Soil, 18.0, 1.2)
    }

    pub fn water() -> Self {
        Self::new(VoxelMaterial::Water, 10.0, 1.0)
    }
}

#[derive(Clone)]
pub struct World3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub voxels: Vec<Voxel>,
}

impl World3D {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        let size = (width * height * depth) as usize;
        let voxels = vec![Voxel::air(); size];
        Self {
            width,
            height,
            depth,
            voxels,
        }
    }

    #[inline]
    pub fn index(&self, x: u32, y: u32, z: u32) -> usize {
        (z * self.width * self.height + y * self.width + x) as usize
    }

    pub fn get(&self, x: u32, y: u32, z: u32) -> &Voxel {
        &self.voxels[self.index(x, y, z)]
    }

    pub fn get_mut(&mut self, x: u32, y: u32, z: u32) -> &mut Voxel {
        let idx = self.index(x, y, z);
        &mut self.voxels[idx]
    }

    pub fn is_valid(&self, x: i32, y: i32, z: i32) -> bool {
        x >= 0 && y >= 0 && z >= 0
            && x < self.width as i32
            && y < self.height as i32
            && z < self.depth as i32
    }

    pub fn generate_basic_world(width: u32, height: u32, depth: u32) -> Self {
        let mut world = Self::new(width, height, depth);
        let mut rng = rand::thread_rng();

        for z in 0..depth {
            for y in 0..height {
                for x in 0..width {
                    let voxel = world.get_mut(x, y, z);

                    // Bottom 30% is rock
                    if z < depth * 3 / 10 {
                        *voxel = Voxel::rock();
                    }
                    // Next 40% is soil
                    else if z < depth * 7 / 10 {
                        *voxel = Voxel::soil();
                        voxel.temperature = 15.0 + rng.gen::<f32>() * 10.0;
                    }
                    // Top 30% is air with occasional water (oceans)
                    else {
                        // Create water "oceans" in some regions
                        let is_ocean = (x < width / 4 || x > width * 3 / 4)
                            && z < depth * 75 / 100;

                        if is_ocean {
                            *voxel = Voxel::water();
                        } else {
                            *voxel = Voxel::air();
                            voxel.temperature = 18.0 + rng.gen::<f32>() * 8.0;
                        }
                    }
                }
            }
        }

        world
    }
}
