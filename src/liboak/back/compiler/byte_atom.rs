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
use ast::ByteType;

pub struct ByteAtomCompiler
{
    byte_type: ByteType
}

impl ByteAtomCompiler
{
    pub fn recognizer(byte_type: ByteType) -> ByteAtomCompiler {
        ByteAtomCompiler {
            byte_type
        }
    }

    pub fn parser(byte_type: ByteType) -> ByteAtomCompiler {
        ByteAtomCompiler::recognizer(byte_type)
    }
}

impl CompileExpr for ByteAtomCompiler
{
    fn compile_expr<'a, 'b, 'c>(&self, context: &mut Context<'a, 'b, 'c>,
                                continuation: Continuation) -> RExpr
    {
        //let byte_ty = self.byte_type;
        let byte_ty = quote_item!(context.cx(), self.byte_type);
        let consume_byte = match self.byte_type {
            ByteType::U8 => quote_item!(context.cx(), oak_runtime::byte_stream::consume_u8),
            ByteType::U16=> quote_item!(context.cx(), oak_runtime::byte_stream::consume_u16)
        }; //TODO : rajouter le pattern d'any single char (+ match dans le code renvoyé)

        continuation
            .map_success(|success, failure| quote_expr!(context.cx(),
        if state.$consume_byte() {
          $success
        }
        else {
          state.error($byte_ty);
          $failure
        }
      ))
            .unwrap_success()
    }
}
