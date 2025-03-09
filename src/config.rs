use std::{
    ffi::CStr,
    os::raw::{c_char, c_double, c_void},
    ptr::null,
};

use sys::{
    ghostty_config_clone, ghostty_config_color_s, ghostty_config_diagnostics_count,
    ghostty_config_finalize, ghostty_config_free, ghostty_config_get,
    ghostty_config_get_diagnostic, ghostty_config_load_cli_args, ghostty_config_load_default_files,
    ghostty_config_load_recursive_files, ghostty_config_open, ghostty_config_t,
    ghostty_diagnostic_s,
};

// Struct for terminal configuration files.
#[derive(Debug)]
pub struct Config {
    handle: ghostty_config_t,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConfigColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Diagnostic {
    pub message: String,
}

// Deep clones a Config.
impl Clone for Config {
    fn clone(&self) -> Self {
        Self {
            handle: unsafe { ghostty_config_clone(self.handle) },
        }
    }
}

// Frees a Config
impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            ghostty_config_free(self.handle);
        }
    }
}

impl Config {
    pub(crate) fn from_raw(raw: ghostty_config_t) -> Self {
        Self { handle: raw }
    }

    // Load the configuration from the CLI args.
    pub fn load_cli_args(&mut self) -> &mut Self {
        unsafe {
            ghostty_config_load_cli_args(self.handle);
        };
        self
    }

    // Load the configuration from the default file locations. This
    // is usually done first. The default file locations are locations
    // such as the home directory.
    pub fn load_default_files(&mut self) -> &mut Self {
        unsafe {
            ghostty_config_load_default_files(self.handle);
        };
        self
    }

    // Load the configuration from the user-specified configuration
    // file locations in the previously loaded configuration. This will
    // recursively continue to load up to a built-in limit.
    pub fn load_recursive_files(&mut self) -> &mut Self {
        unsafe {
            ghostty_config_load_recursive_files(self.handle);
        };
        self
    }

    // Finalize the config for loading.
    pub fn finalize(&mut self) -> &mut Self {
        unsafe {
            ghostty_config_finalize(self.handle);
        };
        self
    }

    fn get_bool(&self, name: &str) -> bool {
        let mut val = false;
        unsafe {
            ghostty_config_get(
                self.handle,
                &mut val as *mut bool as *mut c_void,
                name.as_ptr() as *const c_char,
                name.len(),
            )
        };
        val
    }

    fn get_str(&self, name: &str) -> String {
        let mut raw: *const i8 = null();
        if !unsafe {
            ghostty_config_get(
                self.handle,
                &mut raw as *mut *const c_char as *mut c_void,
                name.as_ptr() as *const c_char,
                name.len(),
            )
        } {
            println!("False");
        }
        if raw.is_null() {
            "".to_string()
        } else {
            unsafe { CStr::from_ptr(raw).to_string_lossy().to_string() }
        }
    }

    fn get_u8(&self, name: &str) -> u8 {
        let mut val = 0;
        unsafe {
            ghostty_config_get(
                self.handle,
                &mut val as *mut u8 as *mut c_void,
                name.as_ptr() as *const c_char,
                name.len(),
            )
        };
        val
    }

    fn get_i16(&self, name: &str) -> i16 {
        let mut val = 0;
        unsafe {
            ghostty_config_get(
                self.handle,
                &mut val as *mut i16 as *mut c_void,
                name.as_ptr() as *const c_char,
                name.len(),
            )
        };
        val
    }

    fn get_u64(&self, name: &str) -> u64 {
        let mut val = 0;
        unsafe {
            ghostty_config_get(
                self.handle,
                &mut val as *mut u64 as *mut c_void,
                name.as_ptr() as *const c_char,
                name.len(),
            )
        };
        val
    }

    fn get_color(&self, name: &str) -> ConfigColor {
        let mut val = ghostty_config_color_s { r: 0, g: 0, b: 0 };
        unsafe {
            ghostty_config_get(
                self.handle,
                &mut val as *mut ghostty_config_color_s as *mut c_void,
                name.as_ptr() as *const c_char,
                name.len(),
            )
        };
        ConfigColor::from_raw(val)
    }

    fn get_double(&self, name: &str) -> f64 {
        let mut val = 0.;
        unsafe {
            ghostty_config_get(
                self.handle,
                &mut val as *mut c_double as *mut c_void,
                name.as_ptr() as *const c_char,
                name.len(),
            )
        };
        val
    }

    pub fn create_initial_window(&self) -> bool {
        self.get_bool("initial-window")
    }

    pub fn do_quit_after_window_closed(&self) -> bool {
        self.get_bool("quit-after-window-closed")
    }

    pub fn window_title(&self) -> String {
        self.get_str("title")
    }

    pub fn window_position(&self) -> (i16, i16) {
        (
            self.get_i16("window-position-x"),
            self.get_i16("window-position-y"),
        )
    }

    pub fn window_new_tab_position(&self) -> String {
        self.get_str("window-new-tab-position")
    }

    pub fn window_decorations(&self) -> bool {
        self.get_bool("window-decoration")
    }

    pub fn window_theme(&self) -> String {
        self.get_str("window-theme")
    }

    pub fn window_step_resize(&self) -> bool {
        self.get_bool("window-step-resize")
    }

    pub fn window_fullscreen(&self) -> bool {
        self.get_bool("fullscreen")
    }

    pub fn window_title_font_family(&self) -> String {
        self.get_str("window-title-font-family")
    }

    pub fn focus_follows_mouse(&self) -> bool {
        self.get_bool("focus-follows-mouse")
    }

    pub fn background_color(&self) -> ConfigColor {
        self.get_color("background")
    }

    pub fn foreground_color(&self) -> ConfigColor {
        self.get_color("foreground")
    }

    pub fn background_opacity(&self) -> f64 {
        self.get_double("background-opacity")
    }

    pub fn background_blur_radius(&self) -> u8 {
        self.get_u8("background-blur")
    }

    pub fn unfocused_split_opacity(&self) -> f64 {
        self.get_double("unfocused-split-opacity")
    }

    pub fn unfocused_split_fill(&self) -> ConfigColor {
        self.get_color("unfocused-split-fill")
    }

    pub fn split_divider_color(&self) -> ConfigColor {
        self.get_color("split-divider-color")
    }

    pub fn resize_overlay(&self) -> String {
        self.get_str("resize-overlay")
    }

    pub fn resize_overlay_position(&self) -> String {
        self.get_str("resize-overlay-position")
    }

    pub fn resize_overlay_duration_ms(&self) -> u64 {
        self.get_u64("resize-overlay-duration")
    }

    pub fn start_maximized(&self) -> bool {
        self.get_bool("maximize")
    }

    //TODO: Input triggers

    pub fn diagnostics(&self) -> impl Iterator<Item = Diagnostic> {
        unsafe {
            (0..ghostty_config_diagnostics_count(self.handle))
                .map(|i| ghostty_config_get_diagnostic(self.handle, i))
                .map(Diagnostic::from_raw)
        }
    }

    pub fn open_editor(&self) {
        unsafe { ghostty_config_open() };
    }
}

impl ConfigColor {
    fn from_raw(raw: ghostty_config_color_s) -> Self {
        Self {
            r: raw.r,
            g: raw.g,
            b: raw.b,
        }
    }
}

impl Diagnostic {
    fn from_raw(raw: ghostty_diagnostic_s) -> Self {
        Self {
            message: unsafe { CStr::from_ptr(raw.message).to_string_lossy().to_string() },
        }
    }
}
