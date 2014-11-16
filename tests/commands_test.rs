#![feature(phase)]
#![feature(macro_rules)]
#[phase(plugin)]

extern crate commands;

fn is_usage_error<T>(cmd : &str, result : Result<Option<T>, String>) -> bool {
    if result.is_ok() {
        false
    }
    else {
        let mut usage_str : String = String::from_str("error: usage: ");
        usage_str.push_str(cmd);
        result.err().unwrap().slice_to(usage_str.len()) == usage_str.as_slice()
    }
}

/// returns whether or not the request command
/// invoked the "help" error message
fn is_help<T>(cmd : &str, result : Result<Option<T>, String>) -> bool {
    if result.is_ok() {
        false
    }
    else {
        let msg : String = result.err().unwrap();
        let mut help_str : String = String::from_str("error: unknown command: ");
        help_str.push_str(cmd);

        msg.slice_to(help_str.len()) == help_str.as_slice()
    }
}

macro_rules! is_type_error(
    ($cmd:expr, $val:ty, $result:expr) => {
        if $result.is_ok() {
            false
        }
        else {
            let msg : String = $result.err().unwrap();
            let type_str : String = format!("error: {}: not a {}: ",
                                            $cmd, stringify!($val));
            msg.slice_to(type_str.len()) == type_str.as_slice()
        }
    }
)

#[test]
/* check that the proper command is invoked with the proper name,
 * and generates the correct result
 */
fn test_commands() {
    commands! {
        with commands : () = {
            ("hello") ~ (say : String) => println!("hello"),
            ("sup", "yo") ~ () => println!("sup"),
            ("eat", "e") ~ (a : uint) => println!("eating {}", a),
            ("point") ~ (x : uint, y : f64) => println!("x + y = {}", x as f64 + y)
        },
        do : {
            assert!(commands("sup", vec![].as_slice()).is_ok());
            assert!(commands("yo", vec![].as_slice()).is_ok());
            assert!(commands("help", vec![].as_slice()).is_ok());
            assert!(commands("eat", vec!["6"].as_slice()).is_ok());
        }
    }

    commands! {
        with c : uint = {
            ("add", "a") ~ (x : uint, y : uint) => x + y,
            ("sub", "s") ~ (x : uint, y : uint) => x - y
        },
        do : {
            assert_eq!(20u, c("add", vec!["12", "8"].as_slice()).unwrap().unwrap());
            assert_eq!(4u, c("sub", vec!["12", "8"].as_slice()).unwrap().unwrap());
            assert_eq!(20u, c("a", vec!["12", "8"].as_slice()).unwrap().unwrap());
            assert_eq!(4u, c("s", vec!["12", "8"].as_slice()).unwrap().unwrap());
        }
    }
}

#[test]
fn test_errors() {
    commands! {
        with c : () = {
            ("zero") ~ () => println!("zero"),
            ("one") ~ (a1 : uint) => println!("one: {}", a1),
            ("two") ~ (a1 : uint, a2 : f64) => println!("two: {}, {}", a1, a2)
        },
        do : {
            /* any number of arguments is an error for the zero command */
            assert!(is_usage_error("zero", c("zero", vec!["a"].as_slice())));
            assert!(is_usage_error("zero", c("zero", vec!["a", "b"].as_slice())));

            /* must have exactly the right number of arguments */
            assert!(is_usage_error("one", c("one", vec![].as_slice())));
            assert!(is_usage_error("one", c("one", vec!["a", "b"].as_slice())));
            assert!(is_usage_error("one", c("one", vec!["a", "b", "c"].as_slice())));
            assert!(is_usage_error("two", c("two", vec![].as_slice())));
            assert!(is_usage_error("two", c("two", vec!["a"].as_slice())));
            assert!(is_usage_error("two", c("two", vec!["a", "b", "c"].as_slice())));

            /* now check that type errors return proper messages */
            assert!(is_type_error!("one", uint, c("one", vec!["a"].as_slice())));
            assert!(is_type_error!("two", f64, c("two", vec!["7", "a"].as_slice())));
            assert!(is_type_error!("two", uint, c("two", vec!["a", "7"].as_slice())));
        }
    }
    /* make sure that commands from previous scopes do not leak into this one */
    commands! {
        with c : () = {
        },
        do : {
            assert!(is_help("zero", c("zero", &[])));
            assert!(is_help("one", c("one", &[])));
            assert!(is_help("two", c("two", &[])));
        }
    }
}

#[test]
fn test_shadowing() {
    /* make sure commands are shadowed */
    commands! {
        with c : () = {
            ("hello") ~ () => println!("hello")
        },
        do : {
            commands! {
                with d : () = {
                    ("greet") ~ () => println!("greet")
                },
                do : {
                    assert!(is_help("hello", d("hello", &[])));
                    assert!(c("hello", &[]).is_ok());
                    assert!(d("greet", &[]).is_ok());
                }
            }
            assert!(is_help("greet", c("greet", &[])));

            commands! {
                with e : () = {
                    ("hello") ~ (greeting : String) => println!("hello {}", greeting)
                },
                do : {
                    assert!(is_usage_error("hello", e("hello", &[])));
                    assert!(c("hello", &[]).is_ok());
                }
            }
            assert!(c("hello", &[]).is_ok());

            commands! {
                with f : uint = {
                    ("hello") ~ () => 5u
                },
                do : {
                    assert_eq!(5, f("hello", &[]).unwrap().unwrap());
                    assert!(c("hello", &[]).is_ok());
                }
            }
        }
    }
}

#[test]
fn test_argv() {
    commands! {
        with c : Vec<String> = {
            ("args") ~ ()(argv : ...) => argv.iter().map(|s| String::from_str(*s)).collect()
        },
        do : {
            let mut v : Vec<String> = c("args", vec![].as_slice()).unwrap().unwrap();
            assert!(v.is_empty());

            v = c("args", vec!["a"].as_slice()).unwrap().unwrap();
            assert_eq!(v, vec![String::from_str("a")]);

            v = c("args", vec!["a", "b"].as_slice()).unwrap().unwrap();
            assert_eq!(v, vec![String::from_str("a"), String::from_str("b")]);
        }
    }

    commands! {
        with c : uint = {
            ("add") ~ (b : uint)(argv : ...) => {
                let mut sum : uint = b;
                for arg in argv.iter() {
                    match from_str::<uint>(*arg) {
                        None => return Err(format!("not a uint: {}", arg)),
                        Some(u) => sum += u
                    }
                }

                sum
            }
        },
        do : {
            assert!(is_usage_error("add", c("add", vec![].as_slice())));
            assert_eq!(12u, c("add", vec!["12"].as_slice()).unwrap().unwrap());
            assert_eq!(12u, c("add", vec!["11", "1"].as_slice()).unwrap().unwrap());
            assert_eq!(12u, c("add", vec!["9", "2", "1"].as_slice()).unwrap().unwrap());


            assert!(is_type_error!("add", uint, c("add", vec!["a", "7"].as_slice())));
            assert_eq!(c("add", vec!["7", "a"].as_slice()).err().unwrap(),
                        String::from_str("not a uint: a"));
            assert_eq!(c("add", vec!["7", "8", "b"].as_slice()).err().unwrap(),
                        String::from_str("not a uint: b"));
        }
    }
}
