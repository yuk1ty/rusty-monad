use crate::{applicative::Applicative, functor::Functor, functor::Pointed, monad::Monad};

#[derive(Debug)]
pub(crate) struct Id<M>(pub M);

impl<A> Monad for Id<A> {
    fn bind<B, F>(self, mut f: F) -> Id<B>
    where
        F: FnMut(A) -> Id<B>,
    {
        f(self.0)
    }
}

impl<A> Pointed for Id<A> {
    fn pure(t: A) -> Id<A> {
        Id(t)
    }
}

impl<A> Applicative for Id<A> {
    fn apply<F, B, C>(self, b: Id<B>, mut f: F) -> Id<C>
    where
        F: FnMut(A, B) -> C,
    {
        Id(f(self.0, b.0))
    }
}

impl<A> Functor for Id<A> {
    type A = A;
    type Lifted<B> = Id<B>;

    fn map<F, B>(self, mut f: F) -> Id<B>
    where
        F: FnMut(A) -> B,
    {
        Id(f(self.0))
    }
}
