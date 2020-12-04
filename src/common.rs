
#[derive(Copy, Clone)]
pub struct CorrelationId {
    raw: [u8; 6]
}

#[derive(Copy, Clone)]
pub struct NodeId {
    raw: [u8; 4]
}

impl CorrelationId {
    pub fn create(data: [u8; 6]) -> CorrelationId {
        CorrelationId { raw: data }
    }
    
    pub fn to_bytes(&self) -> [u8;6] {
        self.raw
    }
}

impl NodeId {
    pub fn create(data: [u8; 4]) -> NodeId {
        NodeId { raw: data }
    }
}

pub fn create_u16(data: [u8; 2]) -> u16 {
    ((data[0] as u16) << 8) | data[1] as u16
}