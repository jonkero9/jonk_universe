use jonk_utils::Jrand;

#[derive(Debug, Clone, Copy)]
pub struct StarSystem {
    /// measured in solar radi, radius of the sun between (2000 - 1); 418,000 miles (696,000 kilometers)
    pub radius: f32,
    /// Total amount of energy emitted from a star per second (25 - 1)
    pub luminosity: f32,
    /// Surface temprature in Kelvin (210,000 - 1,000)
    pub surface_temp: f32,
    /// Solar masses (10000,1);  mass of our sun; 1.99 x 1030 kilograms (330,000 Earth masses)
    pub mass: f32,
    /// (8, 0)
    pub num_of_planets: u32,
}

impl StarSystem {
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

