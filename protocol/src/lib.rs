use serde::{Deserialize, Serialize};

/// Page to worker game messages, carried as `Custom` payloads on the engine
/// wire. Extend this enum as the game grows; the transport (input, resize,
/// picking, stats) is handled by `nightshade-api` and needs no messages here.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Command {
    /// Toggles whether the spawned cubes spin.
    SetSpin { spinning: bool },
    /// Example game message. Replace with your own as the game grows.
    SpawnCube,
    /// Recolors the picked entity, demonstrating the selection round trip.
    PaintSelected,
}

/// Worker to page game messages, carried as `Custom` payloads on the engine
/// wire.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    /// Example game message. Replace with your own as the game grows.
    CubeCount { count: u32 },
}
