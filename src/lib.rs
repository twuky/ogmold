pub mod ogmo;
pub use ogmo::*;

use nanoserde::{DeJson};
use std::{io::Read, path::{self, Path, PathBuf}};

#[derive(Clone)]
pub struct Ogmo {
    pub project: ProjectData,
    pub levels: Vec<(PathBuf, Option<Level>)>,
}

#[derive(Debug, Clone)]
pub struct Level {
    /// short filename, minus '.json'
    pub name: String,
    /// full path to the level file
    pub path: String,
    /// internal ogmo data
    pub data: LevelData,
}

impl Level {
    pub fn from_file(path: &str) -> Option<Self> {
        let path = PathBuf::from(path);
        let mut file = std::fs::File::open(&path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let level = LevelData::deserialize_json(&contents);
        match level {
            Ok(l) => Some(Level { name: path.file_prefix().unwrap().to_str().unwrap().to_string(), path: path.to_str().unwrap().to_string().replace("/./", "/"), data: l }),
            Err(e) => {
                println!("File: {:?}", path);
                println!("Error: {:?}", e);
                None
            }
        }
    }

    pub fn from_json(json: &str) -> Option<Self> {
        let level = LevelData::deserialize_json(json);
        match level {
            Ok(l) => Some(Level { name: "level".to_string(), path: "./".to_string(), data: l }),
            Err(e) => {
                println!("Error: {:?}", e);
                None
            }
        }
    }

    /// Iterate over all layers in this level in order
    pub fn layers(&self) -> impl Iterator<Item = &Layer> {
        self.data.layers.iter()
    }

    /// Iterate over all Tile Layers in this level in order
    pub fn tile_layers(&self) -> impl Iterator<Item = &TileLayer> {
        self.data.layers.iter().filter(|l| matches!(l, Layer::Tile(_))).map(|l| l.as_tile().unwrap())
    }

    /// Iterate over all Grid Layers in this level in order
    pub fn grid_layers(&self) -> impl Iterator<Item = &GridLayer> {
        self.data.layers.iter().filter(|l| matches!(l, Layer::Grid(_))).map(|l| l.as_grid().unwrap())
    }

    /// Iterate over all Entity Layers in this level in order
    pub fn entity_layers(&self) -> impl Iterator<Item = &EntityLayer> {
        self.data.layers.iter().filter(|l| matches!(l, Layer::Entity(_))).map(|l| l.as_entity().unwrap())
    }

    /// Iterate over all Decal Layers in this level in order
    pub fn decal_layers(&self) -> impl Iterator<Item = &DecalLayer> {
        self.data.layers.iter().filter(|l| matches!(l, Layer::Decal(_))).map(|l| l.as_decal().unwrap())
    }
}

impl Ogmo {
    pub fn new(project_path: &str) -> Option<Self> {
        Self::from_file(project_path)
    }

    pub fn from_json(json: &str) -> Option<Self> {
        let project = ProjectData::deserialize_json(json);
        let mut levels = Vec::new();

        match project {
            Ok(p) => {
                let mut level_jsons = Vec::default();

                for level_path in &p.level_paths {
                    let level_path = PathBuf::from("./".to_string() + level_path);
                    let max_depth = p.directory_depth;
                    // get all json files in all subdirectories up to max_depth
                    search(0, max_depth, &level_path, &mut level_jsons);
                }

                for path in level_jsons {
                    levels.push((path, None));
                }

                return Some(Self { project: p, levels });
            }
            Err(e) => {
                println!("project load error");
                println!("Error: {:?}", e);
            }
        };
        None
    }

    pub fn from_file(project_path: &str) -> Option<Self> {
        let mut file = std::fs::File::open(project_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let path = path::Path::new(project_path);
        let base_dir = path.parent().unwrap();

        let project = ProjectData::deserialize_json(&contents);
        let mut levels = Vec::new();

        match project {
            Ok(p) => {
                let mut level_jsons = Vec::default();

                for level_path in &p.level_paths {
                    let level_path = base_dir.join(level_path);
                    let max_depth = p.directory_depth;
                    // get all json files in all subdirectories up to max_depth
                    search(0, max_depth, &level_path, &mut level_jsons);
                }

                for path in level_jsons {
                    levels.push((path, None));
                }

                return Some(Self { project: p, levels });
            }
            Err(e) => {
                println!("project load error");
                println!("Error: {:?}", e);
            }
        };
        None
    }

    pub fn get_level(&self, name: &str) -> Option<&Level> {
        for (_key, level) in &self.levels {
            if let Some(level) = level
                && level.name == name {
                    return Some(level);
                }
        }
        None
    }

    pub fn get_level_by_path(&self, path: &str) -> Option<&Level> {
        for (_key, level) in &self.levels {
            if let Some(level) = level
                && level.path == path {
                    return Some(level);
                }
        }
        None
    }

    pub fn iter_levels(&self) -> impl Iterator<Item = &Level> {
        self.levels.iter().map_while(|(_path, level)| {
            level.as_ref()
        })
    }

    fn load(path: &PathBuf) -> Option<LevelData> {
        let mut file = std::fs::File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let level = LevelData::deserialize_json(&contents);
        match level {
            Ok(l) => Some(l),
            Err(e) => {
                println!("File: {:?}", path);
                println!("Error: {:?}", e);
                None
            }
        }
    }

    /// Load a level by name.
    /// 
    /// This will check the possible levels as defined by the project for a level with the given name.
    pub fn load_level(&mut self, name: &str) -> Option<&Level> {
        for (path, lvl) in &mut self.levels {
            if path.file_prefix().unwrap().to_str().unwrap() == name {
                if let Some(level) = lvl {
                    return Some(level);
                }

                // if the level is not loaded, load it
                let level = Self::load(path);
                if let Some(level) = level {
                    lvl.replace(Level {
                        name: path.file_prefix().unwrap().to_str().unwrap().to_string(),
                        path: path.to_str().unwrap().to_string().replace("/./", "/"),
                        data: level,
                    });
                }
                return lvl.as_ref();
            }
        }
        None
    }

    pub fn load_all_levels(&mut self) {
        for (path, lvl) in &mut self.levels.clone() {
            let level = Self::load(path);

            if let Some(level) = level {
                lvl.replace(Level {
                    name: path.file_prefix().unwrap().to_str().unwrap().to_string(),
                    path: path.to_str().unwrap().to_string().replace("/./", "/"),
                    data: level,
                });
            }
        }
    }
}

/// recursively search folders up to a maximum depth
fn search(depth: i32, max_depth: i32, path: &path::PathBuf, out: &mut Vec<path::PathBuf>) {
    if depth > max_depth {
        return;
    }
    let mut dirs = Vec::default();
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            if path.extension().unwrap() == "json" {
                out.push(path);
            }
        } else if path.is_dir() {
            dirs.push(path);
        }
    }
    for dir in dirs {
        search(depth + 1, max_depth, &dir, out);
    }
}