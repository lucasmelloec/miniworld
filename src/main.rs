use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

struct GreetTimer(Timer);

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)
            .add_system(greet_people);
    }
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name(String::from("Lucas")));
    commands
        .spawn()
        .insert(Person)
        .insert(Name(String::from("Fernanda")));
    commands
        .spawn()
        .insert(Person)
        .insert(Name(String::from("Honhon")));
    commands
        .spawn()
        .insert(Person)
        .insert(Name(String::from("Lady")));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0)
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

