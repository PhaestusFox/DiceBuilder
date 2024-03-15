use std::{num::NonZeroU8, time::Duration};

use bevy::{input::mouse::MouseMotion, prelude::*};

#[derive(Resource)]
pub struct VoxelSize(NonZeroU8);

impl VoxelSize {
    fn new(size: u8) -> VoxelSize {
        VoxelSize(NonZeroU8::new(size).expect("Voxel Size >0"))
    }
}

#[derive(Component)]
pub struct VoxelCam {
    pub move_north : KeyCode,
    pub move_south : KeyCode,
    pub move_east : KeyCode,
    pub move_west : KeyCode,    
    pub move_up : KeyCode,    
    pub move_down : KeyCode,
    pub face: FaceMode,
    pub min_move_time: Option<Duration>,
    pub hold_delay: Duration,
}

pub enum FaceMode {
    Curser,
    Entity(Entity),
    Position(Vec3),
    Mouse {
        invert_y: bool,
        sensitivity: f32,
    },
    Keyboard {
        pitch_up: KeyCode,
        pitch_down: KeyCode,
        yaw_left: KeyCode,
        yaw_right: KeyCode,
    }
}

impl FaceMode {
    fn default_data(&self) -> VoxelCameraData {
        match self {
            Self::Mouse { .. } => VoxelCameraData::Mouse { pitch: 0., yaw: 0. },
            _ => todo!()
        }
    }

    fn data_discriminant(&self) -> std::mem::Discriminant<VoxelCameraData> {
        std::mem::discriminant(&self.default_data())
    }
}

#[derive(Component)]
pub struct LastMoved(Duration);

pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn as_vec3(&self) -> Vec3 {
        match self {
            Direction::North => Vec3::NEG_Z,
            Direction::South => Vec3::Z,
            Direction::East => Vec3::X,
            Direction::West => Vec3::NEG_X,
            Direction::Up => Vec3::Y,
            Direction::Down => Vec3::NEG_Y,
        }
    }
}

pub struct VoxelCamPlugin {
    pub voxel_size: u8,
}

impl Plugin for VoxelCamPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(VoxelSize::new(self.voxel_size))
        .add_systems(Update, (move_camera, rotate_camera_mouse, cam_look_at))
        .add_systems(Last, add_camera_data);
    }
}

fn move_camera(
    mut commands: Commands,
    mut cameras: Query<(Entity, &mut Transform, &VoxelCam, Option<&LastMoved>)>,
    inputs: Res<ButtonInput<KeyCode>>,
    voxel_size: Res<VoxelSize>,
    time: Res<Time>
) {
    for (entity, mut transform, config, delay) in &mut cameras {
        let mut can_move = false;
        let mut moved = false;
        let mut delta = Vec3::ZERO;
        match (config.min_move_time, delay) {
            (None, None) => {},
            (Some(min), Some(last)) => {
                can_move = time.elapsed() - last.0 > min;
            },
            (None, Some(_)) |
            (Some(_), None) => {
                can_move = true;
            },
        }
        if can_move {
            if inputs.just_pressed(config.move_north) {
                delta += Direction::North.as_vec3();
                moved = true;
            }
            if inputs.just_pressed(config.move_south) {
                delta += Direction::South.as_vec3();
                moved = true;
            }
            if inputs.just_pressed(config.move_east) {
                delta += Direction::East.as_vec3();
                moved = true;
            }
            if inputs.just_pressed(config.move_west) {
                delta += Direction::West.as_vec3();
                moved = true;
            }
            if inputs.just_pressed(config.move_up) {
                delta += Direction::Up.as_vec3();
                moved = true;
            }
            if inputs.just_pressed(config.move_down) {
                delta += Direction::Down.as_vec3();
                moved = true;
            }
        }
        if let Some(last) = delay {
            can_move = !moved && time.elapsed() - last.0 > config.hold_delay;
        } else {
            can_move = !moved;
        }
        if can_move {
            if inputs.pressed(config.move_north) {
                delta += Direction::North.as_vec3();
                moved = true;
            }
            if inputs.pressed(config.move_south) {
                delta += Direction::South.as_vec3();
                moved = true;
            }
            if inputs.pressed(config.move_east) {
                delta += Direction::East.as_vec3();
                moved = true;
            }
            if inputs.pressed(config.move_west) {
                delta += Direction::West.as_vec3();
                moved = true;
            }
            if inputs.pressed(config.move_up) {
                delta += Direction::Up.as_vec3();
                moved = true;
            }
            if inputs.pressed(config.move_down) {
                delta += Direction::Down.as_vec3();
                moved = true;
            }
        }
        
        if moved {
            transform.translation += delta * voxel_size.0.get() as f32;
            commands.entity(entity).insert(LastMoved(time.elapsed()));
        }
    }
}

fn add_camera_data(
    mut commands: Commands,
    mut cameras: Query<(Entity, &VoxelCam, Option<&mut VoxelCameraData>), Changed<VoxelCam>>
) {
    for (entity, config, data) in &mut cameras {
        let Some(mut data) = data else {commands.entity(entity).insert(config.face.default_data()); continue;};
        if std::mem::discriminant(data.as_ref()) == config.face.data_discriminant() {continue;}
        *data = config.face.default_data();
    }
}

#[derive(Component)]
enum VoxelCameraData {
    Mouse {
        pitch: f32,
        yaw: f32,
    }
}

fn rotate_camera_mouse(
    mut mouse_events: EventReader<MouseMotion>,
    mut cameras: Query<(&mut VoxelCameraData, &VoxelCam)>
) {
    let delta: Vec2 = mouse_events.read().map(|e| e.delta).sum();
    for (mut data, config) in &mut cameras {
        if let VoxelCameraData::Mouse { ref mut pitch, ref mut yaw } = data.as_mut() {
            if let FaceMode::Mouse { invert_y, sensitivity } = config.face {
                *pitch += delta.y * sensitivity * if invert_y {-1.} else {1.};
                *yaw += delta.x * sensitivity;
            } else {
                warn!("VoxelCameraData is set to mouse but FaceMode is Not")
            }
        }
    }
}

fn cam_look_at(
    mut camera: Query<(&mut Transform, &VoxelCameraData), Changed<VoxelCameraData>>,
) {
    for (mut transform, data) in &mut camera {
        match data {
            VoxelCameraData::Mouse { pitch, yaw } => {
                transform.rotation = Quat::from_rotation_y(*yaw) * Quat::from_rotation_x(-pitch);
            }
        }
    }
}