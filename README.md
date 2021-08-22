# httpd

A really simple HTTP static server built from the Rust Book example. It does
note use any kind of HTTP library, instead it simply mathches the requested
paths and folder by decoding the TCP stream and running a regex against it.

## TODO

- [ ] Handle errors instead of using `.unrwap()`
- [ ] Basic HTTP validation
- [ ] Return correct headers
- [ ] Parse options instead of using hardcoded values
- [ ] Add some basic styling
