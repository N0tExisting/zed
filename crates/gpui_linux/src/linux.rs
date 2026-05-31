mod dispatcher;
mod headless;
mod keyboard;
mod platform;
#[cfg(any(feature = "wayland", feature = "x11"))]
mod text_system;
#[cfg(feature = "wayland")]
mod wayland;
#[cfg(feature = "x11")]
mod x11;

#[cfg(any(feature = "wayland", feature = "x11"))]
mod xdg_desktop_portal;

pub use dispatcher::*;
pub(crate) use headless::*;
pub(crate) use keyboard::*;
pub(crate) use platform::*;
#[cfg(any(feature = "wayland", feature = "x11"))]
pub(crate) use text_system::*;
#[cfg(feature = "wayland")]
pub(crate) use wayland::*;
#[cfg(feature = "x11")]
pub(crate) use x11::*;

use gpui::{Compositor, Platform, guess_compositor};

use std::rc::Rc;

fn new_headless() -> Rc<dyn Platform> {
    Rc::new(LinuxPlatform {
        inner: HeadlessClient::new(),
    })
}

/// This can only be reached when the feature `x11`/`wayland` is enabled in gpui but not here
/// (It shoudn't happen, but it did happen to me, so ¯\_(ツ)_/¯)
/// panics from ´unreachable!´
fn disabled_compositor(compositor: Compositor) -> Rc<dyn Platform> {
    unreachable!(
        "Got compositor `{}`, but gpui_linux was compiled without that compositor feature",
        compositor
    );
    new_headless()
}

/// Returns the default platform implementation for the current OS.
pub fn current_platform(headless: bool) -> Rc<dyn Platform> {
    #[cfg(feature = "x11")]
    use anyhow::Context as _;

    if headless {
        return new_headless();
    }

    match guess_compositor() {
        Compositor::Headless => new_headless(),

        #[cfg(feature = "wayland")]
        Compositor::Wayland => Rc::new(LinuxPlatform {
            inner: WaylandClient::new(),
        }),
        #[cfg(not(feature = "wayland"))]
        Compositor::Wayland => disabled_compositor(Compositor::Wayland),

        #[cfg(feature = "x11")]
        Compositor::X11 => Rc::new(LinuxPlatform {
            inner: X11Client::new()
                .context("Failed to initialize X11 client.")
                .unwrap(),
        }),
        #[cfg(not(feature = "x11"))]
        Compositor::X11 => disabled_compositor(Compositor::X11),
    }
}
