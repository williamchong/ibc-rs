use core::marker::PhantomData;
use core::fmt::{self, Debug, Display};
use core::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};

pub struct Tagged<Tag, Value>(pub Value, pub PhantomData<Tag>);

impl <Tag, Value> Tagged<Tag, Value> {
  pub fn new(value: Value) -> Self {
    Tagged(value, PhantomData)
  }

  pub fn value(&self) -> &Value {
    &self.0
  }

  pub fn untag(self) -> Value {
    self.0
  }

  pub fn map<T>(&self, mapper: impl FnOnce(&Value) -> T) ->
    Tagged<Tag, T>
  {
    Tagged::new(mapper(self.value()))
  }

  pub fn map_into<T>(self, mapper: impl FnOnce(Value) -> T) ->
    Tagged<Tag, T>
  {
    Tagged::new(mapper(self.0))
  }
}

impl <Tag, Value> Tagged<Tag, Option<Value>> {
  pub fn transpose(self) -> Option<Tagged<Tag, Value>> {
    self.0.map(Tagged::new)
  }
}

impl <Tag, Value, E> Tagged<Tag, Result<Value, E>> {
  pub fn transpose(self) -> Result<Tagged<Tag, Value>, E> {
    self.0.map(Tagged::new)
  }
}

impl <Tag, Value: Clone> Clone for Tagged<Tag, Value> {
  fn clone(&self) -> Self {
    Self::new(self.0.clone())
  }
}

impl <Tag, Value: Debug> Debug for Tagged<Tag, Value> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Debug::fmt(self.value(), f)
  }
}

impl <Tag, Value: Display> Display for Tagged<Tag, Value> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Display::fmt(self.value(), f)
  }
}

impl <Tag, Value: PartialEq> PartialEq for Tagged<Tag, Value> {
  fn eq(&self, other: &Self) -> bool {
    self.value().eq(other.value())
  }
}

impl <Tag, Value: Eq> Eq for Tagged<Tag, Value> {}

impl <Tag, Value: PartialOrd> PartialOrd for Tagged<Tag, Value> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      self.value().partial_cmp(other.value())
    }
}

impl <Tag, Value: Ord> Ord for Tagged<Tag, Value> {
    fn cmp(&self, other: &Self) -> Ordering {
      self.value().cmp(other.value())
    }
}