use super::LinkedList;
use super::cursor::CursorMut;

pub struct SelectionMut<'a, T> {
    list: &'a mut LinkedList<T>,
    start: usize,
    end: usize,
}

impl<'a, T> SelectionMut<'a, T> {
    pub(super) fn new(list: &'a mut LinkedList<T>, start: usize, end: usize) -> Self {
        Self { list, start, end }
    }

    pub fn into_cursor_at_start(self) -> CursorMut<'a, T> {
        CursorMut::new(self.list, self.start)
    }

    pub fn into_cursor_at_end(self) -> CursorMut<'a, T> {
        CursorMut::new(self.list, self.end)
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    pub fn move_start_prev(&mut self) -> bool {
        if self.start == 0 {
            return false;
        }
        self.start -= 1;
        true
    }

    pub fn move_start_next(&mut self) -> bool {
        if self.start == self.end {
            return false;
        }
        self.start += 1;
        true
    }

    pub fn move_end_prev(&mut self) -> bool {
        if self.end == self.start {
            return false;
        }
        self.end -= 1;
        true
    }

    pub fn move_end_next(&mut self) -> bool {
        if self.end == self.list.items.len() {
            return false;
        }
        self.end += 1;
        true
    }

    pub fn first(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        self.list.items.get(self.start)
    }

    pub fn last(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        self.list.items.get(self.end - 1)
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }
        self.list.items.get_mut(self.start)
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None;
        }
        self.list.items.get_mut(self.end - 1)
    }

    pub fn peek_before_start(&self) -> Option<&T> {
        let i = self.start.checked_sub(1)?;
        self.list.items.get(i)
    }

    pub fn peek_after_end(&self) -> Option<&T> {
        self.list.items.get(self.end)
    }

    pub fn peek_before_start_mut(&mut self) -> Option<&mut T> {
        let i = self.start.checked_sub(1)?;
        self.list.items.get_mut(i)
    }

    pub fn peek_after_end_mut(&mut self) -> Option<&mut T> {
        self.list.items.get_mut(self.end)
    }

    pub fn insert_before_start(&mut self, value: T) {
        self.list.items.insert(self.start, value);
        self.start += 1;
        self.end += 1;
    }

    pub fn insert_at_start(&mut self, value: T) {
        self.list.items.insert(self.start, value);
        self.end += 1;
    }

    pub fn insert_at_end(&mut self, value: T) {
        self.list.items.insert(self.end, value);
        self.end += 1;
    }

    pub fn insert_after_end(&mut self, value: T) {
        self.list.items.insert(self.end, value);
    }

    pub fn take(&mut self) -> LinkedList<T> {
        let items = self.list.items.drain(self.start..self.end).collect();
        self.end = self.start;
        LinkedList { items }
    }

    pub fn replace(&mut self, other: LinkedList<T>) -> LinkedList<T> {
        let len = other.items.len();
        let items = self
            .list
            .items
            .splice(self.start..self.end, other.items)
            .collect();
        self.end = self.start + len;
        LinkedList { items }
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        self.list.items[self.start..self.end].iter()
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut T> + ExactSizeIterator {
        self.list.items[self.start..self.end].iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::list::tests::list_of;

    #[test]
    fn cursor_round_trip() {
        let mut list = list_of(&[0, 1]);
        let mut cursor = list.cursor_mut_at_start();
        cursor.move_next();
        let selection = cursor.into_selection();
        assert!(selection.is_empty());
        assert_eq!(selection.peek_before_start(), Some(&0));
        assert_eq!(selection.peek_after_end(), Some(&1));
        let cursor = selection.into_cursor_at_end();
        assert_eq!(cursor.peek_before(), Some(&0));
        assert_eq!(cursor.peek_after(), Some(&1));
    }

    #[test]
    fn into_cursor_at_each_end() {
        let mut list = list_of(&[0, 1, 2]);
        let mut selection = list.cursor_mut_at_start().into_selection();
        selection.move_end_next();
        selection.move_end_next();
        let cursor = selection.into_cursor_at_end();
        assert_eq!(cursor.peek_before(), Some(&1));
        let mut selection = cursor.into_selection();
        selection.move_start_prev();
        let cursor = selection.into_cursor_at_start();
        assert_eq!(cursor.peek_before(), Some(&0));
        assert_eq!(cursor.peek_after(), Some(&1));
    }

    #[test]
    fn boundaries_stop_at_each_other_and_ends() {
        let mut list = list_of(&[0, 1]);
        let mut selection = list.cursor_mut_at_start().into_selection();
        assert!(!selection.move_start_prev());
        assert!(!selection.move_start_next());
        assert!(!selection.move_end_prev());
        assert!(selection.move_end_next());
        assert!(selection.move_end_next());
        assert!(!selection.move_end_next());
        assert_eq!(selection.len(), 2);
        assert!(selection.move_start_next());
        assert!(selection.move_start_next());
        assert!(!selection.move_start_next());
        assert!(selection.is_empty());
    }

    #[test]
    fn first_and_last_inside_selection() {
        let mut list = list_of(&[0, 1, 2, 3]);
        let mut selection = list.cursor_mut_at_start().into_selection();
        assert_eq!(selection.first(), None);
        assert_eq!(selection.last(), None);
        selection.move_end_next();
        selection.move_end_next();
        selection.move_end_next();
        selection.move_start_next();
        assert_eq!(selection.first(), Some(&1));
        assert_eq!(selection.last(), Some(&2));
        *selection.first_mut().unwrap() += 10;
        *selection.last_mut().unwrap() += 20;
        assert_eq!(list.items, [0, 11, 22, 3]);
    }

    #[test]
    fn peek_mut_outside_selection() {
        let mut list = list_of(&[0, 1, 2]);
        let mut selection = list.cursor_mut_at_start().into_selection();
        selection.move_end_next();
        selection.move_end_next();
        selection.move_start_next();
        *selection.peek_before_start_mut().unwrap() += 10;
        *selection.peek_after_end_mut().unwrap() += 20;
        assert_eq!(list.items, [10, 1, 22]);
    }

    #[test]
    fn insert_outside_selection() {
        let mut list = list_of(&[1]);
        let mut selection = list.cursor_mut_at_start().into_selection();
        selection.move_end_next();
        selection.insert_before_start(0);
        selection.insert_after_end(2);
        assert_eq!(selection.iter().copied().collect::<Vec<_>>(), [1]);
        assert_eq!(list.items, [0, 1, 2]);
    }

    #[test]
    fn insert_inside_selection() {
        let mut list = list_of(&[0, 2, 4]);
        let mut selection = list.cursor_mut_at_start().into_selection();
        selection.move_end_next();
        selection.move_end_next();
        selection.move_start_next();
        selection.insert_at_start(1);
        selection.insert_at_end(3);
        assert_eq!(selection.iter().copied().collect::<Vec<_>>(), [1, 2, 3]);
        assert_eq!(selection.peek_before_start(), Some(&0));
        assert_eq!(selection.peek_after_end(), Some(&4));
        assert_eq!(list.items, [0, 1, 2, 3, 4]);
    }

    #[test]
    fn insert_inside_empty_selection() {
        let mut list = list_of(&[0, 3]);
        let mut selection = list.cursor_mut_at_start().into_selection();
        selection.move_end_next();
        selection.move_start_next();
        selection.insert_at_start(2);
        selection.insert_at_start(1);
        assert_eq!(selection.len(), 2);
        assert_eq!(selection.first(), Some(&1));
        assert_eq!(selection.last(), Some(&2));
        assert_eq!(list.items, [0, 1, 2, 3]);
    }

    #[test]
    fn take_empties_selection() {
        let mut list = list_of(&[0, 1, 2, 3]);
        let mut selection = list.cursor_mut_at_start().into_selection();
        selection.move_end_next();
        selection.move_end_next();
        selection.move_end_next();
        selection.move_start_next();
        let taken = selection.take();
        assert!(selection.is_empty());
        assert_eq!(selection.peek_before_start(), Some(&0));
        assert_eq!(selection.peek_after_end(), Some(&3));
        assert_eq!(taken.items, [1, 2]);
        assert_eq!(list.items, [0, 3]);
    }

    #[test]
    fn replace_selects_new_contents() {
        let mut list = list_of(&[0, 1, 2]);
        let mut selection = list.cursor_mut_at_start().into_selection();
        selection.move_end_next();
        selection.move_end_next();
        selection.move_start_next();
        let old = selection.replace(list_of(&[10, 11, 12]));
        assert_eq!(selection.iter().copied().collect::<Vec<_>>(), [10, 11, 12]);
        assert_eq!(selection.peek_after_end(), Some(&2));
        assert_eq!(old.items, [1]);
        assert_eq!(list.items, [0, 10, 11, 12, 2]);
    }

    #[test]
    fn iter_mut_within_selection() {
        let mut list = list_of(&[0, 1, 2]);
        let mut selection = list.cursor_mut_at_start().into_selection();
        selection.move_end_next();
        selection.move_end_next();
        selection.move_start_next();
        for value in selection.iter_mut() {
            *value *= 10;
        }
        assert_eq!(selection.iter().rev().copied().collect::<Vec<_>>(), [10]);
        assert_eq!(list.items, [0, 10, 2]);
    }
}
