---
title: "Why Rust?"
date: 2019-10-29T22:02:00+01:00
categories: [rust]
---

For most programming languages I feel you have to sacrifice either safety and convenience or performance.

You have languages like C and C++ which is incredibly fast since they don’t have any runtime or garbage collector. But for that, you have to sacrifice quite a lot of effort to write the code and you can easily end up with bugs that are quite hard to catch.

On the other side, you have languages like Python and Ruby which are easy and fast to write since they handle a lot of the hard stuff in the background like memory management. But for that, you have to sacrifice quite a lot of runtime performance.

Go is definitely one of my favorite programming languages and I feel that it has done a quite good attempt to manage both, although it does not quite reach up to the raw performance of C and C++. But there are a few things I dislike about go, like the way error handling is handled and the missing package manager.

And then we have Rust which aims to be both performant, reliable and productive. And I believe these goals have quite a lot to do with the hype around Rust right now. Rust is even the most loved programming language according to the stack overflow survey. But why is Rust so loved by programmers if there is no garbage collector to help you out with memory management and no runtime to help out with multithreading?

## Rust is performant
Rust compiles into native machine code and can run on multiple platforms. It is even a great fit for embedded systems. Native machine code with no overhead in the form of a garbage collector or runtime makes Rust binaries so performant that they are even comparable with C and C++.

## Rust is reliable
Instead of a garbage collector, Rust has quite strict rules that you have to follow that guarantee memory and thread-safety. If you are not following these rules the compiler won’t even let you compile the code. The compiler is also enforcing you to handle all possible errors and return values right away, so bad code will never make it to production. But don’t worry, the compiler is quite helpful in pointing out your mistakes, so it is quite easy for you to fix them before your colleagues will point them out to you during the code review.

## Rust helps you be productive
You will not be able to write a prototype as fast in Rust as you can in JavaScript or Python. Writing Rust code initially takes a bit longer, but I feel I save quite a lot of time by having the compiler point out silly mistakes right away. At least compared to having to run my program just to see it fail, or even worse, having a customer or project manager finding it for you in production. I definitely prefer having the compiler pointing me to the exact line number where I made the mistake than trying to get a project manager explaining to me the steps he made to produce the bug, so I can reproduce it.

Another big timesaver is refactoring. The compiler will once again help you out if you break some existing code. I have done some bigger changes to an API, and all I had to do was just to follow the error messages from the compiler to find all the places it broke. When I was through the error messages I could see the program run as expected again.

## Powerful language features
Rust has a lot of the more powerful language features we see in modern programming languages like pattern matching, generics, and traits. Rust also has one of the more powerful macro features I have seen and is really great for code reuse, even in more complex situations where a function call won’t be possible. And of course, no runtime overhead since they are converted into actual code at compile time. The usage of monadic error handling in Rust makes it really easy to make sure that you have handled all possible return values without being a pain. And if you are a fan of functional programming Rust also has some functions like map(), filter() and find().

## Cargo
The package manager for Rust is called Cargo and it does more than just grabbing the packages for you. Cargo will help you with everything from creating your project, to building your code. You can also use it to handle your dependencies, testing your code and upload your packages to [crates.io](https://crates.io/), which is the packages registry for Rust. And best of all, most of it works straight out of the box. No need to write a huge make file or learn a tool like Webpack. You mostly just have to list your dependencies and Cargo will take care of the rest.

## Helpful community
I got to admit that Rust is one of the more difficult languages I have learned. Rust really has some more difficult concepts you have to learn, which is not common in other programming languages. But to help you, there is actually an [official book](https://doc.rust-lang.org/book/) that you could read for free, and it is great. If you are of the type that prefers physical books you could also grab a [hard copy](https://www.amazon.com/Rust-Programming-Language-Covers-2018/dp/1718500440/ref=tmm_pap_swatch_0?_encoding=UTF8&qid=1572382513&sr=8-1). Rust also has the most polite and helpful community I have come across. So if you ever get stuck, don’t worry about getting help from the community.

## Libraries
There are about 30 000 libraries available on [crates.io](https://crates.io/), at the moment of writing. That might sound like a small number compared to the 1 000 000 libraries on NPM, but I still feel that I can find what I need. And also I find that the quality of the libraries usually is better than what I am used to from JavaScript. You have libraries like the Actix web which is an incredibly performant web framework. Just check out the [benchmarks](https://www.techempower.com/benchmarks/#section=data-r18). The serialization framework Serde is also top of the class and is even faster than comparable libraries in C++. And Diesel is such a good ORM tool, that it is even more popular than the native client libraries for each database.

## So where should you start?
So hopefully you are now all convinced that Rust is a good choice for you as well. Or did I forget to mention that Rust is also quite fun to write as well? Anyway, a great place to start is by reading [the book](https://doc.rust-lang.org/book/). You could also grab a [hard copy](https://www.amazon.com/Rust-Programming-Language-Covers-2018/dp/1718500440/ref=tmm_pap_swatch_0?_encoding=UTF8&qid=1572382513&sr=8-1) if you prefer. And if you are into web development you can be one of the first to know when I publish a new post or tutorial about that, by signing up for my newsletter or checking out the other posts on my blog.
