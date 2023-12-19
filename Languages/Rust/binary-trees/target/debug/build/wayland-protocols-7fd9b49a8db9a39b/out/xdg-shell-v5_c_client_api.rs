
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2008-2013 Kristian Høgsberg
    Copyright © 2013      Rafael Antognolli
    Copyright © 2013      Jasper St. Pierre
    Copyright © 2010-2013 Intel Corporation

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

pub mod xdg_shell {
    //! create desktop-style surfaces
    //!
    //! xdg_shell allows clients to turn a wl_surface into a "real window"
    //! which can be dragged, resized, stacked, and moved around by the
    //! user. Everything about this interface is suited towards traditional
    //! desktop environments.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// latest protocol version
    ///
    /// The 'current' member of this enum gives the version of the
    /// protocol.  Implementations can compare this to the version
    /// they implement using static_assert to ensure the protocol and
    /// implementation versions match.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Version {
        /// Always the latest version
        Current = 5,
    }
    impl Version {
        pub fn from_raw(n: u32) -> Option<Version> {
            match n {
                5 => Some(Version::Current),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }


    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// given wl_surface has another role
        Role = 0,
        /// xdg_shell was destroyed before children
        DefunctSurfaces = 1,
        /// the client tried to map or destroy a non-topmost popup
        NotTheTopmostPopup = 2,
        /// the client specified an invalid popup parent surface
        InvalidPopupParent = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                1 => Some(Error::DefunctSurfaces),
                2 => Some(Error::NotTheTopmostPopup),
                3 => Some(Error::InvalidPopupParent),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// destroy xdg_shell
        ///
        /// Destroy this xdg_shell object.
        /// 
        /// Destroying a bound xdg_shell object while there are surfaces
        /// still alive created by this xdg_shell object instance is illegal
        /// and will result in a protocol error.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// enable use of this unstable version
        ///
        /// Negotiate the unstable version of the interface.  This
        /// mechanism is in place to ensure client and server agree on the
        /// unstable versions of the protocol that they speak or exit
        /// cleanly if they don't agree.  This request will go away once
        /// the xdg-shell protocol is stable.
        UseUnstableVersion {version: i32, },
        /// create a shell surface from a surface
        ///
        /// This creates an xdg_surface for the given surface and gives it the
        /// xdg_surface role. A wl_surface can only be given an xdg_surface role
        /// once. If get_xdg_surface is called with a wl_surface that already has
        /// an active xdg_surface associated with it, or if it had any other role,
        /// an error is raised.
        /// 
        /// See the documentation of xdg_surface for more details about what an
        /// xdg_surface is and how it is used.
        GetXdgSurface {id: Proxy<super::xdg_surface::XdgSurface>, surface: Proxy<super::wl_surface::WlSurface>, },
        /// create a popup for a surface
        ///
        /// This creates an xdg_popup for the given surface and gives it the
        /// xdg_popup role. A wl_surface can only be given an xdg_popup role
        /// once. If get_xdg_popup is called with a wl_surface that already has
        /// an active xdg_popup associated with it, or if it had any other role,
        /// an error is raised.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event.
        /// 
        /// See the documentation of xdg_popup for more details about what an
        /// xdg_popup is and how it is used.
        GetXdgPopup {id: Proxy<super::xdg_popup::XdgPopup>, surface: Proxy<super::wl_surface::WlSurface>, parent: Proxy<super::wl_surface::WlSurface>, seat: Proxy<super::wl_seat::WlSeat>, serial: u32, x: i32, y: i32, },
        /// respond to a ping event
        ///
        /// A client must respond to a ping event with a pong request or
        /// the client may be deemed unresponsive.
        Pong {serial: u32, },
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
                Request::UseUnstableVersion { version, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = version;
                    f(1, &mut _args_array)
                },
                Request::GetXdgSurface { id, surface, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    f(2, &mut _args_array)
                },
                Request::GetXdgPopup { id, surface, parent, seat, serial, x, y, } => {
                    let mut _args_array: [wl_argument; 7] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    _args_array[2].o = parent.c_ptr() as *mut _;
                    _args_array[3].o = seat.c_ptr() as *mut _;
                    _args_array[4].u = serial;
                    _args_array[5].i = x;
                    _args_array[6].i = y;
                    f(3, &mut _args_array)
                },
                Request::Pong { serial, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(4, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// check if the client is alive
        ///
        /// The ping event asks the client if it's still alive. Pass the
        /// serial specified in the event back to the compositor by sending
        /// a "pong" request back with the specified serial.
        /// 
        /// Compositors can use this to determine if the client is still
        /// alive. It's unspecified what will happen if the client doesn't
        /// respond to the ping request, or in what timeframe. Clients should
        /// try to respond in a reasonable amount of time.
        /// 
        /// A compositor is free to ping in any way it wants, but a client must
        /// always respond to any xdg_shell object it created.
        Ping {serial: u32, },
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
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct XdgShell;

    impl Interface for XdgShell {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_shell";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::xdg_shell_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy xdg_shell
        ///
        /// Destroy this xdg_shell object.
        /// 
        /// Destroying a bound xdg_shell object while there are surfaces
        /// still alive created by this xdg_shell object instance is illegal
        /// and will result in a protocol error.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// enable use of this unstable version
        ///
        /// Negotiate the unstable version of the interface.  This
        /// mechanism is in place to ensure client and server agree on the
        /// unstable versions of the protocol that they speak or exit
        /// cleanly if they don't agree.  This request will go away once
        /// the xdg-shell protocol is stable.
        fn use_unstable_version(&self, version: i32) ->();
        /// create a shell surface from a surface
        ///
        /// This creates an xdg_surface for the given surface and gives it the
        /// xdg_surface role. A wl_surface can only be given an xdg_surface role
        /// once. If get_xdg_surface is called with a wl_surface that already has
        /// an active xdg_surface associated with it, or if it had any other role,
        /// an error is raised.
        /// 
        /// See the documentation of xdg_surface for more details about what an
        /// xdg_surface is and how it is used.
        fn get_xdg_surface(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::xdg_surface::XdgSurface>, ()>;
        /// create a popup for a surface
        ///
        /// This creates an xdg_popup for the given surface and gives it the
        /// xdg_popup role. A wl_surface can only be given an xdg_popup role
        /// once. If get_xdg_popup is called with a wl_surface that already has
        /// an active xdg_popup associated with it, or if it had any other role,
        /// an error is raised.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event.
        /// 
        /// See the documentation of xdg_popup for more details about what an
        /// xdg_popup is and how it is used.
        fn get_xdg_popup(&self, surface: &Proxy<super::wl_surface::WlSurface>, parent: &Proxy<super::wl_surface::WlSurface>, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32, x: i32, y: i32) ->Result<NewProxy<super::xdg_popup::XdgPopup>, ()>;
        /// respond to a ping event
        ///
        /// A client must respond to a ping event with a pong request or
        /// the client may be deemed unresponsive.
        fn pong(&self, serial: u32) ->();
    }

    impl RequestsTrait for Proxy<XdgShell> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn use_unstable_version(&self, version: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::UseUnstableVersion {
                version: version,
            };
            self.send(msg);
        }

        fn get_xdg_surface(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::xdg_surface::XdgSurface>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::xdg_surface::XdgSurface>();
            let msg = Request::GetXdgSurface {
                id: unsafe { Proxy::<super::xdg_surface::XdgSurface>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                surface: surface.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn get_xdg_popup(&self, surface: &Proxy<super::wl_surface::WlSurface>, parent: &Proxy<super::wl_surface::WlSurface>, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32, x: i32, y: i32) ->Result<NewProxy<super::xdg_popup::XdgPopup>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::xdg_popup::XdgPopup>();
            let msg = Request::GetXdgPopup {
                id: unsafe { Proxy::<super::xdg_popup::XdgPopup>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                surface: surface.clone(),
                parent: parent.clone(),
                seat: seat.clone(),
                serial: serial,
                x: x,
                y: y,
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn pong(&self, serial: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Pong {
                serial: serial,
            };
            self.send(msg);
        }

    }
}

pub mod xdg_surface {
    //! A desktop window
    //!
    //! An interface that may be implemented by a wl_surface, for
    //! implementations that provide a desktop-style user interface.
    //! 
    //! It provides requests to treat surfaces like windows, allowing to set
    //! properties like maximized, fullscreen, minimized, and to move and resize
    //! them, and associate metadata like title and app id.
    //! 
    //! The client must call wl_surface.commit on the corresponding wl_surface
    //! for the xdg_surface state to take effect. Prior to committing the new
    //! state, it can set up initial configuration, such as maximizing or setting
    //! a window geometry.
    //! 
    //! Even without attaching a buffer the compositor must respond to initial
    //! committed configuration, for instance sending a configure event with
    //! expected window geometry if the client maximized its surface during
    //! initialization.
    //! 
    //! For a surface to be mapped by the compositor the client must have
    //! committed both an xdg_surface state and a buffer.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// edge values for resizing
    ///
    /// These values are used to indicate which edge of a surface
    /// is being dragged in a resize operation.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ResizeEdge {
        None = 0,
        Top = 1,
        Bottom = 2,
        Left = 4,
        TopLeft = 5,
        BottomLeft = 6,
        Right = 8,
        TopRight = 9,
        BottomRight = 10,
    }
    impl ResizeEdge {
        pub fn from_raw(n: u32) -> Option<ResizeEdge> {
            match n {
                0 => Some(ResizeEdge::None),
                1 => Some(ResizeEdge::Top),
                2 => Some(ResizeEdge::Bottom),
                4 => Some(ResizeEdge::Left),
                5 => Some(ResizeEdge::TopLeft),
                6 => Some(ResizeEdge::BottomLeft),
                8 => Some(ResizeEdge::Right),
                9 => Some(ResizeEdge::TopRight),
                10 => Some(ResizeEdge::BottomRight),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// types of state on the surface
    ///
    /// The different state values used on the surface. This is designed for
    /// state values like maximized, fullscreen. It is paired with the
    /// configure event to ensure that both the client and the compositor
    /// setting the state can be synchronized.
    /// 
    /// States set in this way are double-buffered. They will get applied on
    /// the next commit.
    /// 
    /// Desktop environments may extend this enum by taking up a range of
    /// values and documenting the range they chose in this description.
    /// They are not required to document the values for the range that they
    /// chose. Ideally, any good extensions from a desktop environment should
    /// make its way into standardization into this enum.
    /// 
    /// The current reserved ranges are:
    /// 
    /// 0x0000 - 0x0FFF: xdg-shell core values, documented below.
    /// 0x1000 - 0x1FFF: GNOME
    /// 0x2000 - 0x2FFF: EFL

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum State {
        /// the surface is maximized
        ///
        /// The surface is maximized. The window geometry specified in the configure
        /// event must be obeyed by the client.
        Maximized = 1,
        /// the surface is fullscreen
        ///
        /// The surface is fullscreen. The window geometry specified in the configure
        /// event must be obeyed by the client.
        Fullscreen = 2,
        /// the surface is being resized
        ///
        /// The surface is being resized. The window geometry specified in the
        /// configure event is a maximum; the client cannot resize beyond it.
        /// Clients that have aspect ratio or cell sizing configuration can use
        /// a smaller size, however.
        Resizing = 3,
        /// the surface is now activated
        ///
        /// Client window decorations should be painted as if the window is
        /// active. Do not assume this means that the window actually has
        /// keyboard or pointer focus.
        Activated = 4,
    }
    impl State {
        pub fn from_raw(n: u32) -> Option<State> {
            match n {
                1 => Some(State::Maximized),
                2 => Some(State::Fullscreen),
                3 => Some(State::Resizing),
                4 => Some(State::Activated),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// Destroy the xdg_surface
        ///
        /// Unmap and destroy the window. The window will be effectively
        /// hidden from the user's point of view, and all state like
        /// maximization, fullscreen, and so on, will be lost.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// set the parent of this surface
        ///
        /// Set the "parent" of this surface. This window should be stacked
        /// above a parent. The parent surface must be mapped as long as this
        /// surface is mapped.
        /// 
        /// Parent windows should be set on dialogs, toolboxes, or other
        /// "auxiliary" surfaces, so that the parent is raised when the dialog
        /// is raised.
        SetParent {parent: Option<Proxy<super::xdg_surface::XdgSurface>>, },
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
        /// set application ID
        ///
        /// Set an application identifier for the surface.
        /// 
        /// The app ID identifies the general class of applications to which
        /// the surface belongs. The compositor can use this to group multiple
        /// surfaces together, or to determine how to launch a new application.
        /// 
        /// For D-Bus activatable applications, the app ID is used as the D-Bus
        /// service name.
        /// 
        /// The compositor shell will try to group application surfaces together
        /// by their app ID.  As a best practice, it is suggested to select app
        /// ID's that match the basename of the application's .desktop file.
        /// For example, "org.freedesktop.FooViewer" where the .desktop file is
        /// "org.freedesktop.FooViewer.desktop".
        /// 
        /// See the desktop-entry specification [0] for more details on
        /// application identifiers and how they relate to well-known D-Bus
        /// names and .desktop files.
        /// 
        /// [0] http://standards.freedesktop.org/desktop-entry-spec/
        SetAppId {app_id: String, },
        /// show the window menu
        ///
        /// Clients implementing client-side decorations might want to show
        /// a context menu when right-clicking on the decorations, giving the
        /// user a menu that they can use to maximize or minimize the window.
        /// 
        /// This request asks the compositor to pop up such a window menu at
        /// the given position, relative to the local surface coordinates of
        /// the parent surface. There are no guarantees as to what menu items
        /// the window menu contains.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event.
        ShowWindowMenu {seat: Proxy<super::wl_seat::WlSeat>, serial: u32, x: i32, y: i32, },
        /// start an interactive move
        ///
        /// Start an interactive, user-driven move of the surface.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event. The passed
        /// serial is used to determine the type of interactive move (touch,
        /// pointer, etc).
        /// 
        /// The server may ignore move requests depending on the state of
        /// the surface (e.g. fullscreen or maximized), or if the passed serial
        /// is no longer valid.
        /// 
        /// If triggered, the surface will lose the focus of the device
        /// (wl_pointer, wl_touch, etc) used for the move. It is up to the
        /// compositor to visually indicate that the move is taking place, such as
        /// updating a pointer cursor, during the move. There is no guarantee
        /// that the device focus will return when the move is completed.
        Move {seat: Proxy<super::wl_seat::WlSeat>, serial: u32, },
        /// start an interactive resize
        ///
        /// Start a user-driven, interactive resize of the surface.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event. The passed
        /// serial is used to determine the type of interactive resize (touch,
        /// pointer, etc).
        /// 
        /// The server may ignore resize requests depending on the state of
        /// the surface (e.g. fullscreen or maximized).
        /// 
        /// If triggered, the client will receive configure events with the
        /// "resize" state enum value and the expected sizes. See the "resize"
        /// enum value for more details about what is required. The client
        /// must also acknowledge configure events using "ack_configure". After
        /// the resize is completed, the client will receive another "configure"
        /// event without the resize state.
        /// 
        /// If triggered, the surface also will lose the focus of the device
        /// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
        /// compositor to visually indicate that the resize is taking place,
        /// such as updating a pointer cursor, during the resize. There is no
        /// guarantee that the device focus will return when the resize is
        /// completed.
        /// 
        /// The edges parameter specifies how the surface should be resized,
        /// and is one of the values of the resize_edge enum. The compositor
        /// may use this information to update the surface position for
        /// example when dragging the top left corner. The compositor may also
        /// use this information to adapt its behavior, e.g. choose an
        /// appropriate cursor image.
        Resize {seat: Proxy<super::wl_seat::WlSeat>, serial: u32, edges: u32, },
        /// ack a configure event
        ///
        /// When a configure event is received, if a client commits the
        /// surface in response to the configure event, then the client
        /// must make an ack_configure request sometime before the commit
        /// request, passing along the serial of the configure event.
        /// 
        /// For instance, the compositor might use this information to move
        /// a surface to the top left only when the client has drawn itself
        /// for the maximized or fullscreen state.
        /// 
        /// If the client receives multiple configure events before it
        /// can respond to one, it only has to ack the last configure event.
        /// 
        /// A client is not required to commit immediately after sending
        /// an ack_configure request - it may even ack_configure several times
        /// before its next surface commit.
        /// 
        /// The compositor expects that the most recently received
        /// ack_configure request at the time of a commit indicates which
        /// configure event the client is responding to.
        AckConfigure {serial: u32, },
        /// set the new window geometry
        ///
        /// The window geometry of a window is its "visible bounds" from the
        /// user's perspective. Client-side decorations often have invisible
        /// portions like drop-shadows which should be ignored for the
        /// purposes of aligning, placing and constraining windows.
        /// 
        /// The window geometry is double buffered, and will be applied at the
        /// time wl_surface.commit of the corresponding wl_surface is called.
        /// 
        /// Once the window geometry of the surface is set once, it is not
        /// possible to unset it, and it will remain the same until
        /// set_window_geometry is called again, even if a new subsurface or
        /// buffer is attached.
        /// 
        /// If never set, the value is the full bounds of the surface,
        /// including any subsurfaces. This updates dynamically on every
        /// commit. This unset mode is meant for extremely simple clients.
        /// 
        /// If responding to a configure event, the window geometry in here
        /// must respect the sizing negotiations specified by the states in
        /// the configure event.
        /// 
        /// The arguments are given in the surface local coordinate space of
        /// the wl_surface associated with this xdg_surface.
        /// 
        /// The width and height must be greater than zero.
        SetWindowGeometry {x: i32, y: i32, width: i32, height: i32, },
        /// maximize the window
        ///
        /// Maximize the surface.
        /// 
        /// After requesting that the surface should be maximized, the compositor
        /// will respond by emitting a configure event with the "maximized" state
        /// and the required window geometry. The client should then update its
        /// content, drawing it in a maximized state, i.e. without shadow or other
        /// decoration outside of the window geometry. The client must also
        /// acknowledge the configure when committing the new content (see
        /// ack_configure).
        /// 
        /// It is up to the compositor to decide how and where to maximize the
        /// surface, for example which output and what region of the screen should
        /// be used.
        /// 
        /// If the surface was already maximized, the compositor will still emit
        /// a configure event with the "maximized" state.
        SetMaximized,
        /// unmaximize the window
        ///
        /// Unmaximize the surface.
        /// 
        /// After requesting that the surface should be unmaximized, the compositor
        /// will respond by emitting a configure event without the "maximized"
        /// state. If available, the compositor will include the window geometry
        /// dimensions the window had prior to being maximized in the configure
        /// request. The client must then update its content, drawing it in a
        /// regular state, i.e. potentially with shadow, etc. The client must also
        /// acknowledge the configure when committing the new content (see
        /// ack_configure).
        /// 
        /// It is up to the compositor to position the surface after it was
        /// unmaximized; usually the position the surface had before maximizing, if
        /// applicable.
        /// 
        /// If the surface was already not maximized, the compositor will still
        /// emit a configure event without the "maximized" state.
        UnsetMaximized,
        /// set the window as fullscreen on a monitor
        ///
        /// Make the surface fullscreen.
        /// 
        /// You can specify an output that you would prefer to be fullscreen.
        /// If this value is NULL, it's up to the compositor to choose which
        /// display will be used to map this surface.
        /// 
        /// If the surface doesn't cover the whole output, the compositor will
        /// position the surface in the center of the output and compensate with
        /// black borders filling the rest of the output.
        SetFullscreen {output: Option<Proxy<super::wl_output::WlOutput>>, },
        UnsetFullscreen,
        /// set the window as minimized
        ///
        /// Request that the compositor minimize your surface. There is no
        /// way to know if the surface is currently minimized, nor is there
        /// any way to unset minimization on this surface.
        /// 
        /// If you are looking to throttle redrawing when minimized, please
        /// instead use the wl_surface.frame event for this, as this will
        /// also work with live previews on windows in Alt-Tab, Expose or
        /// similar compositor features.
        SetMinimized,
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
                Request::SetParent { parent, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = parent.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    f(1, &mut _args_array)
                },
                Request::SetTitle { title, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(title).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(2, &mut _args_array)
                },
                Request::SetAppId { app_id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(app_id).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(3, &mut _args_array)
                },
                Request::ShowWindowMenu { seat, serial, x, y, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    _args_array[2].i = x;
                    _args_array[3].i = y;
                    f(4, &mut _args_array)
                },
                Request::Move { seat, serial, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    f(5, &mut _args_array)
                },
                Request::Resize { seat, serial, edges, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    _args_array[2].u = edges;
                    f(6, &mut _args_array)
                },
                Request::AckConfigure { serial, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(7, &mut _args_array)
                },
                Request::SetWindowGeometry { x, y, width, height, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(8, &mut _args_array)
                },
                Request::SetMaximized => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(9, &mut _args_array)
                },
                Request::UnsetMaximized => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(10, &mut _args_array)
                },
                Request::SetFullscreen { output, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = output.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    f(11, &mut _args_array)
                },
                Request::UnsetFullscreen => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(12, &mut _args_array)
                },
                Request::SetMinimized => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(13, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// suggest a surface change
        ///
        /// The configure event asks the client to resize its surface or to
        /// change its state.
        /// 
        /// The width and height arguments specify a hint to the window
        /// about how its surface should be resized in window geometry
        /// coordinates. See set_window_geometry.
        /// 
        /// If the width or height arguments are zero, it means the client
        /// should decide its own window dimension. This may happen when the
        /// compositor need to configure the state of the surface but doesn't
        /// have any information about any previous or expected dimension.
        /// 
        /// The states listed in the event specify how the width/height
        /// arguments should be interpreted, and possibly how it should be
        /// drawn.
        /// 
        /// Clients should arrange their surface for the new size and
        /// states, and then send a ack_configure request with the serial
        /// sent in this configure event at some point before committing
        /// the new surface.
        /// 
        /// If the client receives multiple configure events before it
        /// can respond to one, it is free to discard all but the last
        /// event it received.
        Configure {width: i32, height: i32, states: Vec<u8>, serial: u32, },
        /// surface wants to be closed
        ///
        /// The close event is sent by the compositor when the user
        /// wants the surface to be closed. This should be equivalent to
        /// the user clicking the close button in client-side decorations,
        /// if your application has any...
        /// 
        /// This is only a request that the user intends to close your
        /// window. The client may choose to ignore this request, or show
        /// a dialog to ask the user to save their data...
        Close,
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
                    Ok(Event::Configure {
                        width: _args[0].i,
                        height: _args[1].i,
                        states: { let array = &*_args[2].a; ::std::slice::from_raw_parts(array.data as *const u8, array.size).to_owned() },
                        serial: _args[3].u,
                }) },
                1 => {
                    Ok(Event::Close) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct XdgSurface;

    impl Interface for XdgSurface {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_surface";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::xdg_surface_interface }
        }
    }

    pub trait RequestsTrait {
        /// Destroy the xdg_surface
        ///
        /// Unmap and destroy the window. The window will be effectively
        /// hidden from the user's point of view, and all state like
        /// maximization, fullscreen, and so on, will be lost.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// set the parent of this surface
        ///
        /// Set the "parent" of this surface. This window should be stacked
        /// above a parent. The parent surface must be mapped as long as this
        /// surface is mapped.
        /// 
        /// Parent windows should be set on dialogs, toolboxes, or other
        /// "auxiliary" surfaces, so that the parent is raised when the dialog
        /// is raised.
        fn set_parent(&self, parent: Option<&Proxy<super::xdg_surface::XdgSurface>>) ->();
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
        /// set application ID
        ///
        /// Set an application identifier for the surface.
        /// 
        /// The app ID identifies the general class of applications to which
        /// the surface belongs. The compositor can use this to group multiple
        /// surfaces together, or to determine how to launch a new application.
        /// 
        /// For D-Bus activatable applications, the app ID is used as the D-Bus
        /// service name.
        /// 
        /// The compositor shell will try to group application surfaces together
        /// by their app ID.  As a best practice, it is suggested to select app
        /// ID's that match the basename of the application's .desktop file.
        /// For example, "org.freedesktop.FooViewer" where the .desktop file is
        /// "org.freedesktop.FooViewer.desktop".
        /// 
        /// See the desktop-entry specification [0] for more details on
        /// application identifiers and how they relate to well-known D-Bus
        /// names and .desktop files.
        /// 
        /// [0] http://standards.freedesktop.org/desktop-entry-spec/
        fn set_app_id(&self, app_id: String) ->();
        /// show the window menu
        ///
        /// Clients implementing client-side decorations might want to show
        /// a context menu when right-clicking on the decorations, giving the
        /// user a menu that they can use to maximize or minimize the window.
        /// 
        /// This request asks the compositor to pop up such a window menu at
        /// the given position, relative to the local surface coordinates of
        /// the parent surface. There are no guarantees as to what menu items
        /// the window menu contains.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event.
        fn show_window_menu(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32, x: i32, y: i32) ->();
        /// start an interactive move
        ///
        /// Start an interactive, user-driven move of the surface.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event. The passed
        /// serial is used to determine the type of interactive move (touch,
        /// pointer, etc).
        /// 
        /// The server may ignore move requests depending on the state of
        /// the surface (e.g. fullscreen or maximized), or if the passed serial
        /// is no longer valid.
        /// 
        /// If triggered, the surface will lose the focus of the device
        /// (wl_pointer, wl_touch, etc) used for the move. It is up to the
        /// compositor to visually indicate that the move is taking place, such as
        /// updating a pointer cursor, during the move. There is no guarantee
        /// that the device focus will return when the move is completed.
        fn _move(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32) ->();
        /// start an interactive resize
        ///
        /// Start a user-driven, interactive resize of the surface.
        /// 
        /// This request must be used in response to some sort of user action
        /// like a button press, key press, or touch down event. The passed
        /// serial is used to determine the type of interactive resize (touch,
        /// pointer, etc).
        /// 
        /// The server may ignore resize requests depending on the state of
        /// the surface (e.g. fullscreen or maximized).
        /// 
        /// If triggered, the client will receive configure events with the
        /// "resize" state enum value and the expected sizes. See the "resize"
        /// enum value for more details about what is required. The client
        /// must also acknowledge configure events using "ack_configure". After
        /// the resize is completed, the client will receive another "configure"
        /// event without the resize state.
        /// 
        /// If triggered, the surface also will lose the focus of the device
        /// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
        /// compositor to visually indicate that the resize is taking place,
        /// such as updating a pointer cursor, during the resize. There is no
        /// guarantee that the device focus will return when the resize is
        /// completed.
        /// 
        /// The edges parameter specifies how the surface should be resized,
        /// and is one of the values of the resize_edge enum. The compositor
        /// may use this information to update the surface position for
        /// example when dragging the top left corner. The compositor may also
        /// use this information to adapt its behavior, e.g. choose an
        /// appropriate cursor image.
        fn resize(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32, edges: u32) ->();
        /// ack a configure event
        ///
        /// When a configure event is received, if a client commits the
        /// surface in response to the configure event, then the client
        /// must make an ack_configure request sometime before the commit
        /// request, passing along the serial of the configure event.
        /// 
        /// For instance, the compositor might use this information to move
        /// a surface to the top left only when the client has drawn itself
        /// for the maximized or fullscreen state.
        /// 
        /// If the client receives multiple configure events before it
        /// can respond to one, it only has to ack the last configure event.
        /// 
        /// A client is not required to commit immediately after sending
        /// an ack_configure request - it may even ack_configure several times
        /// before its next surface commit.
        /// 
        /// The compositor expects that the most recently received
        /// ack_configure request at the time of a commit indicates which
        /// configure event the client is responding to.
        fn ack_configure(&self, serial: u32) ->();
        /// set the new window geometry
        ///
        /// The window geometry of a window is its "visible bounds" from the
        /// user's perspective. Client-side decorations often have invisible
        /// portions like drop-shadows which should be ignored for the
        /// purposes of aligning, placing and constraining windows.
        /// 
        /// The window geometry is double buffered, and will be applied at the
        /// time wl_surface.commit of the corresponding wl_surface is called.
        /// 
        /// Once the window geometry of the surface is set once, it is not
        /// possible to unset it, and it will remain the same until
        /// set_window_geometry is called again, even if a new subsurface or
        /// buffer is attached.
        /// 
        /// If never set, the value is the full bounds of the surface,
        /// including any subsurfaces. This updates dynamically on every
        /// commit. This unset mode is meant for extremely simple clients.
        /// 
        /// If responding to a configure event, the window geometry in here
        /// must respect the sizing negotiations specified by the states in
        /// the configure event.
        /// 
        /// The arguments are given in the surface local coordinate space of
        /// the wl_surface associated with this xdg_surface.
        /// 
        /// The width and height must be greater than zero.
        fn set_window_geometry(&self, x: i32, y: i32, width: i32, height: i32) ->();
        /// maximize the window
        ///
        /// Maximize the surface.
        /// 
        /// After requesting that the surface should be maximized, the compositor
        /// will respond by emitting a configure event with the "maximized" state
        /// and the required window geometry. The client should then update its
        /// content, drawing it in a maximized state, i.e. without shadow or other
        /// decoration outside of the window geometry. The client must also
        /// acknowledge the configure when committing the new content (see
        /// ack_configure).
        /// 
        /// It is up to the compositor to decide how and where to maximize the
        /// surface, for example which output and what region of the screen should
        /// be used.
        /// 
        /// If the surface was already maximized, the compositor will still emit
        /// a configure event with the "maximized" state.
        fn set_maximized(&self) ->();
        /// unmaximize the window
        ///
        /// Unmaximize the surface.
        /// 
        /// After requesting that the surface should be unmaximized, the compositor
        /// will respond by emitting a configure event without the "maximized"
        /// state. If available, the compositor will include the window geometry
        /// dimensions the window had prior to being maximized in the configure
        /// request. The client must then update its content, drawing it in a
        /// regular state, i.e. potentially with shadow, etc. The client must also
        /// acknowledge the configure when committing the new content (see
        /// ack_configure).
        /// 
        /// It is up to the compositor to position the surface after it was
        /// unmaximized; usually the position the surface had before maximizing, if
        /// applicable.
        /// 
        /// If the surface was already not maximized, the compositor will still
        /// emit a configure event without the "maximized" state.
        fn unset_maximized(&self) ->();
        /// set the window as fullscreen on a monitor
        ///
        /// Make the surface fullscreen.
        /// 
        /// You can specify an output that you would prefer to be fullscreen.
        /// If this value is NULL, it's up to the compositor to choose which
        /// display will be used to map this surface.
        /// 
        /// If the surface doesn't cover the whole output, the compositor will
        /// position the surface in the center of the output and compensate with
        /// black borders filling the rest of the output.
        fn set_fullscreen(&self, output: Option<&Proxy<super::wl_output::WlOutput>>) ->();
        fn unset_fullscreen(&self) ->();
        /// set the window as minimized
        ///
        /// Request that the compositor minimize your surface. There is no
        /// way to know if the surface is currently minimized, nor is there
        /// any way to unset minimization on this surface.
        /// 
        /// If you are looking to throttle redrawing when minimized, please
        /// instead use the wl_surface.frame event for this, as this will
        /// also work with live previews on windows in Alt-Tab, Expose or
        /// similar compositor features.
        fn set_minimized(&self) ->();
    }

    impl RequestsTrait for Proxy<XdgSurface> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn set_parent(&self, parent: Option<&Proxy<super::xdg_surface::XdgSurface>>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetParent {
                parent : parent.map(|o| o.clone()),
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

        fn set_app_id(&self, app_id: String) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetAppId {
                app_id: app_id,
            };
            self.send(msg);
        }

        fn show_window_menu(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32, x: i32, y: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::ShowWindowMenu {
                seat: seat.clone(),
                serial: serial,
                x: x,
                y: y,
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

        fn resize(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32, edges: u32) ->() {
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

        fn ack_configure(&self, serial: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::AckConfigure {
                serial: serial,
            };
            self.send(msg);
        }

        fn set_window_geometry(&self, x: i32, y: i32, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetWindowGeometry {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.send(msg);
        }

        fn set_maximized(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetMaximized;
            self.send(msg);
        }

        fn unset_maximized(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::UnsetMaximized;
            self.send(msg);
        }

        fn set_fullscreen(&self, output: Option<&Proxy<super::wl_output::WlOutput>>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetFullscreen {
                output : output.map(|o| o.clone()),
            };
            self.send(msg);
        }

        fn unset_fullscreen(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::UnsetFullscreen;
            self.send(msg);
        }

        fn set_minimized(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetMinimized;
            self.send(msg);
        }

    }
}

pub mod xdg_popup {
    //! short-lived, popup surfaces for menus
    //!
    //! A popup surface is a short-lived, temporary surface that can be
    //! used to implement menus. It takes an explicit grab on the surface
    //! that will be dismissed when the user dismisses the popup. This can
    //! be done by the user clicking outside the surface, using the keyboard,
    //! or even locking the screen through closing the lid or a timeout.
    //! 
    //! When the popup is dismissed, a popup_done event will be sent out,
    //! and at the same time the surface will be unmapped. The xdg_popup
    //! object is now inert and cannot be reactivated, so clients should
    //! destroy it. Explicitly destroying the xdg_popup object will also
    //! dismiss the popup and unmap the surface.
    //! 
    //! Clients will receive events for all their surfaces during this
    //! grab (which is an "owner-events" grab in X11 parlance). This is
    //! done so that users can navigate through submenus and other
    //! "nested" popup windows without having to dismiss the topmost
    //! popup.
    //! 
    //! Clients that want to dismiss the popup when another surface of
    //! their own is clicked should dismiss the popup using the destroy
    //! request.
    //! 
    //! The parent surface must have either an xdg_surface or xdg_popup
    //! role.
    //! 
    //! Specifying an xdg_popup for the parent means that the popups are
    //! nested, with this popup now being the topmost popup. Nested
    //! popups must be destroyed in the reverse order they were created
    //! in, e.g. the only popup you are allowed to destroy at all times
    //! is the topmost one.
    //! 
    //! If there is an existing popup when creating a new popup, the
    //! parent must be the current topmost popup.
    //! 
    //! A parent surface must be mapped before the new popup is mapped.
    //! 
    //! When compositors choose to dismiss a popup, they will likely
    //! dismiss every nested popup as well. When a compositor dismisses
    //! popups, it will follow the same dismissing order as required
    //! from the client.
    //! 
    //! The x and y arguments passed when creating the popup object specify
    //! where the top left of the popup should be placed, relative to the
    //! local surface coordinates of the parent surface. See
    //! xdg_shell.get_xdg_popup.
    //! 
    //! The client must call wl_surface.commit on the corresponding wl_surface
    //! for the xdg_popup state to take effect.
    //! 
    //! For a surface to be mapped by the compositor the client must have
    //! committed both the xdg_popup state and a buffer.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// remove xdg_popup interface
        ///
        /// This destroys the popup. Explicitly destroying the xdg_popup
        /// object will also dismiss the popup, and unmap the surface.
        /// 
        /// If this xdg_popup is not the "topmost" popup, a protocol error
        /// will be sent.
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
        /// popup interaction is done
        ///
        /// The popup_done event is sent out when a popup is dismissed by the
        /// compositor. The client should destroy the xdg_popup object at this
        /// point.
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
                    Ok(Event::PopupDone) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct XdgPopup;

    impl Interface for XdgPopup {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_popup";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::xdg_popup_interface }
        }
    }

    pub trait RequestsTrait {
        /// remove xdg_popup interface
        ///
        /// This destroys the popup. Explicitly destroying the xdg_popup
        /// object will also dismiss the popup, and unmap the surface.
        /// 
        /// If this xdg_popup is not the "topmost" popup, a protocol error
        /// will be sent.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<XdgPopup> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

