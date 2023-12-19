
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2018 Simon Ser

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

pub mod zxdg_decoration_manager_v1 {
    //! window decoration manager
    //!
    //! This interface allows a compositor to announce support for server-side
    //! decorations.
    //! 
    //! A window decoration is a set of window controls as deemed appropriate by
    //! the party managing them, such as user interface components used to move,
    //! resize and change a window's state.
    //! 
    //! A client can use this protocol to request being decorated by a supporting
    //! compositor.
    //! 
    //! If compositor and client do not negotiate the use of a server-side
    //! decoration using this protocol, clients continue to self-decorate as they
    //! see fit.
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
        /// destroy the decoration manager object
        ///
        /// Destroy the decoration manager. This doesn't destroy objects created
        /// with the manager.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// create a new toplevel decoration object
        ///
        /// Create a new decoration object associated with the given toplevel.
        /// 
        /// Creating an xdg_toplevel_decoration from an xdg_toplevel which has a
        /// buffer attached or committed is a client error, and any attempts by a
        /// client to attach or manipulate a buffer prior to the first
        /// xdg_toplevel_decoration.configure event must also be treated as
        /// errors.
        GetToplevelDecoration {id: Proxy<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>, toplevel: Proxy<super::xdg_toplevel::XdgToplevel>, },
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
                Request::GetToplevelDecoration { id, toplevel, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = toplevel.c_ptr() as *mut _;
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


    pub struct ZxdgDecorationManagerV1;

    impl Interface for ZxdgDecorationManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_decoration_manager_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_decoration_manager_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the decoration manager object
        ///
        /// Destroy the decoration manager. This doesn't destroy objects created
        /// with the manager.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// create a new toplevel decoration object
        ///
        /// Create a new decoration object associated with the given toplevel.
        /// 
        /// Creating an xdg_toplevel_decoration from an xdg_toplevel which has a
        /// buffer attached or committed is a client error, and any attempts by a
        /// client to attach or manipulate a buffer prior to the first
        /// xdg_toplevel_decoration.configure event must also be treated as
        /// errors.
        fn get_toplevel_decoration(&self, toplevel: &Proxy<super::xdg_toplevel::XdgToplevel>) ->Result<NewProxy<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>, ()>;
    }

    impl RequestsTrait for Proxy<ZxdgDecorationManagerV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn get_toplevel_decoration(&self, toplevel: &Proxy<super::xdg_toplevel::XdgToplevel>) ->Result<NewProxy<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>();
            let msg = Request::GetToplevelDecoration {
                id: unsafe { Proxy::<super::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                toplevel: toplevel.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod zxdg_toplevel_decoration_v1 {
    //! decoration object for a toplevel surface
    //!
    //! The decoration object allows the compositor to toggle server-side window
    //! decorations for a toplevel surface. The client can request to switch to
    //! another mode.
    //! 
    //! The xdg_toplevel_decoration object must be destroyed before its
    //! xdg_toplevel.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// xdg_toplevel has a buffer attached before configure
        UnconfiguredBuffer = 0,
        /// xdg_toplevel already has a decoration object
        AlreadyConstructed = 1,
        /// xdg_toplevel destroyed before the decoration object
        Orphaned = 2,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::UnconfiguredBuffer),
                1 => Some(Error::AlreadyConstructed),
                2 => Some(Error::Orphaned),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// window decoration modes
    ///
    /// These values describe window decoration modes.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Mode {
        /// no server-side window decoration
        ClientSide = 1,
        /// server-side window decoration
        ServerSide = 2,
    }
    impl Mode {
        pub fn from_raw(n: u32) -> Option<Mode> {
            match n {
                1 => Some(Mode::ClientSide),
                2 => Some(Mode::ServerSide),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// destroy the decoration object
        ///
        /// Switch back to a mode without any server-side decorations at the next
        /// commit.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// set the decoration mode
        ///
        /// Set the toplevel surface decoration mode. This informs the compositor
        /// that the client prefers the provided decoration mode.
        /// 
        /// After requesting a decoration mode, the compositor will respond by
        /// emitting a xdg_surface.configure event. The client should then update
        /// its content, drawing it without decorations if the received mode is
        /// server-side decorations. The client must also acknowledge the configure
        /// when committing the new content (see xdg_surface.ack_configure).
        /// 
        /// The compositor can decide not to use the client's mode and enforce a
        /// different mode instead.
        /// 
        /// Clients whose decoration mode depend on the xdg_toplevel state may send
        /// a set_mode request in response to a xdg_surface.configure event and wait
        /// for the next xdg_surface.configure event to prevent unwanted state.
        /// Such clients are responsible for preventing configure loops and must
        /// make sure not to send multiple successive set_mode requests with the
        /// same decoration mode.
        SetMode {mode: Mode, },
        /// unset the decoration mode
        ///
        /// Unset the toplevel surface decoration mode. This informs the compositor
        /// that the client doesn't prefer a particular decoration mode.
        /// 
        /// This request has the same semantics as set_mode.
        UnsetMode,
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
                Request::SetMode { mode, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = mode.to_raw();
                    f(1, &mut _args_array)
                },
                Request::UnsetMode => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// suggest a surface change
        ///
        /// The configure event asks the client to change its decoration mode. The
        /// configured state should not be applied immediately. Clients must send an
        /// ack_configure in response to this event. See xdg_surface.configure and
        /// xdg_surface.ack_configure for details.
        /// 
        /// A configure event can be sent at any time. The specified mode must be
        /// obeyed by the client.
        Configure {mode: Mode, },
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
                    Ok(Event::Configure {
                        mode: Mode::from_raw(_args[0].u).ok_or(())?,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZxdgToplevelDecorationV1;

    impl Interface for ZxdgToplevelDecorationV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zxdg_toplevel_decoration_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zxdg_toplevel_decoration_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the decoration object
        ///
        /// Switch back to a mode without any server-side decorations at the next
        /// commit.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// set the decoration mode
        ///
        /// Set the toplevel surface decoration mode. This informs the compositor
        /// that the client prefers the provided decoration mode.
        /// 
        /// After requesting a decoration mode, the compositor will respond by
        /// emitting a xdg_surface.configure event. The client should then update
        /// its content, drawing it without decorations if the received mode is
        /// server-side decorations. The client must also acknowledge the configure
        /// when committing the new content (see xdg_surface.ack_configure).
        /// 
        /// The compositor can decide not to use the client's mode and enforce a
        /// different mode instead.
        /// 
        /// Clients whose decoration mode depend on the xdg_toplevel state may send
        /// a set_mode request in response to a xdg_surface.configure event and wait
        /// for the next xdg_surface.configure event to prevent unwanted state.
        /// Such clients are responsible for preventing configure loops and must
        /// make sure not to send multiple successive set_mode requests with the
        /// same decoration mode.
        fn set_mode(&self, mode: Mode) ->();
        /// unset the decoration mode
        ///
        /// Unset the toplevel surface decoration mode. This informs the compositor
        /// that the client doesn't prefer a particular decoration mode.
        /// 
        /// This request has the same semantics as set_mode.
        fn unset_mode(&self) ->();
    }

    impl RequestsTrait for Proxy<ZxdgToplevelDecorationV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn set_mode(&self, mode: Mode) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetMode {
                mode: mode,
            };
            self.send(msg);
        }

        fn unset_mode(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::UnsetMode;
            self.send(msg);
        }

    }
}

