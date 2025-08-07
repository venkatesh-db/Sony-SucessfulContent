
use std::fs::OpenOptions;
use memmap2::Mmap;

pub fn run() {
    let file = OpenOptions::new().read(true).open("demo.txt");
    if let Ok(file) = file {
        unsafe {
            let mmap = Mmap::map(&file).unwrap();
            let contents = std::str::from_utf8(&mmap).unwrap();
            println!("Memory-mapped contents:\n{}", contents);
        }
    } else {
        println!("demo.txt not found");
    }
}
