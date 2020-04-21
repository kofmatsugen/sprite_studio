// スプラッシュ画像を表示して次のステートに移動する処理実装
use super::{
    id::{AnimationKey, FileId, PackKey},
    translation::SplashTranslation,
};
use crate::{
    components::{AnimationTime, PlayAnimationKey},
    load::AnimationLoad,
    resource::{data::AnimationData, AnimationStore},
    system::{AnimationTransitionSystem, BuildNodesSystem},
    types::event::{AnimationEvent, AnimationEventChannel},
};
use amethyst::{
    assets::{Processor, ProgressCounter},
    core::transform::Transform,
    core::ArcThreadPool,
    ecs::{Builder, Dispatcher, DispatcherBuilder, Entity, Read, ReaderId, WorldExt, Write},
    renderer::{camera::Camera, ActiveCamera},
    shred::World,
    window::ScreenDimensions,
    GameData, SimpleState, SimpleTrans, StateData,
};

pub struct SplashState<'a, 'b, T: SimpleState> {
    progress_counter: ProgressCounter,
    setuped: bool,
    next_state: std::marker::PhantomData<T>,
    camera_entity: Option<Entity>,
    splash_entity: Option<Entity>,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    reader: Option<ReaderId<AnimationEvent<SplashTranslation>>>,
}

impl<'a, 'b, T> SplashState<'a, 'b, T>
where
    T: SimpleState,
{
    pub fn new() -> Self {
        SplashState {
            progress_counter: ProgressCounter::default(),
            setuped: false,
            next_state: std::marker::PhantomData,
            camera_entity: None,
            splash_entity: None,
            dispatcher: None,
            reader: None,
        }
    }
}

impl<'a, 'b, T> SimpleState for SplashState<'a, 'b, T>
where
    T: 'static + SimpleState + Default,
{
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world.load_animation_files::<SplashTranslation>(
            FileId::SpriteStudioSplash,
            &mut self.progress_counter,
        );

        self.camera_entity = initialise_camera(&mut data.world).into();
        self.dispatcher = setup_dispatcher(&mut data.world).into();
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if self.progress_counter.is_complete() == true && self.setuped == false {
            self.splash_entity = create_splash(&mut data.world).into();
            self.setuped = true;
        }

        if self.reader.is_none() == true {
            self.reader = data
                .world
                .exec(
                    |mut channel: Write<AnimationEventChannel<SplashTranslation>>| {
                        channel.register_reader()
                    },
                )
                .into();
        }

        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

        let end_splash =
            data.world
                .exec(|channel: Read<AnimationEventChannel<SplashTranslation>>| {
                    match (self.reader.as_mut(), self.splash_entity) {
                        (Some(reader), Some(splash_entity)) => {
                            is_end_splash(splash_entity, reader, &channel)
                        }
                        _ => false,
                    }
                });

        if end_splash == true {
            // 次ステートに移動するときはスプラッシュ画像を開放，エンティティも破棄
            let _ = data.world.delete_entity(self.splash_entity.unwrap());
            let _ = data.world.delete_entity(self.camera_entity.unwrap());
            data.world
                .exec(|mut store: Write<AnimationStore<SplashTranslation>>| {
                    store.unload_file(&FileId::SpriteStudioSplash);
                });
            let dispatcher = self.dispatcher.take();
            dispatcher.map(|d| d.dispose(data.world));
            SimpleTrans::Switch(Box::new(T::default()))
        } else {
            SimpleTrans::None
        }
    }
}

fn is_end_splash(
    splash_entity: Entity,
    reader: &mut ReaderId<AnimationEvent<SplashTranslation>>,
    channel: &AnimationEventChannel<SplashTranslation>,
) -> bool {
    channel
        .read(reader)
        .find(|event| match event {
            AnimationEvent::End { entity, .. } => *entity == splash_entity,
            _ => false,
        })
        .is_some()
}

fn create_splash(world: &mut World) -> Entity {
    let mut anim_key = PlayAnimationKey::<SplashTranslation>::new(FileId::SpriteStudioSplash);
    anim_key.set_pack(PackKey::SpriteStudioSplash);
    anim_key.set_animation(AnimationKey::SplashInOut);
    let mut anim_time = AnimationTime::new();
    anim_time.set_speed(1.0);

    world
        .create_entity()
        .with(anim_key)
        .with(anim_time)
        .with(Transform::default())
        .build()
}

fn initialise_camera(world: &mut World) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut camera_transform = Transform::default();
    camera_transform.set_translation_z(1024.0);

    let camera = world
        .create_entity()
        .with(camera_transform)
        .with(Camera::standard_2d(width, height))
        .build();

    world.insert(ActiveCamera {
        entity: Some(camera),
    });

    camera
}

fn setup_dispatcher<'a, 'b>(world: &mut World) -> Dispatcher<'a, 'b> {
    let mut builder = DispatcherBuilder::new();

    builder.add(
        Processor::<AnimationData<SplashTranslation>>::new(),
        "splash_processor",
        &[],
    );
    builder.add(
        AnimationTransitionSystem::<SplashTranslation>::new(),
        "splash_translation",
        &[],
    );
    builder.add(
        BuildNodesSystem::<SplashTranslation>::new(),
        "splash_build",
        &[],
    );

    let mut dispatcher = builder
        .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
        .build();
    dispatcher.setup(world);

    dispatcher
}
