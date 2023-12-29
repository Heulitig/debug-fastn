#![allow(dead_code)]

#[derive(serde::Deserialize, Debug, PartialEq, Default, Clone, serde::Serialize)]
pub struct NodeData {
    pub name: String,
    pub node: ftd::node::Node,
    pub html_data: ftd::node::HTMLData,
    pub bag: indexmap::IndexMap<String, ftd::interpreter::Thing>,
    pub aliases: ftd::Map<String>,
    pub dummy_nodes: ftd::VecMap<ftd::node::DummyNode>,
    pub raw_nodes: ftd::Map<ftd::node::RawNode>,
    pub js: std::collections::HashSet<String>,
    pub css: std::collections::HashSet<String>,
    pub rive_data: Vec<ftd::executor::RiveData>,
}

impl NodeData {
    #[tracing::instrument(skip_all)]
    pub fn from_rt(rt: ftd::executor::RT) -> NodeData {
        let node = rt.main.to_node(rt.name.as_str(), &mut vec![]);
        let html_data = rt.html_data.to_html_data(rt.name.as_str());
        let raw_node =
            ftd::node::RawNode::from_element_constructors(rt.element_constructor, rt.name.as_str());
        let dummy_node =
            ftd::node::DummyNode::from_dummy_instructions(rt.dummy_instructions, rt.name.as_str());

        NodeData {
            name: rt.name.to_string(),
            node,
            html_data,
            bag: rt.bag,
            aliases: rt.aliases,
            dummy_nodes: dummy_node,
            raw_nodes: raw_node,
            js: rt.js,
            css: rt.css,
            rive_data: rt.rive_data,
        }
    }
}
