mod old {

    // #![feature(type_alias_impl_trait)]
    //
    use std::future::Future;

    trait Props {}
    struct Context<T: Props> {
        _props: std::marker::PhantomData<T>,
    }
    struct VNode {}

    // type FC<T: Props> = fn(&mut Context<T>) -> VNode;
    // type FC<T: Props> = fn(&mut Context<T>) -> Box<dyn Future<Output = VNode>>;

    impl Props for () {}

    // async fn some_component(g: &mut Context<()>) -> VNode {
    //     rsx! {
    //         <div>

    //         </div>
    //     }
    // }
    // Absolve ourselves of any type data about the context itself
    trait ContextApplier {
        fn use_hook<O, H>(
            &mut self,
            initializer: impl FnOnce() -> H,
            runner: impl Fn(&mut H) -> O,
            tear_down: impl Fn(&mut H),
        ) -> O;
    }
    impl<T: Props> ContextApplier for Context<T> {
        fn use_hook<O, H>(
            &mut self,
            initializer: impl FnOnce() -> H,
            runner: impl Fn(&mut H) -> O,
            tear_down: impl Fn(&mut H),
        ) -> O {
            todo!()
        }
    }

    fn use_state<T>(c: &mut impl ContextApplier, g: impl Fn() -> T) -> T {
        c.use_hook(|| {}, |_| {}, |_| {});
        g()
    }

    enum SomeComponent {
        Imperative,
        Async,
    }

    // impl<F, G> From<F> for SomeComponent
    // where
    //     F: Fn() -> G,
    //     G: Future<Output = ()>,
    // {
    //     fn from(_: F) -> Self {
    //         SomeComponent::Async
    //     }
    // }

    // impl From<fn() -> ()> for SomeComponent {
    //     fn from(_: F) -> Self {
    //         SomeComponent::Async
    //     }
    // }
    // impl<F> Into<SomeComponent> for fn() -> F
    // where
    //     F: Future<Output = ()>,
    // {
    //     fn into(self) -> SomeComponent {
    //         todo!()
    //     }
    // }

    // #[test]
    // fn test() {
    //     let b: SomeComponent = test_comp.into();
    // }

    // Does this make sense?
    // Any component labeled with async can halt its rendering, but won't be able to process updates?
    // Or, those updates can still happen virtually, just not propogated into the view?
    // async fn test_comp() -> () {
    //     timer::new(300).await;
    //     html! {
    //         <div>
    //             "hello world!"
    //         </div>
    //     }
    // }

    // fn use_state<T: Props>(c: &mut Context<T>) {}

    // async fn another_component(ctx: &mut Context<()>) -> VNode {
    //     // delay the re-render until component when the future is ready
    //     // "use_future" loads the promise and provides a value (aka a loadable)
    //     let value = use_effect(move || async {
    //         get_value().join(timer::new(300));
    //         set_value(blah);
    //     });

    //     rsx! {
    //         <Suspense fallback={<div>"Loading..."</div>}>
    //             <div>
    //                 "hello {name}!"
    //             </div>
    //         <Suspense />
    //     }
    // }

    /*

    Rationale
    Today, you can do use_async and do some async operations,







    */
    // type FC<P: Props> = fn(&mut Context<P>) -> VNode;

    // static Example: FC<()> = |_| async {
    //     // some async work
    // };

    // type FC2 = fn() -> impl Future<Output = ()>;
    // struct fc<P: Props>(fn(&mut Context<P>) -> G);
    // fn blah<P: Props, G: Future<Output = VNode>>(a: fn(&mut Context<P>) -> G) {}

    // static Example2: FC2<()> = fc(|_| async { VNode {} });
    // static Example2: () = blah(|_: &mut Context<()>| async { VNode {} });

    // static Example: FC<()> = |_| {
    //     let g = async { VNode {} };
    //     Box::new(g)
    // };

    // static Example2:  = || {};

    // type FA<R: Future<Output = i32>> = fn(i32) -> R;

    // async fn my_component()
    // static MyThing: FA<dyn Future<Output = i32>> = |_| async { 10 };

    // type SomeFn = fn() -> ();

    // static MyFn: SomeFn = || {};
}

mod old2 {
    mod vdom {
        //! Virtual DOM implementation
        use super::*;

