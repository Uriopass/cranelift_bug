use legion::WorldOptions;

fn main() {
    let mut w = legion::world::World::new(WorldOptions::default());
    w.push((1,));
}
