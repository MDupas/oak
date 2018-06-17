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
use std::ops::IndexMut;

pub struct TransformByteAtom<'a: 'c, 'b: 'a, 'c>
{
    grammar: &'c AGrammar<'a, 'b>,
    exprs: Vec<Expression>,
}

impl<'a, 'b, 'c> TransformByteAtom<'a, 'b, 'c>
{
    pub fn analyse(mut grammar: AGrammar<'a, 'b>) -> Partial<AGrammar<'a, 'b>> { //TODO : créer un nouveau fichier dans lequel on va visiter et tout
        let  e ={               // et récupérer l'ancien undeclared_rule
            let mut analyser = TransformByteAtom {
                grammar: *grammar,
                exprs: grammar.exprs.clone()
            };
            for rule in &grammar.rules {
                analyser.visit_expr(rule.expr_idx);
            }
            analyser.exprs
        };
        grammar.exprs = e;
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
    fn visit_non_terminal_symbol(&mut self, this: usize, rule: Ident) {
        if self.grammar.atom_kind == AtomKind::Byte {
            if rule.name.as_str() == "u8" {
                /*
                let mut newGrammar = self.grammar.clone();
                newGrammar.exprs[this] = Expression::ByteAtom(ByteType::U8);
                self.grammar = newGrammar;
                */
                /*
                let mut new_grammar = self.grammar.clone();
                let mut new_exprs = new_grammar.exprs.clone();
                new_exprs[this] = Expression::ByteAtom(ByteType::U8);
                new_grammar.exprs = new_exprs;
                self.grammar = new_grammar;
                */
//        let mut g = &mut self.grammar;
//        let mut e = &mut g.exprs;
//        let mut a = &mut e[this];
//        *a = Expression::ByteAtom(ByteType::U8);
                if let Some(elem) = self.grammar.exprs.get_mut(1) {
                    *elem = Expression::ByteAtom(ByteType::U8);
                }
//        self.grammar.exprs
            }
                else if rule.name.as_str() == "u16" {
                    self.grammar.exprs[this] = Expression::ByteAtom(ByteType::U16);
                }
        }
            else {
                let contains_key = self.grammar.rules.iter()
                    .find(|r| r.ident() == rule).is_some();
                if !contains_key {
                    self.grammar.expr_err(
                        this,
                        format!("Undeclared rule `{}`.", rule)
                    );
                }
            }
    }

    unit_visitor_impl!(byte_atom);
}