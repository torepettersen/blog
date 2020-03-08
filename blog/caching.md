---
title: "Rust Web Development Tutorial: Caching"
description: "A tutorial on how to use Redis as a cache."
categories: [rust, tutorial]
date: 2020-02-16T10:00:00
--- 

If your web application becomes really popular, you might end up finding yourself in a situation where you are reaching the limits of your database. You can always give it a try to scale up your database, with all the complexities that come with that. But it could also make sense to see if it is possible to reduce the load before going down that road.

If you f.eks are making a weather app, you will probably be getting a lot of requests for big cities like Berlin or London. Then it would not be necessary to ask the database for the same data over and over again since it will not be changing in between every request. Instead, we can store frequently requested data in a cache, to move the load away from the database on over to the cache.

For this tutorial, we are going to use Redis as a cache, so you should have that installed on your computer. I also assume that you know how to [make a rest api](https://cloudmaker.dev/how-to-create-a-rest-api-in-rust/), since I will use that a primer for this tutorial.

Now let’s start by adding Redis as a dependency in our `Cargo.toml`.
```toml
[dependencies]
redis = { version = "0.15", features = ["r2d2"] }
```

We also need to add the Redis URL to our environment variables.
```
# .env
REDIS_URL=redis://localhost
```

Next, we are going to set up a connection pool for Redis, just like we have done for our database.

```rust
// src/cache.rs
use crate::api_error::ApiError;
use lazy_static::lazy_static;
use r2d2;
use redis::{Client, ConnectionLike};
use std::env;

type Pool = r2d2::Pool<Client>;
pub type CacheConnection = r2d2::PooledConnection<Client>;

lazy_static! {
    static ref POOL: Pool = {
        let redis_url = env::var("REDIS_URL").expect("Redis url not set");
        let client = redis::Client::open(redis_url).expect("Failed to create redis client");
        Pool::new(client).expect("Failed to create redis pool")
    };
}

pub fn init() {
    info!("Initializing Cache");
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get redis connection");
    assert_eq!(true, conn.check_connection(), "Redis connection check failed");
}

pub fn connection() -> Result<CacheConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}
```
We also have to remeber to initate our cache before we can use it in our APIs.
```rust
// src/main.rs
// ..
mod cache;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    db::init();
    cache::init();
    // ..
```
And for convenience let’s implement `From<RedisError>` for `ApiError` so that we can handle those errors with`?`.

```rust
// src/api_error.rs
use redis::RedisError;

impl From<RedisError> for ApiError {
    fn from(error: RedisError) -> ApiError {
        ApiError::new(500, format!("Redis error: {}", error))
    }
}
```

Now that we have set up our cache we can use it for our user API.
```rust
// src/user/model.rs
use crate::cache;
use redis::Commands;

impl User {
    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        if let Some(user) = User::cache_find(id)? {
            return Ok(user);
        }

        let conn = db::connection()?;
        let user = user::table
            .filter(user::id.eq(id))
            .first::<User>(&conn)?;

        user.cache_set()?;

        Ok(user)
    }

    pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;
        let user = diesel::update(user::table)
            .filter(user::id.eq(id))
            .set(user)
            .get_result::<User>(&conn)?;

        user.cache_set()?;

        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;
        let res = diesel::delete(
                user::table
                    .filter(user::id.eq(id))
            )
            .execute(&conn)?;

        User::cache_delete(id)?;

        Ok(res)
    }

    fn cache_find(id: Uuid) -> Result<Option<Self>, ApiError> {
        let cache_key = format!("user.{}", id);
        let mut cache = cache::connection()?;
        let res: Vec<u8> = cache.get(&cache_key)?;
        match serde_json::from_slice::<User>(&res).ok() {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }

    fn cache_set(&self) -> Result<(), ApiError> {
        let cache_key = format!("user.{}", self.id);
        let mut cache = cache::connection()?;
        if let Some(cache_user) = serde_json::to_vec(self).ok() {
            let _: () = cache.set_ex(&cache_key, cache_user, 3600)?;
        }
        Ok(())
    }

    fn cache_delete(id: Uuid) -> Result<(), ApiError> {
        let cache_key = format!("user.{}", id);
        let mut cache = cache::connection()?;
        let _: () = cache.del(cache_key)?;
        Ok(())
    }
}
```

Our API for finding a single user will now first check the cache if the user is already there, and will only ask the database in cases where the user was not found in the cache.

We also need to make sure to invalidate our cache on updates and deletes, since updates and deletions will make the data in our cache invalid.

We have also set a timeout on all entries in the cache to one hour so that the data don’t stay in the cache too long if it is not frequently used.

You can also find the complete code for this tutorial on [github](https://github.com/thecloudmaker/actix_tutorials/tree/master/caching).
