use bevy::{
    prelude::{Circle, *},
    sprite::MaterialMesh2dBundle,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

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

        .add_event::<TimerStart>()
        .add_event::<TimerEnd>()

        .add_systems(Startup, setup)
        .add_systems(PreUpdate, poll_for_button_press)
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

    // Timer text
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "0.0",
            TextStyle {
                font_size: 100.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    });
}

fn poll_for_button_press(
    mut timer_end: EventWriter<TimerEnd>,
    mut text: Query<&mut Text>,
    time: Res<Time>,
) {
    let mut text = text.single_mut();
    if text.sections[0].value == "0.0" {
        let button_pressed = unsafe {button_is_pressed()};
        if button_pressed {
            text.sections[0].value = "5.0".to_string();
        }
    } else {
        let since = time.delta_seconds();
        let timervalue = text.sections[0].value.parse::<f32>().unwrap();
        let new_value = timervalue - since;
        text.sections[0].value = new_value.to_string();

        if new_value <= 0.0 {
            timer_end.send(TimerEnd(0));
        }
    }
}

#[derive(Event)]
struct TimerStart(u32);

#[derive(Event)]
struct TimerEnd(u32);

// Function that can be called by rustwasm
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(raw_module="../../global.ts")]
extern "C" {
    fn button_is_pressed() -> bool;
}

#[cfg(not(target_arch = "wasm32"))]
fn button_is_pressed() -> bool {
    return false;
}