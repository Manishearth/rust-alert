// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use AlertMethods;

use cocoa::appkit::{NSPoint, NSRect, NSSize};
use cocoa::base::{objc_getClass, sel_registerName};
use cocoa::base;
use std::mem::{transmute, transmute_copy};
use std::from_str::FromStr;
use core_foundation::base::TCFType;
use core_foundation::string::CFString;

/// The low-level alert type.
struct NSAlert;
/// The low-level type of a text field.
struct NSTextField;

/// An alert.
pub struct Alert {
    nsalert: *NSAlert,
    nstextfield: Option<*NSTextField>,
}

impl AlertMethods for Alert {
    /// Creates a new alert with an OK and Cancel button.
    fn new(message_text: &str) -> Alert {
        unsafe {
            let alert_string: CFString = FromStr::from_str(message_text).unwrap();
            let cancel_string = CFString::from_static_string("Cancel");
            let empty_string = CFString::from_static_string("");
            let class = objc_getClass(transmute(&"NSAlert"[0]));
            let selector = sel_registerName(transmute(&"alertWithMessageText:defaultButton:\
                                                        alternateButton:otherButton:\
                                                        informativeTextWithFormat:"[0]));
            let nsalert = base::invoke_msg_id_id_id_id_id_id(class,
                                                             selector,
                                                             transmute_copy(&alert_string),
                                                             transmute(0),
                                                             transmute_copy(&cancel_string),
                                                             transmute(0),
                                                             transmute_copy(&empty_string));
            Alert {
                nsalert: transmute(nsalert),
                nstextfield: None,
            }
        }
    }

    fn add_prompt(&mut self) {
        unsafe {
            // [NSTextField alloc]
            let class = objc_getClass(transmute(&"NSTextField"[0]));
            let selector = sel_registerName(transmute(&"alloc"[0]));
            let nstextfield = base::invoke_msg_id(class, selector);

            // [nstextfield initWithFrame: NSMakeRect(0, 0, 200, 24)]
            let selector = sel_registerName(transmute(&"initWithFrame:"[0]));
            let frame = NSRect {
                origin: NSPoint::new(0.0, 0.0),
                size: NSSize::new(200.0, 24.0),
            };
            let nstextfield = base::invoke_msg_id_NSRect(nstextfield, selector, &frame);

            // [nsalert setAccessoryView: nstextfield];
            let selector = sel_registerName(transmute(&"setAccessoryView:"[0]));
            base::invoke_msg_void_id(transmute(self.nsalert), selector, nstextfield);

            // [nsalert layout];
            let selector = sel_registerName(transmute(&"layout"[0]));
            base::invoke_msg_void(transmute(self.nsalert), selector);

            self.nstextfield = Some(transmute(nstextfield))
        }
    }

    fn run(&self) {
        unsafe {
            let selector = sel_registerName(transmute(&"runModal"[0]));
            base::invoke_msg_void(transmute(self.nsalert), selector)
        }
    }

    fn prompt_value(&self) -> String {
        unsafe {
            // [nstextfield stringValue]
            let selector = sel_registerName(transmute(&"stringValue"[0]));
            match self.nstextfield {
                None => fail!("No prompt!"),
                Some(nstextfield) => {
                    let string = base::invoke_msg_id(transmute(nstextfield), selector);
                    let result: CFString = TCFType::wrap_under_get_rule(transmute(string));
                    result.to_str()
                }
            }
        }
    }
}

