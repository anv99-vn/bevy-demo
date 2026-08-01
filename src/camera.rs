use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraSettings {
    pub sensitivity: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self { sensitivity: 0.01 }
    }
}

#[derive(Component)]
pub struct OrbitCamera {
    yaw: f32,
    pitch: f32,
    distance: f32,
    target: Vec3,
}

#[derive(Component)]
pub struct BlocksCameraRotation;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.5, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
        OrbitCamera {
            yaw: 0.0,
            pitch: 0.35,
            distance: 6.0,
            target: Vec3::ZERO,
        },
    ));
}

pub fn update(
    mut q: Query<(&mut Transform, &mut OrbitCamera)>,
    ui: Query<&Interaction, With<BlocksCameraRotation>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut scroll: EventReader<MouseWheel>,
    mut motion: EventReader<MouseMotion>,
    settings: Res<CameraSettings>,
) {
    let sens = settings.sensitivity;
    let over_ui = ui.iter().any(|i| *i == Interaction::Pressed);
    for (mut transform, mut cam) in &mut q {
        let drag = mouse.pressed(MouseButton::Left);
        for ev in motion.read() {
            if drag && !over_ui {
                cam.yaw -= ev.delta.x * sens;
                cam.pitch = (cam.pitch + ev.delta.y * sens).clamp(-1.4, 1.4);
            }
        }

        for ev in scroll.read() {
            if !over_ui {
                cam.distance = (cam.distance - ev.y * 0.5).clamp(2.0, 30.0);
            }
        }

        let rot = Quat::from_euler(EulerRot::YXZ, cam.yaw, -cam.pitch, 0.0);
        let pos = cam.target + rot * Vec3::new(0.0, 0.0, cam.distance);
        *transform = Transform::from_translation(pos).looking_at(cam.target, Vec3::Y);
    }
}
