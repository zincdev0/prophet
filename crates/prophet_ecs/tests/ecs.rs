#[cfg(test)]

extern crate prophet_ecs;

use prophet_ecs::Ecs;

#[test]
fn new()
        -> () {
    let _ecs = Ecs::new();
}

#[test]
fn new_component()
        -> () {
    let mut ecs = Ecs::new();
    let _component = ecs.new_component();
}
