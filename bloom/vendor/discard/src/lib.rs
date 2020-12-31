//! There are situations where you need to intentionally leak *some*
//! memory but not *other* memory. This crate can help!
//!
//! But before I explain, you are probably wondering: why would I
//! want to leak memory in the first place?
//!
//! There are certain rare situations where leaking memory is either
//! desirable or *necessary*.
//!
//! As an example, let's say I am using [stdweb](https://crates.io/crates/stdweb),
//! which lets me use JavaScript APIs in Rust.
//!
//! So I write some code like this:
//!
//! ```rust,ignore
//! node.add_event_listener(|event: ClickEvent| {
//!     // ...
//! });
//! ```
//!
//! Seems reasonable, right? But there's a problem: the `add_event_listener` method
//! returns an `EventListenerHandle`, and when the `EventListenerHandle` is dropped it
//! will remove the event listener.
//!
//! Because I'm not using the `EventListenerHandle`, it is immediately dropped, so it
//! receives no events!
//!
//! Okay, no problem, just use [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html):
//!
//! ```rust,ignore
//! // This will automatically remove the event listener when `handle` is dropped
//! let handle = node.add_event_listener(|event: ClickEvent| {
//!     // ...
//! });
//!
//! // Now it will no longer remove the event listener
//! std::mem::forget(handle);
//! ```
//!
//! Now the event listener will remain alive forever, which is what I want.
//!
//! But there's two problems with this:
//!
//! 1) I want it to keep the event listener alive forever, but I *also* want it to clean up any
//!    unused internal memory. Using [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html)
//!    causes it to leak ***all*** of the memory, which is wasteful.
//!
//! 2) There are situations where I want to leak the event listener, and then later *unleak* it.
//!    That's not possible with [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html).
//!
//! The solution to all of these problems is:
//!
//! 1. The `EventListenerHandle` should ***not*** implement the [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) trait.
//!
//! 2. The `EventListenerHandle` should implement the [`Discard`](trait.Discard.html) trait instead.
//!
//! 3. The `add_event_listener` method should return `DiscardOnDrop<EventListenerHandle>`.
//!
//! Now let's look at what is possible:
//!
//! ```rust,ignore
//! // This will automatically remove the event listener when `handle` is dropped
//! let handle = node.add_event_listener(|event: ClickEvent| {
//!     // ...
//! });
//!
//! // Now it will no longer remove the event listener, this is similar to `std::mem::forget`
//! let leaked = DiscardOnDrop::leak(handle);
//!
//! // Now it will remove the event listener, even though it was leaked
//! leaked.discard();
//! ```
//!
//! There's two huge differences between [`DiscardOnDrop::leak`](struct.DiscardOnDrop.html#method.leak)
//! and [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html):
//!
//! 1) [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html) leaks ***all*** of the memory,
//!    [`DiscardOnDrop::leak`](struct.DiscardOnDrop.html#method.leak) leaks the *minimal*
//!    amount of memory: unused memory is properly cleaned up.
//!
//! 2) With [`std::mem::forget`](https://doc.rust-lang.org/std/mem/fn.forget.html) you cannot
//!    clean up a value after it has been leaked, but with
//!    [`DiscardOnDrop::leak`](struct.DiscardOnDrop.html#method.leak) you can manually discard
//!    the value even after it has been leaked.
//!
//! Most of the time you don't need to worry about any of this:
//! [`DiscardOnDrop`](struct.DiscardOnDrop.html) will automatically call
//! [`discard`](trait.Discard.html#tymethod.discard) when it is dropped, so in that situation
//! [`Discard`](trait.Discard.html) behaves the same as [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html).
//!
//! So you can use normal Rust idioms, and everything works as you would expect. You only need to
//! worry about [`Discard`](trait.Discard.html) when you need to intentionally leak some memory.

#![deny(
    missing_docs,
    missing_debug_implementations
)]

use std::ops::{Deref, DerefMut};
use std::mem::ManuallyDrop;

/// This trait is very similar to [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html):
/// it allows for cleaning up memory and resources when they are no longer needed.
///
/// However, unlike [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) you need to
/// manually call the [`discard`](#tymethod.discard) method.
///
/// It is extremely common to use [`DiscardOnDrop`](struct.DiscardOnDrop.html), which will cause it
/// to automatically call the [`discard`](#tymethod.discard) method when it is dropped. In that situation
/// [`Discard`](trait.Discard.html) behaves exactly the same as [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html).
///
/// You can use [`DiscardOnDrop::leak`](struct.DiscardOnDrop.html#method.leak) to intentionally leak the value
/// (which causes [`discard`](#tymethod.discard) to not be called), and then later you can manually call
/// [`discard`](#tymethod.discard) to clean up the resources, even after the resources have been leaked.
///
/// See the [module documentation](index.html) for more details.
pub trait Discard {
    /// This consumes the value and cleans up any memory / resources / etc. that the value was
    /// using.
    ///
    /// See the [module documentation](index.html) for more details.
    fn discard(self);
}


