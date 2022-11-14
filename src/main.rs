use bevy::prelude::*;
use rand::Rng;
use std::cmp::{max, min};
use std::fmt;
use bevy::sprite::MaterialMesh2dBundle;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BACKGROUND_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

// Enum that will be used as a global state for the game
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

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Clone)]
struct Organism {
    id: i32,
    dna: Dna,
}

impl Organism {}

type RawDna = [u8; 10];

#[derive(Component, Clone)]
struct Dna {
    dna: RawDna,
}

impl Dna {
    fn get_bit_at(input: u8, n: u8) -> Option<bool> {
        if n < 8 {
            Some(input & (1 << n) != 0)
        } else {
            None
        }
    }
}

impl fmt::Display for Dna {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for int in self.dna {
            write!(f, "{},", int)?;
        }
        Ok(())
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
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle::default());

    for id in 0..100 {
        commands.spawn(Organism {
            id,
            dna: Dna {
                dna: rand::random::<RawDna>(),
            },
        });
    }

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });

}

#[derive(Resource)]
struct GreetTimer(Timer);

fn randomly_generate_dna() -> RawDna {
    rand::random::<RawDna>()
}

fn greet_organism(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Organism>) {
    if timer.0.tick(time.delta()).just_finished() {
        for organism in query.iter() {
            println!(
                "hello {}! With dna like this: {}",
                organism.id, organism.dna
            );
        }
    }
}

fn spawn_organism(mut commands: Commands, organism: Organism) {
    commands.spawn(organism);
}

fn hello_world() {
    println!("hello evolutionator!!");
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
