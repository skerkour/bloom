use auxiliary_macro::hidden_repr;
use pin_project::pin_project;

#[pin_project]
#[hidden_repr(packed)]
struct S {
    //~^ ERROR may not be used on #[repr(packed)] types
    #[cfg(any())]
    #[pin]
    f: u32,
    #[cfg(not(any()))]
    #[pin]
    f: u8,
}

fn main() {}
