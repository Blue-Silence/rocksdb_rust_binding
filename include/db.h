#pragma once
#include <memory>
#include <iostream>
#include "rocksdb/db.h"
#include "rust/cxx.h"
#include <string>


using Iterator = rocksdb::Iterator;

class Pair;
class ReGet;

class DB
{
public:
    const rocksdb::DB *db;
    ~DB();

    //string Get(string key) const;
    //void Put(string key, string value) const;
    void Put(const uint8_t* key, size_t k_l, const uint8_t* value, size_t v_l) const;
    void Delete(const uint8_t* key, size_t k_l) const;
    ReGet Get(const uint8_t* key, size_t k_l) const;
    std::unique_ptr<rocksdb::Iterator> Prefix_Iter(const uint8_t* key, size_t k_l) const;
    std::unique_ptr<rocksdb::Iterator> Start_Iter() const;
};

std::unique_ptr<DB> open_default(rust::string path, size_t thread_high, size_t thread_low);

Pair Next(const rocksdb::Iterator *iter);
