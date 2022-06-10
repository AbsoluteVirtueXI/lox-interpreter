use lox_interpreter::lox::Lox;
fn main() {
    let mut lox = Lox { had_error: false };
    lox.main();
}
