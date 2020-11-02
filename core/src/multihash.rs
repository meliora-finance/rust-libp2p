// Copyright 2020 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use multihash::{Multihash as MultihashGeneric, U64};
use sha2::{Digest as ShaDigestTrait, Sha256};
use std::convert::TryFrom;

pub type Multihash = MultihashGeneric<U64>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Code {
    Identity = 0x00,
    Sha2_256 = 0x12,
}

impl Code {
    pub fn digest(self, input: &[u8]) -> Multihash {
        match self {
            Code::Identity => {
                self.wrap(&input)
            },
            Code::Sha2_256 => {
                let digest = Sha256::digest(input);
                self.wrap(&digest.as_ref())
            }
       }
   }

   pub fn wrap(self, digest: &[u8]) -> Multihash {
       Multihash::wrap(self.into(), digest).expect("Cannot fail as digest size is correct")
   }
}

impl From<Code> for u64 {
    fn from(code: Code) -> Self {
        code as _
    }
}

impl TryFrom<u64> for Code {
    type Error = ();

    fn try_from(code: u64) -> Result<Self, Self::Error> {
        match code {
            0x00 => Ok(Code::Identity),
            0x12 => Ok(Code::Sha2_256),
            _ => Err(())
        }
    }
}
