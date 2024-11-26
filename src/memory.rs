pub trait Addressable {
    fn read(&self, addr: u16) -> Option<u8>;
    fn write(&mut self, addr: u16, value: u8) -> bool;

    // fn read2(&self, addr: u16) -> Option<u16> {
    //     let x0 = self.read(addr);
    //     let x1 = self.read(addr+1);

    //     Ok(Some(x0 as u16) | Some(x1 as u16) << 8);

    // }

    fn read2(&self, addr: u16) -> Option<u16> {
        let x0 = self.read(addr);
        let x1 = self.read(addr + 1);
        println!("addr={:?}", addr);
        println!("x0={:?}", x0);
        println!("x1={:?}", x1);
        println!("x11111={:?}", (Some(3).unwrap() | Some(0).unwrap() << 8));

        Some((x0.unwrap() as u16) | ((x1.unwrap() as u16) << 8))
    }

    fn write2(&mut self, addr: u16, value: u16) -> bool {
        let lower = value & 0xff;
        let upper = (value & 0xff00) >> 8;

        self.write(addr, lower as u8);
        self.write(addr + 1, upper as u8)
    }

    fn copy(&mut self, from: u16, to: u16, n: usize) -> bool {
        for i in 0..n {
            if let Some(x) = self.read(from + (i as u16)) {
                if !self.write(to + (i as u16), x) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

pub struct LinearMemory {
    bytes: Vec<u8>,
    size: usize,
}

impl LinearMemory {
    pub fn new(n: usize) -> Self {
        Self {
            bytes: vec![0; n],
            size: n,
        }
    }
}

impl Addressable for LinearMemory {
    fn read(&self, addr: u16) -> Option<u8> {
        // if addr < self.size {
        //     Some(self.bytes[addr])
        // }
        self.bytes.get(addr as usize).copied()
    }

    fn write(&mut self, addr: u16, value: u8) -> bool {
        if addr < self.size as u16 {
            self.bytes[addr as usize] = value;
            true
        } else {
            false
        }
    }
}
