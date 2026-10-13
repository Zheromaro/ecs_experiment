mod custom_errors;
mod entities;
pub mod resource;
use eyre::Result;

use crate::{
    entities::{Entities, query::Query},
    resource::Resource,
};
use std::any::Any;

#[derive(Debug, Default)]
pub struct World {
    resources: Resource,
    entities: Entities,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }

    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }

    /// Query for a resource and get a mutable refrence to it.
    /// the type of the resource must be added in to find it
    /// '''
    /// use ecs::World;
    /// let mut world = World::new();
    /// world.add_resource(10_u32);
    /// {
    ///     let resource = world.`get_resource_mut::<u32>`().unwarp();
    ///     *resource += 1;
    /// }
    /// let resource = world.`get_resource::<u32>`().unwarp();
    /// assert_eq(*resource, 11);
    /// '''
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }

    /// This will remove the resource from the world, and it doesn't care if the resource exist at this point of time.
    ///
    pub fn delete_resource<T: Any>(&mut self) {
        self.resources.remove::<T>();
    }

    pub fn register_component<T: Any + 'static>(&mut self) {
        self.entities.register_component::<T>();
    }

    pub fn create_entity(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }

    pub fn query(&self) -> Query<'_> {
        Query::new(&self.entities)
    }

    pub fn delete_component_by_entity_id<T: Any>(&mut self, index: usize) -> Result<()> {
        self.entities.delete_component_by_entity_id::<T>(index)
    }
}
