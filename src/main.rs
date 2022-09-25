use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use std::cmp::{max, min};

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component, Clone)]
struct Organism {
    dna: [u8; 10],
}

struct Entity(u64);

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

fn add_organism(mut commands: Commands) {
    commands.spawn().insert(Organism {
        dna: rand::random::<[u8; 10]>(),
    });
    commands.spawn().insert(Organism {
        dna: rand::random::<[u8; 10]>(),
    });
    commands.spawn().insert(Organism {
        dna: rand::random::<[u8; 10]>(),
    });
}

struct GreetTimer(Timer);

fn randomly_generate_dna() -> [u8; 10] {
    rand::random::<[u8; 10]>()
}

fn greet_organism(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Organism>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}
// oh sexy
fn sexual_reproduction(mother: &Organism, father: &Organism) -> Organism {
    let dna = mother
        .dna
        .iter()
        .flat_map(|gene1| {
            father.dna.iter().flat_map(|gene2| {
                let mut rng = rand::thread_rng();
                rng.gen_range(min(gene1, gene2)..max(gene1, gene2))
            })
        })
        .collect();

    Organism { dna }
}

fn asexual_reproduction(organism: &Organism) -> Organism {
    organism.clone()
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
