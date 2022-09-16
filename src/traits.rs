use bevy::utils::hashbrown::{
    hash_map::{Drain, Values, ValuesMut},
    HashMap,
};
use std::{
    iter::{Filter, Map},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub trait EngineEntity {
    fn label(&self) -> &str;
}

#[derive(Debug, Default)]
pub struct EngineRepo<Entity, FromSource> {
    repo: HashMap<String, Entity>,
    /// This just for having FromSource generic type.
    _phantom: PhantomData<FromSource>,
}

// ! Combines Deref and DerefMut and our special functions for HashMap<String, Entity>.

impl<Entity, FromSource> Deref for EngineRepo<Entity, FromSource> {
    type Target = HashMap<String, Entity>;
    fn deref(&self) -> &Self::Target {
        &self.repo
    }
}
impl<Entity, FromSource> DerefMut for EngineRepo<Entity, FromSource> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.repo
    }
}

impl<Entity, FromSource> EngineRepo<Entity, FromSource>
where
    Entity: EngineEntity,
{
    #[inline]
    #[allow(clippy::type_complexity)]
    pub fn drain_all(&mut self) -> Map<Drain<'_, String, Entity>, fn((String, Entity)) -> Entity> {
        self.repo.drain().map(|(_, entity)| entity)
    }
    #[inline]
    pub fn drain_many(&mut self, mut predicate: impl FnMut(&&mut Entity) -> bool) -> Vec<Entity> {
        self.repo
            .drain_filter(|_, v| predicate(&v))
            .map(|(_, v)| v)
            .collect()
    }
    /// Drain one entity
    #[inline]
    pub fn drain_one(&mut self, predicate: impl FnMut(&&Entity) -> bool) -> Option<Entity> {
        let inner = &mut self.repo;
        if let Some(found) = inner.values().find(predicate) {
            return inner.remove(&found.label().to_owned());
        }
        None
    }
    // -------
    #[inline]
    pub fn delete_many(&mut self, predicate: impl FnMut(&&mut Entity) -> bool) -> bool {
        !self.drain_many(predicate).is_empty()
    }
    #[inline]
    pub fn delete(&mut self, predicate: impl FnMut(&&Entity) -> bool) -> bool {
        self.drain_one(predicate).is_some()
    }
    // -------
    #[inline]
    pub fn find_many(&self, predicate: impl FnMut(&&Entity) -> bool) -> Vec<&Entity> {
        self.repo.values().filter(predicate).collect()
    }
    #[inline]
    pub fn find(&self, predicate: impl FnMut(&&Entity) -> bool) -> Option<&Entity> {
        self.repo.values().find(predicate)
    }
    // -------
    #[inline]
    pub fn find_mut_many(
        &mut self,
        predicate: impl FnMut(&&mut Entity) -> bool,
    ) -> Vec<&mut Entity> {
        self.repo.values_mut().filter(predicate).collect()
    }
    #[inline]
    pub fn find_mut(&mut self, predicate: impl FnMut(&&mut Entity) -> bool) -> Option<&mut Entity> {
        self.repo.values_mut().find(predicate)
    }
    // -------
    #[inline]
    pub fn filter<O: FnMut(&&Entity) -> bool>(
        &self,
        predicate: O,
    ) -> Filter<Values<'_, String, Entity>, O> {
        self.repo.values().filter(predicate)
    }
    #[inline]
    pub fn filter_mut<O: FnMut(&&mut Entity) -> bool>(
        &mut self,
        predicate: O,
    ) -> Filter<ValuesMut<'_, String, Entity>, O> {
        self.repo.values_mut().filter(predicate)
    }
    // -------
    #[inline]
    pub fn for_each(&self, f: impl FnMut(&Entity)) {
        self.repo.values().for_each(f)
    }
    #[inline]
    pub fn for_each_mut(&mut self, f: impl FnMut(&mut Entity)) {
        self.repo.values_mut().for_each(f)
    }
}
