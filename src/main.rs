use bevy::prelude::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Organism;

#[derive(Component)]
struct Name(String);

fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

fn add_organism(mut commands: Commands) {
    commands.spawn().insert(Organism).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Organism).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Organism).insert(Name("Zayna Nieves".to_string()));
}

fn greet_organism(query: Query<&Name, With<Organism>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}

struct Entity(u64);

fn hello_world() {
    println!("hello evolutionator!!");
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_organism)
            .add_system(hello_world)
            .add_system(greet_organism);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

