
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright 2014 © Stephen "Lyude" Chandler Paul
    Copyright 2015-2016 © Red Hat, Inc.

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

pub mod zwp_tablet_manager_v1 {
    //! controller object for graphic tablet devices
    //!
    //! An object that provides access to the graphics tablets available on this
    //! system. All tablets are associated with a seat, to get access to the
    //! actual tablets, use wp_tablet_manager.get_tablet_seat.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// get the tablet seat
        ///
        /// Get the wp_tablet_seat object for the given seat. This object
        /// provides access to all graphics tablets in this seat.
        GetTabletSeat {tablet_seat: Proxy<super::zwp_tablet_seat_v1::ZwpTabletSeatV1>, seat: Proxy<super::wl_seat::WlSeat>, },
        /// release the memory for the tablet manager object
        ///
        /// Destroy the wp_tablet_manager object. Objects created from this
        /// object are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
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
                Request::GetTabletSeat { tablet_seat, seat, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = tablet_seat.c_ptr() as *mut _;
                    _args_array[1].o = seat.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                },
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
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


    pub struct ZwpTabletManagerV1;

    impl Interface for ZwpTabletManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_manager_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_manager_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// get the tablet seat
        ///
        /// Get the wp_tablet_seat object for the given seat. This object
        /// provides access to all graphics tablets in this seat.
        fn get_tablet_seat(&self, seat: &Proxy<super::wl_seat::WlSeat>) ->Result<NewProxy<super::zwp_tablet_seat_v1::ZwpTabletSeatV1>, ()>;
        /// release the memory for the tablet manager object
        ///
        /// Destroy the wp_tablet_manager object. Objects created from this
        /// object are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletManagerV1> {
        fn get_tablet_seat(&self, seat: &Proxy<super::wl_seat::WlSeat>) ->Result<NewProxy<super::zwp_tablet_seat_v1::ZwpTabletSeatV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_tablet_seat_newproxy = self.child::<super::zwp_tablet_seat_v1::ZwpTabletSeatV1>();
            let msg = Request::GetTabletSeat {
                tablet_seat: unsafe { Proxy::<super::zwp_tablet_seat_v1::ZwpTabletSeatV1>::from_c_ptr(_arg_tablet_seat_newproxy.c_ptr()) },
                seat: seat.clone(),
            };
            self.send(msg);
            Ok(_arg_tablet_seat_newproxy)
        }

        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

