use bevy::{
    asset::LoadedUntypedAsset,
    prelude::{Entity, Handle, Resource},
    scene::Scene,
    utils::HashSet,
};
use kiddo::float::kdtree::KdTree;
use std::{fmt::Debug, path::PathBuf};

pub trait PathBufLibraryResource: Resource + Debug {
    fn insert(&mut self, _value: PathBuf) -> bool {
        todo!();
    }
    fn contains(&self, _value: &PathBuf) -> bool {
        todo!();
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Default)]
pub struct FontsPathBufRes(HashSet<PathBuf>);
impl PathBufLibraryResource for FontsPathBufRes {
    fn insert(&mut self, value: PathBuf) -> bool {
        return self.0.insert(value);
    }

    fn contains(&self, value: &PathBuf) -> bool {
        return self.0.contains(value);
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Default)]
pub struct MapsPathBufRes(HashSet<PathBuf>);
impl PathBufLibraryResource for MapsPathBufRes {
    fn insert(&mut self, value: PathBuf) -> bool {
        return self.0.insert(value);
    }
    fn contains(&self, value: &PathBuf) -> bool {
        return self.0.contains(value);
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Default)]
pub struct ModelsPathBufRes(HashSet<PathBuf>);
impl PathBufLibraryResource for ModelsPathBufRes {
    fn insert(&mut self, value: PathBuf) -> bool {
        return self.0.insert(value);
    }
    fn contains(&self, value: &PathBuf) -> bool {
        return self.0.contains(value);
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Default)]
pub struct ImagesPathBufRes(HashSet<PathBuf>);
impl PathBufLibraryResource for ImagesPathBufRes {
    fn insert(&mut self, value: PathBuf) -> bool {
        return self.0.insert(value);
    }
    fn contains(&self, value: &PathBuf) -> bool {
        return self.0.contains(value);
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Default)]
pub struct AudioPathBufRes(HashSet<PathBuf>);
impl PathBufLibraryResource for AudioPathBufRes {
    fn insert(&mut self, value: PathBuf) -> bool {
        return self.0.insert(value);
    }
    fn contains(&self, value: &PathBuf) -> bool {
        return self.0.contains(value);
    }
}

#[derive(Resource, Default)]
pub struct RequiredAssets {
    pub models: Vec<Handle<Scene>>,
    pub model_tree: KdTree<f32, usize, 3, 32, u32>,
    pub model_locations: Vec<[f32; 3]>,
    pub audio: Vec<Handle<LoadedUntypedAsset>>,
    pub audio_tree: KdTree<f32, usize, 3, 32, u32>,
    pub audio_locations: Vec<[f32; 3]>,
    // images: HashSet<PathBuf>,
    // fonts: HashSet<PathBuf>,
}

impl RequiredAssets {
    pub fn with_capacities(
        models_len: usize,
        model_locations_len: usize,
        audio_len: usize,
        audio_locations_len: usize,
    ) -> Self {
        RequiredAssets {
            models: Vec::with_capacity(models_len),
            model_tree: KdTree::with_capacity(model_locations_len),
            model_locations: Vec::with_capacity(model_locations_len),
            audio: Vec::with_capacity(audio_len),
            audio_tree: KdTree::with_capacity(audio_locations_len),
            audio_locations: Vec::with_capacity(audio_locations_len),
        }
    }
}

#[derive(Resource, Default)]
pub struct MasterVolume(pub u16);

#[derive(Resource, Default)]
pub struct DisplayQuality(pub u16);

#[derive(Resource, Default)]
pub struct OrderFileLen(pub usize);

#[derive(Resource, Default)]
pub struct EntityPoolRes(pub Vec<Option<Entity>>);

impl EntityPoolRes {
    pub fn _with_capacity(capacity: usize) -> Self {
        return EntityPoolRes(Vec::with_capacity(capacity));
    }

    pub fn initialize(capacity: usize) -> Self {
        EntityPoolRes((0..capacity).map(|_| return None).collect())
    }
}
