// Copyright 2018 Marin Dupas (Sorbonne Université)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use back::compiler::*;
use ast::Bytes;

pub struct ByteLiteralCompiler
{
    literal: Bytes
}

impl ByteLiteralCompiler
{
    pub fn recognizer(literal: Bytes) -> ByteLiteralCompiler {
        ByteLiteralCompiler {
            literal: literal
        }
    }

    pub fn parser(literal: Bytes) -> ByteLiteralCompiler {
        ByteLiteralCompiler::recognizer(literal)
    }
}

impl CompileExpr for ByteLiteralCompiler
{
    fn compile_expr<'a, 'b, 'c>(&self, context: &mut Context<'a, 'b, 'c>,
                                continuation: Continuation) -> RExpr
    {
        let lit = self.literal.as_str();//TODO correct
        continuation
            .map_success(|success, failure| quote_expr!(context.cx(),
        if state.consume_prefix($lit) {
          $success
        }
        else {
          state.error($lit);
          $failure
        }
      ))
            .unwrap_success()
    }
}
