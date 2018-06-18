// Copyright 2014 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use middle::analysis::ast::*;
use partial::Partial::*;

pub struct TransformByteAtom<'a: 'c, 'b: 'a, 'c>
{
    grammar: &'c AGrammar<'a, 'b>,
    exprs: Vec<Expression>,
}

impl<'a, 'b, 'c> TransformByteAtom<'a, 'b, 'c>
{
    pub fn analyse(mut grammar: AGrammar<'a, 'b>) -> Partial<AGrammar<'a, 'b>> {
        if grammar.atom_kind == AtomKind::Byte {
            let e ={
                let mut analyser = TransformByteAtom {
                    grammar: &grammar,
                    exprs: grammar.exprs.clone()
                };
                for rule in &grammar.rules {
                    analyser.visit_expr(rule.expr_idx);
                }
                analyser.exprs
            };
            grammar.exprs = e;
        }
        Value(grammar)
    }
}

impl<'a, 'b, 'c> ExprByIndex for TransformByteAtom<'a, 'b, 'c>
{
    fn expr_by_index(&self, index: usize) -> Expression {
        self.grammar.expr_by_index(index).clone()
    }
}

impl<'a, 'b, 'c> Visitor<()> for TransformByteAtom<'a, 'b, 'c>
{
    unit_visitor_impl!(str_literal);
    unit_visitor_impl!(atom);
    unit_visitor_impl!(sequence);
    unit_visitor_impl!(choice);
    unit_visitor_impl!(byte_atom);

    fn visit_non_terminal_symbol(&mut self, this: usize, rule: Ident) {
        if rule.name.as_str() == "u8" {
            self.exprs[this] = Expression::ByteAtom(ByteType::U8);
        }
        else if rule.name.as_str() == "u16" {
            self.exprs[this] = Expression::ByteAtom(ByteType::U16);
        }
    }
}