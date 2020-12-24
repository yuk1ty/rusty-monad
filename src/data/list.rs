use crate::{applicative::Applicative, functor::Functor, functor::Pointed, monad::Monad};

// pub enum List<T> {
//     Cons { head: T, tail: Box<List<T>> },
//     Nil,
// }

// impl<A> Pointed for List<A> {
//     fn pure(t: A) -> List<A> {
//         List::Cons {
//             head: t,
//             tail: Box::new(List::Nil),
//         }
//     }
// }

// impl<A> Functor for List<A> {
//     type A = A;
//     type Lifted<B> = Functor<B>;

//     fn map<A, B>(self, f: F) -> List<B>
//     where
//         F: Fn(A) -> B,
//     {
//         todo!()
//         // match self {
//         //     List::Cons(head, tail)
//         // }
//     }
// }

// #[test]
// fn test_create_cons_cell() {
//     use crate::data::list::List::*;
//     let cell: List<i32> = Cons {
//         head: 1,
//         tail: Box::new(Cons {
//             head: 2,
//             tail: Box::new(Cons {
//                 head: 3,
//                 tail: Box::new(Nil),
//             }),
//         }),
//     };
// }
