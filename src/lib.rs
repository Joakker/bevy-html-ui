use bevy::prelude::*;

mod event;
mod node;
mod resource;

pub use {event::SpawnHtml, resource::UiFonts};

#[derive(Clone, Default)]
pub struct HtmlUiPlugin;

impl Plugin for HtmlUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnHtml>()
            .add_systems(Update, event::spawn_ui.run_if(on_event::<SpawnHtml>()));
    }
}
