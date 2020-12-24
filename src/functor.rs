pub(crate) trait Functor {
    type A;
    type Lifted<B>: Functor;

    fn map<F, B>(self, f: F) -> Self::Lifted<B>
    where
        F: FnMut(Self::A) -> B;
}

pub(crate) trait Pointed: Functor {
    fn pure(t: Self::A) -> Self::Lifted<Self::A>;
}
