extern crate rapier3d as rapier; // For the debug UI.

use bevy::{
    asset::{Handle, HandleUntyped},
    ecs::prelude::*,
    gltf2::{Gltf, GltfMesh},
    math::Vec3,
    pbr2::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    prelude::{App, AssetServer, Assets, ClearColor, CoreStage, Time, Transform},
    render2::{
        camera::PerspectiveCameraBundle,
        color::Color,
        mesh::{shape, Mesh},
    },
    PipelinedDefaultPlugins,
};
use bevy_rapier3d::{na::Point3, prelude::*};
use rapier3d::pipeline::PhysicsPipeline;
use std::convert::TryInto;
static DEBUG: &str = "debug";

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    GltfAssetsLoading,
    InGame,
}
#[derive(Default)]
struct AssetsLoading(Vec<HandleUntyped>);

fn setup_assets(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    println!("loading assets");
    // we can have different asset types
    let handle: Handle<Gltf> =
        server.load(format!("{}/../assets/Suzanne.gltf", env!("CARGO_MANIFEST_DIR")).as_str());
    // add them all to our collection for tracking
    loading.0.push(handle.clone_untyped());
}

fn check_assets_ready(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
    mut app_state: ResMut<State<AppState>>,
) {
    use bevy::asset::LoadState;

    match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {
            println!("Loading failed...");
            // one of our assets had an error
        }
        LoadState::Loaded => {
            println!("loading successful!");
            // all assets are now ready

            // this might be a good place to transition into your in-game state

            // remove the resource to drop the tracking handles
            app_state.set(AppState::InGame).unwrap();
            commands.remove_resource::<AssetsLoading>();

            // (note: if you don't have any other handles to the assets
            // elsewhere, they will get unloaded after this)
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
            println!("Loading...");
        }
    }
}

fn setup_physics(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let color = 0;

    /* Create the ground. */
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(5.0, 0.1, 5.0),
        ..Default::default()
    };

    commands
        .spawn_bundle(collider)
        // .insert(ColliderDebugRender::with_id(color))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(10., 0.2, 10.))),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("EAFFFF").unwrap(),
                // vary key PBR parameters on a grid of spheres to show the effect
                metallic: 1.0,
                perceptual_roughness: 0.0,
                reflectance: 1.0,
                ..Default::default()
            }),
            transform: Transform::from_xyz(-5.0, -2.5, 0.0),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0.0, 10.0, 0.0).into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(0.5),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(collider)
        // .insert(mesh)
        // .insert(ColliderDebugRender::with_id(color))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 0.5,
                subdivisions: 4,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("EAFFFF").unwrap(),
                // vary key PBR parameters on a grid of spheres to show the effect
                metallic: 1.0,
                perceptual_roughness: 0.0,
                reflectance: 1.0,
                ..Default::default()
            }),
            transform: Transform::from_xyz(-5.0, -2.5, 0.0),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    // Then any asset in the folder can be accessed like this:
    let suzanne_gltf_handle: Handle<Gltf> = asset_server
        .load(format!("{}/../assets/Suzanne.gltf", env!("CARGO_MANIFEST_DIR")).as_str());
    let suzanne_gltf = gltfs.get(suzanne_gltf_handle).unwrap();
    let suzanne_gltf_mesh = suzanne_gltf.named_meshes.get("Suzanne").unwrap();

    let suzanne_primitive_mesh = gltf_meshes.get(suzanne_gltf_mesh).unwrap();
    let suzanne_handle = &suzanne_primitive_mesh.primitives[0].mesh;
    let suzanne_mesh = meshes.get(suzanne_handle.clone()).unwrap();

    let sharedshapetuple: (Vec<Point3<f32>>, Vec<[u32; 3]>) =
        SharedShapeMesh(suzanne_mesh.clone()).try_into().unwrap();

    let tri_coll = ColliderShape::trimesh(sharedshapetuple.0, sharedshapetuple.1);
    let rb = RigidBodyBundle {
        position: Vec3::new(0.0, 5.0, 0.0).into(),
        ..Default::default()
    };
    let coll = ColliderBundle {
        shape: tri_coll,
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        ..Default::default()
    };
    commands
        .spawn_bundle(coll)
        .insert_bundle(rb)
        .insert_bundle(PbrBundle {
            mesh: suzanne_handle.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("EAFFFF").unwrap(),
                // vary key PBR parameters on a grid of spheres to show the effect
                metallic: 1.0,
                perceptual_roughness: 0.0,
                reflectance: 1.0,
                ..Default::default()
            }),
            transform: Transform::from_xyz(-5.0, -2.5, 0.0),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}

fn main() {
    App::new()
        .add_plugins(PipelinedDefaultPlugins)
        .add_plugin(bevy_winit::WinitPlugin::default())
        // .add_plugin(bevy_wgpu::WgpuPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        // .add_plugin(DebugUiPlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(enable_physics_profiling)
        .add_state(AppState::GltfAssetsLoading)
        .init_resource::<AssetsLoading>()
        .add_system_set(SystemSet::on_enter(AppState::GltfAssetsLoading).with_system(setup_assets))
        .add_system_set(
            SystemSet::on_update(AppState::GltfAssetsLoading).with_system(check_assets_ready),
        )
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_physics))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(50.0, 50.0, 50.0)),
        point_light: PointLight {
            intensity: 100_000.0,
            range: 3000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0., 10., 15.0).looking_at(Vec3::new(0.0, 5.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}
