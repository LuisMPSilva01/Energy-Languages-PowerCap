
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2014, 2015 Collabora, Ltd.

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

pub mod zwp_linux_dmabuf_v1 {
    //! factory for creating dmabuf-based wl_buffers
    //!
    //! Following the interfaces from:
    //! https://www.khronos.org/registry/egl/extensions/EXT/EGL_EXT_image_dma_buf_import.txt
    //! and the Linux DRM sub-system's AddFb2 ioctl.
    //! 
    //! This interface offers ways to create generic dmabuf-based
    //! wl_buffers. Immediately after a client binds to this interface,
    //! the set of supported formats and format modifiers is sent with
    //! 'format' and 'modifier' events.
    //! 
    //! The following are required from clients:
    //! 
    //! - Clients must ensure that either all data in the dma-buf is
    //! coherent for all subsequent read access or that coherency is
    //! correctly handled by the underlying kernel-side dma-buf
    //! implementation.
    //! 
    //! - Don't make any more attachments after sending the buffer to the
    //! compositor. Making more attachments later increases the risk of
    //! the compositor not being able to use (re-import) an existing
    //! dmabuf-based wl_buffer.
    //! 
    //! The underlying graphics stack must ensure the following:
    //! 
    //! - The dmabuf file descriptors relayed to the server will stay valid
    //! for the whole lifetime of the wl_buffer. This means the server may
    //! at any time use those fds to import the dmabuf into any kernel
    //! sub-system that might accept it.
    //! 
    //! To create a wl_buffer from one or more dmabufs, a client creates a
    //! zwp_linux_dmabuf_params_v1 object with a zwp_linux_dmabuf_v1.create_params
    //! request. All planes required by the intended format are added with
    //! the 'add' request. Finally, a 'create' or 'create_immed' request is
    //! issued, which has the following outcome depending on the import success.
    //! 
    //! The 'create' request,
    //! - on success, triggers a 'created' event which provides the final
    //! wl_buffer to the client.
    //! - on failure, triggers a 'failed' event to convey that the server
    //! cannot use the dmabufs received from the client.
    //! 
    //! For the 'create_immed' request,
    //! - on success, the server immediately imports the added dmabufs to
    //! create a wl_buffer. No event is sent from the server in this case.
    //! - on failure, the server can choose to either:
    //! - terminate the client by raising a fatal error.
    //! - mark the wl_buffer as failed, and send a 'failed' event to the
    //! client. If the client uses a failed wl_buffer as an argument to any
    //! request, the behaviour is compositor implementation-defined.
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
        /// unbind the factory
        ///
        /// Objects created through this interface, especially wl_buffers, will
        /// remain valid.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// create a temporary object for buffer parameters
        ///
        /// This temporary object is used to collect multiple dmabuf handles into
        /// a single batch to create a wl_buffer. It can only be used once and
        /// should be destroyed after a 'created' or 'failed' event has been
        /// received.
        CreateParams {params_id: Proxy<super::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1>, },
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
                Request::CreateParams { params_id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = params_id.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// supported buffer format
        ///
        /// This event advertises one buffer format that the server supports.
        /// All the supported formats are advertised once when the client
        /// binds to this interface. A roundtrip after binding guarantees
        /// that the client has received all supported formats.
        /// 
        /// For the definition of the format codes, see the
        /// zwp_linux_buffer_params_v1::create request.
        /// 
        /// Warning: the 'format' event is likely to be deprecated and replaced
        /// with the 'modifier' event introduced in zwp_linux_dmabuf_v1
        /// version 3, described below. Please refrain from using the information
        /// received from this event.
        Format {format: u32, },
        /// supported buffer format modifier
        ///
        /// This event advertises the formats that the server supports, along with
        /// the modifiers supported for each format. All the supported modifiers
        /// for all the supported formats are advertised once when the client
        /// binds to this interface. A roundtrip after binding guarantees that
        /// the client has received all supported format-modifier pairs.
        /// 
        /// For the definition of the format and modifier codes, see the
        /// zwp_linux_buffer_params_v1::create request.
        ///
        /// Only available since version 3 of the interface
        Modifier {format: u32, modifier_hi: u32, modifier_lo: u32, },
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
                    Ok(Event::Format {
                        format: _args[0].u,
                }) },
                1 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::Modifier {
                        format: _args[0].u,
                        modifier_hi: _args[1].u,
                        modifier_lo: _args[2].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpLinuxDmabufV1;

    impl Interface for ZwpLinuxDmabufV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_linux_dmabuf_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_linux_dmabuf_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// unbind the factory
        ///
        /// Objects created through this interface, especially wl_buffers, will
        /// remain valid.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// create a temporary object for buffer parameters
        ///
        /// This temporary object is used to collect multiple dmabuf handles into
        /// a single batch to create a wl_buffer. It can only be used once and
        /// should be destroyed after a 'created' or 'failed' event has been
        /// received.
        fn create_params(&self) ->Result<NewProxy<super::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1>, ()>;
    }

