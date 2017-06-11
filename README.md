# libproxer-rust [![Build Status](https://travis-ci.org/cuechan/libproxer-rust.svg?branch=master)](https://travis-ci.org/cuechan/libproxer-rust)
This is a small project to learn some rust basics. I don't expect that this will ever be stable.


# Accessing the Proxer API

```rust
  let prxr = Proxer::api::Api::new("your_api_key_here".to_string);

  let info: Proxer::api::types::Info = prxr.get_full_info(53);
```

This example creates a api object and fetches the full data for an entry
