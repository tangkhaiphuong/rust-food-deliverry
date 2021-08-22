# rust-food-delivery
Food Delivery in Rust

# Install 3rd library
Due to this example need to link 3rd library `libmysql` so please install `vcpkg`
to make progress better.

Please follow tutorial from: `https://github.com/microsoft/vcpkg#quick-start-windows`

## Download mysql

### Windows
```
vcpkg.exe --triplet x64-windows-static install libmysql 
```

### Linux
```
./vcpkg --triplet x64-linux install libmysql
```

### MacOS
```
https://stackoverflow.com/questions/54969208/how-to-link-mysql-client-installed-from-homebrew-with-diesel-cli

```
