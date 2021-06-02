# music
Repository for music-based project started on Raspberry Pi. Currently written in Python using multiple basic libraries and the Synthesizer library for music output.

# Current programs and definitions:

## PyTunes.py

This will be a growing python library of useful code that I make. Currently contains two functions: keyboard and rooot

### rooot

`rooot(note)` just takes a string input of the desired note, and will output the frequency value of the note in range C1 - B1 in Hz. Flats are denoted as a lowercase "b" as follows: `Bb`, or `Gb`, and sharps as "#" `A#`, or `F#`

#### Sample usage:

`x = rooot('A#')`

`print(x)`

`58.27047018976126`

### keyboard

`keyboard(root, nkeys, form)` creates a full keyboard starting at the root frequency of choice and with the selected number of keys, and following the mode, or scale of choice

`root` input can be obtained using the `rooot(note)` function described above. `nkeys` is a design choice. 

`form` has the following options: `maj`, `min`, `pent`, `minpent`, `blues` 

Currently `min` only corresponds to the harmonic minor scale and will eventually be increased to include the melodic minor (maybe)

#### Sample usage:

`root = rooot('Bb')`

`N = 24 #Corresponding to 3 octaves`

`notes = keyboard(root, N, 'min')`

This sample would give me 3 octaves all in Bb harmonic minor.

## im2tunes.py

This is the original purpose of the project. On a whim I decided to see if I could turn an image into music using RGB values converted to notes/frequencies. Currently uses .jpg images or any other 3 color channel images. It is also currently configured to use colors in the standard 8 bit 0-255 range. 

## tunes2im.py

This is incomplete as of this commit, working on creating an inverse of the im2tunes program, hoping to utilize fft, identify the 3 highest frequencies, and convert to rgb values. 
