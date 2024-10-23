use rust_cpp::wrapper;


fn main() {
    let db = wrapper::DB::open_default("/dev/shm/db_test".to_string());
    println!("Re:{:?}",db.put(&"123".as_bytes().to_vec(), &"Okk".as_bytes().to_vec()));
    println!("Re:{:?}",db.get(&"123".as_bytes().to_vec()));
    println!("Re:{:?}",db.get(&"Empty".as_bytes().to_vec()));
}