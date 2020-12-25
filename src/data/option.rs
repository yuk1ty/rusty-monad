use crate::{applicative::Applicative, functor::Functor, functor::Pointed, monad::Monad};

impl<A> Monad for Option<A> {
    fn bind<B, F>(self, mut f: F) -> Option<B>
    where
        F: FnMut(A) -> Option<B>,
    {
        self.and_then(f)
    }
}

impl<A> Pointed for Option<A> {
    fn pure(t: A) -> Option<A> {
        Some(t)
    }
}

impl<A> Applicative for Option<A> {
    fn apply<F, B, C>(self, b: Option<B>, mut f: F) -> Option<C>
    where
        F: FnMut(A, B) -> C,
    {
        let a = self?;
        let b = b?;
        Some(f(a, b))
    }
}

impl<A> Functor for Option<A> {
    type A = A;
    type Lifted<B> = Option<B>;

    fn map<F, B>(self, mut f: F) -> Option<B>
    where
        F: FnMut(A) -> B,
    {
        self.map(f)
    }
}

#[cfg(test)]
mod option_test {
    use super::*;

    #[test]
    fn test_option_monad() {
        struct Employee {
            name: String,
            department: String,
        }

        fn lookup_by_name(name: &str) -> Option<Employee> {
            Some(Employee {
                name: name.to_string(),
                department: "some_department".to_string(),
            })
        }

        fn no_one_look_up(_name: &str) -> Option<Employee> {
            None
        }

        let actual = lookup_by_name("joe").map(|employee| employee.department);
        assert_eq!(actual, Some("some_department".to_string()));

        let actual = no_one_look_up("joe").map(|employee| employee.department);
        assert_eq!(actual, None);
    }
}
