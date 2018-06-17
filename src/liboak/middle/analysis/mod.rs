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

use front::ast::FGrammar;
use middle::analysis::ast::*;
use middle::analysis::duplicate::*;
use middle::analysis::undeclared_rule::*;
use middle::analysis::undeclared_action::*;
use middle::analysis::well_formedness::*;
use middle::analysis::attribute::*;
use middle::analysis::transform_byte_atom::*;


use rust;
use middle::analysis::ast::AGrammar;
use syntax::ast::TyKind::Path;
use syntax::ast::ItemKind::Ty;

mod duplicate;
mod undeclared_rule;
mod undeclared_action;
mod well_formedness;
mod attribute;
mod transform_byte_atom;
pub mod ast;

pub fn analyse<'a, 'b>(cx: &'a ExtCtxt<'b>, fgrammar: FGrammar) -> Partial<AGrammar<'a, 'b>> {
  let grammar = AGrammar::new(cx, fgrammar.name, fgrammar.exprs, fgrammar.exprs_info);
  let frust_items = fgrammar.rust_items;
  let fattributes = fgrammar.attributes;
  rule_duplicate(grammar, fgrammar.rules)
  .and_then(|grammar| rust_functions_duplicate(grammar, frust_items))
  .and_then(|grammar| extract_stream_type(grammar)) //TODO : it simply work like that ???
  .and_then(|grammar| extract_atom_kind(grammar))
  .and_then(|grammar| TransformByteAtom::analyse(grammar))
  .and_then(|grammar| UndeclaredRule::analyse(grammar))
  .and_then(|grammar| UndeclaredAction::analyse(grammar))
  .and_then(|grammar| WellFormedness::analyse(grammar))
  .and_then(|grammar| decorate_with_attributes(grammar, fattributes))
}

/// Modify the default Stream type in the grammar if the user redefined it in its item list.
fn extract_stream_type<'a, 'b>(mut grammar: AGrammar<'a, 'b>)
                               -> Partial<AGrammar<'a, 'b>>
{
  let mut stream_redefined = false;
  {
    let stream_alias =
      grammar.rust_items.iter().find(|item| {
        match &item.node {
          &rust::ItemKind::Ty(_,_) => {
            &*item.ident.name.as_str() == "Stream"
          }
          _ => false
        }
      });

    if let Some(ty) = stream_alias {
      grammar.stream_alias = ty.clone();
      stream_redefined = true;
    }
  }
  if !stream_redefined {
    grammar.rust_items.push(grammar.stream_alias.clone());
  }
  Partial::Value(grammar)
}

/// Extract the AtomKind choose by the user
fn extract_atom_kind<'a, 'b>(mut grammar: AGrammar<'a, 'b>)
                             -> Partial<AGrammar<'a, 'b>>
{
  let stream_alias = grammar.stream_alias.clone();

  let ty = match stream_alias.node {
    Ty(ref ty,_) => ty,
    _ => unreachable!("Type of `stream_alias` check in `extract_atom_kind`.")
  };
  let str_atom_kind = match ty.node {
    Path(_, ref path) => path.segments[path.segments.len() - 1].ident.name.as_str(),
    _ => {
      grammar.span_err(ty.span, format!("Stream must be a simple type."));
      return Partial::Nothing
    }
  };
  if str_atom_kind == "StrStream" {
    grammar.atom_kind = AtomKind::Char;
  } else if str_atom_kind == "ByteStream"{
    grammar.atom_kind = AtomKind::Byte;
  } else {
    grammar.span_err(ty.span,format!("Only streams supported are StrStream and ByteStream."));
    return Partial::Nothing
  }
  Partial::Value(grammar)
}
