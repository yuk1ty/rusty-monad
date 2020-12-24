use crate::applicative::Applicative;

pub(crate) trait Monad: Applicative {
    fn bind<B, F>(self, f: F) -> Self::Lifted<B>
    where
        F: FnMut(Self::A) -> Self::Lifted<B>;
}
