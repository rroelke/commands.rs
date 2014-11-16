#![feature(phase)]
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

#[test]
fn test_commands() {
    commands! {
        with commands : () = {
            ("hello") ~ (say : String) => println!("hello"),
            ("sup", "yo") ~ () => println!("sup"),
            ("eat", "e") ~ (a : uint) => println!("eating {}", a),
            ("point") ~ (x : uint, y : f64) => println!("x + y = {}", x as f64 + y)
        },
        do : {
            assert!(is_usage_error("hello", commands("hello", vec![].as_slice())));
            assert!(commands("sup", vec![].as_slice()).is_ok());
            assert!(commands("yo", vec![].as_slice()).is_ok());
            assert!(commands("help", vec![].as_slice()).is_ok());
            assert!(commands("eat", vec!["6"].as_slice()).is_ok());
            assert!(commands("e", vec!["hi"].as_slice()).is_err());
            assert!(is_usage_error("e", commands("e", vec!["6", "8"].as_slice())));
            assert!(is_usage_error("eat", commands("eat", vec!["6", "8"].as_slice())));

            assert!(is_usage_error("point", commands("point", vec!["hi"].as_slice())));
            assert!(is_usage_error("point", commands("point", vec!["6"].as_slice())));
            assert!(is_usage_error("point", commands("point", vec!["hi", "6", "8"].as_slice())));
        }
    }
}
