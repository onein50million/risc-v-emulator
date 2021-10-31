mod format;
mod opcodes;
mod processor;
mod memory;

fn main() {
    let code = std::fs::read("starship-os.bin").unwrap();
    let mut processor = processor::Processor::new(code);


    // let mut time_sum = 0;
    // let mut loops = 0;
    loop {
        // let operation_start_time = std::time::Instant::now();
        match processor.process(){
            processor::ProcesserResult::Break => break,
            processor::ProcesserResult::Continue => {}
        }
        // let time_elapsed = operation_start_time.elapsed().as_nanos() as u64;
        // time_sum += time_elapsed;
        // loops += 1;
    }
    // println!("Average Time: {:?}", (time_sum as f64) / loops as f64);


}
