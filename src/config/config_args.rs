pub struct Config_Args {
    config_mode : String,
}

impl Config_Args {
    pub fn new(args: &[String]) -> &str {
        if args.is_empty(){
            return "computerMode";
        } else if args[1] == "virtualBoxMode"{
            return "virtualBoxMode";
        } else {    
            return "computerMode";
        }
    }
}