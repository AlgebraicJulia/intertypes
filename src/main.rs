use lalrpop_util::lalrpop_mod;

mod cli;
mod syntax;

lalrpop_mod!(pub parser);

fn main() {
    cli::run();
}

#[test]
fn parse_path() {
    use lasso::Spur;
    use syntax::{ParserState, TypeExpr};
    let mut state = ParserState::default();
    let texpr = parser::TypeExprParser::new()
        .parse(&mut state, "banana.peel.opened")
        .unwrap();

    let TypeExpr::Path(sid, fields) = texpr else {
        panic!("failed to parse a path")
    };

    assert_eq!(sid.val, state.intern("banana"));

    assert_eq!(
        fields
            .into_iter()
            .map(|ss| { ss.val })
            .collect::<Vec<Spur>>(),
        vec![state.intern("peel"), state.intern("opened")]
    );
}

#[test]
fn parse_app() {
    use lasso::Spur;
    use syntax::{ParserState, TypeExpr};
    let mut state = ParserState::default();
    let texpr = parser::TypeExprParser::new()
        .parse(&mut state, "banana.peel[apple, banana]")
        .unwrap();

    let TypeExpr::App(ste, args) = texpr else {
        panic!("failed to parse an app")
    };

    let TypeExpr::Path(sid, fields) = ste.val else {
        panic!("failed to parse a path")
    };

    assert_eq!(sid.val, state.intern("banana"));

    assert_eq!(
        fields
            .into_iter()
            .map(|ss| { ss.val })
            .collect::<Vec<Spur>>(),
        vec![state.intern("peel")]
    );

    assert_eq!(args.len(), 2);
}

#[test]
fn parse_point() {
    use lasso::Spur;
    use syntax::{ParserState, TypeDef, TypeExpr};
    let mut state = ParserState::default();
    let point_def = "
        record Point {
            x: f64;
            y: f64;
        }
    ";
    let def = parser::TypeDefParser::new()
        .parse(&mut state, point_def)
        .unwrap();
}

#[test]
fn parse_graph() {
    use lasso::Spur;
    use syntax::{ParserState, TypeDef, TypeExpr};
    let mut state = ParserState::default();
    let def = parser::TypeDefParser::new()
        .parse(
            &mut state,
            "
            record Graph {
                Edge: fintype;
                Vertex: fintype;
                src: Edge -> Vertex;
                tgt: Edge -> Vertex;
            }
        ",
        )
        .unwrap();
}
