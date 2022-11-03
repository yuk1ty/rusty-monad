pub mod applicative;
pub mod data;
pub mod functor;
pub mod monad;

macro_rules! mdo {
    ($i:ident <- $e:expr; $($t:tt)*) => {
        $e.bind(move |$i| mdo!($($t)*))
    };
    ($e:expr; $($t:tt)*) => {
        $e.bind(move |()| mdo!($($t)*))
    };
    (ret $e:expr) => {
        $e
    };
}

mod test_do {
    use crate::data::option::*;
    use crate::{applicative::Applicative, functor::Functor, functor::Pointed, monad::Monad};

    #[test]
    fn test_do_notation() {
        let actual = mdo! {
            x <- Option::pure(1);
            y <- Option::pure(2);
            ret Option::pure(x + y)
        };
        assert_eq!(actual, Some(3));
    }
}
