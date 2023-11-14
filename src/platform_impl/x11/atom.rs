use std::ffi::CString;

use x11_dl::xlib::{Atom, Display, False, Xlib};

#[derive(Default)]
pub struct XDNDAtoms {
    pub xdnd_aware: Atom,
    pub xdnd_enter: Atom,
    pub xdnd_selection: Atom,
    pub xdnd_position: Atom,
    pub xdnd_action_copy: Atom,
    pub xdnd_leave: Atom,
    pub xdnd_status: Atom,
    pub xdnd_drop: Atom,
    pub xa_atom: Atom,
    pub types: [Atom; 1],
}

impl XDNDAtoms {
    /// # Safety
    pub unsafe fn new(xlib: &Xlib, display: *mut Display) -> Self {
        Self {
            xdnd_aware: (xlib.XInternAtom)(
                display,
                CString::new("XdndAware")
                    .unwrap()
                    .as_bytes_with_nul()
                    .as_ptr(),
                False,
            ),
            xdnd_enter: (xlib.XInternAtom)(
                display,
                CString::new("XdndEnter")
                    .unwrap()
                    .as_bytes_with_nul()
                    .as_ptr(),
                False,
            ),
            xdnd_selection: (xlib.XInternAtom)(
                display,
                CString::new("XdndSelection")
                    .unwrap()
                    .as_bytes_with_nul()
                    .as_ptr(),
                False,
            ),
            xdnd_position: (xlib.XInternAtom)(
                display,
                CString::new("XdndPosition")
                    .unwrap()
                    .as_bytes_with_nul()
                    .as_ptr(),
                False,
            ),
            xdnd_action_copy: (xlib.XInternAtom)(
                display,
                CString::new("XdndActionCopy")
                    .unwrap()
                    .as_bytes_with_nul()
                    .as_ptr(),
                False,
            ),
            xdnd_leave: (xlib.XInternAtom)(
                display,
                CString::new("XdndLeave")
                    .unwrap()
                    .as_bytes_with_nul()
                    .as_ptr(),
                False,
            ),
            xdnd_status: (xlib.XInternAtom)(
                display,
                CString::new("XdndStatus")
                    .unwrap()
                    .as_bytes_with_nul()
                    .as_ptr(),
                False,
            ),
            xdnd_drop: (xlib.XInternAtom)(
                display,
                CString::new("XdndDrop")
                    .unwrap()
                    .as_bytes_with_nul()
                    .as_ptr(),
                False,
            ),
            xa_atom: (xlib.XInternAtom)(
                display,
                CString::new("XA_ATOM")
                    .unwrap()
                    .as_bytes_with_nul()
                    .as_ptr(),
                False,
            ),
            types: [(xlib.XInternAtom)(
                display,
                CString::new("text/uri-list")
                    .unwrap()
                    .as_bytes_with_nul()
                    .as_ptr(),
                False,
            )],
        }
    }
}
