use std::ffi::CString;
use std::path::Path;
use std::ptr::{self, NonNull};
use std::sync::{Arc, Mutex, OnceLock};

use crate::error::{Result, Zip7Error};
pub use crate::item::Zip7Item;

mod error;
mod item;

static INIT: OnceLock<()> = OnceLock::new();

#[derive(Debug)]
pub struct Handle(NonNull<zip7_sys::Handle>);

unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { zip7_sys::close_archive(self.0.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct Zip7Archive {
    handle: Arc<Mutex<Handle>>,
    item_index: usize,
    item_count: usize,
}

impl Zip7Archive {
    pub fn new<P>(path: P, password: Option<&[u8]>) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        INIT.get_or_init(|| unsafe { zip7_sys::init() });

        let mut handle = ptr::null_mut();
        let path = CString::new(path.as_ref().to_string_lossy().as_bytes())?;
        let password = if let Some(pw) = password {
            CString::new(pw)?
        } else {
            CString::default()
        };
        let password_ptr = if password.is_empty() {
            ptr::null()
        } else {
            password.as_ptr()
        };
        let res = unsafe { zip7_sys::open_archive(path.as_ptr(), password_ptr, &mut handle) };
        if res != 0 {
            return Err(res.into());
        }
        let handle = NonNull::new(handle).ok_or(Zip7Error::NullHandle)?;

        let item_count = unsafe { zip7_sys::items_count(handle.as_ptr()) } as usize;

        Ok(Self {
            handle: Arc::new(Mutex::new(Handle(handle))),
            item_index: 0,
            item_count,
        })
    }

    pub fn extract(&mut self) -> Result<Vec<Zip7Item>> {
        let code = {
            let handle = self.handle.lock().unwrap();
            let handle_ptr = handle.0.as_ptr();
            unsafe { zip7_sys::extract(handle_ptr) }
        };
        if code != 0 {
            return Err(code.into());
        }
        self.item_index = 0;
        Ok(self.into_iter().filter_map(Into::into).collect())
    }

    pub fn len(&self) -> usize {
        self.item_count
    }

    pub fn is_empty(&self) -> bool {
        self.item_count == 0
    }
}

impl Iterator for &mut Zip7Archive {
    type Item = Zip7Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.item_index >= self.item_count {
            return None;
        }
        let index = self.item_index;
        self.item_index += 1;

        Some(Zip7Item::new(Arc::clone(&self.handle), index))
    }
}
