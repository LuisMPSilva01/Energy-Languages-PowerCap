
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2008-2013 Kristian Høgsberg
    Copyright © 2013      Rafael Antognolli
    Copyright © 2013      Jasper St. Pierre
    Copyright © 2010-2013 Intel Corporation
    Copyright © 2015-2017 Samsung Electronics Co., Ltd
    Copyright © 2015-2017 Red Hat Inc.

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

pub mod xdg_wm_base {
    //! create desktop-style surfaces
    //!
    //! The xdg_wm_base interface is exposed as a global object enabling clients
    //! to turn their wl_surfaces into windows in a desktop environment. It
    //! defines the basic functionality needed for clients and the compositor to
    //! create windows that can be dragged, resized, maximized, etc, as well as
    //! creating transient windows such as popup menus.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// given wl_surface has another role
        Role = 0,
        /// xdg_wm_base was destroyed before children
        DefunctSurfaces = 1,
        /// the client tried to map or destroy a non-topmost popup
        NotTheTopmostPopup = 2,
        /// the client specified an invalid popup parent surface
        InvalidPopupParent = 3,
        /// the client provided an invalid surface state
        InvalidSurfaceState = 4,
        /// the client provided an invalid positioner
        InvalidPositioner = 5,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::Role),
                1 => Some(Error::DefunctSurfaces),
                2 => Some(Error::NotTheTopmostPopup),
                3 => Some(Error::InvalidPopupParent),
                4 => Some(Error::InvalidSurfaceState),
                5 => Some(Error::InvalidPositioner),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// destroy xdg_wm_base
        ///
        /// Destroy this xdg_wm_base object.
        /// 
        /// Destroying a bound xdg_wm_base object while there are surfaces
        /// still alive created by this xdg_wm_base object instance is illegal
        /// and will result in a protocol error.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// create a positioner object
        ///
        /// Create a positioner object. A positioner object is used to position
        /// surfaces relative to some parent surface. See the interface description
        /// and xdg_surface.get_popup for details.
        CreatePositioner {id: Proxy<super::xdg_positioner::XdgPositioner>, },
        /// create a shell surface from a surface
        ///
        /// This creates an xdg_surface for the given surface. While xdg_surface
        /// itself is not a role, the corresponding surface may only be assigned
        /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup.
        /// 
        /// This creates an xdg_surface for the given surface. An xdg_surface is
        /// used as basis to define a role to a given surface, such as xdg_toplevel
        /// or xdg_popup. It also manages functionality shared between xdg_surface
        /// based surface roles.
        /// 
        /// See the documentation of xdg_surface for more details about what an
        /// xdg_surface is and how it is used.
        GetXdgSurface {id: Proxy<super::xdg_surface::XdgSurface>, surface: Proxy<super::wl_surface::WlSurface>, },
        /// respond to a ping event
        ///
        /// A client must respond to a ping event with a pong request or
        /// the client may be deemed unresponsive. See xdg_wm_base.ping.
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
                Request::CreatePositioner { id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
                Request::GetXdgSurface { id, surface, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    f(2, &mut _args_array)
                },
                Request::Pong { serial, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(3, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// check if the client is alive
        ///
        /// The ping event asks the client if it's still alive. Pass the
        /// serial specified in the event back to the compositor by sending
        /// a "pong" request back with the specified serial. See xdg_wm_base.ping.
        /// 
        /// Compositors can use this to determine if the client is still
        /// alive. It's unspecified what will happen if the client doesn't
        /// respond to the ping request, or in what timeframe. Clients should
        /// try to respond in a reasonable amount of time.
        /// 
        /// A compositor is free to ping in any way it wants, but a client must
        /// always respond to any xdg_wm_base object it created.
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


    pub struct XdgWmBase;

    impl Interface for XdgWmBase {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_wm_base";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::xdg_wm_base_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy xdg_wm_base
        ///
        /// Destroy this xdg_wm_base object.
        /// 
        /// Destroying a bound xdg_wm_base object while there are surfaces
        /// still alive created by this xdg_wm_base object instance is illegal
        /// and will result in a protocol error.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// create a positioner object
        ///
        /// Create a positioner object. A positioner object is used to position
        /// surfaces relative to some parent surface. See the interface description
        /// and xdg_surface.get_popup for details.
        fn create_positioner(&self) ->Result<NewProxy<super::xdg_positioner::XdgPositioner>, ()>;
        /// create a shell surface from a surface
        ///
        /// This creates an xdg_surface for the given surface. While xdg_surface
        /// itself is not a role, the corresponding surface may only be assigned
        /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup.
        /// 
        /// This creates an xdg_surface for the given surface. An xdg_surface is
        /// used as basis to define a role to a given surface, such as xdg_toplevel
        /// or xdg_popup. It also manages functionality shared between xdg_surface
        /// based surface roles.
        /// 
        /// See the documentation of xdg_surface for more details about what an
        /// xdg_surface is and how it is used.
        fn get_xdg_surface(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::xdg_surface::XdgSurface>, ()>;
        /// respond to a ping event
        ///
        /// A client must respond to a ping event with a pong request or
        /// the client may be deemed unresponsive. See xdg_wm_base.ping.
        fn pong(&self, serial: u32) ->();
    }

    impl RequestsTrait for Proxy<XdgWmBase> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn create_positioner(&self) ->Result<NewProxy<super::xdg_positioner::XdgPositioner>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::xdg_positioner::XdgPositioner>();
            let msg = Request::CreatePositioner {
                id: unsafe { Proxy::<super::xdg_positioner::XdgPositioner>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
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

pub mod xdg_positioner {
    //! child surface positioner
    //!
    //! The xdg_positioner provides a collection of rules for the placement of a
    //! child surface relative to a parent surface. Rules can be defined to ensure
    //! the child surface remains within the visible area's borders, and to
    //! specify how the child surface changes its position, such as sliding along
    //! an axis, or flipping around a rectangle. These positioner-created rules are
    //! constrained by the requirement that a child surface must intersect with or
    //! be at least partially adjacent to its parent surface.
    //! 
    //! See the various requests for details about possible rules.
    //! 
    //! At the time of the request, the compositor makes a copy of the rules
    //! specified by the xdg_positioner. Thus, after the request is complete the
    //! xdg_positioner object can be destroyed or reused; further changes to the
    //! object will have no effect on previous usages.
    //! 
    //! For an xdg_positioner object to be considered complete, it must have a
    //! non-zero size set by set_size, and a non-zero anchor rectangle set by
    //! set_anchor_rect. Passing an incomplete xdg_positioner object when
    //! positioning a surface raises an error.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// invalid input provided
        InvalidInput = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidInput),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }


    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Anchor {
        None = 0,
        Top = 1,
        Bottom = 2,
        Left = 3,
        Right = 4,
        TopLeft = 5,
        BottomLeft = 6,
        TopRight = 7,
        BottomRight = 8,
    }
    impl Anchor {
        pub fn from_raw(n: u32) -> Option<Anchor> {
            match n {
                0 => Some(Anchor::None),
                1 => Some(Anchor::Top),
                2 => Some(Anchor::Bottom),
                3 => Some(Anchor::Left),
                4 => Some(Anchor::Right),
                5 => Some(Anchor::TopLeft),
                6 => Some(Anchor::BottomLeft),
                7 => Some(Anchor::TopRight),
                8 => Some(Anchor::BottomRight),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }


    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Gravity {
        None = 0,
        Top = 1,
        Bottom = 2,
        Left = 3,
        Right = 4,
        TopLeft = 5,
        BottomLeft = 6,
        TopRight = 7,
        BottomRight = 8,
    }
    impl Gravity {
        pub fn from_raw(n: u32) -> Option<Gravity> {
            match n {
                0 => Some(Gravity::None),
                1 => Some(Gravity::Top),
                2 => Some(Gravity::Bottom),
                3 => Some(Gravity::Left),
                4 => Some(Gravity::Right),
                5 => Some(Gravity::TopLeft),
                6 => Some(Gravity::BottomLeft),
                7 => Some(Gravity::TopRight),
                8 => Some(Gravity::BottomRight),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    bitflags! {
        /// constraint adjustments
        ///
        /// The constraint adjustment value define ways the compositor will adjust
        /// the position of the surface, if the unadjusted position would result
        /// in the surface being partly constrained.
        /// 
        /// Whether a surface is considered 'constrained' is left to the compositor
        /// to determine. For example, the surface may be partly outside the
        /// compositor's defined 'work area', thus necessitating the child surface's
        /// position be adjusted until it is entirely inside the work area.
        /// 
        /// The adjustments can be combined, according to a defined precedence: 1)
        /// Flip, 2) Slide, 3) Resize.
        pub struct ConstraintAdjustment: u32 {
            /// don't move the child surface when constrained
            ///
            /// Don't alter the surface position even if it is constrained on some
            /// axis, for example partially outside the edge of an output.
            const None = 0;
            /// move along the x axis until unconstrained
            ///
            /// Slide the surface along the x axis until it is no longer constrained.
            /// 
            /// First try to slide towards the direction of the gravity on the x axis
            /// until either the edge in the opposite direction of the gravity is
            /// unconstrained or the edge in the direction of the gravity is
            /// constrained.
            /// 
            /// Then try to slide towards the opposite direction of the gravity on the
            /// x axis until either the edge in the direction of the gravity is
            /// unconstrained or the edge in the opposite direction of the gravity is
            /// constrained.
            const SlideX = 1;
            /// move along the y axis until unconstrained
            ///
            /// Slide the surface along the y axis until it is no longer constrained.
            /// 
            /// First try to slide towards the direction of the gravity on the y axis
            /// until either the edge in the opposite direction of the gravity is
            /// unconstrained or the edge in the direction of the gravity is
            /// constrained.
            /// 
            /// Then try to slide towards the opposite direction of the gravity on the
            /// y axis until either the edge in the direction of the gravity is
            /// unconstrained or the edge in the opposite direction of the gravity is
            /// constrained.
            const SlideY = 2;
            /// invert the anchor and gravity on the x axis
            ///
            /// Invert the anchor and gravity on the x axis if the surface is
            /// constrained on the x axis. For example, if the left edge of the
            /// surface is constrained, the gravity is 'left' and the anchor is
            /// 'left', change the gravity to 'right' and the anchor to 'right'.
            /// 
            /// If the adjusted position also ends up being constrained, the resulting
            /// position of the flip_x adjustment will be the one before the
            /// adjustment.
            const FlipX = 4;
            /// invert the anchor and gravity on the y axis
            ///
            /// Invert the anchor and gravity on the y axis if the surface is
            /// constrained on the y axis. For example, if the bottom edge of the
            /// surface is constrained, the gravity is 'bottom' and the anchor is
            /// 'bottom', change the gravity to 'top' and the anchor to 'top'.
            /// 
            /// The adjusted position is calculated given the original anchor
            /// rectangle and offset, but with the new flipped anchor and gravity
            /// values.
            /// 
            /// If the adjusted position also ends up being constrained, the resulting
            /// position of the flip_y adjustment will be the one before the
            /// adjustment.
            const FlipY = 8;
            /// horizontally resize the surface
            ///
            /// Resize the surface horizontally so that it is completely
            /// unconstrained.
            const ResizeX = 16;
            /// vertically resize the surface
            ///
            /// Resize the surface vertically so that it is completely unconstrained.
            const ResizeY = 32;
        }
    }
    impl ConstraintAdjustment {
        pub fn from_raw(n: u32) -> Option<ConstraintAdjustment> {
            Some(ConstraintAdjustment::from_bits_truncate(n))

        }
        pub fn to_raw(&self) -> u32 {
            self.bits()
        }
    }

    pub enum Request {
        /// destroy the xdg_positioner object
        ///
        /// Notify the compositor that the xdg_positioner will no longer be used.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// set the size of the to-be positioned rectangle
        ///
        /// Set the size of the surface that is to be positioned with the positioner
        /// object. The size is in surface-local coordinates and corresponds to the
        /// window geometry. See xdg_surface.set_window_geometry.
        /// 
        /// If a zero or negative size is set the invalid_input error is raised.
        SetSize {width: i32, height: i32, },
        /// set the anchor rectangle within the parent surface
        ///
        /// Specify the anchor rectangle within the parent surface that the child
        /// surface will be placed relative to. The rectangle is relative to the
        /// window geometry as defined by xdg_surface.set_window_geometry of the
        /// parent surface.
        /// 
        /// When the xdg_positioner object is used to position a child surface, the
        /// anchor rectangle may not extend outside the window geometry of the
        /// positioned child's parent surface.
        /// 
        /// If a negative size is set the invalid_input error is raised.
        SetAnchorRect {x: i32, y: i32, width: i32, height: i32, },
        /// set anchor rectangle anchor
        ///
        /// Defines the anchor point for the anchor rectangle. The specified anchor
        /// is used derive an anchor point that the child surface will be
        /// positioned relative to. If a corner anchor is set (e.g. 'top_left' or
        /// 'bottom_right'), the anchor point will be at the specified corner;
        /// otherwise, the derived anchor point will be centered on the specified
        /// edge, or in the center of the anchor rectangle if no edge is specified.
        SetAnchor {anchor: Anchor, },
        /// set child surface gravity
        ///
        /// Defines in what direction a surface should be positioned, relative to
        /// the anchor point of the parent surface. If a corner gravity is
        /// specified (e.g. 'bottom_right' or 'top_left'), then the child surface
        /// will be placed towards the specified gravity; otherwise, the child
        /// surface will be centered over the anchor point on any axis that had no
        /// gravity specified.
        SetGravity {gravity: Gravity, },
        /// set the adjustment to be done when constrained
        ///
        /// Specify how the window should be positioned if the originally intended
        /// position caused the surface to be constrained, meaning at least
        /// partially outside positioning boundaries set by the compositor. The
        /// adjustment is set by constructing a bitmask describing the adjustment to
        /// be made when the surface is constrained on that axis.
        /// 
        /// If no bit for one axis is set, the compositor will assume that the child
        /// surface should not change its position on that axis when constrained.
        /// 
        /// If more than one bit for one axis is set, the order of how adjustments
        /// are applied is specified in the corresponding adjustment descriptions.
        /// 
        /// The default adjustment is none.
        SetConstraintAdjustment {constraint_adjustment: u32, },
        /// set surface position offset
        ///
        /// Specify the surface position offset relative to the position of the
        /// anchor on the anchor rectangle and the anchor on the surface. For
        /// example if the anchor of the anchor rectangle is at (x, y), the surface
        /// has the gravity bottom|right, and the offset is (ox, oy), the calculated
        /// surface position will be (x + ox, y + oy). The offset position of the
        /// surface is the one used for constraint testing. See
        /// set_constraint_adjustment.
        /// 
        /// An example use case is placing a popup menu on top of a user interface
        /// element, while aligning the user interface element of the parent surface
        /// with some user interface element placed somewhere in the popup surface.
        SetOffset {x: i32, y: i32, },
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
                Request::SetSize { width, height, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = width;
                    _args_array[1].i = height;
                    f(1, &mut _args_array)
                },
                Request::SetAnchorRect { x, y, width, height, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(2, &mut _args_array)
                },
                Request::SetAnchor { anchor, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = anchor.to_raw();
                    f(3, &mut _args_array)
                },
                Request::SetGravity { gravity, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = gravity.to_raw();
                    f(4, &mut _args_array)
                },
                Request::SetConstraintAdjustment { constraint_adjustment, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = constraint_adjustment;
                    f(5, &mut _args_array)
                },
                Request::SetOffset { x, y, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    f(6, &mut _args_array)
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


    pub struct XdgPositioner;

    impl Interface for XdgPositioner {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_positioner";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::xdg_positioner_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the xdg_positioner object
        ///
        /// Notify the compositor that the xdg_positioner will no longer be used.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// set the size of the to-be positioned rectangle
        ///
        /// Set the size of the surface that is to be positioned with the positioner
        /// object. The size is in surface-local coordinates and corresponds to the
        /// window geometry. See xdg_surface.set_window_geometry.
        /// 
        /// If a zero or negative size is set the invalid_input error is raised.
        fn set_size(&self, width: i32, height: i32) ->();
        /// set the anchor rectangle within the parent surface
        ///
        /// Specify the anchor rectangle within the parent surface that the child
        /// surface will be placed relative to. The rectangle is relative to the
        /// window geometry as defined by xdg_surface.set_window_geometry of the
        /// parent surface.
        /// 
        /// When the xdg_positioner object is used to position a child surface, the
        /// anchor rectangle may not extend outside the window geometry of the
        /// positioned child's parent surface.
        /// 
        /// If a negative size is set the invalid_input error is raised.
        fn set_anchor_rect(&self, x: i32, y: i32, width: i32, height: i32) ->();
        /// set anchor rectangle anchor
        ///
        /// Defines the anchor point for the anchor rectangle. The specified anchor
        /// is used derive an anchor point that the child surface will be
        /// positioned relative to. If a corner anchor is set (e.g. 'top_left' or
        /// 'bottom_right'), the anchor point will be at the specified corner;
        /// otherwise, the derived anchor point will be centered on the specified
        /// edge, or in the center of the anchor rectangle if no edge is specified.
        fn set_anchor(&self, anchor: Anchor) ->();
        /// set child surface gravity
        ///
        /// Defines in what direction a surface should be positioned, relative to
        /// the anchor point of the parent surface. If a corner gravity is
        /// specified (e.g. 'bottom_right' or 'top_left'), then the child surface
        /// will be placed towards the specified gravity; otherwise, the child
        /// surface will be centered over the anchor point on any axis that had no
        /// gravity specified.
        fn set_gravity(&self, gravity: Gravity) ->();
        /// set the adjustment to be done when constrained
        ///
        /// Specify how the window should be positioned if the originally intended
        /// position caused the surface to be constrained, meaning at least
        /// partially outside positioning boundaries set by the compositor. The
        /// adjustment is set by constructing a bitmask describing the adjustment to
        /// be made when the surface is constrained on that axis.
        /// 
        /// If no bit for one axis is set, the compositor will assume that the child
        /// surface should not change its position on that axis when constrained.
        /// 
        /// If more than one bit for one axis is set, the order of how adjustments
        /// are applied is specified in the corresponding adjustment descriptions.
        /// 
        /// The default adjustment is none.
        fn set_constraint_adjustment(&self, constraint_adjustment: u32) ->();
        /// set surface position offset
        ///
        /// Specify the surface position offset relative to the position of the
        /// anchor on the anchor rectangle and the anchor on the surface. For
        /// example if the anchor of the anchor rectangle is at (x, y), the surface
        /// has the gravity bottom|right, and the offset is (ox, oy), the calculated
        /// surface position will be (x + ox, y + oy). The offset position of the
        /// surface is the one used for constraint testing. See
        /// set_constraint_adjustment.
        /// 
        /// An example use case is placing a popup menu on top of a user interface
        /// element, while aligning the user interface element of the parent surface
        /// with some user interface element placed somewhere in the popup surface.
        fn set_offset(&self, x: i32, y: i32) ->();
    }

    impl RequestsTrait for Proxy<XdgPositioner> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn set_size(&self, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetSize {
                width: width,
                height: height,
            };
            self.send(msg);
        }

        fn set_anchor_rect(&self, x: i32, y: i32, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetAnchorRect {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.send(msg);
        }

        fn set_anchor(&self, anchor: Anchor) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetAnchor {
                anchor: anchor,
            };
            self.send(msg);
        }

        fn set_gravity(&self, gravity: Gravity) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetGravity {
                gravity: gravity,
            };
            self.send(msg);
        }

        fn set_constraint_adjustment(&self, constraint_adjustment: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetConstraintAdjustment {
                constraint_adjustment: constraint_adjustment,
            };
            self.send(msg);
        }

        fn set_offset(&self, x: i32, y: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetOffset {
                x: x,
                y: y,
            };
            self.send(msg);
        }

    }
}

pub mod xdg_surface {
    //! desktop user interface surface base interface
    //!
    //! An interface that may be implemented by a wl_surface, for
    //! implementations that provide a desktop-style user interface.
    //! 
    //! It provides a base set of functionality required to construct user
    //! interface elements requiring management by the compositor, such as
    //! toplevel windows, menus, etc. The types of functionality are split into
    //! xdg_surface roles.
    //! 
    //! Creating an xdg_surface does not set the role for a wl_surface. In order
    //! to map an xdg_surface, the client must create a role-specific object
    //! using, e.g., get_toplevel, get_popup. The wl_surface for any given
    //! xdg_surface can have at most one role, and may not be assigned any role
    //! not based on xdg_surface.
    //! 
    //! A role must be assigned before any other requests are made to the
    //! xdg_surface object.
    //! 
    //! The client must call wl_surface.commit on the corresponding wl_surface
    //! for the xdg_surface state to take effect.
    //! 
    //! Creating an xdg_surface from a wl_surface which has a buffer attached or
    //! committed is a client error, and any attempts by a client to attach or
    //! manipulate a buffer prior to the first xdg_surface.configure call must
    //! also be treated as errors.
    //! 
    //! Mapping an xdg_surface-based role surface is defined as making it
    //! possible for the surface to be shown by the compositor. Note that
    //! a mapped surface is not guaranteed to be visible once it is mapped.
    //! 
    //! For an xdg_surface to be mapped by the compositor, the following
    //! conditions must be met:
    //! (1) the client has assigned an xdg_surface-based role to the surface
    //! (2) the client has set and committed the xdg_surface state and the
    //! role-dependent state to the surface
    //! (3) the client has committed a buffer to the surface
    //! 
    //! A newly-unmapped surface is considered to have met condition (1) out
    //! of the 3 required conditions for mapping a surface if its role surface
    //! has not been destroyed.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        NotConstructed = 1,
        AlreadyConstructed = 2,
        UnconfiguredBuffer = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                1 => Some(Error::NotConstructed),
                2 => Some(Error::AlreadyConstructed),
                3 => Some(Error::UnconfiguredBuffer),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// destroy the xdg_surface
        ///
        /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
        /// after its role object has been destroyed.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// assign the xdg_toplevel surface role
        ///
        /// This creates an xdg_toplevel object for the given xdg_surface and gives
        /// the associated wl_surface the xdg_toplevel role.
        /// 
        /// See the documentation of xdg_toplevel for more details about what an
        /// xdg_toplevel is and how it is used.
        GetToplevel {id: Proxy<super::xdg_toplevel::XdgToplevel>, },
        /// assign the xdg_popup surface role
        ///
        /// This creates an xdg_popup object for the given xdg_surface and gives
        /// the associated wl_surface the xdg_popup role.
        /// 
        /// If null is passed as a parent, a parent surface must be specified using
        /// some other protocol, before committing the initial state.
        /// 
        /// See the documentation of xdg_popup for more details about what an
        /// xdg_popup is and how it is used.
        GetPopup {id: Proxy<super::xdg_popup::XdgPopup>, parent: Option<Proxy<super::xdg_surface::XdgSurface>>, positioner: Proxy<super::xdg_positioner::XdgPositioner>, },
        /// set the new window geometry
        ///
        /// The window geometry of a surface is its "visible bounds" from the
        /// user's perspective. Client-side decorations often have invisible
        /// portions like drop-shadows which should be ignored for the
        /// purposes of aligning, placing and constraining windows.
        /// 
        /// The window geometry is double buffered, and will be applied at the
        /// time wl_surface.commit of the corresponding wl_surface is called.
        /// 
        /// When maintaining a position, the compositor should treat the (x, y)
        /// coordinate of the window geometry as the top left corner of the window.
        /// A client changing the (x, y) window geometry coordinate should in
        /// general not alter the position of the window.
        /// 
        /// Once the window geometry of the surface is set, it is not possible to
        /// unset it, and it will remain the same until set_window_geometry is
        /// called again, even if a new subsurface or buffer is attached.
        /// 
        /// If never set, the value is the full bounds of the surface,
        /// including any subsurfaces. This updates dynamically on every
        /// commit. This unset is meant for extremely simple clients.
        /// 
        /// The arguments are given in the surface-local coordinate space of
        /// the wl_surface associated with this xdg_surface.
        /// 
        /// The width and height must be greater than zero. Setting an invalid size
        /// will raise an error. When applied, the effective window geometry will be
        /// the set window geometry clamped to the bounding rectangle of the
        /// combined geometry of the surface of the xdg_surface and the associated
        /// subsurfaces.
        SetWindowGeometry {x: i32, y: i32, width: i32, height: i32, },
        /// ack a configure event
        ///
        /// When a configure event is received, if a client commits the
        /// surface in response to the configure event, then the client
        /// must make an ack_configure request sometime before the commit
        /// request, passing along the serial of the configure event.
        /// 
        /// For instance, for toplevel surfaces the compositor might use this
        /// information to move a surface to the top left only when the client has
        /// drawn itself for the maximized or fullscreen state.
        /// 
        /// If the client receives multiple configure events before it
        /// can respond to one, it only has to ack the last configure event.
        /// 
        /// A client is not required to commit immediately after sending
        /// an ack_configure request - it may even ack_configure several times
        /// before its next surface commit.
        /// 
        /// A client may send multiple ack_configure requests before committing, but
        /// only the last request sent before a commit indicates which configure
        /// event the client really is responding to.
        AckConfigure {serial: u32, },
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
                Request::GetToplevel { id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
                Request::GetPopup { id, parent, positioner, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = parent.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[2].o = positioner.c_ptr() as *mut _;
                    f(2, &mut _args_array)
                },
                Request::SetWindowGeometry { x, y, width, height, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(3, &mut _args_array)
                },
                Request::AckConfigure { serial, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(4, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// suggest a surface change
        ///
        /// The configure event marks the end of a configure sequence. A configure
        /// sequence is a set of one or more events configuring the state of the
        /// xdg_surface, including the final xdg_surface.configure event.
        /// 
        /// Where applicable, xdg_surface surface roles will during a configure
        /// sequence extend this event as a latched state sent as events before the
        /// xdg_surface.configure event. Such events should be considered to make up
        /// a set of atomically applied configuration states, where the
        /// xdg_surface.configure commits the accumulated state.
        /// 
        /// Clients should arrange their surface for the new states, and then send
        /// an ack_configure request with the serial sent in this configure event at
        /// some point before committing the new surface.
        /// 
        /// If the client receives multiple configure events before it can respond
        /// to one, it is free to discard all but the last event it received.
        Configure {serial: u32, },
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
                        serial: _args[0].u,
                }) },
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
        /// destroy the xdg_surface
        ///
        /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
        /// after its role object has been destroyed.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// assign the xdg_toplevel surface role
        ///
        /// This creates an xdg_toplevel object for the given xdg_surface and gives
        /// the associated wl_surface the xdg_toplevel role.
        /// 
        /// See the documentation of xdg_toplevel for more details about what an
        /// xdg_toplevel is and how it is used.
        fn get_toplevel(&self) ->Result<NewProxy<super::xdg_toplevel::XdgToplevel>, ()>;
        /// assign the xdg_popup surface role
        ///
        /// This creates an xdg_popup object for the given xdg_surface and gives
        /// the associated wl_surface the xdg_popup role.
        /// 
        /// If null is passed as a parent, a parent surface must be specified using
        /// some other protocol, before committing the initial state.
        /// 
        /// See the documentation of xdg_popup for more details about what an
        /// xdg_popup is and how it is used.
        fn get_popup(&self, parent: Option<&Proxy<super::xdg_surface::XdgSurface>>, positioner: &Proxy<super::xdg_positioner::XdgPositioner>) ->Result<NewProxy<super::xdg_popup::XdgPopup>, ()>;
        /// set the new window geometry
        ///
        /// The window geometry of a surface is its "visible bounds" from the
        /// user's perspective. Client-side decorations often have invisible
        /// portions like drop-shadows which should be ignored for the
        /// purposes of aligning, placing and constraining windows.
        /// 
        /// The window geometry is double buffered, and will be applied at the
        /// time wl_surface.commit of the corresponding wl_surface is called.
        /// 
        /// When maintaining a position, the compositor should treat the (x, y)
        /// coordinate of the window geometry as the top left corner of the window.
        /// A client changing the (x, y) window geometry coordinate should in
        /// general not alter the position of the window.
        /// 
        /// Once the window geometry of the surface is set, it is not possible to
        /// unset it, and it will remain the same until set_window_geometry is
        /// called again, even if a new subsurface or buffer is attached.
        /// 
        /// If never set, the value is the full bounds of the surface,
        /// including any subsurfaces. This updates dynamically on every
        /// commit. This unset is meant for extremely simple clients.
        /// 
        /// The arguments are given in the surface-local coordinate space of
        /// the wl_surface associated with this xdg_surface.
        /// 
        /// The width and height must be greater than zero. Setting an invalid size
        /// will raise an error. When applied, the effective window geometry will be
        /// the set window geometry clamped to the bounding rectangle of the
        /// combined geometry of the surface of the xdg_surface and the associated
        /// subsurfaces.
        fn set_window_geometry(&self, x: i32, y: i32, width: i32, height: i32) ->();
        /// ack a configure event
        ///
        /// When a configure event is received, if a client commits the
        /// surface in response to the configure event, then the client
        /// must make an ack_configure request sometime before the commit
        /// request, passing along the serial of the configure event.
        /// 
        /// For instance, for toplevel surfaces the compositor might use this
        /// information to move a surface to the top left only when the client has
        /// drawn itself for the maximized or fullscreen state.
        /// 
        /// If the client receives multiple configure events before it
        /// can respond to one, it only has to ack the last configure event.
        /// 
        /// A client is not required to commit immediately after sending
        /// an ack_configure request - it may even ack_configure several times
        /// before its next surface commit.
        /// 
        /// A client may send multiple ack_configure requests before committing, but
        /// only the last request sent before a commit indicates which configure
        /// event the client really is responding to.
        fn ack_configure(&self, serial: u32) ->();
    }

    impl RequestsTrait for Proxy<XdgSurface> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn get_toplevel(&self) ->Result<NewProxy<super::xdg_toplevel::XdgToplevel>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::xdg_toplevel::XdgToplevel>();
            let msg = Request::GetToplevel {
                id: unsafe { Proxy::<super::xdg_toplevel::XdgToplevel>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn get_popup(&self, parent: Option<&Proxy<super::xdg_surface::XdgSurface>>, positioner: &Proxy<super::xdg_positioner::XdgPositioner>) ->Result<NewProxy<super::xdg_popup::XdgPopup>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::xdg_popup::XdgPopup>();
            let msg = Request::GetPopup {
                id: unsafe { Proxy::<super::xdg_popup::XdgPopup>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                parent : parent.map(|o| o.clone()),
                positioner: positioner.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
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

        fn ack_configure(&self, serial: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::AckConfigure {
                serial: serial,
            };
            self.send(msg);
        }

    }
}

pub mod xdg_toplevel {
    //! toplevel surface
    //!
    //! This interface defines an xdg_surface role which allows a surface to,
    //! among other things, set window-like properties such as maximize,
    //! fullscreen, and minimize, set application-specific metadata like title and
    //! id, and well as trigger user interactive operations such as interactive
    //! resize and move.
    //! 
    //! Unmapping an xdg_toplevel means that the surface cannot be shown
    //! by the compositor until it is explicitly mapped again.
    //! All active operations (e.g., move, resize) are canceled and all
    //! attributes (e.g. title, state, stacking, ...) are discarded for
    //! an xdg_toplevel surface when it is unmapped.
    //! 
    //! Attaching a null buffer to a toplevel unmaps the surface.
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
        /// The surface is fullscreen. The window geometry specified in the
        /// configure event is a maximum; the client cannot resize beyond it. For
        /// a surface to cover the whole fullscreened area, the geometry
        /// dimensions must be obeyed by the client. For more details, see
        /// xdg_toplevel.set_fullscreen.
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
        /// the surface is tiled
        ///
        /// The window is currently in a tiled layout and the left edge is
        /// considered to be adjacent to another part of the tiling grid.
        TiledLeft = 5,
        /// the surface is tiled
        ///
        /// The window is currently in a tiled layout and the right edge is
        /// considered to be adjacent to another part of the tiling grid.
        TiledRight = 6,
        /// the surface is tiled
        ///
        /// The window is currently in a tiled layout and the top edge is
        /// considered to be adjacent to another part of the tiling grid.
        TiledTop = 7,
        /// the surface is tiled
        ///
        /// The window is currently in a tiled layout and the bottom edge is
        /// considered to be adjacent to another part of the tiling grid.
        TiledBottom = 8,
    }
    impl State {
        pub fn from_raw(n: u32) -> Option<State> {
            match n {
                1 => Some(State::Maximized),
                2 => Some(State::Fullscreen),
                3 => Some(State::Resizing),
                4 => Some(State::Activated),
                5 => Some(State::TiledLeft),
                6 => Some(State::TiledRight),
                7 => Some(State::TiledTop),
                8 => Some(State::TiledBottom),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// destroy the xdg_toplevel
        ///
        /// This request destroys the role surface and unmaps the surface;
        /// see "Unmapping" behavior in interface section for details.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// set the parent of this surface
        ///
        /// Set the "parent" of this surface. This surface should be stacked
        /// above the parent surface and all other ancestor surfaces.
        /// 
        /// Parent windows should be set on dialogs, toolboxes, or other
        /// "auxiliary" surfaces, so that the parent is raised when the dialog
        /// is raised.
        /// 
        /// Setting a null parent for a child window removes any parent-child
        /// relationship for the child. Setting a null parent for a window which
        /// currently has no parent is a no-op.
        /// 
        /// If the parent is unmapped then its children are managed as
        /// though the parent of the now-unmapped parent has become the
        /// parent of this surface. If no parent exists for the now-unmapped
        /// parent then the children are managed as though they have no
        /// parent surface.
        SetParent {parent: Option<Proxy<super::xdg_toplevel::XdgToplevel>>, },
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
        /// by their app ID. As a best practice, it is suggested to select app
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
        /// set the maximum size
        ///
        /// Set a maximum size for the window.
        /// 
        /// The client can specify a maximum size so that the compositor does
        /// not try to configure the window beyond this size.
        /// 
        /// The width and height arguments are in window geometry coordinates.
        /// See xdg_surface.set_window_geometry.
        /// 
        /// Values set in this way are double-buffered. They will get applied
        /// on the next commit.
        /// 
        /// The compositor can use this information to allow or disallow
        /// different states like maximize or fullscreen and draw accurate
        /// animations.
        /// 
        /// Similarly, a tiling window manager may use this information to
        /// place and resize client windows in a more effective way.
        /// 
        /// The client should not rely on the compositor to obey the maximum
        /// size. The compositor may decide to ignore the values set by the
        /// client and request a larger size.
        /// 
        /// If never set, or a value of zero in the request, means that the
        /// client has no expected maximum size in the given dimension.
        /// As a result, a client wishing to reset the maximum size
        /// to an unspecified state can use zero for width and height in the
        /// request.
        /// 
        /// Requesting a maximum size to be smaller than the minimum size of
        /// a surface is illegal and will result in a protocol error.
        /// 
        /// The width and height must be greater than or equal to zero. Using
        /// strictly negative values for width and height will result in a
        /// protocol error.
        SetMaxSize {width: i32, height: i32, },
        /// set the minimum size
        ///
        /// Set a minimum size for the window.
        /// 
        /// The client can specify a minimum size so that the compositor does
        /// not try to configure the window below this size.
        /// 
        /// The width and height arguments are in window geometry coordinates.
        /// See xdg_surface.set_window_geometry.
        /// 
        /// Values set in this way are double-buffered. They will get applied
        /// on the next commit.
        /// 
        /// The compositor can use this information to allow or disallow
        /// different states like maximize or fullscreen and draw accurate
        /// animations.
        /// 
        /// Similarly, a tiling window manager may use this information to
        /// place and resize client windows in a more effective way.
        /// 
        /// The client should not rely on the compositor to obey the minimum
        /// size. The compositor may decide to ignore the values set by the
        /// client and request a smaller size.
        /// 
        /// If never set, or a value of zero in the request, means that the
        /// client has no expected minimum size in the given dimension.
        /// As a result, a client wishing to reset the minimum size
        /// to an unspecified state can use zero for width and height in the
        /// request.
        /// 
        /// Requesting a minimum size to be larger than the maximum size of
        /// a surface is illegal and will result in a protocol error.
        /// 
        /// The width and height must be greater than or equal to zero. Using
        /// strictly negative values for width and height will result in a
        /// protocol error.
        SetMinSize {width: i32, height: i32, },
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
        /// 
        /// If the surface is in a fullscreen state, this request has no direct
        /// effect. It will alter the state the surface is returned to when
        /// unmaximized if not overridden by the compositor.
        SetMaximized,
        /// unmaximize the window
        ///
        /// Unmaximize the surface.
        /// 
        /// After requesting that the surface should be unmaximized, the compositor
        /// will respond by emitting a configure event without the "maximized"
        /// state. If available, the compositor will include the window geometry
        /// dimensions the window had prior to being maximized in the configure
        /// event. The client must then update its content, drawing it in a
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
        /// 
        /// If the surface is in a fullscreen state, this request has no direct
        /// effect. It will alter the state the surface is returned to when
        /// unmaximized if not overridden by the compositor.
        UnsetMaximized,
        /// set the window as fullscreen on an output
        ///
        /// Make the surface fullscreen.
        /// 
        /// After requesting that the surface should be fullscreened, the
        /// compositor will respond by emitting a configure event with the
        /// "fullscreen" state and the fullscreen window geometry. The client must
        /// also acknowledge the configure when committing the new content (see
        /// ack_configure).
        /// 
        /// The output passed by the request indicates the client's preference as
        /// to which display it should be set fullscreen on. If this value is NULL,
        /// it's up to the compositor to choose which display will be used to map
        /// this surface.
        /// 
        /// If the surface doesn't cover the whole output, the compositor will
        /// position the surface in the center of the output and compensate with
        /// with border fill covering the rest of the output. The content of the
        /// border fill is undefined, but should be assumed to be in some way that
        /// attempts to blend into the surrounding area (e.g. solid black).
        /// 
        /// If the fullscreened surface is not opaque, the compositor must make
        /// sure that other screen content not part of the same surface tree (made
        /// up of subsurfaces, popups or similarly coupled surfaces) are not
        /// visible below the fullscreened surface.
        SetFullscreen {output: Option<Proxy<super::wl_output::WlOutput>>, },
        /// unset the window as fullscreen
        ///
        /// Make the surface no longer fullscreen.
        /// 
        /// After requesting that the surface should be unfullscreened, the
        /// compositor will respond by emitting a configure event without the
        /// "fullscreen" state.
        /// 
        /// Making a surface unfullscreen sets states for the surface based on the following:
        /// * the state(s) it may have had before becoming fullscreen
        /// * any state(s) decided by the compositor
        /// * any state(s) requested by the client while the surface was fullscreen
        /// 
        /// The compositor may include the previous window geometry dimensions in
        /// the configure event, if applicable.
        /// 
        /// The client must also acknowledge the configure when committing the new
        /// content (see ack_configure).
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
                Request::SetMaxSize { width, height, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = width;
                    _args_array[1].i = height;
                    f(7, &mut _args_array)
                },
                Request::SetMinSize { width, height, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = width;
                    _args_array[1].i = height;
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
        /// This configure event asks the client to resize its toplevel surface or
        /// to change its state. The configured state should not be applied
        /// immediately. See xdg_surface.configure for details.
        /// 
        /// The width and height arguments specify a hint to the window
        /// about how its surface should be resized in window geometry
        /// coordinates. See set_window_geometry.
        /// 
        /// If the width or height arguments are zero, it means the client
        /// should decide its own window dimension. This may happen when the
        /// compositor needs to configure the state of the surface but doesn't
        /// have any information about any previous or expected dimension.
        /// 
        /// The states listed in the event specify how the width/height
        /// arguments should be interpreted, and possibly how it should be
        /// drawn.
        /// 
        /// Clients must send an ack_configure in response to this event. See
        /// xdg_surface.configure and xdg_surface.ack_configure for details.
        Configure {width: i32, height: i32, states: Vec<u8>, },
        /// surface wants to be closed
        ///
        /// The close event is sent by the compositor when the user
        /// wants the surface to be closed. This should be equivalent to
        /// the user clicking the close button in client-side decorations,
        /// if your application has any.
        /// 
        /// This is only a request that the user intends to close the
        /// window. The client may choose to ignore this request, or show
        /// a dialog to ask the user to save their data, etc.
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
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Configure {
                        width: _args[0].i,
                        height: _args[1].i,
                        states: { let array = &*_args[2].a; ::std::slice::from_raw_parts(array.data as *const u8, array.size).to_owned() },
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


    pub struct XdgToplevel;

    impl Interface for XdgToplevel {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "xdg_toplevel";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::xdg_toplevel_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the xdg_toplevel
        ///
        /// This request destroys the role surface and unmaps the surface;
        /// see "Unmapping" behavior in interface section for details.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// set the parent of this surface
        ///
        /// Set the "parent" of this surface. This surface should be stacked
        /// above the parent surface and all other ancestor surfaces.
        /// 
        /// Parent windows should be set on dialogs, toolboxes, or other
        /// "auxiliary" surfaces, so that the parent is raised when the dialog
        /// is raised.
        /// 
        /// Setting a null parent for a child window removes any parent-child
        /// relationship for the child. Setting a null parent for a window which
        /// currently has no parent is a no-op.
        /// 
        /// If the parent is unmapped then its children are managed as
        /// though the parent of the now-unmapped parent has become the
        /// parent of this surface. If no parent exists for the now-unmapped
        /// parent then the children are managed as though they have no
        /// parent surface.
        fn set_parent(&self, parent: Option<&Proxy<super::xdg_toplevel::XdgToplevel>>) ->();
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
        /// by their app ID. As a best practice, it is suggested to select app
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
        /// set the maximum size
        ///
        /// Set a maximum size for the window.
        /// 
        /// The client can specify a maximum size so that the compositor does
        /// not try to configure the window beyond this size.
        /// 
        /// The width and height arguments are in window geometry coordinates.
        /// See xdg_surface.set_window_geometry.
        /// 
        /// Values set in this way are double-buffered. They will get applied
        /// on the next commit.
        /// 
        /// The compositor can use this information to allow or disallow
        /// different states like maximize or fullscreen and draw accurate
        /// animations.
        /// 
        /// Similarly, a tiling window manager may use this information to
        /// place and resize client windows in a more effective way.
        /// 
        /// The client should not rely on the compositor to obey the maximum
        /// size. The compositor may decide to ignore the values set by the
        /// client and request a larger size.
        /// 
        /// If never set, or a value of zero in the request, means that the
        /// client has no expected maximum size in the given dimension.
        /// As a result, a client wishing to reset the maximum size
        /// to an unspecified state can use zero for width and height in the
        /// request.
        /// 
        /// Requesting a maximum size to be smaller than the minimum size of
        /// a surface is illegal and will result in a protocol error.
        /// 
        /// The width and height must be greater than or equal to zero. Using
        /// strictly negative values for width and height will result in a
        /// protocol error.
        fn set_max_size(&self, width: i32, height: i32) ->();
        /// set the minimum size
        ///
        /// Set a minimum size for the window.
        /// 
        /// The client can specify a minimum size so that the compositor does
        /// not try to configure the window below this size.
        /// 
        /// The width and height arguments are in window geometry coordinates.
        /// See xdg_surface.set_window_geometry.
        /// 
        /// Values set in this way are double-buffered. They will get applied
        /// on the next commit.
        /// 
        /// The compositor can use this information to allow or disallow
        /// different states like maximize or fullscreen and draw accurate
        /// animations.
        /// 
        /// Similarly, a tiling window manager may use this information to
        /// place and resize client windows in a more effective way.
        /// 
        /// The client should not rely on the compositor to obey the minimum
        /// size. The compositor may decide to ignore the values set by the
        /// client and request a smaller size.
        /// 
        /// If never set, or a value of zero in the request, means that the
        /// client has no expected minimum size in the given dimension.
        /// As a result, a client wishing to reset the minimum size
        /// to an unspecified state can use zero for width and height in the
        /// request.
        /// 
        /// Requesting a minimum size to be larger than the maximum size of
        /// a surface is illegal and will result in a protocol error.
        /// 
        /// The width and height must be greater than or equal to zero. Using
        /// strictly negative values for width and height will result in a
        /// protocol error.
        fn set_min_size(&self, width: i32, height: i32) ->();
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
        /// 
        /// If the surface is in a fullscreen state, this request has no direct
        /// effect. It will alter the state the surface is returned to when
        /// unmaximized if not overridden by the compositor.
        fn set_maximized(&self) ->();
        /// unmaximize the window
        ///
        /// Unmaximize the surface.
        /// 
        /// After requesting that the surface should be unmaximized, the compositor
        /// will respond by emitting a configure event without the "maximized"
        /// state. If available, the compositor will include the window geometry
        /// dimensions the window had prior to being maximized in the configure
        /// event. The client must then update its content, drawing it in a
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
        /// 
        /// If the surface is in a fullscreen state, this request has no direct
        /// effect. It will alter the state the surface is returned to when
        /// unmaximized if not overridden by the compositor.
        fn unset_maximized(&self) ->();
        /// set the window as fullscreen on an output
        ///
        /// Make the surface fullscreen.
        /// 
        /// After requesting that the surface should be fullscreened, the
        /// compositor will respond by emitting a configure event with the
        /// "fullscreen" state and the fullscreen window geometry. The client must
        /// also acknowledge the configure when committing the new content (see
        /// ack_configure).
        /// 
        /// The output passed by the request indicates the client's preference as
        /// to which display it should be set fullscreen on. If this value is NULL,
        /// it's up to the compositor to choose which display will be used to map
        /// this surface.
        /// 
        /// If the surface doesn't cover the whole output, the compositor will
        /// position the surface in the center of the output and compensate with
        /// with border fill covering the rest of the output. The content of the
        /// border fill is undefined, but should be assumed to be in some way that
        /// attempts to blend into the surrounding area (e.g. solid black).
        /// 
        /// If the fullscreened surface is not opaque, the compositor must make
        /// sure that other screen content not part of the same surface tree (made
        /// up of subsurfaces, popups or similarly coupled surfaces) are not
        /// visible below the fullscreened surface.
        fn set_fullscreen(&self, output: Option<&Proxy<super::wl_output::WlOutput>>) ->();
        /// unset the window as fullscreen
        ///
        /// Make the surface no longer fullscreen.
        /// 
        /// After requesting that the surface should be unfullscreened, the
        /// compositor will respond by emitting a configure event without the
        /// "fullscreen" state.
        /// 
        /// Making a surface unfullscreen sets states for the surface based on the following:
        /// * the state(s) it may have had before becoming fullscreen
        /// * any state(s) decided by the compositor
        /// * any state(s) requested by the client while the surface was fullscreen
        /// 
        /// The compositor may include the previous window geometry dimensions in
        /// the configure event, if applicable.
        /// 
        /// The client must also acknowledge the configure when committing the new
        /// content (see ack_configure).
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

    impl RequestsTrait for Proxy<XdgToplevel> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn set_parent(&self, parent: Option<&Proxy<super::xdg_toplevel::XdgToplevel>>) ->() {
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

        fn set_max_size(&self, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetMaxSize {
                width: width,
                height: height,
            };
            self.send(msg);
        }

        fn set_min_size(&self, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetMinSize {
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
    //! A popup surface is a short-lived, temporary surface. It can be used to
    //! implement for example menus, popovers, tooltips and other similar user
    //! interface concepts.
    //! 
    //! A popup can be made to take an explicit grab. See xdg_popup.grab for
    //! details.
    //! 
    //! When the popup is dismissed, a popup_done event will be sent out, and at
    //! the same time the surface will be unmapped. See the xdg_popup.popup_done
    //! event for details.
    //! 
    //! Explicitly destroying the xdg_popup object will also dismiss the popup and
    //! unmap the surface. Clients that want to dismiss the popup when another
    //! surface of their own is clicked should dismiss the popup using the destroy
    //! request.
    //! 
    //! A newly created xdg_popup will be stacked on top of all previously created
    //! xdg_popup surfaces associated with the same xdg_toplevel.
    //! 
    //! The parent of an xdg_popup must be mapped (see the xdg_surface
    //! description) before the xdg_popup itself.
    //! 
    //! The x and y arguments passed when creating the popup object specify
    //! where the top left of the popup should be placed, relative to the
    //! local surface coordinates of the parent surface. See
    //! xdg_surface.get_popup. An xdg_popup must intersect with or be at least
    //! partially adjacent to its parent surface.
    //! 
    //! The client must call wl_surface.commit on the corresponding wl_surface
    //! for the xdg_popup state to take effect.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// tried to grab after being mapped
        InvalidGrab = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidGrab),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

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
        /// make the popup take an explicit grab
        ///
        /// This request makes the created popup take an explicit grab. An explicit
        /// grab will be dismissed when the user dismisses the popup, or when the
        /// client destroys the xdg_popup. This can be done by the user clicking
        /// outside the surface, using the keyboard, or even locking the screen
        /// through closing the lid or a timeout.
        /// 
        /// If the compositor denies the grab, the popup will be immediately
        /// dismissed.
        /// 
        /// This request must be used in response to some sort of user action like a
        /// button press, key press, or touch down event. The serial number of the
        /// event should be passed as 'serial'.
        /// 
        /// The parent of a grabbing popup must either be an xdg_toplevel surface or
        /// another xdg_popup with an explicit grab. If the parent is another
        /// xdg_popup it means that the popups are nested, with this popup now being
        /// the topmost popup.
        /// 
        /// Nested popups must be destroyed in the reverse order they were created
        /// in, e.g. the only popup you are allowed to destroy at all times is the
        /// topmost one.
        /// 
        /// When compositors choose to dismiss a popup, they may dismiss every
        /// nested grabbing popup as well. When a compositor dismisses popups, it
        /// will follow the same dismissing order as required from the client.
        /// 
        /// The parent of a grabbing popup must either be another xdg_popup with an
        /// active explicit grab, or an xdg_popup or xdg_toplevel, if there are no
        /// explicit grabs already taken.
        /// 
        /// If the topmost grabbing popup is destroyed, the grab will be returned to
        /// the parent of the popup, if that parent previously had an explicit grab.
        /// 
        /// If the parent is a grabbing popup which has already been dismissed, this
        /// popup will be immediately dismissed. If the parent is a popup that did
        /// not take an explicit grab, an error will be raised.
        /// 
        /// During a popup grab, the client owning the grab will receive pointer
        /// and touch events for all their surfaces as normal (similar to an
        /// "owner-events" grab in X11 parlance), while the top most grabbing popup
        /// will always have keyboard focus.
        Grab {seat: Proxy<super::wl_seat::WlSeat>, serial: u32, },
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
                Request::Grab { seat, serial, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    _args_array[1].u = serial;
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// configure the popup surface
        ///
        /// This event asks the popup surface to configure itself given the
        /// configuration. The configured state should not be applied immediately.
        /// See xdg_surface.configure for details.
        /// 
        /// The x and y arguments represent the position the popup was placed at
        /// given the xdg_positioner rule, relative to the upper left corner of the
        /// window geometry of the parent surface.
        Configure {x: i32, y: i32, width: i32, height: i32, },
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
                    let _args = ::std::slice::from_raw_parts(args, 4);
                    Ok(Event::Configure {
                        x: _args[0].i,
                        y: _args[1].i,
                        width: _args[2].i,
                        height: _args[3].i,
                }) },
                1 => {
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
        /// make the popup take an explicit grab
        ///
        /// This request makes the created popup take an explicit grab. An explicit
        /// grab will be dismissed when the user dismisses the popup, or when the
        /// client destroys the xdg_popup. This can be done by the user clicking
        /// outside the surface, using the keyboard, or even locking the screen
        /// through closing the lid or a timeout.
        /// 
        /// If the compositor denies the grab, the popup will be immediately
        /// dismissed.
        /// 
        /// This request must be used in response to some sort of user action like a
        /// button press, key press, or touch down event. The serial number of the
        /// event should be passed as 'serial'.
        /// 
        /// The parent of a grabbing popup must either be an xdg_toplevel surface or
        /// another xdg_popup with an explicit grab. If the parent is another
        /// xdg_popup it means that the popups are nested, with this popup now being
        /// the topmost popup.
        /// 
        /// Nested popups must be destroyed in the reverse order they were created
        /// in, e.g. the only popup you are allowed to destroy at all times is the
        /// topmost one.
        /// 
        /// When compositors choose to dismiss a popup, they may dismiss every
        /// nested grabbing popup as well. When a compositor dismisses popups, it
        /// will follow the same dismissing order as required from the client.
        /// 
        /// The parent of a grabbing popup must either be another xdg_popup with an
        /// active explicit grab, or an xdg_popup or xdg_toplevel, if there are no
        /// explicit grabs already taken.
        /// 
        /// If the topmost grabbing popup is destroyed, the grab will be returned to
        /// the parent of the popup, if that parent previously had an explicit grab.
        /// 
        /// If the parent is a grabbing popup which has already been dismissed, this
        /// popup will be immediately dismissed. If the parent is a popup that did
        /// not take an explicit grab, an error will be raised.
        /// 
        /// During a popup grab, the client owning the grab will receive pointer
        /// and touch events for all their surfaces as normal (similar to an
        /// "owner-events" grab in X11 parlance), while the top most grabbing popup
        /// will always have keyboard focus.
        fn grab(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32) ->();
    }

    impl RequestsTrait for Proxy<XdgPopup> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn grab(&self, seat: &Proxy<super::wl_seat::WlSeat>, serial: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Grab {
                seat: seat.clone(),
                serial: serial,
            };
            self.send(msg);
        }

    }
}