        pub struct VDom {
            patches: Vec<Patch>,
        }

        impl VDom {
            // fn new(root: ComponentFn) -> Self {
            //     let scope = Scope::new();
            //     Self {}
            // }
        }
    }

    mod nodes {}

    mod patch {}

    mod scope {
        //! Wrappers around components

        pub struct Scope {}

        impl Scope {
            fn new() -> Self {
                Self {}
            }
        }
    }

    mod context {}

    struct EventListener {}

    struct VNode {
        /// key-value pairs of attributes
        attributes: Vec<(&'static str, &'static str)>,

        /// onclick/onhover/on etc listeners
        /// goal is to standardize around a set of cross-platform listeners?
        listeners: Vec<EventListener>,

        /// Direct children, non arena-allocated
        children: Vec<VNode>,
    }

    enum ElementType {
        div,
        p,
        a,
        img,
    }

    struct ComponentContext {}
    type ComponentFn = fn(ctx: &ComponentContext) -> VNode;

    enum Patch {}

    mod tests {
        use super::*;

        /// Ensure components can be made from the raw components
        #[test]
        fn simple_test() {
            fn component(ctx: &ComponentContext) -> VNode {
                println!("Running component");
                VNode {}
            }

            let dom = VDom::new(component);
        }

        /// Ensure components can be made from the raw components
        #[test]
        fn simple_test_closure() {
            let component: ComponentFn = |ctx| {
                println!("Running component");
                VNode {}
            };

            let dom = VDom::new(component);
        }
    }
}

mod text {
    //! Old methods that clouded the element implementation
    //! These all add a dedicated text renderer implementation

    mod vnode {

        impl From<&str> for VNode {
            fn from(other: &str) -> Self {
                VNode::text(other)
            }
        }

        impl From<String> for VNode {
            fn from(other: String) -> Self {
                VNode::text(other.as_str())
            }
        }

        // -----------------------------------------------
        //  Allow VNodes to be iterated for map-based UI
        // -----------------------------------------------
        impl IntoIterator for VNode {
            type Item = VNode;
            // TODO: Is this possible with an array [VNode] instead of a vec?
            type IntoIter = ::std::vec::IntoIter<VNode>;

            fn into_iter(self) -> Self::IntoIter {
                vec![self].into_iter()
            }
        }

        impl Into<::std::vec::IntoIter<VNode>> for VNode {
            fn into(self) -> ::std::vec::IntoIter<VNode> {
                self.into_iter()
            }
        }

