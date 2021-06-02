from synthesizer import Player, Synthesizer, Waveform, Writer
from matplotlib import image as im
from PyTunes import keyboard, rooot
import numpy as np
import time

file = '/home/pi/Pictures/munch.jpeg'

image = im.imread(file)

rows = np.shape(image)[0]
col = np.shape(image)[1]



nkeys = 20

k = 0
chords_n = np.zeros([rows*col,3])

for i in range(rows):
    for j in range(col):
        rgb = image[i,j,:]
        chords_n[k,:] = np.around(rgb*((nkeys-1)/256))
        k = k+1
    
chords = np.zeros(np.shape(chords_n))
chord_old = np.zeros(3)
octave_shift = False

changes = [['E','min'],['A','maj']]
root = rooot(changes[0][0])*2
note = keyboard(root,nkeys,changes[0][1])
pos = 0


for i in range(np.shape(chords_n)[0]):
    for j in range(np.shape(chords_n)[1]):
        chords[i,j] = note[int(chords_n[i,j])]
        
        if chords[i,j] == chord_old[j] and octave_shift == True:
            chords[i,j] = chords[i,j]*(2**(7/12))
        else:
            k = 1
        
        chord_old[j] = chords[i,j]
        if i%16 == 0 and i != 0:
            if pos != len(changes)-1:
                pos = pos + 1
                note = keyboard(rooot(changes[pos][0]),nkeys,changes[pos][1])
            else:
                pos = 0
                note = keyboard(rooot(changes[pos][0]),nkeys,changes[pos][1])

pl = Player()
pl.open_stream()
synth = Synthesizer(osc1_waveform=Waveform.square, osc1_volume=0.2, use_osc2=False, osc2_waveform=Waveform.sawtooth, osc2_volume=0.1)

for i in range(np.shape(chords)[0]):
    chord = chords[i,:].tolist()
    print(chord)
    pl.play_wave(synth.generate_chord(chord,0.25))
    
