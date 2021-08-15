extern crate rapier3d as rapier; // For the debug UI.

use bevy::{
    PipelinedDefaultPlugins, 
    ecs::prelude::*, 
    pbr2::{PointLight, PointLightBundle}, 
    prelude::{App, Transform, FaceToward}, 
    render2::camera::PerspectiveCameraBundle,
    math::{Mat4, Vec3},
};
use bevy_rapier3d::prelude::*;

use rapier::geometry::SolverFlags;
use rapier3d::pipeline::{PairFilterContext, PhysicsPipeline};
// use ui::DebugUiPlugin;

// #[path = "../../src_debug_ui/mod.rs"]
// mod ui;

#[derive(PartialEq, Eq, Clone, Copy)]
enum CustomFilterTag {
    GroupA,
    GroupB,
}

impl CustomFilterTag {
    fn with_id(id: usize) -> Self {
        if id % 2 == 0 {
            Self::GroupA
        } else {
            Self::GroupB
        }
    }
}

// A custom filter that allows contacts only between rigid-bodies with the
// same user_data value.
// Note that using collision groups would be a more efficient way of doing
// this, but we use custom filters instead for demonstration purpose.
struct SameUserDataFilter;
impl<'a> PhysicsHooksWithQuery<'_, '_, &'a CustomFilterTag> for SameUserDataFilter {
    fn filter_contact_pair(
        &self,
        context: &PairFilterContext<RigidBodyComponentsSet, ColliderComponentsSet>,
        tags: &Query<&'a CustomFilterTag>,
    ) -> Option<SolverFlags> {
        if tags.get(context.collider1.entity()).ok().copied()
            == tags.get(context.collider2.entity()).ok().copied()
        {
            Some(SolverFlags::COMPUTE_IMPULSES)
        } else {
            None
        }
    }
}

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::rgb(
        //     0xF9 as f32 / 255.0,
        //     0xF9 as f32 / 255.0,
        //     0xFF as f32 / 255.0,
        // )))
        // .insert_resource(Msaa::default())
        .add_plugins(PipelinedDefaultPlugins)
        .add_plugin(bevy_winit::WinitPlugin::default())
        // .add_plugin(bevy_wgpu::WgpuPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<&CustomFilterTag>::default())
        .add_plugin(RapierRenderPlugin)
        // .add_plugin(DebugUiPlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_startup_system(enable_physics_profiling)
        .run();
}

fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(100.0, 10.0, 200.0)),
        point_light: PointLight {
            intensity: 100_000.0,
            range: 3000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(Mat4::face_toward(
            Vec3::new(-30.0, 30.0, 100.0),
            Vec3::new(0.0, 10.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        )),
        ..Default::default()
    });
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    commands.insert_resource(PhysicsHooksWithQueryObject(Box::new(SameUserDataFilter {})));

    let ground_size = 10.0;

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size, 1.2, ground_size),
        position: [0.0, -10.0, 0.0].into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete)
        .insert(CustomFilterTag::GroupA);

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size, 1.2, ground_size),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete)
        .insert(CustomFilterTag::GroupB);

    /*
     * Create the cubes
     */
    let num = 4;
    let rad = 0.5;

    let shift = rad * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;
    let mut color = 0;

    for i in 0..num {
        for j in 0usize..num * 5 {
            let x = (i as f32 + j as f32 * 0.2) * shift - centerx;
            let y = j as f32 * shift + centery + 2.0;
            color += 1;

            // Build the rigid body.
            let body = RigidBodyBundle {
                position: [x, y, 0.0].into(),
                ..Default::default()
            };

            let collider = ColliderBundle {
                shape: ColliderShape::cuboid(rad, rad, rad),
                flags: ActiveHooks::FILTER_CONTACT_PAIRS.into(),
                ..Default::default()
            };
            commands
                .spawn_bundle(body)
                .insert_bundle(collider)
                .insert(ColliderDebugRender::with_id(color % 2))
                .insert(ColliderPositionSync::Discrete)
                .insert(CustomFilterTag::with_id(color));
        }
    }
}