        // -----------------------------------------------
        //  Allow debug/display adherent to the HTML spec
        // -----------------------------------------------
        use std::fmt;
        impl fmt::Debug for VNode {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    VNode::Element(e) => write!(f, "Node::{:?}", e),
                    VNode::Text(t) => write!(f, "Node::{:?}", t),
                    VNode::Component(c) => write!(f, "Node::{:?}", c),
                }
            }
        }

        // Turn a VNode into an HTML string (delegate impl to variants)
        impl fmt::Display for VNode {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    VNode::Element(element) => write!(f, "{}", element),
                    VNode::Text(text) => write!(f, "{}", text),
                    VNode::Component(c) => write!(f, "{}", c),
                }
            }
        }
    }

    mod velement {
        // -----------------------------------------------
        //  Allow debug/display adherent to the HTML spec
        // -----------------------------------------------
        use std::fmt;
        impl fmt::Debug for VElement {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "Element(<{}>, attrs: {:?}, children: {:?})",
                    self.tag, self.attrs, self.children,
                )
            }
        }

        impl fmt::Display for VElement {
            // Turn a VElement and all of it's children (recursively) into an HTML string
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "<{}", self.tag).unwrap();

                for (attr, value) in self.attrs.iter() {
                    write!(f, r#" {}="{}""#, attr, value)?;
                }

                write!(f, ">")?;

                for child in self.children.iter() {
                    write!(f, "{}", child.to_string())?;
                }

                if !crate::validation::is_self_closing(&self.tag) {
                    write!(f, "</{}>", self.tag)?;
                }

                Ok(())
            }
        }
    }

    mod vtext {
        // -----------------------------------------------
        //  Convert from primitives directly into VText
        // -----------------------------------------------
        impl From<&str> for VText {
            fn from(text: &str) -> Self {
                VText {
                    text: text.to_string(),
                }
            }
        }

        impl From<String> for VText {
            fn from(text: String) -> Self {
                VText { text }
            }
        }

        // -----------------------------------------------
        //  Allow debug/display adherent to the HTML spec
        // -----------------------------------------------
        use std::fmt;
        impl fmt::Debug for VText {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Text({})", self.text)
            }
        }

        // Turn a VText into an HTML string
        impl fmt::Display for VText {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.text)
            }
        }
    }

    mod iterables {

        // TODO @Jon
        // Set this up so instead of the view trait, we can just take functions
        // Functions with no context should just be rendered
        // But functions with a context should be treated as regular components

        // impl<V: View> From<Vec<V>> for IterableNodes {
        //     fn from(other: Vec<V>) -> Self {
        //         IterableNodes(other.into_iter().map(|it| it.render()).collect())
        //     }
        // }

        // impl<V: View> From<&Vec<V>> for IterableNodes {
        //     fn from(other: &Vec<V>) -> Self {
        //         IterableNodes(other.iter().map(|it| it.render()).collect())
        //     }
        // }

        // impl<V: View> From<&[V]> for IterableNodes {
        //     fn from(other: &[V]) -> Self {
        //         IterableNodes(other.iter().map(|it| it.render()).collect())
        //     }
        // }

        impl From<&str> for IterableNodes {
            fn from(other: &str) -> Self {
                IterableNodes(vec![VNode::text(other)])
            }
        }

        impl From<String> for IterableNodes {
            fn from(other: String) -> Self {
                IterableNodes(vec![VNode::text(other.as_str())])
            }
        }
    }

    mod tests {
        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn self_closing_tag_to_string() {
                let node = VNode::element("br");

                // No </br> since self closing tag
                assert_eq!(&node.to_string(), "<br>");
            }

            #[test]
            fn to_string() {
                let mut node = VNode::Element(VElement::new("div"));
                node.as_velement_mut()
                    .unwrap()
                    .attrs
                    .insert("id".into(), "some-id".into());

                let mut child = VNode::Element(VElement::new("span"));

                let mut text = VNode::Text(VText::new("Hello world"));

                child.as_velement_mut().unwrap().children.push(text);

                node.as_velement_mut().unwrap().children.push(child);

                let expected = r#"<div id="some-id"><span>Hello world</span></div>"#;

                assert_eq!(node.to_string(), expected);
            }
        }
    }

    mod ddiff {
        /// The diffing algorithm to compare two VNode trees and generate a list of patches to update the VDom.
        /// Currently, using an index-based patching algorithm
        ///
        pub mod diff {
            use super::*;
            use crate::nodes::{VNode, VText};
            use std::cmp::min;
            use std::collections::HashMap;
            use std::mem;

            // pub use apply_patches::patch;

            /// A Patch encodes an operation that modifies a real DOM element.
            ///
            /// To update the real DOM that a user sees you'll want to first diff your
            /// old virtual dom and new virtual dom.
            ///
            /// This diff operation will generate `Vec<Patch>` with zero or more patches that, when
            /// applied to your real DOM, will make your real DOM look like your new virtual dom.
            ///
            /// Each Patch has a u32 node index that helps us identify the real DOM node that it applies to.
            ///
            /// Our old virtual dom's nodes are indexed depth first, as shown in this illustration
            /// (0 being the root node, 1 being it's first child, 2 being it's first child's first child).
            ///
            /// ```text
            ///             .─.
            ///            ( 0 )
            ///             `┬'
            ///         ┌────┴──────┐
            ///         │           │
            ///         ▼           ▼
            ///        .─.         .─.
            ///       ( 1 )       ( 4 )
            ///        `┬'         `─'
            ///    ┌────┴───┐       │
            ///    │        │       ├─────┬─────┐
            ///    ▼        ▼       │     │     │
            ///   .─.      .─.      ▼     ▼     ▼
            ///  ( 2 )    ( 3 )    .─.   .─.   .─.
            ///   `─'      `─'    ( 5 ) ( 6 ) ( 7 )
            ///                    `─'   `─'   `─'
            /// ```
            ///
            /// The patching process is tested in a real browser in crates/virtual-dom-rs/tests/diff_patch.rs
            #[derive(PartialEq)]
            pub enum Patch<'a> {
                /// Append a vector of child nodes to a parent node id.
                AppendChildren(NodeIdx, Vec<&'a VNode>),
                /// For a `node_i32`, remove all children besides the first `len`
                TruncateChildren(NodeIdx, usize),
                /// Replace a node with another node. This typically happens when a node's tag changes.
                /// ex: <div> becomes <span>
                Replace(NodeIdx, &'a VNode),
                /// Add attributes that the new node has that the old node does not
                AddAttributes(NodeIdx, HashMap<&'a str, &'a str>),
                /// Remove attributes that the old node had that the new node doesn't
                RemoveAttributes(NodeIdx, Vec<&'a str>),
                /// Change the text of a Text node.
                ChangeText(NodeIdx, &'a VText),
            }

            type NodeIdx = usize;

            impl<'a> Patch<'a> {
                /// Every Patch is meant to be applied to a specific node within the DOM. Get the
                /// index of the DOM node that this patch should apply to. DOM nodes are indexed
                /// depth first with the root node in the tree having index 0.
                pub fn node_idx(&self) -> usize {
                    match self {
                        Patch::AppendChildren(node_idx, _) => *node_idx,
                        Patch::TruncateChildren(node_idx, _) => *node_idx,
                        Patch::Replace(node_idx, _) => *node_idx,
                        Patch::AddAttributes(node_idx, _) => *node_idx,
                        Patch::RemoveAttributes(node_idx, _) => *node_idx,
                        Patch::ChangeText(node_idx, _) => *node_idx,
                    }
                }
            }

            /// Given two VNode's generate Patch's that would turn the old virtual node's
            /// real DOM node equivalent into the new VNode's real DOM node equivalent.
            pub fn diff_vnodes<'a>(old: &'a VNode, new: &'a VNode) -> Vec<Patch<'a>> {
                diff_recursive(&old, &new, &mut 0)
            }

            fn diff_recursive<'a, 'b>(
                old: &'a VNode,
                new: &'a VNode,
                cur_node_idx: &'b mut usize,
            ) -> Vec<Patch<'a>> {
                let mut patches = vec![];
                let mut replace = false;

                // Different enum variants, replace!
                // VNodes are of different types, and therefore will cause a re-render.
                // TODO: Handle previously-mounted children so they don't get re-mounted
                if mem::discriminant(old) != mem::discriminant(new) {
                    replace = true;
                }

                if let (VNode::Element(old_element), VNode::Element(new_element)) = (old, new) {
                    // Replace if there are different element tags
                    if old_element.tag != new_element.tag {
                        replace = true;
                    }

                    // Replace if two elements have different keys
                    // TODO: More robust key support. This is just an early stopgap to allow you to force replace
                    // an element... say if it's event changed. Just change the key name for now.
                    // In the future we want keys to be used to create a Patch::ReOrder to re-order siblings
                    if old_element.attrs.get("key").is_some()
                        && old_element.attrs.get("key") != new_element.attrs.get("key")
                    {
                        replace = true;
                    }
                }

                // Handle replacing of a node
                if replace {
                    patches.push(Patch::Replace(*cur_node_idx, &new));
                    if let VNode::Element(old_element_node) = old {
                        for child in old_element_node.children.iter() {
                            increment_node_idx_for_children(child, cur_node_idx);
                        }
                    }
                    return patches;
                }

                // The following comparison can only contain identical variants, other
                // cases have already been handled above by comparing variant
                // discriminants.
                match (old, new) {
                    // We're comparing two text nodes
                    (VNode::Text(old_text), VNode::Text(new_text)) => {
                        if old_text != new_text {
                            patches.push(Patch::ChangeText(*cur_node_idx, &new_text));
                        }
                    }

                    // We're comparing two element nodes
                    (VNode::Element(old_element), VNode::Element(new_element)) => {
                        let mut add_attributes: HashMap<&str, &str> = HashMap::new();
                        let mut remove_attributes: Vec<&str> = vec![];

                        // TODO: -> split out into func
                        for (new_attr_name, new_attr_val) in new_element.attrs.iter() {
                            match old_element.attrs.get(new_attr_name) {
                                Some(ref old_attr_val) => {
                                    if old_attr_val != &new_attr_val {
                                        add_attributes.insert(new_attr_name, new_attr_val);
                                    }
                                }
                                None => {
                                    add_attributes.insert(new_attr_name, new_attr_val);
                                }
                            };
                        }

                        // TODO: -> split out into func
                        for (old_attr_name, old_attr_val) in old_element.attrs.iter() {
                            if add_attributes.get(&old_attr_name[..]).is_some() {
                                continue;
                            };

                            match new_element.attrs.get(old_attr_name) {
                                Some(ref new_attr_val) => {
                                    if new_attr_val != &old_attr_val {
                                        remove_attributes.push(old_attr_name);
                                    }
                                }
                                None => {
                                    remove_attributes.push(old_attr_name);
                                }
                            };
                        }

                        if add_attributes.len() > 0 {
                            patches.push(Patch::AddAttributes(*cur_node_idx, add_attributes));
                        }
                        if remove_attributes.len() > 0 {
                            patches.push(Patch::RemoveAttributes(*cur_node_idx, remove_attributes));
                        }

                        let old_child_count = old_element.children.len();
                        let new_child_count = new_element.children.len();

                        if new_child_count > old_child_count {
                            let append_patch: Vec<&'a VNode> =
                                new_element.children[old_child_count..].iter().collect();
                            patches.push(Patch::AppendChildren(*cur_node_idx, append_patch))
                        }

                        if new_child_count < old_child_count {
                            patches.push(Patch::TruncateChildren(*cur_node_idx, new_child_count))
                        }

                        let min_count = min(old_child_count, new_child_count);
                        for index in 0..min_count {
                            *cur_node_idx = *cur_node_idx + 1;
                            let old_child = &old_element.children[index];
                            let new_child = &new_element.children[index];
                            patches.append(&mut diff_recursive(
                                &old_child,
                                &new_child,
                                cur_node_idx,
                            ))
                        }
                        if new_child_count < old_child_count {
                            for child in old_element.children[min_count..].iter() {
                                increment_node_idx_for_children(child, cur_node_idx);
                            }
                        }
                    }
                    (VNode::Text(_), VNode::Element(_)) | (VNode::Element(_), VNode::Text(_)) => {
                        unreachable!(
                            "Unequal variant discriminants should already have been handled"
                        );
                    }
                    _ => todo!("Diffing Not yet implemented for all node types"),
                };

                //    new_root.create_element()
                patches
            }

            fn increment_node_idx_for_children<'a, 'b>(
                old: &'a VNode,
                cur_node_idx: &'b mut usize,
            ) {
                *cur_node_idx += 1;
                if let VNode::Element(element_node) = old {
                    for child in element_node.children.iter() {
                        increment_node_idx_for_children(&child, cur_node_idx);
                    }
                }
            }

            // #[cfg(test)]
            // mod tests {
            //     use super::*;
            //     use crate::prelude::*;
            //     type VirtualNode = VNode;

            //     /// Test that we generate the right Vec<Patch> for some start and end virtual dom.
            //     pub struct DiffTestCase<'a> {
            //         // ex: "Patching root level nodes works"
            //         pub description: &'static str,
            //         // ex: html! { <div> </div> }
            //         pub old: VNode,
            //         // ex: html! { <strong> </strong> }
            //         pub new: VNode,
            //         // ex: vec![Patch::Replace(0, &html! { <strong></strong> })],
            //         pub expected: Vec<Patch<'a>>,
            //     }

            //     impl<'a> DiffTestCase<'a> {
            //         pub fn test(&self) {
            //             // ex: vec![Patch::Replace(0, &html! { <strong></strong> })],
            //             let patches = diff_vnodes(&self.old, &self.new);

            //             assert_eq!(patches, self.expected, "{}", self.description);
            //         }
            //     }
            //     use super::*;
            //     use crate::nodes::{VNode, VText};
            //     use std::collections::HashMap;

            //     #[test]
            //     fn replace_node() {
            //         DiffTestCase {
            //             description: "Replace the root if the tag changed",
            //             old: html! { <div> </div> },
            //             new: html! { <span> </span> },
            //             expected: vec![Patch::Replace(0, &html! { <span></span> })],
            //         }
            //         .test();
            //         DiffTestCase {
            //             description: "Replace a child node",
            //             old: html! { <div> <b></b> </div> },
            //             new: html! { <div> <strong></strong> </div> },
            //             expected: vec![Patch::Replace(1, &html! { <strong></strong> })],
            //         }
            //         .test();
            //         DiffTestCase {
            //             description: "Replace node with a child",
            //             old: html! { <div> <b>1</b> <b></b> </div> },
            //             new: html! { <div> <i>1</i> <i></i> </div>},
            //             expected: vec![
            //                 Patch::Replace(1, &html! { <i>1</i> }),
            //                 Patch::Replace(3, &html! { <i></i> }),
            //             ], //required to check correct index
            //         }
            //         .test();
            //     }

            //     #[test]
            //     fn add_children() {
            //         DiffTestCase {
            //             description: "Added a new node to the root node",
            //             old: html! { <div> <b></b> </div> },
            //             new: html! { <div> <b></b> <span></span> </div> },
            //             expected: vec![Patch::AppendChildren(0, vec![&html! { <span></span> }])],
            //         }
            //         .test();
            //     }

            //     #[test]
            //     fn remove_nodes() {
            //         DiffTestCase {
            //             description: "Remove all child nodes at and after child sibling index 1",
            //             old: html! { <div> <b></b> <span></span> </div> },
            //             new: html! { <div> </div> },
            //             expected: vec![Patch::TruncateChildren(0, 0)],
            //         }
            //         .test();
            //         DiffTestCase {
            //             description: "Remove a child and a grandchild node",
            //             old: html! {
            //             <div>
            //              <span>
            //                <b></b>
            //                // This `i` tag will get removed
            //                <i></i>
            //              </span>
            //              // This `strong` tag will get removed
            //              <strong></strong>
            //             </div> },
            //             new: html! {
            //             <div>
            //              <span>
            //               <b></b>
            //              </span>
            //             </div> },
            //             expected: vec![Patch::TruncateChildren(0, 1), Patch::TruncateChildren(1, 1)],
            //         }
            //         .test();
            //         DiffTestCase {
            //             description: "Removing child and change next node after parent",
            //             old: html! { <div> <b> <i></i> <i></i> </b> <b></b> </div> },
            //             new: html! { <div> <b> <i></i> </b> <i></i> </div>},
            //             expected: vec![
            //                 Patch::TruncateChildren(1, 1),
            //                 Patch::Replace(4, &html! { <i></i> }),
            //             ], //required to check correct index
            //         }
            //         .test();
            //     }

            //     #[test]
            //     fn add_attributes() {
            //         let mut attributes = HashMap::new();
            //         attributes.insert("id", "hello");

            //         DiffTestCase {
            //             old: html! { <div> </div> },
            //             new: html! { <div id="hello"> </div> },
            //             expected: vec![Patch::AddAttributes(0, attributes.clone())],
            //             description: "Add attributes",
            //         }
            //         .test();

            //         DiffTestCase {
            //             old: html! { <div id="foobar"> </div> },
            //             new: html! { <div id="hello"> </div> },
            //             expected: vec![Patch::AddAttributes(0, attributes)],
            //             description: "Change attribute",
            //         }
            //         .test();
            //     }

            //     #[test]
            //     fn remove_attributes() {
            //         DiffTestCase {
            //             old: html! { <div id="hey-there"></div> },
            //             new: html! { <div> </div> },
            //             expected: vec![Patch::RemoveAttributes(0, vec!["id"])],
            //             description: "Add attributes",
            //         }
            //         .test();
            //     }

            //     #[test]
            //     fn change_attribute() {
            //         let mut attributes = HashMap::new();
            //         attributes.insert("id", "changed");

            //         DiffTestCase {
            //             description: "Add attributes",
            //             old: html! { <div id="hey-there"></div> },
            //             new: html! { <div id="changed"> </div> },
            //             expected: vec![Patch::AddAttributes(0, attributes)],
            //         }
            //         .test();
            //     }

            //     #[test]
            //     fn replace_text_node() {
            //         DiffTestCase {
            //             description: "Replace text node",
            //             old: html! { Old },
            //             new: html! { New },
            //             expected: vec![Patch::ChangeText(0, &VText::new("New"))],
            //         }
            //         .test();
            //     }

            //     // Initially motivated by having two elements where all that changed was an event listener
            //     // because right now we don't patch event listeners. So.. until we have a solution
            //     // for that we can just give them different keys to force a replace.
            //     #[test]
            //     fn replace_if_different_keys() {
            //         DiffTestCase {
            //             description: "If two nodes have different keys always generate a full replace.",
            //             old: html! { <div key="1"> </div> },
            //             new: html! { <div key="2"> </div> },
            //             expected: vec![Patch::Replace(0, &html! {<div key="2"> </div>})],
            //         }
            //         .test()
            //     }

            //     //    // TODO: Key support
            //     //    #[test]
            //     //    fn reorder_chldren() {
            //     //        let mut attributes = HashMap::new();
            //     //        attributes.insert("class", "foo");
            //     //
            //     //        let old_children = vec![
            //     //            // old node 0
            //     //            html! { <div key="hello", id="same-id", style="",></div> },
            //     //            // removed
            //     //            html! { <div key="gets-removed",> { "This node gets removed"} </div>},
            //     //            // old node 2
            //     //            html! { <div key="world", class="changed-class",></div>},
            //     //            // removed
            //     //            html! { <div key="this-got-removed",> { "This node gets removed"} </div>},
            //     //        ];
            //     //
            //     //        let new_children = vec![
            //     //            html! { <div key="world", class="foo",></div> },
            //     //            html! { <div key="new",> </div>},
            //     //            html! { <div key="hello", id="same-id",></div>},
            //     //        ];
            //     //
            //     //        test(DiffTestCase {
            //     //            old: html! { <div> { old_children } </div> },
            //     //            new: html! { <div> { new_children } </div> },
            //     //            expected: vec![
            //     //                // TODO: Come up with the patch structure for keyed nodes..
            //     //                // keying should only work if all children have keys..
            //     //            ],
            //     //            description: "Add attributes",
            //     //        })
            //     //    }
            // }
        }
    }

    mod vcomponent {
        // -----------------------------------------------
        //  Allow debug/display adherent to the HTML spec
        // -----------------------------------------------

        impl fmt::Debug for VComponent {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // TODO: @JON Implement how components should be formatted when spit out to html
                // It probably can't be as straightforward as renderinng their VNodes
                // It _could_ be, but we can't really implement that directly
                // Instead, we should drop a vnode labeled with the component id/key

                // write!(
                //     f,
                //     "Element(<{}>, attrs: {:?}, children: {:?})",
                //     self.tag, self.attrs, self.children,
                // )
                Ok(())
            }
        }

        impl fmt::Display for VComponent {
            // Turn a VElement and all of it's children (recursively) into an HTML string
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // write!(f, "<{}", self.tag).unwrap();

                // for (attr, value) in self.attrs.iter() {
                //     write!(f, r#" {}="{}""#, attr, value)?;
                // }

                // write!(f, ">")?;

                // for child in self.children.iter() {
                //     write!(f, "{}", child.to_string())?;
                // }

                // if !crate::validation::is_self_closing(&self.tag) {
                //     write!(f, "</{}>", self.tag)?;
                // }

                Ok(())
            }
        }
    }
}

