use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::cmp::{PartialEq, Eq};
use std::fmt::{Display, Formatter, Error};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug)]
pub struct Id<E, K> {
    val: E,
    phantom: PhantomData<K>,
}

impl<E: Display, K> Display for Id<E, K> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        self.val.fmt(fmt)
    }
}

impl<E: Clone, K> Clone for Id<E, K> {
    fn clone(&self) -> Self {
        Id {
            val: self.val.clone(),
            phantom: PhantomData,
        } 
    }
}

impl<E: Copy, K> Copy for Id<E, K> {}

impl<E, K> Deref for Id<E, K> {
    type Target = E;

    fn deref<'a>(&'a self) -> &'a E {
        &self.val
    }
}

impl<E, K> DerefMut for Id<E, K> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut E {
        &mut self.val
    }
}

impl<E: PartialEq, K> PartialEq for Id<E, K> {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<E: Eq, K> Eq for Id<E, K> {}

impl<E: Hash, K> Hash for Id<E, K> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.val.hash(state);
    }
}

impl<E: FromStr, K> FromStr for Id<E, K> {
    type Err = E::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        E::from_str(s).map(|id| Self::new(id))
    }
}

impl<E, K> Id<E, K> {
    pub fn new(val: E) -> Id<E, K> {
        Id {
            val: val,
            phantom: PhantomData,
        }
    }
}
