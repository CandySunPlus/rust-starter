pub struct WindowCount<I> {
    iter: I,
    count: usize,
}

impl<I: Iterator> Iterator for WindowCount<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let data = (0..self.count)
            .filter_map(|_| self.iter.next())
            .collect::<Vec<_>>();

        if data.is_empty() {
            None
        } else {
            Some(data)
        }
    }
}

pub trait IteratorExt: Iterator {
    fn window_count(self, count: usize) -> WindowCount<Self>
    where
        Self: Sized,
    {
        WindowCount { iter: self, count }
    }
}

impl<T: ?Sized> IteratorExt for T where T: Iterator {}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_iterator_ext() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let res = data.iter().window_count(2).collect::<Vec<Vec<_>>>();
        assert_eq!(res[0][0], &1);
        assert_eq!(res[2][0], &5);
    }
}
