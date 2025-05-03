use bevy::prelude::*;

const TILE_SIDE_SIZE: u32 = 8;
const TILE_COLUMNS: u32 = 15;
const TILE_ROWS: u32 = 10;

pub struct SpritesLoader;
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[repr(usize)]
pub enum SpriteState {
    FRONT = 0,
    SIDE,
    JUMP,
    FALL,
}

pub struct Atlas {
    pub image: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
}
#[derive(Resource)]
pub struct Sprites {
    pub player: Atlas,
}

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.insert_resource(Sprites {
        player: load_player_sprite(asset_server, texture_atlas_layouts),
    });
    // let texture: Handle<Image> = asset_server.load("assets/pico/tilemaps/player.png");
    // let layout =
    //     TextureAtlasLayout::from_grid(UVec2::splat(8), , TILE_ROWS, None, None);
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);
}

fn load_player_sprite(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> Atlas {
    let texture: Handle<Image> = asset_server.load("pico/tilemaps/player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(8), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    return Atlas {
        image: texture,
        atlas: texture_atlas_layout,
    };
}
