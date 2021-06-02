import numpy as np

def rooot(note):
    hs = 2**(1/12)
    notes = np.zeros(12)
    notes[0] = 27.5*hs**3
    
    for i in range(1,12):
        notes[i] = notes[i-1]*hs
        
    if note == 'C' or note == 'B#':
        freq = notes[0]
        
    elif note == 'C#' or note == 'Db':
        freq = notes[1]
        
    elif note == 'D':
        freq = notes[2]
    
    elif note == 'D#' or note == 'Eb':
        freq = notes[3]
        
    elif note == 'E':
        freq = notes[4]
    
    elif note == 'E#' or note == 'F':
        freq = notes[5]
        
    elif note == 'F#' or note == 'Gb':
        freq = notes[6]
        
    elif note == 'G':
        freq = notes[7]
        
    elif note == 'G#' or note == 'Ab':
        freq = notes[8]
        
    elif note == 'A':
        freq = notes[9]
        
    elif note == 'A#' or note == 'Bb':
        freq = notes[10]
        
    elif note == 'B' or note == 'Cb':
        freq = notes[11]        
        
    return freq

def keyboard(root,nkeys,form):
    hs = 2**(1/12)
    note = np.zeros(nkeys)
    note[0] = root
    if form.lower() == 'maj':
        steps = [2,2,1,2,2,2,1]
        for i in range(1,nkeys):            
            note[i] = note[i-1]*hs**steps[((i-1)%7)]
    if form.lower() == 'min':
        steps = [2,1,2,2,1,3,1]
        for i  in range(1,nkeys):
            note[i] = note[i-1]*hs**steps[((i-1)%7)]
            
    if form.lower() == 'pent':
        steps = [2,2,3,2,3]
        for i  in range(1,nkeys):
            note[i] = note[i-1]*hs**steps[((i-1)%5)]
            
    if form.lower() == 'minpent':
        steps = [3,2,2,3,2]
        for i  in range(1,nkeys):
            note[i] = note[i-1]*hs**steps[((i-1)%5)]   
            
    if form.lower() == 'blues':
        steps = [3,2,1,1,3,2]
        for i  in range(1,nkeys):
            note[i] = note[i-1]*hs**steps[((i-1)%6)]      
            
    return note