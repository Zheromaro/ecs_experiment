use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

#[derive(Debug, Default)]
pub struct Entities {
    components: HashMap<TypeId, Vec<Option<Rc<RefCell<dyn Any>>>>>,
}

impl Entities {
    fn register_component<T: Any + 'static>(&mut self) {
        self.components.insert(TypeId::of::<T>(), vec![]);
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use super::*;

    #[test]
    fn register_entity() {
        let mut entities = Entities::default();
        entities.register_component::<Health>();
        let health_component = entities.components.get(&TypeId::of::<Health>()).unwrap();
        assert_eq!(health_component.len(), 0);
    }

    struct Health(pub u32);
}
