use std::convert::TryInto;

#[derive(Copy,Clone, Debug)]
enum SegmentTypes {
    Ram,
    ConnectedDevices,
    Device,
    Unknown,
}
#[derive(Debug)]
struct Segment{
    segment_type: SegmentTypes,
    size: usize,
}

const MAX_DEVICES: usize = 64;
const MAX_DEVICE_SIZE: usize = 1024;

const SEGMENTS: [Segment; 3] = [
    Segment{segment_type: SegmentTypes::Ram, size: 0xFFFF },
    Segment{segment_type: SegmentTypes::ConnectedDevices, size: MAX_DEVICES*8},
    Segment{segment_type: SegmentTypes::Device, size: MAX_DEVICES*MAX_DEVICE_SIZE},

];

fn return_segment(address: usize) -> SegmentTypes{
    let mut i = 0;
    while i < SEGMENTS.len(){
        // println!("Start: {:}", SEGMENT_STARTS[i]);
        // println!("Size: {:}", SEGMENTS[i].size);
        // println!("address: {:}", address);
        if address >= SEGMENT_STARTS[i] && address < SEGMENT_STARTS[i] + SEGMENTS[i].size{
            return SEGMENTS[i].segment_type;
        }
        i += 1;
    }
    return SegmentTypes::Unknown;

}


const fn calculate_start(index: usize) -> usize{
    let mut sum = 0;
    let mut i = 0;
    while i<index{
        sum += SEGMENTS[i].size;
        i+=1;
    }
    return sum;
}

const SEGMENT_STARTS: [usize; SEGMENTS.len()] = [
    calculate_start(0),
    calculate_start(1),
    calculate_start(2),
];

trait Device{
    fn get_type(&self) -> u64;
    fn get_bytes(&self, address: usize, num_bytes: usize) -> Vec<u8>;
    fn set_bytes(&mut self, address: usize, num_bytes: usize, bytes: Vec<u8>);
}

struct Display{
    state: [u64;64],
}
impl Device for Display{
    fn get_type(&self) -> u64 {
        return 1;
    }

    fn get_bytes(&self, address: usize, num_bytes: usize) -> Vec<u8> {
        return self.state[address/8].to_le_bytes()[(address%8)..][..num_bytes].to_vec();
    }

    fn set_bytes(&mut self, address: usize, num_bytes: usize, bytes: Vec<u8>) {
        self.state[address/8] = u64::from_le_bytes(bytes[..num_bytes].try_into().unwrap());
    }
}

pub struct Memory {
    ram: Vec<u8>,
    devices: Vec<Box<dyn Device>>,
}

impl Memory{
    pub fn new(initial_bytes: Vec<u8>)-> Self{

        const RAM_INDEX: usize = 0; //TODO: make this smarter

        assert!(matches!(SEGMENTS[RAM_INDEX].segment_type,SegmentTypes::Ram));
        let mut ram = vec![0; SEGMENTS[RAM_INDEX].size];
        ram.splice(0..initial_bytes.len(), initial_bytes);
        let memory = Self{
            ram,
            devices: vec![]
        };
        return memory;
    }

    pub fn get_ram_length(&self) -> usize{
        return self.ram.len();
    }

    pub fn get_bytes(&self, address: usize, num_bytes: usize) -> Vec<u8> {
        // println!("segment: {:?}", return_segment(address));
        // println!("address: {:}", address);
        match return_segment(address){
            SegmentTypes::Ram => {
                return self.ram[address..][..num_bytes].to_vec();
            }
            SegmentTypes::ConnectedDevices => {
                return self.devices[address/MAX_DEVICE_SIZE].get_type().to_le_bytes()[..num_bytes].to_vec();
            }
            SegmentTypes::Device => {
                return self.devices[address/MAX_DEVICE_SIZE].get_bytes(address%MAX_DEVICE_SIZE, num_bytes)
            }
            SegmentTypes::Unknown => {
                return 0xCAFEBEEDEADBEEFu64.to_le_bytes()[..num_bytes].to_vec();
            }
        }
    }

    pub fn set_bytes(&mut self, address: usize, num_bytes: usize, bytes: Vec<u8>) {
        match return_segment(address) {
            SegmentTypes::Ram => {
                for i in 0..num_bytes {
                    self.ram[address + i] = bytes[i];
                }
            }
            SegmentTypes::ConnectedDevices => {}
            SegmentTypes::Device => {}
            SegmentTypes::Unknown => {}
        }
    }


}