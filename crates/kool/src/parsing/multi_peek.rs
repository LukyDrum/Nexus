pub struct MultiPeekable<Iter, Item>
where
    Iter: Iterator<Item = Item>,
{
    inner: Iter,
    peeked: Vec<Item>,
}

impl<Iter, Item> MultiPeekable<Iter, Item>
where
    Iter: Iterator<Item = Item>,
{
    pub fn new(inner: Iter) -> Self {
        Self {
            inner,
            peeked: Vec::new(),
        }
    }

    pub fn peek(&mut self) -> Option<&Item> {
        if self.peeked.is_empty() {
            let next = self.inner.next()?;
            self.peeked.push(next);
        }

        self.peeked.last()
    }

    pub fn multi_peek(&mut self, count: usize) -> &[Item] {
        let to_take = count - self.peeked.len();
        for _ in 0..to_take {
            let Some(next) = self.inner.next() else {
                break;
            };

            self.peeked.push(next);
        }

        let count = self.peeked.len().min(count);

        &self.peeked[0..count]
    }
}

impl<Iter, Item> Iterator for MultiPeekable<Iter, Item>
where
    Iter: Iterator<Item = Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.is_empty() {
            self.inner.next()
        } else {
            Some(self.peeked.remove(0))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parsing::multi_peek::MultiPeekable;

    fn setup() -> MultiPeekable<core::array::IntoIter<u32, 5>, u32> {
        MultiPeekable::new([10, 20, 30, 40, 50].into_iter())
    }

    #[test]
    fn peek_and_next() {
        let mut nums = setup();

        assert_eq!(nums.peek(), Some(&10));
        assert_eq!(nums.next(), Some(10));
    }

    #[test]
    fn multi_peek() {
        let mut nums = setup();

        assert_eq!(nums.multi_peek(3), &[10, 20, 30]);
        assert_eq!(nums.multi_peek(6), &[10, 20, 30, 40, 50]);
    }

    #[test]
    fn multi_peek_and_nexts() {
        let mut nums = setup();

        assert_eq!(nums.multi_peek(2), &[10, 20]);
        assert_eq!(nums.next(), Some(10));
        assert_eq!(nums.next(), Some(20));
        assert_eq!(nums.next(), Some(30));
        assert_eq!(nums.peek(), Some(&40));
        assert_eq!(nums.peek(), Some(&40));
        assert_eq!(nums.next(), Some(40));
        assert_eq!(nums.next(), Some(50));
        assert_eq!(nums.peek(), None);
        assert_eq!(nums.multi_peek(2), &[]);
    }
}
