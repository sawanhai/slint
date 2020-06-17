/*!
The module responsible for the code generation.

There is one sub module for every language
*/

use crate::diagnostics::Diagnostics;
use crate::object_tree::{Component, ElementRc};
use std::rc::Rc;

#[cfg(feature = "cpp")]
mod cpp;

#[cfg(feature = "rust")]
pub mod rust;

pub fn generate(
    destination: &mut impl std::io::Write,
    component: &Rc<Component>,
    diag: &mut Diagnostics,
) -> std::io::Result<()> {
    #![allow(unused_variables)]
    #[cfg(feature = "cpp")]
    {
        if let Some(output) = cpp::generate(component, diag) {
            write!(destination, "{}", output)?;
        }
    }
    Ok(())
}

/// Visit each item in order in which they should appear in the children tree array.
/// The parameter of the visitor are the item, and the first_children_offset
#[allow(dead_code)]
pub fn build_array_helper(component: &Component, mut visit_item: impl FnMut(&ElementRc, u32)) {
    visit_item(&component.root_element, 1);
    visit_children(&component.root_element, 1, &mut visit_item);

    fn sub_children_count(e: &ElementRc) -> usize {
        let mut count = e.borrow().children.len();
        for i in &e.borrow().children {
            count += sub_children_count(i);
        }
        count
    }

    fn visit_children(
        item: &ElementRc,
        children_offset: u32,
        visit_item: &mut impl FnMut(&ElementRc, u32),
    ) {
        let mut offset = children_offset + item.borrow().children.len() as u32;
        for i in &item.borrow().children {
            visit_item(i, offset);
            offset += sub_children_count(i) as u32;
        }

        let mut offset = children_offset + item.borrow().children.len() as u32;
        for e in &item.borrow().children {
            visit_children(e, offset, visit_item);
            offset += sub_children_count(e) as u32;
        }
    }
}
