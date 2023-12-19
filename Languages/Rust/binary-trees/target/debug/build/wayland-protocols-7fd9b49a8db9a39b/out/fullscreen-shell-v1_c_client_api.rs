
//
// This file was auto-generated, do not edit directly.
//

pub mod zwp_fullscreen_shell_v1 {
    //! displays a single surface per output
    //!
    //! Displays a single surface per output.
    //! 
    //! This interface provides a mechanism for a single client to display
    //! simple full-screen surfaces.  While there technically may be multiple
    //! clients bound to this interface, only one of those clients should be
    //! shown at a time.
    //! 
    //! To present a surface, the client uses either the present_surface or
    //! present_surface_for_mode requests.  Presenting a surface takes effect
    //! on the next wl_surface.commit.  See the individual requests for
    //! details about scaling and mode switches.
    //! 
    //! The client can have at most one surface per output at any time.
    //! Requesting a surface to be presented on an output that already has a
    //! surface replaces the previously presented surface.  Presenting a null
    //! surface removes its content and effectively disables the output.
    //! Exactly what happens when an output is "disabled" is
    //! compositor-specific.  The same surface may be presented on multiple
    //! outputs simultaneously.
    //! 
    //! Once a surface is presented on an output, it stays on that output
    //! until either the client removes it or the compositor destroys the
    //! output.  This way, the client can update the output's contents by
    //! simply attaching a new buffer.
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
    /// capabilities advertised by the compositor
    ///
    /// Various capabilities that can be advertised by the compositor.  They
    /// are advertised one-at-a-time when the wl_fullscreen_shell interface is
    /// bound.  See the wl_fullscreen_shell.capability event for more details.
    /// 
    /// ARBITRARY_MODES:
    /// This is a hint to the client that indicates that the compositor is
    /// capable of setting practically any mode on its outputs.  If this
    /// capability is provided, wl_fullscreen_shell.present_surface_for_mode
    /// will almost never fail and clients should feel free to set whatever
    /// mode they like.  If the compositor does not advertise this, it may
    /// still support some modes that are not advertised through wl_global.mode
    /// but it is less likely.
    /// 
    /// CURSOR_PLANE:
    /// This is a hint to the client that indicates that the compositor can
    /// handle a cursor surface from the client without actually compositing.
    /// This may be because of a hardware cursor plane or some other mechanism.
    /// If the compositor does not advertise this capability then setting
    /// wl_pointer.cursor may degrade performance or be ignored entirely.  If
    /// CURSOR_PLANE is not advertised, it is recommended that the client draw
    /// its own cursor and set wl_pointer.cursor(NULL).

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Capability {
        /// compositor is capable of almost any output mode
        ArbitraryModes = 1,
        /// compositor has a separate cursor plane
        CursorPlane = 2,
    }
    impl Capability {
        pub fn from_raw(n: u32) -> Option<Capability> {
            match n {
                1 => Some(Capability::ArbitraryModes),
                2 => Some(Capability::CursorPlane),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// different method to set the surface fullscreen
    ///
    /// Hints to indicate to the compositor how to deal with a conflict
    /// between the dimensions of the surface and the dimensions of the
    /// output. The compositor is free to ignore this parameter.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum PresentMethod {
        /// no preference, apply default policy
        Default = 0,
        /// center the surface on the output
        Center = 1,
        /// scale the surface, preserving aspect ratio, to the largest size that will fit on the output
        Zoom = 2,
        /// scale the surface, preserving aspect ratio, to fully fill the output cropping if needed
        ZoomCrop = 3,
        /// scale the surface to the size of the output ignoring aspect ratio
        Stretch = 4,
    }
    impl PresentMethod {
        pub fn from_raw(n: u32) -> Option<PresentMethod> {
            match n {
                0 => Some(PresentMethod::Default),
                1 => Some(PresentMethod::Center),
                2 => Some(PresentMethod::Zoom),
                3 => Some(PresentMethod::ZoomCrop),
                4 => Some(PresentMethod::Stretch),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// wl_fullscreen_shell error values
    ///
    /// These errors can be emitted in response to wl_fullscreen_shell requests.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// present_method is not known
        InvalidMethod = 0,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::InvalidMethod),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// release the wl_fullscreen_shell interface
        ///
        /// Release the binding from the wl_fullscreen_shell interface.
        /// 
        /// This destroys the server-side object and frees this binding.  If
        /// the client binds to wl_fullscreen_shell multiple times, it may wish
        /// to free some of those bindings.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Release,
        /// present surface for display
        ///
        /// Present a surface on the given output.
        /// 
        /// If the output is null, the compositor will present the surface on
        /// whatever display (or displays) it thinks best.  In particular, this
        /// may replace any or all surfaces currently presented so it should
        /// not be used in combination with placing surfaces on specific
        /// outputs.
        /// 
        /// The method parameter is a hint to the compositor for how the surface
        /// is to be presented.  In particular, it tells the compositor how to
        /// handle a size mismatch between the presented surface and the
        /// output.  The compositor is free to ignore this parameter.
        /// 
        /// The "zoom", "zoom_crop", and "stretch" methods imply a scaling
        /// operation on the surface.  This will override any kind of output
        /// scaling, so the buffer_scale property of the surface is effectively
        /// ignored.
        PresentSurface {surface: Option<Proxy<super::wl_surface::WlSurface>>, method: u32, output: Option<Proxy<super::wl_output::WlOutput>>, },
        /// present surface for display at a particular mode
        ///
        /// Presents a surface on the given output for a particular mode.
        /// 
        /// If the current size of the output differs from that of the surface,
        /// the compositor will attempt to change the size of the output to
        /// match the surface.  The result of the mode-switch operation will be
        /// returned via the provided wl_fullscreen_shell_mode_feedback object.
        /// 
        /// If the current output mode matches the one requested or if the
        /// compositor successfully switches the mode to match the surface,
        /// then the mode_successful event will be sent and the output will
        /// contain the contents of the given surface.  If the compositor
        /// cannot match the output size to the surface size, the mode_failed
        /// will be sent and the output will contain the contents of the
        /// previously presented surface (if any).  If another surface is
        /// presented on the given output before either of these has a chance
        /// to happen, the present_cancelled event will be sent.
        /// 
        /// Due to race conditions and other issues unknown to the client, no
        /// mode-switch operation is guaranteed to succeed.  However, if the
        /// mode is one advertised by wl_output.mode or if the compositor
        /// advertises the ARBITRARY_MODES capability, then the client should
        /// expect that the mode-switch operation will usually succeed.
        /// 
        /// If the size of the presented surface changes, the resulting output
        /// is undefined.  The compositor may attempt to change the output mode
        /// to compensate.  However, there is no guarantee that a suitable mode
        /// will be found and the client has no way to be notified of success
        /// or failure.
        /// 
        /// The framerate parameter specifies the desired framerate for the
        /// output in mHz.  The compositor is free to ignore this parameter.  A
        /// value of 0 indicates that the client has no preference.
        /// 
        /// If the value of wl_output.scale differs from wl_surface.buffer_scale,
        /// then the compositor may choose a mode that matches either the buffer
        /// size or the surface size.  In either case, the surface will fill the
        /// output.
        PresentSurfaceForMode {surface: Proxy<super::wl_surface::WlSurface>, output: Proxy<super::wl_output::WlOutput>, framerate: i32, feedback: Proxy<super::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1>, },
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
                Request::Release => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(0, &mut _args_array)
                },
                Request::PresentSurface { surface, method, output, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = surface.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[1].u = method;
                    _args_array[2].o = output.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    f(1, &mut _args_array)
                },
                Request::PresentSurfaceForMode { surface, output, framerate, feedback, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = surface.c_ptr() as *mut _;
                    _args_array[1].o = output.c_ptr() as *mut _;
                    _args_array[2].i = framerate;
                    _args_array[3].o = feedback.c_ptr() as *mut _;
                    f(2, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// advertises a capability of the compositor
        ///
        /// Advertises a single capability of the compositor.
        /// 
        /// When the wl_fullscreen_shell interface is bound, this event is emitted
        /// once for each capability advertised.  Valid capabilities are given by
        /// the wl_fullscreen_shell.capability enum.  If clients want to take
        /// advantage of any of these capabilities, they should use a
        /// wl_display.sync request immediately after binding to ensure that they
        /// receive all the capability events.
        Capability {capability: u32, },
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
                    Ok(Event::Capability {
                        capability: _args[0].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpFullscreenShellV1;

    impl Interface for ZwpFullscreenShellV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_fullscreen_shell_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_fullscreen_shell_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// release the wl_fullscreen_shell interface
        ///
        /// Release the binding from the wl_fullscreen_shell interface.
        /// 
        /// This destroys the server-side object and frees this binding.  If
        /// the client binds to wl_fullscreen_shell multiple times, it may wish
        /// to free some of those bindings.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn release(&self) ->();
        /// present surface for display
        ///
        /// Present a surface on the given output.
        /// 
        /// If the output is null, the compositor will present the surface on
        /// whatever display (or displays) it thinks best.  In particular, this
        /// may replace any or all surfaces currently presented so it should
        /// not be used in combination with placing surfaces on specific
        /// outputs.
        /// 
        /// The method parameter is a hint to the compositor for how the surface
        /// is to be presented.  In particular, it tells the compositor how to
        /// handle a size mismatch between the presented surface and the
        /// output.  The compositor is free to ignore this parameter.
        /// 
        /// The "zoom", "zoom_crop", and "stretch" methods imply a scaling
        /// operation on the surface.  This will override any kind of output
        /// scaling, so the buffer_scale property of the surface is effectively
        /// ignored.
        fn present_surface(&self, surface: Option<&Proxy<super::wl_surface::WlSurface>>, method: u32, output: Option<&Proxy<super::wl_output::WlOutput>>) ->();
        /// present surface for display at a particular mode
        ///
        /// Presents a surface on the given output for a particular mode.
        /// 
        /// If the current size of the output differs from that of the surface,
        /// the compositor will attempt to change the size of the output to
        /// match the surface.  The result of the mode-switch operation will be
        /// returned via the provided wl_fullscreen_shell_mode_feedback object.
        /// 
        /// If the current output mode matches the one requested or if the
        /// compositor successfully switches the mode to match the surface,
        /// then the mode_successful event will be sent and the output will
        /// contain the contents of the given surface.  If the compositor
        /// cannot match the output size to the surface size, the mode_failed
        /// will be sent and the output will contain the contents of the
        /// previously presented surface (if any).  If another surface is
        /// presented on the given output before either of these has a chance
        /// to happen, the present_cancelled event will be sent.
        /// 
        /// Due to race conditions and other issues unknown to the client, no
        /// mode-switch operation is guaranteed to succeed.  However, if the
        /// mode is one advertised by wl_output.mode or if the compositor
        /// advertises the ARBITRARY_MODES capability, then the client should
        /// expect that the mode-switch operation will usually succeed.
        /// 
        /// If the size of the presented surface changes, the resulting output
        /// is undefined.  The compositor may attempt to change the output mode
        /// to compensate.  However, there is no guarantee that a suitable mode
        /// will be found and the client has no way to be notified of success
        /// or failure.
        /// 
        /// The framerate parameter specifies the desired framerate for the
        /// output in mHz.  The compositor is free to ignore this parameter.  A
        /// value of 0 indicates that the client has no preference.
        /// 
        /// If the value of wl_output.scale differs from wl_surface.buffer_scale,
        /// then the compositor may choose a mode that matches either the buffer
        /// size or the surface size.  In either case, the surface will fill the
        /// output.
        fn present_surface_for_mode(&self, surface: &Proxy<super::wl_surface::WlSurface>, output: &Proxy<super::wl_output::WlOutput>, framerate: i32) ->Result<NewProxy<super::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1>, ()>;
    }

    impl RequestsTrait for Proxy<ZwpFullscreenShellV1> {
        fn release(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Release;
            self.send(msg);
        }

        fn present_surface(&self, surface: Option<&Proxy<super::wl_surface::WlSurface>>, method: u32, output: Option<&Proxy<super::wl_output::WlOutput>>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::PresentSurface {
                surface : surface.map(|o| o.clone()),
                method: method,
                output : output.map(|o| o.clone()),
            };
            self.send(msg);
        }

        fn present_surface_for_mode(&self, surface: &Proxy<super::wl_surface::WlSurface>, output: &Proxy<super::wl_output::WlOutput>, framerate: i32) ->Result<NewProxy<super::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_feedback_newproxy = self.child::<super::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1>();
            let msg = Request::PresentSurfaceForMode {
                surface: surface.clone(),
                output: output.clone(),
                framerate: framerate,
                feedback: unsafe { Proxy::<super::zwp_fullscreen_shell_mode_feedback_v1::ZwpFullscreenShellModeFeedbackV1>::from_c_ptr(_arg_feedback_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_feedback_newproxy)
        }

    }
}

pub mod zwp_fullscreen_shell_mode_feedback_v1 {
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
        /// mode switch succeeded
        ///
        /// This event indicates that the attempted mode switch operation was
        /// successful.  A surface of the size requested in the mode switch
        /// will fill the output without scaling.
        /// 
        /// Upon receiving this event, the client should destroy the
        /// wl_fullscreen_shell_mode_feedback object.
        ModeSuccessful,
        /// mode switch failed
        ///
        /// This event indicates that the attempted mode switch operation
        /// failed.  This may be because the requested output mode is not
        /// possible or it may mean that the compositor does not want to allow it.
        /// 
        /// Upon receiving this event, the client should destroy the
        /// wl_fullscreen_shell_mode_feedback object.
        ModeFailed,
        /// mode switch cancelled
        ///
        /// This event indicates that the attempted mode switch operation was
        /// cancelled.  Most likely this is because the client requested a
        /// second mode switch before the first one completed.
        /// 
        /// Upon receiving this event, the client should destroy the
        /// wl_fullscreen_shell_mode_feedback object.
        PresentCancelled,
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
                    Ok(Event::ModeSuccessful) },
                1 => {
                    Ok(Event::ModeFailed) },
                2 => {
                    Ok(Event::PresentCancelled) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpFullscreenShellModeFeedbackV1;

    impl Interface for ZwpFullscreenShellModeFeedbackV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_fullscreen_shell_mode_feedback_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_fullscreen_shell_mode_feedback_v1_interface }
        }
    }

    pub trait RequestsTrait {
    }

    impl RequestsTrait for Proxy<ZwpFullscreenShellModeFeedbackV1> {
    }
}

