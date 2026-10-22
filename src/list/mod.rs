mod cursor;

pub use cursor::CursorMut;

/// A doubly-linked list.
///
/// The name is a lie; it is actually backed by a `Vec`. The interface reflects what would be possible with a real linked list.
pub struct LinkedList<T> {
    items: Vec<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> + ExactSizeIterator {
        self.items.iter_mut()
    }

    pub fn cursor_mut_at_start<'a>(&'a mut self) -> CursorMut<'a, T> {
        CursorMut::new(self, 0)
    }

    pub fn cursor_mut_at_end<'a>(&'a mut self) -> CursorMut<'a, T> {
        let index = self.items.len();
        CursorMut::new(self, index)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(super) fn list_of(values: &[i32]) -> LinkedList<i32> {
        let mut list = LinkedList::new();
        let mut cursor = list.cursor_mut_at_start();
        for &value in values {
            cursor.insert_left(value);
        }
        list
    }

    #[test]
    fn new_empty() {
        let _ = LinkedList::<i32>::new();
    }

    #[test]
    fn default_is_empty() {
        let list = LinkedList::<i32>::default();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn len_counts_elements() {
        let list = list_of(&[0, 1, 2]);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn iter_both_directions() {
        let list = list_of(&[0, 1, 2]);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), [0, 1, 2]);
        assert_eq!(list.iter().rev().copied().collect::<Vec<_>>(), [2, 1, 0]);
    }

    #[test]
    fn iter_mut_modifies_values() {
        let mut list = list_of(&[0, 1, 2]);
        for value in list.iter_mut() {
            *value *= 10;
        }
        assert_eq!(list.items, [0, 10, 20]);
    }
}
