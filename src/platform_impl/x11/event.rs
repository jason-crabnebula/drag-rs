use std::ffi::CString;
use x11_dl::xlib::{
    ClientMessage, Display, False, PropModeReplace, SelectionNotify, Window, XEvent,
    XSelectionRequestEvent, Xlib,
};

use super::{atom::XDNDAtoms, XDNDState};

pub(crate) unsafe fn send_selection_notify(
    xlib: &Xlib,
    display: *mut Display,
    selection_request: &XSelectionRequestEvent,
    path_str: &str,
) {
    let property_data: CString = CString::new(format!("file://{}\r\n", path_str)).unwrap();

    (xlib.XChangeProperty)(
        display,
        selection_request.requestor,
        selection_request.property,
        selection_request.target,
        8,
        PropModeReplace,
        property_data.as_bytes_with_nul().as_ptr(),
        property_data.as_bytes_with_nul().len() as i32,
    );

    #[allow(clippy::uninit_assumed_init)]
    let mut message: XEvent = std::mem::MaybeUninit::uninit().assume_init();

    message.selection.type_ = SelectionNotify;
    message.selection.display = display;
    message.selection.requestor = selection_request.requestor;
    message.selection.selection = selection_request.selection;
    message.selection.target = selection_request.target;
    message.selection.property = selection_request.property;
    message.selection.time = selection_request.time;

    if (xlib.XSendEvent)(display, selection_request.requestor, False, 0, &mut message) == 0 {
        dbg!("Error XSendEvent SelectionNotify");
    }
}

pub(crate) unsafe fn send_xdnd_enter(
    xlib: &Xlib,
    xdnd_atoms: &XDNDAtoms,
    display: *mut Display,
    xdnd_version: u32,
    source: Window,
    target: Window,
) {
    #[allow(clippy::uninit_assumed_init)]
    let mut message: XEvent = std::mem::MaybeUninit::uninit().assume_init();

    message.client_message.type_ = ClientMessage;
    message.client_message.display = display;
    message.client_message.window = target;
    message.client_message.message_type = xdnd_atoms.xdnd_enter;
    message.client_message.format = 32;
    message.client_message.data.set_long(0, source as i64);
    message
        .client_message
        .data
        .set_long(1, (xdnd_version << 24).into());

    // FIXME: currently using URI_LIST only.
    message
        .client_message
        .data
        .set_long(2, xdnd_atoms.types[0].try_into().unwrap());

    if (xlib.XSendEvent)(display, target, False, 0, &mut message) == 0 {
        dbg!("Error XSendEvent XDND_ENTER");
    }
}

pub(crate) unsafe fn send_xdnd_drop(
    xlib: &Xlib,
    xdnd_state: &XDNDState,
    xdnd_atoms: &XDNDAtoms,
    display: *mut Display,
    source: Window,
    target: Window,
) {
    #[allow(clippy::uninit_assumed_init)]
    let mut message: XEvent = std::mem::MaybeUninit::uninit().assume_init();

    message.client_message.type_ = ClientMessage;
    message.client_message.display = display;
    message.client_message.window = target;
    message.client_message.message_type = xdnd_atoms.xdnd_drop;
    message.client_message.format = 32;
    message.client_message.data.set_long(0, source as i64);
    message
        .client_message
        .data
        .set_long(2, xdnd_state.last_position_timestamp as i64);

    if (xlib.XSendEvent)(display, target, False, 0, &mut message) == 0 {
        dbg!("Error XSendEvent XDND_DROP");
    }
}

pub(crate) unsafe fn send_xdnd_leave(
    xlib: &Xlib,
    xdnd_atoms: &XDNDAtoms,
    display: *mut Display,
    source: Window,
    target: Window,
) {
    #[allow(clippy::uninit_assumed_init)]
    let mut message: XEvent = std::mem::MaybeUninit::uninit().assume_init();

    message.client_message.type_ = ClientMessage;
    message.client_message.display = display;
    message.client_message.window = target;
    message.client_message.message_type = xdnd_atoms.xdnd_leave;
    message.client_message.format = 32;
    message.client_message.data.set_long(0, source as i64);

    if (xlib.XSendEvent)(display, target, False, 0, &mut message) == 0 {
        dbg!("Error XSendEvent XDND_LEAVE");
    }
}

pub(crate) unsafe fn send_xdnd_position(
    xlib: &Xlib,
    xdnd_atoms: &XDNDAtoms,
    display: *mut Display,
    source: Window,
    target: Window,
    time: u64,
    p_root_x: i32,
    p_root_y: i32,
) {
    #[allow(clippy::uninit_assumed_init)]
    let mut message: XEvent = std::mem::MaybeUninit::uninit().assume_init();

    message.client_message.type_ = ClientMessage;
    message.client_message.display = display;
    message.client_message.window = target;
    message.client_message.message_type = xdnd_atoms.xdnd_position;
    message.client_message.format = 32;
    message.client_message.data.set_long(0, source as i64);
    message
        .client_message
        .data
        .set_long(2, (p_root_x << 16 | p_root_y) as i64);
    message.client_message.data.set_long(3, time as i64);
    message
        .client_message
        .data
        .set_long(4, xdnd_atoms.xdnd_action_copy as i64);

    if (xlib.XSendEvent)(display, target, False, 0, &mut message) == 0 {
        dbg!("Error XSendEvent");
    }
}
