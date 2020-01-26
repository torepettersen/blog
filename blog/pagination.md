---
title: "Rust Web Development Tutorial: Pagination"
description: "A tutorial on how to extend Diesel's query builder to make pagination."
categories: [rust, tutorial]
date: 2020-01-12T10:06:00
---

In this tutorial, we are going to build pagination for our API results. To solve this problem we need to hook into Diesel’s query builder, so I will also cover how we can do that. This tutorial builds on my tutorial on [building a REST API](/how-to-create-a-rest-api-in-rust), so I highly recommend reading that first or cloning the code from [github](https://github.com/thecloudmaker/actix_tutorials/tree/master/rest_api).

## Extending the query builder
Since we are going to extend our database API, it would make sense to make a separate folder for that. So let’s rename and move the `src/db.rs` file into `src/db/connection.rs`. We also need to remember to create a mod file to continue to expose the methods outside of the module.

``` rust
mod connection;
mod paginate;

pub use connection::*;
pub use paginate::*;
```

The pagination module is of course for our pagination and we will continue with that. Since diesel doesn’t support pagination out of the box, we have to extend the query builder ourselves.

For simplicity, we are going to use offset pagination, although that is not the most efficient on bigger data sets. But you should hopefully be able to use what you learn in this tutorial extend Diesel's query builder with queries for pagination that fits your use case. The query we are going to execute should be able to limit the number of entries that we get and to count the total entries. And we can do that with a query that looks like this:

``` sql
SELECT *, COUNT(*) OVER () FROM (subselect t) LIMIT $1 OFFSET $2
```
To extend the query builder with this query, we need to create a struct that implements the `QueryFragment` trait. A struct that implements `QueryFragment` also needs to implement `QueryId`, which we can implement with the derive attribute.

The struct represents an executable query, so we will also implement `RunQueryDsl` which will add functions like `execute` and `load`. The query also has a return type, which we can state by implementing the `Query` trait.

``` rust
use diesel::prelude::*;
use diesel::pg::Pg;
use diesel::query_builder::*;
use diesel::sql_types::BigInt;

const DEFAULT_PAGE_SIZE: i64 = 10;

#[derive(QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    page_size: i64,
}

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            page_size: DEFAULT_PAGE_SIZE,
            page,
        }
    }
}

impl<T> QueryFragment<Pg> for Paginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.page_size)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.page_size;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}
```

Now we can use the paginate function on queries and load them into a `Vec<(T, i64)>`. So let’s try that on in the user API.

``` rust
// src/user/model.rs
use crate::db::Paginate;
//..

#[derive(Debug, Deserialize)]
pub struct Params {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    // ..
}

impl User {
    pub fn find_all(params: Params) -> Result<(Vec<Self>, i64), ApiError> {
        let conn = db::connection()?;
        let mut query = user::table.into_boxed();
        // ..

        let (users, total_pages) = match params.page {
            Some(page) => {
                let res = query.paginate(page).load::<(User, i64)>(&conn)?;

                let total = res.get(0).map(|x| x.1).unwrap_or(0);
                let users = res.into_iter().map(|x| x.0).collect();
                let total_pages = (total as f64 / 10 as f64).ceil() as i64;
                
                (users, total_pages)
            },
            None => (query.load(&conn)?, 1),
        };
        
        Ok((users, total_pages))
    }
    // ..
```
Now we just have to pass the total pages back to the route before we can give it a try.

``` rust
// src/user/routes.rs
// ..
#[get("/users")]
async fn find_all(filters: web::Query<Params>) -> Result<HttpResponse, ApiError> {
    let (users, total_pages) = User::find_all(filters.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({"users": users, "total_pages": total_pages})))
}
// ..
```
Now we should be able to test our endpoint with the page parameter, but you might notice that we are still not able to change the page size. Also wouldn't it be nicer if we didn't had to write all this boilerplate code each time we add pagination. We can add a another trait and few functions to handle that for us.

``` rust
// src/db/paginate.rs
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::HasSqlType;
// ..
impl<T> Paginated<T> {
    pub fn page_size(self, page_size: i64) -> Self {
        Paginated { page_size, ..self }
    }

    pub fn load_and_count_pages<U>(self, conn: &PgConnection) -> QueryResult<(Vec<U>, i64)>
    where
        Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let page_size = self.page_size;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / page_size as f64).ceil() as i64;
        Ok((records, total_pages))
    }
}

pub trait LoadPaginated<U>: Query + QueryId + QueryFragment<Pg> + LoadQuery<PgConnection, U> {
    fn load_with_pagination(self, conn: &PgConnection, page: Option<i64>, page_size: Option<i64>) -> QueryResult<(Vec<U>, i64)>;
}

impl<T, U> LoadPaginated<U> for T
where
    Self: Query + QueryId + QueryFragment<Pg> + LoadQuery<PgConnection, U>,
    U: Queryable<Self::SqlType, Pg>,
    Pg: HasSqlType<Self::SqlType>,
{
    fn load_with_pagination(self, conn: &PgConnection, page: Option<i64>, page_size: Option<i64>) -> QueryResult<(Vec<U>, i64)> {
        let (records, total_pages) = match page {
            Some(page) => {
                let mut query = self.paginate(page);
                if let Some(page_size) = page_size {
                    query = query.page_size(page_size);
                }

                query.load_and_count_pages::<U>(conn)?
            },
            None => (self.load::<U>(conn)?, 1),
        };

        Ok((records, total_pages))
    }
}
```
Now it should be a bit easier to add pagination with our `LoadPaginated` trait, which also allow us to add the parameter for page size.

``` rust
// src/user/model.rs
use crate::db::LoadPaginated;
// ..
impl User {
    pub fn find_all(params: Params) -> Result<(Vec<Self>, i64), ApiError> {
        let conn = db::connection()?;
        let mut query = user::table.into_boxed();
        // ..
        
        let (users, total_pages) = query
            .load_with_pagination(&conn, params.page, params.page_size)?;
             
        Ok((users, total_pages))
    }
    // ..
```
Now we should also be able to use our API with the page size parameter as well. In case you need it, you can find the complete code on [github](https://github.com/thecloudmaker/actix_tutorials/tree/master/pagination).

## What is up next?
In the previos post about sorting and filtering, we have made a bit of a mess and some quite repetitive code. So in the next tutorial, I will show how we can use macros to clean up our code and avoid repeating ourselves unnecessarily.
