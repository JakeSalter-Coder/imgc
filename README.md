# imgc
A command line tool to quickly convert images in the terminal. 

## Supported File Formats
- png
- jpeg
- jpg
- heic
- heif
- gif
- webp
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
├ img-backup/
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

