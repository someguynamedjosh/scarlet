use super::structure::{Module, Token};

pub trait Parser<'s, 't: 's, O>: FnMut(&'s [Token<'t>]) -> Option<(&'s [Token<'t>], O)> {}

impl<'s, 't: 's, O, T> Parser<'s, 't, O> for T where
    T: FnMut(&'s [Token<'t>]) -> Option<(&'s [Token<'t>], O)>
{
}

fn tag<'s, 't: 's>(expect: &'s str) -> impl Parser<'s, 't, ()> {
    move |input: &'s [Token<'t>]| {
        if !input.is_empty() && input[0].is_symbol(expect) {
            Some((&input[1..], ()))
        } else {
            None
        }
    }
}

struct BracketGroup<'s, 't> {
    start: &'s Token<'t>,
    body: &'s [Token<'t>],
    end: &'s Token<'t>,
}

fn is_open_bracket(token: &Token) -> bool {
    match token {
        Token::Symbol("{") | Token::Symbol("[") | Token::Symbol("(") => true,
        _ => false,
    }
}

fn is_close_bracket(token: &Token) -> bool {
    match token {
        Token::Symbol("}") | Token::Symbol("]") | Token::Symbol(")") => true,
        _ => false,
    }
}

fn bracket_group<'s, 't: 's>() -> impl Parser<'s, 't, BracketGroup<'s, 't>> {
    move |input: &'s [Token<'t>]| {
        if input.is_empty() || !is_open_bracket(&input[0]) {
            return None;
        }
        let mut bracket_count = 1;
        let mut remainder = &input[1..];
        while bracket_count > 0 {
            if remainder.is_empty() {
                return None;
            } else if is_open_bracket(&remainder[0]) {
                bracket_count += 1;
            } else if is_close_bracket(&remainder[0]) {
                bracket_count -= 1;
            }
            remainder = &remainder[1..];
        }
        let captured_len = input.len() - remainder.len();
        debug_assert!(captured_len >= 2);
        let captured = &input[..captured_len];
        let start = &captured[0];
        let body = &captured[1..captured_len - 1];
        let end = &captured[captured_len - 1];
        Some((remainder, BracketGroup { start, body, end }))
    }
}

fn curly_bracket_group<'s, 't: 's>() -> impl Parser<'s, 't, BracketGroup<'s, 't>> {
    |input| {
        let (input, brackets) = bracket_group()(input)?;
        if brackets.start != &Token::Symbol("{") || brackets.end != &Token::Symbol("}") {
            return None;
        }
        Some((input, brackets))
    }
}

fn curly_bracket_compound_with_label<'s, 't: 's>(label: &'static str) -> Box<Transformer<'s, 't>> {
    Box::new(move |input: &'s [Token<'t>]| {
        let (input, _) = tag(label)(input)?;
        let (input, brackets) = curly_bracket_group()(input)?;
        let body = brackets.body.to_owned();
        let token = Token::Compound { label, body };
        Some((input, vec![token]))
    })
}

type Transformer<'s, 't> = dyn Parser<'s, 't, Vec<Token<'t>>> + 's;

fn compound<'s, 't: 's>() -> Box<Transformer<'s, 't>> {
    Box::new(move |input: &'s [Token<'t>]| {
        let (input, _) = tag("compound")(input)?;
        let (input, brackets) = bracket_group()(input)?;
        if brackets.start != &Token::Symbol("{") || brackets.end != &Token::Symbol("}") {
            return None;
        }
        if brackets.body.len() == 0 {
            return None;
        }
        let mut body = brackets.body.to_owned();
        let label = body.remove(0);
        if let Token::Symbol(label) = label {
            let token = Token::Compound { body, label };
            Some((input, vec![token]))
        } else {
            None
        }
    })
}

fn builtin<'s, 't: 's>() -> Box<Transformer<'s, 't>> {
    let label = "builtin";
    Box::new(move |input: &'s [Token<'t>]| {
        let (input, _) = tag(label)(input)?;
        let (input, brackets) = curly_bracket_group()(input)?;
        let mut body = brackets.body.to_owned();
        if body.len() < 1 {
            return None;
        }
        body[0].wrap("builtin_name");
        let token = Token::Compound { label, body };
        Some((input, vec![token]))
    })
}

