pub fn cycle_n(v: Vec<i32>, n: usize) -> Vec<i32> {
    std::iter::repeat(v.into_iter()).take(n).flatten().collect()
}

pub fn cycle_n_manual(v: Vec<i32>, n: usize) -> Vec<i32> {
    let it = v.into_iter();
    let mut res = Vec::new();

    for _ in 0..n {
        for x in it.clone() {
            res.push(x);
        }
    }
    res
}

pub fn cycle_n_trait(v: Vec<i32>, n: usize) -> Vec<i32> {
    v.into_iter().cycle_n(n).collect()
}

pub trait Itermisc: std::iter::Iterator {
    fn cycle_n(self, n: usize) -> CycleN<Self>
    where
        Self: Clone,
    {
        CycleN::new(self, n)
    }
}

impl<T: ?Sized> Itermisc for T where T: std::iter::Iterator {}

pub struct CycleN<I> {
    orig: I,
    iter: I,
    tick: usize,
}
impl<I: Clone> CycleN<I> {
    pub fn new(iter: I, n: usize) -> CycleN<I> {
        CycleN {
            orig: iter.clone(),
            iter,
            tick: n,
        }
    }
}

impl<I> Iterator for CycleN<I>
where
    I: Clone + Iterator,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        match self.iter.next() {
            None if self.tick > 0 => {
                self.tick -= 1;
                self.iter = self.orig.clone();
                self.iter.next()
            }
            y => y,
        }
    }
}
