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
