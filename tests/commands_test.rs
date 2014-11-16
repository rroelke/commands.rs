#![feature(phase)]
#[phase(plugin)]

extern crate commands;

#[test]
fn test_commands() {
    commands! {
        with commands = {
            ("hello") ~ (say : String) => println!("hello"),
            ("sup", "yo") ~ () => println!("sup"),
            ("eat", "e") ~ (a : uint) => println!("eating {}", a)
        },
        do : {
            commands("hello", Vec::new().as_slice());
            commands("sup", Vec::new().as_slice());
            commands("yo", Vec::new().as_slice());
            commands("help", Vec::new().as_slice());
            commands("eat", vec!["6"].as_slice());
            commands("e", vec!["hi"].as_slice());
        }
    }
}
