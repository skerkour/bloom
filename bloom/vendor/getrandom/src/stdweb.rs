// Copyright 2018 Developers of the Rand project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use crate::Error;

extern crate std;
use std::thread_local;

use stdweb::js;

#[derive(Clone, Copy, PartialEq)]
enum RngSource {
    Browser,
    Node,
}

thread_local!(
    static RNG_SOURCE: Result<RngSource, Error> = getrandom_init();
);

pub(crate) fn getrandom_inner(dest: &mut [u8]) -> Result<(), Error> {
    RNG_SOURCE.with(|&source| getrandom_fill(source?, dest))
}

fn getrandom_init() -> Result<RngSource, Error> {
    if js! { return typeof self === "object"; } == true {
        // We are in a Browser or WebWorker
        let supported = js! { return typeof self.crypto === "object"; };
        if supported == true {
            Ok(RngSource::Browser)
        } else {
            Err(Error::WEB_CRYPTO)
        }
    } else {
        // We are in Node.js
        let supported = js! {
            try {
                require("crypto");
                return true;
            } catch(err) {
                return false;
            }
        };
        if supported == true {
            Ok(RngSource::Node)
        } else {
            Err(Error::NODE_CRYPTO)
        }
    }
}

fn getrandom_fill(source: RngSource, dest: &mut [u8]) -> Result<(), Error> {
    for chunk in dest.chunks_mut(65536) {
        let len = chunk.len() as u32;
        let ptr = chunk.as_mut_ptr() as i32;

        let success = js! {
            try {
                let array = new Uint8Array(@{ len });

                if @{ source == RngSource::Browser } {
                    self.crypto.getRandomValues(array);
                } else {
                    require("crypto").randomFillSync(array);
                }

                HEAPU8.set(array, @{ ptr });
                return true;
            } catch(err) {
                return false;
            }
        };

        if success != true {
            return match source {
                RngSource::Browser => Err(Error::WEB_GET_RANDOM_VALUES),
                RngSource::Node => Err(Error::NODE_RANDOM_FILL_SYNC),
            };
        }
    }
    Ok(())
}
