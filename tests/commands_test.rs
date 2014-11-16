#![feature(phase)]
#[phase(plugin)]

extern crate commands;

fn is_usage_error(result : Result<(), String>) -> bool {
    if result.is_ok() {
        false
    }
    else {
        /* TODO: match first part of string */
        let usage_str : &str = "error: usage:";
        result.err().unwrap().slice_to(usage_str.len()) == usage_str
    }
}

#[test]
fn test_commands() {
    commands! {
        with commands = {
            ("hello") ~ (say : String) => println!("hello"),
            ("sup", "yo") ~ () => println!("sup"),
            ("eat", "e") ~ (a : uint) => println!("eating {}", a)
        },
        do : {
            assert!(is_usage_error(commands("hello", Vec::new().as_slice())));
            assert!(commands("sup", Vec::new().as_slice()).is_ok());
            assert!(commands("yo", Vec::new().as_slice()).is_ok());
            assert!(commands("help", Vec::new().as_slice()).is_ok());
            assert!(commands("eat", vec!["6"].as_slice()).is_ok());
            assert!(commands("e", vec!["hi"].as_slice()).is_err());
            assert!(is_usage_error(commands("e", vec!["6", "8"].as_slice())));
        }
    }
}
