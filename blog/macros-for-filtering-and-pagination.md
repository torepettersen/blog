---
title: "Rust Web Development Tutorial: Macros"
description: "A tutorial on how to use macros to reduce boilerplate code and improve readability."
categories: [rust, tutorial]
date: 2020-01-26T11:38:00
---

In this tutorial, we are going to create a couple of macros to avoid repeating ourselves all the time and to make our code a bit cleaner. The code we are going to clean up is from the tutorial on [sorting and filtering](https://cloudmaker.dev/sorting-and-filtering-with-diesel/), so I highly recommend reading that first or cloning the code from [github](https://github.com/thecloudmaker/actix_tutorials/tree/master/filtering_and_sorting).

## Define our first macro for sorting
The macros we will create are for helping us with database functions like sorting and filtering. We will start by creating a new file in the db folder and create our first macro to help us with sorting.

``` rust
// src/db/macros.rs
#[macro_export]
macro_rules! sort_by {
   ($query:expr, $sort_by:expr, $(($param:expr, $column:expr)),*) => {
       {
           if let Some(sort_by) = $sort_by {
               $query = match sort_by.as_ref() {
                   $(
                       $param => $query.order($column.asc()),
                       concat!($param, ".asc") => $query.order($column.asc()),
                       concat!($param, ".desc") => $query.order($column.desc()),
                   )*
                   _ => $query,
               }
           }
           $query
       }
   };
}
```

This might be a lot of new syntax, but I will not go too deep into the details since that is already explained quite well in the [official docs](https://doc.rust-lang.org/1.7.0/book/macros.html). But in short, we are defining a macro that expects a query and the parameter that we will use to sort by. Also, we need to define which values we allow the parameter to be and which column the value is referring to. Then we just use this information to rebuild a more generic query.

Don’t forget to add the macro module to the `db/mod.rs` file, before we continue:
``` rust
// src/db/mod.rs
mod connection;
mod paginate;
mod macros;

pub use connection::*;
pub use paginate::*;
```

 Now let’s replace our existing implementation for sorting and instead use our new macro:
``` rust
// src/user/model.rs
use crate::sort_by;
// ..

impl User {
   pub fn find_all(params: Params) -> Result<(Vec<Self>, i64), ApiError> {
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

       query = sort_by!(query, params.sort_by,
           ("id", user::id),
           ("email", user::email),
           ("created_at", user::created_at),
           ("updated_at", user::updated_at)
       );
      
       let (users, total_pages) = query
           .load_with_pagination(&conn, params.page, params.page_size)?;
      
       Ok((users, total_pages))
   }
   // ..
```
For the sorting we now went down from 17 lines to 6. That is quite an improvement and makes it a bit easier to read. The more values we allow to sort by, the more lines we will save since we now only have to add one new line for each extra value instead of tree without the macro.
Also notice that the macro is being placed at the root of our crate instead of in the database module when we are adding it to the namespace.

## Macro for filtering
The macro for filtering is going to be a bit more complex, since we need to have the option for selecting if we want to use `gt`, `le` or `like` to compare the values. But we can do that as well with a bit of creativity.

``` rust
// src/db/macros.rs
#[macro_export]
macro_rules! filter {
   ($query:expr, $(($column:expr, @$expression_method:ident, $param:expr)),*) => {
       {
           $(
               if let Some(item) = $param {
                   let filter = filter!($column, @$expression_method, item);
                   $query = $query.filter(filter);
               }
           )*
           $query
       }
   };
   ($column:expr, @like, $item:expr) => { $column.like($item) };
   ($column:expr, @ge, $item:expr) => { $column.ge($item) };
   ($column:expr, @le, $item:expr) => { $column.le($item) };
}
```
As with our last macro we also have to pass the query into the macro for filtering. For each filter we want, we also need to pass the column, which expression method we are going to use and which parameter we want to compare with.
You probably notice that this new macro has several arms. It works a bit similar to the match statement, just that for the macros we are comparing complete statements instead of a single variable.

Also we have created our own keywords `@like`, `@ge` and `@le`, to differentiate the different expression methods. The reason we are using `@` as a prefix is that it is not used in prefix position, meaning it will not conflict with anything.

Now let’s use our new macro.
``` rust
use crate::{filter, sort_by};
// ..
impl User {
   pub fn find_all(params: Params) -> Result<(Vec<Self>, i64), ApiError> {
       let conn = db::connection()?;

       let mut query = user::table.into_boxed();

       query = filter!(query,
           (user::email, @like, params.email),
           (user::created_at, @ge, params.created_at_gte),
           (user::created_at, @le, params.created_at_lte),
           (user::updated_at, @ge, params.updated_at_gte),
           (user::updated_at, @le, params.updated_at_lte)
       );

       query = sort_by!(query, params.sort_by,
           ("id", user::id),
           ("email", user::email),
           ("created_at", user::created_at),
           ("updated_at", user::updated_at)
       );

       let (users, total_pages) = query
           .load_with_pagination(&conn, params.page, params.page_size)?;
      
       Ok((users, total_pages))
   }
   // ..
```

In this case we were able to reduce the filtering from 15 to 7 lines. But I feel that the even bigger win here is readability. Our custom keywords make it really easy to see which expression is used for each column and to which param it is compared with.

So hopefully now everything should work as before, but improved readability is always a plus.

And as always, you will be able to find the complete code on [github](https://github.com/thecloudmaker/actix_tutorials/tree/master/macros).
