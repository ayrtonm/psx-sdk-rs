#![feature(box_patterns)]
#[derive(Debug)]
enum Node<T> {
    Leaf(T),
    Branch(Box<Node<T>>, Box<Node<T>>),
}

#[derive(Debug, Clone)]
struct HuffmanEncoding {
    code: Code,
    symbol: Symbol,
}
type Code = u16;
type Symbol = u8;

fn mk_code(node: Node<Symbol>, suffix: Code) -> Vec<HuffmanEncoding> {
    match node {
        Node::Leaf(t) => vec![HuffmanEncoding {
            code: suffix,
            symbol: t,
        }],
        Node::Branch(box left, box right) => {
            let left_codes = mk_code(left, (suffix << 1) | 0);
            let right_codes = mk_code(right, (suffix << 1) | 1);
            left_codes.iter().chain(right_codes.iter()).cloned().collect()
        },
    }
}

fn code_len(c: Code) -> usize {
    let leading_zeros = c.leading_zeros() as usize;
    (std::mem::size_of::<Code>() * 8) - leading_zeros
}

fn main() {
    let exe = include_bytes!("../../rotating_square.psexe");
    let mut pairs = Vec::new();
    for symbol in 0..=255 {
        let count = exe.iter().filter(|&n| *n == symbol as Symbol).count();
        if count > 0 {
            pairs.push((count, Node::Leaf(symbol)));
        }
    }

    while pairs.len() != 1 {
        pairs.sort_by_key(|wi| -(wi.0 as isize));
        let a = pairs.pop().unwrap();
        let b = pairs.pop().unwrap();
        let combined_count = a.0 + b.0;
        let c = Node::Branch(Box::new(a.1), Box::new(b.1));
        pairs.push((combined_count, c));
    }

    let mut code = mk_code(pairs.pop().unwrap().1, 1);
    code.sort_by_key(|ci| ci.code);
    let mut compressed_exe = Vec::new();
    let mut remaining_bytes = 16;
    let mut compressed_data = 0;
    for b in exe {
        let encoded_b = code.iter().find(|c| c.symbol == *b).unwrap();
        if remaining_bytes >= code_len(encoded_b.code) {
            remaining_bytes -= code_len(encoded_b.code);
            compressed_data |= encoded_b.code << remaining_bytes;
        } else {
            compressed_exe.push(compressed_data);
            remaining_bytes = 16;
            compressed_data = 0;
        }
    }
    let compression_ratio = compressed_exe.len() as f32 / (exe.len() >> 1) as f32;
    let mut writeable_data: Vec<u8> = Vec::new();
    for data in compressed_exe {
        writeable_data.push((data & 0x00ff) as u8);
        writeable_data.push((data >> 8) as u8);
    }
    let mut s = String::new();
    s.push_str("let v = [\n");
    for c in &code {
        s.push_str(&format!("    ({:#04x?}, {:#x}),\n", c.symbol, c.code));
    }
    s.push_str("];");
    std::fs::write("../rotating_square.psexe.hf", writeable_data).unwrap();
    std::fs::write("../rotating_square_code.hf", s).unwrap();
    println!("Compression ratio: {:?}", compression_ratio);
}
