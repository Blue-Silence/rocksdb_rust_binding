#include <iostream>
#include <rocksdb_rust_binding/include/db.h>
#include <string>
#include "rocksdb/db.h"
#include "rocksdb/slice.h"
#include <rocksdb_rust_binding/src/lib.rs.h>
#include <filesystem>

std::unique_ptr<DB> open_default(rust::string path)
{
    rocksdb::DB *db = nullptr;
    rocksdb::Options options;
    options.create_if_missing = true;
    std::filesystem::create_directories(std::string{path});
    rocksdb::Status status = rocksdb::DB::Open(options, std::string{path}, &db);
    if (!status.ok()) {
        throw std::runtime_error(status.ToString());
    }
    //assert(status.ok());
    return std::unique_ptr<DB>(new DB{db});
}

DB::~DB()
{
    delete this->db;
}

using ROCKSDB_NAMESPACE::Slice;

void DB::Put(const uint8_t *key, size_t k_l, const uint8_t *value, size_t v_l) const
{
    rocksdb::DB *db = (rocksdb::DB *)this->db;
    rocksdb::Status s = db->Put(rocksdb::WriteOptions(), Slice{(const char *)key, k_l}, Slice{(const char *)value, v_l});
    if (!s.ok())
        throw std::runtime_error(s.ToString());
}

void DB::Delete(const uint8_t *key, size_t k_l) const
{
    rocksdb::DB *db = (rocksdb::DB *)this->db;
    rocksdb::Status s = db->Delete(rocksdb::WriteOptions(), Slice{(const char *)key, k_l});

    if (!s.ok())
        throw std::runtime_error(s.ToString());
}

std::unique_ptr<std::string> DB::Get(const uint8_t *key, size_t k_l) const
{
    rocksdb::DB *db = (rocksdb::DB *)this->db;
    auto value = new std::string;
    rocksdb::Status s = db->Get(rocksdb::ReadOptions(), Slice{(const char *)key, k_l}, value);

    if (!s.ok())
        throw std::runtime_error(s.ToString());
    return std::unique_ptr<std::string>(value);
}

std::unique_ptr<rocksdb::Iterator> DB::Prefix_Iter(const uint8_t *key, size_t k_l) const
{
    rocksdb::DB *db = (rocksdb::DB *)this->db;

    auto option = rocksdb::ReadOptions();
    rocksdb::Iterator *it = db->NewIterator(option);
    it->Seek(Slice{(const char *)key, k_l});
    return std::unique_ptr<rocksdb::Iterator>(it);
}

std::unique_ptr<rocksdb::Iterator> DB::Start_Iter() const
{
    rocksdb::DB *db = (rocksdb::DB *)this->db;

    auto option = rocksdb::ReadOptions();
    rocksdb::Iterator *it = db->NewIterator(option);
    it->SeekToFirst();
    return std::unique_ptr<rocksdb::Iterator>(it);
}

Pair Next(const rocksdb::Iterator *iter) 
{
    if (!iter->Valid())
        throw std::runtime_error("");
    Pair re = Pair{std::unique_ptr<std::string>(new std::string{std::move(iter->key().ToString())}), std::unique_ptr<std::string>(new std::string{std::move(iter->value().ToString())})};
    ((rocksdb::Iterator *)iter)->Next();
    return re;
}

