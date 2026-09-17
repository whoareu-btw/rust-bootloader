#![no_std]
#![no_main]

mod ui;

use core::panic::PanicInfo;
use core::fmt::{self, Write};

pub type Handle = *mut core::ffi::c_void;

#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct Status(pub usize);

impl Status {
    pub const SUCCESS: Status = Status(0);
    pub const NOT_READY: Status = Status((1 << (core::mem::size_of::<usize>() * 8 - 1)) | 6);
}

#[repr(C)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub const LOADED_IMAGE_PROTOCOL_GUID: Guid = Guid {
    data1: 0x5B1B31A1,
    data2: 0x9562,
    data3: 0x11D2,
    data4: [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
};

pub const SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: Guid = Guid {
    data1: 0x964e5b22,
    data2: 0x6459,
    data3: 0x11d2,
    data4: [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
};

#[repr(C)]
pub struct InputKey {
    pub scan_code: u16,
    pub unicode_char:u16,
}

#[repr(C)]
pub struct SimpleTextInputProtocol {
    pub reset: extern "efiapi" fn(*mut SimpleTextInputProtocol, bool) -> Status,
    pub read_key_stroke: extern "efiapi" fn(*mut SimpleTextInputProtocol, *mut InputKey) -> Status,
    pub wait_for_key: Handle,
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: extern "efiapi" fn(*mut SimpleTextOutputProtocol, bool) -> Status,
    pub output_string: extern "efiapi" fn(*mut SimpleTextOutputProtocol, *const u16) -> Status,
    pub test_string: *mut core::ffi::c_void,
    pub query_mode: *mut core::ffi::c_void,
    pub set_mode: *mut core::ffi::c_void,
    pub set_attribute: extern "efiapi" fn(*mut SimpleTextOutputProtocol, usize) -> Status,
    pub clear_screen: extern "efiapi" fn(*mut SimpleTextOutputProtocol) -> Status,
}

#[repr(C)]
pub struct LoadedImageProtocol {
    pub revision: u32,
    pub parent_handle: Handle,
    pub system_table: *mut SystemTable,
    pub device_handle: Handle,
    pub file_path: *mut core::ffi::c_void,
    pub reserved: *mut core::ffi::c_void,
    pub load_options_size: u32,
    pub load_options: *mut core::ffi::c_void,
    pub image_base: u64,
    pub image_code_type: u32,
    pub image_data_typr: u32,
    pub unload: extern "efiapi" fn(Handle) -> Status,
}

#[repr(C)]
pub struct SimpleFileSystemProtocol {
    pub revision: u64,
    pub open_volume: extern "efiapi" fn(*mut SimpleFileSystemProtocol, *mut *mut FileProtocol) -> Status,
}

#[repr(C)]
pub struct FileProtocol {
    pub revision: u64,
    pub open: extern "efiapi" fn(*mut FileProtocol, *mut *mut FileProtocol, *const u16, u64, u64) -> Status,
    pub close: extern "efiapi" fn(*mut FileProtocol) -> Status,
    pub delete: extern "efiapi" fn(*mut FileProtocol) -> Status,
    pub read: extern "efiapi" fn(*mut FileProtocol, *mut usize, *mut core::ffi::c_void) -> Status,
}

#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct BootServices {
    pub hdr: TableHeader,
    pub _pad1: [usize; 2],
    pub _pad2: [usize; 3],

    pub allocate_pool: extern "efiapi" fn(u32, usize, *mut *mut core::ffi::c_void) -> Status,
    pub free_pool: extern "efiapi" fn(*mut core::ffi::c_void) -> Status,
    pub _pad3: [usize; 9],

    pub handle_protocol: extern "efiapi" fn(Handle, *const Guid, *mut *mut core::ffi::c_void) -> Status,
    pub _pad4: [usize; 5],

    pub load_image: extern "efiapi" fn(bool, Handle, *mut core::ffi::c_void, *mut core::ffi::c_void, usize, *mut Handle) -> Status,
    pub start_image: extern "efiapi" fn(Handle, *mut usize, *mut *mut u16) -> Status,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub enum ResetType {
    Cold = 0,
    Warm = 1,
    Shutdown = 2,
    Update = 3,
}

#[repr(C)]
pub struct RuntimeServices {
    pub hdr: TableHeader,
    pub get_time: *mut core::ffi::c_void,
    pub set_time: *mut core::ffi::c_void,
    pub get_wakeup_time: *mut core::ffi::c_void,
    pub set_wakeup_time: *mut core::ffi::c_void,
    pub set_virtual_address_map: *mut core::ffi::c_void,
    pub convert_pointer: *mut core::ffi::c_void,
    pub get_variable: *mut core::ffi::c_void,
    pub get_next_variable_name: *mut core::ffi::c_void,
    pub set_variable: *mut core::ffi::c_void,
    pub get_next_high_monotonic_count: *mut core::ffi::c_void,
    pub reset_system: extern "efiapi" fn(ResetType, Status, usize, *mut core::ffi::c_void),
}

#[repr(C)]
pub struct SystemTable {
    pub hdr: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_revision: u32,
    pub console_in_handle: Handle,
    pub con_in: *mut SimpleTextInputProtocol,
    pub console_out_handle: Handle,
    pub con_out: *mut SimpleTextOutputProtocol,
    pub standard_error_handle: Handle,
    pub std_err: *mut SimpleTextOutputProtocol,
    pub runtime_services: *mut RuntimeServices,
    pub boot_services: *mut BootServices,
}

pub struct UefiConsole(pub *mut SimpleTextOutputProtocol);

impl Write for UefiConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            let mut buf = [0u16; 3];
            let mut idx = 0;

            if c == '\n' {
                buf[idx] = b'\r' as u16;
                idx +=1;
            }

            buf[idx] = c as u16;
            buf[idx + 1] = 0;

            unsafe  {
                ((*self.0).output_string)(self.0, buf.as_ptr());
            }
        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
pub extern "efiapi" fn efi_main(image_handle: Handle, system_table: *mut SystemTable) -> Status {
    unsafe {
        let con_out = (*system_table).con_out;
        let con_in = (*system_table).con_in;
        let boot_services = (*system_table).boot_services;

        ((*con_out).reset)(con_out, false);
        let mut console = UefiConsole(con_out);

        let selected_option = ui::show_menu(&mut console, con_in, con_out);

        if selected_option == 0 {
            let _ = write!(console, "\n[+] Searching boot disk...\n");
            let mut loaded_image: *mut LoadedImageProtocol = core::ptr::null_mut();

            let protocol_status = ((*boot_services).handle_protocol)(
                image_handle,
                &LOADED_IMAGE_PROTOCOL_GUID as *const Guid,
                &mut loaded_image as *mut *mut _ as *mut *mut core::ffi::c_void
            );

            if protocol_status == Status::SUCCESS {
                let mut file_system: *mut SimpleFileSystemProtocol = core::ptr::null_mut();
                let fs_status = ((*boot_services).handle_protocol)(
                    (*loaded_image).device_handle,
                    &SIMPLE_FILE_SYSTEM_PROTOCOL_GUID as *const Guid,
                    &mut file_system as *mut *mut _ as *mut *mut core::ffi::c_void
                );

                if fs_status == Status::SUCCESS {
                    let mut root_dir: *mut FileProtocol = core::ptr::null_mut();
                    if ((*file_system).open_volume)(file_system, &mut root_dir) == Status::SUCCESS {
                        let mut kernel_file: *mut FileProtocol = core::ptr::null_mut();
                        let kernel_path = [
                            b'\\' as u16, b'E' as u16, b'F' as u16, b'I' as u16, b'\\' as u16,
                            b'l' as u16, b'a' as u16, b'z' as u16, b'y' as u16, b'b' as u16, b'o' as u16, b'o' as u16, b't' as u16, b'\\' as u16,
                            b'v' as u16, b'm' as u16, b'l' as u16, b'i' as u16, b'n' as u16, b'u' as u16, b'z' as u16,
                            b'-' as u16, b'l' as u16, b't' as u16, b's' as u16, 0
                        ];

                        if ((*root_dir).open)(root_dir, &mut kernel_file, kernel_path.as_ptr(), 1, 0) == Status::SUCCESS {
                            let _ = write!(console, "[+] bzImage found\n");
                            let mut kernel_buffer: *mut core::ffi::c_void = core::ptr::null_mut();

                            if ((*boot_services).allocate_pool)(2, 32 * 1024 * 1024, &mut kernel_buffer) == Status::SUCCESS {
                                let mut read_size: usize = 32 * 1024 * 1024;
                                if ((*kernel_file).read)(kernel_file, &mut read_size, kernel_buffer) == Status::SUCCESS {
                                    let _ = write!(console, "[+] Reading {} bytes to RAM...\n", read_size);
                                    let mut kernel_handle: Handle = core::ptr::null_mut();

                                    let load_status = ((*boot_services).load_image)(
                                        false, image_handle, core::ptr::null_mut(),
                                        kernel_buffer, read_size, &mut kernel_handle
                                    );

                                    if load_status == Status::SUCCESS {
                                        let _ = write!(console, "[+] Kernel loaded to RAM.\n");
                                        let _ = write!(console, "[+] Setting up TTY console output...\n");

                                        let mut kernel_image: *mut LoadedImageProtocol = core::ptr::null_mut();
                                        let load_proto_status = ((*boot_services).handle_protocol)(
                                            kernel_handle,
                                            &LOADED_IMAGE_PROTOCOL_GUID as *const Guid,
                                            &mut kernel_image as *mut *mut _ as *mut *mut core::ffi::c_void
                                        );

                                        if load_proto_status == Status::SUCCESS {
                                            let cmdline_str = "initrd=\\EFI\\lazyboot\\initramfs-lts root=/dev/vda3 rw rootfstype=ext4 modules=ext4 earlycon=efifb console=tty0 loglevel=quiet\0";
                                            let mut cmdline_utf16: [u16; 128] = [0; 128];
                                            let mut cmd_len = 0;

                                            for (i, c) in cmdline_str.chars().enumerate() {
                                                cmdline_utf16[i] = c as u16;
                                                cmd_len += 1;
                                            }

                                            (*kernel_image).load_options = cmdline_utf16.as_ptr() as *mut core::ffi::c_void;
                                            (*kernel_image).load_options_size = (cmd_len * core::mem::size_of::<u16>()) as u32;
                                            (*kernel_image).device_handle = (*loaded_image).device_handle;
                                        }

                                        let _ = write!(console, "[+] Handing off CPU to Linux Kernel...\n");
                                        ((*boot_services).start_image)(kernel_handle, core::ptr::null_mut(), core::ptr::null_mut());

                                    } else {
                                        let _ = write!(console, "[-] ERROR 7: Load image failed\n");
                                    }
                                } else {
                                    let _ = write!(console, "[-] ERROR 6: Failed reading vmlinuz file to RAM\n");
                                }
                            } else {
                                let _ = write!(console, "[-] ERROR 5: Failed to allocate RAM\n");
                            }
                        } else {
                            let _ = write!(console, "[-] ERROR 4: file 'vmlinux' not found\n");
                        }
                    } else {
                        let _ = write!(console, "[-] ERROR 3: open_volume rejected by UEFI\n");
                    }
                } else {
                    let _ = write!(console, "[-] ERROR 2: SimpleFileSystemProtocol not found.\n");
                }
            } else {
                let _ = write!(console, "[-] ERROR 1: LoadedImageProtocol not valid.\n");
            }
        }
        else if selected_option == 1 {
            let _ = write!(console, "\n[+] Powering off system...\n");
            let runtime_services = (*system_table).runtime_services;
            ((*runtime_services).reset_system)(
                ResetType::Shutdown,
                Status::SUCCESS,
                0,
                core::ptr::null_mut()
            );
        }
    }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
