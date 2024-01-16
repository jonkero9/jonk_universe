use crate::{
    model::{
        planet::Planet,
        star_system::{StarColor, StarSystem},
    },
    VecI,
};
use jonk_utils::Jrand;
use std::collections::HashMap;

pub fn new_universe(location: VecI, size: VecI) -> HashMap<u64, StarSystem> {
    let mut jonk_random = Jrand::new();
    let mut star_map: HashMap<u64, StarSystem> = HashMap::new();
    for x in location.x..(location.x + size.x) {
        for y in location.y..(location.y + size.y) {
            let hash_key = jonk_utils::cantor_hash(x, y);
            jonk_random.seed = hash_key;
            if jonk_random.rnd_range(0, 20) == 1 {
                let star = new_star(VecI { x, y }, &mut jonk_random);
                star_map.insert(hash_key, star);
            }
        }
    }
    return star_map;
}

pub fn new_star(location: VecI, jonk_random: &mut Jrand) -> StarSystem {
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
        location,
        planets: generate_planets(location, jonk_random),
    };
}

fn generate_planets(location: VecI, jonk_random: &mut Jrand) -> Vec<Planet> {
    return (0..jonk_random.rnd_range(0, 13))
        .map(|_x| Planet {
            location,
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
