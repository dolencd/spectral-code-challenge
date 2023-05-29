# Spectral code challenge

The easy way to go about this challenge was to make a Yarn workspace with Typescript, then use nice-grpc and probably Express to keep it simple. Then some tests using Jest would have ensured that it works as expected.

Instead I went with Rust. I've been learning it for quite a while and this felt like a good excuse to continue after the pause that came with moving to The Netherlands.

In many ways it went really well. The project works as expected and I used quite a few libraries and features that I hadn't used before.

## Running the project

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Install system dependencies for Tonic, the gRPC library](https://github.com/hyperium/tonic)
3. Navigate to the root of the project.
4. Start the data provider by running `cargo run -p data-provider`
5. In a separate terminal window, start the data provider by running `cargo run -p api-server`
6. Open a browser window to `http://localhost:3000`
7. Profit!

## Corners cut

Unlike with JavaScript/TypeScript I don't have thousands of hours of Rust development behind me, so some things took more time than I'd like them to. That mixed with some unexpected things to do this weekend meant that I had to cut some corners. I do intend to go back and do it properly next weekend or the week after. It is still a great excuse project to practice Rust.

### Tests

You'll notice that there are no tests in the entire project. Normally I'd make sure to have automated tests for all the logic, but here I decided to skip them. Rust's type system pretty much guarantees that the code will run, so the main thing to test is whether it runs correctly. In this case, however, there isn't much logic that can fail in a way.

That said there are still a few places that could definitely use some tests, but I haven't gotten to them yet.

### Project structure

With all the rewriting and reformatting of code the structure of the project is not as clear and clean as I'd like it to be. This is something I might actually get to before you see the code. I'll remove this in that case.

### Reading data

Initially I wanted to read and process the data during compile time. The processing required ended up making this very complicated, so I went with just including the raw CSV file and processing it at runtime.

The ideal way of reading data in this case would be to read and process everything during compile time, then have the function return an async iterator over that array. This could then easily either be collected into a Vector or streamed in chunks or piece by piece.

### Streaming data

As mentioned in the email, an actual device would stream data (either line by line or in batches). This would also make sense when sending a large amount of data.

With the reading being handled by an async iterator, the next step to do that would be to make response of the gRPC method be a stream of elements or arrays of elements. After that I'd just do the same thing on the HTTP API side as well and update the frontend to handle this.

## Deployment

Recently I've found [Shuttle](https://www.shuttle.rs/) which takes an interesting approach to IaC and I wanted to try it out. Something for next weekend.

Either that or Terraform and AWS.

## CI Pipeline

It's generally good to have some kind of automated formatting, testing and linting even on solo projects. I haven't set up a pipeline for Rust yet, but it's probably not much different than JS/TS. I might do it at some point.
