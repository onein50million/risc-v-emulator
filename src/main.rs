
pub mod format;
pub mod opcodes;
pub mod processor;
pub mod memory;

fn main() {
    let processor = processor::Processor::new(vec![]);
    // for i in 0xFFFF..0xFFFFFF{
    //     println!("{:?} @ {:X}", processor.memory.get_bytes(i, std::mem::size_of_val(&i)),i);
    //     // std::thread::sleep(std::time::Duration::from_millis(10));
    // }

    println!("{:?}", processor.get_bytes(0xFF,1000000).len());
    processor.memory.get_device(1);


}