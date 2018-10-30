#! /usr/bin/python

#  Author:  Blair Edwards 2018
#  The aim here is to plot a graph of ping results, to get an idea of overall latency
#  I'm mainly using it to test my internet, but it could be used for any host

#  TODO:
#  Plot the graph in real-time
#  Cap graph length
#  Consider bar, histogram and plot -styles of graph
#  Better colours :D

##  Dependencies:
#  matplotlib



##  Imports
from time import sleep
from os import name
#  Native RegEx package
import re
#  Shell interaction - subprocess is preferable to os nowadays, according to
#  https://docs.python.org/3/library/os.html#os.system
from subprocess import run
#  Using `matplotlib` for graphing functionality
import matplotlib .pyplot as plt



##  Variables
#  Check which type of system we're running on and modify the
#    ping command and RegEx accordingly
if name == "nt":
	pingCountArg = "-n"
elif name == "posix":
	pingCountArg = "-c"
else:  #  Hope for the best...  Should die properly in future
	pingCountArg = "-c"

#  Ping Variables
pingDelay = 0.5
pingCount = 100
#  Destination for the ping comamnd
pingDest = "1.1.1.1"
#  RegEx pattern to search for the ping time and capture the timing value
regexPat = re .compile ('time=([0-9]+\.?[0-9]*) ?ms')
#  Array to hold all the ping times
pingTimes = ()



##  Main Programme

#  Set up the graph
plt .ion ()
plt .xlabel ("Pings")
plt .ylabel ("Time (ms)")
plt .show ()

#  Consecutively perform pings and plot the results
for i in range (pingCount):
	#  Run the command and capture the system's response
	pingResp = run (["ping", pingCountArg, "1", pingDest], capture_output = True)
	#  Extract the timing data and re-build the tuple
	pingTimes = pingTimes + (float (regexPat .findall (str (pingResp .stdout)) [0]),)

	#  Re-plot the graph
	plt .plot (pingTimes, 'b', linewidth = 1)
	plt .draw ()
	plt .pause (0.001)

	sleep (pingDelay)

input ("Press [enter] to exit...")
