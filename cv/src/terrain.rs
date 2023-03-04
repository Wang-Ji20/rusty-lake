/*!
 * Terrain module provides landform support for cv.
 *
 */

use std::fmt::Display;

use crate::config::Config;

/**
 * Terrian represents the landform of a land..
 *  Notes
 *  -----
 * River: lake, river, .. etc
 * Sea: Salty lake, sea, ocean...
 */

#[derive(Clone)]
pub enum Terrain {
    Plain,
    Mountain,
    Desert,
    Highland,
    Island,
    Forest,
    Sea,
    River,
}

impl Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Terrain::Desert => "Desert",
            Terrain::Forest => "Forest",
            Terrain::Highland => "Highland",
            Terrain::Island => "Island",
            Terrain::Mountain => "Mountain",
            Terrain::Plain => "Plain",
            Terrain::River => "River",
            Terrain::Sea => "Sea",
        };
        write!(f, "Terrain: {}", s)
    }
}

/**
 * This Struct owns and manages all terrain state.
 * land borrows terrain from terrian manager
 * and population grows in land..
 */
pub struct TerrainManager {
    terrain_map: Vec<Terrain>,
}

impl TerrainManager {
    pub fn new(config: &Config) -> Option<TerrainManager> {
        let ln: usize = config.land_num.try_into().ok()?;
        let terrian_map = vec![Terrain::Plain; ln];
        Some(TerrainManager {
            terrain_map: terrian_map,
        })
    }
}

impl Display for TerrainManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Terrian Manager:\n")?;
        self.terrain_map
            .iter()
            .map(|t| -> Result<(), std::fmt::Error> { write!(f, " {}\n", t) })
            .collect()
    }
}
