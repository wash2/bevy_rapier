use super::{IntoEntity, IntoHandle};
use crate::rapier::dynamics::{
    RigidBodyActivation, RigidBodyCcd, RigidBodyChanges, RigidBodyColliders, RigidBodyDamping,
    RigidBodyDominance, RigidBodyForces, RigidBodyHandle, RigidBodyIds, RigidBodyMassProps,
    RigidBodyPosition, RigidBodyType, RigidBodyVelocity,
};
use bevy::prelude::*;
use rapier::data::{ComponentSet, ComponentSetMut, ComponentSetOption, Index};

impl IntoHandle<RigidBodyHandle> for Entity {
    #[inline]
    fn handle(self) -> RigidBodyHandle {
        RigidBodyHandle::from_raw_parts(self.id(), self.generation())
    }
}

impl IntoEntity for RigidBodyHandle {
    #[inline]
    fn entity(self) -> Entity {
        self.0.entity()
    }
}

pub type RigidBodyComponentsQuery<'w, 's> = QuerySet<(
    Query<
        'w,
        's,
        (
            Entity,
            &'static RigidBodyPosition,
            &'static RigidBodyVelocity,
            &'static RigidBodyMassProps,
            &'static RigidBodyIds,
            &'static RigidBodyForces,
            &'static RigidBodyActivation,
            &'static RigidBodyChanges,
            &'static RigidBodyCcd,
            &'static RigidBodyColliders,
            &'static RigidBodyDamping,
            &'static RigidBodyDominance,
            &'static RigidBodyType,
        ),
    >,
    Query<
        'w,
        's,
        (
            Entity,
            &'static mut RigidBodyPosition,
            &'static mut RigidBodyVelocity,
            &'static mut RigidBodyMassProps,
            &'static mut RigidBodyIds,
            &'static mut RigidBodyForces,
            &'static mut RigidBodyActivation,
            &'static mut RigidBodyChanges,
            &'static mut RigidBodyCcd,
            // Need for handling collider removals.
            &'static mut RigidBodyColliders,
        ),
    >,
    Query<
        'w,
        's,
        (
            Entity,
            &'static mut RigidBodyChanges,
            &'static mut RigidBodyActivation,
            Or<(Changed<RigidBodyPosition>, Added<RigidBodyPosition>)>,
            Or<(Changed<RigidBodyVelocity>, Added<RigidBodyVelocity>)>,
            Or<(Changed<RigidBodyForces>, Added<RigidBodyForces>)>,
            Or<(Changed<RigidBodyType>, Added<RigidBodyType>)>,
            Or<(Changed<RigidBodyColliders>, Added<RigidBodyColliders>)>,
        ),
        Or<(
            Changed<RigidBodyPosition>,
            Added<RigidBodyPosition>,
            Changed<RigidBodyVelocity>,
            Added<RigidBodyVelocity>,
            Changed<RigidBodyForces>,
            Added<RigidBodyForces>,
            Changed<RigidBodyActivation>,
            Added<RigidBodyActivation>,
            Changed<RigidBodyType>,
            Added<RigidBodyType>,
            Changed<RigidBodyColliders>,
            Added<RigidBodyColliders>,
        )>,
    >,
    Query<
        'w,
        's,
        &'static mut RigidBodyChanges,
        Or<(Changed<RigidBodyActivation>, Added<RigidBodyActivation>)>,
    >,
)>;

pub struct RigidBodyComponentsSet<'w, 's>(pub RigidBodyComponentsQuery<'w, 's>);

impl_component_set_mut!(RigidBodyComponentsSet, RigidBodyPosition, |data| data.1);
impl_component_set_mut!(RigidBodyComponentsSet, RigidBodyVelocity, |data| data.2);
impl_component_set_mut!(RigidBodyComponentsSet, RigidBodyMassProps, |data| data.3);
impl_component_set_mut!(RigidBodyComponentsSet, RigidBodyIds, |data| data.4);
impl_component_set_mut!(RigidBodyComponentsSet, RigidBodyForces, |data| data.5);
impl_component_set_mut!(RigidBodyComponentsSet, RigidBodyActivation, |data| data.6);
impl_component_set_mut!(RigidBodyComponentsSet, RigidBodyChanges, |data| data.7);
impl_component_set_mut!(RigidBodyComponentsSet, RigidBodyCcd, |data| data.8);
impl_component_set_mut!(RigidBodyComponentsSet, RigidBodyColliders, |data| data.9);

impl_component_set!(RigidBodyComponentsSet, RigidBodyDamping, |data| data.10);
impl_component_set!(RigidBodyComponentsSet, RigidBodyDominance, |data| data.11);
impl_component_set!(RigidBodyComponentsSet, RigidBodyType, |data| data.12);

#[derive(Bundle)]
pub struct RigidBodyBundle {
    pub body_type: RigidBodyType,
    pub position: RigidBodyPosition,
    pub velocity: RigidBodyVelocity,
    pub mass_properties: RigidBodyMassProps,
    pub forces: RigidBodyForces,
    pub activation: RigidBodyActivation,
    pub damping: RigidBodyDamping,
    pub dominance: RigidBodyDominance,
    pub ccd: RigidBodyCcd,
    pub changes: RigidBodyChanges,
    pub ids: RigidBodyIds,
    pub colliders: RigidBodyColliders,
}

impl Default for RigidBodyBundle {
    fn default() -> Self {
        Self {
            body_type: RigidBodyType::Dynamic,
            position: RigidBodyPosition::default(),
            velocity: RigidBodyVelocity::default(),
            mass_properties: RigidBodyMassProps::default(),
            forces: RigidBodyForces::default(),
            activation: RigidBodyActivation::default(),
            damping: RigidBodyDamping::default(),
            dominance: RigidBodyDominance::default(),
            ccd: RigidBodyCcd::default(),
            changes: RigidBodyChanges::default(),
            ids: RigidBodyIds::default(),
            colliders: RigidBodyColliders::default(),
        }
    }
}
