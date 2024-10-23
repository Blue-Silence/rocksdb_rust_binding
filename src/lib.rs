#[cxx::bridge]
mod ffi {
    pub struct Pair {
        pub key: UniquePtr<CxxString>,
        pub value: UniquePtr<CxxString>,
    }
    unsafe extern "C++" {
        include!("rocksdb_rust_binding/include/db.h");

        type DB;
        fn open_default(path: String) -> Result<UniquePtr<DB>>;
        unsafe fn Get(self: &DB, key: *const u8, k_l: usize) -> Result<UniquePtr<CxxString>>;
        unsafe fn Put(
            self: &DB,
            key: *const u8,
            k_l: usize,
            value: *const u8,
            v_l: usize,
        ) -> Result<()>;
        unsafe fn Delete(self: &DB, key: *const u8, k_l: usize) -> Result<()>;

        type Iterator;
        unsafe fn Prefix_Iter(self: &DB, key: *const u8, k_l: usize) -> UniquePtr<Iterator>;
        unsafe fn Start_Iter(self: &DB) -> UniquePtr<Iterator>;
        unsafe fn Next(iter: *const Iterator) -> Result<Pair>;
    }
}

unsafe impl Sync for ffi::DB {}
unsafe impl Send for ffi::DB {}

mod wrapper {
    pub struct DB {
        db: cxx::UniquePtr<crate::ffi::DB>,
    }
    impl DB {
        pub fn open_default(path: String) -> Result<Self, cxx::Exception> {
            match crate::ffi::open_default(path) {
                Ok(db) => Ok(DB{db}),
                Err(e) => Err(e),
            }
        }

        fn put(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {
            //self.db.Put(key, value)
            unsafe {
                self.db
                    .Put(key.as_ptr(), key.len(), value.as_ptr(), value.len())
            }
        }

        pub fn put1(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put2(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put3(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put4(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put5(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put6(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put7(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put8(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put9(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put10(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put11(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}
        pub fn put12(&self, key: &Vec<u8>, value: &Vec<u8>) -> Result<(), cxx::Exception> {self.put(key, value)}

        pub fn get(&self, key: &Vec<u8>) -> Result<Option<Vec<u8>>, cxx::Exception> {
            let re = unsafe { self.db.Get(key.as_ptr(), key.len()) };
            match re {
                Ok(s) => Ok(Some(s.as_bytes().to_vec())),
                Err(e) => {
                    if e.what() == "NotFound: " {
                        Ok(None)
                    } else {
                        //println!("Get:a{}a", e.what());
                        Err(e)
                    }
                }
            }
        }

        pub fn delete(&self, key: &Vec<u8>) -> Result<(), cxx::Exception> {
            unsafe { self.db.Delete(key.as_ptr(), key.len()) }
        }
    }

    pub struct DbIterator {
        iter: cxx::UniquePtr<crate::ffi::Iterator>,
    }

    impl DbIterator {
        pub fn new(p: cxx::UniquePtr<crate::ffi::Iterator>) -> DbIterator {
            DbIterator { iter: p }
        }
        pub fn prefix_iter(db: &DB, key: &Vec<u8>) -> Self {
            DbIterator { iter : unsafe { db.db.Prefix_Iter(key.as_ptr(), key.len()) } }
        }
        pub fn start_iter(db: &DB) -> Self {
            DbIterator { iter : unsafe { db.db.Start_Iter() } }
        }
    }
    impl Iterator for DbIterator {
        type Item = (Vec<u8>, Vec<u8>);

        fn next(&mut self) -> Option<Self::Item> {
            let re = unsafe { crate::ffi::Next(self.iter.as_ref()?) };
            match re {
                Ok(v) => Some((v.key.as_bytes().to_vec(), v.value.as_bytes().to_vec())),
                Err(_) => None,
            }
        }
    }
}

pub use wrapper::*;
