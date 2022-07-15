# You need the rust tool chain available, since the test harness still uses cargo during it's own build
FROM rust:1.62
WORKDIR /app

# Using git clone since this repo has an example app inside, but in theory any app with tests should work!
RUN git clone https://github.com/edude03/static-test-harness

# You have to set the INPUT_DIR envvar to where to get the tests from - IE, where to run `cargo test`
ENV INPUT_DIR /app/static-test-harness/example

# This will pull this binary from github and install it into the path
RUN cargo install --git https://github.com/edude03/static-test-harness

# Then, just run this binary :)
CMD ["static-test-harness"]
