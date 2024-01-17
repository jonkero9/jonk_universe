use crate::Vector2DI;

use super::planet::Planet;

#[derive(Debug, Clone, Copy)]
pub enum StarColor {
    Red,
    Orange,
    Yellow,
    White,
    Blue,
}

#[derive(Debug, Clone)]
pub struct StarSystem {
    pub location: Vector2DI,
    /// measured in solar radi, radius of the sun between (2000 - 1); 418,000 miles (696,000 kilometers)
    pub radius: f32,
    /// Total amount of energy emitted from a star per second (25 - 1)
    pub luminosity: f32,
    /// Surface temprature in Kelvin (30,000 - 1,000)
    pub surface_temp: f32,
    /// Solar masses (10000,1);  mass of our sun; 1.99 x 1030 kilograms (330,000 Earth masses)
    pub mass: f32,
    pub planets: Vec<Planet>,
    /// Star color based of surface temp
    pub star_color: StarColor,
}
