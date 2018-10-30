#! /usr/bin/python

#  Author:  Blair Edwards 2018
#  The aim here is to plot a graph of ping results, to get an idea of overall latency
#  I'm mainly using it to test my internet, but it could be used for any host

#  TODO:  Account for Windows-ness and run a different ping command; plus how to RegEx?

##  Dependencies:
#  matplotlib


##  Imports
from time import sleep
#  Native RegEx package
import re
#  Shell interaction - subprocess is preferable to os nowadays, according to
#  https://docs.python.org/3/library/os.html#os.system
from subprocess import run
#  Using `matplotlib` for graphing functionality
import matplotlib .pyplot as plt

##  Variables
pingDelay = 0.5
#  Destination for the ping comamnd
pingDest = "1.1.1.1"
#  RegEx pattern to search for the ping time and capture the timing value
regexPat = re .compile ('time=([0-9]+\.?[0-9]*) ms')
#  Array to hold all the ping times
pingTimes = ()

for i in range (10):
	#  Run the command and capture the system's response
	pingResp = run (["ping", "-c 1", pingDest], capture_output = True)
	pingTimes += (float (regexPat .findall (str (pingResp .stdout)) [0]),)
	
	sleep (pingDelay)
	#  Print the resulting float :D
print (pingTimes)

#  Now we need a way to plot some kind of graph...
plt .plot ([1, 2, 3, 4])
plt .ylabel ("Let's go!")
plt .show ()
