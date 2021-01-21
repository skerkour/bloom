
#[cfg(feature = "headers")]
pub use self::item::Item;

pub use self::vec_map::VecMap;

#[cfg(feature = "headers")]
pub use self::vec_map::Entry;

#[cfg(feature = "headers")]
mod cell;

#[cfg(feature = "headers")]
mod item;

mod vec_map;
