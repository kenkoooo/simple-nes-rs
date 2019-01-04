pub trait CpuBus {
    fn read_word(&mut self, addr: u16) -> u16;

    fn read(&mut self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, data: u8);
}
