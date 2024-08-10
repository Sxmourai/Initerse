use std::path::PathBuf;

use color_eyre::eyre::{eyre, Context, ContextCompat};
use toml::Value;

use super::*;

async fn parse_celestial(entry: Result<std::fs::DirEntry, std::io::Error>) -> Result<Celestial> {
    let raw = std::fs::read_to_string(entry?.path())?;
    let mut config = toml::de::from_str::<toml::Table>(&raw)?;
    let (x,y) = match config.remove("size").context("No size key in config")? {
        Value::Array(arr) => {(arr[0].as_integer().unwrap(),arr[1].as_integer().unwrap())},
        size => Err(eyre!("Invalid size {:?}", size))?,
    };
    let path = match config.remove("path").context("No path key in config")? {
        Value::String(path) => {path},
        path => Err(eyre!("Invalid path {:?}", path))?,
    };
    let flavor = match &config.remove("flavor").context("No flavor key in config")? {
        toml::Value::String(flavor) => {
            match flavor.as_str() {
                "star"     => CelestialFlavor::Star(parse_star(config).context("Error parsing star")?),
                "planet"   => CelestialFlavor::Planet(parse_planet(config).context("Error parsing planet")?),
                "asteroid" => CelestialFlavor::Asteroid(parse_asteroid(config).context("Error parsing asteroid")?),
                flavor => {return Err(Report::msg(format!("Invalid flavor for celestial ({:?})", flavor)))}
            }
        }
        _ => return Err(Report::msg("Invalid type for flavor, should be string")),
    };
    let texture = load_texture(&format!("assets/{}",path)).await?;
    Ok(Celestial {
        texture,
        size: ivec2(x as i32, y as i32),
        flavor,
    })
}

pub fn parse_star(config: toml::map::Map<String, toml::Value>) -> Result<Star> {
    let star = Star {
        
    };
    Ok(star)
}
pub fn parse_planet(config: toml::map::Map<String, toml::Value>) -> Result<Planet> {
    let planet = Planet {
        
    };
    Ok(planet)
}
pub fn parse_asteroid(config: toml::map::Map<String, toml::Value>) -> Result<Asteroid> {
    let asteroid = Asteroid {
        
    };
    Ok(asteroid)
}

pub async fn parse_celestials() -> Vec<Celestial> {
    let mut celestials = Vec::new();
    for entry in std::fs::read_dir("assets/celestials").unwrap() {
        celestials.push(match parse_celestial(entry).await {
            Ok(file) => file,
            Err(err) => {warn!("Skipping one config entry: {:?}", err);continue},
        })
    }

    celestials
}

pub struct Celestial {
    texture: Texture2D,
    pub size: IVec2,
    flavor: CelestialFlavor,
}
pub enum CelestialFlavor {
    Star(Star),
    Planet(Planet),
    Asteroid(Asteroid),
}
impl Celestial {
    pub fn texture(&self) -> &Texture2D {&self.texture}
}
//         match self {
//             Celestial::Star(s) => {
//                 s.texture();
//             },
//             Celestial::Planet(p) => {
//                 p.texture();
//             },
//             Celestial::Asteroid(a) => {
//                 a.texture();
//             },
//         }
//     }
// }

pub struct Star {
    
}
// impl Star {
//     pub fn texture(&self) {
//         texture_texture_ex(texture, x, y, color, params)
//     }
// }
pub struct Planet {
    
}
// impl Planet {
//     pub fn texture(&self) {
//         texture_texture_ex(texture, x, y, color, params)
//     }
// }
pub struct Asteroid {
    
}
// impl Asteroid {
//     pub fn texture(&self) {
//         texture_texture_ex(texture, x, y, color, params)
//     }
// }