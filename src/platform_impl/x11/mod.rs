use crate::{DragItem, Image};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use x11_dl::xlib::{
    ButtonPress, ButtonPressMask, ButtonRelease, ButtonReleaseMask, ClientMessage, EnterNotify,
    EnterWindowMask, ExposureMask, LeaveNotify, LeaveWindowMask, MotionNotify, PointerMotionMask,
    PropModeReplace, SelectionRequest, Time, Window, XEvent,
};

use self::atom::XDNDAtoms;

mod atom;
mod event;
mod utils;

const XDND_PROTOCOL_VERSION: u32 = 5;

#[derive(Debug, Default)]
pub struct XDNDState {
    exchange_started: bool,
    status_received: bool,
    last_position_timestamp: Time,
    target_window: Window,
}

pub fn set_drag_source() {
    std::thread::spawn(move || unsafe {
        let xlib = x11_dl::xlib::Xlib::open().unwrap();
        let display = (xlib.XOpenDisplay)(std::ptr::null());
        let screen = (xlib.XDefaultScreen)(display);
        let window = (xlib.XCreateSimpleWindow)(
            display,
            (xlib.XRootWindow)(display, screen),
            0,
            0,
            200,
            200,
            1,
            0xFF << 16,
            0xFF,
        );

        let xdnd_version: u32 = XDND_PROTOCOL_VERSION;
        let mut cursor_in_src_window: bool = false;

        let mut xdnd_state: XDNDState = XDNDState::default();
        let xdnd_atoms: XDNDAtoms = XDNDAtoms::new(&xlib, display);

        (xlib.XChangeProperty)(
            display,
            window,
            xdnd_atoms.xdnd_aware,
            xdnd_atoms.xa_atom,
            32,
            PropModeReplace,
            xdnd_version.to_ne_bytes().as_ptr(),
            1,
        );

        if (xlib.XSelectInput)(
            display,
            window,
            ExposureMask
                | ButtonPressMask
                | ButtonReleaseMask
                | PointerMotionMask
                | EnterWindowMask
                | LeaveWindowMask,
        ) == 0
        {
            println!("Failed to select input");
        }

        (xlib.XMapWindow)(display, window);

        #[allow(clippy::uninit_assumed_init)]
        let mut event: XEvent = std::mem::MaybeUninit::uninit().assume_init();
        loop {
            (xlib.XNextEvent)(display, &mut event);

            let event_type = event.get_type();
            (|| {
                match event_type {
                    SelectionRequest => {
                        if xdnd_state.exchange_started {
                            event::send_selection_notify(
                                &xlib,
                                display,
                                &event.selection_request,
                                "/home/jason/drag-rs/examples/icon.png",
                            );
                        }
                    }
                    MotionNotify => {
                        if !cursor_in_src_window {
                            let win: Window = (xlib.XDefaultRootWindow)(display);
                            let target_window = utils::get_window_pointer_is_over(
                                &xlib,
                                display,
                                win,
                                event.motion.x_root,
                                event.motion.y_root,
                                0,
                                0,
                            );
                            if target_window.is_none() {
                                return;
                            }
                            let target_window = target_window.unwrap();

                            if xdnd_state.exchange_started
                                && target_window != xdnd_state.target_window
                            {
                                println!("Reset exchange state");
                                event::send_xdnd_leave(
                                    &xlib,
                                    &xdnd_atoms,
                                    display,
                                    window,
                                    xdnd_state.target_window,
                                );

                                xdnd_state = XDNDState::default();
                            }

                            if !xdnd_state.exchange_started {
                                let supports_xdnd = utils::has_correct_xdnd_aware_property(
                                    &xlib,
                                    &xdnd_atoms,
                                    display,
                                    target_window,
                                    XDND_PROTOCOL_VERSION,
                                );
                                dbg!(supports_xdnd);
                                if supports_xdnd == 0 {
                                    return;
                                }

                                (xlib.XSetSelectionOwner)(
                                    display,
                                    xdnd_atoms.xdnd_selection,
                                    window,
                                    event.motion.time,
                                );

                                event::send_xdnd_enter(
                                    &xlib,
                                    &xdnd_atoms,
                                    display,
                                    supports_xdnd as u32,
                                    win,
                                    target_window,
                                );
                                xdnd_state.exchange_started = true;
                                xdnd_state.target_window = target_window;
                            } else {
                                event::send_xdnd_position(
                                    &xlib,
                                    &xdnd_atoms,
                                    display,
                                    window,
                                    target_window,
                                    event.motion.time,
                                    event.motion.x_root,
                                    event.motion.y_root,
                                );
                                xdnd_state.last_position_timestamp = event.motion.time;
                            }
                        }
                    }
                    ButtonPress => {
                        println!("ButtonPress");
                    }
                    ButtonRelease => {
                        if xdnd_state.exchange_started && xdnd_state.status_received {
                            event::send_xdnd_drop(
                                &xlib,
                                &xdnd_state,
                                &xdnd_atoms,
                                display,
                                window,
                                xdnd_state.target_window,
                            );
                        }
                    }
                    EnterNotify => {
                        cursor_in_src_window = true;
                    }
                    LeaveNotify => {
                        cursor_in_src_window = false;
                    }
                    ClientMessage => {
                        // println!("ClientMessage");
                        // print_client_message(&xlib, display, &event.client_message);
                        if event.client_message.message_type == xdnd_atoms.xdnd_status {
                            xdnd_state.status_received = true;

                            // Check if target accept the drop
                            if event.client_message.data.get_long(1) & 0x1 != 1 {
                                println!("Target not accept drop");
                                if xdnd_state.exchange_started {
                                    event::send_xdnd_leave(
                                        &xlib,
                                        &xdnd_atoms,
                                        display,
                                        window,
                                        xdnd_state.target_window,
                                    );
                                }
                                xdnd_state = XDNDState::default();
                            }
                        }
                    }
                    _ => {
                        dbg!(event_type);
                    }
                }
            })();
        }
    });
}

pub fn start_drag<W: HasRawWindowHandle>(_handle: &W, _item: DragItem, _image: Image) {}
