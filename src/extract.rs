use std::iter::Peekable;

use proc_macro2::{Delimiter, TokenNode, TokenStream, TokenTreeIter};

use super::Asm;

fn node_to_string(node: TokenNode) -> String {
    match node {
        TokenNode::Term(t) => t.as_str().to_string(),
        TokenNode::Op(c, _) => c.to_string(),
        TokenNode::Literal(lit) => lit.to_string(),
        _ => panic!("unexpected"),
    }
}

fn get_next(iter: &mut Peekable<TokenTreeIter>) -> String {
    let start = node_to_string(iter.next().expect(&format!("expected more")).kind);
    if start == "%" {
        start + &get_next(iter)
    } else {
        start
    }
}

fn expect(iter: &mut Peekable<TokenTreeIter>, token: &str) {
    let term = get_next(iter);
    assert_eq!(term, token);
}

fn get_body(iter: &mut Peekable<TokenTreeIter>) -> Vec<String> {
    // println!("body: {:#?}", iter);
    let mut results = Vec::new();
    let mut current = String::new();
    let mut comma = false;
    for token in iter {
        let val = node_to_string(token.kind);
        if val == "%" || val == "$" {
            current.push_str(&val);
            continue;
        }
        if val == "," {
            comma = true;
            current.push_str(", ");
            continue;
        }
        if comma {
            current.push_str(&val);
            comma = false;
        } else {
            if !current.is_empty() {
                results.push(current.clone());
                current.clear();
            }
            current.push_str(&val);
            current.push_str(" ");
            comma = true;
        }
    }
    if !current.is_empty() {
        results.push(current.clone());
    }
    results
}

fn expect_body(iter: &mut Peekable<TokenTreeIter>) -> Vec<String> {
    if let TokenNode::Group(d, stream) = iter.next().unwrap().kind {
        let mut stream = stream.into_iter().peekable();
        assert_eq!(d, Delimiter::Brace);
        get_body(&mut stream)
    } else {
        panic!("body was missing");
    }
}

pub fn extract_impl(name: String, asm: &mut Peekable<TokenTreeIter>) -> Asm {
    let next = asm.next().expect("expected more").kind;
    let (ret, body) = match next {
        TokenNode::Op('-', _) => {
            expect(asm, ">");
            let ret = get_next(asm);
            (Some(ret), expect_body(asm))
        }
        TokenNode::Group(d, stream) => {
            assert_eq!(d, Delimiter::Brace);
            let mut stream = stream.into_iter().peekable();
            (None, get_body(&mut stream))
        }
        _ => panic!("body was missing!"),
    };

    Asm { name, ret, body }
}

pub fn extract_asm(asm: TokenStream) {
    let asm = &mut asm.into_iter().peekable();
    let target = "\"".to_string() + &::std::env::var("TARGET").unwrap() + "\"";

    while asm.peek().is_some() {
        let impl_target = get_next(asm);
        expect(asm, "fn");
        let name = get_next(asm);
        let asm = extract_impl(name, asm);
        if impl_target == target {
            asm.generate();
        }
    }
}
