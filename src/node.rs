use bevy::prelude::*;
use bevy_ecss::{Class, StyleSheet};
use html_parser_rscx::{Element, Node};

pub fn spawn_node(
    node: &Node,
    commands: &mut Commands,
    entity: Entity,
    asset_server: &AssetServer,
) {
    match node {
        Node::Text(text) => spawn_text(text, commands, entity),
        Node::Element(element) => spawn_element(element, commands, entity, asset_server),
        Node::Comment(_) => {}
    }
}

pub fn spawn_text(text: &String, commands: &mut Commands, entity: Entity) {
    commands.entity(entity).insert(TextBundle::from_section(
        text.to_owned(),
        TextStyle::default(),
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
) {
    {
        let mut entity = commands.entity(entity);

        if "img" == name.as_str() {
            if let Some(path) = attributes.get("src").cloned().flatten() {
                entity.insert(ImageBundle {
                    image: asset_server.load(path).into(),
                    ..Default::default()
                });
            } else {
                error!("Image tag lacks source");
            }
        } else {
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
                (None, Some(_)) => error!("Attributes `bg` and `margin` must be defined together"),
                (Some(_), None) => error!("Attributes `bg` and `margin` must be defined together"),
                _ => (),
            }
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
    spawn_children(children, commands, entity, asset_server);
}

pub fn spawn_children(
    children: &[Node],
    commands: &mut Commands,
    parent: Entity,
    asset_server: &AssetServer,
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
                c.push((commands.spawn_empty().id(), node));
            });
    });

    for (child, node) in c {
        spawn_node(node, commands, child, asset_server);
    }
}
