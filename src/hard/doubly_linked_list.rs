use std::marker::PhantomData;
use std::ptr;

struct Node<T> {
    value: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

pub struct LinkedList<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    len: usize,
}

pub struct Cursor<'a, T> {
    current: *mut Node<T>,
    list: &'a mut LinkedList<T>,
}

pub struct Iter<'a, T> {
    current: *mut Node<T>,
    _marker: PhantomData<&'a T>,
}

#[allow(unsafe_op_in_unsafe_fn)]
impl<T> Node<T> {
    fn new(value: T) -> *mut Self {
        Box::into_raw(Box::new(Self {
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
    
    unsafe fn link(a: *mut Self, b: *mut Self) {
        if !a.is_null() {
            (*a).next = b;
        }
        if !b.is_null() {
            (*b).prev = a;
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_back(&mut self, element: T) {
        self.cursor_back().insert_after(element);
    }

    pub fn push_front(&mut self, element: T) {
        self.cursor_front().insert_before(element);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.cursor_back().take()
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.cursor_front().take()
    }
    
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.head,
            list: self,
        }
    }

    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            current: self.tail,
            list: self,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head;
        while !current.is_null() {
            unsafe {
                let next = (*current).next;
                drop(Box::from_raw(current));
                current = next;
            }
        }
    }
}

impl<T> Cursor<'_, T> {
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.current.is_null() {
            None
        } else {
            unsafe { Some(&mut (*self.current).value) }
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            if self.current.is_null() {
                None
            } else {
                self.current = (*self.current).next;
                self.peek_mut()
            }
        }
    }

    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            if self.current.is_null() {
                None
            } else {
                self.current = (*self.current).prev;
                self.peek_mut()
            }
        }
    }

    pub fn take(&mut self) -> Option<T> {
        if self.current.is_null() {
            return None;
        }

        unsafe {
            let node = self.current;
            let prev = (*node).prev;
            let next = (*node).next;

            // Связываем соседние узлы между собой
            Node::link(prev, next);

            // Обновляем head и tail списка если нужно
            if self.list.head == node {
                self.list.head = next;
            }
            if self.list.tail == node {
                self.list.tail = prev;
            }

            // Перемещаем курсор
            self.current = if !next.is_null() {
                next
            } else {
                prev
            };

            self.list.len -= 1;

            // Извлекаем значение из узла
            Some(Box::from_raw(node).value)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        unsafe {
            let new_node = Node::new(element);

            if self.list.head.is_null() {
                // Вставляем в пустой список
                self.list.head = new_node;
                self.list.tail = new_node;
                self.current = new_node;
            } else if self.current.is_null() {
                panic!("Cursor is out of bounds");
            } else {
                let next = (*self.current).next;
                Node::link(self.current, new_node);
                Node::link(new_node, next);

                if next.is_null() {
                    self.list.tail = new_node;
                }
            }

            self.list.len += 1;
        }
    }

    pub fn insert_before(&mut self, element: T) {
        unsafe {
            let new_node = Node::new(element);

            if self.list.head.is_null() {
                // Вставляем в пустой список
                self.list.head = new_node;
                self.list.tail = new_node;
                self.current = new_node;
            } else if self.current.is_null() {
                panic!("Cursor is out of bounds");
            } else {
                let prev = (*self.current).prev;
                Node::link(prev, new_node);
                Node::link(new_node, self.current);

                if prev.is_null() {
                    self.list.head = new_node;
                }
            }

            self.list.len += 1;
        }
    }

    pub fn seek_forward(&mut self, n: usize) -> bool {
        (0..n).all(|_| self.next().is_some())
    }

    pub fn seek_backward(&mut self, n: usize) -> bool {
        (0..n).all(|_| self.prev().is_some())
    }    
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            unsafe {
                let value = &(*self.current).value;
                self.current = (*self.current).next;
                Some(value)
            }
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut list = Self::new();
        for elem in iter {
            list.push_back(elem);
        }
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_generic() {
        struct Foo;
        LinkedList::<Foo>::new();
    }

    // ———————————————————————————————————————————————————————————
    // Tests for Step 1: push / pop at front and back
    // ———————————————————————————————————————————————————————————
    #[test]
    fn basics_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    // push / pop at back ————————————————————————————————————————
    #[test]
    fn basics_single_element_back() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(5);
        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn basics_push_pop_at_back() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for i in 0..10 {
            list.push_back(i);
            assert_eq!(list.len(), i as usize + 1);
            assert!(!list.is_empty());
        }

        for i in (0..10).rev() {
            assert_eq!(list.len(), i as usize + 1);
            assert!(!list.is_empty());
            assert_eq!(i, list.pop_back().unwrap());
        }

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    // push / pop at front ———————————————————————————————————————
    #[test]
    fn basics_single_element_front() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(5);
        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn basics_push_pop_at_front() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for i in 0..10 {
            list.push_front(i);
            assert_eq!(list.len(), i as usize + 1);
            assert!(!list.is_empty());
        }

        for i in (0..10).rev() {
            assert_eq!(list.len(), i as usize + 1);
            assert!(!list.is_empty());
            assert_eq!(i, list.pop_front().unwrap());
        }

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    // push / pop at mixed sides —————————————————————————————————
    #[test]
    fn basics_push_front_pop_back() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for i in 0..10 {
            list.push_front(i);
            assert_eq!(list.len(), i as usize + 1);
            assert!(!list.is_empty());
        }

        for i in 0..10 {
            assert_eq!(list.len(), 10 - i as usize);
            assert!(!list.is_empty());
            assert_eq!(i, list.pop_back().unwrap());
        }

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn basics_push_back_pop_front() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for i in 0..10 {
            list.push_back(i);
            assert_eq!(list.len(), i as usize + 1);
            assert!(!list.is_empty());
        }

        for i in 0..10 {
            assert_eq!(list.len(), 10 - i as usize);
            assert!(!list.is_empty());
            assert_eq!(i, list.pop_front().unwrap());
        }

        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    // ———————————————————————————————————————————————————————————
    // Tests for Step 2: iteration
    // ———————————————————————————————————————————————————————————
    #[test]
    fn iter() {
        let mut list: LinkedList<i32> = LinkedList::new();
        for num in 0..10 {
            list.push_back(num);
        }

        for (num, &entered_num) in (0..10).zip(list.iter()) {
            assert_eq!(num, entered_num);
        }

    }

    // ———————————————————————————————————————————————————————————
    // Tests for Step 3: full cursor functionality
    // ———————————————————————————————————————————————————————————
    #[test]
    fn cursor_insert_before_on_empty_list() {
        // insert_after on empty list is already tested via push_back()
        let mut list = LinkedList::new();
        list.cursor_front().insert_before(0);
        assert_eq!(Some(0), list.pop_front());
    }

    #[test]
    fn cursor_insert_after_in_middle() {
        let mut list = (0..10).collect::<LinkedList<_>>();
        {
            let mut cursor = list.cursor_front();
            let didnt_run_into_end = cursor.seek_forward(4);
            assert!(didnt_run_into_end);
            for n in (0..10).rev() {
                cursor.insert_after(n);
            }

        }

        assert_eq!(list.len(), 20);
        let expected = (0..5).chain(0..10).chain(5..10);
        assert!(expected.eq(list.iter().cloned()));
    }

    #[test]
    fn cursor_insert_before_in_middle() {
        let mut list = (0..10).collect::<LinkedList<_>>();
        {
            let mut cursor = list.cursor_back();
            let didnt_run_into_end = cursor.seek_backward(4);
            assert!(didnt_run_into_end);
            for n in 0..10 {
                cursor.insert_before(n);
            }

        }

        assert_eq!(list.len(), 20);
        let expected = (0..5).chain(0..10).chain(5..10);
        assert!(expected.eq(list.iter().cloned()));
    }

    // "iterates" via next() and checks that it visits the right elements
    #[test]
    fn cursor_next_and_peek() {
        let mut list = (0..10).collect::<LinkedList<_>>();
        let mut cursor = list.cursor_front();
        assert_eq!(cursor.peek_mut(), Some(&mut 0));
        for n in 1..10 {
            let next = cursor.next().cloned();
            assert_eq!(next, Some(n));
            assert_eq!(next, cursor.peek_mut().cloned());
        }

    }

    // "iterates" via prev() and checks that it visits the right elements
    #[test]
    fn cursor_prev_and_peek() {
        let mut list = (0..10).collect::<LinkedList<_>>();
        let mut cursor = list.cursor_back();
        assert_eq!(cursor.peek_mut(), Some(&mut 9));
        for n in (0..9).rev() {
            let prev = cursor.prev().cloned();
            assert_eq!(prev, Some(n));
            assert_eq!(prev, cursor.peek_mut().cloned());
        }

    }

    // removes all elements starting from the middle
    #[test]
    fn cursor_take() {
        let mut list = (0..10).collect::<LinkedList<_>>();
        let mut cursor = list.cursor_front();
        cursor.seek_forward(5);
        for expected in (5..10).chain((0..5).rev()) {
            assert_eq!(cursor.take(), Some(expected));
        }

    }

    // ———————————————————————————————————————————————————————————
    // Tests for Step 4: clean-up via `Drop`
    // ———————————————————————————————————————————————————————————
    // The leak tests that are also for this step are separated into
    // their own files so that nothing else interferes with the allocator
    // whilst they run
    // checks number of drops
    // may pass for incorrect programs if double frees happen
    // exactly as often as destructor leaks
    #[test]
    fn drop_no_double_frees() {
        use std::cell::Cell;
        struct DropCounter<'a>(&'a Cell<usize>);
        impl Drop for DropCounter<'_> {
            fn drop(&mut self) {
                let num = self.0.get();
                self.0.set(num + 1);
            }

        }

        const N: usize = 15;
        let counter = Cell::new(0);
        let list = std::iter::repeat_with(|| DropCounter(&counter))
            .take(N)
            .collect::<LinkedList<_>>();
        assert_eq!(list.len(), N);
        drop(list);
        assert_eq!(counter.get(), N);
    }

    #[test]
    fn drop_large_list() {
        drop((0..2_000_000).collect::<LinkedList<i32>>());
    }
}