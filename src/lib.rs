use std::ffi::CString;
use std::path::Path;
use std::ptr::{self, NonNull};
use std::sync::{Arc, Mutex, OnceLock};

use crate::error::{Result, Zip7Error};

mod error;

static INIT: OnceLock<()> = OnceLock::new();

#[derive(Debug)]
pub struct Handle(NonNull<zip7_sys::Handle>);

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
}
