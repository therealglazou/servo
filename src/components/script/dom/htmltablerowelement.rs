/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLTableRowElementBinding;
use dom::bindings::utils::ErrorResult;
use dom::document::AbstractDocument;
use dom::element::HTMLTableRowElementTypeId;
use dom::htmlelement::HTMLElement;
use dom::node::{AbstractNode, Node};
use servo_util::str::DOMString;

pub struct HTMLTableRowElement {
    htmlelement: HTMLElement,
}

impl HTMLTableRowElement {
    pub fn new_inherited(localName: DOMString, document: AbstractDocument) -> HTMLTableRowElement {
        HTMLTableRowElement {
            htmlelement: HTMLElement::new_inherited(HTMLTableRowElementTypeId, localName, document)
        }
    }

    pub fn new(localName: DOMString, document: AbstractDocument) -> AbstractNode {
        let element = HTMLTableRowElement::new_inherited(localName, document);
        Node::reflect_node(@mut element, document, HTMLTableRowElementBinding::Wrap)
    }
}

impl HTMLTableRowElement {
    pub fn RowIndex(&self) -> i32 {
        0
    }

    pub fn GetRowIndex(&self) -> i32 {
        0
    }

    pub fn SectionRowIndex(&self) -> i32 {
        0
    }

    pub fn GetSectionRowIndex(&self) -> i32 {
        0
    }

    pub fn DeleteCell(&mut self, _index: i32) -> ErrorResult {
        Ok(())
    }

    pub fn Align(&self) -> DOMString {
        ~""
    }

    pub fn SetAlign(&self, _align: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn Ch(&self) -> DOMString {
        ~""
    }

    pub fn SetCh(&self, _ch: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn ChOff(&self) -> DOMString {
        ~""
    }

    pub fn SetChOff(&self, _ch_off: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn VAlign(&self) -> DOMString {
        ~""
    }

    pub fn SetVAlign(&self, _v_align: DOMString) -> ErrorResult {
        Ok(())
    }

    pub fn BgColor(&self) -> DOMString {
        ~""
    }

    pub fn SetBgColor(&self, _bg_color: DOMString) -> ErrorResult {
        Ok(())
    }
}
