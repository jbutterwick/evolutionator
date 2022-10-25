use bevy::prelude::*;
use rand::Rng;
use std::cmp::{max, min};

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Splash,
    Menu,
    Game,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
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
    dna: [u8; 10],
}

struct Entity(u64);

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

fn add_organism(mut commands: Commands) {
    for id in num {
        commands.spawn().insert(Organism {
            id,
            dna: rand::random::<[u8; 10]>(),
        });
    }
}

struct GreetTimer(Timer);

fn randomly_generate_dna() -> [u8; 10] {
    rand::random::<[u8; 10]>()
}

fn greet_organism(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Organism>) {
    if timer.0.tick(time.delta()).just_finished() {
        for organism in query.iter() {
            println!("hello {}!", organism.id);
        }
    }
}

fn get_bit_at(input: u8, n: u8) -> Option<bool> {
    if n < 8 {
        Some(input & (1 << n) != 0)
    } else {
        None
    }
}

// oh sexy
fn sexual_reproduction(mut commands: Commands, mother: &Organism, father: &Organism) {
    if mother.dna.multiplicative && father.dna.multiplicative {}
    commands.spawn().insert(Organism {
        id,
        dna: mother
            .dna
            .into_iter()
            .flat_map(|gene1| {
                father.dna.into_iter().flat_map(|gene2| {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(min(gene1, gene2)..max(gene1, gene2))
                })
            })
            .collect(),
    });
}

fn asexual_reproduction(mut commands: Commands, organism: &Organism, num: i32) {
    for id in 0..num {
        commands.spawn().insert(Organism {
            id,
            dna: organism.dna.clone(),
        });
    }
}

fn hello_world() {
    println!("hello evolutionator!!");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_organism)
            .add_system(greet_organism);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
