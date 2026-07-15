use crate::state::Scene;
use crate::systems::example;
use nightshade::prelude::{load_procedural_textures, spawn_sun};
use nightshade_api::prelude::*;

/// Builds the scene through the `nightshade-api` facade: background and image
/// based lighting, the reference grid, a sun, an orbit camera, and the first
/// cube. The selection outline is an engine setting the facade does not
/// expose, so it is set directly.
pub fn initialize(scene: &mut Scene, world: &mut World) {
    if let Some((width, height)) = world
        .expect_resource::<nightshade::ecs::window::resources::Window>()
        .cached_viewport_size
    {
        world
            .expect_resource_mut::<nightshade::ecs::window::resources::Window>()
            .active_viewport_rect = Some(nightshade::render::config::ViewportRect {
            x: 0.0,
            y: 0.0,
            width: width as f32,
            height: height as f32,
        });
    }

    set_background(world, Background::Nebula);
    show_grid(world, true);
    world
        .expect_resource_mut::<nightshade::ecs::graphics::selection::Selection>()
        .outline_enabled = true;
    world
        .expect_resource_mut::<nightshade::ecs::graphics::selection::Selection>()
        .outline_color = [1.0, 0.5, 0.15, 1.0];

    load_procedural_textures(world);
    spawn_sun(world);
    orbit_camera(world, vec3(0.0, 0.5, 0.0), 8.0);

    example::spawn_cube_on_ring(scene, world);
}
