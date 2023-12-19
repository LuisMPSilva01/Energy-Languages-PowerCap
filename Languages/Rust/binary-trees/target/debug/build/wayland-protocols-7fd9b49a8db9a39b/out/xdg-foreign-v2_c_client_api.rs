
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2015-2016 Red Hat Inc.

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice (including the next
    paragraph) shall be included in all copies or substantial portions of the
    Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
    THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
*/

pub mod zxdg_exporter_v2 {
    //! interface for exporting surfaces
    //!
    //! A global interface used for exporting surfaces that can later be imported
    //! using xdg_importer.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the xdg_exporter object
        ///
        /// Notify the compositor that the xdg_exporter object will no longer be
        /// used.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// export a toplevel surface
        ///
        /// The export_toplevel request exports the passed surface so that it can later be
        /// imported via xdg_importer. When called, a new xdg_exported object will
        /// be created and xdg_exported.handle will be sent immediately. See the
        /// corresponding interface and event for details.
        /// 
        /// A surface may be exported multiple times, and each exported handle may
        /// be used to create a xdg_imported multiple times. Only xdg_toplevel
        /// equivalent surfaces may be exported.
        ExportToplevel {id: Proxy<super::zxdg_exported_v2::ZxdgExportedV2>, surface: Proxy<super::wl_surface::WlSurface>, },
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
                Request::ExportToplevel { id, surface, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
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


    pub struct ZxdgExporterV2;

    impl Interface for ZxdgExporterV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_exporter_v2";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_exporter_v2_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the xdg_exporter object
        ///
        /// Notify the compositor that the xdg_exporter object will no longer be
        /// used.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// export a toplevel surface
        ///
        /// The export_toplevel request exports the passed surface so that it can later be
        /// imported via xdg_importer. When called, a new xdg_exported object will
        /// be created and xdg_exported.handle will be sent immediately. See the
        /// corresponding interface and event for details.
        /// 
        /// A surface may be exported multiple times, and each exported handle may
        /// be used to create a xdg_imported multiple times. Only xdg_toplevel
        /// equivalent surfaces may be exported.
        fn export_toplevel(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::zxdg_exported_v2::ZxdgExportedV2>, ()>;
    }

    impl RequestsTrait for Proxy<ZxdgExporterV2> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn export_toplevel(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::zxdg_exported_v2::ZxdgExportedV2>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::zxdg_exported_v2::ZxdgExportedV2>();
            let msg = Request::ExportToplevel {
                id: unsafe { Proxy::<super::zxdg_exported_v2::ZxdgExportedV2>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                surface: surface.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod zxdg_importer_v2 {
    //! interface for importing surfaces
    //!
    //! A global interface used for importing surfaces exported by xdg_exporter.
    //! With this interface, a client can create a reference to a surface of
    //! another client.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the xdg_importer object
        ///
        /// Notify the compositor that the xdg_importer object will no longer be
        /// used.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// import a toplevel surface
        ///
        /// The import_toplevel request imports a surface from any client given a handle
        /// retrieved by exporting said surface using xdg_exporter.export_toplevel.
        /// When called, a new xdg_imported object will be created. This new object
        /// represents the imported surface, and the importing client can
        /// manipulate its relationship using it. See xdg_imported for details.
        ImportToplevel {id: Proxy<super::zxdg_imported_v2::ZxdgImportedV2>, handle: String, },
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
                Request::ImportToplevel { id, handle, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    let _arg_1 = ::std::ffi::CString::new(handle).unwrap();
                    _args_array[1].s = _arg_1.as_ptr();
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


    pub struct ZxdgImporterV2;

    impl Interface for ZxdgImporterV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_importer_v2";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_importer_v2_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the xdg_importer object
        ///
        /// Notify the compositor that the xdg_importer object will no longer be
        /// used.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// import a toplevel surface
        ///
        /// The import_toplevel request imports a surface from any client given a handle
        /// retrieved by exporting said surface using xdg_exporter.export_toplevel.
        /// When called, a new xdg_imported object will be created. This new object
        /// represents the imported surface, and the importing client can
        /// manipulate its relationship using it. See xdg_imported for details.
        fn import_toplevel(&self, handle: String) ->Result<NewProxy<super::zxdg_imported_v2::ZxdgImportedV2>, ()>;
    }

    impl RequestsTrait for Proxy<ZxdgImporterV2> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn import_toplevel(&self, handle: String) ->Result<NewProxy<super::zxdg_imported_v2::ZxdgImportedV2>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::zxdg_imported_v2::ZxdgImportedV2>();
            let msg = Request::ImportToplevel {
                id: unsafe { Proxy::<super::zxdg_imported_v2::ZxdgImportedV2>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                handle: handle,
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod zxdg_exported_v2 {
    //! an exported surface handle
    //!
    //! A xdg_exported object represents an exported reference to a surface. The
    //! exported surface may be referenced as long as the xdg_exported object not
    //! destroyed. Destroying the xdg_exported invalidates any relationship the
    //! importer may have established using xdg_imported.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// unexport the exported surface
        ///
        /// Revoke the previously exported surface. This invalidates any
        /// relationship the importer may have set up using the xdg_imported created
        /// given the handle sent via xdg_exported.handle.
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
        /// the exported surface handle
        ///
        /// The handle event contains the unique handle of this exported surface
        /// reference. It may be shared with any client, which then can use it to
        /// import the surface by calling xdg_importer.import_toplevel. A handle
        /// may be used to import the surface multiple times.
        Handle {handle: String, },
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
                    Ok(Event::Handle {
                        handle: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZxdgExportedV2;

    impl Interface for ZxdgExportedV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_exported_v2";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_exported_v2_interface }
        }
    }

    pub trait RequestsTrait {
        /// unexport the exported surface
        ///
        /// Revoke the previously exported surface. This invalidates any
        /// relationship the importer may have set up using the xdg_imported created
        /// given the handle sent via xdg_exported.handle.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZxdgExportedV2> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

pub mod zxdg_imported_v2 {
    //! an imported surface handle
    //!
    //! A xdg_imported object represents an imported reference to surface exported
    //! by some client. A client can use this interface to manipulate
    //! relationships between its own surfaces and the imported surface.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the xdg_imported object
        ///
        /// Notify the compositor that it will no longer use the xdg_imported
        /// object. Any relationship that may have been set up will at this point
        /// be invalidated.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// set as the parent of some surface
        ///
        /// Set the imported surface as the parent of some surface of the client.
        /// The passed surface must be a xdg_toplevel equivalent. Calling this
        /// function sets up a surface to surface relation with the same stacking
        /// and positioning semantics as xdg_toplevel.set_parent.
        SetParentOf {surface: Proxy<super::wl_surface::WlSurface>, },
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
                Request::SetParentOf { surface, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = surface.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// the imported surface handle has been destroyed
        ///
        /// The imported surface handle has been destroyed and any relationship set
        /// up has been invalidated. This may happen for various reasons, for
        /// example if the exported surface or the exported surface handle has been
        /// destroyed, if the handle used for importing was invalid.
        Destroyed,
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
                    Ok(Event::Destroyed) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZxdgImportedV2;

    impl Interface for ZxdgImportedV2 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_imported_v2";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_imported_v2_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the xdg_imported object
        ///
        /// Notify the compositor that it will no longer use the xdg_imported
        /// object. Any relationship that may have been set up will at this point
        /// be invalidated.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// set as the parent of some surface
        ///
        /// Set the imported surface as the parent of some surface of the client.
        /// The passed surface must be a xdg_toplevel equivalent. Calling this
        /// function sets up a surface to surface relation with the same stacking
        /// and positioning semantics as xdg_toplevel.set_parent.
        fn set_parent_of(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->();
    }

    impl RequestsTrait for Proxy<ZxdgImportedV2> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn set_parent_of(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetParentOf {
                surface: surface.clone(),
            };
            self.send(msg);
        }

    }
}

