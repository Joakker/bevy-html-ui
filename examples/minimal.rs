use bevy::prelude::*;
use bevy_ecss::prelude::*;
use bevy_html_ui::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, EcssPlugin::default(), HtmlUiPlugin))
        .add_systems(Startup, (setup, apply_deferred, spawn).chain());

    app.run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    macro_rules! load_ubuntu {
        ($a:literal) => {
            asset_server.load(format!("UbuntuMono-{}.ttf", $a))
        };
    }
    commands.insert_resource(UiFonts {
        normal: load_ubuntu!("R"),
        bold: load_ubuntu!("B"),
        italic: load_ubuntu!("I"),
        bold_italic: load_ubuntu!("BI"),
    });

    commands.spawn(Camera3dBundle::default());
}

fn spawn(mut events: EventWriter<SpawnHtml>) {
    let contents = std::fs::read_to_string("assets/example.html").expect("Cannot open html file");

    events.send(SpawnHtml {
        contents,
        root: None,
    });
}
