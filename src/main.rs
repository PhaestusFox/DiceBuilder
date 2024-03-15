use bevy::prelude::*;
use voxel_cam::{VoxelCam, VoxelCamPlugin};

mod voxel_cam;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, VoxelCamPlugin {
        voxel_size: 1,
    })).add_systems(Startup, spawn_camera)
    .add_systems(Update, output_player_pos);
    app.run();
}

fn spawn_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3::ONE)),
        ..Default::default()
    });
    commands.spawn((
        Camera3dBundle::default(),
        VoxelCam {
            move_north: KeyCode::KeyW,
            move_south: KeyCode::KeyS,
            move_east: KeyCode::KeyD,
            move_west: KeyCode::KeyA,
            move_up: KeyCode::KeyQ,
            move_down: KeyCode::KeyE,
            face: voxel_cam::FaceMode::Mouse { invert_y: false, sensitivity: 0.001 },
            min_move_time: None,
            hold_delay: std::time::Duration::from_millis(250),
        }
    ));
}

fn output_player_pos(
    player: Query<&Transform, With<VoxelCam>>
) {
    for player in &player {
        println!("Player @ {}", player.translation)
    }
}