# proxer-rs
[![cates.io](https://img.shields.io/crates/v/proxer.svg)](https://crates.io/crates/proxer)
[![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)](https://crates.io/crates/proxer)
[![Build Status](https://travis-ci.org/cuechan/proxer-rs.svg?branch=master)](https://travis-ci.org/cuechan/proxer-rs)

Access [proxer.me](https://proxer.me) with rust


# Making Requests

Making requests is fairly simple:


```rust
let prxr = proxer::Api::new("yourapikey");
// or load the api-key from an environment variable
let prxr = proxer::Api::with_env_key("PROXER_API_KEY");


let req = client.api().info().get_fullentry(
  // this struct is equivalent to the documented parameters
  // for this endpoint. See http://proxer.me/wiki/Proxer_API/v1/Info#Get_Full_Entry
  parameter::InfoGetFullEntry {
    id: 53
  }
);


// send the request
match req.send() {
  Ok(data) => println!("{:#?}", data),
  Err(e) => {
    match e {
      error::Error::Api(k) => panic!("API error: {}", k),
      error::Error::Json(k) => panic!("something is wrong: ", k),
        error::Error::Http => panic!("interwebs error"),
        error::Error::Unknown => panic!("i dont know what happened"),
    }
  }
}

```

This example creates a api object and fetches the full data for an entry

The library is as strong typed as possible (which is a good thing for guaranteeing type safety).
It tries to be a 1:1 wrapper to the api while providing some nice tweaks like `Iterator`s for pageable endpoints.


```rust
let req = client.api().info().get_comments(
  parameter::InfoGetComments {
    id: 53
  }
);


for comment in req.pager() {
  // Now, new comments are automagically fetched when a page end is reached

  println!("user:   {}", comment.unwrap().username);
  println!("rating: {}", comment.unwrap().rating);
}
```
