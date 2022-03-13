use bevy::prelude::*;

mod inner;

fn main() {
    App::new()
        .add_plugin(inner::InnerPlugin)
        .add_system(print_stuff)
        .run();
}

fn print_stuff(params: inner::OpaqueParams) {
    inner::print_private(params);
}
