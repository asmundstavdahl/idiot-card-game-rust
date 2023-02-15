pub fn clear() -> String {
    "\x1B[2J\x1B[f".to_string()
}

pub fn bold(msg: &String) -> String {
    format!("\x1B[1m{}\x1B[0m", msg)
}

pub fn invert(msg: &String) -> String {
    format!("\x1B[7m{}\x1B[0m", msg)
}

pub fn error(msg: &String) -> String {
    invert(&bold(msg))
}
