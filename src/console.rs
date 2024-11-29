pub struct Console;

impl Console {
    fn private(&self, message: &str) {
        println!("Private: {}", message);
    }
    pub fn log(&self, message: &str) {
        println!("{}", message);
    }

    pub fn error(&self, message: &str) {
        eprintln!("{}", message);
    }

    pub fn debug(&self, message: &str) {
        println!("{}", message);
    }

    pub fn clear(&self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}
#[allow(non_upper_case_globals)]
pub static console: Console = Console;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_log() {
        console.log("Hello, JS?");
    }

    #[test]
    fn test_console_error() {
        console.error("Hello, JS?");
    }

    #[test]
    fn test_console_debug() {
        console.debug("Hello, JS?");
    }

    #[test]
    fn test_console_clear() {
        console.clear();
    }
}