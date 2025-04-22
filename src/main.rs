use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn hello_world() {
    println!("hello world");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Vaughn Woerpel".to_string())));
    commands.spawn((Person, Name("Ian Le".to_string())));
    commands.spawn((Person, Name("Asher Saunders".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}", name.0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people).chain())
        .run();
}
