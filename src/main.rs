use bevy::prelude::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component, Clone, Copy)]
struct Organism { name: Name, dna: [u8;10] }

#[derive(Component)]
struct Name(String);

struct Entity(u64);

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

fn add_organism(mut commands: Commands) {
    commands.spawn().insert(Organism{ name: ("Elaina Proctor".to_string()})));
    commands.spawn().insert(Organism{ name: ("Renzo Hume".to_string()})));
    commands.spawn().insert(Organism{ name: ("Zayna Nieves".to_string()})));
}

struct GreetTimer(Timer);

fn greet_organism(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Organism>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

fn sexual_reproduction(mother: &Organism, father: &Organism) -> Organism {
    let dna = for gene1,gene2 in mother.dna, father.dna
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