fn structt<'s, 't: 's>() -> Box<Transformer<'s, 't>> {
    curly_bracket_compound_with_label("struct")
}

fn parentheses<'s, 't: 's>() -> Box<Transformer<'s, 't>> {
    Box::new(move |input: &'s [Token<'t>]| {
        let (input, brackets) = bracket_group()(input)?;
        if brackets.start != &Token::Symbol("(") || brackets.end != &Token::Symbol(")") {
            return None;
        }
        let body = brackets.body.to_owned();
        let label = "parenthesis_group";
        let token = Token::Compound { body, label };
        Some((input, vec![token]))
    })
}

fn field<'s, 't: 's>() -> Box<Transformer<'s, 't>> {
    Box::new(move |input: &'s [Token<'t>]| {
        if input.len() < 3 {
            return None;
        }
        if &input[1] != &Token::Symbol(".") {
            return None;
        }
        let token = Token::Compound {
            label: "field",
            body: vec![input[0].clone(), input[2].clone()],
        };
        Some((&input[3..], vec![token]))
    })
}

fn substitute_shorthand<'s, 't: 's>() -> Box<Transformer<'s, 't>> {
    Box::new(move |input: &'s [Token<'t>]| {
        if input.len() < 2 {
            return None;
        }
        if !input[1].is_compound("parenthesis_group") {
            return None;
        }
        let mut base = input[0].clone();
        base.wrap("substitution_target");
        let mut args = input[1].clone();
        args.wrap("substitutions");
        let token = Token::Compound {
            label: "substitute",
            body: vec![base, args],
        };
        Some((&input[2..], vec![token]))
    })
}

fn matchh<'s, 't: 's>() -> Box<Transformer<'s, 't>> {
    Box::new(move |input: &'s [Token<'t>]| {
        if input.len() < 4 {
            return None;
        }
        if !input[1].is_symbol("match") {
            return None;
        }
        let base = input[0].clone();
        let (input, cases) = curly_bracket_group()(input)?;
        let cases = Token::Compound {
            label: "cases",
            body: cases.body.to_owned(),
        };
        let token = Token::Compound {
            label: "match",
            body: vec![base, cases],
        };
        Some((input, vec![token]))
    })
}

fn make_transformers<'s, 't: 's>(precedence: u8) -> Vec<Box<Transformer<'s, 't>>> {
    match precedence {
        0 => vec![
            compound::<'s, 't>(),
            builtin::<'s, 't>(),
            structt::<'s, 't>(),
            parentheses::<'s, 't>(),
        ],
        10 => vec![
            field::<'s, 't>(),
            substitute_shorthand::<'s, 't>(),
            matchh::<'s, 't>(),
        ],
        _ => vec![],
    }
}

fn transform_sequence<'s, 't>(
    input: &'s [Token<'t>],
    output: &mut Vec<Token<'t>>,
    transformers: &mut [Box<Transformer<'s, 't>>],
) {
    if input.len() == 0 {
        return;
    }
    for transformer in &mut *transformers {
        if let Some((rest, mut tokens)) = transformer(input) {
            output.append(&mut tokens);
            return transform_sequence(rest, output, transformers);
        }
    }
    output.push(input[0].clone());
    transform_sequence(&input[1..], output, transformers);
}

fn transform_compounds<'t>(input: &mut [Token<'t>], precedence: u8) {
    for token in input {
        match token {
            Token::Compound { body, .. } => {
                let owned_body = std::mem::take(body);
                let mut transformers = make_transformers(precedence);
                transform_sequence(&owned_body[..], body, &mut transformers[..]);
            }
            Token::Symbol(..) => continue,
        }
    }
}

pub fn full_transform(input: Vec<Token>) -> Vec<Token> {
    let mut input = input;
    for precedence in 0..=255 {
        let mut transformers = make_transformers(precedence);
        let mut result = Vec::new();
        transform_sequence(&input[..], &mut result, &mut transformers[..]);
        drop(transformers);
        transform_compounds(&mut result, precedence);
        input = result;
    }
    input
}

pub fn transform_module(module: Module) -> Module {
    let self_content = full_transform(module.self_content);
    let mut children = Vec::new();
    for (name, child) in module.children {
        let child = transform_module(child);
        children.push((name, child));
    }
    Module {
        self_content,
        children,
    }
}
