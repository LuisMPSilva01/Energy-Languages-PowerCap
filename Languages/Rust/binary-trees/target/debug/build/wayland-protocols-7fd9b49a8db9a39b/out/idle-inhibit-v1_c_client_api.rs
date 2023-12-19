
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2015 Samsung Electronics Co., Ltd

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

pub mod zwp_idle_inhibit_manager_v1 {
    //! control behavior when display idles
    //!
    //! This interface permits inhibiting the idle behavior such as screen
    //! blanking, locking, and screensaving.  The client binds the idle manager
    //! globally, then creates idle-inhibitor objects for each surface.
    //! 
    //! Warning! The protocol described in this file is experimental and
    //! backward incompatible changes may be made. Backward compatible changes
    //! may be added together with the corresponding interface version bump.
    //! Backward incompatible changes are done by bumping the version number in
    //! the protocol and interface names and resetting the interface version.
    //! Once the protocol is to be declared stable, the 'z' prefix and the
    //! version number in the protocol and interface names are removed and the
    //! interface version number is reset.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the idle inhibitor object
        ///
        /// Destroy the inhibit manager.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// create a new inhibitor object
        ///
        /// Create a new inhibitor object associated with the given surface.
        CreateInhibitor {id: Proxy<super::zwp_idle_inhibitor_v1::ZwpIdleInhibitorV1>, surface: Proxy<super::wl_surface::WlSurface>, },
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
                Request::CreateInhibitor { id, surface, } => {
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


    pub struct ZwpIdleInhibitManagerV1;

    impl Interface for ZwpIdleInhibitManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_idle_inhibit_manager_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_idle_inhibit_manager_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the idle inhibitor object
        ///
        /// Destroy the inhibit manager.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// create a new inhibitor object
        ///
        /// Create a new inhibitor object associated with the given surface.
        fn create_inhibitor(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::zwp_idle_inhibitor_v1::ZwpIdleInhibitorV1>, ()>;
    }

    impl RequestsTrait for Proxy<ZwpIdleInhibitManagerV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn create_inhibitor(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::zwp_idle_inhibitor_v1::ZwpIdleInhibitorV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::zwp_idle_inhibitor_v1::ZwpIdleInhibitorV1>();
            let msg = Request::CreateInhibitor {
                id: unsafe { Proxy::<super::zwp_idle_inhibitor_v1::ZwpIdleInhibitorV1>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                surface: surface.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod zwp_idle_inhibitor_v1 {
    //! context object for inhibiting idle behavior
    //!
    //! An idle inhibitor prevents the output that the associated surface is
    //! visible on from being set to a state where it is not visually usable due
    //! to lack of user interaction (e.g. blanked, dimmed, locked, set to power
    //! save, etc.)  Any screensaver processes are also blocked from displaying.
    //! 
    //! If the surface is destroyed, unmapped, becomes occluded, loses
    //! visibility, or otherwise becomes not visually relevant for the user, the
    //! idle inhibitor will not be honored by the compositor; if the surface
    //! subsequently regains visibility the inhibitor takes effect once again.
    //! Likewise, the inhibitor isn't honored if the system was already idled at
    //! the time the inhibitor was established, although if the system later
    //! de-idles and re-idles the inhibitor will take effect.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the idle inhibitor object
        ///
        /// Remove the inhibitor effect from the associated wl_surface.
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


    pub struct ZwpIdleInhibitorV1;

    impl Interface for ZwpIdleInhibitorV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_idle_inhibitor_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_idle_inhibitor_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the idle inhibitor object
        ///
        /// Remove the inhibitor effect from the associated wl_surface.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpIdleInhibitorV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

