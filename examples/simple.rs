use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use wraithlib::client::splash::{SplashImageProperties, SplashPlugin};
use wraithlib::client::ClientPlugins;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    level: Level::DEBUG,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "WraithLibrary [ Simple Preview ]".into(),
                        resolution: (1280.0, 720.0).into(),
                        resizable: true,
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(ClientPlugins.set(SplashPlugin {
            images: vec![SplashImageProperties {
                path: "images/wraithaven.png".into(),
                ..default()
            }],
            ..default()
        }))
        .run();
}