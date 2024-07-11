pub struct ConfigArgs<'a> {
    config_mode: &'a str,
}

impl<'a> ConfigArgs<'a> {
    pub fn new(args: &[String]) -> Self {
        let config_mode = if args.get(1).map_or(true, |arg| arg.is_empty()) {
            "pc_config"
        } else if args[1] == "vm_config" {
            "vm_config"
        } else {
            "pc_config"
        };

        ConfigArgs { config_mode }
    }

    pub fn get_config_mode(&self) -> &str {
        self.config_mode
    }
}
