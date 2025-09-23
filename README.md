# imgc
A command line tool to quickly convert images in the terminal. 

## Supported File Formats
- .png
- .jpeg
- .jpg
- .heic
- .heif
- .gif
- .webp
## Usage
Example `imgc *.jpg webp` converts all jpg files in the working directory to webp.
#### **Before**
```
project-root/
├ photo1.jpg
├ photo2.jpg
├ photo3.JPG
```

#### **After**
```
project-root/
├ imgc-backup/
    ├ photo1.jpg
    ├ photo2.jpg
├ photo1.webp
├ photo2.webp
├ photo3.JPG
```
## List of Flags
- `-q` Quality of the conversion, provided as a percentage.
    - Example `imgc -q 90 *.png jpg` produces jpg images from the png
        source with 90% quality.
- `-o` Name of the output. Can be singular or plural, where plural
        examples proceed in alphabetical order.
    - Singular example `imgc -o my-photo source.png webp` produces 
    ```
    source.png
    my-photo.webp # same subject as source.png
    ```
    - Plural example `imgc -o my-photo *.png webp` produces
    ```
    banana.png
    apple.png
    my-photo1.webp # same subject as apple.png
    my-photo2.webp # same subject as banana.png
    ```
## Build From Source
This project is built using [Cargo for Rust](https://doc.rust-lang.org/cargo/). To build the binaries, run
```
$ cargo build
```
from the project's root directory.
