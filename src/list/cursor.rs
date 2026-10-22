use super::LinkedList;

// Not bothering with an immutable cursor for now.
pub struct CursorMut<'a, T> {
    list: &'a mut LinkedList<T>,
    index: usize,
}

impl<'a, T> CursorMut<'a, T> {
    pub(super) fn new(list: &'a mut LinkedList<T>, index: usize) -> Self {
        Self { list, index }
    }

    pub fn move_left(&mut self) -> bool {
        if self.index == 0 {
            return false;
        }
        self.index -= 1;
        true
    }

    pub fn move_right(&mut self) -> bool {
        if self.index == self.list.items.len() {
            return false;
        }
        self.index += 1;
        true
    }

    pub fn peek_left(&self) -> Option<&T> {
        let i = self.index.checked_sub(1)?;
        self.list.items.get(i)
    }

    pub fn peek_right(&self) -> Option<&T> {
        self.list.items.get(self.index)
    }

    pub fn peek_left_mut(&mut self) -> Option<&mut T> {
        let i = self.index.checked_sub(1)?;
        self.list.items.get_mut(i)
    }

    pub fn peek_right_mut(&mut self) -> Option<&mut T> {
        self.list.items.get_mut(self.index)
    }

    pub fn insert_left(&mut self, value: T) {
        self.list.items.insert(self.index, value);
        self.index += 1;
    }

    pub fn insert_right(&mut self, value: T) {
        self.list.items.insert(self.index, value);
    }

    pub fn remove_left(&mut self) -> Option<T> {
        if self.index == 0 {
            return None;
        }
        self.index -= 1;
        Some(self.list.items.remove(self.index))
    }

    pub fn remove_right(&mut self) -> Option<T> {
        if self.index == self.list.items.len() {
            return None;
        }
        Some(self.list.items.remove(self.index))
    }

    pub fn splice_left(&mut self, other: LinkedList<T>) {
        let len = other.items.len();
        self.list.items.splice(self.index..self.index, other.items);
        self.index += len;
    }

    pub fn splice_right(&mut self, other: LinkedList<T>) {
        self.list.items.splice(self.index..self.index, other.items);
    }

    pub fn split_left(&mut self) -> LinkedList<T> {
        let right = self.list.items.split_off(self.index);
        let left = std::mem::replace(&mut self.list.items, right);
        self.index = 0;
        LinkedList { items: left }
    }

    pub fn split_right(&mut self) -> LinkedList<T> {
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
    fn insert_left_into_empty() {
        let mut list = LinkedList::new();
        list.cursor_mut_at_start().insert_left(1);
        assert_eq!(list.items, [1]);
    }

    #[test]
    fn insert_left_twice_keeps_order() {
        let mut list = LinkedList::new();
        let mut cursor = list.cursor_mut_at_start();
        cursor.insert_left(1);
        cursor.insert_left(2);
        assert_eq!(list.items, [1, 2]);
    }

    #[test]
    fn insert_right_twice_keeps_order() {
        let mut list = LinkedList::new();
        let mut cursor = list.cursor_mut_at_start();
        cursor.insert_right(2);
        cursor.insert_right(1);
        assert_eq!(cursor.peek_left(), None);
        assert_eq!(cursor.peek_right(), Some(&1));
        assert_eq!(list.items, [1, 2]);
    }

    #[test]
    fn peek_around_cursor() {
        let mut list = LinkedList::new();
        list.cursor_mut_at_start().insert_left(1);
        let mut cursor = list.cursor_mut_at_start();
        assert_eq!(cursor.peek_left(), None);
        assert_eq!(cursor.peek_right(), Some(&1));
        cursor.insert_left(0);
        assert_eq!(cursor.peek_left(), Some(&0));
        assert_eq!(cursor.peek_right(), Some(&1));
    }

    #[test]
    fn peek_mut_modifies_values() {
        let mut list = LinkedList::new();
        list.cursor_mut_at_start().insert_left(1);
        let mut cursor = list.cursor_mut_at_start();
        cursor.insert_left(0);
        *cursor.peek_left_mut().unwrap() += 10;
        *cursor.peek_right_mut().unwrap() += 20;
        assert_eq!(list.items, [10, 21]);
    }

    #[test]
    fn split_left_takes_elements_before_cursor() {
        let mut list = LinkedList::new();
        list.cursor_mut_at_start().insert_left(2);
        let mut cursor = list.cursor_mut_at_start();
        cursor.insert_left(0);
        cursor.insert_left(1);
        let left = cursor.split_left();
        assert_eq!(cursor.peek_left(), None);
        assert_eq!(cursor.peek_right(), Some(&2));
        assert_eq!(left.items, [0, 1]);
        assert_eq!(list.items, [2]);
    }

    #[test]
    fn split_right_takes_elements_after_cursor() {
        let mut list = LinkedList::new();
        list.cursor_mut_at_start().insert_left(2);
        let mut cursor = list.cursor_mut_at_start();
        cursor.insert_left(0);
        cursor.insert_left(1);
        let right = cursor.split_right();
        assert_eq!(cursor.peek_left(), Some(&1));
        assert_eq!(cursor.peek_right(), None);
        assert_eq!(right.items, [2]);
        assert_eq!(list.items, [0, 1]);
    }

    #[test]
    fn cursor_at_end_peeks_last() {
        let mut list = list_of(&[0, 1]);
        let cursor = list.cursor_mut_at_end();
        assert_eq!(cursor.peek_left(), Some(&1));
        assert_eq!(cursor.peek_right(), None);
    }

    #[test]
    fn move_stops_at_ends() {
        let mut list = list_of(&[0, 1]);
        let mut cursor = list.cursor_mut_at_start();
        assert!(!cursor.move_left());
        assert!(cursor.move_right());
        assert_eq!(cursor.peek_left(), Some(&0));
        assert_eq!(cursor.peek_right(), Some(&1));
        assert!(cursor.move_right());
        assert!(!cursor.move_right());
        assert_eq!(cursor.peek_left(), Some(&1));
        assert!(cursor.move_left());
        assert_eq!(cursor.peek_right(), Some(&1));
    }

    #[test]
    fn remove_on_each_side() {
        let mut list = list_of(&[0, 1, 2]);
        let mut cursor = list.cursor_mut_at_start();
        cursor.move_right();
        assert_eq!(cursor.remove_left(), Some(0));
        assert_eq!(cursor.remove_left(), None);
        assert_eq!(cursor.remove_right(), Some(1));
        assert_eq!(cursor.peek_right(), Some(&2));
        assert_eq!(cursor.remove_right(), Some(2));
        assert_eq!(cursor.remove_right(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn splice_left_puts_cursor_after_spliced() {
        let mut list = list_of(&[0, 3]);
        let mut cursor = list.cursor_mut_at_start();
        cursor.move_right();
        cursor.splice_left(list_of(&[1, 2]));
        assert_eq!(cursor.peek_left(), Some(&2));
        assert_eq!(cursor.peek_right(), Some(&3));
        assert_eq!(list.items, [0, 1, 2, 3]);
    }

    #[test]
    fn splice_right_keeps_cursor_before_spliced() {
        let mut list = list_of(&[0, 3]);
        let mut cursor = list.cursor_mut_at_start();
        cursor.move_right();
        cursor.splice_right(list_of(&[1, 2]));
        assert_eq!(cursor.peek_left(), Some(&0));
        assert_eq!(cursor.peek_right(), Some(&1));
        assert_eq!(list.items, [0, 1, 2, 3]);
    }
}
