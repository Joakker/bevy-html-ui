use bevy::prelude::*;
use bevy_ecss::prelude::*;
use bevy_html_ui::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, EcssPlugin::default(), HtmlUiPlugin))
        .add_systems(
            Startup,
            |mut commands: Commands, mut events: EventWriter<SpawnHtml>| {
                commands.spawn(Camera3dBundle::default());

                let contents =
                    std::fs::read_to_string("assets/example.html").expect("Cannot read html file");
                events.send(SpawnHtml {
                    contents,
                    root: None,
                });
            },
        );

    app.run()
}
