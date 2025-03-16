use serde::{Deserialize};
use std::collections::HashMap;
use bevy::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Deserialize,Debug)]
pub struct SeiSprite {
    pub name: String,
    pub i: u32,
    pub j: u32,
    pub x0: u32,
    pub y0: u32,
    pub x1: u32,
    pub y1: u32
}

#[derive(Deserialize,Debug)]
pub struct SeiVec2 {
    pub x: i64,
    pub y: i64,
}

#[derive(Deserialize,Debug)]
pub struct SeiSpritesheet {
    pub image: String,
    pub offset: SeiVec2,
    pub padding: SeiVec2,
    pub spriteSize: SeiVec2,
    pub mappings: HashMap<String, SeiSprite>
}

#[derive(Default, Debug)]
pub struct Asset {
    pub mappings: HashMap<String, usize>,
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}

#[derive(Default, Debug, Resource)]
pub struct AssetLibrary {
    pub assets: HashMap<String, Asset>,
}

fn load_assets(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let path = Path::new("assets/spritesheet.json");

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    // Deserialize the JSON into our struct
    let library: HashMap<String, SeiSpritesheet> = serde_json::from_reader(reader).unwrap();
    let mut asset_lib = AssetLibrary::default();

    for (name, ss) in library.iter() {
        let base64_image = if let Some(stripped) = ss.image.strip_prefix("data:image/png;base64,") {
            stripped
        } else {
            ss.image.as_str()
        };

        let buffer = BASE64.decode(&base64_image).unwrap();
        let image = Image::from_buffer(
            &buffer,
            bevy::image::ImageType::Extension("png"),
            bevy::image::CompressedImageFormats::NONE,
            false,
            bevy::image::ImageSampler::Default,
            bevy::asset::RenderAssetUsages::RENDER_WORLD | bevy::asset::RenderAssetUsages::MAIN_WORLD,
        ).unwrap();

        let img_handle = images.add(image);
        let mut texture_atlas = TextureAtlasBuilder::default()
            .build();
        let mut layout = TextureAtlasLayout::new_empty(UVec2::new(1000, 1000));
        let mut mappings: HashMap<String, usize> = HashMap::new();

        for (mapping_name, sprite) in ss.mappings.iter() {
            let sprite_index = layout.add_texture(URect::new(sprite.x0, sprite.y0, sprite.x1, sprite.y1));
            mappings.insert(mapping_name.clone(), sprite_index);
        }

        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let asset = Asset { mappings: mappings.clone(), layout: texture_atlas_layout.clone(), image: img_handle.clone() };
        asset_lib.assets.insert(name.clone(), asset);

        // commands.spawn((
        //     Sprite::from_atlas_image(
        //         img_handle,
        //         TextureAtlas {
        //             layout: texture_atlas_layout,
        //             index: mappings["fulldots"],
        //         },
        //     ),
        //     Transform {
        //         translation: Vec3::new(0., 0., -1.),
        //         scale: Vec3::splat(2.0),
        //         ..default()
        //     },
        // ));
    }

    println!("{:?}", asset_lib);
    commands.insert_resource(asset_lib);
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
   fn build(&self, app: &mut App) {
       app.add_systems(Startup, load_assets);
   }
}
