use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

pub struct InnerPlugin;
impl Plugin for InnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

// if you mark this as `pub`, the example compiles and runs fine, but exposes an implementation detail.
#[derive(Component)]
struct Private(String);

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(Private("this is private".to_string()));
    commands
        .spawn()
        .insert(Private("this is also private".to_string()));
}

use bevy::ecs::system::lifetimeless::Read;
#[derive(SystemParam)]
pub struct OpaqueParams<'w, 's> {
    q: Query<'w, 's, Read<Private>>,
    // ...other params, etc.
}

// prints the value of private components
pub fn print_private(OpaqueParams { q }: OpaqueParams) {
    for p in q.iter() {
        println!("{}", p.0);
    }
}
