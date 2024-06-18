use crate::CommandResult;

impl CommandResult {
    pub fn print(&self) {
        use CommandResult::*;
        match self {
            AddRemove(result) => match result {
                Ok(ok) => return (),
                Err(err) => println!("{}", err.to_string())
            },
            DependencyThemeList(result) => match result {
                Ok(ok) => {
                    for item in ok {
                        println!("{}", item);
                    }
                }
                Err(err) => println!("{}", err.to_string())
            },
            ConfigList(result) => match result {
                Ok(ok) => {
                    for item in ok {
                        println!("{}", item.name);
                    }
                }
                Err(err) => println!("{}", err.to_string())
            }
        }
    }
}
