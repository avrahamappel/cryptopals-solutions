pub trait Sorted {
    fn sorted(self) -> Self;
}

impl<T> Sorted for Vec<T>
where
    T: Ord,
{
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}
