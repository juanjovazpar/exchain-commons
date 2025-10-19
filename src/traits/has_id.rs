use std::{hash::Hash};

/* 
    Types implementing `HasId` must define an associated `Id` type
    that is `Eq`, `Hash`, and `Clone`, making it suitable for use
    as a key in hash-based collections.
*/
pub trait HasId {
    type Id: Eq + Hash + Clone;
    fn id(&self) -> Self::Id;
}