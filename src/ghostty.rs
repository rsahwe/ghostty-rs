use std::{
    marker::PhantomData,
    slice::from_raw_parts,
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Release},
    },
};

use ghostty_sys::{
    ghostty_build_mode_e, ghostty_build_mode_e_GHOSTTY_BUILD_MODE_DEBUG,
    ghostty_build_mode_e_GHOSTTY_BUILD_MODE_RELEASE_FAST,
    ghostty_build_mode_e_GHOSTTY_BUILD_MODE_RELEASE_SAFE,
    ghostty_build_mode_e_GHOSTTY_BUILD_MODE_RELEASE_SMALL, ghostty_cli_main, ghostty_info,
    ghostty_info_s, ghostty_init,
};

// Main type from which more functions can be called. Create with init().
#[derive(Debug)]
pub struct Ghostty {
    // Impossible to construct
    marker: PhantomData<()>,
}

// Build mode. ghostty-sys uses ReleaseSafe by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuildMode {
    Debug,
    ReleaseSafe,
    ReleaseFast,
    ReleaseSmall,
}

// Build info for ghostty.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Info {
    pub build_mode: BuildMode,
    pub version: String,
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

// Initializes the ghostty global state.
pub fn init() -> Option<Ghostty> {
    let res = unsafe { ghostty_init() };

    bool::then(res == 0, || {
        INITIALIZED.store(true, Release);
        Ghostty {
            marker: PhantomData,
        }
    })
}

// Runs the cli version. Cannot be used if init() was called.
pub fn cli_main(mut args: Vec<String>) -> ! {
    if INITIALIZED.load(Acquire) {
        panic!("Ghostty already initialized!");
    }

    let mut raw_args = args
        .iter_mut()
        .map(|st| st.as_mut_ptr())
        .collect::<Vec<_>>();

    unsafe { ghostty_cli_main(raw_args.len(), raw_args.as_mut_ptr() as *mut *mut i8) };

    unreachable!()
}

impl Ghostty {}

impl Info {
    // Should be safe
    fn from_raw(raw: ghostty_info_s) -> Self {
        Self {
            build_mode: BuildMode::from_raw(raw.build_mode),
            version: unsafe {
                String::from_utf8(from_raw_parts(raw.version as *const u8, raw.version_len).into())
                    .unwrap()
            },
        }
    }
}

impl BuildMode {
    fn from_raw(raw: ghostty_build_mode_e) -> Self {
        match raw {
            ghostty_build_mode_e_GHOSTTY_BUILD_MODE_DEBUG => BuildMode::Debug,
            ghostty_build_mode_e_GHOSTTY_BUILD_MODE_RELEASE_FAST => BuildMode::ReleaseFast,
            ghostty_build_mode_e_GHOSTTY_BUILD_MODE_RELEASE_SAFE => BuildMode::ReleaseSafe,
            ghostty_build_mode_e_GHOSTTY_BUILD_MODE_RELEASE_SMALL => BuildMode::ReleaseSmall,
            // Assume debug when type is unknown because it never should be
            _ => BuildMode::Debug,
        }
    }
}

// Get the ghostty build info.
pub fn info() -> Info {
    unsafe { Info::from_raw(ghostty_info()) }
}
