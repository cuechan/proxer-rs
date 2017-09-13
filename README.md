# libproxer-rust [![Build Status](https://travis-ci.org/cuechan/proxer-rs.svg?branch=master)](https://travis-ci.org/cuechan/proxer-rs)
This is a small project to learn some rust basics. I don't expect that this will ever be ~stable~ ready for production use.


# Accessing the Proxer API

```rust
  let prxr = proxer::Api::new("");


  let foo = self.info_api.info.get_full_info(42);

  if foo.is_err() {
      match foo.unwrap_err() {
          proxer::error::Error::Http => println!("interwebs error"),
          proxer::error::Error::Json => println!("I cant understand your Json"),
          proxer::error::Error::Api(e) => println!("API error: {}", e),
          proxer::error::Error::Unknown => println!("i dont know what happened"),
      }
  }

  // everything went fine
  println!("{:#?}", foo.unwrap());
```

This example creates a api object and fetches the full data for an entry
