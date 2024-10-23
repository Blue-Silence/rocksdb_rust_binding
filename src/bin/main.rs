use rocksdb_rust_binding::DB;


fn main() {
    let db = DB::open_default("/dev/shm/db_test".to_string());
    println!("Re:{:?}",db.put(&"123".as_bytes().to_vec(), &"Okk".as_bytes().to_vec()));
    println!("Re:{:?}",db.get(&"123".as_bytes().to_vec()));
    println!("Re:{:?}",db.get(&"Empty".as_bytes().to_vec()));
}