mod assets;
mod organism;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use organism::*;
use rand::{thread_rng, Rng};

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    Game,
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

const X_MAX: f32 = 1920.00;
const Y_MAX: f32 = 1080.00;

#[derive(Component, Clone)]
struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub(crate) fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub(crate) fn random() -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(0.0..X_MAX),
            y: rng.gen_range(0.0..Y_MAX),
        }
    }
}

struct Entity(u64);

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    for id in 0..100 {
        let organism = Organism::new(id);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
            ..default()
        });
        commands.spawn(organism);
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_organism(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Organism>) {
    if timer.0.tick(time.delta()).just_finished() {
        for organism in query.iter() {
            println!(
                "hello organism #{}! You've been created with dna that looks like this: {}",
                organism.id, organism.dna
            );
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Once)))
            .add_startup_system(setup)
            .add_system(greet_organism);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
