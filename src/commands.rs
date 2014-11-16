#![feature(macro_rules)]

#[macro_export]
macro_rules! commands {
    (with $commands:ident : $ret:ty = {$(($($cmd:expr),+) ~ ($($name:ident : $arg:ty)*) => $code:expr),+},
            do : $action:expr) => ({
        use std::collections::hash_map::HashMap;

        let mut help : HashMap<String, String> = HashMap::new();
        $({ /* put command in help message map */
            let mut _usage : String = String::new();
            $(
                _usage.push_str(" <");
                _usage.push_str(stringify!($arg));
                _usage.push_str("> ");
            )*
            /* then put usage string into help map with all command names */
            $(help.insert(String::from_str($cmd), String::from_str($cmd) + _usage);)+
        })+

        /* now build the mapping from command name to functions */
        {
            let mut index_map : HashMap<String, uint> = HashMap::new();
            let mut commands : HashMap<uint, |&str, &[&str]| -> Result<Option<$ret>, String>> = HashMap::new();

            let mut _command_num : uint = 0;
            $({ /* for each command, give it a map entry */
                let command = |_cmd : &str, argv : &[&str]| -> Result<Option<$ret>, String> {
                    let mut _i : uint = 0;
                    $(let mut $name : $arg;)*
                    $(
                        if _i < argv.len() {
                            $name = match from_str::<$arg>(argv[_i]) {
                                Some(val) => val,
                                None => return Err(format!("error: {}: not a {}: {}",
                                                       _cmd, stringify!($arg), argv[_i]))
                            };
                        }
                        else {
                            return Err(format!("error: usage: {}", help[String::from_str(_cmd)]))
                        }
                        _i += 1;
                    )*
                    if _i < argv.len() {
                        return Err(format!("error: usage: {}", help[String::from_str(_cmd)]))
                    }
                    Ok(Some($code))
                };
                commands.insert(_command_num, command);
                $(index_map.insert(String::from_str($cmd), _command_num);)+
                _command_num += 1;
            })+

            /* make "help" command */
            let mut help_cmd : String = String::from_str("Commands:\n");
            for cmd in index_map.keys() {
                help_cmd.push('\t');
                help_cmd.push_str(help[*cmd].as_slice());
                help_cmd.push('\n');
            }

            commands.insert(_command_num, |_cmd : &str, _argv : &[&str]| {
                println!("{}", help_cmd);
                Ok(None)
            });
            index_map.insert(String::from_str("help"), _command_num);

            let $commands = |cmd : &str, argv : &[&str]| -> Result<Option<$ret>, String> {
                match index_map.get(&String::from_str(cmd)) {
                    Some(index) => match commands.get_mut(index) {
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
