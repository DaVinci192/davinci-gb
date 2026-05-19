const DMA_SRC_HI: u16 = 0xFF51;
const DMA_SRC_LO: u16 = 0xFF52;

const DMA_DST_HI: u16 = 0xFF53;
const DMA_DST_LO: u16 = 0xFF54;

const DMA_LEN: u16 = 0xFF55;

pub struct VramDma {
    src_hi: u8,
    src_lo: u8,

    dst_hi: u8,
    dst_lo: u8,

    dma_len: u8,
}

impl VramDma {
    pub fn new() -> Self {
        Self {
            src_hi: 0xFF,
            src_lo: 0xFF,

            dst_hi: 0xFF,
            dst_lo: 0xFF,

            dma_len: 0xFF,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            DMA_SRC_HI => 0xFF, // w
            DMA_SRC_LO => 0xFF, // w
            DMA_DST_HI => 0xFF, // w
            DMA_DST_LO => 0xFF, // w
            DMA_LEN => self.dma_len,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, address: u16, val: u8) {
        match address {
            DMA_SRC_HI => self.src_hi = val,
            DMA_SRC_LO => self.src_lo = val,
            DMA_DST_HI => self.dst_hi = val,
            DMA_DST_LO => self.dst_lo = val,
            DMA_LEN => self.dma_len = val,
            _ => unreachable!(),
        }       
    }
}