pub mod zwp_tablet_seat_v1 {
    //! controller object for graphic tablet devices of a seat
    //!
    //! An object that provides access to the graphics tablets available on this
    //! seat. After binding to this interface, the compositor sends a set of
    //! wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// release the memory for the tablet seat object
        ///
        /// Destroy the wp_tablet_seat object. Objects created from this
        /// object are unaffected and should be destroyed separately.
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
        /// new device notification
        ///
        /// This event is sent whenever a new tablet becomes available on this
        /// seat. This event only provides the object id of the tablet, any
        /// static information about the tablet (device name, vid/pid, etc.) is
        /// sent through the wp_tablet interface.
        TabletAdded {id: NewProxy<super::zwp_tablet_v1::ZwpTabletV1>, },
        /// a new tool has been used with a tablet
        ///
        /// This event is sent whenever a tool that has not previously been used
        /// with a tablet comes into use. This event only provides the object id
        /// of the tool; any static information about the tool (capabilities,
        /// type, etc.) is sent through the wp_tablet_tool interface.
        ToolAdded {id: NewProxy<super::zwp_tablet_tool_v1::ZwpTabletToolV1>, },
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
                    Ok(Event::TabletAdded {
                        id: NewProxy::<super::zwp_tablet_v1::ZwpTabletV1>::from_c_ptr(_args[0].o as *mut _),
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::ToolAdded {
                        id: NewProxy::<super::zwp_tablet_tool_v1::ZwpTabletToolV1>::from_c_ptr(_args[0].o as *mut _),
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTabletSeatV1;

    impl Interface for ZwpTabletSeatV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_seat_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_seat_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// release the memory for the tablet seat object
        ///
        /// Destroy the wp_tablet_seat object. Objects created from this
        /// object are unaffected and should be destroyed separately.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletSeatV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

pub mod zwp_tablet_tool_v1 {
    //! a physical tablet tool
    //!
    //! An object that represents a physical tool that has been, or is
    //! currently in use with a tablet in this seat. Each wp_tablet_tool
    //! object stays valid until the client destroys it; the compositor
    //! reuses the wp_tablet_tool object to indicate that the object's
    //! respective physical tool has come into proximity of a tablet again.
    //! 
    //! A wp_tablet_tool object's relation to a physical tool depends on the
    //! tablet's ability to report serial numbers. If the tablet supports
    //! this capability, then the object represents a specific physical tool
    //! and can be identified even when used on multiple tablets.
    //! 
    //! A tablet tool has a number of static characteristics, e.g. tool type,
    //! hardware_serial and capabilities. These capabilities are sent in an
    //! event sequence after the wp_tablet_seat.tool_added event before any
    //! actual events from this tool. This initial event sequence is
    //! terminated by a wp_tablet_tool.done event.
    //! 
    //! Tablet tool events are grouped by wp_tablet_tool.frame events.
    //! Any events received before a wp_tablet_tool.frame event should be
    //! considered part of the same hardware state change.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// a physical tool type
    ///
    /// Describes the physical type of a tool. The physical type of a tool
    /// generally defines its base usage.
    /// 
    /// The mouse tool represents a mouse-shaped tool that is not a relative
    /// device but bound to the tablet's surface, providing absolute
    /// coordinates.
    /// 
    /// The lens tool is a mouse-shaped tool with an attached lens to
    /// provide precision focus.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Type {
        /// Pen
        Pen = 0x140,
        /// Eraser
        Eraser = 0x141,
        /// Brush
        Brush = 0x142,
        /// Pencil
        Pencil = 0x143,
        /// Airbrush
        Airbrush = 0x144,
        /// Finger
        Finger = 0x145,
        /// Mouse
        Mouse = 0x146,
        /// Lens
        Lens = 0x147,
    }
    impl Type {
        pub fn from_raw(n: u32) -> Option<Type> {
            match n {
                0x140 => Some(Type::Pen),
                0x141 => Some(Type::Eraser),
                0x142 => Some(Type::Brush),
                0x143 => Some(Type::Pencil),
                0x144 => Some(Type::Airbrush),
                0x145 => Some(Type::Finger),
                0x146 => Some(Type::Mouse),
                0x147 => Some(Type::Lens),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// capability flags for a tool
    ///
    /// Describes extra capabilities on a tablet.
    /// 
    /// Any tool must provide x and y values, extra axes are
    /// device-specific.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Capability {
        /// Tilt axes
        Tilt = 1,
        /// Pressure axis
        Pressure = 2,
        /// Distance axis
        Distance = 3,
        /// Z-rotation axis
        Rotation = 4,
        /// Slider axis
        Slider = 5,
        /// Wheel axis
        Wheel = 6,
    }
    impl Capability {
        pub fn from_raw(n: u32) -> Option<Capability> {
            match n {
                1 => Some(Capability::Tilt),
                2 => Some(Capability::Pressure),
                3 => Some(Capability::Distance),
                4 => Some(Capability::Rotation),
                5 => Some(Capability::Slider),
                6 => Some(Capability::Wheel),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// physical button state
    ///
    /// Describes the physical state of a button that produced the button event.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ButtonState {
        /// button is not pressed
        Released = 0,
        /// button is pressed
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
        /// set the tablet tool's surface
        ///
        /// Sets the surface of the cursor used for this tool on the given
        /// tablet. This request only takes effect if the tool is in proximity
        /// of one of the requesting client's surfaces or the surface parameter
        /// is the current pointer surface. If there was a previous surface set
        /// with this request it is replaced. If surface is NULL, the cursor
        /// image is hidden.
        /// 
        /// The parameters hotspot_x and hotspot_y define the position of the
        /// pointer surface relative to the pointer location. Its top-left corner
        /// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
        /// coordinates of the pointer location, in surface-local coordinates.
        /// 
        /// On surface.attach requests to the pointer surface, hotspot_x and
        /// hotspot_y are decremented by the x and y parameters passed to the
        /// request. Attach must be confirmed by wl_surface.commit as usual.
        /// 
        /// The hotspot can also be updated by passing the currently set pointer
        /// surface to this request with new values for hotspot_x and hotspot_y.
        /// 
        /// The current and pending input regions of the wl_surface are cleared,
        /// and wl_surface.set_input_region is ignored until the wl_surface is no
        /// longer used as the cursor. When the use as a cursor ends, the current
        /// and pending input regions become undefined, and the wl_surface is
        /// unmapped.
        /// 
        /// This request gives the surface the role of a cursor. The role
        /// assigned by this request is the same as assigned by
        /// wl_pointer.set_cursor meaning the same surface can be
        /// used both as a wl_pointer cursor and a wp_tablet cursor. If the
        /// surface already has another role, it raises a protocol error.
        /// The surface may be used on multiple tablets and across multiple
        /// seats.
        SetCursor {serial: u32, surface: Option<Proxy<super::wl_surface::WlSurface>>, hotspot_x: i32, hotspot_y: i32, },
        /// destroy the tool object
        ///
        /// This destroys the client's resource for this tool object.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
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
                Request::SetCursor { serial, surface, hotspot_x, hotspot_y, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    _args_array[1].o = surface.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[2].i = hotspot_x;
                    _args_array[3].i = hotspot_y;
                    f(0, &mut _args_array)
                },
                Request::Destroy => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// tool type
        ///
        /// The tool type is the high-level type of the tool and usually decides
        /// the interaction expected from this tool.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        Type {tool_type: Type, },
        /// unique hardware serial number of the tool
        ///
        /// If the physical tool can be identified by a unique 64-bit serial
        /// number, this event notifies the client of this serial number.
        /// 
        /// If multiple tablets are available in the same seat and the tool is
        /// uniquely identifiable by the serial number, that tool may move
        /// between tablets.
        /// 
        /// Otherwise, if the tool has no serial number and this event is
        /// missing, the tool is tied to the tablet it first comes into
        /// proximity with. Even if the physical tool is used on multiple
        /// tablets, separate wp_tablet_tool objects will be created, one per
        /// tablet.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        HardwareSerial {hardware_serial_hi: u32, hardware_serial_lo: u32, },
        /// hardware id notification in Wacom's format
        ///
        /// This event notifies the client of a hardware id available on this tool.
        /// 
        /// The hardware id is a device-specific 64-bit id that provides extra
        /// information about the tool in use, beyond the wl_tool.type
        /// enumeration. The format of the id is specific to tablets made by
        /// Wacom Inc. For example, the hardware id of a Wacom Grip
        /// Pen (a stylus) is 0x802.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        HardwareIdWacom {hardware_id_hi: u32, hardware_id_lo: u32, },
        /// tool capability notification
        ///
        /// This event notifies the client of any capabilities of this tool,
        /// beyond the main set of x/y axes and tip up/down detection.
        /// 
        /// One event is sent for each extra capability available on this tool.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet_tool.done event.
        Capability {capability: Capability, },
        /// tool description events sequence complete
        ///
        /// This event signals the end of the initial burst of descriptive
        /// events. A client may consider the static description of the tool to
        /// be complete and finalize initialization of the tool.
        Done,
        /// tool removed
        ///
        /// This event is sent when the tool is removed from the system and will
        /// send no further events. Should the physical tool come back into
        /// proximity later, a new wp_tablet_tool object will be created.
        /// 
        /// It is compositor-dependent when a tool is removed. A compositor may
        /// remove a tool on proximity out, tablet removal or any other reason.
        /// A compositor may also keep a tool alive until shutdown.
        /// 
        /// If the tool is currently in proximity, a proximity_out event will be
        /// sent before the removed event. See wp_tablet_tool.proximity_out for
        /// the handling of any buttons logically down.
        /// 
        /// When this event is received, the client must wp_tablet_tool.destroy
        /// the object.
        Removed,
        /// proximity in event
        ///
        /// Notification that this tool is focused on a certain surface.
        /// 
        /// This event can be received when the tool has moved from one surface to
        /// another, or when the tool has come back into proximity above the
        /// surface.
        /// 
        /// If any button is logically down when the tool comes into proximity,
        /// the respective button event is sent after the proximity_in event but
        /// within the same frame as the proximity_in event.
        ProximityIn {serial: u32, tablet: Proxy<super::zwp_tablet_v1::ZwpTabletV1>, surface: Proxy<super::wl_surface::WlSurface>, },
        /// proximity out event
        ///
        /// Notification that this tool has either left proximity, or is no
        /// longer focused on a certain surface.
        /// 
        /// When the tablet tool leaves proximity of the tablet, button release
        /// events are sent for each button that was held down at the time of
        /// leaving proximity. These events are sent before the proximity_out
        /// event but within the same wp_tablet.frame.
        /// 
        /// If the tool stays within proximity of the tablet, but the focus
        /// changes from one surface to another, a button release event may not
        /// be sent until the button is actually released or the tool leaves the
        /// proximity of the tablet.
        ProximityOut,
        /// tablet tool is making contact
        ///
        /// Sent whenever the tablet tool comes in contact with the surface of the
        /// tablet.
        /// 
        /// If the tool is already in contact with the tablet when entering the
        /// input region, the client owning said region will receive a
        /// wp_tablet.proximity_in event, followed by a wp_tablet.down
        /// event and a wp_tablet.frame event.
        /// 
        /// Note that this event describes logical contact, not physical
        /// contact. On some devices, a compositor may not consider a tool in
        /// logical contact until a minimum physical pressure threshold is
        /// exceeded.
        Down {serial: u32, },
        /// tablet tool is no longer making contact
        ///
        /// Sent whenever the tablet tool stops making contact with the surface of
        /// the tablet, or when the tablet tool moves out of the input region
        /// and the compositor grab (if any) is dismissed.
        /// 
        /// If the tablet tool moves out of the input region while in contact
        /// with the surface of the tablet and the compositor does not have an
        /// ongoing grab on the surface, the client owning said region will
        /// receive a wp_tablet.up event, followed by a wp_tablet.proximity_out
        /// event and a wp_tablet.frame event. If the compositor has an ongoing
        /// grab on this device, this event sequence is sent whenever the grab
        /// is dismissed in the future.
        /// 
        /// Note that this event describes logical contact, not physical
        /// contact. On some devices, a compositor may not consider a tool out
        /// of logical contact until physical pressure falls below a specific
        /// threshold.
        Up,
        /// motion event
        ///
        /// Sent whenever a tablet tool moves.
        Motion {x: f64, y: f64, },
        /// pressure change event
        ///
        /// Sent whenever the pressure axis on a tool changes. The value of this
        /// event is normalized to a value between 0 and 65535.
        /// 
        /// Note that pressure may be nonzero even when a tool is not in logical
        /// contact. See the down and up events for more details.
        Pressure {pressure: u32, },
        /// distance change event
        ///
        /// Sent whenever the distance axis on a tool changes. The value of this
        /// event is normalized to a value between 0 and 65535.
        /// 
        /// Note that distance may be nonzero even when a tool is not in logical
        /// contact. See the down and up events for more details.
        Distance {distance: u32, },
        /// tilt change event
        ///
        /// Sent whenever one or both of the tilt axes on a tool change. Each tilt
        /// value is in 0.01 of a degree, relative to the z-axis of the tablet.
        /// The angle is positive when the top of a tool tilts along the
        /// positive x or y axis.
        Tilt {tilt_x: i32, tilt_y: i32, },
        /// z-rotation change event
        ///
        /// Sent whenever the z-rotation axis on the tool changes. The
        /// rotation value is in 0.01 of a degree clockwise from the tool's
        /// logical neutral position.
        Rotation {degrees: i32, },
        /// Slider position change event
        ///
        /// Sent whenever the slider position on the tool changes. The
        /// value is normalized between -65535 and 65535, with 0 as the logical
        /// neutral position of the slider.
        /// 
        /// The slider is available on e.g. the Wacom Airbrush tool.
        Slider {position: i32, },
        /// Wheel delta event
        ///
        /// Sent whenever the wheel on the tool emits an event. This event
        /// contains two values for the same axis change. The degrees value is
        /// in 0.01 of a degree in the same orientation as the
        /// wl_pointer.vertical_scroll axis. The clicks value is in discrete
        /// logical clicks of the mouse wheel. This value may be zero if the
        /// movement of the wheel was less than one logical click.
        /// 
        /// Clients should choose either value and avoid mixing degrees and
        /// clicks. The compositor may accumulate values smaller than a logical
        /// click and emulate click events when a certain threshold is met.
        /// Thus, wl_tablet_tool.wheel events with non-zero clicks values may
        /// have different degrees values.
        Wheel {degrees: i32, clicks: i32, },
        /// button event
        ///
        /// Sent whenever a button on the tool is pressed or released.
        /// 
        /// If a button is held down when the tool moves in or out of proximity,
        /// button events are generated by the compositor. See
        /// wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for
        /// details.
        Button {serial: u32, button: u32, state: ButtonState, },
        /// frame event
        ///
        /// Marks the end of a series of axis and/or button updates from the
        /// tablet. The Wayland protocol requires axis updates to be sent
        /// sequentially, however all events within a frame should be considered
        /// one hardware event.
        Frame {time: u32, },
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
                    Ok(Event::Type {
                        tool_type: Type::from_raw(_args[0].u).ok_or(())?,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::HardwareSerial {
                        hardware_serial_hi: _args[0].u,
                        hardware_serial_lo: _args[1].u,
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::HardwareIdWacom {
                        hardware_id_hi: _args[0].u,
                        hardware_id_lo: _args[1].u,
                }) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Capability {
                        capability: Capability::from_raw(_args[0].u).ok_or(())?,
                }) },
                4 => {
                    Ok(Event::Done) },
                5 => {
                    Ok(Event::Removed) },
                6 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::ProximityIn {
                        serial: _args[0].u,
                        tablet: Proxy::<super::zwp_tablet_v1::ZwpTabletV1>::from_c_ptr(_args[1].o as *mut _),
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[2].o as *mut _),
                }) },
                7 => {
                    Ok(Event::ProximityOut) },
                8 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Down {
                        serial: _args[0].u,
                }) },
                9 => {
                    Ok(Event::Up) },
                10 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Motion {
                        x: (_args[0].f as f64)/256.,
                        y: (_args[1].f as f64)/256.,
                }) },
                11 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Pressure {
                        pressure: _args[0].u,
                }) },
                12 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Distance {
                        distance: _args[0].u,
                }) },
                13 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Tilt {
                        tilt_x: _args[0].i,
                        tilt_y: _args[1].i,
                }) },
                14 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Rotation {
                        degrees: _args[0].i,
                }) },
                15 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Slider {
                        position: _args[0].i,
                }) },
                16 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Wheel {
                        degrees: _args[0].i,
                        clicks: _args[1].i,
                }) },
                17 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Button {
                        serial: _args[0].u,
                        button: _args[1].u,
                        state: ButtonState::from_raw(_args[2].u).ok_or(())?,
                }) },
                18 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Frame {
                        time: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTabletToolV1;

    impl Interface for ZwpTabletToolV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_tool_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_tool_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// set the tablet tool's surface
        ///
        /// Sets the surface of the cursor used for this tool on the given
        /// tablet. This request only takes effect if the tool is in proximity
        /// of one of the requesting client's surfaces or the surface parameter
        /// is the current pointer surface. If there was a previous surface set
        /// with this request it is replaced. If surface is NULL, the cursor
        /// image is hidden.
        /// 
        /// The parameters hotspot_x and hotspot_y define the position of the
        /// pointer surface relative to the pointer location. Its top-left corner
        /// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
        /// coordinates of the pointer location, in surface-local coordinates.
        /// 
        /// On surface.attach requests to the pointer surface, hotspot_x and
        /// hotspot_y are decremented by the x and y parameters passed to the
        /// request. Attach must be confirmed by wl_surface.commit as usual.
        /// 
        /// The hotspot can also be updated by passing the currently set pointer
        /// surface to this request with new values for hotspot_x and hotspot_y.
        /// 
        /// The current and pending input regions of the wl_surface are cleared,
        /// and wl_surface.set_input_region is ignored until the wl_surface is no
        /// longer used as the cursor. When the use as a cursor ends, the current
        /// and pending input regions become undefined, and the wl_surface is
        /// unmapped.
        /// 
        /// This request gives the surface the role of a cursor. The role
        /// assigned by this request is the same as assigned by
        /// wl_pointer.set_cursor meaning the same surface can be
        /// used both as a wl_pointer cursor and a wp_tablet cursor. If the
        /// surface already has another role, it raises a protocol error.
        /// The surface may be used on multiple tablets and across multiple
        /// seats.
        fn set_cursor(&self, serial: u32, surface: Option<&Proxy<super::wl_surface::WlSurface>>, hotspot_x: i32, hotspot_y: i32) ->();
        /// destroy the tool object
        ///
        /// This destroys the client's resource for this tool object.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletToolV1> {
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

        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

pub mod zwp_tablet_v1 {
    //! graphics tablet device
    //!
    //! The wp_tablet interface represents one graphics tablet device. The
    //! tablet interface itself does not generate events; all events are
    //! generated by wp_tablet_tool objects when in proximity above a tablet.
    //! 
    //! A tablet has a number of static characteristics, e.g. device name and
    //! pid/vid. These capabilities are sent in an event sequence after the
    //! wp_tablet_seat.tablet_added event. This initial event sequence is
    //! terminated by a wp_tablet.done event.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the tablet object
        ///
        /// This destroys the client's resource for this tablet object.
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
        /// tablet device name
        ///
        /// This event is sent in the initial burst of events before the
        /// wp_tablet.done event.
        Name {name: String, },
        /// tablet device USB vendor/product id
        ///
        /// This event is sent in the initial burst of events before the
        /// wp_tablet.done event.
        Id {vid: u32, pid: u32, },
        /// path to the device
        ///
        /// A system-specific device path that indicates which device is behind
        /// this wp_tablet. This information may be used to gather additional
        /// information about the device, e.g. through libwacom.
        /// 
        /// A device may have more than one device path. If so, multiple
        /// wp_tablet.path events are sent. A device may be emulated and not
        /// have a device path, and in that case this event will not be sent.
        /// 
        /// The format of the path is unspecified, it may be a device node, a
        /// sysfs path, or some other identifier. It is up to the client to
        /// identify the string provided.
        /// 
        /// This event is sent in the initial burst of events before the
        /// wp_tablet.done event.
        Path {path: String, },
        /// tablet description events sequence complete
        ///
        /// This event is sent immediately to signal the end of the initial
        /// burst of descriptive events. A client may consider the static
        /// description of the tablet to be complete and finalize initialization
        /// of the tablet.
        Done,
        /// tablet removed event
        ///
        /// Sent when the tablet has been removed from the system. When a tablet
        /// is removed, some tools may be removed.
        /// 
        /// When this event is received, the client must wp_tablet.destroy
        /// the object.
        Removed,
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
                    Ok(Event::Name {
                        name: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Id {
                        vid: _args[0].u,
                        pid: _args[1].u,
                }) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::Path {
                        path: ::std::ffi::CStr::from_ptr(_args[0].s).to_string_lossy().into_owned(),
                }) },
                3 => {
                    Ok(Event::Done) },
                4 => {
                    Ok(Event::Removed) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTabletV1;

    impl Interface for ZwpTabletV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_tablet_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_tablet_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the tablet object
        ///
        /// This destroys the client's resource for this tablet object.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
    }

    impl RequestsTrait for Proxy<ZwpTabletV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

    }
}

