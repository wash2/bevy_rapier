pub use self::collider_component_set::*;
pub use self::components::*;
pub use self::plugins::*;
pub use self::mesh_collider::*;
pub use self::resources::*;
pub use self::rigid_body_component_set::*;
pub use self::systems::*;

use crate::rapier::data::{ComponentSet, ComponentSetMut, ComponentSetOption, Index};
use crate::rapier::prelude::*;
use bevy::prelude::{Entity, Query, QuerySet};

pub trait IntoHandle<H> {
    fn handle(self) -> H;
}

pub trait IntoEntity {
    fn entity(self) -> Entity;
}

impl IntoHandle<Index> for Entity {
    #[inline]
    fn handle(self) -> Index {
        Index::from_raw_parts(self.id(), self.generation())
    }
}

impl IntoEntity for Index {
    #[inline]
    fn entity(self) -> Entity {
        let (id, gen) = self.into_raw_parts();
        let bits = u64::from(gen) << 32 | u64::from(id);
        Entity::from_bits(bits)
    }
}

impl IntoHandle<JointHandle> for Entity {
    #[inline]
    fn handle(self) -> JointHandle {
        JointHandle::from_raw_parts(self.id(), self.generation())
    }
}

impl IntoEntity for JointHandle {
    fn entity(self) -> Entity {
        self.0.entity()
    }
}

pub trait BundleBuilder {
    type Bundle;
    fn bundle(&self) -> Self::Bundle;
}

macro_rules! impl_component_set_mut(
    ($ComponentsSet: ident, $T: ty, |$data: ident| $data_expr: expr) => {
        impl<'w, 's> ComponentSetOption<$T> for $ComponentsSet<'w, 's> {
            #[inline(always)]
            fn get(&self, handle: Index) -> Option<&$T> {
                self.0.q0().get_component(handle.entity()).ok()
            }
        }

        impl<'w, 's> ComponentSet<$T> for $ComponentsSet<'w, 's> {
            #[inline(always)]
            fn size_hint(&self) -> usize {
                0
            }

            #[inline(always)]
            fn for_each(&self, mut f: impl FnMut(Index, &$T)) {
                self.0.q0().for_each(|$data| f($data.0.handle(), $data_expr))
            }
        }

        impl<'w, 's> ComponentSetMut<$T> for $ComponentsSet<'w, 's> {
            #[inline(always)]
            fn set_internal(&mut self, handle: Index, val: $T) {
                let _ = self.0.q1_mut().get_component_mut(handle.entity()).map(|mut data| *data = val);
            }

            #[inline(always)]
            fn map_mut_internal<Result>(
                &mut self,
                handle: Index,
                f: impl FnOnce(&mut $T) -> Result,
            ) -> Option<Result> {
                self.0.q1_mut()
                    .get_component_mut(handle.entity())
                    .map(|mut data| f(&mut data))
                    .ok()
            }
        }
    }
);

macro_rules! impl_component_set_wo_query_set(
    ($ComponentsSet: ident, $T: ty, |$data: ident| $data_expr: expr) => {
        impl<'w, 's> ComponentSetOption<$T> for $ComponentsSet<'w, 's> {
            #[inline(always)]
            fn get(&self, handle: Index) -> Option<&$T> {
                self.0.get_component(handle.entity()).ok()
            }
        }

        impl<'w, 's> ComponentSet<$T> for $ComponentsSet<'w, 's> {
            #[inline(always)]
            fn size_hint(&self) -> usize {
                0
            }

            #[inline(always)]
            fn for_each(&self, mut f: impl FnMut(Index, &$T)) {
                self.0.for_each(|$data| f($data.0.handle(), $data_expr))
            }
        }
    }
);

macro_rules! impl_component_set(
    ($ComponentsSet: ident, $T: ty, |$data: ident| $data_expr: expr) => {
        impl<'w, 's> ComponentSetOption<$T> for $ComponentsSet<'w, 's> {
            #[inline(always)]
            fn get(&self, handle: Index) -> Option<&$T> {
                self.0.q0().get_component(handle.entity()).ok()
            }
        }

        impl<'w, 's> ComponentSet<$T> for $ComponentsSet<'w, 's> {
            #[inline(always)]
            fn size_hint(&self) -> usize {
                0
            }

            #[inline(always)]
            fn for_each(&self, mut f: impl FnMut(Index, &$T)) {
                self.0.q0().for_each(|$data| f($data.0.handle(), $data_expr))
            }
        }
    }
);

macro_rules! impl_component_set_option(
    ($ComponentsSet: ident, $T: ty) => {
        impl<'w, 's> ComponentSetOption<$T> for $ComponentsSet<'w, 's> {
            #[inline(always)]
            fn get(&self, handle: Index) -> Option<&$T> {
                self.0.q0().get_component(handle.entity()).ok()
            }
        }
    }
);

pub type ComponentSetQueryMut<'w, 's, T> = QuerySet<(
    Query<'w, 's, (Entity, &'static T)>,
    Query<'w, 's, (Entity, &'static mut T)>,
)>;

pub struct QueryComponentSetMut<'w, 's, T: 'static + Send + Sync>(ComponentSetQueryMut<'w, 's, T>);

impl<'w, 's, T: 'static + Send + Sync> ComponentSetOption<T> for QueryComponentSetMut<'w, 's, T> {
    #[inline(always)]
    fn get(&self, handle: Index) -> Option<&T> {
        self.0.q0().get_component(handle.entity()).ok()
    }
}

impl<'w, 's, T: 'static + Send + Sync> ComponentSet<T> for QueryComponentSetMut<'w, 's, T> {
    #[inline(always)]
    fn size_hint(&self) -> usize {
        0
    }

    #[inline(always)]
    fn for_each(&self, mut f: impl FnMut(Index, &T)) {
        self.0.q0().for_each(|data| f(data.0.handle(), &data.1))
    }
}

impl<'w, 's, T: 'static + Send + Sync> ComponentSetMut<T> for QueryComponentSetMut<'w, 's, T> {
    #[inline(always)]
    fn set_internal(&mut self, handle: Index, val: T) {
        let _ = self
            .0
            .q1_mut()
            .get_mut(handle.entity())
            .map(|mut data| *data.1 = val);
    }

    #[inline(always)]
    fn map_mut_internal<Result>(
        &mut self,
        handle: Index,
        f: impl FnOnce(&mut T) -> Result,
    ) -> Option<Result> {
        self.0
            .q1_mut()
            .get_component_mut(handle.entity())
            .map(|mut data| f(&mut data))
            .ok()
    }
}

mod collider_component_set;
mod components;
mod mesh_collider;
mod plugins;
mod resources;
mod rigid_body_component_set;
mod systems;
