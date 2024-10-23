use rocksdb_rust_binding as rocksdb;
fn main() {
    let db = rocksdb::DB::open_default("/dev/shm/db_test".to_string());
    println!("Re:{:?}",db.put(&"123".as_bytes().to_vec(), &"Okk".as_bytes().to_vec()));
    println!("Re:{:?}",db.get(&"123".as_bytes().to_vec()));
    println!("Re:{:?}",db.get(&"Empty".as_bytes().to_vec()));
    for i in 0..1000 {
        println!("Re:{:?}",db.put(&i.to_string().as_bytes().to_vec(), &(i*10).to_string().as_bytes().to_vec()));
    }

    let mut iter = rocksdb::DbIterator::prefix_iter(&db, &7.to_string().as_bytes().to_vec());
    for (k,v) in iter {
        println!("k:{:?}, v:{:?} number:: k:{}, v:{}", k, v, String::from_utf8(k.clone()).unwrap(), String::from_utf8(v.clone()).unwrap());
    }
}