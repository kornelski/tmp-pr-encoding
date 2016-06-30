// Copyright 2015-2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use handles::*;
use variant::*;
use super::*;

pub struct UserDefinedDecoder;

impl UserDefinedDecoder {
    pub fn new() -> VariantDecoder {
        VariantDecoder::UserDefined(UserDefinedDecoder)
    }

    pub fn max_utf16_buffer_length(&self, byte_length: usize) -> usize {
        byte_length
    }

    pub fn max_utf8_buffer_length(&self, byte_length: usize) -> usize {
        byte_length * 3
    }

    pub fn max_utf8_buffer_length_with_replacement(&self, byte_length: usize) -> usize {
        byte_length * 3
    }

    decoder_functions!({},
                       {},
                       {
                           if b < 0x80 {
                               // XXX optimize ASCII
                               destination_handle.write_ascii(b);
                               continue;
                           }
                           destination_handle.write_upper_bmp((b as usize + 0xF700usize) as u16);
                           continue;
                       },
                       self,
                       src_consumed,
                       dest,
                       b,
                       destination_handle,
                       _unread_handle,
                       check_space_bmp);
}

pub struct UserDefinedEncoder;

impl UserDefinedEncoder {
    pub fn new(encoding: &'static Encoding) -> Encoder {
        Encoder::new(encoding, VariantEncoder::UserDefined(UserDefinedEncoder))
    }

    pub fn max_buffer_length_from_utf16(&self, u16_length: usize) -> usize {
        u16_length
    }

    pub fn max_buffer_length_from_utf8(&self, byte_length: usize) -> usize {
        byte_length
    }

    encoder_functions!({},
                       {
                           if c <= '\u{7F}' {
                               // TODO optimize ASCII run
                               destination_handle.write_one(c as u8);
                               continue;
                           }
                           if c < '\u{F780}' || c > '\u{F7FF}' {
                               return (EncoderResult::Unmappable(c),
                                       unread_handle.consumed(),
                                       destination_handle.written());
                           }
                           destination_handle.write_one((c as usize - 0xF700usize) as u8);
                           continue;
                       },
                       self,
                       src_consumed,
                       source,
                       dest,
                       c,
                       destination_handle,
                       unread_handle,
                       check_space_one);
}

#[cfg(test)]
mod tests {
    use super::super::testing::*;
    use super::super::*;

    fn decode_x_user_defined(bytes: &[u8], expect: &str) {
        decode(X_USER_DEFINED, bytes, expect);
    }

    fn encode_x_user_defined(string: &str, expect: &[u8]) {
        encode(X_USER_DEFINED, string, expect);
    }

    #[test]
    fn test_x_user_defined_decode() {
        // ASCII
        decode_x_user_defined(b"\x61\x62", "\u{0061}\u{0062}");

        decode_x_user_defined(b"\x80\xFF", "\u{F780}\u{F7FF}");
    }

    #[test]
    fn test_x_user_defined_encode() {
        // ASCII
        encode_x_user_defined("\u{0061}\u{0062}", b"\x61\x62");

        encode_x_user_defined("\u{F780}\u{F7FF}", b"\x80\xFF");
        encode_x_user_defined("\u{F77F}\u{F800}", b"&#63359;&#63488;");
    }

}
