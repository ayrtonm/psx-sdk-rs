#![feature(min_const_generics, box_patterns)]
use std::mem::size_of;

#[derive(Debug)]
enum Node<T> {
    Leaf(T),
    Branch(Box<Node<T>>, Box<Node<T>>),
}

type Code = u16;
type Symbol = u8;
type Count = usize;
#[derive(Debug)]
struct SymbolCount {
    node: Node<Symbol>,
    count: Count,
}
#[derive(Clone)]
struct Entry {
    symbol: Symbol,
    code: Code,
}

fn count_symbols<const N: usize>(exe: &[Symbol; N]) -> Vec<SymbolCount> {
    let mut pairs = Vec::new();
    for symbol in 0..=255 {
        let count = exe.iter().filter(|&n| *n == symbol).count();
        if count > 0 {
            pairs.push(SymbolCount {
                node: Node::Leaf(symbol),
                count,
            });
        }
    }
    pairs
}

fn build_tree(leaves: &mut Vec<SymbolCount>) {
    while leaves.len() != 1 {
        leaves.sort_by_key(|l| -(l.count as isize));
        let a = leaves.pop().unwrap();
        let b = leaves.pop().unwrap();
        let combined_count = a.count + b.count;
        let branch = Node::Branch(Box::new(a.node), Box::new(b.node));
        leaves.push(SymbolCount {
            node: branch,
            count: combined_count,
        });
    }
}

fn make_code(root: Node<Symbol>, suffix: Option<Code>) -> Vec<Entry> {
    let suffix = suffix.unwrap_or(1);
    match root {
        Node::Leaf(t) => vec![Entry {
                symbol: t,
                code: suffix,
        }],
        Node::Branch(box left, box right) => {
            let left_codes = make_code(left, Some((suffix << 1) | 1));
            let right_codes = make_code(right, Some((suffix << 1) | 0));
            left_codes.iter().chain(right_codes.iter()).cloned().collect()
        },
    }
}

fn code_len(c: Code) -> usize {
    let leading_zeros = c.leading_zeros() as usize;
    (size_of::<Code>() * 8) - leading_zeros - 1
}

fn print_code(huffman_code: &Vec<Entry>) {
    for c in huffman_code {
        println!("{:2} {:#04x} {:#06x} {2:#018b}", code_len(c.code), c.symbol, c.code);
    }
}

fn main() {
    let exe = include_bytes!("../../rotating_square.psexe");
    let mut symbol_counts = count_symbols(exe);
    build_tree(&mut symbol_counts);
    let huffman_code = make_code(symbol_counts.pop().unwrap().node, None);
    let output_code = true;
    if output_code {
        let mut s = String::from("type Code = u16;\ntype Symbol = u8;\n");
        s.push_str(&format!("pub const CODES: [Code; {}] = [\n", huffman_code.len()));
        for c in &huffman_code {
            s.push_str(&format!("    {:#x},\n", c.code));
        }
        s.push_str("];\n");
        s.push_str(&format!("pub const SYMBOLS: [Symbol; {}] = [\n", huffman_code.len()));
        for c in &huffman_code {
            s.push_str(&format!("    {:#x},\n", c.symbol));
        }
        s.push_str("];");
        std::fs::write("huffman_code.rs", s).unwrap();
    }
    print_code(&huffman_code);
    let mut compressed_exe = Vec::new();
    let mut remaining_bits = size_of::<Code>() * 8;
    let mut compressed_data = 0;
    for b in exe {
        let encoded_b = huffman_code.iter().find(|c| c.symbol == *b).unwrap();
        if remaining_bits < code_len(encoded_b.code) {
            compressed_exe.push(compressed_data);
            compressed_data = 0;
            remaining_bits = size_of::<Code>() * 8;
        }
        remaining_bits -= code_len(encoded_b.code);
        compressed_data |= encoded_b.code << remaining_bits;
        println!("{:#06x}", encoded_b.code);
    }
    let compression_ratio = (size_of::<Code>() * compressed_exe.len()) as f32 / exe.len() as f32;
    println!("Compression ratio: {:?}", compression_ratio);
    let compressed_exe = compressed_exe.iter().map(|w| vec![(w & 0x00ff) as u8, (w >> 8) as u8]).flatten().collect::<Vec<u8>>();
    std::fs::write("../rotating_square.psexe.hzip", compressed_exe).unwrap();
}
