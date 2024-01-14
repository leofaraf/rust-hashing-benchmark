# This is benchmark of rust hashing algorythms

The result on my laptop (Macbook Air M1):
```
Algorithm: bcrypt
Benchmark result: 237ms


Algorithm: unix
Benchmark result: 0ms


Algorithm: sha256 (DEPRECATED)
Benchmark result: 30ms


Algorithm: sha512
Benchmark result: 25ms


Algorithm: sha1
Benchmark result: 271ms


Algorithm: md5 (DEPRECATED)
Benchmark result: 2ms


Algorithm: bsdi (DEPRECATED)
Benchmark result: 50ms


Algorithm: argon2 (hash_password)
Benchmark result: 365ms
```

## Libraries:

- pwhash (bcrypt, unix, sha256, sha512, sha1, md5, bsdi)

- argon2
