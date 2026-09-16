use crate::{Status, SimpleTextInputProtocol, SimpleTextOutputProtocol, UefiConsole, InputKey};
use core::fmt::Write;

const SCAN_UP: u16 = 0x01;
const SCAN_DOWN: u16 = 0x02;
const CHAR_ENTER: u16 = 0x0D;

const COLOR_DEFAULT: usize = 0x07;
const COLOR_HIGHLIGHT: usize = 0x70;

pub fn show_menu(
    console: &mut UefiConsole,
    con_in: *mut SimpleTextInputProtocol,
    con_out_ptr: *mut SimpleTextOutputProtocol,
) -> usize {
    let options = [
        "Boot Linux Kernel ",
        "Poweroff System   "
    ];
    let mut selected_index = 0;

    loop {
        unsafe {
            ((*con_out_ptr).set_attribute)(con_out_ptr, COLOR_DEFAULT);
            ((*con_out_ptr).clear_screen)(con_out_ptr);
        }

        let _ = write!(console, "\n\n");
        let _ = write!(console, "    ----------------------------------------\n");
        let _ = write!(console, "            Aaah, lazy. Time to boot        \n");
        let _ = write!(console, "    ----------------------------------------\n\n");

        for i in 0..options.len() {
            unsafe { ((*con_out_ptr).set_attribute)(con_out_ptr, COLOR_DEFAULT); }
            let _ = write!(console, "    ");

            if i == selected_index {
                unsafe { ((*con_out_ptr).set_attribute)(con_out_ptr, COLOR_HIGHLIGHT); }
                let _ = write!(console, "{}", options[i]);

                unsafe { ((*con_out_ptr).set_attribute)(con_out_ptr, COLOR_DEFAULT); }
            } else {
                let _ = write!(console, "{}", options[i]);
            }

            let _ = write!(console, "\n");
        }

        let _ = write!(console, "\n    [ UP / DOWN: Navigation ] | [ ENTER: Chose ]\n");

        loop {
            let mut key = InputKey { scan_code: 0, unicode_char: 0 };
            let status = unsafe { ((*con_in).read_key_stroke)(con_in, &mut key) };

            if status == crate::Status::NOT_READY {
                core::hint::spin_loop();
                continue;
            }

            if status == crate::Status::SUCCESS {
                if key.scan_code == SCAN_UP {
                    if selected_index > 0 { selected_index -= 1; }
                    break;
                }
                else if key.scan_code == SCAN_DOWN {
                    if selected_index < options.len() - 1 { selected_index += 1; }
                    break;
                }
                else if key.unicode_char == CHAR_ENTER {
                    unsafe {
                        ((*con_out_ptr).set_attribute)(con_out_ptr, COLOR_DEFAULT);
                        ((*con_out_ptr).clear_screen)(con_out_ptr);
                    }
                    return selected_index;
                }
            }
        }
    }
}
