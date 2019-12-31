---
title: "Rust Web Development Tutorial: Email Confirmations"
description: "A tutorial on how to verify users with a email confirmations."
categories: [rust, tutorial]
date: 2019-12-10T18:15:00
---

In this tutorial, we are going to verify our API users with an email confirmation. This tutorial builds on the two previous tutorials on [creating a REST API](/how-to-create-a-rest-api-in-rust) and [authentication](/authenticate-api-users).

For this tutorial we are going to use [Sendinblue](https://www.sendinblue.com) and I would suggest activating the account right away since transactional email needs to be manually activated by support. Don’t worry though, when you have signed up, you just have to send a short email and let them know that you want transactional email activated. I only needed to wait two and a half hours, which I find quite fast for a service I don’t even pay for.

## Creating a model for the email verification token

We are going to start to create a new migration for our email verification token. And we will of course do that with `diesel-cli`.

``` bash
diesel migration generate email_verification_token
```

In the two generated files we add our migration scripts.
``` sql
// up.sql
CREATE TABLE email_verification_token (
    id BYTEA PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);    
```

``` sql
// down.sql
DROP TABLE email_verification_token;
```
Now that we have created the migration, we can execute it with the `diesel migration run` command.

Next we are going to create a model so that we have a representation of the email token in Rust. We are also going to implement methods for finding, creating and deleting the tokens.

<<< @/blog/email-verification/model.rs

We are using a random generated 32-bit value to make it difficult to guess the token. But for production applications I would also suggest a rate limiter to make brute force attacks more difficult.

We don’t want to have several tokens for each email, so we are using `.on_conflict()` to handle these conflicts and using that to overwrite the old token when we are creating a new. By using this approach, we also have a way for the user to create a new token in case he deleted the confirmation email by mistake or waited too long to activate the user.

## Short generics example
To keep it simple I have avoided to use generics until now, but for our email API we can really make our life a bit easier by using that. Generics allows us to create functions that will accept multiple types for the same parameter, as long as the type has the trait that we need.

We can take our `ApiError::new()` method as an example. You have probably noticed that we have written a lot of `.to_string()` inside our `ApiError::new()` functions, since the message is required to be a `String`. But what if we let the message be a generic type? We don’t really care as long as we can turn the message into a `String`, do we?

``` rust
// src/api_error.rs
impl ApiError {
    pub fn new<T: Into<String>>(status_code: u16, message: T) -> ApiError {
        ApiError { status_code, message: message.into() }
    }
}
```

This should not break our code since `String` also implements `Into<String>`. But what happens now if you remove one of the `.to_string()` methods inside one of the `ApiError::new()` menthods, so that we will give a string literal as a parameter instead of a `String`. It still seems to work since `&str` also implements `Into<String>`.

## Email API
For calling the Sendinblue API we will need to be able to do an http call. For this we are going to use `reqwest`. We are also going to need a way to decode and encode our token into something that can be sent with the email and passed back with the json request. For this we can use the `hex` crate that will help us convert a chunk of bytes into hexadecimal. So let’s add these dependencies.

``` toml
[dependencies]
hex = "0.4"
reqwest = "0.9"
```

Now that installed the dependencies we can go ahead and create our email sending API.
 
<<< @/blog/email-verification/api.rs

If you are new to Rust I guess you might stumble a bit on this line: `impl<T: Into<String>> From<T> for Contact`. It is not too much more complicated than our last example, but let’s break that into a couple of parts that will make it easier to understand.

What if we would have written this line like this: `impl From<String> for Contact`. This hopefully seems a bit more familiar. Here we are implementing the `From` trait  for `Contact` so that we can easily convert a `String` into a `Contact`. But we also want to do the same for `&str`. So to avoid repeating ourselves we are defining a generic type `<T: Into<String>>`, that we can use instead of the `String`. Now we are sure that whatever type that we are getting, we will be able to turn into a `String` with the `.into()` method.

If you have a look at the Sendinblue API documentation, you can see that it does not look exactly like our model. The Sendinblue API says that they want the recipient in the `to` field and the HTML in the `htmlContent` field. Serde easily let’s us change that with the `#[serde(rename = “name”)]` attribute.

In the end we have the send method that will call the Sendinblue API to send our email. If we succeed we will return the message id to the caller.

## Invitation and registration endpoint
Now that we have API’s for sending email and creating tokens we can create the endpoints for sending the confirmation email and registration the user.

<<< @/blog/email-verification/routes.rs

For our invite endpoint we are just creating a token, encoding it and sending it to via email. At best we should have had a frontend where we could have directed the user, so he could just click a link instead of having to copy and paste the activation token.

I will not create a frontend for now, since I still have more topics I want to cover about Rust first. But in the future I might be creating a frontend for our app if there is any interest. In that case I would probably be using [elm](https://elm-lang.org/), since that aims to reliable, fast and easy to refactor, just like Rust.

For the registration endpoint we are just searching for the token and validating that it matches the email and that it is not expired. To not reveal too much information to an attacker, we will just let the user know that the token is invalid if not everything is correct. The only exception is if the token is expired, since that can be helpful information for the user.

Now let’s give it a try. First use the invite endpoint to send yourself the confirmation email. Then register with the token you received in the email.

![Registered](./Registered.png)

The complete code example for this tutorial is available on [github](https://github.com/thecloudmaker/actix-web-email-verification).

## What is up next?
Now that you know how to create a REST API in Rust, you will probably make an app that will get a lot of users. Our endpoint for finding all the users will be giving us back way to many users. Therefore the next tutorial will be about how we can narrow down the results with filtering and also allow for sorting of the results.
