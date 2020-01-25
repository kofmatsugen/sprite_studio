pub mod animation;
pub mod data;
pub mod name;
pub mod pack;
pub mod part;
mod part_timeline;
pub mod timeline;

use crate::traits::{animation_file::AnimationFile, AnimationUser, FileId};
use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    ecs::{Read, ReadExpect, World, Write},
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        Texture,
    },
};
use std::collections::BTreeMap;

pub type AnimationHandle<U> = Handle<data::AnimationData<U>>;
pub struct AnimationStore<ID, U>
where
    ID: FileId,
{
    animations: BTreeMap<ID, AnimationHandle<U>>,
    sprite_sheets: BTreeMap<ID, Vec<SpriteSheetHandle>>,
}

impl<ID, U> Default for AnimationStore<ID, U>
where
    ID: FileId,
    U: AnimationUser,
{
    fn default() -> Self {
        AnimationStore {
            animations: BTreeMap::new(),
            sprite_sheets: BTreeMap::new(),
        }
    }
}

impl WorldExt for &mut World {
    // パス名を指定してロード
    fn load_animation_with_path<F, ID, U>(
        &mut self,
        id: ID,
        dir_path: F, // アニメーションファイルのあるディレクトリパス指定
        progress: &mut ProgressCounter,
    ) where
        ID: FileId + AnimationFile,
        U: AnimationUser,
        F: Into<String>,
    {
        self.exec(
            |(mut store, loader, storage): (
                Write<AnimationStore<ID, U>>,
                ReadExpect<Loader>,
                Read<AssetStorage<data::AnimationData<U>>>,
            )| {
                let dir_path = dir_path.into();
                let path = format!("{}/animation/animation.anim.ron", dir_path);
                log::info!("load animation: {:?}", path);
                let handle = loader.load(path, RonFormat, progress, &storage);

                store.animations.insert(id, handle);
            },
        );
    }

    // パス名を指定してロード
    fn load_sprite_with_path<F, ID, U>(
        &mut self,
        id: ID,
        dir_path: F,
        sprite_sheet_num: usize,
        _progress: &mut ProgressCounter,
    ) where
        ID: FileId + AnimationFile,
        U: AnimationUser,
        F: Into<String>,
    {
        self.exec(
            |(mut store, loader, tex_storage, sprite_storage): (
                Write<AnimationStore<ID, U>>,
                ReadExpect<Loader>,
                Read<AssetStorage<Texture>>,
                Read<AssetStorage<SpriteSheet>>,
            )| {
                let dir_path = dir_path.into();
                let mut sheets = vec![];
                for i in 0..sprite_sheet_num {
                    let sprite_path = format!("{}/image/sprite{:03}.png", dir_path, i);
                    let sheet_path = format!("{}/sheet/sprite{:03}.sheet.ron", dir_path, i);

                    log::info!("load sprite: {:?}", sprite_path);
                    log::info!("load sheet: {:?}", sheet_path);

                    let texture =
                        loader.load(sprite_path, ImageFormat::default(), (), &tex_storage);
                    let sheet =
                        loader.load(sheet_path, SpriteSheetFormat(texture), (), &sprite_storage);
                    sheets.push(sheet);
                }

                store.sprite_sheets.insert(id, sheets);
            },
        );
    }
}

pub trait WorldExt {
    // パス名を指定してロード
    fn load_animation_with_path<F, ID, U>(
        &mut self,
        id: ID,
        dir_path: F, // アニメーションファイルのあるディレクトリパス指定
        progress: &mut ProgressCounter,
    ) where
        ID: FileId + AnimationFile,
        U: AnimationUser,
        F: Into<String>;

    // パス名を指定してロード
    fn load_sprite_with_path<F, ID, U>(
        &mut self,
        id: ID,
        dir_path: F,
        sprite_sheet_num: usize,
        _progress: &mut ProgressCounter,
    ) where
        ID: FileId + AnimationFile,
        U: AnimationUser,
        F: Into<String>;

    fn load_animation<ID, U>(&mut self, id: ID, progress: &mut ProgressCounter)
    where
        ID: FileId + AnimationFile,
        U: AnimationUser,
    {
        let file_name = id.to_file_name();
        log::info!("load {}", file_name);
        self.load_animation_with_path::<_, ID, U>(id, file_name, progress);
    }

    fn load_sprite_sheet<ID, U>(&mut self, id: ID, progress: &mut ProgressCounter)
    where
        ID: FileId + AnimationFile,
        U: AnimationUser,
    {
        let file_name = id.to_file_name();
        let num = id.sprite_sheet_num();
        log::info!("load {} of num {}", file_name, num);
        self.load_sprite_with_path::<_, ID, U>(id, file_name, num, progress);
    }

    fn load_animation_files<ID, U>(&mut self, id: ID, progress: &mut ProgressCounter)
    where
        ID: FileId + AnimationFile,
        U: AnimationUser,
    {
        self.load_animation::<ID, U>(id, progress);
        self.load_sprite_sheet::<ID, U>(id, progress);
    }
}

impl<ID, U> AnimationStore<ID, U>
where
    ID: FileId,
    U: AnimationUser,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_animation_handle(&self, id: &ID) -> Option<&AnimationHandle<U>> {
        self.animations.get(id)
    }

    pub fn get_sprite_handle(&self, id: &ID, map_id: usize) -> Option<&SpriteSheetHandle> {
        self.sprite_sheets
            .get(id)
            .and_then(|sprite_sheets| sprite_sheets.get(map_id))
    }
}
