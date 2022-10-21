use super::*;

fn build_codes(root: Rc<Option<TreeNode<u8>>>) -> Vec<BitVec> {
    let mut codes: Vec<BitVec> = vec![BitVec::new(); 256];
    let mut empty_prefix = BitVec::new();
    walk_tree(root, &mut empty_prefix, &mut codes);
    codes
}

pub(crate) fn get_codes(input: &[u8]) -> Vec<BitVec> {
    let mut freqs = vec![0 as FreqType; 256];
    // Считаем кол-во разных байтов
    input.iter().for_each(|&e| freqs[e as usize] += 1);
    let tree = build_tree(&freqs);
    build_codes(tree)
}

pub(crate) fn serialize_codes(codes: &[BitVec]) -> BitVec {
    let mut res = BitVec::new();
    for c in codes {
        res.push_u8(c.len() as u8);
        res.concat(c);
    }
    res
}

pub(crate) fn deserialize_codes(source: &BitVec) -> Vec<BitVec> {
    let mut offset = 0;
    let mut codes = vec![BitVec::new(); 256];
    for i in 0..=255 {
        let size = source.get_range_u8(offset);
        offset += 8;
        codes[i] = source.get_range(offset, size as usize);
        offset += size as usize;
    }
    codes
}

pub(crate) fn build_tree(freqs: &[FreqType]) -> Rc<Option<TreeNode<u8>>> {
    let empty_node = TreeNode::<u8>::empty_node_ref();

    println!("Freq array: {:?}", freqs);
    // Создаем по ноде для каждого байта (частота встречаемости, байт)(нода байта)
    let mut freqs_nodes: BTreeMap<(FreqType, u16), Rc<Option<TreeNode<u8>>>> = freqs
        .iter()
        .enumerate()
        .filter(|(_, &freq)| freq != 0)
        .map(|(i, &freq)| {
            (
                (freq, i as u16),
                Rc::new(Some(TreeNode::<u8>::new(
                    i as u8,
                    empty_node.clone(),
                    empty_node.clone(),
                ))),
            )
        })
        .collect();

    // 
    while freqs_nodes.len() > 1 {
        let ((freq1, key), node1) = freqs_nodes.pop_first().unwrap();
        let ((freq2, _), node2) = freqs_nodes.pop_first().unwrap();

        let summing_node = Rc::new(Some(TreeNode::<u8>::new(0, node1, node2)));
        freqs_nodes.insert((freq1 + freq2, key), summing_node);
    }
    freqs_nodes.pop_first().unwrap().1
}

pub(crate) fn walk_tree(
    root: Rc<Option<TreeNode<u8>>>,
    prefix: &mut BitVec,
    codes: &mut Vec<BitVec>,
) {
    if let Some(ref root) = root.as_ref() {
        if root.is_leaf() {
            codes[root.data as usize] = prefix.clone();
        }
        if root.left.is_some() {
            prefix.push_back(0);
            walk_tree(root.left.clone(), prefix, codes);
            prefix.pop_back();
        }
        if root.right.is_some() {
            prefix.push_back(1);
            walk_tree(root.right.clone(), prefix, codes);
            prefix.pop_back();
        }
    }
}