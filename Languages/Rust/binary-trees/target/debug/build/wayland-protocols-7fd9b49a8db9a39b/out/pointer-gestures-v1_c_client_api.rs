
//
// This file was auto-generated, do not edit directly.
//

pub mod zwp_pointer_gestures_v1 {
    //! touchpad gestures
    //!
    //! A global interface to provide semantic touchpad gestures for a given
    //! pointer.
    //! 
    //! Two gestures are currently supported: swipe and zoom/rotate.
    //! All gestures follow a three-stage cycle: begin, update, end and
    //! are identified by a unique id.
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
        /// get swipe gesture
        ///
        /// Create a swipe gesture object. See the
        /// wl_pointer_gesture_swipe interface for details.
        GetSwipeGesture {id: Proxy<super::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1>, pointer: Proxy<super::wl_pointer::WlPointer>, },
        /// get pinch gesture
        ///
        /// Create a pinch gesture object. See the
        /// wl_pointer_gesture_pinch interface for details.
        GetPinchGesture {id: Proxy<super::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1>, pointer: Proxy<super::wl_pointer::WlPointer>, },
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
                Request::GetSwipeGesture { id, pointer, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = pointer.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                },
                Request::GetPinchGesture { id, pointer, } => {
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


    pub struct ZwpPointerGesturesV1;

    impl Interface for ZwpPointerGesturesV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_pointer_gestures_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_pointer_gestures_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// get swipe gesture
        ///
        /// Create a swipe gesture object. See the
        /// wl_pointer_gesture_swipe interface for details.
        fn get_swipe_gesture(&self, pointer: &Proxy<super::wl_pointer::WlPointer>) ->Result<NewProxy<super::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1>, ()>;
        /// get pinch gesture
        ///
        /// Create a pinch gesture object. See the
        /// wl_pointer_gesture_pinch interface for details.
        fn get_pinch_gesture(&self, pointer: &Proxy<super::wl_pointer::WlPointer>) ->Result<NewProxy<super::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1>, ()>;
    }

