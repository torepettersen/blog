---
title: "Rust Web Development Tutorial: Sorting and Filtering"
description: "A tutorial on how to filter and sort API results with Diesel query builder"
date: 2019-12-30T22:30:00+01:00
categories: [rust, tutorial]
---

In this tutorial, we are going to allow our API results to be filtered and sorted. To help us solve this we are going to use Diesel’s query builder to build a conditional query. This tutorial builds on my tutorial on [building a REST API](/how-to-create-a-rest-api-in-rust), so I highly recommend reading that first or cloning the code from [github](https://github.com/thecloudmaker/actix_tutorials/tree/master/rest_api).

We are going to start by creating a struct for the parameters that we allow to filter and sort by. Then we pass the filters to our API for finding users before we build up our conditional query.

``` rust
// src/user/model.rs
//..
#[derive(Debug, Deserialize)]
pub struct Params {
    pub email: Option<String>,
    pub sort_by: Option<String>,
    #[serde(rename = "created_at[gte]")]
    pub created_at_gte: Option<NaiveDateTime>,
    #[serde(rename = "created_at[lte]")]
    pub created_at_lte: Option<NaiveDateTime>,
    #[serde(rename = "updated_at[gte]")]
    pub updated_at_gte: Option<NaiveDateTime>,
    #[serde(rename = "updated_at[lte]")]
    pub updated_at_lte: Option<NaiveDateTime>,
}

impl User {
    pub fn find_all(params: Params) -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let mut query = user::table.into_boxed();

        if let Some(email) = params.email {
            query = query.filter(user::email.like(email));
        }
        if let Some(created_at_gte) = params.created_at_gte {
            query = query.filter(user::created_at.ge(created_at_gte));
        }
        if let Some(created_at_lte) = params.created_at_lte {
            query = query.filter(user::created_at.le(created_at_lte));
        }
        if let Some(updated_at_gte) = params.updated_at_gte {
            query = query.filter(user::updated_at.ge(updated_at_gte));
        }
        if let Some(updated_at_lte) = params.updated_at_lte {
            query = query.filter(user::updated_at.le(updated_at_lte));
        }
        if let Some(sort_by) = params.sort_by {
            query = match sort_by.as_ref() {
                "id" => query.order(user::id.asc()),
                "id.asc" => query.order(user::id.asc()),
                "id.desc" => query.order(user::id.desc()),
                "email" => query.order(user::email.asc()),
                "email.asc" => query.order(user::email.asc()),
                "email.desc" => query.order(user::email.desc()),
                "created_at" => query.order(user::created_at.asc()),
                "created_at.asc" => query.order(user::created_at.asc()),
                "created_at.desc" => query.order(user::created_at.desc()),
                "updated_at" => query.order(user::updated_at.asc()),
                "updated_at.asc" => query.order(user::updated_at.asc()),
                "updated_at.desc" => query.order(user::updated_at.desc()),
                _ => query,
            };
        }

        let users = query
            .load::<User>(&conn)?;

        Ok(users)
    }
    //..
}
```
Notice that we are using `.into_boxed()` when we create the initial query. This is used to box the query into a single type so that the compiler knows how to deal with it when we build our conditional query.

You probably also notice that we have to be quite explicit about the behavior of our API. For every single parameter, we have to define the expected behavior and we also have to define every possible parameter for sorting. The good thing about that is that we know what to expect from our API and we will not end up with any surprising behavior.

If you are a bit like me you would probably also think that this kind of code is a bit messy. I feel that I have to repeat myself a bit too much by writing this kind of behavior over and over again when I write several APIs. To solve that problem we can use [macros](https://doc.rust-lang.org/1.29.0/book/2018-edition/appendix-04-macros.html), but I will be covering that in an upcoming tutorial.
Next, we need to grab the parameters from the request and pass them to our API.

``` rust
// src/user/routes.rs
//..
use crate::user::{User, UserMessage, Params};

#[get("/users")]
async fn find_all(params: web::Query<Params>) -> Result<HttpResponse, ApiError> {
    let users = User::find_all(params.into_inner())?;
    Ok(HttpResponse::Ok().json(users))
}
//..
```

Now we can give our API a try. Here are a few examples requests you can try and you will also be able to combine the parameters.
``` bash
$ curl 'http://localhost:5000/users?email=john%'
$ curl 'http://localhost:5000/users?created_at[lte]=2019-12-11T00:00:00'
$ curl 'http://localhost:5000/users?sort_by=created_at.desc'
```

The complete source code of this tutorial is available on [github](https://github.com/thecloudmaker/actix_tutorials/tree/master/filtering_and_sorting) in case you need it.

## What is up next?
Next up I will make a tutorial on how we can hook into Diesel’s query builder to extend our API with pagination. You can sign up for the newsletter if you want to be the first to know.
