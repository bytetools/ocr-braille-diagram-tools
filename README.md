# OCR Braille Diagram Tools

This tool should not exist.
And it also doens't work very well.
*BUT*, it has a few, limited, useful cases.

Here is the basic idea: import an *existing* diagram of some kind and use OCR+manual touch ups to have appropriate highlights and text values.
Then, whiteout existing text and replace it with braille.
This is an arduous, annoying process, but it can be useful in rare cases.

## Requirements

* `python` (3.8+)
	* `louis` (in `requirements.txt`)
* `libouis` (from package manager)

## Usage

Use `./easy.sh`.
The file must be called `diagram.png` in the same directory as the `easy.sh` file.
The generated file will be called `out.png`.

### Generate OCR Data

```bash
./easy.sh gen-ocr
```

This creates an `out.json` file with all OCR data.

### Show OCR Detection Boxes

```bash
./easy.sh show-labels
```

This writes to the `out.png` file with OCR boxes highlighted in red, using the `out.json` file to create the highlights.
The highlight box has the number on the left of it, that is the box ID.
This is important for later.

### Editing Data

Open `out.png` in an image viewer which will automatically update when the file is written to.
When any of these commands are run, the `out.png` file will be rewritten with the new data.

```bash
./easy.sh edit-json
# merges boxes 1 and 2, from smallest (x,y) to largest (x+w,y+h)
merge|1|2
# split box 3 vertically in two
vsplit|3
# split box 7 horizontally in two
hsplit|7
# show raw JSON data of box 9
show|9
# change text of box 1 to "hello world"
text|1|hello world
# add new box at (420,69) with width of 80, height of 150
add|420|69|80|150
# add 10px width/height to the left/right/top/bottom of box 1
paddl|1|10
paddr|1|10
paddt|1|10
paddb|1|10
# remove 15px width/height to the left/right/top/bottom of box 3
triml|3|15
trimr|3|15
trimt|3|15
trimb|3|15
# move box 1 left/right/up/down by 8px
movel|1|8
mover|1|8
moveu|1|8
moved|1|8
# remove box 6
rem|6
# save data to `out.json`
save
# exit program
^C
$ 
```

### Whiteout Existing Text

```bash
./easy.sh whiteout
```

This will rewrite `out.png` with an image that has all the boxes defined in `out.json` replaces by entirely white pixels.

### Add Braille To Diagram

```bash
./easy.sh add-braille
```

This will rewrite `out.png` with braille text at the position of all the OCR objects in `out.json`.

## Examples

See the `/done` directory for examples of OCR, `png`s at various states of done-ness and the final version.
Most of these have a massive fault: the braille font is *NOT* the right size and is not readable.

But it should give you an idea of how to use the tool.

## Contribution

If you'd like to contribue, please use [the Github](https://github.com/bytetools/ocr-braille-diagram-tools) repo unless you work with Bytetools and have permission.