    impl RequestsTrait for Proxy<ZwpLinuxDmabufV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn create_params(&self) ->Result<NewProxy<super::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_params_id_newproxy = self.child::<super::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1>();
            let msg = Request::CreateParams {
                params_id: unsafe { Proxy::<super::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1>::from_c_ptr(_arg_params_id_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_params_id_newproxy)
        }

    }
}

pub mod zwp_linux_buffer_params_v1 {
    //! parameters for creating a dmabuf-based wl_buffer
    //!
    //! This temporary object is a collection of dmabufs and other
    //! parameters that together form a single logical buffer. The temporary
    //! object may eventually create one wl_buffer unless cancelled by
    //! destroying it before requesting 'create'.
    //! 
    //! Single-planar formats only require one dmabuf, however
    //! multi-planar formats may require more than one dmabuf. For all
    //! formats, an 'add' request must be called once per plane (even if the
    //! underlying dmabuf fd is identical).
    //! 
    //! You must use consecutive plane indices ('plane_idx' argument for 'add')
    //! from zero to the number of planes used by the drm_fourcc format code.
    //! All planes required by the format must be given exactly once, but can
    //! be given in any order. Each plane index can be set only once.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Error {
        /// the dmabuf_batch object has already been used to create a wl_buffer
        AlreadyUsed = 0,
        /// plane index out of bounds
        PlaneIdx = 1,
        /// the plane index was already set
        PlaneSet = 2,
        /// missing or too many planes to create a buffer
        Incomplete = 3,
        /// format not supported
        InvalidFormat = 4,
        /// invalid width or height
        InvalidDimensions = 5,
        /// offset + stride * height goes out of dmabuf bounds
        OutOfBounds = 6,
        /// invalid wl_buffer resulted from importing dmabufs via the create_immed request on given buffer_params
        InvalidWlBuffer = 7,
    }
    impl Error {
        pub fn from_raw(n: u32) -> Option<Error> {
            match n {
                0 => Some(Error::AlreadyUsed),
                1 => Some(Error::PlaneIdx),
                2 => Some(Error::PlaneSet),
                3 => Some(Error::Incomplete),
                4 => Some(Error::InvalidFormat),
                5 => Some(Error::InvalidDimensions),
                6 => Some(Error::OutOfBounds),
                7 => Some(Error::InvalidWlBuffer),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }


    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum Flags {
        /// contents are y-inverted
        YInvert = 1,
        /// content is interlaced
        Interlaced = 2,
        /// bottom field first
        BottomFirst = 4,
    }
    impl Flags {
        pub fn from_raw(n: u32) -> Option<Flags> {
            match n {
                1 => Some(Flags::YInvert),
                2 => Some(Flags::Interlaced),
                4 => Some(Flags::BottomFirst),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// delete this object, used or not
        ///
        /// Cleans up the temporary data sent to the server for dmabuf-based
        /// wl_buffer creation.
        ///
        /// This is a destructor, once sent this object cannot be used any longer.
        Destroy,
        /// add a dmabuf to the temporary set
        ///
        /// This request adds one dmabuf to the set in this
        /// zwp_linux_buffer_params_v1.
        /// 
        /// The 64-bit unsigned value combined from modifier_hi and modifier_lo
        /// is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the
        /// fb modifier, which is defined in drm_mode.h of Linux UAPI.
        /// This is an opaque token. Drivers use this token to express tiling,
        /// compression, etc. driver-specific modifications to the base format
        /// defined by the DRM fourcc code.
        /// 
        /// This request raises the PLANE_IDX error if plane_idx is too large.
        /// The error PLANE_SET is raised if attempting to set a plane that
        /// was already set.
        Add {fd: ::std::os::unix::io::RawFd, plane_idx: u32, offset: u32, stride: u32, modifier_hi: u32, modifier_lo: u32, },
        /// create a wl_buffer from the given dmabufs
        ///
        /// This asks for creation of a wl_buffer from the added dmabuf
        /// buffers. The wl_buffer is not created immediately but returned via
        /// the 'created' event if the dmabuf sharing succeeds. The sharing
        /// may fail at runtime for reasons a client cannot predict, in
        /// which case the 'failed' event is triggered.
        /// 
        /// The 'format' argument is a DRM_FORMAT code, as defined by the
        /// libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the
        /// authoritative source on how the format codes should work.
        /// 
        /// The 'flags' is a bitfield of the flags defined in enum "flags".
        /// 'y_invert' means the that the image needs to be y-flipped.
        /// 
        /// Flag 'interlaced' means that the frame in the buffer is not
        /// progressive as usual, but interlaced. An interlaced buffer as
        /// supported here must always contain both top and bottom fields.
        /// The top field always begins on the first pixel row. The temporal
        /// ordering between the two fields is top field first, unless
        /// 'bottom_first' is specified. It is undefined whether 'bottom_first'
        /// is ignored if 'interlaced' is not set.
        /// 
        /// This protocol does not convey any information about field rate,
        /// duration, or timing, other than the relative ordering between the
        /// two fields in one buffer. A compositor may have to estimate the
        /// intended field rate from the incoming buffer rate. It is undefined
        /// whether the time of receiving wl_surface.commit with a new buffer
        /// attached, applying the wl_surface state, wl_surface.frame callback
        /// trigger, presentation, or any other point in the compositor cycle
        /// is used to measure the frame or field times. There is no support
        /// for detecting missed or late frames/fields/buffers either, and
        /// there is no support whatsoever for cooperating with interlaced
        /// compositor output.
        /// 
        /// The composited image quality resulting from the use of interlaced
        /// buffers is explicitly undefined. A compositor may use elaborate
        /// hardware features or software to deinterlace and create progressive
        /// output frames from a sequence of interlaced input buffers, or it
        /// may produce substandard image quality. However, compositors that
        /// cannot guarantee reasonable image quality in all cases are recommended
        /// to just reject all interlaced buffers.
        /// 
        /// Any argument errors, including non-positive width or height,
        /// mismatch between the number of planes and the format, bad
        /// format, bad offset or stride, may be indicated by fatal protocol
        /// errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,
        /// OUT_OF_BOUNDS.
        /// 
        /// Dmabuf import errors in the server that are not obvious client
        /// bugs are returned via the 'failed' event as non-fatal. This
        /// allows attempting dmabuf sharing and falling back in the client
        /// if it fails.
        /// 
        /// This request can be sent only once in the object's lifetime, after
        /// which the only legal request is destroy. This object should be
        /// destroyed after issuing a 'create' request. Attempting to use this
        /// object after issuing 'create' raises ALREADY_USED protocol error.
        /// 
        /// It is not mandatory to issue 'create'. If a client wants to
        /// cancel the buffer creation, it can just destroy this object.
        Create {width: i32, height: i32, format: u32, flags: u32, },
        /// immediately create a wl_buffer from the given dmabufs
        ///
        /// This asks for immediate creation of a wl_buffer by importing the
        /// added dmabufs.
        /// 
        /// In case of import success, no event is sent from the server, and the
        /// wl_buffer is ready to be used by the client.
        /// 
        /// Upon import failure, either of the following may happen, as seen fit
        /// by the implementation:
        /// - the client is terminated with one of the following fatal protocol
        /// errors:
        /// - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
        /// in case of argument errors such as mismatch between the number
        /// of planes and the format, bad format, non-positive width or
        /// height, or bad offset or stride.
        /// - INVALID_WL_BUFFER, in case the cause for failure is unknown or
        /// plaform specific.
        /// - the server creates an invalid wl_buffer, marks it as failed and
        /// sends a 'failed' event to the client. The result of using this
        /// invalid wl_buffer as an argument in any request by the client is
        /// defined by the compositor implementation.
        /// 
        /// This takes the same arguments as a 'create' request, and obeys the
        /// same restrictions.
        ///
        /// Only available since version 2 of the interface
        CreateImmed {buffer_id: Proxy<super::wl_buffer::WlBuffer>, width: i32, height: i32, format: u32, flags: u32, },
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
                Request::Add { fd, plane_idx, offset, stride, modifier_hi, modifier_lo, } => {
                    let mut _args_array: [wl_argument; 6] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].h = fd;
                    _args_array[1].u = plane_idx;
                    _args_array[2].u = offset;
                    _args_array[3].u = stride;
                    _args_array[4].u = modifier_hi;
                    _args_array[5].u = modifier_lo;
                    f(1, &mut _args_array)
                },
                Request::Create { width, height, format, flags, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = width;
                    _args_array[1].i = height;
                    _args_array[2].u = format;
                    _args_array[3].u = flags;
                    f(2, &mut _args_array)
                },
                Request::CreateImmed { buffer_id, width, height, format, flags, } => {
                    let mut _args_array: [wl_argument; 5] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = buffer_id.c_ptr() as *mut _;
                    _args_array[1].i = width;
                    _args_array[2].i = height;
                    _args_array[3].u = format;
                    _args_array[4].u = flags;
                    f(3, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// buffer creation succeeded
        ///
        /// This event indicates that the attempted buffer creation was
        /// successful. It provides the new wl_buffer referencing the dmabuf(s).
        /// 
        /// Upon receiving this event, the client should destroy the
        /// zlinux_dmabuf_params object.
        Created {buffer: NewProxy<super::wl_buffer::WlBuffer>, },
        /// buffer creation failed
        ///
        /// This event indicates that the attempted buffer creation has
        /// failed. It usually means that one of the dmabuf constraints
        /// has not been fulfilled.
        /// 
        /// Upon receiving this event, the client should destroy the
        /// zlinux_buffer_params object.
        Failed,
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
                    Ok(Event::Created {
                        buffer: NewProxy::<super::wl_buffer::WlBuffer>::from_c_ptr(_args[0].o as *mut _),
                }) },
                1 => {
                    Ok(Event::Failed) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpLinuxBufferParamsV1;

    impl Interface for ZwpLinuxBufferParamsV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_linux_buffer_params_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_linux_buffer_params_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// delete this object, used or not
        ///
        /// Cleans up the temporary data sent to the server for dmabuf-based
        /// wl_buffer creation.
        ///
        /// This is a destructor, you cannot send requests to this object any longer once this method is called.
        fn destroy(&self) ->();
        /// add a dmabuf to the temporary set
        ///
        /// This request adds one dmabuf to the set in this
        /// zwp_linux_buffer_params_v1.
        /// 
        /// The 64-bit unsigned value combined from modifier_hi and modifier_lo
        /// is the dmabuf layout modifier. DRM AddFB2 ioctl calls this the
        /// fb modifier, which is defined in drm_mode.h of Linux UAPI.
        /// This is an opaque token. Drivers use this token to express tiling,
        /// compression, etc. driver-specific modifications to the base format
        /// defined by the DRM fourcc code.
        /// 
        /// This request raises the PLANE_IDX error if plane_idx is too large.
        /// The error PLANE_SET is raised if attempting to set a plane that
        /// was already set.
        fn add(&self, fd: ::std::os::unix::io::RawFd, plane_idx: u32, offset: u32, stride: u32, modifier_hi: u32, modifier_lo: u32) ->();
        /// create a wl_buffer from the given dmabufs
        ///
        /// This asks for creation of a wl_buffer from the added dmabuf
        /// buffers. The wl_buffer is not created immediately but returned via
        /// the 'created' event if the dmabuf sharing succeeds. The sharing
        /// may fail at runtime for reasons a client cannot predict, in
        /// which case the 'failed' event is triggered.
        /// 
        /// The 'format' argument is a DRM_FORMAT code, as defined by the
        /// libdrm's drm_fourcc.h. The Linux kernel's DRM sub-system is the
        /// authoritative source on how the format codes should work.
        /// 
        /// The 'flags' is a bitfield of the flags defined in enum "flags".
        /// 'y_invert' means the that the image needs to be y-flipped.
        /// 
        /// Flag 'interlaced' means that the frame in the buffer is not
        /// progressive as usual, but interlaced. An interlaced buffer as
        /// supported here must always contain both top and bottom fields.
        /// The top field always begins on the first pixel row. The temporal
        /// ordering between the two fields is top field first, unless
        /// 'bottom_first' is specified. It is undefined whether 'bottom_first'
        /// is ignored if 'interlaced' is not set.
        /// 
        /// This protocol does not convey any information about field rate,
        /// duration, or timing, other than the relative ordering between the
        /// two fields in one buffer. A compositor may have to estimate the
        /// intended field rate from the incoming buffer rate. It is undefined
        /// whether the time of receiving wl_surface.commit with a new buffer
        /// attached, applying the wl_surface state, wl_surface.frame callback
        /// trigger, presentation, or any other point in the compositor cycle
        /// is used to measure the frame or field times. There is no support
        /// for detecting missed or late frames/fields/buffers either, and
        /// there is no support whatsoever for cooperating with interlaced
        /// compositor output.
        /// 
        /// The composited image quality resulting from the use of interlaced
        /// buffers is explicitly undefined. A compositor may use elaborate
        /// hardware features or software to deinterlace and create progressive
        /// output frames from a sequence of interlaced input buffers, or it
        /// may produce substandard image quality. However, compositors that
        /// cannot guarantee reasonable image quality in all cases are recommended
        /// to just reject all interlaced buffers.
        /// 
        /// Any argument errors, including non-positive width or height,
        /// mismatch between the number of planes and the format, bad
        /// format, bad offset or stride, may be indicated by fatal protocol
        /// errors: INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS,
        /// OUT_OF_BOUNDS.
        /// 
        /// Dmabuf import errors in the server that are not obvious client
        /// bugs are returned via the 'failed' event as non-fatal. This
        /// allows attempting dmabuf sharing and falling back in the client
        /// if it fails.
        /// 
        /// This request can be sent only once in the object's lifetime, after
        /// which the only legal request is destroy. This object should be
        /// destroyed after issuing a 'create' request. Attempting to use this
        /// object after issuing 'create' raises ALREADY_USED protocol error.
        /// 
        /// It is not mandatory to issue 'create'. If a client wants to
        /// cancel the buffer creation, it can just destroy this object.
        fn create(&self, width: i32, height: i32, format: u32, flags: u32) ->();
        /// immediately create a wl_buffer from the given dmabufs
        ///
        /// This asks for immediate creation of a wl_buffer by importing the
        /// added dmabufs.
        /// 
        /// In case of import success, no event is sent from the server, and the
        /// wl_buffer is ready to be used by the client.
        /// 
        /// Upon import failure, either of the following may happen, as seen fit
        /// by the implementation:
        /// - the client is terminated with one of the following fatal protocol
        /// errors:
        /// - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
        /// in case of argument errors such as mismatch between the number
        /// of planes and the format, bad format, non-positive width or
        /// height, or bad offset or stride.
        /// - INVALID_WL_BUFFER, in case the cause for failure is unknown or
        /// plaform specific.
        /// - the server creates an invalid wl_buffer, marks it as failed and
        /// sends a 'failed' event to the client. The result of using this
        /// invalid wl_buffer as an argument in any request by the client is
        /// defined by the compositor implementation.
        /// 
        /// This takes the same arguments as a 'create' request, and obeys the
        /// same restrictions.
        ///
        /// Only available since version 2 of the interface
        fn create_immed(&self, width: i32, height: i32, format: u32, flags: u32) ->Result<NewProxy<super::wl_buffer::WlBuffer>, ()>;
    }

    impl RequestsTrait for Proxy<ZwpLinuxBufferParamsV1> {
        fn destroy(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Destroy;
            self.send(msg);
        }

        fn add(&self, fd: ::std::os::unix::io::RawFd, plane_idx: u32, offset: u32, stride: u32, modifier_hi: u32, modifier_lo: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Add {
                fd: fd,
                plane_idx: plane_idx,
                offset: offset,
                stride: stride,
                modifier_hi: modifier_hi,
                modifier_lo: modifier_lo,
            };
            self.send(msg);
        }

        fn create(&self, width: i32, height: i32, format: u32, flags: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Create {
                width: width,
                height: height,
                format: format,
                flags: flags,
            };
            self.send(msg);
        }

        fn create_immed(&self, width: i32, height: i32, format: u32, flags: u32) ->Result<NewProxy<super::wl_buffer::WlBuffer>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_buffer_id_newproxy = self.child::<super::wl_buffer::WlBuffer>();
            let msg = Request::CreateImmed {
                buffer_id: unsafe { Proxy::<super::wl_buffer::WlBuffer>::from_c_ptr(_arg_buffer_id_newproxy.c_ptr()) },
                width: width,
                height: height,
                format: format,
                flags: flags,
            };
            self.send(msg);
            Ok(_arg_buffer_id_newproxy)
        }

    }
}

