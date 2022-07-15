# Static test harness

Lets you run rust tests without the rust toolchain (for example, in a minimal docker container)

## Using it 

Run `INPUT_DIR=$PWD/example cargo run`

and you should get

```
Finished dev [unoptimized + debuginfo] target(s) in 0.67s
Running `target/debug/static-test-harness`

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
