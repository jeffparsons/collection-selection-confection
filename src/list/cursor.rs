use super::LinkedList;
use super::selection::SelectionMut;

// Not bothering with an immutable cursor for now.
pub struct CursorMut<'a, T> {
    list: &'a mut LinkedList<T>,
    index: usize,
}

impl<'a, T> CursorMut<'a, T> {
    pub(super) fn new(list: &'a mut LinkedList<T>, index: usize) -> Self {
        Self { list, index }
    }

    /// Converts into an empty selection at the cursor's position.
    pub fn into_selection(self) -> SelectionMut<'a, T> {
        SelectionMut::new(self.list, self.index, self.index)
    }

    /// Moves back one element.
    ///
    /// Returns `false`, without moving, if already at the start.
    pub fn move_prev(&mut self) -> bool {
        if self.index == 0 {
            return false;
        }
        self.index -= 1;
        true
    }

    /// Moves forward one element.
    ///
    /// Returns `false`, without moving, if already at the end.
    pub fn move_next(&mut self) -> bool {
        if self.index == self.list.items.len() {
            return false;
        }
        self.index += 1;
        true
    }

    /// Returns the element just before the cursor.
    pub fn peek_before(&self) -> Option<&T> {
        let i = self.index.checked_sub(1)?;
        self.list.items.get(i)
    }

    /// Returns the element just after the cursor.
    pub fn peek_after(&self) -> Option<&T> {
        self.list.items.get(self.index)
    }

    pub fn peek_before_mut(&mut self) -> Option<&mut T> {
        let i = self.index.checked_sub(1)?;
        self.list.items.get_mut(i)
    }

    pub fn peek_after_mut(&mut self) -> Option<&mut T> {
        self.list.items.get_mut(self.index)
    }

    /// Inserts an element just before the cursor.
    pub fn insert_before(&mut self, value: T) {
        self.list.items.insert(self.index, value);
        self.index += 1;
    }

    /// Inserts an element just after the cursor.
    pub fn insert_after(&mut self, value: T) {
        self.list.items.insert(self.index, value);
    }

    /// Removes and returns the element just before the cursor.
    pub fn remove_before(&mut self) -> Option<T> {
        if self.index == 0 {
            return None;
        }
        self.index -= 1;
        Some(self.list.items.remove(self.index))
    }

    /// Removes and returns the element just after the cursor.
    pub fn remove_after(&mut self) -> Option<T> {
        if self.index == self.list.items.len() {
            return None;
        }
        Some(self.list.items.remove(self.index))
    }

    /// Inserts all elements of `other` just before the cursor.
    pub fn splice_before(&mut self, other: LinkedList<T>) {
        let len = other.items.len();
        self.list.items.splice(self.index..self.index, other.items);
        self.index += len;
    }

    /// Inserts all elements of `other` just after the cursor.
    pub fn splice_after(&mut self, other: LinkedList<T>) {
        self.list.items.splice(self.index..self.index, other.items);
    }

    /// Splits off and returns all elements before the cursor.
    pub fn split_before(&mut self) -> LinkedList<T> {
        let after = self.list.items.split_off(self.index);
        let before = std::mem::replace(&mut self.list.items, after);
        self.index = 0;
        LinkedList { items: before }
    }

    /// Splits off and returns all elements after the cursor.
    pub fn split_after(&mut self) -> LinkedList<T> {
        LinkedList {
            items: self.list.items.split_off(self.index),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list::tests::list_of;

    #[test]
    fn insert_before_into_empty() {
        let mut list = LinkedList::new();
        list.cursor_mut_at_start().insert_before(1);
        assert_eq!(list.items, [1]);
    }

    #[test]
    fn insert_before_twice_keeps_order() {
        let mut list = LinkedList::new();
        let mut cursor = list.cursor_mut_at_start();
        cursor.insert_before(1);
        cursor.insert_before(2);
        assert_eq!(list.items, [1, 2]);
    }

    #[test]
    fn insert_after_twice_keeps_order() {
        let mut list = LinkedList::new();
        let mut cursor = list.cursor_mut_at_start();
        cursor.insert_after(2);
        cursor.insert_after(1);
        assert_eq!(cursor.peek_before(), None);
        assert_eq!(cursor.peek_after(), Some(&1));
        assert_eq!(list.items, [1, 2]);
    }

    #[test]
    fn peek_around_cursor() {
        let mut list = LinkedList::new();
        list.cursor_mut_at_start().insert_before(1);
        let mut cursor = list.cursor_mut_at_start();
        assert_eq!(cursor.peek_before(), None);
        assert_eq!(cursor.peek_after(), Some(&1));
        cursor.insert_before(0);
        assert_eq!(cursor.peek_before(), Some(&0));
        assert_eq!(cursor.peek_after(), Some(&1));
    }

    #[test]
    fn peek_mut_modifies_values() {
        let mut list = LinkedList::new();
        list.cursor_mut_at_start().insert_before(1);
        let mut cursor = list.cursor_mut_at_start();
        cursor.insert_before(0);
        *cursor.peek_before_mut().unwrap() += 10;
        *cursor.peek_after_mut().unwrap() += 20;
        assert_eq!(list.items, [10, 21]);
    }

    #[test]
    fn split_before_takes_elements_before_cursor() {
        let mut list = LinkedList::new();
        list.cursor_mut_at_start().insert_before(2);
        let mut cursor = list.cursor_mut_at_start();
        cursor.insert_before(0);
        cursor.insert_before(1);
        let before = cursor.split_before();
        assert_eq!(cursor.peek_before(), None);
        assert_eq!(cursor.peek_after(), Some(&2));
        assert_eq!(before.items, [0, 1]);
        assert_eq!(list.items, [2]);
    }

    #[test]
    fn split_after_takes_elements_after_cursor() {
        let mut list = LinkedList::new();
        list.cursor_mut_at_start().insert_before(2);
        let mut cursor = list.cursor_mut_at_start();
        cursor.insert_before(0);
        cursor.insert_before(1);
        let after = cursor.split_after();
        assert_eq!(cursor.peek_before(), Some(&1));
        assert_eq!(cursor.peek_after(), None);
        assert_eq!(after.items, [2]);
        assert_eq!(list.items, [0, 1]);
    }

    #[test]
    fn cursor_at_end_peeks_last() {
        let mut list = list_of(&[0, 1]);
        let cursor = list.cursor_mut_at_end();
        assert_eq!(cursor.peek_before(), Some(&1));
        assert_eq!(cursor.peek_after(), None);
    }

    #[test]
    fn move_stops_at_ends() {
        let mut list = list_of(&[0, 1]);
        let mut cursor = list.cursor_mut_at_start();
        assert!(!cursor.move_prev());
        assert!(cursor.move_next());
        assert_eq!(cursor.peek_before(), Some(&0));
        assert_eq!(cursor.peek_after(), Some(&1));
        assert!(cursor.move_next());
        assert!(!cursor.move_next());
        assert_eq!(cursor.peek_before(), Some(&1));
        assert!(cursor.move_prev());
        assert_eq!(cursor.peek_after(), Some(&1));
    }

    #[test]
    fn remove_on_each_side() {
        let mut list = list_of(&[0, 1, 2]);
        let mut cursor = list.cursor_mut_at_start();
        cursor.move_next();
        assert_eq!(cursor.remove_before(), Some(0));
        assert_eq!(cursor.remove_before(), None);
        assert_eq!(cursor.remove_after(), Some(1));
        assert_eq!(cursor.peek_after(), Some(&2));
        assert_eq!(cursor.remove_after(), Some(2));
        assert_eq!(cursor.remove_after(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn splice_before_puts_cursor_after_spliced() {
        let mut list = list_of(&[0, 3]);
        let mut cursor = list.cursor_mut_at_start();
        cursor.move_next();
        cursor.splice_before(list_of(&[1, 2]));
        assert_eq!(cursor.peek_before(), Some(&2));
        assert_eq!(cursor.peek_after(), Some(&3));
        assert_eq!(list.items, [0, 1, 2, 3]);
    }

    #[test]
    fn splice_after_keeps_cursor_before_spliced() {
        let mut list = list_of(&[0, 3]);
        let mut cursor = list.cursor_mut_at_start();
        cursor.move_next();
        cursor.splice_after(list_of(&[1, 2]));
        assert_eq!(cursor.peek_before(), Some(&0));
        assert_eq!(cursor.peek_after(), Some(&1));
        assert_eq!(list.items, [0, 1, 2, 3]);
    }
}
