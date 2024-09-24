use core::fmt::Write;
struct SerialPort {
    data: u16,
    int_en: u16,
    fifo: u16,
    line: u16,
    modem: u16, // idk what modem does but its in the spec ig? - Caz
    line_sts: u16,
}

const COM1: u16 = 0x3F8;
const COM2: u16 = 0x2F8;

impl SerialPort {
    const LINE_READY: u8 = 1 << 0;
    const LINE_EMPTY: u8 = 1 << 5;

    const fn new_port(port: u16) -> Self {
        Self {
            data: port,
            int_en: port + 1,
            fifo: port + 2,
            line: port + 3,
            modem: port + 4,
            line_sts: port + 5,
        }
    }
    fn init_serial(&mut self) {
        unsafe {}
    }
}
