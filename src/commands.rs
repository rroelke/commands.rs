#![feature(macro_rules)]

#[macro_export]
macro_rules! commands {
    (with $commands:ident : $ret:ty =
    {$($desc:expr : ($($cmd:expr),+)($($name:ident : $arg:ty),*)$(($argv:ident : ...))* => $code:expr),*},
     do : $action:expr) => ({
        use std::collections::hash_map::HashMap;

        /* help map could be empty if there are no commands */
        let mut __usage : HashMap<String, String> = HashMap::new();
        let mut __desc : HashMap<String, String> = HashMap::new();
        $({ /* put command in help message map */
            let mut __argc : uint = 0;
            let mut __cmd_usage : String = format!("");
            $(
                __argc += 1;
                __cmd_usage.push_str("<");
                __cmd_usage.push_str(stringify!($arg));
                __cmd_usage.push_str("> ");
            )*
            if __argc > 0 {
                assert_eq!(__cmd_usage.pop().unwrap(), ' ');
            }
            /* add ... if there are varargs */
            let mut _v : uint = 0;
            $(assert!(Some(stringify!($argv)).is_some()); /* no-op, to iterate over argvs */
              _v += 1)*;
            if _v > 0 {
                __cmd_usage.push_str(" [...]*");
            }
            /* then put usage string into help map with all command names */
            $(
                __usage.insert(String::from_str($cmd), __cmd_usage.clone());
                __desc.insert(String::from_str($cmd), String::from_str($desc));
             )+
        })*

        /* now build the mapping from command name to functions */
        {
            let mut __index_map : HashMap<String, uint> = HashMap::new();
            let mut __commands : HashMap<uint, |&str, &[&str]| -> Result<Option<$ret>, String>> = HashMap::new();

            let mut __command_num : uint = 0;
            $({ /* for each command, give it a map entry */
                let command = |_cmd : &str, argv : &[&str]| -> Result<Option<$ret>, String> {
                    /* first count the expected number of arguments */
                    let mut __i : uint = 0;
                    $(let mut $name : $arg; __i += 1;)*
                    if argv.len() < __i {
                        return Err(format!("error: usage: {} {}",
                                           _cmd, __usage[String::from_str(_cmd)]))
                    }

                    /* then copy remaining arguments to ... identifiers, if any */
                    let mut __k : uint = 0;
                    $(let $argv : &[&str] = argv.slice_from(__i); __k += 1;)*
                    if __k == 0 && argv.len() > __i {
                        return Err(format!("error: usage: {} {}",
                                           _cmd, __usage[String::from_str(_cmd)]))
                    }

                    /* finally, actually parse out the typed arguments */
                    let mut __j : uint = 0;
                    $(
                        $name = match from_str::<$arg>(argv[__j]) {
                            Some(val) => val,
                            None => return Err(format!("error: {}: not a {}: {}",
                                                   _cmd, stringify!($arg), argv[__j]))
                        };
                        __j += 1;
                    )*
                    assert_eq!(__j, __i);
                    Ok(Some($code))
                };
                __commands.insert(__command_num, command);
                $(__index_map.insert(String::from_str($cmd), __command_num);)*
                __command_num += 1;
            })*

            /* make "help" command */
            let mut __help_cmd : String = String::from_str("Commands:\n");
            $({
                let mut __aliases : Vec<String> = Vec::new();
                let mut __alias_str : String = format!("(");
                $(
                    __aliases.push(String::from_str($cmd));
                    __alias_str.push_str($cmd);
                    __alias_str.push_str(" | ");
                )+
                assert_eq!(__alias_str.pop().unwrap(), ' ');
                assert_eq!(__alias_str.pop().unwrap(), '|');
                assert_eq!(__alias_str.pop().unwrap(), ' ');
                __alias_str.push(')');

                __help_cmd.push_str(format!("\t{} {} : {}\n", __alias_str,
                                            __usage[__aliases[0]], $desc).as_slice());
            })*

            __commands.insert(__command_num, |_cmd : &str, _argv : &[&str]| {
                println!("{}", __help_cmd);
                Ok(None)
            });
            __index_map.insert(String::from_str("help"), __command_num);

            /* lastly, define the closure that performs the action */
            let $commands = |cmd : &str, argv : &[&str]| -> Result<Option<$ret>, String> {
                match __index_map.get(&String::from_str(cmd)) {
                    Some(index) => match __commands.get_mut(index) {
                        Some(ref mut f) => (**f)(cmd, argv),
                        None => unreachable!()
                    },
                    None => Err(format!("error: unknown command: {}\nType 'help' to list commands.", cmd))
                }
            };

            $action
        }
    })
}
