---
title: "Rust Web Development Tutorial: REST API"
description: "A tutorial on how to create a REST API in Rust with Actix web framework 2.0 and Diesel ORM."
categories: [rust, tutorial]
date: 2019-11-14T22:52:00
---

In this tutorial, we are going to create a REST API in Rust with Actix web 2.0 and Diesel. We will be using Postgres as our database, so if you don’t have Postgres installed on your computer, you should do that first.

## Hello world
We are going to start by creating our project with Cargo and move into the project directory. 

``` shell-session
$ cargo new rest_api
$ cd rest_api
```

We need to add Actix web to our dependencies for our first example. So let’s add that to the `Cargo.toml`.
``` toml
[dependencies]
actix-web = "2.0"
actix-rt = "1.0"
```

And then we set up the request handler and server in `src/main.rs`.

<<< @/blog/how-to-create-a-rest-api-in-rust/hello_world/src/main.rs

Now that we have created our first server, let’s run it with `cargo run`. To test our REST API, let’s visit [localhost:5000](http://localhost:5000), and we should hopefully see our `hello world`.

## Auto Reloading

It could be quite tedious to recompile the code manually every time that we make a change, so let’s have `cargo-watch` recompile it for us on every change. It would also make sense to combine it with the `listenfd` crate and the `systemfd` utility to keep the connection open while our code recompiles. This way we avoid that our REST client breaks off the requests while the code recompiles since it can’t reach the server. By keeping the connection open we could just make a call to the server and the server will respond as soon as it has recompiled and is ready to handle our request.

For this, we need to install `cargo-watch` and `systemfd`. Both are written in Rust and available on [crates.io](https://crates.io/), so we can install them with cargo.

``` shell-session
$ cargo install systemfd cargo-watch
```

We also need to add `listenfd` to our dependencies.
``` toml
[dependencies]
listenfd = "0.3"
```

Then we need to make some changes to `src/main.rs` so that we can use the listener that is provided for us by `systemfd`, but also have a fallback for cases when we don’t need it. Like when we are deploying our code.

<<< @/blog/how-to-create-a-rest-api-in-rust/auto_reload/src/main.rs

Now we can run the server and file watcher that will automatically recompile our code on changes with this command.

``` shell-session
$ systemfd --no-pid -s http::5000 -- cargo watch -x run
```

## Environment variables and logging
You would probably deploy your code at some point. Then you might want to run the server with some different settings than on your local machine, like using a different port or a different level of logging. You might also need to use some secrets that should not be part of your code, like database credentials. For this we could use environment variables.

Also when you deploy your code you could be sure that it will run into problems at some point. And to help with solving these problems it is important with good logging, so that we could figure out what went wrong and solve the problem.

For setting up environment variables and logging we are going to add another few dependencies.
``` toml
[dependencies]
dotenv = "0.11"
log = "0.4"
env_logger = "0.6"
```

For convenience let’s set up some default parameters that we could use for development. We could do that by creating a `.env` file in the root of our project.

```
RUST_LOG=rest_api=info,actix=info
HOST=127.0.0.1
PORT=5000
```
The log crate provides five different logs levels which are `error`, `warn`, `info`, `debug` and `trace`, where `error` represents the highest-priority log messages and `trace` the lowest. For this tutorial we will set the log level to `info` for our REST API and Actix, meaning we will get all messages from `error`, `warn` and `info`.

To activate logging and environment variable we would only need to make a few small changes to our main file.

<<< @/blog/how-to-create-a-rest-api-in-rust/env_and_logger/src/main.rs

The `dotenv().ok()` function will grab the environment variables from the `.env` file and add them to our servers environment variables. This way we could use these variables by using the `std::env::var()` function, as we have done for setting the host and port.

The log crate will provide five macros that we could use for writing log messages. One for each log level: `error!`, `warn!`, `info!` `debug!` and `trace!`. To see our log messages in `stdout` or `stderr` we need to initiate the `env_logger` which we do with a single function: `env_logger::init()`.

## Api endpoints
Our API will be sending and receiving json data, so we need a way to Serialize and Deserialize json into a data structure recognized by Rust. For this we are going to use Serde. So we need to add that to our list of dependencies.

``` toml
[dependencies]
serde = "1.0"
serde_json = "1.0"
```

Now we will define a user model and we will add the `Serialize` and `Deserialize` annotations so that our model can be extracted from and converted to json.

<<< @/blog/how-to-create-a-rest-api-in-rust/endpoints/src/user/model.rs

Let’s go ahead and create our REST API’s. Our next step will be to persist the data, but for now we will just be using hard coded dummy data.

<<< @/blog/how-to-create-a-rest-api-in-rust/endpoints/src/user/routes.rs

We also need to connect the user routes with the user model and make it available outside of the user directory.

<<< @/blog/how-to-create-a-rest-api-in-rust/endpoints/src/user/mod.rs

We can now replace our “Hello world” endpoint with the actual user endpoints.

<<< @/blog/how-to-create-a-rest-api-in-rust/endpoints/src/main.rs

We should now be able to test the user endpoints we just created by calling them. You can f.eks use [Insomnia](https://insomnia.rest/) or [curl](https://curl.haxx.se/) for that.

## Persisting data

Having a few endpoints is not really helpful if we are not able to persist the data. For this we are going to use Diesel, which is a quite a mature ORM. Diesel will let us connect to Postgres, MySQL and SQLite, but in this tutorial we will only be covering Postgres.

Diesel depends on [`openssl`](https://www.openssl.org/source/) and [`libpq`](https://www.postgresql.org/download/), so we need to install those before we can install Diesel CLI. If you are using a Debian like OS you could simply install that using `apt`.

``` shell-session
$ sudo apt install openssl libpq-dev -y
```

When we have installed the needed dependencies we can install Diesel CLI.

``` shell-session
$ cargo install diesel_cli --no-default-features --features postgres
```

To let Diesel know where our database is, we need to add `DATABASE_URL` to our `.env` file.

```
DATABASE_URL=postgres://postgres:password@localhost/rest_api
```

We can use Diesel CLI to set up Diesel in our project, and create files for our user migration.
``` shell-session
$ diesel setup
$ diesel migration generate create_user
```
In the migrations folder we should now be able to find a folder for our first migration. This folder should contain two files. One named `up.sql` where we will be creating our user schema and one named `down.sql` which should revert everything we did in the `up.sql` file.

<<< @/blog/how-to-create-a-rest-api-in-rust/persisting_data/migrations/2019-09-08-152208_create_user/up.sql
<<< @/blog/how-to-create-a-rest-api-in-rust/persisting_data/migrations/2019-09-08-152208_create_user/down.sql

Now that we have created our first migration, we can run it with Diesel CLI.
``` shell-session
$ diesel migration run
```
This command should also create a schema file that we will use later for building sql queries. Default location for this file is `src/schema.rs`.

When we are dealing with databases we should be prepared for that problems that can occur like connection issues or database conflicts. So we are going to create an own error type to handle these problems.

<<< @/blog/how-to-create-a-rest-api-in-rust/persisting_data/src/api_error.rs

Our error type consists of a status code and a message that we will use to create the error message. We create the error message by implementing `ResponseError`, which we are using to create a json response.

In case we have an internal server error, it is probably not the best show the users what went wrong. For this case we will just let the user know that something went wrong and write the error message to the logs.

Our error type also implements `From<diesel::result::Error>`, so that we don’t have to do that for each time we have to handle a Diesel error.

``` toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
diesel = { version = "1.4", features = ["postgres", "r2d2", "uuid", "chrono"] }
diesel_migrations = "1.4"
lazy_static = "1.4"
r2d2 = "0.8"
uuid = { version = "0.6", features = ["serde", "v4"] }
```
For handling state we will be using statics, although Actix has built in state management. You could read my post on [loose coupling](/why-should-you-care-about-loose-coupling/) to understand why I decided to go for this approach, although some people might disagree with it.

Now let’s establish a database connection and use `r2d2` to efficiently handle the connection pool.

<<< @/blog/how-to-create-a-rest-api-in-rust/persisting_data/src/db.rs

With the database connection established we can finally create the API for creating, reading, updating and deleting the user data.

<<< @/blog/how-to-create-a-rest-api-in-rust/persisting_data/src/user/model.rs

And with the user API in place we could use that instead of the fake data we used earlier.

<<< @/blog/how-to-create-a-rest-api-in-rust/persisting_data/src/user/routes.rs

Now we just have to add `db`, `schema` and `api_error` models in our main file. I also strongly recommend to initiate the database, although that is not strictly necessary. We are using `lazy_static` to handle the database pool. So if we don’t initiate it right way, it won’t be initiated before it will be used. Which then will be when the first user tires to call our API.

<<< @/blog/how-to-create-a-rest-api-in-rust/persisting_data/src/main.rs

We should now be able to create, read, update and delete users via the endpoints.

In case you need it you can also find the complete code on [github](https://github.com/thecloudmaker/actix_tutorials/tree/master/rest_api).

In case you have any questions or suggestions for improvement, feel free to contact me. You could also contact me in case you have suggestions for tutorials or topics I could cover in an upcoming blog post.

Next up I am planning to show how to Authenticate our users. I also have a list of other topics that I plan to cover and you could be the first to know by signing up for the newsletter.
