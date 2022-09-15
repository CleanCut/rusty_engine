use std::{
    convert::TryInto,
    iter::{Filter, Map},
};

use bevy::utils::hashbrown::{
    hash_map::{Drain, Iter, IterMut, Values, ValuesMut},
    HashMap,
};

pub type EntityMap<T> = bevy::utils::hashbrown::HashMap<String, T>;

pub trait Entity {
    type Source;
    fn new<L: Into<String>, S: Into<Self::Source>>(label: L, from: S) -> Self;
    fn label(&self) -> &str;
}

pub trait Reposit<T: Entity>
where
    Self: Default,
{
    fn inner(&self) -> &HashMap<String, T>;
    fn inner_mut(&mut self) -> &mut HashMap<String, T>;
    #[inline]
    /// This is same as classic one (`sprites` or `texts`).iter()
    fn inner_iter(&self) -> Iter<String, T> {
        self.inner().iter()
    }
    #[inline]
    /// This is same as classic one (`sprites` or `texts`).iter_mut()
    fn inner_iter_mut(&mut self) -> IterMut<String, T> {
        self.inner_mut().iter_mut()
    }
    #[inline]
    /// Reset the hash map
    fn reset(&mut self) {
        *self = Default::default();
    }
    // -------
    /// Same as values()
    #[inline]
    fn iter(&self) -> Values<String, T> {
        self.inner().values()
    }
    // -------
    /// Same as values_mut()
    #[inline]
    fn iter_mut(&mut self) -> ValuesMut<String, T> {
        self.inner_mut().values_mut()
    }
    // -------
    #[inline]
    fn values(&self) -> Values<String, T> {
        self.inner().values()
    }
    // -------
    #[inline]
    fn values_mut(&mut self) -> ValuesMut<String, T> {
        self.inner_mut().values_mut()
    }
    // -------
    #[inline]
    fn clear(&mut self) {
        self.inner_mut().clear();
    }
    // -------
    #[inline]
    fn get_many_mut<const N: usize>(&mut self, label: [&str; N]) -> Option<[&'_ mut T; N]> {
        self.inner_mut().get_many_mut(label)
    }
    #[inline]
    fn get(&self, label: impl AsRef<str>) -> Option<&T> {
        self.inner().get(label.as_ref())
    }
    #[inline]
    fn get_mut(&mut self, label: impl AsRef<str>) -> Option<&mut T> {
        self.inner_mut().get_mut(label.as_ref())
    }
    // -------
    #[inline]
    fn remove(&mut self, label: impl AsRef<str>) -> Option<T> {
        self.inner_mut().remove(label.as_ref())
    }
    // -------
    #[inline]
    fn add_many<L: Into<String>, S: Into<T::Source>, const N: usize>(
        &mut self,
        ones: [(L, S); N],
    ) -> [&mut T; N] {
        let inner = self.inner_mut();
        let ones = ones.map(|(label, from)| {
            let label = label.into();
            inner.insert(label.clone(), T::new(label.clone(), from.into()));
            label
        });
        // TODO! optimization
        inner
            .get_many_mut(
                ones.iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
            .unwrap()
    }
    #[inline]
    /// Create and add a [`Entity`] to the game. Use the `&mut Target` that is returned to adjust
    /// the translation, rotation, etc. Use a *unique* label for each sprite. Attempting to add two
    /// sprites with the same label will cause a crash.
    fn add<L: Into<String>, S: Into<T::Source>>(&mut self, label: L, from: S) -> &mut T {
        let inner = self.inner_mut();
        let label = label.into();
        inner.insert(label.clone(), T::new(&label, from));
        // Unwrap: Can't crash because we just inserted the sprite
        inner.get_mut(&label).unwrap()
    }
    // -------
    #[inline]
    fn add_clod_many<const N: usize>(&mut self, ones: [T; N]) -> [&mut T; N] {
        let inner = self.inner_mut();

        let ones = ones.map(|sprite| {
            let label = sprite.label().to_string();
            inner.insert(label.clone(), sprite);
            label
        });
        // TODO! optimization
        inner
            .get_many_mut(
                ones.iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            )
            .unwrap()
    }
    #[inline]
    fn add_clod(&mut self, one: T) -> &mut T {
        let inner = self.inner_mut();
        let label = one.label().to_string();
        inner.insert(label.to_owned(), one);
        // Unwrap: Can't crash because we just inserted the sprite
        inner.get_mut(&label).unwrap()
    }
    // -------
    #[inline]
    #[allow(clippy::type_complexity)]
    fn drain_all(&mut self) -> Map<Drain<'_, String, T>, fn((String, T)) -> T> {
        self.inner_mut().drain().map(|(_, sprite)| sprite)
    }
    #[inline]
    fn drain_many(&mut self, mut predicate: impl FnMut(&&mut T) -> bool) -> Vec<T> {
        self.inner_mut()
            .drain_filter(|_, v| predicate(&v))
            .map(|(_, v)| v)
            .collect()
    }
    /// Drain one entity
    #[inline]
    fn drain(&mut self, predicate: impl FnMut(&&T) -> bool) -> Option<T> {
        let inner = self.inner_mut();
        if let Some(found) = inner.values().find(predicate) {
            return inner.remove(&found.label().to_owned());
        }
        None
    }
    // -------
    #[inline]
    fn delete_many(&mut self, predicate: impl FnMut(&&mut T) -> bool) -> bool {
        !self.drain_many(predicate).is_empty()
    }
    #[inline]
    fn delete(&mut self, predicate: impl FnMut(&&T) -> bool) -> bool {
        self.drain(predicate).is_some()
    }
    // -------
    #[inline]
    fn find_many(&self, predicate: impl FnMut(&&T) -> bool) -> Vec<&T> {
        self.inner().values().filter(predicate).collect()
    }
    #[inline]
    fn find(&self, predicate: impl FnMut(&&T) -> bool) -> Option<&T> {
        self.inner().values().find(predicate)
    }
    // -------
    #[inline]
    fn find_mut_many(&mut self, predicate: impl FnMut(&&mut T) -> bool) -> Vec<&mut T> {
        self.inner_mut().values_mut().filter(predicate).collect()
    }
    #[inline]
    fn find_mut(&mut self, predicate: impl FnMut(&&mut T) -> bool) -> Option<&mut T> {
        self.inner_mut().values_mut().find(predicate)
    }
    // -------
    #[inline]
    fn filter<O: FnMut(&&T) -> bool>(&self, predicate: O) -> Filter<Values<'_, String, T>, O> {
        self.inner().values().filter(predicate)
    }
    #[inline]
    fn filter_mut<O: FnMut(&&mut T) -> bool>(
        &mut self,
        predicate: O,
    ) -> Filter<ValuesMut<'_, String, T>, O> {
        self.inner_mut().values_mut().filter(predicate)
    }
    // -------
    #[inline]
    fn for_each(&self, f: impl FnMut(&T)) {
        self.inner().values().for_each(f)
    }
    #[inline]
    fn for_each_mut(&mut self, f: impl FnMut(&mut T)) {
        self.inner_mut().values_mut().for_each(f)
    }
}
