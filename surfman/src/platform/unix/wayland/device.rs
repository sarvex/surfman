// surfman/surfman/src/platform/unix/wayland/device.rs
//
//! A wrapper around Wayland `EGLDisplay`s.

use crate::{Error, GLApi};
use super::connection::{Connection, NativeConnection};

pub use crate::platform::unix::generic::device::Adapter;

/// A thread-local handle to a device.
///
/// Devices contain most of the relevant surface management methods.
pub struct Device {
    pub(crate) native_connection: Box<dyn NativeConnection>,
    pub(crate) adapter: Adapter,
}

impl Device {
    #[inline]
    pub(crate) fn new(connection: &Connection, adapter: &Adapter) -> Result<Device, Error> {
        Ok(Device {
            native_connection: connection.native_connection.retain(),
            adapter: (*adapter).clone(),
        })
    }

    /// Returns the display server connection that this device was created with.
    #[inline]
    pub fn connection(&self) -> Connection {
        Connection { native_connection: self.native_connection.retain() }
    }

    /// Returns the adapter that this device was created with.
    #[inline]
    pub fn adapter(&self) -> Adapter {
        self.adapter.clone()
    }

    /// Returns the OpenGL API flavor that this device supports (OpenGL or OpenGL ES).
    #[inline]
    pub fn gl_api(&self) -> GLApi {
        GLApi::GLES
    }
}
