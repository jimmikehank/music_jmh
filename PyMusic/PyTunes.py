def rooot(note):
    hs = 2**(1/12)
    notes = [27.5*hs**3]

    for i in range(1,12):
        notes.append(notes[i-1]*hs)

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
    note = [root]

    def keys_gen(note,steps,nkeys):
        for i in range(1,nkeys):
            note.append(note[i-1]*hs**steps[((i-1)%len(steps))])
        return note

    if form.lower() == 'maj':
        steps = [2,2,1,2,2,2,1]
        note = keys_gen(note,steps,nkeys)

    if form.lower() == 'harmin':
        steps = [2,1,2,2,1,3,1]
        note = keys_gen(note,steps,nkeys)

    if form.lower() == 'natmin':
        steps = [2,1,2,2,1,2,2]
        note = keys_gen(note,steps,nkeys)

    if form.lower() == 'pent':
        steps = [2,2,3,2,3]
        note = keys_gen(note,steps,nkeys)

    if form.lower() == 'minpent':
        steps = [3,2,2,3,2]
        note = keys_gen(note,steps,nkeys)

    if form.lower() == 'blues':
        steps = [3,2,1,1,3,2]
        note = keys_gen(note,steps,nkeys)

    return note
