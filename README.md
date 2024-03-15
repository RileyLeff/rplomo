# rplomo

## Plant Optimization Models in R

A (very) thin **R** wrapper around a **pl**ant **o**ptimization **mo**deling framework written in Rust.

### why is this public? it's barely breathing!

Currently under construction, but needs to be public to access the free tier MacOS 14 + M1 runners.

### what can it do right now?

Currently working on a soil-hydraulics focused transpiration model.

As of 3/15/2024 this package has been successfully integrated with its rust crate counterpart, but the model logic has not been integrated yet.

### what's the plan?

This is a spiritual successor to the repository "ezsperry-beta", which i'm not even going to link to because no sane person should ever read that code.

The big goals of this project are essentially are:

- break the model functionality out into distinct modules/crates (i.e. one for photosynthesis, one for soil hydraulics, etc)
- make model execution generic over traits, with specific implementations configurable at runtime (i.e. vuln. curve inputs could be anything that implements a VC trait, i.e. weibull or exponential curve)
- make it easy to compose different model setups from the set of base components

Expect many breaking changes moving forward. Nowhere close to a stable API here.

### what if I want to use it now?
 If you're using this now, lock your dependencies to a specific version. It's extremely unlikely that future versions of this package will be compatible with any code written with current version of this package.
