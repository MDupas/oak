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

//! Implementation of `Stream` for `Vec<u8>` type. It implements all traits required by `CharStream`.
pub use std::ops::Range;
pub use syntex_pos::Span;
use stream::*;
use std::cmp::Ordering;
use make_span;

impl Stream for Vec<u8>
{
    type Output = ByteStream;
    fn stream<>(self) -> ByteStream {
        ByteStream::new(self)
    }
}

/// Represents a stream from a `Vec<u8>`. It implements all traits required by `CharStream`.
#[derive(Clone)]
pub struct ByteStream
{
    bytes: Vec<u8>,
    offset: usize
}

impl ByteStream
{
    fn new(bytes: Vec<u8>) -> ByteStream {
        ByteStream {
            bytes,
            offset: 0
        }
    }

    #[inline(always)]
    fn assert_same_raw_data(&self, other: &ByteStream) {
        debug_assert!(self.bytes.as_ptr() == other.bytes.as_ptr(),
                      "Operations between two streams are only defined when they share the same raw data.");
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn current_byte(&self) -> &u8 {
        &self.bytes[self.offset]
    }

    // TODO : Repérer quand un caractère ne prend pas de place ?
    pub fn line_column(&self) -> (usize, usize) {
        let mut line = 0;
        let mut column = 0;
        let u8_carriage = '\n' as u8;
        for x in 0..self.offset {
            if self.bytes[x] == u8_carriage {
                column += 1;
                line = 0;
            }
            else {
                line += 1;
            }
        }
        (line, column)
    }
}

impl Iterator for ByteStream
{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.has_next() {
            let current = self.current_byte().clone();
            self.offset += 1;
            Some(current)
        } else {
            None
        }
    }
}

impl PartialEq for ByteStream
{
    fn eq(&self, other: &Self) -> bool {
        self.assert_same_raw_data(other);
        self.offset == other.offset
    }
}

impl Eq for ByteStream {}

impl PartialOrd for ByteStream
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.assert_same_raw_data(other);
        self.offset.partial_cmp(&other.offset)
    }
}

impl Ord for ByteStream
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.assert_same_raw_data(other);
        self.offset.cmp(&other.offset)
    }
}


impl Location for ByteStream
{
    fn location(&self) -> String {
        let (line, column) = self.line_column();
        format!("{}:{}", line, column)
    }
}

impl CodeSnippet for ByteStream
{
    fn code_snippet(&self, _len_hint: usize) -> String {
        if ! self.has_next() {
            String::from("<end-of-file>")
        } else {
            self.current_byte().to_string()
        }
    }
}

impl ConsumePrefix<&'static str> for ByteStream
{
    fn consume_prefix(&mut self, prefix: &'static str) -> bool {
        let mut count = 0;
        for byte in prefix.bytes() {
            if self.bytes[self.offset + count].clone() == byte {
                count += 1;
            } else {
                break;
            }
        }
        if prefix.bytes().len() == count {
            self.offset += count;
            true
        } else {
            false
        }
    }
}

impl ConsumePrefix<UnisgnedInt> for ByteStream
{//TODO : verif que tout est ok
    fn consume_prefix(&mut self, prefix: UnsignedInt) -> bool {
        let mut count = 0;
        for byte in prefix.into() {
            if self.bytes[self.offset + count].clone() == byte {
                count += 1;
            } else {
                break;
            }
        }
        if prefix.bytes().len() == count {
            self.offset += count;
            true
        } else {
            false
        }
    }
}

impl HasNext for ByteStream
{
    fn has_next(&self) -> bool {
        self.offset < self.bytes.len()
    }
}

impl StreamSpan for Range<ByteStream>
{
    type Output = Span;
    fn stream_span(&self) -> Self::Output {
        make_span(
            self.start.offset,
            self.end.offset)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn consume_prefix_test<'a>(stream: ByteStream, prefix: &'static str,
                               prefix_match: bool, next_byte: Option<u8>)
    {
        let mut s2 = stream.clone();
        assert_eq!(s2.consume_prefix(prefix), prefix_match);
        assert_eq!(s2.next(), next_byte);
    }

    #[test]
    fn test_consume_prefix() {
        let s1 = ByteStream::new("abc".bytes());
        consume_prefix_test(s1, "abc", true, None);
        consume_prefix_test(s1, "ab", true, Some('c'));
        consume_prefix_test(s1, "", true, Some('a'));
        consume_prefix_test(s1, "ac", false, Some('a'));
        consume_prefix_test(s1, "z", false, Some('a'));
    }
}