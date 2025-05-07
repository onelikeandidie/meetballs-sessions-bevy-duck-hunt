use bevy::{prelude::*, window::WindowResolution};
use lib::SimpleGamePlugin;

pub mod lib;

// Recomend going with a multiple of 2
const WINDOW_SCALE: f32 = 2.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                // Set the window resolution to match the NES
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Duck Hunt".into(),
                        name: Some("meetballs.duckhunt.client".into()),
                        resolution: WindowResolution::new(
                            256.0 * WINDOW_SCALE,
                            240.0 * WINDOW_SCALE,
                        )
                        .with_scale_factor_override(WINDOW_SCALE),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                // Set to nearest scaling, this will make our pixels crispy
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(SimpleGamePlugin)
        .run();
}
