mod resource;
use crate::resource::Resource;
use std::any::Any;

#[derive(Debug, Default)]
pub struct World {
    resources: Resource,
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
    ///
    pub fn get_resource_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.resources.get_mut::<T>()
    }
}
