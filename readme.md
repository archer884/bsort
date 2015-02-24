bsort
=====

A worthless bubble sort library for Rust.

> Please don't use this!

Up until now, I never got around to writing a bubble sort. Like, ever. Mostly because, when I was first learning about programming, the first piece of advice I read was "don't ever use bubble sort." I didn't actually know what it was then, so I ... Yeah, I pretty much had no trouble following that advice. I looked up how to implement "quicksort," and that's what I used--right up until I found out that A) `Array.Sort()` is an optimized quicksort, and B) that linq is cooler than your mom.

Those findings came in quick succession.

Anyway, after that, it never seemed like a worthwhile exercise to try writing bubble sort because it just didn't sound all that hard to do. Enter Rust: a language where practically everything seems hard to do until you understand the Borrow Checker's take on it. So, here you have it: my first ever implementation of bubble sort.
