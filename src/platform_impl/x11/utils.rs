use std::{
    ffi::{c_void, CStr},
    ptr::null_mut,
};

use x11_dl::xlib::{
    AnyPropertyType, Atom, Colormap, Display, False, Success, Window, XClientMessageEvent,
    XWindowAttributes, Xlib,
};

use super::atom::XDNDAtoms;

pub unsafe fn print_client_message(
    xlib: &Xlib,
    display: *mut Display,
    message: &XClientMessageEvent,
) {
    // Get atom type as string
    let message_type_str = (xlib.XGetAtomName)(display, message.message_type);
    println!(
        "Message type: [{}]",
        CStr::from_ptr(message_type_str).to_string_lossy()
    );
    // (xlib.XFree)(message_type_str as *mut c_void);

    // Handle format
    println!("Message word size: {} bits", message.format);
    print!("Message ");
    match message.format {
        8 => {
            println!("bytes: ");
            for i in 0..20 {
                print!("{} ", message.data.get_byte(i));
            }
        }
        16 => {
            println!("16-bit shorts: ");
            for i in 0..10 {
                print!("{} ", message.data.get_short(i));
            }
        }
        32 => {
            println!("32-bit longs: ");
            for i in 0..5 {
                print!("{} ", message.data.get_long(i));
            }
        }
        _ => {}
    };
    println!("\n");
}

pub(crate) fn get_window_pointer_is_over(
    xlib: &Xlib,
    display: *mut Display,
    starting_window: Window,
    p_root_x: i32,
    p_root_y: i32,
    origin_x: i32,
    origin_y: i32,
) -> Option<Window> {
    unsafe {
        let mut return_window: Option<Window> = None;
        let mut root_return: Window = Window::default();
        let mut parent_return: Window = Window::default();
        let mut child_list: *mut Window = std::ptr::null_mut();

        let mut num_of_children: u32 = u32::default();

        if (xlib.XQueryTree)(
            display,
            starting_window,
            &mut root_return,
            &mut parent_return,
            &mut child_list,
            &mut num_of_children,
        ) != 0
        {
            for i in (0..num_of_children).rev() {
                let mut child_attrs = XWindowAttributes {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                    border_width: 0,
                    depth: 0,
                    visual: null_mut(),
                    root: Window::default(),
                    class: 0,
                    bit_gravity: 0,
                    win_gravity: 0,
                    backing_store: 0,
                    backing_planes: 0,
                    backing_pixel: 0,
                    save_under: False,
                    colormap: Colormap::default(),
                    map_installed: False,
                    map_state: 0,
                    all_event_masks: 0,
                    your_event_mask: 0,
                    do_not_propagate_mask: 0,
                    override_redirect: False,
                    screen: null_mut(),
                };
                (xlib.XGetWindowAttributes)(
                    display,
                    *child_list.wrapping_add(i.try_into().unwrap()),
                    &mut child_attrs,
                );

                if p_root_x >= origin_x + child_attrs.x
                    && p_root_x < origin_x + child_attrs.x + child_attrs.width
                    && p_root_y >= origin_y + child_attrs.y
                    && p_root_y < origin_y + child_attrs.y + child_attrs.height
                {
                    return_window = get_window_pointer_is_over(
                        xlib,
                        display,
                        *child_list.wrapping_add(i.try_into().unwrap()),
                        p_root_x,
                        p_root_y,
                        origin_x + child_attrs.x,
                        origin_y + child_attrs.y,
                    );
                    break;
                }
            }
            (xlib.XFree)(child_list as *mut c_void);
        }

        if return_window.is_none() {
            return_window = Some(starting_window);
        }

        return_window
    }
}

pub(crate) fn has_correct_xdnd_aware_property(
    xlib: &Xlib,
    xdnd_atoms: &XDNDAtoms,
    display: *mut Display,
    window: Window,
    xdnd_protocol_version: u32,
) -> i32 {
    unsafe {
        let mut ret_val: i32 = 0;
        let mut actual_type: Atom = Atom::default();
        let mut actual_format: i32 = 0;
        let mut num_of_items: u64 = 0;
        let mut bytes_after_return: u64 = 0;
        let mut data: *mut u8 = std::ptr::null_mut();

        let result = (xlib.XGetWindowProperty)(
            display,
            window,
            xdnd_atoms.xdnd_aware,
            0,
            1024,
            False,
            AnyPropertyType as u64,
            &mut actual_type,
            &mut actual_format,
            &mut num_of_items,
            &mut bytes_after_return,
            &mut data,
        );

        if result == Success as i32 && actual_type != 0 {
            // Assume architecture is little endian and just read first byte for
            // XDND protocol version
            if *data.wrapping_add(0) as u32 <= xdnd_protocol_version {
                ret_val = *data.wrapping_add(0) as i32;
            }

            (xlib.XFree)(data as *mut c_void);
        }

        ret_val
    }
}
