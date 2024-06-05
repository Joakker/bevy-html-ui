use bevy::prelude::*;

mod event;
mod node;

pub use event::SpawnHtml;

pub struct HtmlUiPlugin;

impl Plugin for HtmlUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnHtml>()
            .add_systems(Update, event::spawn_ui.run_if(on_event::<SpawnHtml>()));
    }
}
