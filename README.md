``` bash
#Creating a project
cargo new <appname> --bin
#--bin tags the app as a app and not a lib

# Run With Cargo
cargo run

# Build
cargo build

# Build for production
cargo build --release
```

to use external libraries
add [dependencies] tag followed by package name
```
[dependencies]
regex = "0.2"
```

to use the libraries
extern crate regex
