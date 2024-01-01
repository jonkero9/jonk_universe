use jonk_utils::Jrand;

#[derive(Debug)]
pub struct StarSystem {
    pub radius: f32,         // Solar radi in meters (2000 - 1)
    pub luminosity: f32,     // Solar lum (25 - 1)
    pub surface_temp: f32,   // Kelvin (210000,1000)
    pub mass: f32,           // Solar masses (10000,1)
    pub num_of_planets: u32, // (8, 0)
}

impl StarSystem {
    // add code here
    pub fn new(star_sector_x: i32, star_sector_y: i32) -> StarSystem {
        let mut jonk_random = Jrand::new();
        jonk_random.seed = jonk_utils::cantor_hash(star_sector_x, star_sector_y);

        return StarSystem {
            radius: jonk_random.rnd_range_float(1.0, 2000.0),
            luminosity: jonk_random.rnd_range_float(1.0, 25.0),
            surface_temp: jonk_random.rnd_range_float(1000.0, 210000.0),
            mass: jonk_random.rnd_range_float(1.0, 10000.0),
            num_of_planets: jonk_random.rnd_range(1, 12),
        };
    }
}

