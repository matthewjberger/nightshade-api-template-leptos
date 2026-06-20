use crate::state::Scene;
use nightshade::prelude::Name;
use nightshade_api::prelude::*;
use protocol::WorkerMessage;

const SPIN_RADIANS_PER_SECOND: f32 = 0.8;
const RING_RADIUS: f32 = 3.0;
const GOLDEN_ANGLE_RADIANS: f32 = 2.399_963;

/// The per-frame logic, written as straight-line `nightshade-api` calls. Each
/// system is a free function taking the plain `Scene` state and the engine
/// `World`. Add more files in `src/systems/` and call them from
/// `Scene::run_systems` to grow the game.
///
/// This one spins every spawned cube while the toggle is on and spawns another
/// on Space.
pub fn tick(scene: &mut Scene, world: &mut World) {
    if scene.spinning {
        let step = SPIN_RADIANS_PER_SECOND * delta_time(world);
        for &cube in &scene.cubes {
            rotate(world, cube, Vec3::y(), step);
        }
    }

    if key_pressed(world, KeyCode::Space) {
        spawn_cube_on_ring(scene, world);
    }
}

/// Spawns a cube on a ring around the origin, colors and names it, and reports
/// the new count to the page.
pub fn spawn_cube_on_ring(scene: &mut Scene, world: &mut World) {
    let index = scene.cubes.len();
    let position = if index == 0 {
        vec3(0.0, 0.5, 0.0)
    } else {
        let angle = index as f32 * GOLDEN_ANGLE_RADIANS;
        vec3(angle.cos() * RING_RADIUS, 0.5, angle.sin() * RING_RADIUS)
    };
    let cube = spawn_cube(world, position);
    set_color(world, cube, color_for(index));
    world.core.set_name(cube, Name(format!("Cube {index}")));
    scene.cubes.push(cube);
    crate::post(&WorkerMessage::CubeCount {
        count: scene.cubes.len() as u32,
    });
}

fn color_for(index: usize) -> [f32; 4] {
    let hue = (index as f32 * GOLDEN_ANGLE_RADIANS).rem_euclid(std::f32::consts::TAU);
    let sector = hue / std::f32::consts::FRAC_PI_3;
    let fraction = sector - sector.floor();
    let rising = fraction;
    let falling = 1.0 - fraction;
    let (red, green, blue) = match sector as u32 % 6 {
        0 => (1.0, rising, 0.0),
        1 => (falling, 1.0, 0.0),
        2 => (0.0, 1.0, rising),
        3 => (0.0, falling, 1.0),
        4 => (rising, 0.0, 1.0),
        _ => (1.0, 0.0, falling),
    };
    [
        0.25 + red * 0.75,
        0.25 + green * 0.75,
        0.25 + blue * 0.75,
        1.0,
    ]
}
