use crate::state::Scene;
use crate::systems::example;
use nightshade::prelude::{Selection, load_procedural_textures, spawn_sun};
use nightshade_api::prelude::*;

/// Builds the scene through the `nightshade-api` facade: background and image
/// based lighting, the reference grid, a sun, an orbit camera, and the first
/// cube. The selection outline is an engine setting the facade does not
/// expose, so it is set directly.
pub fn initialize(scene: &mut Scene, world: &mut World) {
    set_background(world, Background::Nebula);
    show_grid(world, true);
    world.res_mut::<Selection>().outline_enabled = true;
    world.res_mut::<Selection>().outline_color = [1.0, 0.5, 0.15, 1.0];

    load_procedural_textures(world);
    spawn_sun(world);
    orbit_camera(world, vec3(0.0, 0.5, 0.0), 8.0);

    example::spawn_cube_on_ring(scene, world);
}