/// If you have a value which implements [`Discard`](trait.Discard.html), you can use
/// [`DiscardOnDrop::new(value)`](struct.DiscardOnDrop.html#method.new) which will wrap the value.
/// When the wrapper is dropped it will automatically call [`value.discard()`](trait.Discard.html#tymethod.discard).
///
/// You can use the [`DiscardOnDrop::leak`](#method.leak) function to unwrap it (which returns `value`). This causes
/// it to no longer call [`discard`](trait.Discard.html#tymethod.discard) when it is dropped, which
/// means it will leak memory unless you manually call [`discard`](trait.Discard.html#tymethod.discard).
///
/// See the [module documentation](index.html) for more details.
#[must_use = "
The DiscardOnDrop is unused, which causes it to be immediately discarded.
You probably don't want that to happen.

How to fix this:

  * Store the DiscardOnDrop in a variable or data structure.

  * Or use the DiscardOnDrop::leak function which will cause it to not be
    discarded (this *will* leak memory!).

See the DiscardOnDrop documentation for more details."]
#[derive(Debug)]
pub struct DiscardOnDrop<A: Discard>(ManuallyDrop<A>);

impl<A: Discard> DiscardOnDrop<A> {
    /// Creates a new `DiscardOnDrop`.
    ///
    /// When the `DiscardOnDrop` is dropped it will automatically call [`discarder.discard()`](trait.Discard.html#tymethod.discard).
    ///
    /// See the [module documentation](index.html) for more details.
    #[inline]
    pub fn new(discarder: A) -> Self {
        DiscardOnDrop(ManuallyDrop::new(discarder))
    }

    /// Returns the wrapped `discarder`.
    ///
    /// It will no longer automatically call [`discarder.discard()`](trait.Discard.html#tymethod.discard), so this ***will*** leak memory
    /// unless you manually call [`discarder.discard()`](trait.Discard.html#tymethod.discard).
    ///
    /// See the [module documentation](index.html) for more details.
    ///
    /// This is implemented as a function (*not* a method) so that way it doesn't interfere with any of the
    /// methods on `discarder`.
    #[inline]
    pub fn leak(this: Self) -> A {
        // We want to move the `A` out of `this`, but that's not allowed because `this` implements `Drop`
        // (and we must also avoid calling `drop()` on `this` or else `A` would get dropped twice).
        //
        // We can do that move by using the unsafe function std::ptr::read(),
        // and then use `mem::forget()` on `this` so it never gets dropped. The `A` will get dropped by the caller.
        //
        // TODO verify that this is completely safe
        unsafe {
            let value: A = ::std::ptr::read(this.0.deref());
            ::std::mem::forget(this);
            value
        }
    }
}

impl<A: Discard> Drop for DiscardOnDrop<A> {
    #[inline]
    fn drop(&mut self) {
        // This only gets called if there is still a valid `A` inside the `ManuallyDrop`,
        // since in `leak()` we prevent `drop()` from being called.
        //
        // Similar to `leak()`, we want to move `A` out of `self` but again we can't,
        // this time because we only have a mutable reference, not a value.
        //
        // The solution is the same though, use `std::ptr::read()` to do the move,
        // the `A` will get dropped by `.discard()` and since we wrapped it in `ManuallyDrop`,
        // it won't be dropped again at the end of this function.
        //
        // TODO verify that this is completely safe
        unsafe {
            let value: A = ::std::ptr::read(self.0.deref());
            value.discard();
        }
    }
}

impl<A: Discard> Deref for DiscardOnDrop<A> {
    type Target = A;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<A: Discard> DerefMut for DiscardOnDrop<A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}


#[cfg(test)]
mod tests {
    use super::{Discard, DiscardOnDrop};
    use std::rc::Rc;
    use std::cell::Cell;

    struct Foo(Rc<Cell<bool>>);

    impl Foo {
        fn new() -> Self {
            Foo(Rc::new(Cell::new(false)))
        }

        fn dropped(&self) -> Rc<Cell<bool>> {
            self.0.clone()
        }

        fn as_mut(&mut self) -> &mut Self {
            self
        }
    }

    impl Discard for Foo {
        fn discard(self) {
            self.0.set(true);
        }
    }


    #[test]
    fn unused() {
        Foo::new();
    }

    #[test]
    fn unused_discard_on_drop() {
        DiscardOnDrop::new(Foo::new());
        DiscardOnDrop::new(Foo::new());
    }

    #[test]
    fn discard() {
        let foo = Foo::new();

        let dropped = foo.dropped();

        assert_eq!(dropped.get(), false);
        foo.discard();
        assert_eq!(dropped.get(), true);
    }

    #[test]
    fn no_discard() {
        let foo = Foo::new();

        let dropped = foo.dropped();

        assert_eq!(dropped.get(), false);
        drop(foo);
        assert_eq!(dropped.get(), false);
    }

    #[test]
    fn discard_on_drop() {
        let foo = DiscardOnDrop::new(Foo::new());

        let dropped = foo.dropped();

        assert_eq!(dropped.get(), false);
        drop(foo);
        assert_eq!(dropped.get(), true);
    }

    #[test]
    fn leak() {
        let foo = DiscardOnDrop::new(Foo::new());

        let dropped = foo.dropped();

        assert_eq!(dropped.get(), false);
        drop(DiscardOnDrop::leak(foo));
        assert_eq!(dropped.get(), false);
    }

    #[test]
    fn deref_mut() {
        let mut foo = DiscardOnDrop::new(Foo::new());

        let dropped = foo.as_mut().dropped();

        assert_eq!(dropped.get(), false);
        drop(DiscardOnDrop::leak(foo));
        assert_eq!(dropped.get(), false);
    }
}
