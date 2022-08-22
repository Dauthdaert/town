use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_tilemap::prelude::*;
use bevy_turborand::RngPlugin;

pub const LAUNCHER_TITLE: &str = "Town";

mod camera;
mod map_gen;

pub fn app() -> App {
    let mut app = App::new();

    //Add bevy resources
    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .insert_resource(ImageSettings::default_nearest());

    //Add bevy and dependency plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(RngPlugin::default());

    //Add custom plugins
    app.add_plugin(camera::CameraPlugin);
    app.add_plugin(map_gen::MapGenPlugin);

    //Add custom resources and systems

    app
}
