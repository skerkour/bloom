#![cfg(feature = "derive")]
#![allow(dead_code)]

use bytemuck::{Zeroable, Pod, TransparentWrapper};

#[derive(Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct Test {
  a: u16,
  b: u16,
}

#[derive(TransparentWrapper)]
#[repr(transparent)]
struct TransparentSingle {
  a: u16,
}

#[derive(TransparentWrapper)]
#[repr(transparent)]
#[transparent(u16)]
struct TransparentWithZeroSized {
  a: u16,
  b: ()
}