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
//! A cursor can be turned into an empty selection, and a selection can be turned into a cursor at either of its endpoints (for a mutable selection you have to pick one, since it consumes the selection).
//!
//! I can imagine it being particularly useful for `BTreeMap`, but the sketch in this crate uses a fake (`Vec`-backed) linked list for simplicity, to focus on the API shape rather than implementation details.
//!
//! # Examples
//!
//! Moving a paragraph down past the next one, like a text editor's "move block down" command:
//!
//! ```
//! # use collection_selection_confection::LinkedList;
//! # let mut lines = LinkedList::new();
//! # let mut cursor = lines.cursor_mut_at_start();
//! # for line in ["Roses are red,", "violets are blue.", "", "Sugar is sweet,", "and so are you."] {
//! #     cursor.insert_before(line.to_string());
//! # }
//! // lines:
//! //     Roses are red,
//! //     violets are blue.
//! //
//! //     Sugar is sweet,
//! //     and so are you.
//!
//! // Select the first paragraph and cut it, along with the blank line after it.
//! let mut selection = lines.cursor_mut_at_start().into_selection();
//! while selection.peek_after_end().is_some_and(|line| !line.is_empty()) {
//!     selection.move_end_next();
//! }
//! let paragraph = selection.take();
//! selection.remove_after_end();
//!
//! // Move past the next paragraph and paste it there.
//! let mut cursor = selection.into_cursor_at_end();
//! while cursor.peek_after().is_some_and(|line| !line.is_empty()) {
//!     cursor.move_next();
//! }
//! cursor.insert_before(String::new());
//! cursor.splice_before(paragraph);
//!
//! // lines:
//! //     Sugar is sweet,
//! //     and so are you.
//! //
//! //     Roses are red,
//! //     violets are blue.
//! # assert_eq!(
//! #     lines.iter().map(String::as_str).collect::<Vec<_>>(),
//! #     ["Sugar is sweet,", "and so are you.", "", "Roses are red,", "violets are blue."],
//! # );
//! ```
//!
//! Unwrapping hard-wrapped paragraphs into one line each, switching back and forth between a cursor and a selection:
//!
//! ```
//! # use collection_selection_confection::LinkedList;
//! # let mut lines = LinkedList::new();
//! # let mut cursor = lines.cursor_mut_at_start();
//! # for line in ["Roses are red,", "violets are blue.", "", "Sugar is sweet,", "and so are you."] {
//! #     cursor.insert_before(line.to_string());
//! # }
//! // lines:
//! //     Roses are red,
//! //     violets are blue.
//! //
//! //     Sugar is sweet,
//! //     and so are you.
//!
//! let mut cursor = lines.cursor_mut_at_start();
//! loop {
//!     // Select the next paragraph.
//!     let mut selection = cursor.into_selection();
//!     while selection.peek_after_end().is_some_and(|line| !line.is_empty()) {
//!         selection.move_end_next();
//!     }
//!
//!     // Replace it with a single line.
//!     let paragraph = selection.take();
//!     if !paragraph.is_empty() {
//!         let joined = paragraph.iter().map(String::as_str).collect::<Vec<_>>().join(" ");
//!         selection.insert_at_start(joined);
//!     }
//!
//!     // Step over the blank line, or stop if we're at the end.
//!     cursor = selection.into_cursor_at_end();
//!     if !cursor.move_next() {
//!         break;
//!     }
//! }
//!
//! // lines:
//! //     Roses are red, violets are blue.
//! //
//! //     Sugar is sweet, and so are you.
//! # assert_eq!(
//! #     lines.iter().map(String::as_str).collect::<Vec<_>>(),
//! #     ["Roses are red, violets are blue.", "", "Sugar is sweet, and so are you."],
//! # );
//! ```
mod list;

pub use list::{CursorMut, LinkedList, SelectionMut};