    impl RequestsTrait for Proxy<ZwpPointerGesturesV1> {
        fn get_swipe_gesture(&self, pointer: &Proxy<super::wl_pointer::WlPointer>) ->Result<NewProxy<super::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1>();
            let msg = Request::GetSwipeGesture {
                id: unsafe { Proxy::<super::zwp_pointer_gesture_swipe_v1::ZwpPointerGestureSwipeV1>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                pointer: pointer.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn get_pinch_gesture(&self, pointer: &Proxy<super::wl_pointer::WlPointer>) ->Result<NewProxy<super::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1>();
            let msg = Request::GetPinchGesture {
                id: unsafe { Proxy::<super::zwp_pointer_gesture_pinch_v1::ZwpPointerGesturePinchV1>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                pointer: pointer.clone(),
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod zwp_pointer_gesture_swipe_v1 {
    //! a swipe gesture object
    //!
    //! A swipe gesture object notifies a client about a multi-finger swipe
    //! gesture detected on an indirect input device such as a touchpad.
    //! The gesture is usually initiated by multiple fingers moving in the
    //! same direction but once initiated the direction may change.
    //! The precise conditions of when such a gesture is detected are
    //! implementation-dependent.
    //! 
    //! A gesture consists of three stages: begin, update (optional) and end.
    //! There cannot be multiple simultaneous pinch or swipe gestures on a
    //! same pointer/seat, how compositors prevent these situations is
    //! implementation-dependent.
    //! 
    //! A gesture may be cancelled by the compositor or the hardware.
    //! Clients should not consider performing permanent or irreversible
    //! actions until the end of a gesture has been received.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the pointer swipe gesture object
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
        /// multi-finger swipe begin
        ///
        /// This event is sent when a multi-finger swipe gesture is detected
        /// on the device.
        Begin {serial: u32, time: u32, surface: Proxy<super::wl_surface::WlSurface>, fingers: u32, },
        /// multi-finger swipe motion
        ///
        /// This event is sent when a multi-finger swipe gesture changes the
        /// position of the logical center.
        /// 
        /// The dx and dy coordinates are relative coordinates of the logical
        /// center of the gesture compared to the previous event.
        Update {time: u32, dx: f64, dy: f64, },
        /// multi-finger swipe end
        ///
        /// This event is sent when a multi-finger swipe gesture ceases to
        /// be valid. This may happen when one or more fingers are lifted or
        /// the gesture is cancelled.
        /// 
        /// When a gesture is cancelled, the client should undo state changes
        /// caused by this gesture. What causes a gesture to be cancelled is
        /// implementation-dependent.
        End {serial: u32, time: u32, cancelled: i32, },
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
                    Ok(Event::Begin {
                        serial: _args[0].u,
                        time: _args[1].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[2].o as *mut _),
                        fingers: _args[3].u,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Update {
                        time: _args[0].u,
                        dx: (_args[1].f as f64)/256.,
                        dy: (_args[2].f as f64)/256.,
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::End {
                        serial: _args[0].u,
                        time: _args[1].u,
                        cancelled: _args[2].i,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpPointerGestureSwipeV1;

    impl Interface for ZwpPointerGestureSwipeV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_pointer_gesture_swipe_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_pointer_gesture_swipe_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the pointer swipe gesture object
        ///
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpPointerGestureSwipeV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

pub mod zwp_pointer_gesture_pinch_v1 {
    //! a pinch gesture object
    //!
    //! A pinch gesture object notifies a client about a multi-finger pinch
    //! gesture detected on an indirect input device such as a touchpad.
    //! The gesture is usually initiated by multiple fingers moving towards
    //! each other or away from each other, or by two or more fingers rotating
    //! around a logical center of gravity. The precise conditions of when
    //! such a gesture is detected are implementation-dependent.
    //! 
    //! A gesture consists of three stages: begin, update (optional) and end.
    //! There cannot be multiple simultaneous pinch or swipe gestures on a
    //! same pointer/seat, how compositors prevent these situations is
    //! implementation-dependent.
    //! 
    //! A gesture may be cancelled by the compositor or the hardware.
    //! Clients should not consider performing permanent or irreversible
    //! actions until the end of a gesture has been received.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the pinch gesture object
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
        /// multi-finger pinch begin
        ///
        /// This event is sent when a multi-finger pinch gesture is detected
        /// on the device.
        Begin {serial: u32, time: u32, surface: Proxy<super::wl_surface::WlSurface>, fingers: u32, },
        /// multi-finger pinch motion
        ///
        /// This event is sent when a multi-finger pinch gesture changes the
        /// position of the logical center, the rotation or the relative scale.
        /// 
        /// The dx and dy coordinates are relative coordinates in the
        /// surface coordinate space of the logical center of the gesture.
        /// 
        /// The scale factor is an absolute scale compared to the
        /// pointer_gesture_pinch.begin event, e.g. a scale of 2 means the fingers
        /// are now twice as far apart as on pointer_gesture_pinch.begin.
        /// 
        /// The rotation is the relative angle in degrees clockwise compared to the previous
        /// pointer_gesture_pinch.begin or pointer_gesture_pinch.update event.
        Update {time: u32, dx: f64, dy: f64, scale: f64, rotation: f64, },
        /// multi-finger pinch end
        ///
        /// This event is sent when a multi-finger pinch gesture ceases to
        /// be valid. This may happen when one or more fingers are lifted or
        /// the gesture is cancelled.
        /// 
        /// When a gesture is cancelled, the client should undo state changes
        /// caused by this gesture. What causes a gesture to be cancelled is
        /// implementation-dependent.
        End {serial: u32, time: u32, cancelled: i32, },
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
                    Ok(Event::Begin {
                        serial: _args[0].u,
                        time: _args[1].u,
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[2].o as *mut _),
                        fingers: _args[3].u,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 5);
                    Ok(Event::Update {
                        time: _args[0].u,
                        dx: (_args[1].f as f64)/256.,
                        dy: (_args[2].f as f64)/256.,
                        scale: (_args[3].f as f64)/256.,
                        rotation: (_args[4].f as f64)/256.,
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::End {
                        serial: _args[0].u,
                        time: _args[1].u,
                        cancelled: _args[2].i,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpPointerGesturePinchV1;

    impl Interface for ZwpPointerGesturePinchV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_pointer_gesture_pinch_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_pointer_gesture_pinch_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the pinch gesture object
        ///
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpPointerGesturePinchV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