// pub mod iterables {
//     use super::*;

//     /// Used by the html! macro for all braced child nodes so that we can use any type
//     /// that implements Into<IterableNodes>
//     ///
//     /// html! { <div> { nodes } </div> }
//     ///
//     /// nodes can be a String .. VNode .. Vec<VNode> ... etc
//     pub struct IterableNodes(Vec<VNode>);

//     impl IterableNodes {
//         /// Retrieve the first node mutably
//         pub fn first(&mut self) -> &mut VNode {
//             self.0.first_mut().unwrap()
//         }

//         /// Retrieve the last node mutably
//         pub fn last(&mut self) -> &mut VNode {
//             self.0.last_mut().unwrap()
//         }
//     }

//     impl IntoIterator for IterableNodes {
//         type Item = VNode;
//         // TODO: Is this possible with an array [VNode] instead of a vec?
//         type IntoIter = ::std::vec::IntoIter<VNode>;

//         fn into_iter(self) -> Self::IntoIter {
//             self.0.into_iter()
//         }
//     }

//     impl From<VNode> for IterableNodes {
//         fn from(other: VNode) -> Self {
//             IterableNodes(vec![other])
//         }
//     }

//     impl From<Vec<VNode>> for IterableNodes {
//         fn from(other: Vec<VNode>) -> Self {
//             IterableNodes(other)
//         }
//     }
// }