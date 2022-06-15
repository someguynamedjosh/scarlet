use super::{Node, NodeChild};

pub fn collect_comma_list<'a, 'n>(list: &'a NodeChild<'n>) -> Vec<&'a Node<'n>> {
    if let NodeChild::Node(list) = list {
        if list.phrase == "multiple items" {
            assert_eq!(list.children.len(), 3);
            assert_eq!(list.children[1], NodeChild::Text(","));
            [
                collect_comma_list(&list.children[0]),
                vec![list.children[2].as_node()],
            ]
            .concat()
        } else {
            vec![list]
        }
    } else {
        vec![]
    }
}

pub fn create_comma_list<'n>(nodes: Vec<Node<'n>>) -> NodeChild<'n> {
    let mut nodes = nodes;
    if let Some(top) = nodes.pop() {
        if nodes.len() > 0 {
            let rest = create_comma_list(nodes);
            NodeChild::Node(Node {
                phrase: "multiple items",
                children: vec![rest, NodeChild::Text(","), NodeChild::Node(top)],
                ..Default::default()
            })
        } else {
            NodeChild::Node(top)
        }
    } else {
        NodeChild::Missing
    }
}
