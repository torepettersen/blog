---
title: "Why Should You Care About Loose Coupling?"
date: 2019-11-07T20:14:00+01:00
categories: [rust]
---

Have you ever been in a situation where a change in your application has caused a strange bug in a completely different part of your application? Or have you at least experienced an API breaking?

You can reduce the chance of these problems by having different parts of your application know as little as possible about each other as possible. This an architectural design called loose coupling.

A loosely coupled application is also a lot easier to split into smaller services if you ever need it. This will be helpful if you ever consider splitting your monolith into microservices.

I have found quite a lot of reasoning about why you should go for a loosely coupled application, but I have found it a bit harder to find some practical examples of how to do that. So here are a few tips to make your application more loosely coupled.

## Folder structure

Imagine you have a folder structure like this
```
.
├── routes
|   ├──order.rs
|   └── user.rs
├── events
|   ├──order.rs
|   └── user.js
├── models
|   ├── order.rs
|   └── user.rs
```
With a folder structure like this, you need to open several folders to understand how each of the features works. And when the list of features increases, I find it hard to navigate, since I am forced to have several folders open at once. Although most of it is irrelevant at the given moment since I am usually working on only one feature at a time.

And also if we ever want to split this into several services, we have to sort through several different folders to connect the parts we need.

An alternative way is to group the foulders by feature, like this.
```
.
├── order
|   ├──mod.rs
|   ├──routes.rs
|   ├──events.rs
|   └── model.rs
├── user
|   ├──mod.rs
|   ├──routes.rs
|   ├──events.rs
|   └── model.rs
```
With a structure like this, it is a lot easier to get an overview of how each feature is working. Here we only need to open one small folder for each feature we are working on. And navigating is also easier since the files that belong together are placed together. 

## Interaction between components

What happens if you let different parts of your application modify the user data in the database? It might work, but in general, the is not a good idea since you would end up with two different approaches that can cause conflict for each other.

Let’s say the one approach has built-in caching, and the other does not care since it was just a quick and dirty implementation. If we now use the second approach to update the database, we will then end up with an invalid cache.

Having different parts of the application changing data in the same database will also make it really hard to get an overview of who is making these changes. And if you don’t know who is making these changes, it will also be hard to debug the code when you find a bug. Also splitting a monolith into microservices is close to impossible when you don’t have an easy overview of which parts are linked together.

If we rather let the different parts of our application interact through APIs, it will be a lot easier to understand how the different parts are linked together. We could even see all the places where the API is used by searching for a reference to it. Another advantage is that we could rewrite the API, without affecting the parts where it is used. This could f.eks could be helpful if you figure that an API is heavily used and needs to add caching.

So how does a good API look like? The best tip I can give is just to keep it simple. If you f.eks want to create an API for finding a user, it should be enough just to pass the id.
``` rust
#[get("/users/{id}")]
fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
   let user = User::find(id.into_inner())?;
   Ok(HttpResponse::Ok().json(user))
}
```

I have seen quite a lot of examples in the rust community where the API connection needs to be passed to the API like this.
``` rust
#[get("/users/{id}")]
fn find(conn: web::Data<PgConnection>, id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
   let user = User::find(conn.into_inner(), id.into_inner())?;
   Ok(HttpResponse::Ok().json(user))
}
```
Right away you will probably not have any problems going for this approach. But you will probably at some time want to do any changes to this API, like adding a cache or maybe publish or subscribe to an event queue.

By adding or removing parameters to our API we will make it break. And if we want to solve that, we have to add this new parameter every single place this API is used. In small projects that might be doable, but what about bigger projects which are maintained by several developers?

## Moving towards Microservices

I don’t suggest starting out with a microservice architecture since that will cause quite a lot of overhead to your project. But if you at some point ever get a lot of customers you might have to scale it up. One way is to divide your application into several smaller services so that each service has a smaller load.

If you followed the tips above you will have a lot easier time to split the different parts of your application into smaller services. You can easily do that by just drag and drop the folders for each feature into your new service.

But wait, this will break our API’s don’t it? Yes, it will, but it is actually a quite easy solution for fixing that since all the users of our API’s don’t make any assumptions on how the API is working. This means that we could just rewrite our APIs to make a remote procedure calls to our newly created service, which will handle the request. And the user of the API doesn’t have to do any changes since the API still looks the same.

## What is coming next?

The next blog post will be a tutorial on how to create a REST API with Rust, Actix Web, and Diesel. If you don’t want to miss out on that, you could sign up for the newsletter and be the first one to know when it is published.

