use bevy::{
    prelude::{Circle, *},
    sprite::MaterialMesh2dBundle,
};

fn main() {
    let mut window_plugin = WindowPlugin::default();
    window_plugin.primary_window = Some(Window {
        fit_canvas_to_parent: true,
        title: "Game".to_string(),
        canvas: Some("#game_canvas".to_string()), // This is the important part
        ..Default::default()
    });
    let plugins = DefaultPlugins.set(window_plugin);
    App::new()
        .add_plugins(plugins)

        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Circle::new(50.)).into(),
        material: materials.add(Color::from(LinearRgba::RED)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });
}
