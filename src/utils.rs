use std::vec::Drain;

pub trait VecUtils<T> {
    fn retain_or_drain<F>(&mut self, f: F) -> Drain<T>
    where
        F: FnMut(&T) -> bool;
}

impl<T> VecUtils<T> for Vec<T> {
    fn retain_or_drain<F>(&mut self, mut f: F) -> Drain<T>
    where
        F: FnMut(&T) -> bool,
    {
        let len = self.len();
        let mut del = 0;
        {
            let v = &mut **self;

            for i in 0..len {
                if !f(&v[i]) {
                    del += 1;
                } else if del > 0 {
                    v.swap(i - del, i);
                }
            }
        }
        self.drain(len - del..)
    }
}
