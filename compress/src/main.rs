#![feature(box_patterns, core_intrinsics, const_type_name, negative_impls)]
use std::convert::TryInto;
use std::mem::size_of;

#[derive(Debug)]
enum Node<T> {
    Leaf(T),
    Branch(Box<Node<T>>, Box<Node<T>>),
}

impl<T> Node<T> {
    pub fn get(&self) -> Option<&T> {
        if let Node::Leaf(t) = self {
            Some(t)
        } else {
            None
        }
    }
}

type Code = u32;
const CODE_TY: &'static str = std::intrinsics::type_name::<Code>();
type Symbol = u32;
const SYMBOL_TY: &'static str = std::intrinsics::type_name::<Symbol>();
// Prefix-free codes can't be compared for equality
struct PrefixFreeCode(Code);
impl !Eq for PrefixFreeCode {}
impl !PartialEq for PrefixFreeCode {}
type SymbolStream<'a, 'b> = &'a mut dyn Iterator<Item = &'b Symbol>;

#[derive(Debug)]
struct SymbolCount {
    node: Node<Symbol>,
    count: usize,
}

#[derive(Clone)]
struct Entry {
    symbol: Symbol,
    code: Code,
}

fn count_symbols(input: SymbolStream) -> Vec<SymbolCount> {
    let mut pairs = Vec::new();
    let input = input.cloned().collect::<Vec<Symbol>>();
    for i in input {
        let insert = pairs
            .iter_mut()
            .find(|p: &&mut SymbolCount| p.node.get().map(|&x| x) == i.try_into().ok())
            .map_or_else(
                || {
                    Some(SymbolCount {
                        node: Node::Leaf(i.try_into().unwrap()),
                        count: 1,
                    })
                },
                |p: &mut SymbolCount| {
                    p.count += 1;
                    None
                },
            );
        insert.map(|new_pair| pairs.push(new_pair));
    }
    pairs
}

fn build_tree(leaves: &mut Vec<SymbolCount>) {
    while leaves.len() != 1 {
        leaves.sort_by_key(|leaf| -(leaf.count as isize));
        let min = leaves.pop().unwrap();
        let second_min = leaves.pop().unwrap();
        let count = min.count + second_min.count;
        let node = Node::Branch(Box::new(min.node), Box::new(second_min.node));
        leaves.push(SymbolCount { node, count });
    }
}

fn assign_codes(root: Node<Symbol>, suffix: Option<Code>) -> Vec<Entry> {
    // All codes are prefixed with a 1 to disambiguate leading zeros
    // For example 0b0 and 0b00 are different codes
    // Adding a leading 1 helps disambiguate them since 0b10 != 0b100
    let suffix = suffix.unwrap_or(1);
    match root {
        Node::Leaf(t) => vec![Entry {
            symbol: t,
            code: suffix,
        }],
        Node::Branch(box left, box right) => {
            let left_entries = assign_codes(left, Some((suffix << 1) | 1));
            let right_entries = assign_codes(right, Some((suffix << 1) | 0));
            left_entries
                .iter()
                .chain(right_entries.iter())
                .cloned()
                .collect()
        },
    }
}

fn len(c: Code) -> usize {
    let leading_zeros = c.leading_zeros() as usize;
    (size_of::<Code>() * 8) - leading_zeros - 1
}

fn remove_prefix(c: Code) -> PrefixFreeCode {
    PrefixFreeCode(c & !(1 << len(c)))
}

fn compress(entries: &Vec<Entry>, input: SymbolStream) -> Vec<u8> {
    let mut output_stream = Vec::new();
    let num_bits = size_of::<Code>() * 8;
    let mut remaining_bits = num_bits;
    let mut current_word = 0;
    for symbol in input {
        let encoded = entries
            .iter()
            .find(|e| e.symbol == *symbol)
            .expect("Couldn't find entry for symbol")
            .code;
        let len = len(encoded);
        let prefix_free = remove_prefix(encoded);
        if remaining_bits < len {
            let second_size = len - remaining_bits;
            let first_part = prefix_free.0 >> second_size;
            current_word |= first_part;
            output_stream.push(current_word);
            current_word = 0;
            let second_part = prefix_free.0 << (num_bits - second_size);
            current_word |= second_part;
            remaining_bits = num_bits - second_size;
        } else if remaining_bits == len {
            current_word |= prefix_free.0;
            output_stream.push(current_word);
            current_word = 0;
            remaining_bits = num_bits;
        } else {
            remaining_bits -= len;
            current_word |= prefix_free.0 << remaining_bits;
        }
    }
    if remaining_bits != num_bits {
        output_stream.push(current_word);
    }
    output_stream
        .iter()
        .map(|x| x.to_le_bytes().iter().cloned().collect::<Vec<u8>>())
        .flatten()
        .collect()
}

fn main() {
    let exe = include_bytes!("../ferris.tim")
        .chunks(size_of::<Symbol>())
        .map(|x| Symbol::from_le_bytes(x.try_into().unwrap()))
        .collect::<Vec<Symbol>>();
    let mut symbol_counts = count_symbols(&mut exe.iter());
    build_tree(&mut symbol_counts);
    let mut entries = assign_codes(symbol_counts.pop().expect("No nodes in tree").node, None);
    assert!(
        symbol_counts.pop().is_none(),
        "Tree contained more than one root"
    );
    let output_stream = compress(&entries, &mut exe.iter());
    let num_symbols = exe.len();
    let header = [
        (num_symbols as u32).to_le_bytes(),
        (entries.len() as u32).to_le_bytes(),
    ]
    .concat();
    let codes = entries
        .iter()
        .map(|e| {
            remove_prefix(e.code)
                .0
                .to_le_bytes()
                .iter()
                .cloned()
                .collect::<Vec<u8>>()
        })
        .flatten()
        .collect::<Vec<u8>>();
    let symbols = entries
        .iter()
        .map(|e| e.symbol.to_le_bytes().iter().cloned().collect::<Vec<u8>>())
        .flatten()
        .collect::<Vec<u8>>();
    let zipped_file = header
        .iter()
        .chain(codes.iter())
        .chain(symbols.iter())
        .chain(output_stream.iter())
        .cloned()
        .collect::<Vec<u8>>();
    std::fs::write("ferris.tim.zip", zipped_file)
        .expect("Couldn't write compressed stream to file");
}
