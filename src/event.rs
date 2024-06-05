use bevy::prelude::*;
use html_parser_rscx::Dom;

use crate::{node::spawn_node, resource::TextFlags, UiFonts};

#[derive(Event)]
pub struct SpawnHtml {
    pub contents: String,
    pub root: Option<Entity>,
}

pub fn spawn_ui(
    mut commands: Commands,
    mut event: EventReader<SpawnHtml>,
    asset_server: Res<AssetServer>,
    fonts: Option<Res<UiFonts>>,
) {
    event.read().for_each(|SpawnHtml { contents, root }| {
        let Ok(Dom { children, .. }) = Dom::parse(contents) else {
            error!("Error parsing html:\n{contents}");
            return;
        };

        let root = root.unwrap_or_else(|| commands.spawn_empty().id());

        let Some(root_node) = children.first() else {
            error!("Html file is empty!");
            return;
        };

        spawn_node(
            root_node,
            &mut commands,
            root,
            &asset_server,
            &fonts,
            TextFlags::NORMAL,
        );
    });
}
