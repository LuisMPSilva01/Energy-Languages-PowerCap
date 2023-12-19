
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2008-2011 Kristian Høgsberg
    Copyright © 2010-2011 Intel Corporation
    Copyright © 2012-2013 Collabora, Ltd.

    Permission is hereby granted, free of charge, to any person
    obtaining a copy of this software and associated documentation files
    (the "Software"), to deal in the Software without restriction,
    including without limitation the rights to use, copy, modify, merge,
    publish, distribute, sublicense, and/or sell copies of the Software,
    and to permit persons to whom the Software is furnished to do so,
    subject to the following conditions:

    The above copyright notice and this permission notice (including the
    next paragraph) shall be included in all copies or substantial
    portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
    NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
    BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
    ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
    CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

pub mod wl_display {
    //! core global object
    //!
    //! The core global object.  This is a special singleton object.  It
    //! is used for internal Wayland protocol features.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// global error values
    ///
    /// These errors are global and can be emitted in response to any
    /// server request.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// server couldn't find object
        InvalidObject = 0,
        /// method doesn't exist on the specified interface
        InvalidMethod = 1,
        /// server is out of memory
        NoMemory = 2,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidObject),
                1 => Some(Error::InvalidMethod),
                2 => Some(Error::NoMemory),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// asynchronous roundtrip
        ///
        /// The sync request asks the server to emit the 'done' event
        /// on the returned wl_callback object.  Since requests are
        /// handled in-order and events are delivered in-order, this can
        /// be used as a barrier to ensure all previous requests and the
        /// resulting events have been handled.
        /// 
        /// The object returned by this request will be destroyed by the
        /// compositor after the callback is fired and as such the client must not
        /// attempt to use it after that point.
        /// 
        /// The callback_data passed in the callback is the event serial.
        Sync {callback: Proxy<super::wl_callback::WlCallback>, },
        /// get global registry object
        ///
        /// This request creates a registry object that allows the client
        /// to list and bind the global objects available from the
        /// compositor.
        GetRegistry {registry: Proxy<super::wl_registry::WlRegistry>, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Sync { callback, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = callback.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                },
                Request::GetRegistry { registry, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = registry.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// fatal error event
        ///
        /// The error event is sent out when a fatal (non-recoverable)
        /// error has occurred.  The object_id argument is the object
        /// where the error occurred, most often in response to a request
        /// to that object.  The code identifies the error and is defined
        /// by the object interface.  As such, each interface defines its
        /// own set of error codes.  The message is a brief description
        /// of the error, for (debugging) convenience.
        Error {object_id: Proxy<AnonymousObject>, code: u32, message: String, },
        /// acknowledge object ID deletion
        ///
        /// This event is used internally by the object ID management
        /// logic.  When a client deletes an object, the server will send
        /// this event to acknowledge that it has seen the delete request.
        /// When the client receives this event, it will know that it can
        /// safely reuse the object ID.
        DeleteId {id: u32, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Error {
                        object_id: Proxy::<AnonymousObject>::from_c_ptr(_args[0].o as *mut _),
                        code: _args[1].u,
                        message: ::std::ffi::CStr::from_ptr(_args[2].s).to_string_lossy().into_owned(),
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::DeleteId {
                        id: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlDisplay;

    impl Interface for WlDisplay {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_display";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_display_interface }
        }
    }

    pub trait RequestsTrait {
        /// asynchronous roundtrip
        ///
        /// The sync request asks the server to emit the 'done' event
        /// on the returned wl_callback object.  Since requests are
        /// handled in-order and events are delivered in-order, this can
        /// be used as a barrier to ensure all previous requests and the
        /// resulting events have been handled.
        /// 
        /// The object returned by this request will be destroyed by the
        /// compositor after the callback is fired and as such the client must not
        /// attempt to use it after that point.
        /// 
        /// The callback_data passed in the callback is the event serial.
        fn sync(&self) ->Result<NewProxy<super::wl_callback::WlCallback>, ()>;
        /// get global registry object
        ///
        /// This request creates a registry object that allows the client
        /// to list and bind the global objects available from the
        /// compositor.
        fn get_registry(&self) ->Result<NewProxy<super::wl_registry::WlRegistry>, ()>;
    }

    impl RequestsTrait for Proxy<WlDisplay> {
        fn sync(&self) ->Result<NewProxy<super::wl_callback::WlCallback>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_callback_newproxy = self.child::<super::wl_callback::WlCallback>();
            let msg = Request::Sync {
                callback: unsafe { Proxy::<super::wl_callback::WlCallback>::from_c_ptr(_arg_callback_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_callback_newproxy)
        }

        fn get_registry(&self) ->Result<NewProxy<super::wl_registry::WlRegistry>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_registry_newproxy = self.child::<super::wl_registry::WlRegistry>();
            let msg = Request::GetRegistry {
                registry: unsafe { Proxy::<super::wl_registry::WlRegistry>::from_c_ptr(_arg_registry_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_registry_newproxy)
        }

    }
}

pub mod wl_registry {
    //! global registry object
    //!
    //! The global registry object.  The server has a number of global
    //! objects that are available to all clients.  These objects
    //! typically represent an actual object in the server (for example,
    //! an input device) or they are singleton objects that provide
    //! extension functionality.
    //! 
    //! When a client creates a registry object, the registry object
    //! will emit a global event for each global currently in the
    //! registry.  Globals come and go as a result of device or
    //! monitor hotplugs, reconfiguration or other events, and the
    //! registry will send out global and global_remove events to
    //! keep the client up to date with the changes.  To mark the end
    //! of the initial burst of events, the client can use the
    //! wl_display.sync request immediately after calling
    //! wl_display.get_registry.
    //! 
    //! A client can bind to a global object by using the bind
    //! request.  This creates a client-side handle that lets the object
    //! emit events to the client and lets the client invoke requests on
    //! the object.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// bind an object to the display
        ///
        /// Binds a new, client-created object to the server using the
        /// specified name as the identifier.
        Bind {name: u32, id: (String, u32, Proxy<AnonymousObject>), },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Bind { name, id, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = name;
                    let _arg_1_s = ::std::ffi::CString::new(id.0).unwrap();
                    _args_array[1].s = _arg_1_s.as_ptr();
                    _args_array[2].u = id.1;
                    _args_array[3].o = ::std::ptr::null_mut();
                    f(0, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// announce global object
        ///
        /// Notify the client of global objects.
        /// 
        /// The event notifies the client that a global object with
        /// the given name is now available, and it implements the
        /// given version of the given interface.
        Global {name: u32, interface: String, version: u32, },
        /// announce removal of global object
        ///
        /// Notify the client of removed global objects.
        /// 
        /// This event notifies the client that the global identified
        /// by name is no longer available.  If the client bound to
        /// the global using the bind request, the client should now
        /// destroy that object.
        /// 
        /// The object remains valid and requests to the object will be
        /// ignored until the client destroys it, to avoid races between
        /// the global going away and a client sending a request to it.
        GlobalRemove {name: u32, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Global {
                        name: _args[0].u,
                        interface: ::std::ffi::CStr::from_ptr(_args[1].s).to_string_lossy().into_owned(),
                        version: _args[2].u,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::GlobalRemove {
                        name: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlRegistry;

    impl Interface for WlRegistry {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_registry";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_registry_interface }
        }
    }

    pub trait RequestsTrait {
        /// bind an object to the display
        ///
        /// Binds a new, client-created object to the server using the
        /// specified name as the identifier.
        fn bind<T: Interface>(&self, version: u32, name: u32) ->Result<NewProxy<T>, ()>;
    }

    impl RequestsTrait for Proxy<WlRegistry> {
        fn bind<T: Interface>(&self, version: u32, name: u32) ->Result<NewProxy<T>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let msg = Request::Bind {
                name: name,
                id: (T::NAME.into(), version, unsafe { Proxy::<AnonymousObject>::new_null() }),
            };

            unsafe {
                let ret = msg.as_raw_c_in(|opcode, args| {
                    ffi_dispatch!(
                        WAYLAND_CLIENT_HANDLE,
                        wl_proxy_marshal_array_constructor_versioned,
                        self.c_ptr(),
                        opcode,
                        args.as_mut_ptr(),
                        T::c_interface(),
                        version
                    )
                });
                Ok(NewProxy::<T>::from_c_ptr(ret))
            }
        }

    }
}

pub mod wl_callback {
    //! callback object
    //!
    //! Clients can handle the 'done' event to get notified when
    //! the related request is done.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
            }
        }
    }

    pub enum Event {
        /// done event
        ///
        /// Notify the client when the related request is done.
        ///
        /// This is a destructor, once received this object cannot be used any longer.
        Done {callback_data: u32, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                Event::Done { .. } => true,
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Done {
                        callback_data: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlCallback;

    impl Interface for WlCallback {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_callback";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_callback_interface }
        }
    }

    pub trait RequestsTrait {
    }

    impl RequestsTrait for Proxy<WlCallback> {
    }
}

pub mod wl_compositor {
    //! the compositor singleton
    //!
    //! A compositor.  This object is a singleton global.  The
    //! compositor is in charge of combining the contents of multiple
    //! surfaces into one displayable output.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// create new surface
        ///
        /// Ask the compositor to create a new surface.
        CreateSurface {id: Proxy<super::wl_surface::WlSurface>, },
        /// create new region
        ///
        /// Ask the compositor to create a new region.
        CreateRegion {id: Proxy<super::wl_region::WlRegion>, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::CreateSurface { id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                },
                Request::CreateRegion { id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlCompositor;

    impl Interface for WlCompositor {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_compositor";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_compositor_interface }
        }
    }

    pub trait RequestsTrait {
        /// create new surface
        ///
        /// Ask the compositor to create a new surface.
        fn create_surface(&self) ->Result<NewProxy<super::wl_surface::WlSurface>, ()>;
        /// create new region
        ///
        /// Ask the compositor to create a new region.
        fn create_region(&self) ->Result<NewProxy<super::wl_region::WlRegion>, ()>;
    }

    impl RequestsTrait for Proxy<WlCompositor> {
        fn create_surface(&self) ->Result<NewProxy<super::wl_surface::WlSurface>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_surface::WlSurface>();
            let msg = Request::CreateSurface {
                id: unsafe { Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn create_region(&self) ->Result<NewProxy<super::wl_region::WlRegion>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_region::WlRegion>();
            let msg = Request::CreateRegion {
                id: unsafe { Proxy::<super::wl_region::WlRegion>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod wl_shm_pool {
    //! a shared memory pool
    //!
    //! The wl_shm_pool object encapsulates a piece of memory shared
    //! between the compositor and client.  Through the wl_shm_pool
    //! object, the client can allocate shared memory wl_buffer objects.
    //! All objects created through the same pool share the same
    //! underlying mapped memory. Reusing the mapped memory avoids the
    //! setup/teardown overhead and is useful when interactively resizing
    //! a surface or for many small buffers.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// create a buffer from the pool
        ///
        /// Create a wl_buffer object from the pool.
        /// 
        /// The buffer is created offset bytes into the pool and has
        /// width and height as specified.  The stride argument specifies
        /// the number of bytes from the beginning of one row to the beginning
        /// of the next.  The format is the pixel format of the buffer and
        /// must be one of those advertised through the wl_shm.format event.
        /// 
        /// A buffer will keep a reference to the pool it was created from
        /// so it is valid to destroy the pool immediately after creating
        /// a buffer from it.
        CreateBuffer {id: Proxy<super::wl_buffer::WlBuffer>, offset: i32, width: i32, height: i32, stride: i32, format: super::wl_shm::Format, },
        /// destroy the pool
        ///
        /// Destroy the shared memory pool.
        /// 
        /// The mmapped memory will be released when all
        /// buffers that have been created from this pool
        /// are gone.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// change the size of the pool mapping
        ///
        /// This request will cause the server to remap the backing memory
        /// for the pool from the file descriptor passed when the pool was
        /// created, but using the new size.  This request can only be
        /// used to make the pool bigger.
        Resize {size: i32, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::CreateBuffer { id, offset, width, height, stride, format, } => {
                    let mut _args_array: [wl_argument; 6] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].i = offset;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    _args_array[4].i = stride;
                    _args_array[5].u = format.to_raw();
                    f(0, &mut _args_array)
                },
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                },
                Request::Resize { size, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = size;
                    f(2, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlShmPool;

    impl Interface for WlShmPool {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_shm_pool";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_shm_pool_interface }
        }
    }

    pub trait RequestsTrait {
        /// create a buffer from the pool
        ///
        /// Create a wl_buffer object from the pool.
        /// 
        /// The buffer is created offset bytes into the pool and has
        /// width and height as specified.  The stride argument specifies
        /// the number of bytes from the beginning of one row to the beginning
        /// of the next.  The format is the pixel format of the buffer and
        /// must be one of those advertised through the wl_shm.format event.
        /// 
        /// A buffer will keep a reference to the pool it was created from
        /// so it is valid to destroy the pool immediately after creating
        /// a buffer from it.
        fn create_buffer(&self, offset: i32, width: i32, height: i32, stride: i32, format: super::wl_shm::Format) ->Result<NewProxy<super::wl_buffer::WlBuffer>, ()>;
        /// destroy the pool
        ///
        /// Destroy the shared memory pool.
        /// 
        /// The mmapped memory will be released when all
        /// buffers that have been created from this pool
        /// are gone.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// change the size of the pool mapping
        ///
        /// This request will cause the server to remap the backing memory
        /// for the pool from the file descriptor passed when the pool was
        /// created, but using the new size.  This request can only be
        /// used to make the pool bigger.
        fn resize(&self, size: i32) ->();
    }

    impl RequestsTrait for Proxy<WlShmPool> {
        fn create_buffer(&self, offset: i32, width: i32, height: i32, stride: i32, format: super::wl_shm::Format) ->Result<NewProxy<super::wl_buffer::WlBuffer>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_buffer::WlBuffer>();
            let msg = Request::CreateBuffer {
                id: unsafe { Proxy::<super::wl_buffer::WlBuffer>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                offset: offset,
                width: width,
                height: height,
                stride: stride,
                format: format,
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn resize(&self, size: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Resize {
                size: size,
            };
            self.send(msg);
        }

    }
}

pub mod wl_shm {
    //! shared memory support
    //!
    //! A global singleton object that provides support for shared
    //! memory.
    //! 
    //! Clients can create wl_shm_pool objects using the create_pool
    //! request.
    //! 
    //! At connection setup time, the wl_shm object emits one or more
    //! format events to inform clients about the valid pixel formats
    //! that can be used for buffers.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// wl_shm error values
    ///
    /// These errors can be emitted in response to wl_shm requests.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// buffer format is not known
        InvalidFormat = 0,
        /// invalid size or stride during pool or buffer creation
        InvalidStride = 1,
        /// mmapping the file descriptor failed
        InvalidFd = 2,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidFormat),
                1 => Some(Error::InvalidStride),
                2 => Some(Error::InvalidFd),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// pixel formats
    ///
    /// This describes the memory layout of an individual pixel.
    /// 
    /// All renderers should support argb8888 and xrgb8888 but any other
    /// formats are optional and may not be supported by the particular
    /// renderer in use.
    /// 
    /// The drm format codes match the #defines in drm_fourcc.h.
    /// The formats actually supported by the compositor will be
    /// reported by the format event.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Format {
        /// 32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian
        Argb8888 = 0,
        /// 32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian
        Xrgb8888 = 1,
        /// 8-bit color index format, [7:0] C
        C8 = 0x20203843,
        /// 8-bit RGB format, [7:0] R:G:B 3:3:2
        Rgb332 = 0x38424752,
        /// 8-bit BGR format, [7:0] B:G:R 2:3:3
        Bgr233 = 0x38524742,
        /// 16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian
        Xrgb4444 = 0x32315258,
        /// 16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian
        Xbgr4444 = 0x32314258,
        /// 16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian
        Rgbx4444 = 0x32315852,
        /// 16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian
        Bgrx4444 = 0x32315842,
        /// 16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian
        Argb4444 = 0x32315241,
        /// 16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian
        Abgr4444 = 0x32314241,
        /// 16-bit RBGA format, [15:0] R:G:B:A 4:4:4:4 little endian
        Rgba4444 = 0x32314152,
        /// 16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian
        Bgra4444 = 0x32314142,
        /// 16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian
        Xrgb1555 = 0x35315258,
        /// 16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian
        Xbgr1555 = 0x35314258,
        /// 16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian
        Rgbx5551 = 0x35315852,
        /// 16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian
        Bgrx5551 = 0x35315842,
        /// 16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian
        Argb1555 = 0x35315241,
        /// 16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian
        Abgr1555 = 0x35314241,
        /// 16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian
        Rgba5551 = 0x35314152,
        /// 16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian
        Bgra5551 = 0x35314142,
        /// 16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian
        Rgb565 = 0x36314752,
        /// 16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian
        Bgr565 = 0x36314742,
        /// 24-bit RGB format, [23:0] R:G:B little endian
        Rgb888 = 0x34324752,
        /// 24-bit BGR format, [23:0] B:G:R little endian
        Bgr888 = 0x34324742,
        /// 32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian
        Xbgr8888 = 0x34324258,
        /// 32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian
        Rgbx8888 = 0x34325852,
        /// 32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian
        Bgrx8888 = 0x34325842,
        /// 32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian
        Abgr8888 = 0x34324241,
        /// 32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian
        Rgba8888 = 0x34324152,
        /// 32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian
        Bgra8888 = 0x34324142,
        /// 32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian
        Xrgb2101010 = 0x30335258,
        /// 32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian
        Xbgr2101010 = 0x30334258,
        /// 32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian
        Rgbx1010102 = 0x30335852,
        /// 32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian
        Bgrx1010102 = 0x30335842,
        /// 32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian
        Argb2101010 = 0x30335241,
        /// 32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian
        Abgr2101010 = 0x30334241,
        /// 32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian
        Rgba1010102 = 0x30334152,
        /// 32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian
        Bgra1010102 = 0x30334142,
        /// packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian
        Yuyv = 0x56595559,
        /// packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian
        Yvyu = 0x55595659,
        /// packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian
        Uyvy = 0x59565955,
        /// packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian
        Vyuy = 0x59555956,
        /// packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian
        Ayuv = 0x56555941,
        /// 2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane
        Nv12 = 0x3231564e,
        /// 2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane
        Nv21 = 0x3132564e,
        /// 2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane
        Nv16 = 0x3631564e,
        /// 2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane
        Nv61 = 0x3136564e,
        /// 3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes
        Yuv410 = 0x39565559,
        /// 3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes
        Yvu410 = 0x39555659,
        /// 3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes
        Yuv411 = 0x31315559,
        /// 3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes
        Yvu411 = 0x31315659,
        /// 3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes
        Yuv420 = 0x32315559,
        /// 3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes
        Yvu420 = 0x32315659,
        /// 3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes
        Yuv422 = 0x36315559,
        /// 3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes
        Yvu422 = 0x36315659,
        /// 3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes
        Yuv444 = 0x34325559,
        /// 3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes
        Yvu444 = 0x34325659,
    }
    impl Format {
        pub fn from_raw(n: u32) -> Option<Format> {
            match n {
                0 => Some(Format::Argb8888),
                1 => Some(Format::Xrgb8888),
                0x20203843 => Some(Format::C8),
                0x38424752 => Some(Format::Rgb332),
                0x38524742 => Some(Format::Bgr233),
                0x32315258 => Some(Format::Xrgb4444),
                0x32314258 => Some(Format::Xbgr4444),
                0x32315852 => Some(Format::Rgbx4444),
                0x32315842 => Some(Format::Bgrx4444),
                0x32315241 => Some(Format::Argb4444),
                0x32314241 => Some(Format::Abgr4444),
                0x32314152 => Some(Format::Rgba4444),
                0x32314142 => Some(Format::Bgra4444),
                0x35315258 => Some(Format::Xrgb1555),
                0x35314258 => Some(Format::Xbgr1555),
                0x35315852 => Some(Format::Rgbx5551),
                0x35315842 => Some(Format::Bgrx5551),
                0x35315241 => Some(Format::Argb1555),
                0x35314241 => Some(Format::Abgr1555),
                0x35314152 => Some(Format::Rgba5551),
                0x35314142 => Some(Format::Bgra5551),
                0x36314752 => Some(Format::Rgb565),
                0x36314742 => Some(Format::Bgr565),
                0x34324752 => Some(Format::Rgb888),
                0x34324742 => Some(Format::Bgr888),
                0x34324258 => Some(Format::Xbgr8888),
                0x34325852 => Some(Format::Rgbx8888),
                0x34325842 => Some(Format::Bgrx8888),
                0x34324241 => Some(Format::Abgr8888),
                0x34324152 => Some(Format::Rgba8888),
                0x34324142 => Some(Format::Bgra8888),
                0x30335258 => Some(Format::Xrgb2101010),
                0x30334258 => Some(Format::Xbgr2101010),
                0x30335852 => Some(Format::Rgbx1010102),
                0x30335842 => Some(Format::Bgrx1010102),
                0x30335241 => Some(Format::Argb2101010),
                0x30334241 => Some(Format::Abgr2101010),
                0x30334152 => Some(Format::Rgba1010102),
                0x30334142 => Some(Format::Bgra1010102),
                0x56595559 => Some(Format::Yuyv),
                0x55595659 => Some(Format::Yvyu),
                0x59565955 => Some(Format::Uyvy),
                0x59555956 => Some(Format::Vyuy),
                0x56555941 => Some(Format::Ayuv),
                0x3231564e => Some(Format::Nv12),
                0x3132564e => Some(Format::Nv21),
                0x3631564e => Some(Format::Nv16),
                0x3136564e => Some(Format::Nv61),
                0x39565559 => Some(Format::Yuv410),
                0x39555659 => Some(Format::Yvu410),
                0x31315559 => Some(Format::Yuv411),
                0x31315659 => Some(Format::Yvu411),
                0x32315559 => Some(Format::Yuv420),
                0x32315659 => Some(Format::Yvu420),
                0x36315559 => Some(Format::Yuv422),
                0x36315659 => Some(Format::Yvu422),
                0x34325559 => Some(Format::Yuv444),
                0x34325659 => Some(Format::Yvu444),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// create a shm pool
        ///
        /// Create a new wl_shm_pool object.
        /// 
        /// The pool can be used to create shared memory based buffer
        /// objects.  The server will mmap size bytes of the passed file
        /// descriptor, to use as backing memory for the pool.
        CreatePool {id: Proxy<super::wl_shm_pool::WlShmPool>, fd: ::std::os::unix::io::RawFd, size: i32, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::CreatePool { id, fd, size, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].h = fd;
                    _args_array[2].i = size;
                    f(0, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// pixel format description
        ///
        /// Informs the client about a valid pixel format that
        /// can be used for buffers. Known formats include
        /// argb8888 and xrgb8888.
        Format {format: Format, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Format {
                        format: Format::from_raw(_args[0].u).ok_or(())?,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlShm;

    impl Interface for WlShm {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_shm";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_shm_interface }
        }
    }

    pub trait RequestsTrait {
        /// create a shm pool
        ///
        /// Create a new wl_shm_pool object.
        /// 
        /// The pool can be used to create shared memory based buffer
        /// objects.  The server will mmap size bytes of the passed file
        /// descriptor, to use as backing memory for the pool.
        fn create_pool(&self, fd: ::std::os::unix::io::RawFd, size: i32) ->Result<NewProxy<super::wl_shm_pool::WlShmPool>, ()>;
    }

    impl RequestsTrait for Proxy<WlShm> {
        fn create_pool(&self, fd: ::std::os::unix::io::RawFd, size: i32) ->Result<NewProxy<super::wl_shm_pool::WlShmPool>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_shm_pool::WlShmPool>();
            let msg = Request::CreatePool {
                id: unsafe { Proxy::<super::wl_shm_pool::WlShmPool>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                fd: fd,
                size: size,
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod wl_buffer {
    //! content for a wl_surface
    //!
    //! A buffer provides the content for a wl_surface. Buffers are
    //! created through factory interfaces such as wl_drm, wl_shm or
    //! similar. It has a width and a height and can be attached to a
    //! wl_surface, but the mechanism by which a client provides and
    //! updates the contents is defined by the buffer factory interface.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy a buffer
        ///
        /// Destroy a buffer. If and how you need to release the backing
        /// storage is defined by the buffer factory interface.
        /// 
        /// For possible side-effects to a surface, see wl_surface.attach.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// compositor releases buffer
        ///
        /// Sent when this wl_buffer is no longer used by the compositor.
        /// The client is now free to reuse or destroy this buffer and its
        /// backing storage.
        /// 
        /// If a client receives a release event before the frame callback
        /// requested in the same wl_surface.commit that attaches this
        /// wl_buffer to a surface, then the client is immediately free to
        /// reuse the buffer and its backing storage, and does not need a
        /// second buffer for the next surface content update. Typically
        /// this is possible, when the compositor maintains a copy of the
        /// wl_surface contents, e.g. as a GL texture. This is an important
        /// optimization for GL(ES) compositors with wl_shm clients.
        Release,
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    Ok(Event::Release) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlBuffer;

    impl Interface for WlBuffer {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_buffer";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_buffer_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy a buffer
        ///
        /// Destroy a buffer. If and how you need to release the backing
        /// storage is defined by the buffer factory interface.
        /// 
        /// For possible side-effects to a surface, see wl_surface.attach.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<WlBuffer> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

pub mod wl_data_offer {
    //! offer to transfer data
    //!
    //! A wl_data_offer represents a piece of data offered for transfer
    //! by another client (the source client).  It is used by the
    //! copy-and-paste and drag-and-drop mechanisms.  The offer
    //! describes the different mime types that the data can be
    //! converted to and provides the mechanism for transferring the
    //! data directly from the source client.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// finish request was called untimely
        InvalidFinish = 0,
        /// action mask contains invalid values
        InvalidActionMask = 1,
        /// action argument has an invalid value
        InvalidAction = 2,
        /// offer doesn't accept this request
        InvalidOffer = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidFinish),
                1 => Some(Error::InvalidActionMask),
                2 => Some(Error::InvalidAction),
                3 => Some(Error::InvalidOffer),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// accept one of the offered mime types
        ///
        /// Indicate that the client can accept the given mime type, or
        /// NULL for not accepted.
        /// 
        /// For objects of version 2 or older, this request is used by the
        /// client to give feedback whether the client can receive the given
        /// mime type, or NULL if none is accepted; the feedback does not
        /// determine whether the drag-and-drop operation succeeds or not.
        /// 
        /// For objects of version 3 or newer, this request determines the
        /// final result of the drag-and-drop operation. If the end result
        /// is that no mime types were accepted, the drag-and-drop operation
        /// will be cancelled and the corresponding drag source will receive
        /// wl_data_source.cancelled. Clients may still use this event in
        /// conjunction with wl_data_source.action for feedback.
        Accept {serial: u32, mime_type: Option<String>, },
        /// request that the data is transferred
        ///
        /// To transfer the offered data, the client issues this request
        /// and indicates the mime type it wants to receive.  The transfer
        /// happens through the passed file descriptor (typically created
        /// with the pipe system call).  The source client writes the data
        /// in the mime type representation requested and then closes the
        /// file descriptor.
        /// 
        /// The receiving client reads from the read end of the pipe until
        /// EOF and then closes its end, at which point the transfer is
        /// complete.
        /// 
        /// This request may happen multiple times for different mime types,
        /// both before and after wl_data_device.drop. Drag-and-drop destination
        /// clients may preemptively fetch data or examine it more closely to
        /// determine acceptance.
        Receive {mime_type: String, fd: ::std::os::unix::io::RawFd, },
        /// destroy data offer
        ///
        /// Destroy the data offer.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// the offer will no longer be used
        ///
        /// Notifies the compositor that the drag destination successfully
        /// finished the drag-and-drop operation.
        /// 
        /// Upon receiving this request, the compositor will emit
        /// wl_data_source.dnd_finished on the drag source client.
        /// 
        /// It is a client error to perform other requests than
        /// wl_data_offer.destroy after this one. It is also an error to perform
        /// this request after a NULL mime type has been set in
        /// wl_data_offer.accept or no action was received through
        /// wl_data_offer.action.
        ///
        /// Only available since version 3 of the interface
        Finish,
        /// set the available/preferred drag-and-drop actions
        ///
        /// Sets the actions that the destination side client supports for
        /// this operation. This request may trigger the emission of
        /// wl_data_source.action and wl_data_offer.action events if the compositor
        /// needs to change the selected action.
        /// 
        /// This request can be called multiple times throughout the
        /// drag-and-drop operation, typically in response to wl_data_device.enter
        /// or wl_data_device.motion events.
        /// 
        /// This request determines the final result of the drag-and-drop
        /// operation. If the end result is that no action is accepted,
        /// the drag source will receive wl_drag_source.cancelled.
        /// 
        /// The dnd_actions argument must contain only values expressed in the
        /// wl_data_device_manager.dnd_actions enum, and the preferred_action
        /// argument must only contain one of those values set, otherwise it
        /// will result in a protocol error.
        /// 
        /// While managing an "ask" action, the destination drag-and-drop client
        /// may perform further wl_data_offer.receive requests, and is expected
        /// to perform one last wl_data_offer.set_actions request with a preferred
        /// action other than "ask" (and optionally wl_data_offer.accept) before
        /// requesting wl_data_offer.finish, in order to convey the action selected
        /// by the user. If the preferred action is not in the
        /// wl_data_offer.source_actions mask, an error will be raised.
        /// 
        /// If the "ask" action is dismissed (e.g. user cancellation), the client
        /// is expected to perform wl_data_offer.destroy right away.
        /// 
        /// This request can only be made on drag-and-drop offers, a protocol error
        /// will be raised otherwise.
        ///
        /// Only available since version 3 of the interface
        SetActions {dnd_actions: u32, preferred_action: u32, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Accept { serial, mime_type, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    let _arg_1 = mime_type.map(|s| ::std::ffi::CString::new(s).unwrap());
                    _args_array[1].s = _arg_1.map(|s| s.as_ptr()).unwrap_or(::std::ptr::null());
                    f(0, &mut _args_array)
                },
                Request::Receive { mime_type, fd, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(mime_type).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].h = fd;
                    f(1, &mut _args_array)
                },
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                },
                Request::Finish => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                },
                Request::SetActions { dnd_actions, preferred_action, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = dnd_actions;
                    _args_array[1].u = preferred_action;
                    f(4, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// advertise offered mime type
        ///
        /// Sent immediately after creating the wl_data_offer object.  One
        /// event per offered mime type.
        Offer {mime_type: String, },
        /// notify the source-side available actions
        ///
        /// This event indicates the actions offered by the data source. It
        /// will be sent right after wl_data_device.enter, or anytime the source
        /// side changes its offered actions through wl_data_source.set_actions.
        ///
        /// Only available since version 3 of the interface
        SourceActions {source_actions: u32, },
        /// notify the selected action
        ///
        /// This event indicates the action selected by the compositor after
        /// matching the source/destination side actions. Only one action (or
        /// none) will be offered here.
        /// 
        /// This event can be emitted multiple times during the drag-and-drop
        /// operation in response to destination side action changes through
        /// wl_data_offer.set_actions.
        /// 
        /// This event will no longer be emitted after wl_data_device.drop
        /// happened on the drag-and-drop destination, the client must
        /// honor the last action received, or the last preferred one set
        /// through wl_data_offer.set_actions when handling an "ask" action.
        /// 
        /// Compositors may also change the selected action on the fly, mainly
        /// in response to keyboard modifier changes during the drag-and-drop
        /// operation.
        /// 
        /// The most recent action received is always the valid one. Prior to
        /// receiving wl_data_device.drop, the chosen action may change (e.g.
        /// due to keyboard modifiers being pressed). At the time of receiving
        /// wl_data_device.drop the drag-and-drop destination must honor the
        /// last action received.
        /// 
        /// Action changes may still happen after wl_data_device.drop,
        /// especially on "ask" actions, where the drag-and-drop destination
        /// may choose another action afterwards. Action changes happening
        /// at this stage are always the result of inter-client negotiation, the
        /// compositor shall no longer be able to induce a different action.
        /// 
        /// Upon "ask" actions, it is expected that the drag-and-drop destination
        /// may potentially choose a different action and/or mime type,
        /// based on wl_data_offer.source_actions and finally chosen by the
        /// user (e.g. popping up a menu with the available options). The
        /// final wl_data_offer.set_actions and wl_data_offer.accept requests
        /// must happen before the call to wl_data_offer.finish.
        ///
        /// Only available since version 3 of the interface
        Action {dnd_action: u32, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Offer {
                        mime_type: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::SourceActions {
                        source_actions: _args[0].u,
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Action {
                        dnd_action: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlDataOffer;

    impl Interface for WlDataOffer {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_data_offer";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_data_offer_interface }
        }
    }

    pub trait RequestsTrait {
        /// accept one of the offered mime types
        ///
        /// Indicate that the client can accept the given mime type, or
        /// NULL for not accepted.
        /// 
        /// For objects of version 2 or older, this request is used by the
        /// client to give feedback whether the client can receive the given
        /// mime type, or NULL if none is accepted; the feedback does not
        /// determine whether the drag-and-drop operation succeeds or not.
        /// 
        /// For objects of version 3 or newer, this request determines the
        /// final result of the drag-and-drop operation. If the end result
        /// is that no mime types were accepted, the drag-and-drop operation
        /// will be cancelled and the corresponding drag source will receive
        /// wl_data_source.cancelled. Clients may still use this event in
        /// conjunction with wl_data_source.action for feedback.
        fn accept(&self, serial: u32, mime_type: Option<String>) ->();
        /// request that the data is transferred
        ///
        /// To transfer the offered data, the client issues this request
        /// and indicates the mime type it wants to receive.  The transfer
        /// happens through the passed file descriptor (typically created
        /// with the pipe system call).  The source client writes the data
        /// in the mime type representation requested and then closes the
        /// file descriptor.
        /// 
        /// The receiving client reads from the read end of the pipe until
        /// EOF and then closes its end, at which point the transfer is
        /// complete.
        /// 
        /// This request may happen multiple times for different mime types,
        /// both before and after wl_data_device.drop. Drag-and-drop destination
        /// clients may preemptively fetch data or examine it more closely to
        /// determine acceptance.
        fn receive(&self, mime_type: String, fd: ::std::os::unix::io::RawFd) ->();
        /// destroy data offer
        ///
        /// Destroy the data offer.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// the offer will no longer be used
        ///
        /// Notifies the compositor that the drag destination successfully
        /// finished the drag-and-drop operation.
        /// 
        /// Upon receiving this request, the compositor will emit
        /// wl_data_source.dnd_finished on the drag source client.
        /// 
        /// It is a client error to perform other requests than
        /// wl_data_offer.destroy after this one. It is also an error to perform
        /// this request after a NULL mime type has been set in
        /// wl_data_offer.accept or no action was received through
        /// wl_data_offer.action.
        ///
        /// Only available since version 3 of the interface
        fn finish(&self) ->();
        /// set the available/preferred drag-and-drop actions
        ///
        /// Sets the actions that the destination side client supports for
        /// this operation. This request may trigger the emission of
        /// wl_data_source.action and wl_data_offer.action events if the compositor
        /// needs to change the selected action.
        /// 
        /// This request can be called multiple times throughout the
        /// drag-and-drop operation, typically in response to wl_data_device.enter
        /// or wl_data_device.motion events.
        /// 
        /// This request determines the final result of the drag-and-drop
        /// operation. If the end result is that no action is accepted,
        /// the drag source will receive wl_drag_source.cancelled.
        /// 
        /// The dnd_actions argument must contain only values expressed in the
        /// wl_data_device_manager.dnd_actions enum, and the preferred_action
        /// argument must only contain one of those values set, otherwise it
        /// will result in a protocol error.
        /// 
        /// While managing an "ask" action, the destination drag-and-drop client
        /// may perform further wl_data_offer.receive requests, and is expected
        /// to perform one last wl_data_offer.set_actions request with a preferred
        /// action other than "ask" (and optionally wl_data_offer.accept) before
        /// requesting wl_data_offer.finish, in order to convey the action selected
        /// by the user. If the preferred action is not in the
        /// wl_data_offer.source_actions mask, an error will be raised.
        /// 
        /// If the "ask" action is dismissed (e.g. user cancellation), the client
        /// is expected to perform wl_data_offer.destroy right away.
        /// 
        /// This request can only be made on drag-and-drop offers, a protocol error
        /// will be raised otherwise.
        ///
        /// Only available since version 3 of the interface
        fn set_actions(&self, dnd_actions: u32, preferred_action: u32) ->();
    }

    impl RequestsTrait for Proxy<WlDataOffer> {
        fn accept(&self, serial: u32, mime_type: Option<String>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Accept {
                serial: serial,
                mime_type: mime_type,
            };
            self.send(msg);
        }

        fn receive(&self, mime_type: String, fd: ::std::os::unix::io::RawFd) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Receive {
                mime_type: mime_type,
                fd: fd,
            };
            self.send(msg);
        }

        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn finish(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Finish;
            self.send(msg);
        }

        fn set_actions(&self, dnd_actions: u32, preferred_action: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetActions {
                dnd_actions: dnd_actions,
                preferred_action: preferred_action,
            };
            self.send(msg);
        }

    }
}

pub mod wl_data_source {
    //! offer to transfer data
    //!
    //! The wl_data_source object is the source side of a wl_data_offer.
    //! It is created by the source client in a data transfer and
    //! provides a way to describe the offered data and a way to respond
    //! to requests to transfer the data.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// action mask contains invalid values
        InvalidActionMask = 0,
        /// source doesn't accept this request
        InvalidSource = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidActionMask),
                1 => Some(Error::InvalidSource),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// add an offered mime type
        ///
        /// This request adds a mime type to the set of mime types
        /// advertised to targets.  Can be called several times to offer
        /// multiple types.
        Offer {mime_type: String, },
        /// destroy the data source
        ///
        /// Destroy the data source.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// set the available drag-and-drop actions
        ///
        /// Sets the actions that the source side client supports for this
        /// operation. This request may trigger wl_data_source.action and
        /// wl_data_offer.action events if the compositor needs to change the
        /// selected action.
        /// 
        /// The dnd_actions argument must contain only values expressed in the
        /// wl_data_device_manager.dnd_actions enum, otherwise it will result
        /// in a protocol error.
        /// 
        /// This request must be made once only, and can only be made on sources
        /// used in drag-and-drop, so it must be performed before
        /// wl_data_device.start_drag. Attempting to use the source other than
        /// for drag-and-drop will raise a protocol error.
        ///
        /// Only available since version 3 of the interface
        SetActions {dnd_actions: u32, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Offer { mime_type, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(mime_type).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(0, &mut _args_array)
                },
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                },
                Request::SetActions { dnd_actions, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = dnd_actions;
                    f(2, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// a target accepts an offered mime type
        ///
        /// Sent when a target accepts pointer_focus or motion events.  If
        /// a target does not accept any of the offered types, type is NULL.
        /// 
        /// Used for feedback during drag-and-drop.
        Target {mime_type: Option<String>, },
        /// send the data
        ///
        /// Request for data from the client.  Send the data as the
        /// specified mime type over the passed file descriptor, then
        /// close it.
        Send {mime_type: String, fd: ::std::os::unix::io::RawFd, },
        /// selection was cancelled
        ///
        /// This data source is no longer valid. There are several reasons why
        /// this could happen:
        /// 
        /// - The data source has been replaced by another data source.
        /// - The drag-and-drop operation was performed, but the drop destination
        /// did not accept any of the mime types offered through
        /// wl_data_source.target.
        /// - The drag-and-drop operation was performed, but the drop destination
        /// did not select any of the actions present in the mask offered through
        /// wl_data_source.action.
        /// - The drag-and-drop operation was performed but didn't happen over a
        /// surface.
        /// - The compositor cancelled the drag-and-drop operation (e.g. compositor
        /// dependent timeouts to avoid stale drag-and-drop transfers).
        /// 
        /// The client should clean up and destroy this data source.
        /// 
        /// For objects of version 2 or older, wl_data_source.cancelled will
        /// only be emitted if the data source was replaced by another data
        /// source.
        Cancelled,
        /// the drag-and-drop operation physically finished
        ///
        /// The user performed the drop action. This event does not indicate
        /// acceptance, wl_data_source.cancelled may still be emitted afterwards
        /// if the drop destination does not accept any mime type.
        /// 
        /// However, this event might however not be received if the compositor
        /// cancelled the drag-and-drop operation before this event could happen.
        /// 
        /// Note that the data_source may still be used in the future and should
        /// not be destroyed here.
        ///
        /// Only available since version 3 of the interface
        DndDropPerformed,
        /// the drag-and-drop operation concluded
        ///
        /// The drop destination finished interoperating with this data
        /// source, so the client is now free to destroy this data source and
        /// free all associated data.
        /// 
        /// If the action used to perform the operation was "move", the
        /// source can now delete the transferred data.
        ///
        /// Only available since version 3 of the interface
        DndFinished,
        /// notify the selected action
        ///
        /// This event indicates the action selected by the compositor after
        /// matching the source/destination side actions. Only one action (or
        /// none) will be offered here.
        /// 
        /// This event can be emitted multiple times during the drag-and-drop
        /// operation, mainly in response to destination side changes through
        /// wl_data_offer.set_actions, and as the data device enters/leaves
        /// surfaces.
        /// 
        /// It is only possible to receive this event after
        /// wl_data_source.dnd_drop_performed if the drag-and-drop operation
        /// ended in an "ask" action, in which case the final wl_data_source.action
        /// event will happen immediately before wl_data_source.dnd_finished.
        /// 
        /// Compositors may also change the selected action on the fly, mainly
        /// in response to keyboard modifier changes during the drag-and-drop
        /// operation.
        /// 
        /// The most recent action received is always the valid one. The chosen
        /// action may change alongside negotiation (e.g. an "ask" action can turn
        /// into a "move" operation), so the effects of the final action must
        /// always be applied in wl_data_offer.dnd_finished.
        /// 
        /// Clients can trigger cursor surface changes from this point, so
        /// they reflect the current action.
        ///
        /// Only available since version 3 of the interface
        Action {dnd_action: u32, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Target {
                        mime_type: if _args[0].s.is_null() { None } else { Some(::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned()) },
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Send {
                        mime_type: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                        fd: _args[1].h,
                }) },
                2 => {
                    Ok(Event::Cancelled) },
                3 => {
                    Ok(Event::DndDropPerformed) },
                4 => {
                    Ok(Event::DndFinished) },
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Action {
                        dnd_action: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlDataSource;

    impl Interface for WlDataSource {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_data_source";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_data_source_interface }
        }
    }

    pub trait RequestsTrait {
        /// add an offered mime type
        ///
        /// This request adds a mime type to the set of mime types
        /// advertised to targets.  Can be called several times to offer
        /// multiple types.
        fn offer(&self, mime_type: String) ->();
        /// destroy the data source
        ///
        /// Destroy the data source.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// set the available drag-and-drop actions
        ///
        /// Sets the actions that the source side client supports for this
        /// operation. This request may trigger wl_data_source.action and
        /// wl_data_offer.action events if the compositor needs to change the
        /// selected action.
        /// 
        /// The dnd_actions argument must contain only values expressed in the
        /// wl_data_device_manager.dnd_actions enum, otherwise it will result
        /// in a protocol error.
        /// 
        /// This request must be made once only, and can only be made on sources
        /// used in drag-and-drop, so it must be performed before
        /// wl_data_device.start_drag. Attempting to use the source other than
        /// for drag-and-drop will raise a protocol error.
        ///
        /// Only available since version 3 of the interface
        fn set_actions(&self, dnd_actions: u32) ->();
    }

    impl RequestsTrait for Proxy<WlDataSource> {
        fn offer(&self, mime_type: String) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Offer {
                mime_type: mime_type,
            };
            self.send(msg);
        }

        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn set_actions(&self, dnd_actions: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetActions {
                dnd_actions: dnd_actions,
            };
            self.send(msg);
        }

    }
}

pub mod wl_data_device {
    //! data transfer device
    //!
    //! There is one wl_data_device per seat which can be obtained
    //! from the global wl_data_device_manager singleton.
    //! 
    //! A wl_data_device provides access to inter-client data transfer
    //! mechanisms such as copy-and-paste and drag-and-drop.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// given wl_surface has another role
        Role = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// start drag-and-drop operation
        ///
        /// This request asks the compositor to start a drag-and-drop
        /// operation on behalf of the client.
        /// 
        /// The source argument is the data source that provides the data
        /// for the eventual data transfer. If source is NULL, enter, leave
        /// and motion events are sent only to the client that initiated the
        /// drag and the client is expected to handle the data passing
        /// internally.
        /// 
        /// The origin surface is the surface where the drag originates and
        /// the client must have an active implicit grab that matches the
        /// serial.
        /// 
        /// The icon surface is an optional (can be NULL) surface that
        /// provides an icon to be moved around with the cursor.  Initially,
        /// the top-left corner of the icon surface is placed at the cursor
        /// hotspot, but subsequent wl_surface.attach request can move the
        /// relative position. Attach requests must be confirmed with
        /// wl_surface.commit as usual. The icon surface is given the role of
        /// a drag-and-drop icon. If the icon surface already has another role,
        /// it raises a protocol error.
        /// 
        /// The current and pending input regions of the icon wl_surface are
        /// cleared, and wl_surface.set_input_region is ignored until the
        /// wl_surface is no longer used as the icon surface. When the use
        /// as an icon ends, the current and pending input regions become
        /// undefined, and the wl_surface is unmapped.
        StartDrag {source: Option<Proxy<super::wl_data_source::WlDataSource>>, origin: Proxy<super::wl_surface::WlSurface>, icon: Option<Proxy<super::wl_surface::WlSurface>>, serial: u32, },
        /// copy data to the selection
        ///
        /// This request asks the compositor to set the selection
        /// to the data from the source on behalf of the client.
        /// 
        /// To unset the selection, set the source to NULL.
        SetSelection {source: Option<Proxy<super::wl_data_source::WlDataSource>>, serial: u32, },
        /// destroy data device
        ///
        /// This request destroys the data device.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        ///
        /// Only available since version 2 of the interface
        Release,
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::StartDrag { source, origin, icon, serial, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = source.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[1].o = origin.c_ptr() as *mut _;
                    _args_array[2].o = icon.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[3].u = serial;
                    f(0, &mut _args_array)
                },
                Request::SetSelection { source, serial, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = source.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[1].u = serial;
                    f(1, &mut _args_array)
                },
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// introduce a new wl_data_offer
        ///
        /// The data_offer event introduces a new wl_data_offer object,
        /// which will subsequently be used in either the
        /// data_device.enter event (for drag-and-drop) or the
        /// data_device.selection event (for selections).  Immediately
        /// following the data_device_data_offer event, the new data_offer
        /// object will send out data_offer.offer events to describe the
        /// mime types it offers.
        DataOffer {id: NewProxy<super::wl_data_offer::WlDataOffer>, },
        /// initiate drag-and-drop session
        ///
        /// This event is sent when an active drag-and-drop pointer enters
        /// a surface owned by the client.  The position of the pointer at
        /// enter time is provided by the x and y arguments, in surface-local
        /// coordinates.
        Enter {serial: u32, surface: Proxy<super::wl_surface::WlSurface>, x: f64, y: f64, id: Option<Proxy<super::wl_data_offer::WlDataOffer>>, },
        /// end drag-and-drop session
        ///
        /// This event is sent when the drag-and-drop pointer leaves the
        /// surface and the session ends.  The client must destroy the
        /// wl_data_offer introduced at enter time at this point.
        Leave,
        /// drag-and-drop session motion
        ///
        /// This event is sent when the drag-and-drop pointer moves within
        /// the currently focused surface. The new position of the pointer
        /// is provided by the x and y arguments, in surface-local
        /// coordinates.
        Motion {time: u32, x: f64, y: f64, },
        /// end drag-and-drop session successfully
        ///
        /// The event is sent when a drag-and-drop operation is ended
        /// because the implicit grab is removed.
        /// 
        /// The drag-and-drop destination is expected to honor the last action
        /// received through wl_data_offer.action, if the resulting action is
        /// "copy" or "move", the destination can still perform
        /// wl_data_offer.receive requests, and is expected to end all
        /// transfers with a wl_data_offer.finish request.
        /// 
        /// If the resulting action is "ask", the action will not be considered
        /// final. The drag-and-drop destination is expected to perform one last
        /// wl_data_offer.set_actions request, or wl_data_offer.destroy in order
        /// to cancel the operation.
        Drop,
        /// advertise new selection
        ///
        /// The selection event is sent out to notify the client of a new
        /// wl_data_offer for the selection for this device.  The
        /// data_device.data_offer and the data_offer.offer events are
        /// sent out immediately before this event to introduce the data
        /// offer object.  The selection event is sent to a client
        /// immediately before receiving keyboard focus and when a new
        /// selection is set while the client has keyboard focus.  The
        /// data_offer is valid until a new data_offer or NULL is received
        /// or until the client loses keyboard focus.  The client must
        /// destroy the previous selection data_offer, if any, upon receiving
        /// this event.
        Selection {id: Option<Proxy<super::wl_data_offer::WlDataOffer>>, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::DataOffer {
                        id: NewProxy::<super::wl_data_offer::WlDataOffer>::from_c_ptr(_args[0].o as *mut _),
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 5);
                    Ok(Event::Enter {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[1].o as *mut _),
                        x: (_args[2].f as f64)/256.,
                        y: (_args[3].f as f64)/256.,
                        id: if _args[4].o.is_null() { None } else { Some(Proxy::<super::wl_data_offer::WlDataOffer>::from_c_ptr(_args[4].o as *mut _)) },
                }) },
                2 => {
                    Ok(Event::Leave) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Motion {
                        time: _args[0].u,
                        x: (_args[1].f as f64)/256.,
                        y: (_args[2].f as f64)/256.,
                }) },
                4 => {
                    Ok(Event::Drop) },
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Selection {
                        id: if _args[0].o.is_null() { None } else { Some(Proxy::<super::wl_data_offer::WlDataOffer>::from_c_ptr(_args[0].o as *mut _)) },
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlDataDevice;

    impl Interface for WlDataDevice {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_data_device";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_data_device_interface }
        }
    }

    pub trait RequestsTrait {
        /// start drag-and-drop operation
        ///
        /// This request asks the compositor to start a drag-and-drop
        /// operation on behalf of the client.
        /// 
        /// The source argument is the data source that provides the data
        /// for the eventual data transfer. If source is NULL, enter, leave
        /// and motion events are sent only to the client that initiated the
        /// drag and the client is expected to handle the data passing
        /// internally.
        /// 
        /// The origin surface is the surface where the drag originates and
        /// the client must have an active implicit grab that matches the
        /// serial.
        /// 
        /// The icon surface is an optional (can be NULL) surface that
        /// provides an icon to be moved around with the cursor.  Initially,
        /// the top-left corner of the icon surface is placed at the cursor
        /// hotspot, but subsequent wl_surface.attach request can move the
        /// relative position. Attach requests must be confirmed with
        /// wl_surface.commit as usual. The icon surface is given the role of
        /// a drag-and-drop icon. If the icon surface already has another role,
        /// it raises a protocol error.
        /// 
        /// The current and pending input regions of the icon wl_surface are
        /// cleared, and wl_surface.set_input_region is ignored until the
        /// wl_surface is no longer used as the icon surface. When the use
        /// as an icon ends, the current and pending input regions become
        /// undefined, and the wl_surface is unmapped.
        fn start_drag(&self, source: Option<&Proxy<super::wl_data_source::WlDataSource>>, origin: &Proxy<super::wl_surface::WlSurface>, icon: Option<&Proxy<super::wl_surface::WlSurface>>, serial: u32) ->();
        /// copy data to the selection
        ///
        /// This request asks the compositor to set the selection
        /// to the data from the source on behalf of the client.
        /// 
        /// To unset the selection, set the source to NULL.
        fn set_selection(&self, source: Option<&Proxy<super::wl_data_source::WlDataSource>>, serial: u32) ->();
        /// destroy data device
        ///
        /// This request destroys the data device.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        ///
        /// Only available since version 2 of the interface
        fn release(&self) ->();
    }

    impl RequestsTrait for Proxy<WlDataDevice> {
        fn start_drag(&self, source: Option<&Proxy<super::wl_data_source::WlDataSource>>, origin: &Proxy<super::wl_surface::WlSurface>, icon: Option<&Proxy<super::wl_surface::WlSurface>>, serial: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::StartDrag {
                source : source.map(|o| o.clone()),
                origin: origin.clone(),
                icon : icon.map(|o| o.clone()),
                serial: serial,
            };
            self.send(msg);
        }

        fn set_selection(&self, source: Option<&Proxy<super::wl_data_source::WlDataSource>>, serial: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetSelection {
                source : source.map(|o| o.clone()),
                serial: serial,
            };
            self.send(msg);
        }

        fn release(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Release;
            self.send(msg);
        }

    }
}

pub mod wl_data_device_manager {
    //! data transfer interface
    //!
    //! The wl_data_device_manager is a singleton global object that
    //! provides access to inter-client data transfer mechanisms such as
    //! copy-and-paste and drag-and-drop.  These mechanisms are tied to
    //! a wl_seat and this interface lets a client get a wl_data_device
    //! corresponding to a wl_seat.
    //! 
    //! Depending on the version bound, the objects created from the bound
    //! wl_data_device_manager object will have different requirements for
    //! functioning properly. See wl_data_source.set_actions,
    //! wl_data_offer.accept and wl_data_offer.finish for details.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    bitflags! {
        /// drag and drop actions
        ///
        /// This is a bitmask of the available/preferred actions in a
        /// drag-and-drop operation.
        /// 
        /// In the compositor, the selected action is a result of matching the
        /// actions offered by the source and destination sides.  "action" events
        /// with a "none" action will be sent to both source and destination if
        /// there is no match. All further checks will effectively happen on
        /// (source actions ∩ destination actions).
        /// 
        /// In addition, compositors may also pick different actions in
        /// reaction to key modifiers being pressed. One common design that
        /// is used in major toolkits (and the behavior recommended for
        /// compositors) is:
        /// 
        /// - If no modifiers are pressed, the first match (in bit order)
        /// will be used.
        /// - Pressing Shift selects "move", if enabled in the mask.
        /// - Pressing Control selects "copy", if enabled in the mask.
        /// 
        /// Behavior beyond that is considered implementation-dependent.
        /// Compositors may for example bind other modifiers (like Alt/Meta)
        /// or drags initiated with other buttons than BTN_LEFT to specific
        /// actions (e.g. "ask").
        pub struct DndAction: u32 {
            /// no action
            const None = 0;
            /// copy action
            const Copy = 1;
            /// move action
            const Move = 2;
            /// ask action
            const Ask = 4;
        }
    }
    impl DndAction {
        pub fn from_raw(n: u32) -> Option<DndAction> {
            Some(DndAction::from_bits_truncate(n))

        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }

    pub enum Request {
        /// create a new data source
        ///
        /// Create a new data source.
        CreateDataSource {id: Proxy<super::wl_data_source::WlDataSource>, },
        /// create a new data device
        ///
        /// Create a new data device for a given seat.
        GetDataDevice {id: Proxy<super::wl_data_device::WlDataDevice>, seat: Proxy<super::wl_seat::WlSeat>, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::CreateDataSource { id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                },
                Request::GetDataDevice { id, seat, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = seat.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlDataDeviceManager;

    impl Interface for WlDataDeviceManager {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_data_device_manager";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_data_device_manager_interface }
        }
    }

    pub trait RequestsTrait {
        /// create a new data source
        ///
        /// Create a new data source.
        fn create_data_source(&self) ->Result<NewProxy<super::wl_data_source::WlDataSource>, ()>;
        /// create a new data device
        ///
        /// Create a new data device for a given seat.
        fn get_data_device(&self, seat: &Proxy<super::wl_seat::WlSeat>) ->Result<NewProxy<super::wl_data_device::WlDataDevice>, ()>;
    }

    impl RequestsTrait for Proxy<WlDataDeviceManager> {
        fn create_data_source(&self) ->Result<NewProxy<super::wl_data_source::WlDataSource>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_data_source::WlDataSource>();
            let msg = Request::CreateDataSource {
                id: unsafe { Proxy::<super::wl_data_source::WlDataSource>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn get_data_device(&self, seat: &Proxy<super::wl_seat::WlSeat>) ->Result<NewProxy<super::wl_data_device::WlDataDevice>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_data_device::WlDataDevice>();
            let msg = Request::GetDataDevice {
                id: unsafe { Proxy::<super::wl_data_device::WlDataDevice>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                seat: seat.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod wl_shell {
    //! create desktop-style surfaces
    //!
    //! This interface is implemented by servers that provide
    //! desktop-style user interfaces.
    //! 
    //! It allows clients to associate a wl_shell_surface with
    //! a basic surface.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// given wl_surface has another role
        Role = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// create a shell surface from a surface
        ///
        /// Create a shell surface for an existing surface. This gives
        /// the wl_surface the role of a shell surface. If the wl_surface
        /// already has another role, it raises a protocol error.
        /// 
        /// Only one shell surface can be associated with a given surface.
        GetShellSurface {id: Proxy<super::wl_shell_surface::WlShellSurface>, surface: Proxy<super::wl_surface::WlSurface>, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::GetShellSurface { id, surface, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlShell;

    impl Interface for WlShell {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_shell";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_shell_interface }
        }
    }

    pub trait RequestsTrait {
        /// create a shell surface from a surface
        ///
        /// Create a shell surface for an existing surface. This gives
        /// the wl_surface the role of a shell surface. If the wl_surface
        /// already has another role, it raises a protocol error.
        /// 
        /// Only one shell surface can be associated with a given surface.
        fn get_shell_surface(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::wl_shell_surface::WlShellSurface>, ()>;
    }

    impl RequestsTrait for Proxy<WlShell> {
        fn get_shell_surface(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::wl_shell_surface::WlShellSurface>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_shell_surface::WlShellSurface>();
            let msg = Request::GetShellSurface {
                id: unsafe { Proxy::<super::wl_shell_surface::WlShellSurface>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                surface: surface.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod wl_shell_surface {
    //! desktop-style metadata interface
    //!
    //! An interface that may be implemented by a wl_surface, for
    //! implementations that provide a desktop-style user interface.
    //! 
    //! It provides requests to treat surfaces like toplevel, fullscreen
    //! or popup windows, move, resize or maximize them, associate
    //! metadata like title and class, etc.
    //! 
    //! On the server side the object is automatically destroyed when
    //! the related wl_surface is destroyed. On the client side,
    //! wl_shell_surface_destroy() must be called before destroying
    //! the wl_surface object.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    bitflags! {
        /// edge values for resizing
        ///
        /// These values are used to indicate which edge of a surface
        /// is being dragged in a resize operation. The server may
        /// use this information to adapt its behavior, e.g. choose
        /// an appropriate cursor image.
        pub struct Resize: u32 {
            /// no edge
            const None = 0;
            /// top edge
            const Top = 1;
            /// bottom edge
            const Bottom = 2;
            /// left edge
            const Left = 4;
            /// top and left edges
            const TopLeft = 5;
            /// bottom and left edges
            const BottomLeft = 6;
            /// right edge
            const Right = 8;
            /// top and right edges
            const TopRight = 9;
            /// bottom and right edges
            const BottomRight = 10;
        }
    }
    impl Resize {
        pub fn from_raw(n: u32) -> Option<Resize> {
            Some(Resize::from_bits_truncate(n))

        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }

    bitflags! {
        /// details of transient behaviour
        ///
        /// These flags specify details of the expected behaviour
        /// of transient surfaces. Used in the set_transient request.
        pub struct Transient: u32 {
            /// do not set keyboard focus
            const Inactive = 0x1;
        }
    }
    impl Transient {
        pub fn from_raw(n: u32) -> Option<Transient> {
            Some(Transient::from_bits_truncate(n))

        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }

    /// different method to set the surface fullscreen
    ///
    /// Hints to indicate to the compositor how to deal with a conflict
    /// between the dimensions of the surface and the dimensions of the
    /// output. The compositor is free to ignore this parameter.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum FullscreenMethod {
        /// no preference, apply default policy
        Default = 0,
        /// scale, preserve the surface's aspect ratio and center on output
        Scale = 1,
        /// switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch
        Driver = 2,
        /// no upscaling, center on output and add black borders to compensate size mismatch
        Fill = 3,
    }
    impl FullscreenMethod {
        pub fn from_raw(n: u32) -> Option<FullscreenMethod> {
            match n {
                0 => Some(FullscreenMethod::Default),
                1 => Some(FullscreenMethod::Scale),
                2 => Some(FullscreenMethod::Driver),
                3 => Some(FullscreenMethod::Fill),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// respond to a ping event
        ///
        /// A client must respond to a ping event with a pong request or
        /// the client may be deemed unresponsive.
        Pong {serial: u32, },
        /// start an interactive move
        ///
        /// Start a pointer-driven move of the surface.
        /// 
        /// This request must be used in response to a button press event.
        /// The server may ignore move requests depending on the state of
        /// the surface (e.g. fullscreen or maximized).
        Move {seat: Proxy<super::wl_seat::WlSeat>, serial: u32, },
        /// start an interactive resize
        ///
        /// Start a pointer-driven resizing of the surface.
        /// 
        /// This request must be used in response to a button press event.
        /// The server may ignore resize requests depending on the state of
        /// the surface (e.g. fullscreen or maximized).
        Resize {seat: Proxy<super::wl_seat::WlSeat>, serial: u32, edges: Resize, },
        /// make the surface a toplevel surface
        ///
        /// Map the surface as a toplevel surface.
        /// 
        /// A toplevel surface is not fullscreen, maximized or transient.
        SetToplevel,
        /// make the surface a transient surface
        ///
        /// Map the surface relative to an existing surface.
        /// 
        /// The x and y arguments specify the location of the upper left
        /// corner of the surface relative to the upper left corner of the
        /// parent surface, in surface-local coordinates.
        /// 
        /// The flags argument controls details of the transient behaviour.
        SetTransient {parent: Proxy<super::wl_surface::WlSurface>, x: i32, y: i32, flags: Transient, },
        /// make the surface a fullscreen surface
        ///
        /// Map the surface as a fullscreen surface.
        /// 
        /// If an output parameter is given then the surface will be made
        /// fullscreen on that output. If the client does not specify the
        /// output then the compositor will apply its policy - usually
        /// choosing the output on which the surface has the biggest surface
        /// area.
        /// 
        /// The client may specify a method to resolve a size conflict
        /// between the output size and the surface size - this is provided
        /// through the method parameter.
        /// 
        /// The framerate parameter is used only when the method is set
        /// to "driver", to indicate the preferred framerate. A value of 0
        /// indicates that the client does not care about framerate.  The
        /// framerate is specified in mHz, that is framerate of 60000 is 60Hz.
        /// 
        /// A method of "scale" or "driver" implies a scaling operation of
        /// the surface, either via a direct scaling operation or a change of
        /// the output mode. This will override any kind of output scaling, so
        /// that mapping a surface with a buffer size equal to the mode can
        /// fill the screen independent of buffer_scale.
        /// 
        /// A method of "fill" means we don't scale up the buffer, however
        /// any output scale is applied. This means that you may run into
        /// an edge case where the application maps a buffer with the same
        /// size of the output mode but buffer_scale 1 (thus making a
        /// surface larger than the output). In this case it is allowed to
        /// downscale the results to fit the screen.
        /// 
        /// The compositor must reply to this request with a configure event
        /// with the dimensions for the output on which the surface will
        /// be made fullscreen.
        SetFullscreen {method: FullscreenMethod, framerate: u32, output: Option<Proxy<super::wl_output::WlOutput>>, },
        /// make the surface a popup surface
        ///
        /// Map the surface as a popup.
        /// 
        /// A popup surface is a transient surface with an added pointer
        /// grab.
        /// 
        /// An existing implicit grab will be changed to owner-events mode,
        /// and the popup grab will continue after the implicit grab ends
        /// (i.e. releasing the mouse button does not cause the popup to
        /// be unmapped).
        /// 
        /// The popup grab continues until the window is destroyed or a
        /// mouse button is pressed in any other client's window. A click
        /// in any of the client's surfaces is reported as normal, however,
        /// clicks in other clients' surfaces will be discarded and trigger
        /// the callback.
        /// 
        /// The x and y arguments specify the location of the upper left
        /// corner of the surface relative to the upper left corner of the
        /// parent surface, in surface-local coordinates.
        SetPopup {seat: Proxy<super::wl_seat::WlSeat>, serial: u32, parent: Proxy<super::wl_surface::WlSurface>, x: i32, y: i32, flags: Transient, },
        /// make the surface a maximized surface
        ///
        /// Map the surface as a maximized surface.
        /// 
        /// If an output parameter is given then the surface will be
        /// maximized on that output. If the client does not specify the
        /// output then the compositor will apply its policy - usually
        /// choosing the output on which the surface has the biggest surface
        /// area.
        /// 
        /// The compositor will reply with a configure event telling
        /// the expected new surface size. The operation is completed
        /// on the next buffer attach to this surface.
        /// 
        /// A maximized surface typically fills the entire output it is
        /// bound to, except for desktop elements such as panels. This is
        /// the main difference between a maximized shell surface and a
        /// fullscreen shell surface.
        /// 
        /// The details depend on the compositor implementation.
        SetMaximized {output: Option<Proxy<super::wl_output::WlOutput>>, },
        /// set surface title
        ///
        /// Set a short title for the surface.
        /// 
        /// This string may be used to identify the surface in a task bar,
        /// window list, or other user interface elements provided by the
        /// compositor.
        /// 
        /// The string must be encoded in UTF-8.
        SetTitle {title: String, },
        /// set surface class
        ///
        /// Set a class for the surface.
        /// 
        /// The surface class identifies the general class of applications
        /// to which the surface belongs. A common convention is to use the
        /// file name (or the full path if it is a non-standard location) of
        /// the application's .desktop file as the class.
        SetClass {class_: String, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Pong { serial, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(0, &mut _args_array)
                },
                Request::Move { seat, serial, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    f(1, &mut _args_array)
                },
                Request::Resize { seat, serial, edges, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    _args_array[2].u = edges.to_raw();
                    f(2, &mut _args_array)
                },
                Request::SetToplevel => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                },
                Request::SetTransient { parent, x, y, flags, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = parent.c_ptr() as *mut _;
                    _args_array[1].i = x;
                    _args_array[2].i = y;
                    _args_array[3].u = flags.to_raw();
                    f(4, &mut _args_array)
                },
                Request::SetFullscreen { method, framerate, output, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = method.to_raw();
                    _args_array[1].u = framerate;
                    _args_array[2].o = output.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    f(5, &mut _args_array)
                },
                Request::SetPopup { seat, serial, parent, x, y, flags, } => {
                    let mut _args_array: [wl_argument; 6] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    _args_array[2].o = parent.c_ptr() as *mut _;
                    _args_array[3].i = x;
                    _args_array[4].i = y;
                    _args_array[5].u = flags.to_raw();
                    f(6, &mut _args_array)
                },
                Request::SetMaximized { output, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = output.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    f(7, &mut _args_array)
                },
                Request::SetTitle { title, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(title).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(8, &mut _args_array)
                },
                Request::SetClass { class_, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(class_).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(9, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// ping client
        ///
        /// Ping a client to check if it is receiving events and sending
        /// requests. A client is expected to reply with a pong request.
        Ping {serial: u32, },
        /// suggest resize
        ///
        /// The configure event asks the client to resize its surface.
        /// 
        /// The size is a hint, in the sense that the client is free to
        /// ignore it if it doesn't resize, pick a smaller size (to
        /// satisfy aspect ratio or resize in steps of NxM pixels).
        /// 
        /// The edges parameter provides a hint about how the surface
        /// was resized. The client may use this information to decide
        /// how to adjust its content to the new size (e.g. a scrolling
        /// area might adjust its content position to leave the viewable
        /// content unmoved).
        /// 
        /// The client is free to dismiss all but the last configure
        /// event it received.
        /// 
        /// The width and height arguments specify the size of the window
        /// in surface-local coordinates.
        Configure {edges: Resize, width: i32, height: i32, },
        /// popup interaction is done
        ///
        /// The popup_done event is sent out when a popup grab is broken,
        /// that is, when the user clicks a surface that doesn't belong
        /// to the client owning the popup surface.
        PopupDone,
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Ping {
                        serial: _args[0].u,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Configure {
                        edges: Resize::from_raw(_args[0].u).ok_or(())?,
                        width: _args[1].i,
                        height: _args[2].i,
                }) },
                2 => {
                    Ok(Event::PopupDone) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlShellSurface;

    impl Interface for WlShellSurface {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_shell_surface";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_shell_surface_interface }
        }
    }

    pub trait RequestsTrait {
        /// respond to a ping event
        ///
        /// A client must respond to a ping event with a pong request or
        /// the client may be deemed unresponsive.
        fn pong(&self, serial: u32) ->();
        /// start an interactive move
        ///
        /// Start a pointer-driven move of the surface.
        /// 
        /// This request must be used in response to a button press event.
        /// The server may ignore move requests depending on the state of
        /// the surface (e.g. fullscreen or maximized).
        fn _move(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32) ->();
        /// start an interactive resize
        ///
        /// Start a pointer-driven resizing of the surface.
        /// 
        /// This request must be used in response to a button press event.
        /// The server may ignore resize requests depending on the state of
        /// the surface (e.g. fullscreen or maximized).
        fn resize(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32, edges: Resize) ->();
        /// make the surface a toplevel surface
        ///
        /// Map the surface as a toplevel surface.
        /// 
        /// A toplevel surface is not fullscreen, maximized or transient.
        fn set_toplevel(&self) ->();
        /// make the surface a transient surface
        ///
        /// Map the surface relative to an existing surface.
        /// 
        /// The x and y arguments specify the location of the upper left
        /// corner of the surface relative to the upper left corner of the
        /// parent surface, in surface-local coordinates.
        /// 
        /// The flags argument controls details of the transient behaviour.
        fn set_transient(&self, parent: &Proxy<super::wl_surface::WlSurface>, x: i32, y: i32, flags: Transient) ->();
        /// make the surface a fullscreen surface
        ///
        /// Map the surface as a fullscreen surface.
        /// 
        /// If an output parameter is given then the surface will be made
        /// fullscreen on that output. If the client does not specify the
        /// output then the compositor will apply its policy - usually
        /// choosing the output on which the surface has the biggest surface
        /// area.
        /// 
        /// The client may specify a method to resolve a size conflict
        /// between the output size and the surface size - this is provided
        /// through the method parameter.
        /// 
        /// The framerate parameter is used only when the method is set
        /// to "driver", to indicate the preferred framerate. A value of 0
        /// indicates that the client does not care about framerate.  The
        /// framerate is specified in mHz, that is framerate of 60000 is 60Hz.
        /// 
        /// A method of "scale" or "driver" implies a scaling operation of
        /// the surface, either via a direct scaling operation or a change of
        /// the output mode. This will override any kind of output scaling, so
        /// that mapping a surface with a buffer size equal to the mode can
        /// fill the screen independent of buffer_scale.
        /// 
        /// A method of "fill" means we don't scale up the buffer, however
        /// any output scale is applied. This means that you may run into
        /// an edge case where the application maps a buffer with the same
        /// size of the output mode but buffer_scale 1 (thus making a
        /// surface larger than the output). In this case it is allowed to
        /// downscale the results to fit the screen.
        /// 
        /// The compositor must reply to this request with a configure event
        /// with the dimensions for the output on which the surface will
        /// be made fullscreen.
        fn set_fullscreen(&self, method: FullscreenMethod, framerate: u32, output: Option<&Proxy<super::wl_output::WlOutput>>) ->();
        /// make the surface a popup surface
        ///
        /// Map the surface as a popup.
        /// 
        /// A popup surface is a transient surface with an added pointer
        /// grab.
        /// 
        /// An existing implicit grab will be changed to owner-events mode,
        /// and the popup grab will continue after the implicit grab ends
        /// (i.e. releasing the mouse button does not cause the popup to
        /// be unmapped).
        /// 
        /// The popup grab continues until the window is destroyed or a
        /// mouse button is pressed in any other client's window. A click
        /// in any of the client's surfaces is reported as normal, however,
        /// clicks in other clients' surfaces will be discarded and trigger
        /// the callback.
        /// 
        /// The x and y arguments specify the location of the upper left
        /// corner of the surface relative to the upper left corner of the
        /// parent surface, in surface-local coordinates.
        fn set_popup(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32, parent: &Proxy<super::wl_surface::WlSurface>, x: i32, y: i32, flags: Transient) ->();
        /// make the surface a maximized surface
        ///
        /// Map the surface as a maximized surface.
        /// 
        /// If an output parameter is given then the surface will be
        /// maximized on that output. If the client does not specify the
        /// output then the compositor will apply its policy - usually
        /// choosing the output on which the surface has the biggest surface
        /// area.
        /// 
        /// The compositor will reply with a configure event telling
        /// the expected new surface size. The operation is completed
        /// on the next buffer attach to this surface.
        /// 
        /// A maximized surface typically fills the entire output it is
        /// bound to, except for desktop elements such as panels. This is
        /// the main difference between a maximized shell surface and a
        /// fullscreen shell surface.
        /// 
        /// The details depend on the compositor implementation.
        fn set_maximized(&self, output: Option<&Proxy<super::wl_output::WlOutput>>) ->();
        /// set surface title
        ///
        /// Set a short title for the surface.
        /// 
        /// This string may be used to identify the surface in a task bar,
        /// window list, or other user interface elements provided by the
        /// compositor.
        /// 
        /// The string must be encoded in UTF-8.
        fn set_title(&self, title: String) ->();
        /// set surface class
        ///
        /// Set a class for the surface.
        /// 
        /// The surface class identifies the general class of applications
        /// to which the surface belongs. A common convention is to use the
        /// file name (or the full path if it is a non-standard location) of
        /// the application's .desktop file as the class.
        fn set_class(&self, class_: String) ->();
    }

    impl RequestsTrait for Proxy<WlShellSurface> {
        fn pong(&self, serial: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Pong {
                serial: serial,
            };
            self.send(msg);
        }

        fn _move(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Move {
                seat: seat.clone(),
                serial: serial,
            };
            self.send(msg);
        }

        fn resize(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32, edges: Resize) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Resize {
                seat: seat.clone(),
                serial: serial,
                edges: edges,
            };
            self.send(msg);
        }

        fn set_toplevel(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetToplevel;
            self.send(msg);
        }

        fn set_transient(&self, parent: &Proxy<super::wl_surface::WlSurface>, x: i32, y: i32, flags: Transient) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetTransient {
                parent: parent.clone(),
                x: x,
                y: y,
                flags: flags,
            };
            self.send(msg);
        }

        fn set_fullscreen(&self, method: FullscreenMethod, framerate: u32, output: Option<&Proxy<super::wl_output::WlOutput>>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetFullscreen {
                method: method,
                framerate: framerate,
                output : output.map(|o| o.clone()),
            };
            self.send(msg);
        }

        fn set_popup(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32, parent: &Proxy<super::wl_surface::WlSurface>, x: i32, y: i32, flags: Transient) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetPopup {
                seat: seat.clone(),
                serial: serial,
                parent: parent.clone(),
                x: x,
                y: y,
                flags: flags,
            };
            self.send(msg);
        }

        fn set_maximized(&self, output: Option<&Proxy<super::wl_output::WlOutput>>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetMaximized {
                output : output.map(|o| o.clone()),
            };
            self.send(msg);
        }

        fn set_title(&self, title: String) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetTitle {
                title: title,
            };
            self.send(msg);
        }

        fn set_class(&self, class_: String) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetClass {
                class_: class_,
            };
            self.send(msg);
        }

    }
}

pub mod wl_surface {
    //! an onscreen surface
    //!
    //! A surface is a rectangular area that is displayed on the screen.
    //! It has a location, size and pixel contents.
    //! 
    //! The size of a surface (and relative positions on it) is described
    //! in surface-local coordinates, which may differ from the buffer
    //! coordinates of the pixel content, in case a buffer_transform
    //! or a buffer_scale is used.
    //! 
    //! A surface without a "role" is fairly useless: a compositor does
    //! not know where, when or how to present it. The role is the
    //! purpose of a wl_surface. Examples of roles are a cursor for a
    //! pointer (as set by wl_pointer.set_cursor), a drag icon
    //! (wl_data_device.start_drag), a sub-surface
    //! (wl_subcompositor.get_subsurface), and a window as defined by a
    //! shell protocol (e.g. wl_shell.get_shell_surface).
    //! 
    //! A surface can have only one role at a time. Initially a
    //! wl_surface does not have a role. Once a wl_surface is given a
    //! role, it is set permanently for the whole lifetime of the
    //! wl_surface object. Giving the current role again is allowed,
    //! unless explicitly forbidden by the relevant interface
    //! specification.
    //! 
    //! Surface roles are given by requests in other interfaces such as
    //! wl_pointer.set_cursor. The request should explicitly mention
    //! that this request gives a role to a wl_surface. Often, this
    //! request also creates a new protocol object that represents the
    //! role and adds additional functionality to wl_surface. When a
    //! client wants to destroy a wl_surface, they must destroy this 'role
    //! object' before the wl_surface.
    //! 
    //! Destroying the role object does not remove the role from the
    //! wl_surface, but it may stop the wl_surface from "playing the role".
    //! For instance, if a wl_subsurface object is destroyed, the wl_surface
    //! it was created for will be unmapped and forget its position and
    //! z-order. It is allowed to create a wl_subsurface for the same
    //! wl_surface again, but it is not allowed to use the wl_surface as
    //! a cursor (cursor is a different role than sub-surface, and role
    //! switching is not allowed).
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// wl_surface error values
    ///
    /// These errors can be emitted in response to wl_surface requests.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// buffer scale value is invalid
        InvalidScale = 0,
        /// buffer transform value is invalid
        InvalidTransform = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidScale),
                1 => Some(Error::InvalidTransform),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// delete surface
        ///
        /// Deletes the surface and invalidates its object ID.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// set the surface contents
        ///
        /// Set a buffer as the content of this surface.
        /// 
        /// The new size of the surface is calculated based on the buffer
        /// size transformed by the inverse buffer_transform and the
        /// inverse buffer_scale. This means that the supplied buffer
        /// must be an integer multiple of the buffer_scale.
        /// 
        /// The x and y arguments specify the location of the new pending
        /// buffer's upper left corner, relative to the current buffer's upper
        /// left corner, in surface-local coordinates. In other words, the
        /// x and y, combined with the new surface size define in which
        /// directions the surface's size changes.
        /// 
        /// Surface contents are double-buffered state, see wl_surface.commit.
        /// 
        /// The initial surface contents are void; there is no content.
        /// wl_surface.attach assigns the given wl_buffer as the pending
        /// wl_buffer. wl_surface.commit makes the pending wl_buffer the new
        /// surface contents, and the size of the surface becomes the size
        /// calculated from the wl_buffer, as described above. After commit,
        /// there is no pending buffer until the next attach.
        /// 
        /// Committing a pending wl_buffer allows the compositor to read the
        /// pixels in the wl_buffer. The compositor may access the pixels at
        /// any time after the wl_surface.commit request. When the compositor
        /// will not access the pixels anymore, it will send the
        /// wl_buffer.release event. Only after receiving wl_buffer.release,
        /// the client may reuse the wl_buffer. A wl_buffer that has been
        /// attached and then replaced by another attach instead of committed
        /// will not receive a release event, and is not used by the
        /// compositor.
        /// 
        /// Destroying the wl_buffer after wl_buffer.release does not change
        /// the surface contents. However, if the client destroys the
        /// wl_buffer before receiving the wl_buffer.release event, the surface
        /// contents become undefined immediately.
        /// 
        /// If wl_surface.attach is sent with a NULL wl_buffer, the
        /// following wl_surface.commit will remove the surface content.
        Attach {buffer: Option<Proxy<super::wl_buffer::WlBuffer>>, x: i32, y: i32, },
        /// mark part of the surface damaged
        ///
        /// This request is used to describe the regions where the pending
        /// buffer is different from the current surface contents, and where
        /// the surface therefore needs to be repainted. The compositor
        /// ignores the parts of the damage that fall outside of the surface.
        /// 
        /// Damage is double-buffered state, see wl_surface.commit.
        /// 
        /// The damage rectangle is specified in surface-local coordinates,
        /// where x and y specify the upper left corner of the damage rectangle.
        /// 
        /// The initial value for pending damage is empty: no damage.
        /// wl_surface.damage adds pending damage: the new pending damage
        /// is the union of old pending damage and the given rectangle.
        /// 
        /// wl_surface.commit assigns pending damage as the current damage,
        /// and clears pending damage. The server will clear the current
        /// damage as it repaints the surface.
        /// 
        /// Alternatively, damage can be posted with wl_surface.damage_buffer
        /// which uses buffer coordinates instead of surface coordinates,
        /// and is probably the preferred and intuitive way of doing this.
        Damage {x: i32, y: i32, width: i32, height: i32, },
        /// request a frame throttling hint
        ///
        /// Request a notification when it is a good time to start drawing a new
        /// frame, by creating a frame callback. This is useful for throttling
        /// redrawing operations, and driving animations.
        /// 
        /// When a client is animating on a wl_surface, it can use the 'frame'
        /// request to get notified when it is a good time to draw and commit the
        /// next frame of animation. If the client commits an update earlier than
        /// that, it is likely that some updates will not make it to the display,
        /// and the client is wasting resources by drawing too often.
        /// 
        /// The frame request will take effect on the next wl_surface.commit.
        /// The notification will only be posted for one frame unless
        /// requested again. For a wl_surface, the notifications are posted in
        /// the order the frame requests were committed.
        /// 
        /// The server must send the notifications so that a client
        /// will not send excessive updates, while still allowing
        /// the highest possible update rate for clients that wait for the reply
        /// before drawing again. The server should give some time for the client
        /// to draw and commit after sending the frame callback events to let it
        /// hit the next output refresh.
        /// 
        /// A server should avoid signaling the frame callbacks if the
        /// surface is not visible in any way, e.g. the surface is off-screen,
        /// or completely obscured by other opaque surfaces.
        /// 
        /// The object returned by this request will be destroyed by the
        /// compositor after the callback is fired and as such the client must not
        /// attempt to use it after that point.
        /// 
        /// The callback_data passed in the callback is the current time, in
        /// milliseconds, with an undefined base.
        Frame {callback: Proxy<super::wl_callback::WlCallback>, },
        /// set opaque region
        ///
        /// This request sets the region of the surface that contains
        /// opaque content.
        /// 
        /// The opaque region is an optimization hint for the compositor
        /// that lets it optimize the redrawing of content behind opaque
        /// regions.  Setting an opaque region is not required for correct
        /// behaviour, but marking transparent content as opaque will result
        /// in repaint artifacts.
        /// 
        /// The opaque region is specified in surface-local coordinates.
        /// 
        /// The compositor ignores the parts of the opaque region that fall
        /// outside of the surface.
        /// 
        /// Opaque region is double-buffered state, see wl_surface.commit.
        /// 
        /// wl_surface.set_opaque_region changes the pending opaque region.
        /// wl_surface.commit copies the pending region to the current region.
        /// Otherwise, the pending and current regions are never changed.
        /// 
        /// The initial value for an opaque region is empty. Setting the pending
        /// opaque region has copy semantics, and the wl_region object can be
        /// destroyed immediately. A NULL wl_region causes the pending opaque
        /// region to be set to empty.
        SetOpaqueRegion {region: Option<Proxy<super::wl_region::WlRegion>>, },
        /// set input region
        ///
        /// This request sets the region of the surface that can receive
        /// pointer and touch events.
        /// 
        /// Input events happening outside of this region will try the next
        /// surface in the server surface stack. The compositor ignores the
        /// parts of the input region that fall outside of the surface.
        /// 
        /// The input region is specified in surface-local coordinates.
        /// 
        /// Input region is double-buffered state, see wl_surface.commit.
        /// 
        /// wl_surface.set_input_region changes the pending input region.
        /// wl_surface.commit copies the pending region to the current region.
        /// Otherwise the pending and current regions are never changed,
        /// except cursor and icon surfaces are special cases, see
        /// wl_pointer.set_cursor and wl_data_device.start_drag.
        /// 
        /// The initial value for an input region is infinite. That means the
        /// whole surface will accept input. Setting the pending input region
        /// has copy semantics, and the wl_region object can be destroyed
        /// immediately. A NULL wl_region causes the input region to be set
        /// to infinite.
        SetInputRegion {region: Option<Proxy<super::wl_region::WlRegion>>, },
        /// commit pending surface state
        ///
        /// Surface state (input, opaque, and damage regions, attached buffers,
        /// etc.) is double-buffered. Protocol requests modify the pending state,
        /// as opposed to the current state in use by the compositor. A commit
        /// request atomically applies all pending state, replacing the current
        /// state. After commit, the new pending state is as documented for each
        /// related request.
        /// 
        /// On commit, a pending wl_buffer is applied first, and all other state
        /// second. This means that all coordinates in double-buffered state are
        /// relative to the new wl_buffer coming into use, except for
        /// wl_surface.attach itself. If there is no pending wl_buffer, the
        /// coordinates are relative to the current surface contents.
        /// 
        /// All requests that need a commit to become effective are documented
        /// to affect double-buffered state.
        /// 
        /// Other interfaces may add further double-buffered surface state.
        Commit,
        /// sets the buffer transformation
        ///
        /// This request sets an optional transformation on how the compositor
        /// interprets the contents of the buffer attached to the surface. The
        /// accepted values for the transform parameter are the values for
        /// wl_output.transform.
        /// 
        /// Buffer transform is double-buffered state, see wl_surface.commit.
        /// 
        /// A newly created surface has its buffer transformation set to normal.
        /// 
        /// wl_surface.set_buffer_transform changes the pending buffer
        /// transformation. wl_surface.commit copies the pending buffer
        /// transformation to the current one. Otherwise, the pending and current
        /// values are never changed.
        /// 
        /// The purpose of this request is to allow clients to render content
        /// according to the output transform, thus permitting the compositor to
        /// use certain optimizations even if the display is rotated. Using
        /// hardware overlays and scanning out a client buffer for fullscreen
        /// surfaces are examples of such optimizations. Those optimizations are
        /// highly dependent on the compositor implementation, so the use of this
        /// request should be considered on a case-by-case basis.
        /// 
        /// Note that if the transform value includes 90 or 270 degree rotation,
        /// the width of the buffer will become the surface height and the height
        /// of the buffer will become the surface width.
        /// 
        /// If transform is not one of the values from the
        /// wl_output.transform enum the invalid_transform protocol error
        /// is raised.
        ///
        /// Only available since version 2 of the interface
        SetBufferTransform {transform: super::wl_output::Transform, },
        /// sets the buffer scaling factor
        ///
        /// This request sets an optional scaling factor on how the compositor
        /// interprets the contents of the buffer attached to the window.
        /// 
        /// Buffer scale is double-buffered state, see wl_surface.commit.
        /// 
        /// A newly created surface has its buffer scale set to 1.
        /// 
        /// wl_surface.set_buffer_scale changes the pending buffer scale.
        /// wl_surface.commit copies the pending buffer scale to the current one.
        /// Otherwise, the pending and current values are never changed.
        /// 
        /// The purpose of this request is to allow clients to supply higher
        /// resolution buffer data for use on high resolution outputs. It is
        /// intended that you pick the same buffer scale as the scale of the
        /// output that the surface is displayed on. This means the compositor
        /// can avoid scaling when rendering the surface on that output.
        /// 
        /// Note that if the scale is larger than 1, then you have to attach
        /// a buffer that is larger (by a factor of scale in each dimension)
        /// than the desired surface size.
        /// 
        /// If scale is not positive the invalid_scale protocol error is
        /// raised.
        ///
        /// Only available since version 3 of the interface
        SetBufferScale {scale: i32, },
        /// mark part of the surface damaged using buffer coordinates
        ///
        /// This request is used to describe the regions where the pending
        /// buffer is different from the current surface contents, and where
        /// the surface therefore needs to be repainted. The compositor
        /// ignores the parts of the damage that fall outside of the surface.
        /// 
        /// Damage is double-buffered state, see wl_surface.commit.
        /// 
        /// The damage rectangle is specified in buffer coordinates,
        /// where x and y specify the upper left corner of the damage rectangle.
        /// 
        /// The initial value for pending damage is empty: no damage.
        /// wl_surface.damage_buffer adds pending damage: the new pending
        /// damage is the union of old pending damage and the given rectangle.
        /// 
        /// wl_surface.commit assigns pending damage as the current damage,
        /// and clears pending damage. The server will clear the current
        /// damage as it repaints the surface.
        /// 
        /// This request differs from wl_surface.damage in only one way - it
        /// takes damage in buffer coordinates instead of surface-local
        /// coordinates. While this generally is more intuitive than surface
        /// coordinates, it is especially desirable when using wp_viewport
        /// or when a drawing library (like EGL) is unaware of buffer scale
        /// and buffer transform.
        /// 
        /// Note: Because buffer transformation changes and damage requests may
        /// be interleaved in the protocol stream, it is impossible to determine
        /// the actual mapping between surface and buffer damage until
        /// wl_surface.commit time. Therefore, compositors wishing to take both
        /// kinds of damage into account will have to accumulate damage from the
        /// two requests separately and only transform from one to the other
        /// after receiving the wl_surface.commit.
        ///
        /// Only available since version 4 of the interface
        DamageBuffer {x: i32, y: i32, width: i32, height: i32, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                },
                Request::Attach { buffer, x, y, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = buffer.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[1].i = x;
                    _args_array[2].i = y;
                    f(1, &mut _args_array)
                },
                Request::Damage { x, y, width, height, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(2, &mut _args_array)
                },
                Request::Frame { callback, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = callback.c_ptr() as *mut _;
                    f(3, &mut _args_array)
                },
                Request::SetOpaqueRegion { region, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = region.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    f(4, &mut _args_array)
                },
                Request::SetInputRegion { region, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = region.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    f(5, &mut _args_array)
                },
                Request::Commit => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(6, &mut _args_array)
                },
                Request::SetBufferTransform { transform, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = transform.to_raw() as i32;
                    f(7, &mut _args_array)
                },
                Request::SetBufferScale { scale, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = scale;
                    f(8, &mut _args_array)
                },
                Request::DamageBuffer { x, y, width, height, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(9, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// surface enters an output
        ///
        /// This is emitted whenever a surface's creation, movement, or resizing
        /// results in some part of it being within the scanout region of an
        /// output.
        /// 
        /// Note that a surface may be overlapping with zero or more outputs.
        Enter {output: Proxy<super::wl_output::WlOutput>, },
        /// surface leaves an output
        ///
        /// This is emitted whenever a surface's creation, movement, or resizing
        /// results in it no longer having any part of it within the scanout region
        /// of an output.
        Leave {output: Proxy<super::wl_output::WlOutput>, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Enter {
                        output: Proxy::<super::wl_output::WlOutput>::from_c_ptr(_args[0].o as *mut _),
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Leave {
                        output: Proxy::<super::wl_output::WlOutput>::from_c_ptr(_args[0].o as *mut _),
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlSurface;

    impl Interface for WlSurface {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_surface";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_surface_interface }
        }
    }

    pub trait RequestsTrait {
        /// delete surface
        ///
        /// Deletes the surface and invalidates its object ID.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// set the surface contents
        ///
        /// Set a buffer as the content of this surface.
        /// 
        /// The new size of the surface is calculated based on the buffer
        /// size transformed by the inverse buffer_transform and the
        /// inverse buffer_scale. This means that the supplied buffer
        /// must be an integer multiple of the buffer_scale.
        /// 
        /// The x and y arguments specify the location of the new pending
        /// buffer's upper left corner, relative to the current buffer's upper
        /// left corner, in surface-local coordinates. In other words, the
        /// x and y, combined with the new surface size define in which
        /// directions the surface's size changes.
        /// 
        /// Surface contents are double-buffered state, see wl_surface.commit.
        /// 
        /// The initial surface contents are void; there is no content.
        /// wl_surface.attach assigns the given wl_buffer as the pending
        /// wl_buffer. wl_surface.commit makes the pending wl_buffer the new
        /// surface contents, and the size of the surface becomes the size
        /// calculated from the wl_buffer, as described above. After commit,
        /// there is no pending buffer until the next attach.
        /// 
        /// Committing a pending wl_buffer allows the compositor to read the
        /// pixels in the wl_buffer. The compositor may access the pixels at
        /// any time after the wl_surface.commit request. When the compositor
        /// will not access the pixels anymore, it will send the
        /// wl_buffer.release event. Only after receiving wl_buffer.release,
        /// the client may reuse the wl_buffer. A wl_buffer that has been
        /// attached and then replaced by another attach instead of committed
        /// will not receive a release event, and is not used by the
        /// compositor.
        /// 
        /// Destroying the wl_buffer after wl_buffer.release does not change
        /// the surface contents. However, if the client destroys the
        /// wl_buffer before receiving the wl_buffer.release event, the surface
        /// contents become undefined immediately.
        /// 
        /// If wl_surface.attach is sent with a NULL wl_buffer, the
        /// following wl_surface.commit will remove the surface content.
        fn attach(&self, buffer: Option<&Proxy<super::wl_buffer::WlBuffer>>, x: i32, y: i32) ->();
        /// mark part of the surface damaged
        ///
        /// This request is used to describe the regions where the pending
        /// buffer is different from the current surface contents, and where
        /// the surface therefore needs to be repainted. The compositor
        /// ignores the parts of the damage that fall outside of the surface.
        /// 
        /// Damage is double-buffered state, see wl_surface.commit.
        /// 
        /// The damage rectangle is specified in surface-local coordinates,
        /// where x and y specify the upper left corner of the damage rectangle.
        /// 
        /// The initial value for pending damage is empty: no damage.
        /// wl_surface.damage adds pending damage: the new pending damage
        /// is the union of old pending damage and the given rectangle.
        /// 
        /// wl_surface.commit assigns pending damage as the current damage,
        /// and clears pending damage. The server will clear the current
        /// damage as it repaints the surface.
        /// 
        /// Alternatively, damage can be posted with wl_surface.damage_buffer
        /// which uses buffer coordinates instead of surface coordinates,
        /// and is probably the preferred and intuitive way of doing this.
        fn damage(&self, x: i32, y: i32, width: i32, height: i32) ->();
        /// request a frame throttling hint
        ///
        /// Request a notification when it is a good time to start drawing a new
        /// frame, by creating a frame callback. This is useful for throttling
        /// redrawing operations, and driving animations.
        /// 
        /// When a client is animating on a wl_surface, it can use the 'frame'
        /// request to get notified when it is a good time to draw and commit the
        /// next frame of animation. If the client commits an update earlier than
        /// that, it is likely that some updates will not make it to the display,
        /// and the client is wasting resources by drawing too often.
        /// 
        /// The frame request will take effect on the next wl_surface.commit.
        /// The notification will only be posted for one frame unless
        /// requested again. For a wl_surface, the notifications are posted in
        /// the order the frame requests were committed.
        /// 
        /// The server must send the notifications so that a client
        /// will not send excessive updates, while still allowing
        /// the highest possible update rate for clients that wait for the reply
        /// before drawing again. The server should give some time for the client
        /// to draw and commit after sending the frame callback events to let it
        /// hit the next output refresh.
        /// 
        /// A server should avoid signaling the frame callbacks if the
        /// surface is not visible in any way, e.g. the surface is off-screen,
        /// or completely obscured by other opaque surfaces.
        /// 
        /// The object returned by this request will be destroyed by the
        /// compositor after the callback is fired and as such the client must not
        /// attempt to use it after that point.
        /// 
        /// The callback_data passed in the callback is the current time, in
        /// milliseconds, with an undefined base.
        fn frame(&self) ->Result<NewProxy<super::wl_callback::WlCallback>, ()>;
        /// set opaque region
        ///
        /// This request sets the region of the surface that contains
        /// opaque content.
        /// 
        /// The opaque region is an optimization hint for the compositor
        /// that lets it optimize the redrawing of content behind opaque
        /// regions.  Setting an opaque region is not required for correct
        /// behaviour, but marking transparent content as opaque will result
        /// in repaint artifacts.
        /// 
        /// The opaque region is specified in surface-local coordinates.
        /// 
        /// The compositor ignores the parts of the opaque region that fall
        /// outside of the surface.
        /// 
        /// Opaque region is double-buffered state, see wl_surface.commit.
        /// 
        /// wl_surface.set_opaque_region changes the pending opaque region.
        /// wl_surface.commit copies the pending region to the current region.
        /// Otherwise, the pending and current regions are never changed.
        /// 
        /// The initial value for an opaque region is empty. Setting the pending
        /// opaque region has copy semantics, and the wl_region object can be
        /// destroyed immediately. A NULL wl_region causes the pending opaque
        /// region to be set to empty.
        fn set_opaque_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) ->();
        /// set input region
        ///
        /// This request sets the region of the surface that can receive
        /// pointer and touch events.
        /// 
        /// Input events happening outside of this region will try the next
        /// surface in the server surface stack. The compositor ignores the
        /// parts of the input region that fall outside of the surface.
        /// 
        /// The input region is specified in surface-local coordinates.
        /// 
        /// Input region is double-buffered state, see wl_surface.commit.
        /// 
        /// wl_surface.set_input_region changes the pending input region.
        /// wl_surface.commit copies the pending region to the current region.
        /// Otherwise the pending and current regions are never changed,
        /// except cursor and icon surfaces are special cases, see
        /// wl_pointer.set_cursor and wl_data_device.start_drag.
        /// 
        /// The initial value for an input region is infinite. That means the
        /// whole surface will accept input. Setting the pending input region
        /// has copy semantics, and the wl_region object can be destroyed
        /// immediately. A NULL wl_region causes the input region to be set
        /// to infinite.
        fn set_input_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) ->();
        /// commit pending surface state
        ///
        /// Surface state (input, opaque, and damage regions, attached buffers,
        /// etc.) is double-buffered. Protocol requests modify the pending state,
        /// as opposed to the current state in use by the compositor. A commit
        /// request atomically applies all pending state, replacing the current
        /// state. After commit, the new pending state is as documented for each
        /// related request.
        /// 
        /// On commit, a pending wl_buffer is applied first, and all other state
        /// second. This means that all coordinates in double-buffered state are
        /// relative to the new wl_buffer coming into use, except for
        /// wl_surface.attach itself. If there is no pending wl_buffer, the
        /// coordinates are relative to the current surface contents.
        /// 
        /// All requests that need a commit to become effective are documented
        /// to affect double-buffered state.
        /// 
        /// Other interfaces may add further double-buffered surface state.
        fn commit(&self) ->();
        /// sets the buffer transformation
        ///
        /// This request sets an optional transformation on how the compositor
        /// interprets the contents of the buffer attached to the surface. The
        /// accepted values for the transform parameter are the values for
        /// wl_output.transform.
        /// 
        /// Buffer transform is double-buffered state, see wl_surface.commit.
        /// 
        /// A newly created surface has its buffer transformation set to normal.
        /// 
        /// wl_surface.set_buffer_transform changes the pending buffer
        /// transformation. wl_surface.commit copies the pending buffer
        /// transformation to the current one. Otherwise, the pending and current
        /// values are never changed.
        /// 
        /// The purpose of this request is to allow clients to render content
        /// according to the output transform, thus permitting the compositor to
        /// use certain optimizations even if the display is rotated. Using
        /// hardware overlays and scanning out a client buffer for fullscreen
        /// surfaces are examples of such optimizations. Those optimizations are
        /// highly dependent on the compositor implementation, so the use of this
        /// request should be considered on a case-by-case basis.
        /// 
        /// Note that if the transform value includes 90 or 270 degree rotation,
        /// the width of the buffer will become the surface height and the height
        /// of the buffer will become the surface width.
        /// 
        /// If transform is not one of the values from the
        /// wl_output.transform enum the invalid_transform protocol error
        /// is raised.
        ///
        /// Only available since version 2 of the interface
        fn set_buffer_transform(&self, transform: super::wl_output::Transform) ->();
        /// sets the buffer scaling factor
        ///
        /// This request sets an optional scaling factor on how the compositor
        /// interprets the contents of the buffer attached to the window.
        /// 
        /// Buffer scale is double-buffered state, see wl_surface.commit.
        /// 
        /// A newly created surface has its buffer scale set to 1.
        /// 
        /// wl_surface.set_buffer_scale changes the pending buffer scale.
        /// wl_surface.commit copies the pending buffer scale to the current one.
        /// Otherwise, the pending and current values are never changed.
        /// 
        /// The purpose of this request is to allow clients to supply higher
        /// resolution buffer data for use on high resolution outputs. It is
        /// intended that you pick the same buffer scale as the scale of the
        /// output that the surface is displayed on. This means the compositor
        /// can avoid scaling when rendering the surface on that output.
        /// 
        /// Note that if the scale is larger than 1, then you have to attach
        /// a buffer that is larger (by a factor of scale in each dimension)
        /// than the desired surface size.
        /// 
        /// If scale is not positive the invalid_scale protocol error is
        /// raised.
        ///
        /// Only available since version 3 of the interface
        fn set_buffer_scale(&self, scale: i32) ->();
        /// mark part of the surface damaged using buffer coordinates
        ///
        /// This request is used to describe the regions where the pending
        /// buffer is different from the current surface contents, and where
        /// the surface therefore needs to be repainted. The compositor
        /// ignores the parts of the damage that fall outside of the surface.
        /// 
        /// Damage is double-buffered state, see wl_surface.commit.
        /// 
        /// The damage rectangle is specified in buffer coordinates,
        /// where x and y specify the upper left corner of the damage rectangle.
        /// 
        /// The initial value for pending damage is empty: no damage.
        /// wl_surface.damage_buffer adds pending damage: the new pending
        /// damage is the union of old pending damage and the given rectangle.
        /// 
        /// wl_surface.commit assigns pending damage as the current damage,
        /// and clears pending damage. The server will clear the current
        /// damage as it repaints the surface.
        /// 
        /// This request differs from wl_surface.damage in only one way - it
        /// takes damage in buffer coordinates instead of surface-local
        /// coordinates. While this generally is more intuitive than surface
        /// coordinates, it is especially desirable when using wp_viewport
        /// or when a drawing library (like EGL) is unaware of buffer scale
        /// and buffer transform.
        /// 
        /// Note: Because buffer transformation changes and damage requests may
        /// be interleaved in the protocol stream, it is impossible to determine
        /// the actual mapping between surface and buffer damage until
        /// wl_surface.commit time. Therefore, compositors wishing to take both
        /// kinds of damage into account will have to accumulate damage from the
        /// two requests separately and only transform from one to the other
        /// after receiving the wl_surface.commit.
        ///
        /// Only available since version 4 of the interface
        fn damage_buffer(&self, x: i32, y: i32, width: i32, height: i32) ->();
    }

    impl RequestsTrait for Proxy<WlSurface> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn attach(&self, buffer: Option<&Proxy<super::wl_buffer::WlBuffer>>, x: i32, y: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Attach {
                buffer : buffer.map(|o| o.clone()),
                x: x,
                y: y,
            };
            self.send(msg);
        }

        fn damage(&self, x: i32, y: i32, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Damage {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.send(msg);
        }

        fn frame(&self) ->Result<NewProxy<super::wl_callback::WlCallback>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_callback_newproxy = self.child::<super::wl_callback::WlCallback>();
            let msg = Request::Frame {
                callback: unsafe { Proxy::<super::wl_callback::WlCallback>::from_c_ptr(_arg_callback_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_callback_newproxy)
        }

        fn set_opaque_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetOpaqueRegion {
                region : region.map(|o| o.clone()),
            };
            self.send(msg);
        }

        fn set_input_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetInputRegion {
                region : region.map(|o| o.clone()),
            };
            self.send(msg);
        }

        fn commit(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Commit;
            self.send(msg);
        }

        fn set_buffer_transform(&self, transform: super::wl_output::Transform) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetBufferTransform {
                transform: transform,
            };
            self.send(msg);
        }

        fn set_buffer_scale(&self, scale: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetBufferScale {
                scale: scale,
            };
            self.send(msg);
        }

        fn damage_buffer(&self, x: i32, y: i32, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::DamageBuffer {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.send(msg);
        }

    }
}

pub mod wl_seat {
    //! group of input devices
    //!
    //! A seat is a group of keyboards, pointer and touch devices. This
    //! object is published as a global during start up, or when such a
    //! device is hot plugged.  A seat typically has a pointer and
    //! maintains a keyboard focus and a pointer focus.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    bitflags! {
        /// seat capability bitmask
        ///
        /// This is a bitmask of capabilities this seat has; if a member is
        /// set, then it is present on the seat.
        pub struct Capability: u32 {
            /// the seat has pointer devices
            const Pointer = 1;
            /// the seat has one or more keyboards
            const Keyboard = 2;
            /// the seat has touch devices
            const Touch = 4;
        }
    }
    impl Capability {
        pub fn from_raw(n: u32) -> Option<Capability> {
            Some(Capability::from_bits_truncate(n))

        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }

    pub enum Request {
        /// return pointer object
        ///
        /// The ID provided will be initialized to the wl_pointer interface
        /// for this seat.
        /// 
        /// This request only takes effect if the seat has the pointer
        /// capability, or has had the pointer capability in the past.
        /// It is a protocol violation to issue this request on a seat that has
        /// never had the pointer capability.
        GetPointer {id: Proxy<super::wl_pointer::WlPointer>, },
        /// return keyboard object
        ///
        /// The ID provided will be initialized to the wl_keyboard interface
        /// for this seat.
        /// 
        /// This request only takes effect if the seat has the keyboard
        /// capability, or has had the keyboard capability in the past.
        /// It is a protocol violation to issue this request on a seat that has
        /// never had the keyboard capability.
        GetKeyboard {id: Proxy<super::wl_keyboard::WlKeyboard>, },
        /// return touch object
        ///
        /// The ID provided will be initialized to the wl_touch interface
        /// for this seat.
        /// 
        /// This request only takes effect if the seat has the touch
        /// capability, or has had the touch capability in the past.
        /// It is a protocol violation to issue this request on a seat that has
        /// never had the touch capability.
        GetTouch {id: Proxy<super::wl_touch::WlTouch>, },
        /// release the seat object
        ///
        /// Using this request a client can tell the server that it is not going to
        /// use the seat object anymore.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        ///
        /// Only available since version 5 of the interface
        Release,
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::GetPointer { id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                },
                Request::GetKeyboard { id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
                Request::GetTouch { id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(2, &mut _args_array)
                },
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// seat capabilities changed
        ///
        /// This is emitted whenever a seat gains or loses the pointer,
        /// keyboard or touch capabilities.  The argument is a capability
        /// enum containing the complete set of capabilities this seat has.
        /// 
        /// When the pointer capability is added, a client may create a
        /// wl_pointer object using the wl_seat.get_pointer request. This object
        /// will receive pointer events until the capability is removed in the
        /// future.
        /// 
        /// When the pointer capability is removed, a client should destroy the
        /// wl_pointer objects associated with the seat where the capability was
        /// removed, using the wl_pointer.release request. No further pointer
        /// events will be received on these objects.
        /// 
        /// In some compositors, if a seat regains the pointer capability and a
        /// client has a previously obtained wl_pointer object of version 4 or
        /// less, that object may start sending pointer events again. This
        /// behavior is considered a misinterpretation of the intended behavior
        /// and must not be relied upon by the client. wl_pointer objects of
        /// version 5 or later must not send events if created before the most
        /// recent event notifying the client of an added pointer capability.
        /// 
        /// The above behavior also applies to wl_keyboard and wl_touch with the
        /// keyboard and touch capabilities, respectively.
        Capabilities {capabilities: Capability, },
        /// unique identifier for this seat
        ///
        /// In a multiseat configuration this can be used by the client to help
        /// identify which physical devices the seat represents. Based on
        /// the seat configuration used by the compositor.
        ///
        /// Only available since version 2 of the interface
        Name {name: String, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Capabilities {
                        capabilities: Capability::from_raw(_args[0].u).ok_or(())?,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Name {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlSeat;

    impl Interface for WlSeat {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_seat";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_seat_interface }
        }
    }

    pub trait RequestsTrait {
        /// return pointer object
        ///
        /// The ID provided will be initialized to the wl_pointer interface
        /// for this seat.
        /// 
        /// This request only takes effect if the seat has the pointer
        /// capability, or has had the pointer capability in the past.
        /// It is a protocol violation to issue this request on a seat that has
        /// never had the pointer capability.
        fn get_pointer(&self) ->Result<NewProxy<super::wl_pointer::WlPointer>, ()>;
        /// return keyboard object
        ///
        /// The ID provided will be initialized to the wl_keyboard interface
        /// for this seat.
        /// 
        /// This request only takes effect if the seat has the keyboard
        /// capability, or has had the keyboard capability in the past.
        /// It is a protocol violation to issue this request on a seat that has
        /// never had the keyboard capability.
        fn get_keyboard(&self) ->Result<NewProxy<super::wl_keyboard::WlKeyboard>, ()>;
        /// return touch object
        ///
        /// The ID provided will be initialized to the wl_touch interface
        /// for this seat.
        /// 
        /// This request only takes effect if the seat has the touch
        /// capability, or has had the touch capability in the past.
        /// It is a protocol violation to issue this request on a seat that has
        /// never had the touch capability.
        fn get_touch(&self) ->Result<NewProxy<super::wl_touch::WlTouch>, ()>;
        /// release the seat object
        ///
        /// Using this request a client can tell the server that it is not going to
        /// use the seat object anymore.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        ///
        /// Only available since version 5 of the interface
        fn release(&self) ->();
    }

    impl RequestsTrait for Proxy<WlSeat> {
        fn get_pointer(&self) ->Result<NewProxy<super::wl_pointer::WlPointer>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_pointer::WlPointer>();
            let msg = Request::GetPointer {
                id: unsafe { Proxy::<super::wl_pointer::WlPointer>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn get_keyboard(&self) ->Result<NewProxy<super::wl_keyboard::WlKeyboard>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_keyboard::WlKeyboard>();
            let msg = Request::GetKeyboard {
                id: unsafe { Proxy::<super::wl_keyboard::WlKeyboard>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn get_touch(&self) ->Result<NewProxy<super::wl_touch::WlTouch>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_touch::WlTouch>();
            let msg = Request::GetTouch {
                id: unsafe { Proxy::<super::wl_touch::WlTouch>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn release(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Release;
            self.send(msg);
        }

    }
}

pub mod wl_pointer {
    //! pointer input device
    //!
    //! The wl_pointer interface represents one or more input devices,
    //! such as mice, which control the pointer location and pointer_focus
    //! of a seat.
    //! 
    //! The wl_pointer interface generates motion, enter and leave
    //! events for the surfaces that the pointer is located over,
    //! and button and axis events for button presses, button releases
    //! and scrolling.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// given wl_surface has another role
        Role = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// physical button state
    ///
    /// Describes the physical state of a button that produced the button
    /// event.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ButtonState {
        /// the button is not pressed
        Released = 0,
        /// the button is pressed
        Pressed = 1,
    }
    impl ButtonState {
        pub fn from_raw(n: u32) -> Option<ButtonState> {
            match n {
                0 => Some(ButtonState::Released),
                1 => Some(ButtonState::Pressed),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// axis types
    ///
    /// Describes the axis types of scroll events.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Axis {
        /// vertical axis
        VerticalScroll = 0,
        /// horizontal axis
        HorizontalScroll = 1,
    }
    impl Axis {
        pub fn from_raw(n: u32) -> Option<Axis> {
            match n {
                0 => Some(Axis::VerticalScroll),
                1 => Some(Axis::HorizontalScroll),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// axis source types
    ///
    /// Describes the source types for axis events. This indicates to the
    /// client how an axis event was physically generated; a client may
    /// adjust the user interface accordingly. For example, scroll events
    /// from a "finger" source may be in a smooth coordinate space with
    /// kinetic scrolling whereas a "wheel" source may be in discrete steps
    /// of a number of lines.
    /// 
    /// The "continuous" axis source is a device generating events in a
    /// continuous coordinate space, but using something other than a
    /// finger. One example for this source is button-based scrolling where
    /// the vertical motion of a device is converted to scroll events while
    /// a button is held down.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum AxisSource {
        /// a physical wheel
        Wheel = 0,
        /// finger on a touch surface
        Finger = 1,
        /// continuous coordinate space
        Continuous = 2,
    }
    impl AxisSource {
        pub fn from_raw(n: u32) -> Option<AxisSource> {
            match n {
                0 => Some(AxisSource::Wheel),
                1 => Some(AxisSource::Finger),
                2 => Some(AxisSource::Continuous),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// set the pointer surface
        ///
        /// Set the pointer surface, i.e., the surface that contains the
        /// pointer image (cursor). This request gives the surface the role
        /// of a cursor. If the surface already has another role, it raises
        /// a protocol error.
        /// 
        /// The cursor actually changes only if the pointer
        /// focus for this device is one of the requesting client's surfaces
        /// or the surface parameter is the current pointer surface. If
        /// there was a previous surface set with this request it is
        /// replaced. If surface is NULL, the pointer image is hidden.
        /// 
        /// The parameters hotspot_x and hotspot_y define the position of
        /// the pointer surface relative to the pointer location. Its
        /// top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
        /// where (x, y) are the coordinates of the pointer location, in
        /// surface-local coordinates.
        /// 
        /// On surface.attach requests to the pointer surface, hotspot_x
        /// and hotspot_y are decremented by the x and y parameters
        /// passed to the request. Attach must be confirmed by
        /// wl_surface.commit as usual.
        /// 
        /// The hotspot can also be updated by passing the currently set
        /// pointer surface to this request with new values for hotspot_x
        /// and hotspot_y.
        /// 
        /// The current and pending input regions of the wl_surface are
        /// cleared, and wl_surface.set_input_region is ignored until the
        /// wl_surface is no longer used as the cursor. When the use as a
        /// cursor ends, the current and pending input regions become
        /// undefined, and the wl_surface is unmapped.
        SetCursor {serial: u32, surface: Option<Proxy<super::wl_surface::WlSurface>>, hotspot_x: i32, hotspot_y: i32, },
        /// release the pointer object
        ///
        /// Using this request a client can tell the server that it is not going to
        /// use the pointer object anymore.
        /// 
        /// This request destroys the pointer proxy object, so clients must not call
        /// wl_pointer_destroy() after using this request.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        ///
        /// Only available since version 3 of the interface
        Release,
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::SetCursor { serial, surface, hotspot_x, hotspot_y, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    _args_array[1].o = surface.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[2].i = hotspot_x;
                    _args_array[3].i = hotspot_y;
                    f(0, &mut _args_array)
                },
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// enter event
        ///
        /// Notification that this seat's pointer is focused on a certain
        /// surface.
        /// 
        /// When a seat's focus enters a surface, the pointer image
        /// is undefined and a client should respond to this event by setting
        /// an appropriate pointer image with the set_cursor request.
        Enter {serial: u32, surface: Proxy<super::wl_surface::WlSurface>, surface_x: f64, surface_y: f64, },
        /// leave event
        ///
        /// Notification that this seat's pointer is no longer focused on
        /// a certain surface.
        /// 
        /// The leave notification is sent before the enter notification
        /// for the new focus.
        Leave {serial: u32, surface: Proxy<super::wl_surface::WlSurface>, },
        /// pointer motion event
        ///
        /// Notification of pointer location change. The arguments
        /// surface_x and surface_y are the location relative to the
        /// focused surface.
        Motion {time: u32, surface_x: f64, surface_y: f64, },
        /// pointer button event
        ///
        /// Mouse button click and release notifications.
        /// 
        /// The location of the click is given by the last motion or
        /// enter event.
        /// The time argument is a timestamp with millisecond
        /// granularity, with an undefined base.
        Button {serial: u32, time: u32, button: u32, state: ButtonState, },
        /// axis event
        ///
        /// Scroll and other axis notifications.
        /// 
        /// For scroll events (vertical and horizontal scroll axes), the
        /// value parameter is the length of a vector along the specified
        /// axis in a coordinate space identical to those of motion events,
        /// representing a relative movement along the specified axis.
        /// 
        /// For devices that support movements non-parallel to axes multiple
        /// axis events will be emitted.
        /// 
        /// When applicable, for example for touch pads, the server can
        /// choose to emit scroll events where the motion vector is
        /// equivalent to a motion event vector.
        /// 
        /// When applicable, a client can transform its content relative to the
        /// scroll distance.
        Axis {time: u32, axis: Axis, value: f64, },
        /// end of a pointer event sequence
        ///
        /// Indicates the end of a set of events that logically belong together.
        /// A client is expected to accumulate the data in all events within the
        /// frame before proceeding.
        /// 
        /// All wl_pointer events before a wl_pointer.frame event belong
        /// logically together. For example, in a diagonal scroll motion the
        /// compositor will send an optional wl_pointer.axis_source event, two
        /// wl_pointer.axis events (horizontal and vertical) and finally a
        /// wl_pointer.frame event. The client may use this information to
        /// calculate a diagonal vector for scrolling.
        /// 
        /// When multiple wl_pointer.axis events occur within the same frame,
        /// the motion vector is the combined motion of all events.
        /// When a wl_pointer.axis and a wl_pointer.axis_stop event occur within
        /// the same frame, this indicates that axis movement in one axis has
        /// stopped but continues in the other axis.
        /// When multiple wl_pointer.axis_stop events occur within the same
        /// frame, this indicates that these axes stopped in the same instance.
        /// 
        /// A wl_pointer.frame event is sent for every logical event group,
        /// even if the group only contains a single wl_pointer event.
        /// Specifically, a client may get a sequence: motion, frame, button,
        /// frame, axis, frame, axis_stop, frame.
        /// 
        /// The wl_pointer.enter and wl_pointer.leave events are logical events
        /// generated by the compositor and not the hardware. These events are
        /// also grouped by a wl_pointer.frame. When a pointer moves from one
        /// surface to another, a compositor should group the
        /// wl_pointer.leave event within the same wl_pointer.frame.
        /// However, a client must not rely on wl_pointer.leave and
        /// wl_pointer.enter being in the same wl_pointer.frame.
        /// Compositor-specific policies may require the wl_pointer.leave and
        /// wl_pointer.enter event being split across multiple wl_pointer.frame
        /// groups.
        ///
        /// Only available since version 5 of the interface
        Frame,
        /// axis source event
        ///
        /// Source information for scroll and other axes.
        /// 
        /// This event does not occur on its own. It is sent before a
        /// wl_pointer.frame event and carries the source information for
        /// all events within that frame.
        /// 
        /// The source specifies how this event was generated. If the source is
        /// wl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be
        /// sent when the user lifts the finger off the device.
        /// 
        /// If the source is wl_pointer axis_source.wheel or
        /// wl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may
        /// or may not be sent. Whether a compositor sends an axis_stop event
        /// for these sources is hardware-specific and implementation-dependent;
        /// clients must not rely on receiving an axis_stop event for these
        /// scroll sources and should treat scroll sequences from these scroll
        /// sources as unterminated by default.
        /// 
        /// This event is optional. If the source is unknown for a particular
        /// axis event sequence, no event is sent.
        /// Only one wl_pointer.axis_source event is permitted per frame.
        /// 
        /// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
        /// not guaranteed.
        ///
        /// Only available since version 5 of the interface
        AxisSource {axis_source: AxisSource, },
        /// axis stop event
        ///
        /// Stop notification for scroll and other axes.
        /// 
        /// For some wl_pointer.axis_source types, a wl_pointer.axis_stop event
        /// is sent to notify a client that the axis sequence has terminated.
        /// This enables the client to implement kinetic scrolling.
        /// See the wl_pointer.axis_source documentation for information on when
        /// this event may be generated.
        /// 
        /// Any wl_pointer.axis events with the same axis_source after this
        /// event should be considered as the start of a new axis motion.
        /// 
        /// The timestamp is to be interpreted identical to the timestamp in the
        /// wl_pointer.axis event. The timestamp value may be the same as a
        /// preceding wl_pointer.axis event.
        ///
        /// Only available since version 5 of the interface
        AxisStop {time: u32, axis: Axis, },
        /// axis click event
        ///
        /// Discrete step information for scroll and other axes.
        /// 
        /// This event carries the axis value of the wl_pointer.axis event in
        /// discrete steps (e.g. mouse wheel clicks).
        /// 
        /// This event does not occur on its own, it is coupled with a
        /// wl_pointer.axis event that represents this axis value on a
        /// continuous scale. The protocol guarantees that each axis_discrete
        /// event is always followed by exactly one axis event with the same
        /// axis number within the same wl_pointer.frame. Note that the protocol
        /// allows for other events to occur between the axis_discrete and
        /// its coupled axis event, including other axis_discrete or axis
        /// events.
        /// 
        /// This event is optional; continuous scrolling devices
        /// like two-finger scrolling on touchpads do not have discrete
        /// steps and do not generate this event.
        /// 
        /// The discrete value carries the directional information. e.g. a value
        /// of -2 is two steps towards the negative direction of this axis.
        /// 
        /// The axis number is identical to the axis number in the associated
        /// axis event.
        /// 
        /// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
        /// not guaranteed.
        ///
        /// Only available since version 5 of the interface
        AxisDiscrete {axis: Axis, discrete: i32, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Enter {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[1].o as *mut _),
                        surface_x: (_args[2].f as f64)/256.,
                        surface_y: (_args[3].f as f64)/256.,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Leave {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[1].o as *mut _),
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Motion {
                        time: _args[0].u,
                        surface_x: (_args[1].f as f64)/256.,
                        surface_y: (_args[2].f as f64)/256.,
                }) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Button {
                        serial: _args[0].u,
                        time: _args[1].u,
                        button: _args[2].u,
                        state: ButtonState::from_raw(_args[3].u).ok_or(())?,
                }) },
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Axis {
                        time: _args[0].u,
                        axis: Axis::from_raw(_args[1].u).ok_or(())?,
                        value: (_args[2].f as f64)/256.,
                }) },
                5 => {
                    Ok(Event::Frame) },
                6 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::AxisSource {
                        axis_source: AxisSource::from_raw(_args[0].u).ok_or(())?,
                }) },
                7 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::AxisStop {
                        time: _args[0].u,
                        axis: Axis::from_raw(_args[1].u).ok_or(())?,
                }) },
                8 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::AxisDiscrete {
                        axis: Axis::from_raw(_args[0].u).ok_or(())?,
                        discrete: _args[1].i,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlPointer;

    impl Interface for WlPointer {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_pointer";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_pointer_interface }
        }
    }

    pub trait RequestsTrait {
        /// set the pointer surface
        ///
        /// Set the pointer surface, i.e., the surface that contains the
        /// pointer image (cursor). This request gives the surface the role
        /// of a cursor. If the surface already has another role, it raises
        /// a protocol error.
        /// 
        /// The cursor actually changes only if the pointer
        /// focus for this device is one of the requesting client's surfaces
        /// or the surface parameter is the current pointer surface. If
        /// there was a previous surface set with this request it is
        /// replaced. If surface is NULL, the pointer image is hidden.
        /// 
        /// The parameters hotspot_x and hotspot_y define the position of
        /// the pointer surface relative to the pointer location. Its
        /// top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
        /// where (x, y) are the coordinates of the pointer location, in
        /// surface-local coordinates.
        /// 
        /// On surface.attach requests to the pointer surface, hotspot_x
        /// and hotspot_y are decremented by the x and y parameters
        /// passed to the request. Attach must be confirmed by
        /// wl_surface.commit as usual.
        /// 
        /// The hotspot can also be updated by passing the currently set
        /// pointer surface to this request with new values for hotspot_x
        /// and hotspot_y.
        /// 
        /// The current and pending input regions of the wl_surface are
        /// cleared, and wl_surface.set_input_region is ignored until the
        /// wl_surface is no longer used as the cursor. When the use as a
        /// cursor ends, the current and pending input regions become
        /// undefined, and the wl_surface is unmapped.
        fn set_cursor(&self, serial: u32, surface: Option<&Proxy<super::wl_surface::WlSurface>>, hotspot_x: i32, hotspot_y: i32) ->();
        /// release the pointer object
        ///
        /// Using this request a client can tell the server that it is not going to
        /// use the pointer object anymore.
        /// 
        /// This request destroys the pointer proxy object, so clients must not call
        /// wl_pointer_destroy() after using this request.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        ///
        /// Only available since version 3 of the interface
        fn release(&self) ->();
    }

    impl RequestsTrait for Proxy<WlPointer> {
        fn set_cursor(&self, serial: u32, surface: Option<&Proxy<super::wl_surface::WlSurface>>, hotspot_x: i32, hotspot_y: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetCursor {
                serial: serial,
                surface : surface.map(|o| o.clone()),
                hotspot_x: hotspot_x,
                hotspot_y: hotspot_y,
            };
            self.send(msg);
        }

        fn release(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Release;
            self.send(msg);
        }

    }
}

pub mod wl_keyboard {
    //! keyboard input device
    //!
    //! The wl_keyboard interface represents one or more keyboards
    //! associated with a seat.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// keyboard mapping format
    ///
    /// This specifies the format of the keymap provided to the
    /// client with the wl_keyboard.keymap event.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum KeymapFormat {
        /// no keymap; client must understand how to interpret the raw keycode
        NoKeymap = 0,
        /// libxkbcommon compatible; to determine the xkb keycode, clients must add 8 to the key event keycode
        XkbV1 = 1,
    }
    impl KeymapFormat {
        pub fn from_raw(n: u32) -> Option<KeymapFormat> {
            match n {
                0 => Some(KeymapFormat::NoKeymap),
                1 => Some(KeymapFormat::XkbV1),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// physical key state
    ///
    /// Describes the physical state of a key that produced the key event.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum KeyState {
        /// key is not pressed
        Released = 0,
        /// key is pressed
        Pressed = 1,
    }
    impl KeyState {
        pub fn from_raw(n: u32) -> Option<KeyState> {
            match n {
                0 => Some(KeyState::Released),
                1 => Some(KeyState::Pressed),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// release the keyboard object
        ///
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        ///
        /// Only available since version 3 of the interface
        Release,
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// keyboard mapping
        ///
        /// This event provides a file descriptor to the client which can be
        /// memory-mapped to provide a keyboard mapping description.
        Keymap {format: KeymapFormat, fd: ::std::os::unix::io::RawFd, size: u32, },
        /// enter event
        ///
        /// Notification that this seat's keyboard focus is on a certain
        /// surface.
        Enter {serial: u32, surface: Proxy<super::wl_surface::WlSurface>, keys: Vec<u8>, },
        /// leave event
        ///
        /// Notification that this seat's keyboard focus is no longer on
        /// a certain surface.
        /// 
        /// The leave notification is sent before the enter notification
        /// for the new focus.
        Leave {serial: u32, surface: Proxy<super::wl_surface::WlSurface>, },
        /// key event
        ///
        /// A key was pressed or released.
        /// The time argument is a timestamp with millisecond
        /// granularity, with an undefined base.
        Key {serial: u32, time: u32, key: u32, state: KeyState, },
        /// modifier and group state
        ///
        /// Notifies clients that the modifier and/or group state has
        /// changed, and it should update its local state.
        Modifiers {serial: u32, mods_depressed: u32, mods_latched: u32, mods_locked: u32, group: u32, },
        /// repeat rate and delay
        ///
        /// Informs the client about the keyboard's repeat rate and delay.
        /// 
        /// This event is sent as soon as the wl_keyboard object has been created,
        /// and is guaranteed to be received by the client before any key press
        /// event.
        /// 
        /// Negative values for either rate or delay are illegal. A rate of zero
        /// will disable any repeating (regardless of the value of delay).
        /// 
        /// This event can be sent later on as well with a new value if necessary,
        /// so clients should continue listening for the event past the creation
        /// of wl_keyboard.
        ///
        /// Only available since version 4 of the interface
        RepeatInfo {rate: i32, delay: i32, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Keymap {
                        format: KeymapFormat::from_raw(_args[0].u).ok_or(())?,
                        fd: _args[1].h,
                        size: _args[2].u,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Enter {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[1].o as *mut _),
                        keys: { let array = &*_args[2].a; ::std::slice::from_raw_parts(array.data as *const u8, array.size).to_owned() },
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Leave {
                        serial: _args[0].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[1].o as *mut _),
                }) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Key {
                        serial: _args[0].u,
                        time: _args[1].u,
                        key: _args[2].u,
                        state: KeyState::from_raw(_args[3].u).ok_or(())?,
                }) },
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 5);
                    Ok(Event::Modifiers {
                        serial: _args[0].u,
                        mods_depressed: _args[1].u,
                        mods_latched: _args[2].u,
                        mods_locked: _args[3].u,
                        group: _args[4].u,
                }) },
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::RepeatInfo {
                        rate: _args[0].i,
                        delay: _args[1].i,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlKeyboard;

    impl Interface for WlKeyboard {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_keyboard";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_keyboard_interface }
        }
    }

    pub trait RequestsTrait {
        /// release the keyboard object
        ///
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        ///
        /// Only available since version 3 of the interface
        fn release(&self) ->();
    }

    impl RequestsTrait for Proxy<WlKeyboard> {
        fn release(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Release;
            self.send(msg);
        }

    }
}

pub mod wl_touch {
    //! touchscreen input device
    //!
    //! The wl_touch interface represents a touchscreen
    //! associated with a seat.
    //! 
    //! Touch interactions can consist of one or more contacts.
    //! For each contact, a series of events is generated, starting
    //! with a down event, followed by zero or more motion events,
    //! and ending with an up event. Events relating to the same
    //! contact point can be identified by the ID of the sequence.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// release the touch object
        ///
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        ///
        /// Only available since version 3 of the interface
        Release,
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// touch down event and beginning of a touch sequence
        ///
        /// A new touch point has appeared on the surface. This touch point is
        /// assigned a unique ID. Future events from this touch point reference
        /// this ID. The ID ceases to be valid after a touch up event and may be
        /// reused in the future.
        Down {serial: u32, time: u32, surface: Proxy<super::wl_surface::WlSurface>, id: i32, x: f64, y: f64, },
        /// end of a touch event sequence
        ///
        /// The touch point has disappeared. No further events will be sent for
        /// this touch point and the touch point's ID is released and may be
        /// reused in a future touch down event.
        Up {serial: u32, time: u32, id: i32, },
        /// update of touch point coordinates
        ///
        /// A touch point has changed coordinates.
        Motion {time: u32, id: i32, x: f64, y: f64, },
        /// end of touch frame event
        ///
        /// Indicates the end of a contact point list.
        Frame,
        /// touch session cancelled
        ///
        /// Sent if the compositor decides the touch stream is a global
        /// gesture. No further events are sent to the clients from that
        /// particular gesture. Touch cancellation applies to all touch points
        /// currently active on this client's surface. The client is
        /// responsible for finalizing the touch points, future touch points on
        /// this surface may reuse the touch point ID.
        Cancel,
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 6);
                    Ok(Event::Down {
                        serial: _args[0].u,
                        time: _args[1].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[2].o as *mut _),
                        id: _args[3].i,
                        x: (_args[4].f as f64)/256.,
                        y: (_args[5].f as f64)/256.,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Up {
                        serial: _args[0].u,
                        time: _args[1].u,
                        id: _args[2].i,
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Motion {
                        time: _args[0].u,
                        id: _args[1].i,
                        x: (_args[2].f as f64)/256.,
                        y: (_args[3].f as f64)/256.,
                }) },
                3 => {
                    Ok(Event::Frame) },
                4 => {
                    Ok(Event::Cancel) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlTouch;

    impl Interface for WlTouch {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_touch";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_touch_interface }
        }
    }

    pub trait RequestsTrait {
        /// release the touch object
        ///
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        ///
        /// Only available since version 3 of the interface
        fn release(&self) ->();
    }

    impl RequestsTrait for Proxy<WlTouch> {
        fn release(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Release;
            self.send(msg);
        }

    }
}

pub mod wl_output {
    //! compositor output region
    //!
    //! An output describes part of the compositor geometry.  The
    //! compositor works in the 'compositor coordinate system' and an
    //! output corresponds to a rectangular area in that space that is
    //! actually visible.  This typically corresponds to a monitor that
    //! displays part of the compositor space.  This object is published
    //! as global during start up, or when a monitor is hotplugged.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// subpixel geometry information
    ///
    /// This enumeration describes how the physical
    /// pixels on an output are laid out.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Subpixel {
        /// unknown geometry
        Unknown = 0,
        /// no geometry
        None = 1,
        /// horizontal RGB
        HorizontalRgb = 2,
        /// horizontal BGR
        HorizontalBgr = 3,
        /// vertical RGB
        VerticalRgb = 4,
        /// vertical BGR
        VerticalBgr = 5,
    }
    impl Subpixel {
        pub fn from_raw(n: u32) -> Option<Subpixel> {
            match n {
                0 => Some(Subpixel::Unknown),
                1 => Some(Subpixel::None),
                2 => Some(Subpixel::HorizontalRgb),
                3 => Some(Subpixel::HorizontalBgr),
                4 => Some(Subpixel::VerticalRgb),
                5 => Some(Subpixel::VerticalBgr),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// transform from framebuffer to output
    ///
    /// This describes the transform that a compositor will apply to a
    /// surface to compensate for the rotation or mirroring of an
    /// output device.
    /// 
    /// The flipped values correspond to an initial flip around a
    /// vertical axis followed by rotation.
    /// 
    /// The purpose is mainly to allow clients to render accordingly and
    /// tell the compositor, so that for fullscreen surfaces, the
    /// compositor will still be able to scan out directly from client
    /// surfaces.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Transform {
        /// no transform
        Normal = 0,
        /// 90 degrees counter-clockwise
        _90 = 1,
        /// 180 degrees counter-clockwise
        _180 = 2,
        /// 270 degrees counter-clockwise
        _270 = 3,
        /// 180 degree flip around a vertical axis
        Flipped = 4,
        /// flip and rotate 90 degrees counter-clockwise
        Flipped90 = 5,
        /// flip and rotate 180 degrees counter-clockwise
        Flipped180 = 6,
        /// flip and rotate 270 degrees counter-clockwise
        Flipped270 = 7,
    }
    impl Transform {
        pub fn from_raw(n: u32) -> Option<Transform> {
            match n {
                0 => Some(Transform::Normal),
                1 => Some(Transform::_90),
                2 => Some(Transform::_180),
                3 => Some(Transform::_270),
                4 => Some(Transform::Flipped),
                5 => Some(Transform::Flipped90),
                6 => Some(Transform::Flipped180),
                7 => Some(Transform::Flipped270),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    bitflags! {
        /// mode information
        ///
        /// These flags describe properties of an output mode.
        /// They are used in the flags bitfield of the mode event.
        pub struct Mode: u32 {
            /// indicates this is the current mode
            const Current = 0x1;
            /// indicates this is the preferred mode
            const Preferred = 0x2;
        }
    }
    impl Mode {
        pub fn from_raw(n: u32) -> Option<Mode> {
            Some(Mode::from_bits_truncate(n))

        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }

    pub enum Request {
        /// release the output object
        ///
        /// Using this request a client can tell the server that it is not going to
        /// use the output object anymore.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        ///
        /// Only available since version 3 of the interface
        Release,
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Release => true,
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// properties of the output
        ///
        /// The geometry event describes geometric properties of the output.
        /// The event is sent when binding to the output object and whenever
        /// any of the properties change.
        Geometry {x: i32, y: i32, physical_width: i32, physical_height: i32, subpixel: Subpixel, make: String, model: String, transform: Transform, },
        /// advertise available modes for the output
        ///
        /// The mode event describes an available mode for the output.
        /// 
        /// The event is sent when binding to the output object and there
        /// will always be one mode, the current mode.  The event is sent
        /// again if an output changes mode, for the mode that is now
        /// current.  In other words, the current mode is always the last
        /// mode that was received with the current flag set.
        /// 
        /// The size of a mode is given in physical hardware units of
        /// the output device. This is not necessarily the same as
        /// the output size in the global compositor space. For instance,
        /// the output may be scaled, as described in wl_output.scale,
        /// or transformed, as described in wl_output.transform.
        Mode {flags: Mode, width: i32, height: i32, refresh: i32, },
        /// sent all information about output
        ///
        /// This event is sent after all other properties have been
        /// sent after binding to the output object and after any
        /// other property changes done after that. This allows
        /// changes to the output properties to be seen as
        /// atomic, even if they happen via multiple events.
        ///
        /// Only available since version 2 of the interface
        Done,
        /// output scaling properties
        ///
        /// This event contains scaling geometry information
        /// that is not in the geometry event. It may be sent after
        /// binding the output object or if the output scale changes
        /// later. If it is not sent, the client should assume a
        /// scale of 1.
        /// 
        /// A scale larger than 1 means that the compositor will
        /// automatically scale surface buffers by this amount
        /// when rendering. This is used for very high resolution
        /// displays where applications rendering at the native
        /// resolution would be too small to be legible.
        /// 
        /// It is intended that scaling aware clients track the
        /// current output of a surface, and if it is on a scaled
        /// output it should use wl_surface.set_buffer_scale with
        /// the scale of the output. That way the compositor can
        /// avoid scaling the surface, and the client can supply
        /// a higher detail image.
        ///
        /// Only available since version 2 of the interface
        Scale {factor: i32, },
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                0 => {
                    let _args = ::std::slice::from_raw_parts(args, 8);
                    Ok(Event::Geometry {
                        x: _args[0].i,
                        y: _args[1].i,
                        physical_width: _args[2].i,
                        physical_height: _args[3].i,
                        subpixel: Subpixel::from_raw(_args[4].i as u32).ok_or(())?,
                        make: ::std::ffi::CStr::from_ptr(_args[5].s).to_string_lossy().into_owned(),
                        model: ::std::ffi::CStr::from_ptr(_args[6].s).to_string_lossy().into_owned(),
                        transform: Transform::from_raw(_args[7].i as u32).ok_or(())?,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Mode {
                        flags: Mode::from_raw(_args[0].u).ok_or(())?,
                        width: _args[1].i,
                        height: _args[2].i,
                        refresh: _args[3].i,
                }) },
                2 => {
                    Ok(Event::Done) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Scale {
                        factor: _args[0].i,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlOutput;

    impl Interface for WlOutput {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_output";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_output_interface }
        }
    }

    pub trait RequestsTrait {
        /// release the output object
        ///
        /// Using this request a client can tell the server that it is not going to
        /// use the output object anymore.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        ///
        /// Only available since version 3 of the interface
        fn release(&self) ->();
    }

    impl RequestsTrait for Proxy<WlOutput> {
        fn release(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Release;
            self.send(msg);
        }

    }
}

pub mod wl_region {
    //! region interface
    //!
    //! A region object describes an area.
    //! 
    //! Region objects are used to describe the opaque and input
    //! regions of a surface.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy region
        ///
        /// Destroy the region.  This will invalidate the object ID.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// add rectangle to region
        ///
        /// Add the specified rectangle to the region.
        Add {x: i32, y: i32, width: i32, height: i32, },
        /// subtract rectangle from region
        ///
        /// Subtract the specified rectangle from the region.
        Subtract {x: i32, y: i32, width: i32, height: i32, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                },
                Request::Add { x, y, width, height, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(1, &mut _args_array)
                },
                Request::Subtract { x, y, width, height, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(2, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlRegion;

    impl Interface for WlRegion {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_region";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_region_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy region
        ///
        /// Destroy the region.  This will invalidate the object ID.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// add rectangle to region
        ///
        /// Add the specified rectangle to the region.
        fn add(&self, x: i32, y: i32, width: i32, height: i32) ->();
        /// subtract rectangle from region
        ///
        /// Subtract the specified rectangle from the region.
        fn subtract(&self, x: i32, y: i32, width: i32, height: i32) ->();
    }

    impl RequestsTrait for Proxy<WlRegion> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn add(&self, x: i32, y: i32, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Add {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.send(msg);
        }

        fn subtract(&self, x: i32, y: i32, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Subtract {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.send(msg);
        }

    }
}

pub mod wl_subcompositor {
    //! sub-surface compositing
    //!
    //! The global interface exposing sub-surface compositing capabilities.
    //! A wl_surface, that has sub-surfaces associated, is called the
    //! parent surface. Sub-surfaces can be arbitrarily nested and create
    //! a tree of sub-surfaces.
    //! 
    //! The root surface in a tree of sub-surfaces is the main
    //! surface. The main surface cannot be a sub-surface, because
    //! sub-surfaces must always have a parent.
    //! 
    //! A main surface with its sub-surfaces forms a (compound) window.
    //! For window management purposes, this set of wl_surface objects is
    //! to be considered as a single window, and it should also behave as
    //! such.
    //! 
    //! The aim of sub-surfaces is to offload some of the compositing work
    //! within a window from clients to the compositor. A prime example is
    //! a video player with decorations and video in separate wl_surface
    //! objects. This should allow the compositor to pass YUV video buffer
    //! processing to dedicated overlay hardware when possible.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// the to-be sub-surface is invalid
        BadSurface = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::BadSurface),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// unbind from the subcompositor interface
        ///
        /// Informs the server that the client will not be using this
        /// protocol object anymore. This does not affect any other
        /// objects, wl_subsurface objects included.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// give a surface the role sub-surface
        ///
        /// Create a sub-surface interface for the given surface, and
        /// associate it with the given parent surface. This turns a
        /// plain wl_surface into a sub-surface.
        /// 
        /// The to-be sub-surface must not already have another role, and it
        /// must not have an existing wl_subsurface object. Otherwise a protocol
        /// error is raised.
        GetSubsurface {id: Proxy<super::wl_subsurface::WlSubsurface>, surface: Proxy<super::wl_surface::WlSurface>, parent: Proxy<super::wl_surface::WlSurface>, },
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                },
                Request::GetSubsurface { id, surface, parent, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    _args_array[2].o = parent.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlSubcompositor;

    impl Interface for WlSubcompositor {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_subcompositor";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_subcompositor_interface }
        }
    }

    pub trait RequestsTrait {
        /// unbind from the subcompositor interface
        ///
        /// Informs the server that the client will not be using this
        /// protocol object anymore. This does not affect any other
        /// objects, wl_subsurface objects included.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// give a surface the role sub-surface
        ///
        /// Create a sub-surface interface for the given surface, and
        /// associate it with the given parent surface. This turns a
        /// plain wl_surface into a sub-surface.
        /// 
        /// The to-be sub-surface must not already have another role, and it
        /// must not have an existing wl_subsurface object. Otherwise a protocol
        /// error is raised.
        fn get_subsurface(&self, surface: &Proxy<super::wl_surface::WlSurface>, parent: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::wl_subsurface::WlSubsurface>, ()>;
    }

    impl RequestsTrait for Proxy<WlSubcompositor> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn get_subsurface(&self, surface: &Proxy<super::wl_surface::WlSurface>, parent: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::wl_subsurface::WlSubsurface>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wl_subsurface::WlSubsurface>();
            let msg = Request::GetSubsurface {
                id: unsafe { Proxy::<super::wl_subsurface::WlSubsurface>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                surface: surface.clone(),
                parent: parent.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod wl_subsurface {
    //! sub-surface interface to a wl_surface
    //!
    //! An additional interface to a wl_surface object, which has been
    //! made a sub-surface. A sub-surface has one parent surface. A
    //! sub-surface's size and position are not limited to that of the parent.
    //! Particularly, a sub-surface is not automatically clipped to its
    //! parent's area.
    //! 
    //! A sub-surface becomes mapped, when a non-NULL wl_buffer is applied
    //! and the parent surface is mapped. The order of which one happens
    //! first is irrelevant. A sub-surface is hidden if the parent becomes
    //! hidden, or if a NULL wl_buffer is applied. These rules apply
    //! recursively through the tree of surfaces.
    //! 
    //! The behaviour of a wl_surface.commit request on a sub-surface
    //! depends on the sub-surface's mode. The possible modes are
    //! synchronized and desynchronized, see methods
    //! wl_subsurface.set_sync and wl_subsurface.set_desync. Synchronized
    //! mode caches the wl_surface state to be applied when the parent's
    //! state gets applied, and desynchronized mode applies the pending
    //! wl_surface state directly. A sub-surface is initially in the
    //! synchronized mode.
    //! 
    //! Sub-surfaces have also other kind of state, which is managed by
    //! wl_subsurface requests, as opposed to wl_surface requests. This
    //! state includes the sub-surface position relative to the parent
    //! surface (wl_subsurface.set_position), and the stacking order of
    //! the parent and its sub-surfaces (wl_subsurface.place_above and
    //! .place_below). This state is applied when the parent surface's
    //! wl_surface state is applied, regardless of the sub-surface's mode.
    //! As the exception, set_sync and set_desync are effective immediately.
    //! 
    //! The main surface can be thought to be always in desynchronized mode,
    //! since it does not have a parent in the sub-surfaces sense.
    //! 
    //! Even if a sub-surface is in desynchronized mode, it will behave as
    //! in synchronized mode, if its parent surface behaves as in
    //! synchronized mode. This rule is applied recursively throughout the
    //! tree of surfaces. This means, that one can set a sub-surface into
    //! synchronized mode, and then assume that all its child and grand-child
    //! sub-surfaces are synchronized, too, without explicitly setting them.
    //! 
    //! If the wl_surface associated with the wl_subsurface is destroyed, the
    //! wl_subsurface object becomes inert. Note, that destroying either object
    //! takes effect immediately. If you need to synchronize the removal
    //! of a sub-surface to the parent surface update, unmap the sub-surface
    //! first by attaching a NULL wl_buffer, update parent, and then destroy
    //! the sub-surface.
    //! 
    //! If the parent wl_surface object is destroyed, the sub-surface is
    //! unmapped.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// wl_surface is not a sibling or the parent
        BadSurface = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::BadSurface),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// remove sub-surface interface
        ///
        /// The sub-surface interface is removed from the wl_surface object
        /// that was turned into a sub-surface with a
        /// wl_subcompositor.get_subsurface request. The wl_surface's association
        /// to the parent is deleted, and the wl_surface loses its role as
        /// a sub-surface. The wl_surface is unmapped.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// reposition the sub-surface
        ///
        /// This schedules a sub-surface position change.
        /// The sub-surface will be moved so that its origin (top left
        /// corner pixel) will be at the location x, y of the parent surface
        /// coordinate system. The coordinates are not restricted to the parent
        /// surface area. Negative values are allowed.
        /// 
        /// The scheduled coordinates will take effect whenever the state of the
        /// parent surface is applied. When this happens depends on whether the
        /// parent surface is in synchronized mode or not. See
        /// wl_subsurface.set_sync and wl_subsurface.set_desync for details.
        /// 
        /// If more than one set_position request is invoked by the client before
        /// the commit of the parent surface, the position of a new request always
        /// replaces the scheduled position from any previous request.
        /// 
        /// The initial position is 0, 0.
        SetPosition {x: i32, y: i32, },
        /// restack the sub-surface
        ///
        /// This sub-surface is taken from the stack, and put back just
        /// above the reference surface, changing the z-order of the sub-surfaces.
        /// The reference surface must be one of the sibling surfaces, or the
        /// parent surface. Using any other surface, including this sub-surface,
        /// will cause a protocol error.
        /// 
        /// The z-order is double-buffered. Requests are handled in order and
        /// applied immediately to a pending state. The final pending state is
        /// copied to the active state the next time the state of the parent
        /// surface is applied. When this happens depends on whether the parent
        /// surface is in synchronized mode or not. See wl_subsurface.set_sync and
        /// wl_subsurface.set_desync for details.
        /// 
        /// A new sub-surface is initially added as the top-most in the stack
        /// of its siblings and parent.
        PlaceAbove {sibling: Proxy<super::wl_surface::WlSurface>, },
        /// restack the sub-surface
        ///
        /// The sub-surface is placed just below the reference surface.
        /// See wl_subsurface.place_above.
        PlaceBelow {sibling: Proxy<super::wl_surface::WlSurface>, },
        /// set sub-surface to synchronized mode
        ///
        /// Change the commit behaviour of the sub-surface to synchronized
        /// mode, also described as the parent dependent mode.
        /// 
        /// In synchronized mode, wl_surface.commit on a sub-surface will
        /// accumulate the committed state in a cache, but the state will
        /// not be applied and hence will not change the compositor output.
        /// The cached state is applied to the sub-surface immediately after
        /// the parent surface's state is applied. This ensures atomic
        /// updates of the parent and all its synchronized sub-surfaces.
        /// Applying the cached state will invalidate the cache, so further
        /// parent surface commits do not (re-)apply old state.
        /// 
        /// See wl_subsurface for the recursive effect of this mode.
        SetSync,
        /// set sub-surface to desynchronized mode
        ///
        /// Change the commit behaviour of the sub-surface to desynchronized
        /// mode, also described as independent or freely running mode.
        /// 
        /// In desynchronized mode, wl_surface.commit on a sub-surface will
        /// apply the pending state directly, without caching, as happens
        /// normally with a wl_surface. Calling wl_surface.commit on the
        /// parent surface has no effect on the sub-surface's wl_surface
        /// state. This mode allows a sub-surface to be updated on its own.
        /// 
        /// If cached state exists when wl_surface.commit is called in
        /// desynchronized mode, the pending state is added to the cached
        /// state, and applied as a whole. This invalidates the cache.
        /// 
        /// Note: even if a sub-surface is set to desynchronized, a parent
        /// sub-surface may override it to behave as synchronized. For details,
        /// see wl_subsurface.
        /// 
        /// If a surface's parent surface behaves as desynchronized, then
        /// the cached state is applied on set_desync.
        SetDesync,
    }

    impl super::MessageGroup for Request {
        fn is_destructor(&self) -> bool {
            match *self {
                Request::Destroy => true,
                _ => false
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Request,()> {
            panic!("Request::from_raw_c can not be used Client-side.")
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            match self {
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                },
                Request::SetPosition { x, y, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    f(1, &mut _args_array)
                },
                Request::PlaceAbove { sibling, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = sibling.c_ptr() as *mut _;
                    f(2, &mut _args_array)
                },
                Request::PlaceBelow { sibling, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = sibling.c_ptr() as *mut _;
                    f(3, &mut _args_array)
                },
                Request::SetSync => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(4, &mut _args_array)
                },
                Request::SetDesync => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(5, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
    }

    impl super::MessageGroup for Event {
        fn is_destructor(&self) -> bool {
            match *self {
            }
        }

        unsafe fn from_raw_c(obj: *mut ::std::os::raw::c_void, opcode: u32, args: *const wl_argument) -> Result<Event,()> {
            match opcode {
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct WlSubsurface;

    impl Interface for WlSubsurface {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wl_subsurface";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wl_subsurface_interface }
        }
    }

    pub trait RequestsTrait {
        /// remove sub-surface interface
        ///
        /// The sub-surface interface is removed from the wl_surface object
        /// that was turned into a sub-surface with a
        /// wl_subcompositor.get_subsurface request. The wl_surface's association
        /// to the parent is deleted, and the wl_surface loses its role as
        /// a sub-surface. The wl_surface is unmapped.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// reposition the sub-surface
        ///
        /// This schedules a sub-surface position change.
        /// The sub-surface will be moved so that its origin (top left
        /// corner pixel) will be at the location x, y of the parent surface
        /// coordinate system. The coordinates are not restricted to the parent
        /// surface area. Negative values are allowed.
        /// 
        /// The scheduled coordinates will take effect whenever the state of the
        /// parent surface is applied. When this happens depends on whether the
        /// parent surface is in synchronized mode or not. See
        /// wl_subsurface.set_sync and wl_subsurface.set_desync for details.
        /// 
        /// If more than one set_position request is invoked by the client before
        /// the commit of the parent surface, the position of a new request always
        /// replaces the scheduled position from any previous request.
        /// 
        /// The initial position is 0, 0.
        fn set_position(&self, x: i32, y: i32) ->();
        /// restack the sub-surface
        ///
        /// This sub-surface is taken from the stack, and put back just
        /// above the reference surface, changing the z-order of the sub-surfaces.
        /// The reference surface must be one of the sibling surfaces, or the
        /// parent surface. Using any other surface, including this sub-surface,
        /// will cause a protocol error.
        /// 
        /// The z-order is double-buffered. Requests are handled in order and
        /// applied immediately to a pending state. The final pending state is
        /// copied to the active state the next time the state of the parent
        /// surface is applied. When this happens depends on whether the parent
        /// surface is in synchronized mode or not. See wl_subsurface.set_sync and
        /// wl_subsurface.set_desync for details.
        /// 
        /// A new sub-surface is initially added as the top-most in the stack
        /// of its siblings and parent.
        fn place_above(&self, sibling: &Proxy<super::wl_surface::WlSurface>) ->();
        /// restack the sub-surface
        ///
        /// The sub-surface is placed just below the reference surface.
        /// See wl_subsurface.place_above.
        fn place_below(&self, sibling: &Proxy<super::wl_surface::WlSurface>) ->();
        /// set sub-surface to synchronized mode
        ///
        /// Change the commit behaviour of the sub-surface to synchronized
        /// mode, also described as the parent dependent mode.
        /// 
        /// In synchronized mode, wl_surface.commit on a sub-surface will
        /// accumulate the committed state in a cache, but the state will
        /// not be applied and hence will not change the compositor output.
        /// The cached state is applied to the sub-surface immediately after
        /// the parent surface's state is applied. This ensures atomic
        /// updates of the parent and all its synchronized sub-surfaces.
        /// Applying the cached state will invalidate the cache, so further
        /// parent surface commits do not (re-)apply old state.
        /// 
        /// See wl_subsurface for the recursive effect of this mode.
        fn set_sync(&self) ->();
        /// set sub-surface to desynchronized mode
        ///
        /// Change the commit behaviour of the sub-surface to desynchronized
        /// mode, also described as independent or freely running mode.
        /// 
        /// In desynchronized mode, wl_surface.commit on a sub-surface will
        /// apply the pending state directly, without caching, as happens
        /// normally with a wl_surface. Calling wl_surface.commit on the
        /// parent surface has no effect on the sub-surface's wl_surface
        /// state. This mode allows a sub-surface to be updated on its own.
        /// 
        /// If cached state exists when wl_surface.commit is called in
        /// desynchronized mode, the pending state is added to the cached
        /// state, and applied as a whole. This invalidates the cache.
        /// 
        /// Note: even if a sub-surface is set to desynchronized, a parent
        /// sub-surface may override it to behave as synchronized. For details,
        /// see wl_subsurface.
        /// 
        /// If a surface's parent surface behaves as desynchronized, then
        /// the cached state is applied on set_desync.
        fn set_desync(&self) ->();
    }

    impl RequestsTrait for Proxy<WlSubsurface> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn set_position(&self, x: i32, y: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetPosition {
                x: x,
                y: y,
            };
            self.send(msg);
        }

        fn place_above(&self, sibling: &Proxy<super::wl_surface::WlSurface>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::PlaceAbove {
                sibling: sibling.clone(),
            };
            self.send(msg);
        }

        fn place_below(&self, sibling: &Proxy<super::wl_surface::WlSurface>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::PlaceBelow {
                sibling: sibling.clone(),
            };
            self.send(msg);
        }

        fn set_sync(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetSync;
            self.send(msg);
        }

        fn set_desync(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetDesync;
            self.send(msg);
        }

    }
}

