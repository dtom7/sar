### Guidelines for end to end (integration) testing

Minimun Rust version is `1.63.0`

Integration tests must be run sequentially, parallel run is not supported

    cargo test --package sar --test integration_test -- --test-threads 1

To see println! & eprintln! messages whie running integration tests, run with `nocapture` option

    cargo test --package sar --test integration_test -- --test-threads 1 --nocapture

In `tests/resources` folder, place the UTF-8 encoded text files to be used for integration testing.

 - `original` files contain text before searching and replacing

 - `expected` files contain text after searching and replacing

one `original` file can have multiple `expected` files depending on the type of tests

`test_run` folder is deleted before and after running integration tests

`original` files in `tests/resources` folder are copied into sub folders inside the `test_run` folder before running integration tests and these files are renamed as `actual` files

`test_run` (folder)
 - `dir1` (folder)
   - `dir11` (folder)
     - `actual-1` (file)
     - `actual-2` (file)
     - `...`      (file)
 - `dir2` (folder)
   - `actual-1` (file)
   - `actual-2` (file)
   - `...`      (file)
- `actual-1` (file)
- `actual-2` (file)
- `...` (file)

integration tests are run on `actual` files residing in `test_run` folder

towards the the end of an integration test,
`expected` and `actual` files can be compared using the function: `assert_results`