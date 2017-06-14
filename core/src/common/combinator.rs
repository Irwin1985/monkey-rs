trait Parser<T, E> {
    fn next(&mut self) -> Option<T>;
    fn preview(&self) -> Option<&T>;
    fn current_pos(&self) -> (i32, i32);
    fn error<S: Into<String>>(&self, message: S) -> E;
}

macro_rules! next {
    ($p:expr) => {
        match $p.next() {
            Some(x) => x,
            None => return Err($p.error("unexpected eof")),
        }
    }
}

macro_rules! predicate {
    ($p:expr, $pred:expr) => {{
        let x = next!($p);
        if $pred(x) {
            x
        } else {
            return Err($p.error(format!("unexpected token {}", x)));
        }
    }}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::vec_deque::VecDeque;

    struct TP {
        input: VecDeque<i32>,
    }

    impl TP {
        fn new(input: &[i32]) -> TP {
            TP { input: VecDeque::from(input.to_vec()) }
        }
    }

    impl Parser<i32, String> for TP {
        fn next(&mut self) -> Option<i32> {
            self.input.pop_front()
        }

        fn preview(&self) -> Option<&i32> {
            self.input.front()
        }

        fn current_pos(&self) -> (i32, i32) {
            (0, 0)
        }

        fn error<S: Into<String>>(&self, message: S) -> String {
            message.into()
        }
    }

    macro_rules! with_res {
        (test $i:ident, $b:block) => {
            #[test]
            fn $i() {
                let result = || {
                    $b;
                    Ok(())
                };
                result().unwrap()
            }
        };
        (panic $e:expr, $i:ident, $b:block) => {
            #[test]
            #[should_panic(expected = $e)]
            fn $i() {
                let result = || {
                    $b;
                    Ok(())
                };
                result().unwrap()
            }
        };
    }

    with_res!(test next_success, {
        let mut p = TP::new(&[1, 2, 3]);
        assert_eq!(next!(p), 1);
        assert_eq!(next!(p), 2);
        assert_eq!(next!(p), 3);
    });

    with_res!(panic "unexpected eof", next_fail_empty, {
        let mut p = TP::new(&[]);
        assert_eq!(next!(p), 1);
    });

    with_res!(test predicate_success, {
        let mut p = TP::new(&[2, 4, 6]);
        assert_eq!(predicate!(p, |x| x % 2 == 0), 2);
        assert_eq!(predicate!(p, |x| x % 2 == 0), 4);
        assert_eq!(predicate!(p, |x| x % 2 == 0), 6);
    });

    with_res!(panic "unexpected eof", predicate_fail_empty, {
        let mut p = TP::new(&[]);
        assert_eq!(predicate!(p, |x| x % 2 == 0), 2);
    });

    with_res!(panic "unexpected token 3", predicate_fail_not_satisfy, {
        let mut p = TP::new(&[3, 5, 7]);
        assert_eq!(predicate!(p, |x| x % 2 == 0), 3);
    });
}