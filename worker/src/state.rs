use nightshade::prelude::Entity;

/// The application state carried across frames by the offscreen driver. Plain
/// data: the spawned cubes and the spin toggle. This is the `nightshade-api`
/// style, no user-side ECS. Selection is owned by the driver and lands in
/// `apply_custom` as an argument.
pub struct Scene {
    pub cubes: Vec<Entity>,
    pub spinning: bool,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            cubes: Vec::new(),
            spinning: true,
        }
    }
}
