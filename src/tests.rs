#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use crate::ghostty::{Ghostty, info, init};

    static INIT: OnceLock<Ghostty> = OnceLock::new();

    #[test]
    fn test_ghostty_init() -> Result<(), ()> {
        let _ghostty = INIT.get_or_init(|| init().expect("Could not init"));

        Ok(())
    }

    #[test]
    fn test_ghostty_info() {
        let mut info = info();
        info.version
            .drain(0..info.version.len())
            .for_each(|c| print!("{c}"));
    }

    #[test]
    fn config_create() -> Result<(), ()> {
        let ghostty = INIT.get_or_init(|| init().expect("Could not init"));
        let conf = ghostty.create_config();
        println!("{:?}", conf);

        Ok(())
    }

    #[test]
    fn config_load() -> Result<(), ()> {
        let ghostty = INIT.get_or_init(|| init().expect("Could not init"));
        let conf = ghostty.load_config();
        println!("{:?}", conf);

        Ok(())
    }

    #[test]
    fn config_get() -> Result<(), ()> {
        let ghostty = INIT.get_or_init(|| init().expect("Could not init"));
        let conf = ghostty.load_config();
        println!("{:?}", conf);
        println!(
            "val 'resize-overlay-duration' is {:?}",
            conf.resize_overlay_duration_ms()
        );

        Ok(())
    }

    #[test]
    fn config_title() -> Result<(), ()> {
        let ghostty = INIT.get_or_init(|| init().expect("Could not init"));
        let conf = ghostty.load_config();
        println!("{:?}", conf);
        println!("val 'title' is {:?}", conf.window_title());

        Ok(())
    }

    #[test]
    fn diagnostics() -> Result<(), ()> {
        let ghostty = INIT.get_or_init(|| init().expect("Could not init"));
        let conf = ghostty.load_config();
        for diag in conf.diagnostics() {
            println!("{:?}", diag);
        }

        Ok(())
    }
}
