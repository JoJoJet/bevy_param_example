#![feature(type_alias_impl_trait)]
use bevy::prelude::*;

mod inner {
    use super::*;

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
    pub struct OpaqueParams<'w, 's> {
        q: Query<'w, 's, Read<Private>>,
        // ...other params, etc...
    }
    #[doc(hidden)]
    pub type OpaqueFetch = impl for<'w, 's> bevy::ecs::system::SystemParamFetch<'w, 's>;

    // `OpaqueFetch` breaks if i remove this module (or the fn `define_opaque` lower down), and I don't know why.
    mod define {
        #[allow(unreachable_code)]
        #[allow(unused)]
        fn dummy<'w, 's>() -> super::OpaqueFetch {}
    }

    // the purpose of this fn is just to fulfill the "defining scope"
    // for the above opaque (existential) type.
    // We simply define a function that never gets called in order to clue the compiler
    // in on what the concrete type of `OpaqueFetch` is.
    #[allow(unreachable_code)]
    #[allow(unused)]
    fn define_opaque<'w, 's>() -> OpaqueFetch {
        type QueryFetch<'w, 's> =
            <Query<'w, 's, Read<Private>> as bevy::ecs::system::SystemParam>::Fetch;

        use bevy::ecs::system::SystemParamState as _;
        <OpaqueParamsState<(QueryFetch<'w, 's>,)>>::init(unreachable!(), unreachable!())
    }

    impl<'w, 's> bevy::ecs::system::SystemParam for OpaqueParams<'w, 's> {
        type Fetch = OpaqueFetch;
    }
    #[doc(hidden)]
    pub struct OpaqueParamsState<TSystemParamState> {
        state: TSystemParamState,
        marker: std::marker::PhantomData<()>,
    }
    unsafe impl<TSystemParamState: bevy::ecs::system::SystemParamState>
        bevy::ecs::system::SystemParamState for OpaqueParamsState<TSystemParamState>
    {
        fn init(
            world: &mut bevy::ecs::world::World,
            system_meta: &mut bevy::ecs::system::SystemMeta,
        ) -> Self {
            Self {
                state: TSystemParamState::init(world, system_meta),
                marker: std::marker::PhantomData,
            }
        }
        fn new_archetype(
            &mut self,
            archetype: &bevy::ecs::archetype::Archetype,
            system_meta: &mut bevy::ecs::system::SystemMeta,
        ) {
            self.state.new_archetype(archetype, system_meta)
        }
        fn apply(&mut self, world: &mut bevy::ecs::world::World) {
            self.state.apply(world)
        }
    }
    impl<'w, 's> bevy::ecs::system::SystemParamFetch<'w, 's>
        for OpaqueParamsState<(
            <Query<'w, 's, Read<Private>> as bevy::ecs::system::SystemParam>::Fetch,
        )>
    {
        type Item = OpaqueParams<'w, 's>;
        unsafe fn get_param(
            state: &'s mut Self,
            system_meta: &bevy::ecs::system::SystemMeta,
            world: &'w bevy::ecs::world::World,
            change_tick: u32,
        ) -> Self::Item {
            OpaqueParams { q : < < Query < 'w , 's , Read < Private > > as bevy :: ecs :: system :: SystemParam > :: Fetch as bevy :: ecs :: system :: SystemParamFetch > :: get_param (& mut state . state . 0 , system_meta , world , change_tick) , }
        }
    }

    // prints the value of private components
    pub fn print_private(OpaqueParams { q }: OpaqueParams) {
        for p in q.iter() {
            println!("{}", p.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugin(inner::InnerPlugin)
        .add_system(print_stuff)
        .run();
}

fn print_stuff(params: inner::OpaqueParams) {
    inner::print_private(params);
}
