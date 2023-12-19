
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2014      Jonas Ådahl
    Copyright © 2015      Red Hat Inc.

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

pub mod zwp_relative_pointer_manager_v1 {
    //! get relative pointer objects
    //!
    //! A global interface used for getting the relative pointer object for a
    //! given pointer.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the relative pointer manager object
        ///
        /// Used by the client to notify the server that it will no longer use this
        /// relative pointer manager object.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// get a relative pointer object
        ///
        /// Create a relative pointer interface given a wl_pointer object. See the
        /// wp_relative_pointer interface for more details.
        GetRelativePointer {id: Proxy<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>, pointer: Proxy<super::wl_pointer::WlPointer>, },
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
                Request::GetRelativePointer { id, pointer, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = pointer.c_ptr() as *mut _;
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


    pub struct ZwpRelativePointerManagerV1;

    impl Interface for ZwpRelativePointerManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_relative_pointer_manager_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_relative_pointer_manager_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the relative pointer manager object
        ///
        /// Used by the client to notify the server that it will no longer use this
        /// relative pointer manager object.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// get a relative pointer object
        ///
        /// Create a relative pointer interface given a wl_pointer object. See the
        /// wp_relative_pointer interface for more details.
        fn get_relative_pointer(&self, pointer: &Proxy<super::wl_pointer::WlPointer>) ->Result<NewProxy<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>, ()>;
    }

    impl RequestsTrait for Proxy<ZwpRelativePointerManagerV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn get_relative_pointer(&self, pointer: &Proxy<super::wl_pointer::WlPointer>) ->Result<NewProxy<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>();
            let msg = Request::GetRelativePointer {
                id: unsafe { Proxy::<super::zwp_relative_pointer_v1::ZwpRelativePointerV1>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                pointer: pointer.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod zwp_relative_pointer_v1 {
    //! relative pointer object
    //!
    //! A wp_relative_pointer object is an extension to the wl_pointer interface
    //! used for emitting relative pointer events. It shares the same focus as
    //! wl_pointer objects of the same seat and will only emit events when it has
    //! focus.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// release the relative pointer object
        ///
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
        /// relative pointer motion
        ///
        /// Relative x/y pointer motion from the pointer of the seat associated with
        /// this object.
        /// 
        /// A relative motion is in the same dimension as regular wl_pointer motion
        /// events, except they do not represent an absolute position. For example,
        /// moving a pointer from (x, y) to (x', y') would have the equivalent
        /// relative motion (x' - x, y' - y). If a pointer motion caused the
        /// absolute pointer position to be clipped by for example the edge of the
        /// monitor, the relative motion is unaffected by the clipping and will
        /// represent the unclipped motion.
        /// 
        /// This event also contains non-accelerated motion deltas. The
        /// non-accelerated delta is, when applicable, the regular pointer motion
        /// delta as it was before having applied motion acceleration and other
        /// transformations such as normalization.
        /// 
        /// Note that the non-accelerated delta does not represent 'raw' events as
        /// they were read from some device. Pointer motion acceleration is device-
        /// and configuration-specific and non-accelerated deltas and accelerated
        /// deltas may have the same value on some devices.
        /// 
        /// Relative motions are not coupled to wl_pointer.motion events, and can be
        /// sent in combination with such events, but also independently. There may
        /// also be scenarios where wl_pointer.motion is sent, but there is no
        /// relative motion. The order of an absolute and relative motion event
        /// originating from the same physical motion is not guaranteed.
        /// 
        /// If the client needs button events or focus state, it can receive them
        /// from a wl_pointer object of the same seat that the wp_relative_pointer
        /// object is associated with.
        RelativeMotion {utime_hi: u32, utime_lo: u32, dx: f64, dy: f64, dx_unaccel: f64, dy_unaccel: f64, },
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
                    Ok(Event::RelativeMotion {
                        utime_hi: _args[0].u,
                        utime_lo: _args[1].u,
                        dx: (_args[2].f as f64)/256.,
                        dy: (_args[3].f as f64)/256.,
                        dx_unaccel: (_args[4].f as f64)/256.,
                        dy_unaccel: (_args[5].f as f64)/256.,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpRelativePointerV1;

    impl Interface for ZwpRelativePointerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_relative_pointer_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_relative_pointer_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// release the relative pointer object
        ///
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpRelativePointerV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

