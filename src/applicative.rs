use crate::functor::Pointed;

pub(crate) trait Applicative: Pointed {
    fn apply<F, B, C>(self, b: Self::Lifted<B>, f: F) -> Self::Lifted<C>
    where
        F: FnMut(Self::A, B) -> C;
}
