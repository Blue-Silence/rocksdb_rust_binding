use rocksdb_rust_binding as rocksdb;



use std::thread::*;
use std::time::SystemTime;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = std::env::args().collect();
    let arg_1 = args[1].as_str();
    let arg_2 = args[2].as_str();
    let max_parallel: usize = arg_1.parse().unwrap();
    let max_iter: usize = arg_2.parse().unwrap();
    let total = max_parallel * max_iter;

    let db = std::sync::Arc::new(rocksdb::DB::open_default("/dev/shm/db_test".to_string()).unwrap());


    let mut dir_ns = vec![];
    let mut file_ns = vec![];
    for i in 0..total {
        dir_ns.push(format!("/dir{}", i).to_string());
        file_ns.push(format!("/file_{}", i).to_string().as_bytes().to_vec());
    }


    let t1 = SystemTime::now();
    scope(|s| {
        for i in 0..max_parallel {
            let db = db.clone();
            let file_ns = &file_ns;
            let dir_ns = &dir_ns;
            s.spawn(move || {
                let t1 = SystemTime::now();
                for j in 0..max_iter {
                    let _ = db.put(&dir_ns[i * max_iter + j].as_bytes().to_vec(), &file_ns[i * max_iter + j]);
                }
                let t2 = SystemTime::now();
                println!("t: {} ms", t2.duration_since(t1).unwrap().as_millis());
            });
        }
    });
    let t2 = SystemTime::now();
    println!("hello from the main thread");
    println!(
        "file create: {}; time: {} ms; average: {} op/s",
        total,
        t2.duration_since(t1).unwrap().as_millis(),
        (total as u64) / t2.duration_since(t1).unwrap().as_secs()
    );
}
