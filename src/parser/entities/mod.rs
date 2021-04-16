use pest::iterators::Pair;

pub mod actions;
pub mod keys;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"] // relative to src
struct GrammarParser;

pub(crate) trait Parse {
    fn parse(pair: Pair<Rule>) -> Self;
}
