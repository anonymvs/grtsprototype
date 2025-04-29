use crate::game::unit_type::UnitConstants;
use bevy::prelude::*;
use bitflags::bitflags;

// #[derive(Component)]
// #[require(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform)]
// pub struct Unit;

#[derive(Resource)]
pub struct CursorPosition(pub Vec2);

impl CursorPosition {
    pub fn set(&mut self, position: Vec2) {
        self.0 = position;
    }
}

bitflags! {
    #[derive(Resource)]
    pub struct UnitFlags: u8 {
        const NONE = 0b00000000;
        const RECT_MOVE = 0b00000001;
        //const ATTACKING = 0b00000010;
        //const DEFENDING = 0b00000100;
        //const DEAD = 0b00001000;
    }
}

// impl UnitFlags {
//     pub fn is_moving(&self) -> bool {
//         self.contains(UnitFlags::MOVING)
//     }

//     pub fn is_attacking(&self) -> bool {
//         self.contains(UnitFlags::ATTACKING)
//     }

//     pub fn is_defending(&self) -> bool {
//         self.contains(UnitFlags::DEFENDING)
//     }

//     pub fn is_dead(&self) -> bool {
//         self.contains(UnitFlags::DEAD)
//     }
// }

#[derive(Component)]
pub struct Playable;

#[derive(Component)]
pub struct UnitState {
    // pub health: f32,
    // pub max_health: f32,
    // pub xp: f32,
    pub velocity: f32,
    pub direction: Vec2,
    pub easing_vector: Vec2,
}

#[derive(Bundle)]
pub struct PlayerUnitBundle {
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
    pub transform: Transform,
    pub state: UnitState,
    pub playable: Playable,
    pub contants: UnitConstants,
}

// #[derive(Component)]
// pub struct Rect(Unit);

// /// Marker for the player
// #[derive(Component)]
// struct Player;

// /// Bundle to make it easy to spawn the player entity
// /// with all the correct components:
// #[derive(Bundle)]
// struct PlayerBundle {
//     marker: Player,
//     health: Health,
//     xp: Xp,
//     // including all the components from another bundle
//     sprite: SpriteBundle,
// }
// }
