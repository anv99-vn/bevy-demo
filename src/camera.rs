use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

const TAU: f32 = std::f32::consts::TAU;

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
    angle_x: f32,
    angle_y: f32,
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
            angle_x: 0.0,
            angle_y: 0.35,
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
                cam.angle_x -= ev.delta.x * sens;
                cam.angle_y -= ev.delta.y * sens;
            }
        }

        for ev in scroll.read() {
            if !over_ui {
                cam.distance = (cam.distance - ev.y * 0.5).clamp(2.0, 30.0);
            }
        }

        cam.angle_x %= TAU;
        cam.angle_y %= TAU;

        let pos = cam.target + Vec3::new(
            cam.angle_x.sin() * cam.angle_y.cos() * cam.distance,
            cam.angle_y.sin() * cam.distance,
            cam.angle_x.cos() * cam.angle_y.cos() * cam.distance,
        );
        *transform = Transform::from_translation(pos).looking_at(cam.target, Vec3::Y);
    }
}
