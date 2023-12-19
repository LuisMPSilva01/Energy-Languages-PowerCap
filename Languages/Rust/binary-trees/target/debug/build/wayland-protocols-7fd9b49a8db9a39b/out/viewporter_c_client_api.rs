
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2013-2016 Collabora, Ltd.

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

pub mod wp_viewporter {
    //! surface cropping and scaling
    //!
    //! The global interface exposing surface cropping and scaling
    //! capabilities is used to instantiate an interface extension for a
    //! wl_surface object. This extended interface will then allow
    //! cropping and scaling the surface contents, effectively
    //! disconnecting the direct relationship between the buffer and the
    //! surface size.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// the surface already has a viewport object associated
        ViewportExists = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::ViewportExists),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// unbind from the cropping and scaling interface
        ///
        /// Informs the server that the client will not be using this
        /// protocol object anymore. This does not affect any other objects,
        /// wp_viewport objects included.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// extend surface interface for crop and scale
        ///
        /// Instantiate an interface extension for the given wl_surface to
        /// crop and scale its content. If the given wl_surface already has
        /// a wp_viewport object associated, the viewport_exists
        /// protocol error is raised.
        GetViewport {id: Proxy<super::wp_viewport::WpViewport>, surface: Proxy<super::wl_surface::WlSurface>, },
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
                Request::GetViewport { id, surface, } => {
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


    pub struct WpViewporter;

    impl Interface for WpViewporter {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wp_viewporter";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wp_viewporter_interface }
        }
    }

    pub trait RequestsTrait {
        /// unbind from the cropping and scaling interface
        ///
        /// Informs the server that the client will not be using this
        /// protocol object anymore. This does not affect any other objects,
        /// wp_viewport objects included.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// extend surface interface for crop and scale
        ///
        /// Instantiate an interface extension for the given wl_surface to
        /// crop and scale its content. If the given wl_surface already has
        /// a wp_viewport object associated, the viewport_exists
        /// protocol error is raised.
        fn get_viewport(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::wp_viewport::WpViewport>, ()>;
    }

    impl RequestsTrait for Proxy<WpViewporter> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn get_viewport(&self, surface: &Proxy<super::wl_surface::WlSurface>) ->Result<NewProxy<super::wp_viewport::WpViewport>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::wp_viewport::WpViewport>();
            let msg = Request::GetViewport {
                id: unsafe { Proxy::<super::wp_viewport::WpViewport>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                surface: surface.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod wp_viewport {
    //! crop and scale interface to a wl_surface
    //!
    //! An additional interface to a wl_surface object, which allows the
    //! client to specify the cropping and scaling of the surface
    //! contents.
    //! 
    //! This interface works with two concepts: the source rectangle (src_x,
    //! src_y, src_width, src_height), and the destination size (dst_width,
    //! dst_height). The contents of the source rectangle are scaled to the
    //! destination size, and content outside the source rectangle is ignored.
    //! This state is double-buffered, and is applied on the next
    //! wl_surface.commit.
    //! 
    //! The two parts of crop and scale state are independent: the source
    //! rectangle, and the destination size. Initially both are unset, that
    //! is, no scaling is applied. The whole of the current wl_buffer is
    //! used as the source, and the surface size is as defined in
    //! wl_surface.attach.
    //! 
    //! If the destination size is set, it causes the surface size to become
    //! dst_width, dst_height. The source (rectangle) is scaled to exactly
    //! this size. This overrides whatever the attached wl_buffer size is,
    //! unless the wl_buffer is NULL. If the wl_buffer is NULL, the surface
    //! has no content and therefore no size. Otherwise, the size is always
    //! at least 1x1 in surface local coordinates.
    //! 
    //! If the source rectangle is set, it defines what area of the wl_buffer is
    //! taken as the source. If the source rectangle is set and the destination
    //! size is not set, then src_width and src_height must be integers, and the
    //! surface size becomes the source rectangle size. This results in cropping
    //! without scaling. If src_width or src_height are not integers and
    //! destination size is not set, the bad_size protocol error is raised when
    //! the surface state is applied.
    //! 
    //! The coordinate transformations from buffer pixel coordinates up to
    //! the surface-local coordinates happen in the following order:
    //! 1. buffer_transform (wl_surface.set_buffer_transform)
    //! 2. buffer_scale (wl_surface.set_buffer_scale)
    //! 3. crop and scale (wp_viewport.set*)
    //! This means, that the source rectangle coordinates of crop and scale
    //! are given in the coordinates after the buffer transform and scale,
    //! i.e. in the coordinates that would be the surface-local coordinates
    //! if the crop and scale was not applied.
    //! 
    //! If src_x or src_y are negative, the bad_value protocol error is raised.
    //! Otherwise, if the source rectangle is partially or completely outside of
    //! the non-NULL wl_buffer, then the out_of_buffer protocol error is raised
    //! when the surface state is applied. A NULL wl_buffer does not raise the
    //! out_of_buffer error.
    //! 
    //! The x, y arguments of wl_surface.attach are applied as normal to
    //! the surface. They indicate how many pixels to remove from the
    //! surface size from the left and the top. In other words, they are
    //! still in the surface-local coordinate system, just like dst_width
    //! and dst_height are.
    //! 
    //! If the wl_surface associated with the wp_viewport is destroyed,
    //! all wp_viewport requests except 'destroy' raise the protocol error
    //! no_surface.
    //! 
    //! If the wp_viewport object is destroyed, the crop and scale
    //! state is removed from the wl_surface. The change will be applied
    //! on the next wl_surface.commit.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// negative or zero values in width or height
        BadValue = 0,
        /// destination size is not integer
        BadSize = 1,
        /// source rectangle extends outside of the content area
        OutOfBuffer = 2,
        /// the wl_surface was destroyed
        NoSurface = 3,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::BadValue),
                1 => Some(Error::BadSize),
                2 => Some(Error::OutOfBuffer),
                3 => Some(Error::NoSurface),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// remove scaling and cropping from the surface
        ///
        /// The associated wl_surface's crop and scale state is removed.
        /// The change is applied on the next wl_surface.commit.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// set the source rectangle for cropping
        ///
        /// Set the source rectangle of the associated wl_surface. See
        /// wp_viewport for the description, and relation to the wl_buffer
        /// size.
        /// 
        /// If all of x, y, width and height are -1.0, the source rectangle is
        /// unset instead. Any other set of values where width or height are zero
        /// or negative, or x or y are negative, raise the bad_value protocol
        /// error.
        /// 
        /// The crop and scale state is double-buffered state, and will be
        /// applied on the next wl_surface.commit.
        SetSource {x: f64, y: f64, width: f64, height: f64, },
        /// set the surface size for scaling
        ///
        /// Set the destination size of the associated wl_surface. See
        /// wp_viewport for the description, and relation to the wl_buffer
        /// size.
        /// 
        /// If width is -1 and height is -1, the destination size is unset
        /// instead. Any other pair of values for width and height that
        /// contains zero or negative values raises the bad_value protocol
        /// error.
        /// 
        /// The crop and scale state is double-buffered state, and will be
        /// applied on the next wl_surface.commit.
        SetDestination {width: i32, height: i32, },
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
                Request::SetSource { x, y, width, height, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].f = (x * 256.) as i32;
                    _args_array[1].f = (y * 256.) as i32;
                    _args_array[2].f = (width * 256.) as i32;
                    _args_array[3].f = (height * 256.) as i32;
                    f(1, &mut _args_array)
                },
                Request::SetDestination { width, height, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = width;
                    _args_array[1].i = height;
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


    pub struct WpViewport;

    impl Interface for WpViewport {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "wp_viewport";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::wp_viewport_interface }
        }
    }

    pub trait RequestsTrait {
        /// remove scaling and cropping from the surface
        ///
        /// The associated wl_surface's crop and scale state is removed.
        /// The change is applied on the next wl_surface.commit.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// set the source rectangle for cropping
        ///
        /// Set the source rectangle of the associated wl_surface. See
        /// wp_viewport for the description, and relation to the wl_buffer
        /// size.
        /// 
        /// If all of x, y, width and height are -1.0, the source rectangle is
        /// unset instead. Any other set of values where width or height are zero
        /// or negative, or x or y are negative, raise the bad_value protocol
        /// error.
        /// 
        /// The crop and scale state is double-buffered state, and will be
        /// applied on the next wl_surface.commit.
        fn set_source(&self, x: f64, y: f64, width: f64, height: f64) ->();
        /// set the surface size for scaling
        ///
        /// Set the destination size of the associated wl_surface. See
        /// wp_viewport for the description, and relation to the wl_buffer
        /// size.
        /// 
        /// If width is -1 and height is -1, the destination size is unset
        /// instead. Any other pair of values for width and height that
        /// contains zero or negative values raises the bad_value protocol
        /// error.
        /// 
        /// The crop and scale state is double-buffered state, and will be
        /// applied on the next wl_surface.commit.
        fn set_destination(&self, width: i32, height: i32) ->();
    }

    impl RequestsTrait for Proxy<WpViewport> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn set_source(&self, x: f64, y: f64, width: f64, height: f64) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetSource {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.send(msg);
        }

        fn set_destination(&self, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetDestination {
                width: width,
                height: height,
            };
            self.send(msg);
        }

    }
}

