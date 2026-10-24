//! A sketch of a _selection_ API for collections that could live alongside _cursors_.
//!
//! The Rust standard library includes unstable cursor APIs for `LinkedList` ([RFC](https://github.com/rust-lang/rfcs/pull/2570), [tracking issue](https://github.com/rust-lang/rust/issues/58533)) and `BTreeMap`/`BTreeSet` ([API change proposal](https://github.com/rust-lang/libs-team/issues/141), [tracking issue](https://github.com/rust-lang/rust/issues/107540)).
//! There has been a lot of bikeshedding over the years about what exactly these interfaces should look like.
//! One point of contention has been whether a cursor should point to _elements_ in a collection, or _gaps_ between those elements.
//! The current APIs in `std` have gone in opposite directions here: `LinkedList` cursors point at elements, whereas `BTreeMap` cursors were [rewritten](https://github.com/rust-lang/rust/pull/118208) to point at gaps instead.
//!
//! I've personally leaned toward the "gaps" formulation if we can only have one.
//! At some point I also proposed having two separate cursor types and converting between them as you navigate, e.g., calling `next_gap(self)` on an `ElemCursor` would always successfully return a `GapCursor`.
//! But nobody liked that idea. :)
//!
//! More recently I've been thinking about how useful it could be to instead have a separate concept of collection _selections_, analogous to selections in a text editor.
//! A selection is:
//!
//! - Like a cursor in that you can move its start and end, and insert, modify, and delete elements around those endpoints.
//! - Like a range iterator in that it allows you to iterate over some subset of a collection.
//! - Like `extract_if` (formerly `drain_filter`), `split_off`, etc. in that it could allow efficiently removing or splitting off part(s) of a collection.
//!
//! A cursor can be turned into an empty selection, and a selection can be turned into a cursor at either of its endpoints (only one of them, for a mutable selection).
//!
//! I can imagine it being particularly useful for `BTreeMap`, but the example in this crate uses a fake (`Vec`-backed) linked list for simplicity, to focus on the API shape rather than implementation details.
mod list;

pub use list::{CursorMut, LinkedList, SelectionMut};
