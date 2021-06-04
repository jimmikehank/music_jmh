#tunes2im Convert frequency data to beautiful artwork
from matplotlib import pyplot as plt
import numpy as np
from scipy.io import wavfile as wav
from scipy.fftpack import fft, fftfreq

[fs,data] = wav.read('/home/pi/Music/ff5_bigbridge.wav')

npixels = 625
sf = 44100

chunk_size = int(np.shape(data)[0]/npixels)
t = np.linspace(0,1,chunk_size)
chunk = np.zeros([npixels,chunk_size])
yf = np.zeros([npixels,chunk_size])
xf = np.zeros([npixels,chunk_size])

for i in range(npixels):
    datasel = range(i*chunk_size,((i*chunk_size)+chunk_size))
    chunk[i,:] = data[datasel,0]
    yf[i,:] = fft(chunk[i,:])
    
    xf[i,:] = fftfreq(chunk_size,1/sf)
    
plt.plot(t,xf[25,:])
plt.show()
