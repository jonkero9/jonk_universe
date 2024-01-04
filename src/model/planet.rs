use crate::VecI;

#[derive(Debug, Clone, Copy)]
pub struct Planet {
    /// location within the universe (x,y)
    pub location: VecI,
    /// distance is the semi-major axis in astronomical units (1 A.U. = 1.496 × 108 kilometers);
    pub distance_au: f32,
    /// radius of the planet in kilometers
    pub radius: f32,
    /// rotation are the sidereal rotation period and sidereal orbital period in seconds
    pub rotation: u32,
    /// revolution are the sidereal rotation period and sidereal orbital period in seconds
    pub revolution: u32,
    /// eccentricity is the the orbital eccentricity = 1 - (perihelion/semi-major axis);
    pub eccentricity: f32,
    /// inclination is the tilt of the orbit with respect to the ecliptic
    pub inclination: f32,
    /// mass (kg)
    pub mass: f32,
    ///
    pub density: f32,
}
