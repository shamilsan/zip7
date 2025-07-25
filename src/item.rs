use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use widestring::U32CString;

use crate::Handle;
use crate::error::Result;

pub struct Zip7Item {
    handle: Arc<Mutex<Handle>>,
    index: usize,
    is_directory: bool,
    path: PathBuf,
    unpacked_size: u64,
    out_path: PathBuf,
}

impl Zip7Item {
    pub fn new(handle: Arc<Mutex<Handle>>, index: usize) -> Self {
        let (is_directory, path, unpacked_size, out_path) = unsafe {
            let handle = handle.lock().unwrap();
            let handle_ptr = handle.0.as_ptr();

            let is_directory = zip7_sys::item_is_dir(handle_ptr, index as _);

            let path_len = zip7_sys::item_path_len(handle_ptr, index as _).min(u16::MAX as _) + 1;
            let mut path_buf = vec![0_u32; path_len as _];
            zip7_sys::item_path(handle_ptr, index as _, path_buf.as_mut_ptr());
            let path = U32CString::from_vec_truncate(path_buf)
                .to_os_string()
                .into();

            let out_path_len =
                zip7_sys::item_out_path_len(handle_ptr, index as _).min(u16::MAX as _) + 1;
            let mut out_path_buf = vec![0_u8; out_path_len as _];
            zip7_sys::item_out_path(handle_ptr, index as _, out_path_buf.as_mut_ptr() as _);
            out_path_buf.pop(); // Remove trailing `\0`
            let out_path = PathBuf::from(String::from_utf8_lossy(&out_path_buf).into_owned());

            let unpacked_size = zip7_sys::item_unpacked_size(handle_ptr, index as _);

            (is_directory, path, unpacked_size, out_path)
        };

        Self {
            handle,
            index,
            is_directory,
            path,
            unpacked_size,
            out_path,
        }
    }

    pub fn is_directory(&self) -> bool {
        self.is_directory
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn size(&self) -> u64 {
        self.unpacked_size
    }

    pub fn set_out_path<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        let path = CString::new(&*path.as_ref().to_string_lossy())?;
        let handle = self.handle.lock().unwrap();
        let handle_ptr = handle.0.as_ptr();
        unsafe { zip7_sys::set_item_out_path(handle_ptr, self.index as _, path.as_ptr()) }
        Ok(())
    }

    pub fn out_path(&self) -> &Path {
        &self.out_path
    }
}
