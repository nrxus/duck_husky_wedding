use std::vec::Drain;
use glm;

pub trait Center<T: glm::Primitive> {
    fn center(&self) -> glm::Vector2<T>;
}

impl Center<f64> for glm::DVec4 {
    fn center(&self) -> glm::DVec2 {
        glm::dvec2(self.x + self.z / 2., self.y + self.w / 2.)
    }
}

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

pub trait Try<E> {
    fn try(self) -> Result<(), E>;
}

impl<E, I: Iterator<Item = Result<(), E>>> Try<E> for I {
    fn try(self) -> Result<(), E> {
        for r in self {
            r?
        }
        Ok(())
    }
}
