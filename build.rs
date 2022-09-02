fn main() {
    #[cfg(not(windows))] {
        panic!("wiiu_hb_installer is currently not supported on non-windows systems.");
    }
}