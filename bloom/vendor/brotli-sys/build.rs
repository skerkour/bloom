extern crate cc;

use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    if !Path::new("brotli/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    let src = env::current_dir().unwrap();
    println!("cargo:include={}", src.join("brotli/include").display());

    cc::Build::new()
        .include("brotli/include")
        .warnings(false)
        .file("brotli/common/dictionary.c")
        .file("brotli/dec/bit_reader.c")
        .file("brotli/dec/decode.c")
        .file("brotli/dec/huffman.c")
        .file("brotli/dec/state.c")
        .file("brotli/enc/backward_references.c")
        .file("brotli/enc/backward_references_hq.c")
        .file("brotli/enc/bit_cost.c")
        .file("brotli/enc/block_splitter.c")
        .file("brotli/enc/brotli_bit_stream.c")
        .file("brotli/enc/cluster.c")
        .file("brotli/enc/compress_fragment.c")
        .file("brotli/enc/compress_fragment_two_pass.c")
        .file("brotli/enc/dictionary_hash.c")
        .file("brotli/enc/encode.c")
        .file("brotli/enc/entropy_encode.c")
        .file("brotli/enc/histogram.c")
        .file("brotli/enc/literal_cost.c")
        .file("brotli/enc/memory.c")
        .file("brotli/enc/metablock.c")
        .file("brotli/enc/static_dict.c")
        .file("brotli/enc/utf8_util.c")
        .compile("libbrotli.a");
}
