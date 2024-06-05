use bevy::prelude::*;
use bevy_ecss::{Class, StyleSheet};
use html_parser_rscx::{Element, Node};

use crate::{resource::TextFlags, UiFonts};

pub fn spawn_node(
    node: &Node,
    commands: &mut Commands,
    entity: Entity,
    asset_server: &AssetServer,
    fonts: &Option<Res<UiFonts>>,
    text_flags: TextFlags,
) {
    match node {
        Node::Text(text) => spawn_text(text, commands, entity, fonts, text_flags),
        Node::Element(element) => {
            spawn_element(element, commands, entity, asset_server, fonts, text_flags)
        }
        Node::Comment(_) => {}
    }
}

pub fn spawn_text(
    text: &str,
    commands: &mut Commands,
    entity: Entity,
    fonts: &Option<Res<UiFonts>>,
    text_flags: TextFlags,
) {
    commands.entity(entity).insert(TextBundle::from_section(
        text.trim_matches(|c| matches!(c, '\n' | '\r')).to_owned(),
        TextStyle {
            font: fonts
                .as_ref()
                .map(|fonts| match text_flags {
                    TextFlags::NORMAL => fonts.normal.clone(),
                    TextFlags::BOLD => fonts.bold.clone(),
                    TextFlags::ITALIC => fonts.italic.clone(),
                    TextFlags::BOLD_ITALIC => fonts.bold_italic.clone(),
                    _ => unreachable!(),
                })
                .unwrap_or_default(),
            ..Default::default()
        },
    ));
}

pub fn spawn_element(
    Element {
        id,
        name,
        variant: _,
        attributes,
        classes,
        children,
        source_span: _,
    }: &Element,
    commands: &mut Commands,
    entity: Entity,
    asset_server: &AssetServer,
    fonts: &Option<Res<UiFonts>>,
    text_flags: TextFlags,
) {
    let mut text_flags = text_flags;
    {
        let mut entity = commands.entity(entity);

        match name.as_str() {
            "img" => {
                if let Some(path) = attributes.get("src").cloned().flatten() {
                    entity.insert(ImageBundle {
                        image: asset_server.load(path).into(),
                        ..Default::default()
                    });
                } else {
                    error!("Image tag lacks source");
                }
            }
            "b" => {
                text_flags |= TextFlags::BOLD;
            }
            "i" => {
                text_flags |= TextFlags::ITALIC;
            }
            "div" => {
                entity.insert(NodeBundle::default());

                match (
                    attributes.get("bg").cloned().flatten(),
                    attributes.get("margin").cloned().flatten(),
                ) {
                    (Some(path), Some(margin)) => {
                        entity.insert(ImageBundle {
                            image: asset_server.load(path).into(),
                            ..Default::default()
                        });

                        if margin.is_empty() {
                            // Do nothing
                        } else if let Ok(margin) = margin.parse::<f32>() {
                            entity.insert(ImageScaleMode::Sliced(TextureSlicer {
                                border: BorderRect::square(margin),
                                ..Default::default()
                            }));
                        } else {
                            error!("Attribute `margin` must be an f32 or empty string");
                        }
                    }
                    (None, Some(_)) => {
                        error!("Attributes `bg` and `margin` must be defined together")
                    }
                    (Some(_), None) => {
                        error!("Attributes `bg` and `margin` must be defined together")
                    }
                    _ => (),
                }
            }
            e => return error!("Unknown tag: {e}"),
        }

        if let Some(path) = attributes.get("stylesheet").cloned().flatten() {
            entity.insert(StyleSheet::new(asset_server.load(path)));
        }

        if let Some(id) = id {
            entity.insert(Name::new(id.to_owned()));
        }

        if !classes.is_empty() {
            entity.insert(Class::new(classes.join(" ")));
        }
    }

    spawn_children(children, commands, entity, asset_server, fonts, text_flags);
}

pub fn spawn_children(
    children: &[Node],
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
    fonts: &Option<Res<UiFonts>>,
    text_flags: TextFlags,
) {
    let mut c = vec![];

    commands.entity(parent).with_children(|commands| {
        children
            .iter()
            .filter(|node| {
                if let Node::Text(text) = node {
                    !text.trim().is_empty()
                } else {
                    matches!(node, Node::Element(_))
                }
            })
            .for_each(|node| {
                c.push((commands.spawn(NodeBundle::default()).id(), node));
            });
    });

    for (child, node) in c {
        spawn_node(node, commands, child, asset_server, fonts, text_flags);
    }
}
