Imaginator
==========

Imaginator is a fast, safe web interface to imagemagick. While imaginator is relatively stable and is deployed in production at pixers, this document is a work in progress.

Dependencies
------------

  * Nightly rust (because we have to use the system allocator)
  * ImageMagick 7 (and whatever else [magick-rust](https://crates.io/crates/magick_rust) requires)

Installation
------------

First, install ImageMagick - magick_rust (and imaginator) needs it to automatically generate correct bindings.

Then, build the binary using `cargo build --release`. Your binary will be in `target/release/imaginator`.
Note that it still depends on ImageMagick – you'll have to install the same version on the machine you want to run it at.

Usage
-----

Once started, imaginator listens on port 3000. It takes a list of filters in a url, and returns an image. For example, resizing an image can be done with the following query:

    curl http://127.0.0.1:3000/download(http:example.com/image.jpg):resize(0.5w, 0.5h)

This will resize the image to 50% of its width, and 50% of its height.


