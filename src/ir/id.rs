use std::sync::atomic::AtomicUsize;

pub struct IdManager<T>
where
    T: From<usize> + Copy + std::fmt::Display,
{
    next: AtomicUsize,
    phantom: std::marker::PhantomData<T>,
}

impl<T> IdManager<T>
where
    T: From<usize> + Copy + std::fmt::Display,
{
    pub fn new() -> Self {
        Self {
            next: AtomicUsize::new(0),
            phantom: std::marker::PhantomData,
        }
    }

    pub fn next(&self) -> T {
        let id = self.next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        T::from(id)
    }
}
