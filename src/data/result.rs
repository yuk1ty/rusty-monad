use crate::{applicative::Applicative, functor::Functor, functor::Pointed, monad::Monad};

impl<A, E> Monad for Result<A, E> {
    fn bind<B, F>(self, f: F) -> Result<B, E>
    where
        F: FnMut(A) -> Result<B, E>,
    {
        self.and_then(f)
    }
}

impl<A, E> Pointed for Result<A, E> {
    fn pure(t: A) -> Result<A, E> {
        Ok(t)
    }
}

impl<A, E> Applicative for Result<A, E> {
    fn apply<F, B, C>(self, b: Result<B, E>, mut f: F) -> Result<C, E>
    where
        F: FnMut(A, B) -> C,
    {
        let a = self?;
        let b = b?;
        Ok(f(a, b))
    }
}

impl<A, E> Functor for Result<A, E> {
    type A = A;
    type Lifted<B> = Result<B, E>;

    fn map<F, B>(self, mut f: F) -> Result<B, E>
    where
        F: FnMut(A) -> B,
    {
        self.map(f)
    }
}
