mod plugins;
pub mod map;
use bevy::log::LogPlugin;
use bevy::prelude::*;

const WINDOW_TITLE: &str = "ED Glass";

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins
        .build()
        .disable::<LogPlugin>()
        .set(WindowPlugin {
            primary_window:  Some(Window {
                title: WINDOW_TITLE.into(),
                ..default()
            }),
            ..default()
        })
    )
    .insert_resource(ClearColor(Color::BLACK));

    // #[cfg(feature = "dev")]
    // {
    //     use bevy::{
    //         diagnostic::{
    //             FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
    //             SystemInformationDiagnosticsPlugin,
    //         },
    //         render::diagnostic::RenderDiagnosticsPlugin,
    //     };
    //     app.add_plugins(bevy_editor_pls::EditorPlugin::default())
    //         .add_plugins((
    //             FrameTimeDiagnosticsPlugin,
    //             LogDiagnosticsPlugin::default(),
    //             SystemInformationDiagnosticsPlugin,
    //             RenderDiagnosticsPlugin,
    //         ));
    // }
}
