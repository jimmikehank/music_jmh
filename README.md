# music
Repository for music-based project started on Raspberry Pi. Currently written in Python using multiple basic libraries and the Synthesizer library for music output.

# Current programs and definitions:

## PyTunes.py

This will be a growing python library of useful code that I make. Currently contains two functions:

## im2tunes.py

This is the original purpose of the project. On a whim I decided to see if I could turn an image into music using RGB values converted to notes/frequencies. Currently uses .jpg images or any other 3 color channel images. It is also currently configured to use colors in the standard 8 bit 0-255 range. 

## tunes2im.py

This is incomplete as of this commit, working on creating an inverse of the im2tunes program, hoping to utilize fft, identify the 3 highest frequencies, and convert to rgb values. 
