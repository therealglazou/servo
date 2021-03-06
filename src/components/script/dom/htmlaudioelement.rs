/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::HTMLAudioElementBinding;
use dom::document::AbstractDocument;
use dom::element::HTMLAudioElementTypeId;
use dom::htmlmediaelement::HTMLMediaElement;
use dom::node::{AbstractNode, Node};
use servo_util::str::DOMString;

pub struct HTMLAudioElement {
    htmlmediaelement: HTMLMediaElement
}

impl HTMLAudioElement {
    pub fn new_inherited(localName: DOMString, document: AbstractDocument) -> HTMLAudioElement {
        HTMLAudioElement {
            htmlmediaelement: HTMLMediaElement::new_inherited(HTMLAudioElementTypeId, localName, document)
        }
    }

    pub fn new(localName: DOMString, document: AbstractDocument) -> AbstractNode {
        let element = HTMLAudioElement::new_inherited(localName, document);
        Node::reflect_node(@mut element, document, HTMLAudioElementBinding::Wrap)
    }
}
