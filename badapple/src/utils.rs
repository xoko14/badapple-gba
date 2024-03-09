use nom::{Slice, number::streaming, error::ErrorKind};

pub struct Reader<'a>{
    pointer: usize,
    source: &'a [u8]
}

impl<'a> Reader<'a>{
    pub fn new(source: &'a[u8]) -> Self {
        Self{
            pointer: 0,
            source
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let res = self.source[self.pointer];
        self.pointer += 1;
        res
    }

    pub fn read_u16(&mut self) -> u16 {
        let end = self.pointer + 2;
        let res = nom_u16(self.source.slice(self.pointer..end));
        self.pointer = end;
        res
    }

    pub fn read_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        let end = self.pointer + 4;
        bytes.clone_from_slice(&self.source[self.pointer..end]);
        self.pointer = end;
        u32::from_le_bytes(bytes)
    }
}

fn nom_u8(data: &[u8]) -> u8{
    streaming::le_u8::<_, (_, ErrorKind)>(data).unwrap().1
}

fn nom_u16(data: &[u8]) -> u16{
    streaming::le_u16::<_, (_, ErrorKind)>(data).unwrap().1
}

pub struct FrameManager{
    width: u8,
    heigth: u8,
    current_x: u8,
    current_y: u8,
}

impl FrameManager {
    pub fn new(width: u8, heigth: u8) -> Self{
        Self{
            width,
            heigth,
            current_x: 0,
            current_y: 0
        }
    }

    pub fn get_pos(&mut self) -> (u8, u8){
        let x = self.current_x;
        let y = self.current_y;

        if x+1 >= self.width{
            self.current_x = 0;
            self.current_y +=1;
        }
        else {
            self.current_x +=1;
        }

        (x, y)
    }
}