# rcloc
A port of the cloc cli tool in rust 

For now you need to specify a directory. 

```cargo run --release -- /home/archie/retrieval-augmented-generation/src
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/rcloc /home/archie/retrieval-augmented-generation/src`
------------------------------------------------------------
Language        files        blank      comment         code
------------------------------------------------------------
Python             21          114          150         1363
------------------------------------------------------------
SUM:                                                    1627
------------------------------------------------------------
```

## TODO 

Support other languages *only Python currently
Add unit tests 
CICD 
