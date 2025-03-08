#[cfg(test)]
mod tests {
    use crate::ghostty::{info, init};

    #[test]
    fn test_ghostty_init() {
        let _ghostty = init().unwrap();
    }

    #[test]
    fn test_ghostty_info() {
        let mut info = info();
        info.version
            .drain(0..info.version.len())
            .for_each(|c| print!("{c}"));
    }
}
