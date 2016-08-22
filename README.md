# michromer

Michromer is a library to interact with [StockFighter](https://www.stockfighter.io). This is still very much a work in progress.

[![Build Status](https://travis-ci.org/elliottneilclark/michromer.svg?branch=master)](https://travis-ci.org/elliottneilclark/michromer)

[![Clippy Linting Result](https://clippy.bashy.io/github/elliottneilclark/michromer/master/badge.svg)]


To use it create a level client that can create orders, cancel orders, read the order book.
```
    let client = Client::new(&key);
    let lc = client.start_level("chock_a_block").unwrap();
```

Now go solve some challenges.
