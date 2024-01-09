use crate::{
    model::{
        planet::Planet,
        star_system::{StarColor, StarSystem},
    },
    VecI,
};
use jonk_utils::Jrand;
use std::collections::HashMap;

pub fn new_universe(size: i32) -> HashMap<u64, StarSystem> {
    let mut jonk_random = Jrand::new();
    let mut star_map: HashMap<u64, StarSystem> = HashMap::new();
    for x in 0..size {
        for y in 0..size {
            let hash_key = jonk_utils::cantor_hash(x, y);
            jonk_random.seed = hash_key;
            if jonk_random.rnd_range(0, 20) == 1 {
                let star = new_star(x, y);
                star_map.insert(hash_key, star.clone());
            }
        }
    }
    return star_map;
}

pub fn new_star(star_sector_x: i32, star_sector_y: i32) -> StarSystem {
    let mut jonk_random = Jrand::new();
    jonk_random.seed = jonk_utils::cantor_hash(star_sector_x, star_sector_y);
    let surf_temp = jonk_random.rnd_range_float(700.0, 25001.0);
    return StarSystem {
        radius: jonk_random.rnd_range_float(1.0, 2000.0),
        luminosity: jonk_random.rnd_range_float(1.0, 25.0),
        surface_temp: surf_temp,
        mass: jonk_random.rnd_range_float(1.0, 10000.0),
        star_color: match surf_temp {
            x if (10000.0..25001.0).contains(&x) => StarColor::Blue,
            x if (6000.0..10000.0).contains(&x) => StarColor::White,
            x if (4000.0..6000.0).contains(&x) => StarColor::Yellow,
            x if (3000.0..4000.0).contains(&x) => StarColor::Orange,
            x if x < 3000. => StarColor::Red,
            _ => StarColor::Red,
        },
        location: VecI {
            x: star_sector_x,
            y: star_sector_y,
        },
        planets: generate_planets(star_sector_x, star_sector_y, &mut jonk_random),
    };
}

fn generate_planets(
    star_sector_x: i32,
    star_sector_y: i32,
    jonk_random: &mut Jrand,
) -> Vec<Planet> {
    return (0..jonk_random.rnd_range(0, 13))
        .map(|_x| Planet {
            location: VecI {
                x: star_sector_x,
                y: star_sector_y,
            },
            distance_au: jonk_random.rnd_range_float(0.0044, 5.),
            radius: jonk_random.rnd_range_float(0.383, 2000000.),
            rotation: jonk_random.rnd_range(1, 300000),
            revolution: jonk_random.rnd_range(1, 300000),
            eccentricity: jonk_random.rnd_range_float(1000., 2000000.),
            inclination: jonk_random.rnd_range_float(1000., 2000000.),
            mass: jonk_random.rnd_range_float(1000., 2000000.),
            density: jonk_random.rnd_range_float(1000., 2000000.),
        })
        .collect();
}
