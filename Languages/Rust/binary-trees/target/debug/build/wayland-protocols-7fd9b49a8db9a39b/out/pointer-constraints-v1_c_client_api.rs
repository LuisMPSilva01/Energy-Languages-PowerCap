
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

pub mod zwp_pointer_constraints_v1 {
    //! constrain the movement of a pointer
    //!
    //! The global interface exposing pointer constraining functionality. It
    //! exposes two requests: lock_pointer for locking the pointer to its
    //! position, and confine_pointer for locking the pointer to a region.
    //! 
    //! The lock_pointer and confine_pointer requests create the objects
    //! wp_locked_pointer and wp_confined_pointer respectively, and the client can
    //! use these objects to interact with the lock.
    //! 
    //! For any surface, only one lock or confinement may be active across all
    //! wl_pointer objects of the same seat. If a lock or confinement is requested
    //! when another lock or confinement is active or requested on the same surface
    //! and with any of the wl_pointer objects of the same seat, an
    //! 'already_constrained' error will be raised.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    /// wp_pointer_constraints error values
    ///
    /// These errors can be emitted in response to wp_pointer_constraints
    /// requests.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// pointer constraint already requested on that surface
        AlreadyConstrained = 1,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                1 => Some(Error::AlreadyConstrained),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// constraint lifetime
    ///
    /// These values represent different lifetime semantics. They are passed
    /// as arguments to the factory requests to specify how the constraint
    /// lifetimes should be managed.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Lifetime {
        /// the pointer constraint is defunct once deactivated
        ///
        /// A oneshot pointer constraint will never reactivate once it has been
        /// deactivated. See the corresponding deactivation event
        /// (wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for
        /// details.
        Oneshot = 1,
        /// the pointer constraint may reactivate
        ///
        /// A persistent pointer constraint may again reactivate once it has
        /// been deactivated. See the corresponding deactivation event
        /// (wp_locked_pointer.unlocked and wp_confined_pointer.unconfined) for
        /// details.
        Persistent = 2,
    }
    impl Lifetime {
        pub fn from_raw(n: u32) -> Option<Lifetime> {
            match n {
                1 => Some(Lifetime::Oneshot),
                2 => Some(Lifetime::Persistent),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// destroy the pointer constraints manager object
        ///
        /// Used by the client to notify the server that it will no longer use this
        /// pointer constraints object.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// lock pointer to a position
        ///
        /// The lock_pointer request lets the client request to disable movements of
        /// the virtual pointer (i.e. the cursor), effectively locking the pointer
        /// to a position. This request may not take effect immediately; in the
        /// future, when the compositor deems implementation-specific constraints
        /// are satisfied, the pointer lock will be activated and the compositor
        /// sends a locked event.
        /// 
        /// The protocol provides no guarantee that the constraints are ever
        /// satisfied, and does not require the compositor to send an error if the
        /// constraints cannot ever be satisfied. It is thus possible to request a
        /// lock that will never activate.
        /// 
        /// There may not be another pointer constraint of any kind requested or
        /// active on the surface for any of the wl_pointer objects of the seat of
        /// the passed pointer when requesting a lock. If there is, an error will be
        /// raised. See general pointer lock documentation for more details.
        /// 
        /// The intersection of the region passed with this request and the input
        /// region of the surface is used to determine where the pointer must be
        /// in order for the lock to activate. It is up to the compositor whether to
        /// warp the pointer or require some kind of user interaction for the lock
        /// to activate. If the region is null the surface input region is used.
        /// 
        /// A surface may receive pointer focus without the lock being activated.
        /// 
        /// The request creates a new object wp_locked_pointer which is used to
        /// interact with the lock as well as receive updates about its state. See
        /// the the description of wp_locked_pointer for further information.
        /// 
        /// Note that while a pointer is locked, the wl_pointer objects of the
        /// corresponding seat will not emit any wl_pointer.motion events, but
        /// relative motion events will still be emitted via wp_relative_pointer
        /// objects of the same seat. wl_pointer.axis and wl_pointer.button events
        /// are unaffected.
        LockPointer {id: Proxy<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>, surface: Proxy<super::wl_surface::WlSurface>, pointer: Proxy<super::wl_pointer::WlPointer>, region: Option<Proxy<super::wl_region::WlRegion>>, lifetime: u32, },
        /// confine pointer to a region
        ///
        /// The confine_pointer request lets the client request to confine the
        /// pointer cursor to a given region. This request may not take effect
        /// immediately; in the future, when the compositor deems implementation-
        /// specific constraints are satisfied, the pointer confinement will be
        /// activated and the compositor sends a confined event.
        /// 
        /// The intersection of the region passed with this request and the input
        /// region of the surface is used to determine where the pointer must be
        /// in order for the confinement to activate. It is up to the compositor
        /// whether to warp the pointer or require some kind of user interaction for
        /// the confinement to activate. If the region is null the surface input
        /// region is used.
        /// 
        /// The request will create a new object wp_confined_pointer which is used
        /// to interact with the confinement as well as receive updates about its
        /// state. See the the description of wp_confined_pointer for further
        /// information.
        ConfinePointer {id: Proxy<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>, surface: Proxy<super::wl_surface::WlSurface>, pointer: Proxy<super::wl_pointer::WlPointer>, region: Option<Proxy<super::wl_region::WlRegion>>, lifetime: u32, },
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
                Request::LockPointer { id, surface, pointer, region, lifetime, } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    _args_array[2].o = pointer.c_ptr() as *mut _;
                    _args_array[3].o = region.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[4].u = lifetime;
                    f(1, &mut _args_array)
                },
                Request::ConfinePointer { id, surface, pointer, region, lifetime, } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    _args_array[2].o = pointer.c_ptr() as *mut _;
                    _args_array[3].o = region.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    _args_array[4].u = lifetime;
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


    pub struct ZwpPointerConstraintsV1;

    impl Interface for ZwpPointerConstraintsV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_pointer_constraints_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_pointer_constraints_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the pointer constraints manager object
        ///
        /// Used by the client to notify the server that it will no longer use this
        /// pointer constraints object.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// lock pointer to a position
        ///
        /// The lock_pointer request lets the client request to disable movements of
        /// the virtual pointer (i.e. the cursor), effectively locking the pointer
        /// to a position. This request may not take effect immediately; in the
        /// future, when the compositor deems implementation-specific constraints
        /// are satisfied, the pointer lock will be activated and the compositor
        /// sends a locked event.
        /// 
        /// The protocol provides no guarantee that the constraints are ever
        /// satisfied, and does not require the compositor to send an error if the
        /// constraints cannot ever be satisfied. It is thus possible to request a
        /// lock that will never activate.
        /// 
        /// There may not be another pointer constraint of any kind requested or
        /// active on the surface for any of the wl_pointer objects of the seat of
        /// the passed pointer when requesting a lock. If there is, an error will be
        /// raised. See general pointer lock documentation for more details.
        /// 
        /// The intersection of the region passed with this request and the input
        /// region of the surface is used to determine where the pointer must be
        /// in order for the lock to activate. It is up to the compositor whether to
        /// warp the pointer or require some kind of user interaction for the lock
        /// to activate. If the region is null the surface input region is used.
        /// 
        /// A surface may receive pointer focus without the lock being activated.
        /// 
        /// The request creates a new object wp_locked_pointer which is used to
        /// interact with the lock as well as receive updates about its state. See
        /// the the description of wp_locked_pointer for further information.
        /// 
        /// Note that while a pointer is locked, the wl_pointer objects of the
        /// corresponding seat will not emit any wl_pointer.motion events, but
        /// relative motion events will still be emitted via wp_relative_pointer
        /// objects of the same seat. wl_pointer.axis and wl_pointer.button events
        /// are unaffected.
        fn lock_pointer(&self, surface: &Proxy<super::wl_surface::WlSurface>, pointer: &Proxy<super::wl_pointer::WlPointer>, region: Option<&Proxy<super::wl_region::WlRegion>>, lifetime: u32) ->Result<NewProxy<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>, ()>;
        /// confine pointer to a region
        ///
        /// The confine_pointer request lets the client request to confine the
        /// pointer cursor to a given region. This request may not take effect
        /// immediately; in the future, when the compositor deems implementation-
        /// specific constraints are satisfied, the pointer confinement will be
        /// activated and the compositor sends a confined event.
        /// 
        /// The intersection of the region passed with this request and the input
        /// region of the surface is used to determine where the pointer must be
        /// in order for the confinement to activate. It is up to the compositor
        /// whether to warp the pointer or require some kind of user interaction for
        /// the confinement to activate. If the region is null the surface input
        /// region is used.
        /// 
        /// The request will create a new object wp_confined_pointer which is used
        /// to interact with the confinement as well as receive updates about its
        /// state. See the the description of wp_confined_pointer for further
        /// information.
        fn confine_pointer(&self, surface: &Proxy<super::wl_surface::WlSurface>, pointer: &Proxy<super::wl_pointer::WlPointer>, region: Option<&Proxy<super::wl_region::WlRegion>>, lifetime: u32) ->Result<NewProxy<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>, ()>;
    }

    impl RequestsTrait for Proxy<ZwpPointerConstraintsV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn lock_pointer(&self, surface: &Proxy<super::wl_surface::WlSurface>, pointer: &Proxy<super::wl_pointer::WlPointer>, region: Option<&Proxy<super::wl_region::WlRegion>>, lifetime: u32) ->Result<NewProxy<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>();
            let msg = Request::LockPointer {
                id: unsafe { Proxy::<super::zwp_locked_pointer_v1::ZwpLockedPointerV1>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                surface: surface.clone(),
                pointer: pointer.clone(),
                region : region.map(|o| o.clone()),
                lifetime: lifetime,
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

        fn confine_pointer(&self, surface: &Proxy<super::wl_surface::WlSurface>, pointer: &Proxy<super::wl_pointer::WlPointer>, region: Option<&Proxy<super::wl_region::WlRegion>>, lifetime: u32) ->Result<NewProxy<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>();
            let msg = Request::ConfinePointer {
                id: unsafe { Proxy::<super::zwp_confined_pointer_v1::ZwpConfinedPointerV1>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
                surface: surface.clone(),
                pointer: pointer.clone(),
                region : region.map(|o| o.clone()),
                lifetime: lifetime,
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

pub mod zwp_locked_pointer_v1 {
    //! receive relative pointer motion events
    //!
    //! The wp_locked_pointer interface represents a locked pointer state.
    //! 
    //! While the lock of this object is active, the wl_pointer objects of the
    //! associated seat will not emit any wl_pointer.motion events.
    //! 
    //! This object will send the event 'locked' when the lock is activated.
    //! Whenever the lock is activated, it is guaranteed that the locked surface
    //! will already have received pointer focus and that the pointer will be
    //! within the region passed to the request creating this object.
    //! 
    //! To unlock the pointer, send the destroy request. This will also destroy
    //! the wp_locked_pointer object.
    //! 
    //! If the compositor decides to unlock the pointer the unlocked event is
    //! sent. See wp_locked_pointer.unlock for details.
    //! 
    //! When unlocking, the compositor may warp the cursor position to the set
    //! cursor position hint. If it does, it will not result in any relative
    //! motion events emitted via wp_relative_pointer.
    //! 
    //! If the surface the lock was requested on is destroyed and the lock is not
    //! yet activated, the wp_locked_pointer object is now defunct and must be
    //! destroyed.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the locked pointer object
        ///
        /// Destroy the locked pointer object. If applicable, the compositor will
        /// unlock the pointer.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// set the pointer cursor position hint
        ///
        /// Set the cursor position hint relative to the top left corner of the
        /// surface.
        /// 
        /// If the client is drawing its own cursor, it should update the position
        /// hint to the position of its own cursor. A compositor may use this
        /// information to warp the pointer upon unlock in order to avoid pointer
        /// jumps.
        /// 
        /// The cursor position hint is double buffered. The new hint will only take
        /// effect when the associated surface gets it pending state applied. See
        /// wl_surface.commit for details.
        SetCursorPositionHint {surface_x: f64, surface_y: f64, },
        /// set a new lock region
        ///
        /// Set a new region used to lock the pointer.
        /// 
        /// The new lock region is double-buffered. The new lock region will
        /// only take effect when the associated surface gets its pending state
        /// applied. See wl_surface.commit for details.
        /// 
        /// For details about the lock region, see wp_locked_pointer.
        SetRegion {region: Option<Proxy<super::wl_region::WlRegion>>, },
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
                Request::SetCursorPositionHint { surface_x, surface_y, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].f = (surface_x * 256.) as i32;
                    _args_array[1].f = (surface_y * 256.) as i32;
                    f(1, &mut _args_array)
                },
                Request::SetRegion { region, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = region.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    f(2, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// lock activation event
        ///
        /// Notification that the pointer lock of the seat's pointer is activated.
        Locked,
        /// lock deactivation event
        ///
        /// Notification that the pointer lock of the seat's pointer is no longer
        /// active. If this is a oneshot pointer lock (see
        /// wp_pointer_constraints.lifetime) this object is now defunct and should
        /// be destroyed. If this is a persistent pointer lock (see
        /// wp_pointer_constraints.lifetime) this pointer lock may again
        /// reactivate in the future.
        Unlocked,
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
                    Ok(Event::Locked) },
                1 => {
                    Ok(Event::Unlocked) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpLockedPointerV1;

    impl Interface for ZwpLockedPointerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_locked_pointer_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_locked_pointer_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the locked pointer object
        ///
        /// Destroy the locked pointer object. If applicable, the compositor will
        /// unlock the pointer.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// set the pointer cursor position hint
        ///
        /// Set the cursor position hint relative to the top left corner of the
        /// surface.
        /// 
        /// If the client is drawing its own cursor, it should update the position
        /// hint to the position of its own cursor. A compositor may use this
        /// information to warp the pointer upon unlock in order to avoid pointer
        /// jumps.
        /// 
        /// The cursor position hint is double buffered. The new hint will only take
        /// effect when the associated surface gets it pending state applied. See
        /// wl_surface.commit for details.
        fn set_cursor_position_hint(&self, surface_x: f64, surface_y: f64) ->();
        /// set a new lock region
        ///
        /// Set a new region used to lock the pointer.
        /// 
        /// The new lock region is double-buffered. The new lock region will
        /// only take effect when the associated surface gets its pending state
        /// applied. See wl_surface.commit for details.
        /// 
        /// For details about the lock region, see wp_locked_pointer.
        fn set_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) ->();
    }

    impl RequestsTrait for Proxy<ZwpLockedPointerV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn set_cursor_position_hint(&self, surface_x: f64, surface_y: f64) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetCursorPositionHint {
                surface_x: surface_x,
                surface_y: surface_y,
            };
            self.send(msg);
        }

        fn set_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetRegion {
                region : region.map(|o| o.clone()),
            };
            self.send(msg);
        }

    }
}

pub mod zwp_confined_pointer_v1 {
    //! confined pointer object
    //!
    //! The wp_confined_pointer interface represents a confined pointer state.
    //! 
    //! This object will send the event 'confined' when the confinement is
    //! activated. Whenever the confinement is activated, it is guaranteed that
    //! the surface the pointer is confined to will already have received pointer
    //! focus and that the pointer will be within the region passed to the request
    //! creating this object. It is up to the compositor to decide whether this
    //! requires some user interaction and if the pointer will warp to within the
    //! passed region if outside.
    //! 
    //! To unconfine the pointer, send the destroy request. This will also destroy
    //! the wp_confined_pointer object.
    //! 
    //! If the compositor decides to unconfine the pointer the unconfined event is
    //! sent. The wp_confined_pointer object is at this point defunct and should
    //! be destroyed.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// destroy the confined pointer object
        ///
        /// Destroy the confined pointer object. If applicable, the compositor will
        /// unconfine the pointer.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// set a new confine region
        ///
        /// Set a new region used to confine the pointer.
        /// 
        /// The new confine region is double-buffered. The new confine region will
        /// only take effect when the associated surface gets its pending state
        /// applied. See wl_surface.commit for details.
        /// 
        /// If the confinement is active when the new confinement region is applied
        /// and the pointer ends up outside of newly applied region, the pointer may
        /// warped to a position within the new confinement region. If warped, a
        /// wl_pointer.motion event will be emitted, but no
        /// wp_relative_pointer.relative_motion event.
        /// 
        /// The compositor may also, instead of using the new region, unconfine the
        /// pointer.
        /// 
        /// For details about the confine region, see wp_confined_pointer.
        SetRegion {region: Option<Proxy<super::wl_region::WlRegion>>, },
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
                Request::SetRegion { region, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = region.map(|o| o.c_ptr() as *mut _).unwrap_or(::std::ptr::null_mut());
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// pointer confined
        ///
        /// Notification that the pointer confinement of the seat's pointer is
        /// activated.
        Confined,
        /// pointer unconfined
        ///
        /// Notification that the pointer confinement of the seat's pointer is no
        /// longer active. If this is a oneshot pointer confinement (see
        /// wp_pointer_constraints.lifetime) this object is now defunct and should
        /// be destroyed. If this is a persistent pointer confinement (see
        /// wp_pointer_constraints.lifetime) this pointer confinement may again
        /// reactivate in the future.
        Unconfined,
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
                    Ok(Event::Confined) },
                1 => {
                    Ok(Event::Unconfined) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpConfinedPointerV1;

    impl Interface for ZwpConfinedPointerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_confined_pointer_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_confined_pointer_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// destroy the confined pointer object
        ///
        /// Destroy the confined pointer object. If applicable, the compositor will
        /// unconfine the pointer.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// set a new confine region
        ///
        /// Set a new region used to confine the pointer.
        /// 
        /// The new confine region is double-buffered. The new confine region will
        /// only take effect when the associated surface gets its pending state
        /// applied. See wl_surface.commit for details.
        /// 
        /// If the confinement is active when the new confinement region is applied
        /// and the pointer ends up outside of newly applied region, the pointer may
        /// warped to a position within the new confinement region. If warped, a
        /// wl_pointer.motion event will be emitted, but no
        /// wp_relative_pointer.relative_motion event.
        /// 
        /// The compositor may also, instead of using the new region, unconfine the
        /// pointer.
        /// 
        /// For details about the confine region, see wp_confined_pointer.
        fn set_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) ->();
    }

    impl RequestsTrait for Proxy<ZwpConfinedPointerV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn set_region(&self, region: Option<&Proxy<super::wl_region::WlRegion>>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetRegion {
                region : region.map(|o| o.clone()),
            };
            self.send(msg);
        }

    }
}

