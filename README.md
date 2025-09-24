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

## Installation

While in development, refer to the [Build From Source](#build-from-source) section.

## Usage

Syntax is defined:

```
imgc [options] [target file(s)] [output file type]
```

A simple example:

```
imgc *.jpg webp
```

This command converts all the jpg files in the working directory to webp, storing the source files in an `imgc-backup/` directory. This is created in the directory where the command is ran.

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

- `-o <FILENAME>` Name of the output. Can be singular or plural, where plural examples convert in alphabetical order. This option will disable the `imgc-backup` directory.
    - Singular example
    ```
    $ imgc -o my-photo.webp source.png webp

    source.png
    my-photo.webp # same subject as source.png
    ```
    - Plural example
    ```
    $ imgc -o my-photo.webp *.png webp

    banana.png
    apple.png
    my-photo1.webp # same subject as apple.png
    my-photo2.webp # same subject as banana.png
    ```

- `-q <PERCENTAGE>` Quality of the conversion, provided as a percentage.
    - Example `imgc -q 90 *.png jpg` produces jpg images from the png
        source with 90% quality.
- `-R` Recursive. This will traverse the root directory and apply the given command on all child directories.
    - Example `imgc -R *.png webp`

    ### **Before**
    ```
    project-root/
    ├ gallery1/
        ├ cat.png
        ├ dog.png
    ├ gallery2/
        ├ boat.png
        ├ car.png
    ```

    ### **After**

    ```
    project-root/
    ├ gallery1/
        ├ imgc-backup/ 
            ├ cat.png
            ├ dog.png
        ├ cat.webp
        ├ dog.webp
    ├ gallery2/
        ├ imgc-backup/ 
            ├ boat.png
            ├ car.png
        ├ boat.webp
        ├ car.webp
    ```

- `-v` Verbose. This will not produce the `imgc-backup` directory, removing the original image. **Options `-v` with `-R` will remove the original files form all child directories.**

## Build From Source

This project is built using [Cargo for Rust](https://doc.rust-lang.org/cargo/). To build the binaries, run

```
$ cargo build
```

from the project's root directory.  

This can also be ran directly.

```
$ cargo run -- [options] [target file(s)] [output file type]
```
