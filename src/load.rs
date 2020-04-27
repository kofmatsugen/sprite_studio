use crate::{
    resource::{data, AnimationStore},
    traits::translate_animation::TranslateAnimation,
};
use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter, RonFormat},
    ecs::{Read, ReadExpect, World, Write},
    renderer::{
        formats::texture::ImageFormat, sprite::SpriteSheet, sprite::SpriteSheetFormat,
        types::Texture,
    },
};

impl AnimationLoad for &mut World {
    // パス名を指定してロード
    fn load_animation_with_path<'s, F, T>(
        &mut self,
        id: T::FileId,
        dir_path: F, // アニメーションファイルのあるディレクトリパス指定
        progress: &mut ProgressCounter,
    ) where
        F: Into<String>,
        T: TranslateAnimation<'s>,
    {
        self.exec(
            |(mut store, loader, storage): (
                Write<AnimationStore<T>>,
                ReadExpect<Loader>,
                Read<AssetStorage<data::AnimationData<T>>>,
            )| {
                let dir_path = dir_path.into();
                let path = format!("sprite_studio/{}/animation/animation.anim.ron", dir_path);
                log::info!("load animation: {:?}", path);
                let handle = loader.load(path, RonFormat, progress, &storage);

                store.animations.insert(id, handle);
            },
        );
    }

    // パス名を指定してロード
    fn load_sprite_with_path<'s, F, T>(
        &mut self,
        id: T::FileId,
        dir_path: F,
        sprite_sheet_num: usize,
        progress: &mut ProgressCounter,
    ) where
        F: Into<String>,
        T: TranslateAnimation<'s>,
    {
        self.exec(
            |(mut store, loader, tex_storage, sprite_storage): (
                Write<AnimationStore<T>>,
                ReadExpect<Loader>,
                Read<AssetStorage<Texture>>,
                Read<AssetStorage<SpriteSheet>>,
            )| {
                let dir_path = dir_path.into();
                let mut sheets = vec![];
                for i in 0..sprite_sheet_num {
                    let sprite_path =
                        format!("sprite_studio/{}/image/sprite{:03}.png", dir_path, i);
                    let sheet_path =
                        format!("sprite_studio/{}/sheet/sprite{:03}.sheet.ron", dir_path, i);

                    log::info!("load sprite: {:?}", sprite_path);
                    log::info!("load sheet: {:?}", sheet_path);

                    let texture = loader.load(
                        sprite_path,
                        ImageFormat::default(),
                        &mut *progress,
                        &tex_storage,
                    );
                    let sheet = loader.load(
                        sheet_path,
                        SpriteSheetFormat(texture),
                        &mut *progress,
                        &sprite_storage,
                    );
                    sheets.push(sheet);
                }

                store.sprite_sheets.insert(id, sheets);
            },
        );
    }
}

pub trait AnimationLoad {
    // パス名を指定してロード
    fn load_animation_with_path<'s, F, T>(
        &mut self,
        id: T::FileId,
        dir_path: F, // アニメーションファイルのあるディレクトリパス指定
        progress: &mut ProgressCounter,
    ) where
        F: Into<String>,
        T: TranslateAnimation<'s>;

    // パス名を指定してロード
    fn load_sprite_with_path<'s, F, T>(
        &mut self,
        id: T::FileId,
        dir_path: F,
        sprite_sheet_num: usize,
        _progress: &mut ProgressCounter,
    ) where
        F: Into<String>,
        T: TranslateAnimation<'s>;

    fn load_animation<'s, T>(&mut self, id: T::FileId, progress: &mut ProgressCounter)
    where
        T: TranslateAnimation<'s>,
    {
        let file_name = T::to_file_name(&id);
        log::info!("load {}", file_name);
        self.load_animation_with_path::<_, T>(id, file_name, progress);
    }

    fn load_sprite_sheet<'s, T>(&mut self, id: T::FileId, progress: &mut ProgressCounter)
    where
        T: TranslateAnimation<'s>,
    {
        let file_name = T::to_file_name(&id);
        let num = T::sprite_sheet_num(&id);
        log::info!("load {} of num {}", file_name, num);
        self.load_sprite_with_path::<_, T>(id, file_name, num, progress);
    }

    fn load_animation_files<'s, T>(&mut self, id: T::FileId, progress: &mut ProgressCounter)
    where
        T: TranslateAnimation<'s>,
    {
        self.load_animation::<T>(id, progress);
        self.load_sprite_sheet::<T>(id, progress);
    }
}
