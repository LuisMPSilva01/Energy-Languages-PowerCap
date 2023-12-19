
//
// This file was auto-generated, do not edit directly.
//

/*
Copyright © 2012, 2013 Intel Corporation

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

pub mod zwp_text_input_v1 {
    //! text input
    //!
    //! An object used for text input. Adds support for text input and input
    //! methods to applications. A text_input object is created from a
    //! wl_text_input_manager and corresponds typically to a text entry in an
    //! application.
    //! 
    //! Requests are used to activate/deactivate the text_input object and set
    //! state information like surrounding and selected text or the content type.
    //! The information about entered text is sent to the text_input object via
    //! the pre-edit and commit events. Using this interface removes the need
    //! for applications to directly process hardware key events and compose text
    //! out of them.
    //! 
    //! Text is generally UTF-8 encoded, indices and lengths are in bytes.
    //! 
    //! Serials are used to synchronize the state between the text input and
    //! an input method. New serials are sent by the text input in the
    //! commit_state request and are used by the input method to indicate
    //! the known text input state in events like preedit_string, commit_string,
    //! and keysym. The text input can then ignore events from the input method
    //! which are based on an outdated state (for example after a reset).
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
    /// content hint
    ///
    /// Content hint is a bitmask to allow to modify the behavior of the text
    /// input.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ContentHint {
        /// no special behaviour
        None = 0x0,
        /// auto completion, correction and capitalization
        Default = 0x7,
        /// hidden and sensitive text
        Password = 0xc0,
        /// suggest word completions
        AutoCompletion = 0x1,
        /// suggest word corrections
        AutoCorrection = 0x2,
        /// switch to uppercase letters at the start of a sentence
        AutoCapitalization = 0x4,
        /// prefer lowercase letters
        Lowercase = 0x8,
        /// prefer uppercase letters
        Uppercase = 0x10,
        /// prefer casing for titles and headings (can be language dependent)
        Titlecase = 0x20,
        /// characters should be hidden
        HiddenText = 0x40,
        /// typed text should not be stored
        SensitiveData = 0x80,
        /// just latin characters should be entered
        Latin = 0x100,
        /// the text input is multiline
        Multiline = 0x200,
    }
    impl ContentHint {
        pub fn from_raw(n: u32) -> Option<ContentHint> {
            match n {
                0x0 => Some(ContentHint::None),
                0x7 => Some(ContentHint::Default),
                0xc0 => Some(ContentHint::Password),
                0x1 => Some(ContentHint::AutoCompletion),
                0x2 => Some(ContentHint::AutoCorrection),
                0x4 => Some(ContentHint::AutoCapitalization),
                0x8 => Some(ContentHint::Lowercase),
                0x10 => Some(ContentHint::Uppercase),
                0x20 => Some(ContentHint::Titlecase),
                0x40 => Some(ContentHint::HiddenText),
                0x80 => Some(ContentHint::SensitiveData),
                0x100 => Some(ContentHint::Latin),
                0x200 => Some(ContentHint::Multiline),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    /// content purpose
    ///
    /// The content purpose allows to specify the primary purpose of a text
    /// input.
    /// 
    /// This allows an input method to show special purpose input panels with
    /// extra characters or to disallow some characters.

    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum ContentPurpose {
        /// default input, allowing all characters
        Normal = 0,
        /// allow only alphabetic characters
        Alpha = 1,
        /// allow only digits
        Digits = 2,
        /// input a number (including decimal separator and sign)
        Number = 3,
        /// input a phone number
        Phone = 4,
        /// input an URL
        Url = 5,
        /// input an email address
        Email = 6,
        /// input a name of a person
        Name = 7,
        /// input a password (combine with password or sensitive_data hint)
        Password = 8,
        /// input a date
        Date = 9,
        /// input a time
        Time = 10,
        /// input a date and time
        Datetime = 11,
        /// input for a terminal
        Terminal = 12,
    }
    impl ContentPurpose {
        pub fn from_raw(n: u32) -> Option<ContentPurpose> {
            match n {
                0 => Some(ContentPurpose::Normal),
                1 => Some(ContentPurpose::Alpha),
                2 => Some(ContentPurpose::Digits),
                3 => Some(ContentPurpose::Number),
                4 => Some(ContentPurpose::Phone),
                5 => Some(ContentPurpose::Url),
                6 => Some(ContentPurpose::Email),
                7 => Some(ContentPurpose::Name),
                8 => Some(ContentPurpose::Password),
                9 => Some(ContentPurpose::Date),
                10 => Some(ContentPurpose::Time),
                11 => Some(ContentPurpose::Datetime),
                12 => Some(ContentPurpose::Terminal),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }


    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum PreeditStyle {
        /// default style for composing text
        Default = 0,
        /// style should be the same as in non-composing text
        None = 1,
        Active = 2,
        Inactive = 3,
        Highlight = 4,
        Underline = 5,
        Selection = 6,
        Incorrect = 7,
    }
    impl PreeditStyle {
        pub fn from_raw(n: u32) -> Option<PreeditStyle> {
            match n {
                0 => Some(PreeditStyle::Default),
                1 => Some(PreeditStyle::None),
                2 => Some(PreeditStyle::Active),
                3 => Some(PreeditStyle::Inactive),
                4 => Some(PreeditStyle::Highlight),
                5 => Some(PreeditStyle::Underline),
                6 => Some(PreeditStyle::Selection),
                7 => Some(PreeditStyle::Incorrect),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }


    #[repr(u32)]
    #[derive(Copy,Clone,Debug,PartialEq)]
    pub enum TextDirection {
        /// automatic text direction based on text and language
        Auto = 0,
        /// left-to-right
        Ltr = 1,
        /// right-to-left
        Rtl = 2,
    }
    impl TextDirection {
        pub fn from_raw(n: u32) -> Option<TextDirection> {
            match n {
                0 => Some(TextDirection::Auto),
                1 => Some(TextDirection::Ltr),
                2 => Some(TextDirection::Rtl),

                _ => Option::None
            }
        }
        pub fn to_raw(&self) -> u32 {
            *self as u32
        }
    }

    pub enum Request {
        /// request activation
        ///
        /// Requests the text_input object to be activated (typically when the
        /// text entry gets focus).
        /// 
        /// The seat argument is a wl_seat which maintains the focus for this
        /// activation. The surface argument is a wl_surface assigned to the
        /// text_input object and tracked for focus lost. The enter event
        /// is emitted on successful activation.
        Activate {seat: Proxy<super::wl_seat::WlSeat>, surface: Proxy<super::wl_surface::WlSurface>, },
        /// request deactivation
        ///
        /// Requests the text_input object to be deactivated (typically when the
        /// text entry lost focus). The seat argument is a wl_seat which was used
        /// for activation.
        Deactivate {seat: Proxy<super::wl_seat::WlSeat>, },
        /// show input panels
        ///
        /// Requests input panels (virtual keyboard) to show.
        ShowInputPanel,
        /// hide input panels
        ///
        /// Requests input panels (virtual keyboard) to hide.
        HideInputPanel,
        /// reset
        ///
        /// Should be called by an editor widget when the input state should be
        /// reset, for example after the text was changed outside of the normal
        /// input method flow.
        Reset,
        /// sets the surrounding text
        ///
        /// Sets the plain surrounding text around the input position. Text is
        /// UTF-8 encoded. Cursor is the byte offset within the
        /// surrounding text. Anchor is the byte offset of the
        /// selection anchor within the surrounding text. If there is no selected
        /// text anchor, then it is the same as cursor.
        SetSurroundingText {text: String, cursor: u32, anchor: u32, },
        /// set content purpose and hint
        ///
        /// Sets the content purpose and content hint. While the purpose is the
        /// basic purpose of an input field, the hint flags allow to modify some
        /// of the behavior.
        /// 
        /// When no content type is explicitly set, a normal content purpose with
        /// default hints (auto completion, auto correction, auto capitalization)
        /// should be assumed.
        SetContentType {hint: u32, purpose: u32, },
        SetCursorRectangle {x: i32, y: i32, width: i32, height: i32, },
        /// sets preferred language
        ///
        /// Sets a specific language. This allows for example a virtual keyboard to
        /// show a language specific layout. The "language" argument is an RFC-3066
        /// format language tag.
        /// 
        /// It could be used for example in a word processor to indicate the
        /// language of the currently edited document or in an instant message
        /// application which tracks languages of contacts.
        SetPreferredLanguage {language: String, },
        CommitState {serial: u32, },
        InvokeAction {button: u32, index: u32, },
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
                Request::Activate { seat, surface, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    _args_array[1].o = surface.c_ptr() as *mut _;
                    f(0, &mut _args_array)
                },
                Request::Deactivate { seat, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = seat.c_ptr() as *mut _;
                    f(1, &mut _args_array)
                },
                Request::ShowInputPanel => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(2, &mut _args_array)
                },
                Request::HideInputPanel => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(3, &mut _args_array)
                },
                Request::Reset => {
                    let mut _args_array: [wl_argument; 0] = unsafe { ::std::mem::zeroed() };
                    f(4, &mut _args_array)
                },
                Request::SetSurroundingText { text, cursor, anchor, } => {
                    let mut _args_array: [wl_argument; 3] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(text).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    _args_array[1].u = cursor;
                    _args_array[2].u = anchor;
                    f(5, &mut _args_array)
                },
                Request::SetContentType { hint, purpose, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = hint;
                    _args_array[1].u = purpose;
                    f(6, &mut _args_array)
                },
                Request::SetCursorRectangle { x, y, width, height, } => {
                    let mut _args_array: [wl_argument; 4] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].i = x;
                    _args_array[1].i = y;
                    _args_array[2].i = width;
                    _args_array[3].i = height;
                    f(7, &mut _args_array)
                },
                Request::SetPreferredLanguage { language, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    let _arg_0 = ::std::ffi::CString::new(language).unwrap();
                    _args_array[0].s = _arg_0.as_ptr();
                    f(8, &mut _args_array)
                },
                Request::CommitState { serial, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = serial;
                    f(9, &mut _args_array)
                },
                Request::InvokeAction { button, index, } => {
                    let mut _args_array: [wl_argument; 2] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].u = button;
                    _args_array[1].u = index;
                    f(10, &mut _args_array)
                },
            }
        }
    }

    pub enum Event {
        /// enter event
        ///
        /// Notify the text_input object when it received focus. Typically in
        /// response to an activate request.
        Enter {surface: Proxy<super::wl_surface::WlSurface>, },
        /// leave event
        ///
        /// Notify the text_input object when it lost focus. Either in response
        /// to a deactivate request or when the assigned surface lost focus or was
        /// destroyed.
        Leave,
        /// modifiers map
        ///
        /// Transfer an array of 0-terminated modifier names. The position in
        /// the array is the index of the modifier as used in the modifiers
        /// bitmask in the keysym event.
        ModifiersMap {map: Vec<u8>, },
        /// state of the input panel
        ///
        /// Notify when the visibility state of the input panel changed.
        InputPanelState {state: u32, },
        /// pre-edit
        ///
        /// Notify when a new composing text (pre-edit) should be set around the
        /// current cursor position. Any previously set composing text should
        /// be removed.
        /// 
        /// The commit text can be used to replace the preedit text on reset
        /// (for example on unfocus).
        /// 
        /// The text input should also handle all preedit_style and preedit_cursor
        /// events occurring directly before preedit_string.
        PreeditString {serial: u32, text: String, commit: String, },
        /// pre-edit styling
        ///
        /// Sets styling information on composing text. The style is applied for
        /// length bytes from index relative to the beginning of the composing
        /// text (as byte offset). Multiple styles can
        /// be applied to a composing text by sending multiple preedit_styling
        /// events.
        /// 
        /// This event is handled as part of a following preedit_string event.
        PreeditStyling {index: u32, length: u32, style: u32, },
        /// pre-edit cursor
        ///
        /// Sets the cursor position inside the composing text (as byte
        /// offset) relative to the start of the composing text. When index is a
        /// negative number no cursor is shown.
        /// 
        /// This event is handled as part of a following preedit_string event.
        PreeditCursor {index: i32, },
        /// commit
        ///
        /// Notify when text should be inserted into the editor widget. The text to
        /// commit could be either just a single character after a key press or the
        /// result of some composing (pre-edit). It could also be an empty text
        /// when some text should be removed (see delete_surrounding_text) or when
        /// the input cursor should be moved (see cursor_position).
        /// 
        /// Any previously set composing text should be removed.
        CommitString {serial: u32, text: String, },
        /// set cursor to new position
        ///
        /// Notify when the cursor or anchor position should be modified.
        /// 
        /// This event should be handled as part of a following commit_string
        /// event.
        CursorPosition {index: i32, anchor: i32, },
        /// delete surrounding text
        ///
        /// Notify when the text around the current cursor position should be
        /// deleted.
        /// 
        /// Index is relative to the current cursor (in bytes).
        /// Length is the length of deleted text (in bytes).
        /// 
        /// This event should be handled as part of a following commit_string
        /// event.
        DeleteSurroundingText {index: i32, length: u32, },
        /// keysym
        ///
        /// Notify when a key event was sent. Key events should not be used
        /// for normal text input operations, which should be done with
        /// commit_string, delete_surrounding_text, etc. The key event follows
        /// the wl_keyboard key event convention. Sym is an XKB keysym, state a
        /// wl_keyboard key_state. Modifiers are a mask for effective modifiers
        /// (where the modifier indices are set by the modifiers_map event)
        Keysym {serial: u32, time: u32, sym: u32, state: u32, modifiers: u32, },
        /// language
        ///
        /// Sets the language of the input text. The "language" argument is an
        /// RFC-3066 format language tag.
        Language {serial: u32, language: String, },
        /// text direction
        ///
        /// Sets the text direction of input text.
        /// 
        /// It is mainly needed for showing an input cursor on the correct side of
        /// the editor when there is no input done yet and making sure neutral
        /// direction text is laid out properly.
        TextDirection {serial: u32, direction: u32, },
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
                    Ok(Event::Enter {
                        surface: Proxy::<super::wl_surface::WlSurface>::from_c_ptr(_args[0].o as *mut _),
                }) },
                1 => {
                    Ok(Event::Leave) },
                2 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::ModifiersMap {
                        map: { let array = &*_args[0].a; ::std::slice::from_raw_parts(array.data as *const u8, array.size).to_owned() },
                }) },
                3 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::InputPanelState {
                        state: _args[0].u,
                }) },
                4 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::PreeditString {
                        serial: _args[0].u,
                        text: ::std::ffi::CStr::from_ptr(_args[1].s).to_string_lossy().into_owned(),
                        commit: ::std::ffi::CStr::from_ptr(_args[2].s).to_string_lossy().into_owned(),
                }) },
                5 => {
                    let _args = ::std::slice::from_raw_parts(args, 3);
                    Ok(Event::PreeditStyling {
                        index: _args[0].u,
                        length: _args[1].u,
                        style: _args[2].u,
                }) },
                6 => {
                    let _args = ::std::slice::from_raw_parts(args, 1);
                    Ok(Event::PreeditCursor {
                        index: _args[0].i,
                }) },
                7 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::CommitString {
                        serial: _args[0].u,
                        text: ::std::ffi::CStr::from_ptr(_args[1].s).to_string_lossy().into_owned(),
                }) },
                8 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::CursorPosition {
                        index: _args[0].i,
                        anchor: _args[1].i,
                }) },
                9 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::DeleteSurroundingText {
                        index: _args[0].i,
                        length: _args[1].u,
                }) },
                10 => {
                    let _args = ::std::slice::from_raw_parts(args, 5);
                    Ok(Event::Keysym {
                        serial: _args[0].u,
                        time: _args[1].u,
                        sym: _args[2].u,
                        state: _args[3].u,
                        modifiers: _args[4].u,
                }) },
                11 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::Language {
                        serial: _args[0].u,
                        language: ::std::ffi::CStr::from_ptr(_args[1].s).to_string_lossy().into_owned(),
                }) },
                12 => {
                    let _args = ::std::slice::from_raw_parts(args, 2);
                    Ok(Event::TextDirection {
                        serial: _args[0].u,
                        direction: _args[1].u,
                }) },
                _ => return Err(())
            }
        }

        fn as_raw_c_in<F, T>(self, f: F) -> T where F: FnOnce(u32, &mut [wl_argument]) -> T {
            panic!("Event::as_raw_c_in can not be used Client-side.")
        }
    }


    pub struct ZwpTextInputV1;

    impl Interface for ZwpTextInputV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_text_input_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_text_input_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// request activation
        ///
        /// Requests the text_input object to be activated (typically when the
        /// text entry gets focus).
        /// 
        /// The seat argument is a wl_seat which maintains the focus for this
        /// activation. The surface argument is a wl_surface assigned to the
        /// text_input object and tracked for focus lost. The enter event
        /// is emitted on successful activation.
        fn activate(&self, seat: &Proxy<super::wl_seat::WlSeat>, surface: &Proxy<super::wl_surface::WlSurface>) ->();
        /// request deactivation
        ///
        /// Requests the text_input object to be deactivated (typically when the
        /// text entry lost focus). The seat argument is a wl_seat which was used
        /// for activation.
        fn deactivate(&self, seat: &Proxy<super::wl_seat::WlSeat>) ->();
        /// show input panels
        ///
        /// Requests input panels (virtual keyboard) to show.
        fn show_input_panel(&self) ->();
        /// hide input panels
        ///
        /// Requests input panels (virtual keyboard) to hide.
        fn hide_input_panel(&self) ->();
        /// reset
        ///
        /// Should be called by an editor widget when the input state should be
        /// reset, for example after the text was changed outside of the normal
        /// input method flow.
        fn reset(&self) ->();
        /// sets the surrounding text
        ///
        /// Sets the plain surrounding text around the input position. Text is
        /// UTF-8 encoded. Cursor is the byte offset within the
        /// surrounding text. Anchor is the byte offset of the
        /// selection anchor within the surrounding text. If there is no selected
        /// text anchor, then it is the same as cursor.
        fn set_surrounding_text(&self, text: String, cursor: u32, anchor: u32) ->();
        /// set content purpose and hint
        ///
        /// Sets the content purpose and content hint. While the purpose is the
        /// basic purpose of an input field, the hint flags allow to modify some
        /// of the behavior.
        /// 
        /// When no content type is explicitly set, a normal content purpose with
        /// default hints (auto completion, auto correction, auto capitalization)
        /// should be assumed.
        fn set_content_type(&self, hint: u32, purpose: u32) ->();
        fn set_cursor_rectangle(&self, x: i32, y: i32, width: i32, height: i32) ->();
        /// sets preferred language
        ///
        /// Sets a specific language. This allows for example a virtual keyboard to
        /// show a language specific layout. The "language" argument is an RFC-3066
        /// format language tag.
        /// 
        /// It could be used for example in a word processor to indicate the
        /// language of the currently edited document or in an instant message
        /// application which tracks languages of contacts.
        fn set_preferred_language(&self, language: String) ->();
        fn commit_state(&self, serial: u32) ->();
        fn invoke_action(&self, button: u32, index: u32) ->();
    }

    impl RequestsTrait for Proxy<ZwpTextInputV1> {
        fn activate(&self, seat: &Proxy<super::wl_seat::WlSeat>, surface: &Proxy<super::wl_surface::WlSurface>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Activate {
                seat: seat.clone(),
                surface: surface.clone(),
            };
            self.send(msg);
        }

        fn deactivate(&self, seat: &Proxy<super::wl_seat::WlSeat>) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Deactivate {
                seat: seat.clone(),
            };
            self.send(msg);
        }

        fn show_input_panel(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::ShowInputPanel;
            self.send(msg);
        }

        fn hide_input_panel(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::HideInputPanel;
            self.send(msg);
        }

        fn reset(&self) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::Reset;
            self.send(msg);
        }

        fn set_surrounding_text(&self, text: String, cursor: u32, anchor: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetSurroundingText {
                text: text,
                cursor: cursor,
                anchor: anchor,
            };
            self.send(msg);
        }

        fn set_content_type(&self, hint: u32, purpose: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetContentType {
                hint: hint,
                purpose: purpose,
            };
            self.send(msg);
        }

        fn set_cursor_rectangle(&self, x: i32, y: i32, width: i32, height: i32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetCursorRectangle {
                x: x,
                y: y,
                width: width,
                height: height,
            };
            self.send(msg);
        }

        fn set_preferred_language(&self, language: String) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::SetPreferredLanguage {
                language: language,
            };
            self.send(msg);
        }

        fn commit_state(&self, serial: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::CommitState {
                serial: serial,
            };
            self.send(msg);
        }

        fn invoke_action(&self, button: u32, index: u32) ->() {
            if !self.is_external() && !self.is_alive() {
                return;
            }
            let msg = Request::InvokeAction {
                button: button,
                index: index,
            };
            self.send(msg);
        }

    }
}

pub mod zwp_text_input_manager_v1 {
    //! text input manager
    //!
    //! A factory for text_input objects. This object is a global singleton.
    use super::{Proxy, NewProxy, AnonymousObject, Interface, MessageGroup};

    use super::sys::common::{wl_argument, wl_interface, wl_array};
    use super::sys::client::*;
    pub enum Request {
        /// create text input
        ///
        /// Creates a new text_input object.
        CreateTextInput {id: Proxy<super::zwp_text_input_v1::ZwpTextInputV1>, },
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
                Request::CreateTextInput { id, } => {
                    let mut _args_array: [wl_argument; 1] = unsafe { ::std::mem::zeroed() };
                    _args_array[0].o = id.c_ptr() as *mut _;
                    f(0, &mut _args_array)
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


    pub struct ZwpTextInputManagerV1;

    impl Interface for ZwpTextInputManagerV1 {
        type Request = Request;
        type Event = Event;
        const NAME: &'static str = "zwp_text_input_manager_v1";
        fn c_interface() -> *const wl_interface {
            unsafe { &super::super::c_interfaces::zwp_text_input_manager_v1_interface }
        }
    }

    pub trait RequestsTrait {
        /// create text input
        ///
        /// Creates a new text_input object.
        fn create_text_input(&self) ->Result<NewProxy<super::zwp_text_input_v1::ZwpTextInputV1>, ()>;
    }

    impl RequestsTrait for Proxy<ZwpTextInputManagerV1> {
        fn create_text_input(&self) ->Result<NewProxy<super::zwp_text_input_v1::ZwpTextInputV1>, ()> {
            if !self.is_external() && !self.is_alive() {
                return Err(());
            }
            let _arg_id_newproxy = self.child::<super::zwp_text_input_v1::ZwpTextInputV1>();
            let msg = Request::CreateTextInput {
                id: unsafe { Proxy::<super::zwp_text_input_v1::ZwpTextInputV1>::from_c_ptr(_arg_id_newproxy.c_ptr()) },
            };
            self.send(msg);
            Ok(_arg_id_newproxy)
        }

    }
}

