import numpy as np
from scipy.signal import correlate

def printArr(arr):
  out = ''
  for i in range(0, len(arr)-1):
    out += "{:.2f},".format(arr[i])
  out += "{:.2f}".format(arr[len(arr)-1])
  out = "vec!["+out+"]"

  print(out)

def test1():
  a = np.asarray([1, 2, 3, 4, 5])
  b = np.asarray([1.5829, 2.6502, 3.56592, 4.99956, 5.562856])
  res = correlate(a,b)
  printArr(res)

test1()



