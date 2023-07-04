pub mod uart;
pub trait UartDev {
    fn config(&self);
    fn read_char(&self)->char;
    fn write_char(&mut self, c:char);
    fn write_string(&mut self, mgs:&str);
}