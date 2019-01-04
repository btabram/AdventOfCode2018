use std::fs;

type ErrorHolder = Box<std::error::Error>;
type Data = Vec<usize>;

#[derive(Debug)]
struct Node {
    value: usize,
    metadata_count: usize,
    metadata: Vec<usize>,
    summed_metadata: usize,
    child_count: usize,
    children: Vec<Node>,
}

impl Node {
    fn new(child_count: usize, metadata_count: usize) -> Node {
        Node {
            value: 0,
            metadata: vec![],
            metadata_count,
            summed_metadata: 0,
            child_count,
            children: vec![],
        }
    }

    fn get_metadata(&self) -> usize {
        self.metadata.iter().sum()
    }

    fn get_summed_metadata(&self) -> usize {
        self.get_metadata() + self.children.iter().map(|n| n.summed_metadata).sum::<usize>()
    }

    fn get_value(&self) -> usize {
        let mut value = 0usize;
        // If the node has children then the value is the sum of the values
        // of the child nodes identified by using the metadata entries as
        // indexes for the children
        if self.child_count > 0 {
            for &m in &self.metadata {
                // The task indexes from 1 not 0
                let m_index = m - 1;
                if self.children.len() > m_index {
                    value += self.children[m_index].value;
                }
            }
        }
        // If the node doens't have children then the value is just the sum of
        // the metadata
        else {
            value = self.get_metadata();
        }
        value
    }
}

fn s_to_i(s: &str) -> usize {
    s.to_string().trim().parse().expect("Failed to parse char as usize")
}

fn read_node(data: &Data, index: usize) -> (usize, Node) {
    let mut i = index;

    // Parse the initial node information
    let mut node = Node::new(data[i], data[i+1]);
    i += 2;

    // Recursively parse any children
    for _ in 0..node.child_count {
        match read_node(data, i) {
            (new_i, n) => {
                i = new_i;
                node.children.push(n);
            },
        }
    }
    assert!(node.child_count == node.children.len());

    // Finally read in any metadata
    for _ in 0..node.metadata_count {
        node.metadata.push(data[i]);
        i += 1;
    }
    assert!(node.metadata_count == node.metadata.len());

    // Work out the total summed metadata for this node and all children
    node.summed_metadata = node.get_summed_metadata();

    // Calculate the value of this node
    node.value = node.get_value();

    println!("{:?}\n", node);
    (i, node)
}

fn main() -> Result<(), ErrorHolder> {
    let input = fs::read_to_string("input.txt")?;
    let data: Data = input.split(' ').map(s_to_i).collect();

    let (_, root) = read_node(&data, 0);

    println!("The sum of all metadata is {}", root.summed_metadata);
    println!("The value of the root node is {}", root.value);

    Ok(())
}

