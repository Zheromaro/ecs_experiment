use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomErrors {
    #[error("Attempting to add component to an entity without calling create component first")]
    CreateComponentNeverCalled,
    #[error("attempted to reference a component that wasn't registered")]
    ComponentNotRegistered,
    #[error("attempted to reference an entity that wasn't registered")]
    EntityDoesNotExist,
    #[error("attempted to get component data that does not exist")]
    ComponentDataDoesNotExist,
}
