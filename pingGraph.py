#! /usr/bin/python

#  Author:  Blair Edwards 2018
#  The aim here is to plot a graph of ping results, to get an idea of overall latency
#  I'm mainly using it to test my internet, but it could be used for any host

#  TODO:  Account for Windows-ness and run a different ping command; plus how to RegEx?

##  Imports
from time import sleep
#  subprocess is preferable to os nowadays, according to
#  https://docs.python.org/3/library/os.html#os.system
#  We're using `Popen ()` instead of `run ()` so we can pipe between `ping` and `grep`
#  Turns out there's a native RegEx package :|
#  So reverting back to `run ()`
import re
from subprocess import run
#  Using `matplotlib` for graphing functionality

#  Destination for the ping comamnd
pingDest = "1.1.1.1"
#  grep parses RegEx a bit differently than I'm used to, but this seems to work great
#grepExp = 'time=[0-9]\{1,\}\.\{0,1\}[0-9]\{1,\} ms'
#  "normal" RegEx pattern to search for the ping time and capture the timing value
regexPat = re .compile ('time=([0-9]+\.?[0-9]*) ms')

for i in range (10):
	#  Run the command and capture the system's response
	pingResp = run (["ping", "-c 1", pingDest], capture_output = True)
	#regexResponse = Popen (["grep", "-o", grepExp], stdin = pingResponse .stdout, stdout = PIPE)
	#pingTime = float (regexResponse .stdout .read () [5:-4])
	pingRespStr = str (pingResp .stdout)
	regexRes = regexPat .findall (pingRespStr)
	pingTime = float (regexRes [0])
	sleep (0.5)
	
	#  Print the resulting float :D
	print (pingTime)

#  Now we need a way to plot some kind of graph...
