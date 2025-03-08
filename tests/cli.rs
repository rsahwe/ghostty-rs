use std::env::args;

#[test]
fn cli() {
    ghostty::ghostty::cli_main(args().collect())
}
