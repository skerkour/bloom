use std::env;

pub fn main() {
    let arg = env::args().nth(1).unwrap();
    let code = qrcode::QrCode::new(arg.as_bytes()).unwrap();

    print!("{}", code.render().dark_color("\x1b[7m  \x1b[0m").light_color("\x1b[49m  \x1b[0m").build());
}
