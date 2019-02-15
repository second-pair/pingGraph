#! /usr/bin/python

#  Author:  Blair Edwards 2018
#  The aim here is to plot a graph of ping results, to get an idea of overall latency
#  I'm mainly using it to test my internet, but it could be used for any host
#  To run ad infinitum, change 'pingCount' in 'settings.json" to 0

##  Config:
#  This script reads settings from `settings.json` in the current working directory

##  Dependencies:
#  matplotlib
#  pyinstaller (for make/installation only)



##  Native Imports
from time import sleep
from datetime import datetime
from os import name
#  Native RegEx package
from re import compile
#  Native JSON package
from json import load
#  Shell interaction - subprocess is preferable to os nowadays, according to
#  https://docs.python.org/3/library/os.html#os.system
from subprocess import run

##  Dependency Imports
#  Using `matplotlib` for graphing functionality
import matplotlib .pyplot as plt
import tkinter


##  Variables
#  Check which type of system we're running on and modify the
#    ping command and RegEx accordingly
if name == "nt":
	pingCountArg = "-n"
elif name == "posix":
	pingCountArg = "-c"
else:  #  Hope for the best...  Should die properly in future
	pingCountArg = "-c"

#  Import the JSON
with open ("settings.json", "r") as settingsFile:
	settings = load (settingsFile)

#  Ping Variables
pingDelay = settings ["pingSettings"]["pingDelay"]
pingCount = settings ["pingSettings"]["pingCount"]
pingBuffer = settings ["pingSettings"]["pingBuffer"]
#  Destination for the ping comamnd
pingDest = settings ["pingSettings"]["pingDest"]
#  Upper bound for an "acceptable" ping time
pingUpperBound = settings ["pingSettings"]["pingUpperBound"]
#  Array to hold all the ping times
pingTimes = ()

#  RegEx pattern to search for the ping time and capture the timing value
regexPat = compile (settings ["regexString"])

#  Graph Text
graphTitle = settings ["graphText"]["graphTitle"]
graphXLabel = settings ["graphText"]["graphXLabel"]
graphYLabel = settings ["graphText"]["graphYLabel"]

#  Colour Variables
colourBackground = settings ["colourSettings"]["colourBackground"]
colourBox = settings ["colourSettings"]["colourBox"]
colourMajDivs = settings ["colourSettings"]["colourMajDivs"]
colourPlotline = settings ["colourSettings"]["colourPlotline"]
colourTitle = settings ["colourSettings"]["colourTitle"]
colourLabels = settings ["colourSettings"]["colourLabels"]
colourNumbering = settings ["colourSettings"]["colourNumbering"]



##  Ping-Logging Function
def runThePingsPre ():
	#  Consecutively perform pings and plot the results
	if pingCount > 0:
		for _ in range (pingCount):
			runThePings ()
			sleep (pingDelay)
	elif pingCount == 0:
		while True:
			runThePings()
			sleep (pingDelay)
	else:
		print ("Error pingCount = %d." % pingCount)
		exit (1)
	return

def runThePings ():
	global pingTimes

	#  Run the command and capture the system's response
	pingResp = run (["ping", pingCountArg, "1", pingDest], capture_output = True)
	regexResp = regexPat .findall (str (pingResp .stdout))

	#  Extract the timing data
	if regexResp == []:
		thePingDatum = 0.0
	else:
		thePingDatum = float (regexResp [0])
	if thePingDatum == 0 or thePingDatum > pingUpperBound:
		print ("[%s]: Extreme ping value:  %dms" % (datetime .now (), thePingDatum))

	#  Re-build the timing tuple
	pingTimes = addPingDatum (pingTimes, thePingDatum)

	#  Re-plot the graph
	plt .plot (pingTimes, colourPlotline, linewidth = 1)
	updateTheGraph ()
	return



##  Function to add latest ping datum to graphing data
def addPingDatum (theData, theNewDatum):
	#  Check if we got a proper ping response
	#someCheckHere
	#  Add the new datum and return
	if len (theData) >= pingBuffer:
		return theData [len (theData) - pingBuffer + 1 :] + (theNewDatum,)
	else:
		return theData + (theNewDatum,)



##  Single function to update all the graph data
def updateTheGraph ():
	#  Draw and refresh the graph
	fig .canvas .draw ()
	fig .canvas .flush_events ()
	ax .clear ()

	#  Re-define all those colours
	ax .patch .set_facecolor (colourBackground)
	plt .title (graphTitle, color = colourTitle)
	plt .xlabel (graphXLabel, color = colourLabels)
	plt .ylabel (graphYLabel, color = colourLabels)
	for yEntry in range (0, 201, 50):
		plt .axhline (yEntry, color = colourMajDivs, linewidth = 1)

	#  Is it possible to conditionally highlight null-responses?



##  Main Programme

#  Determine DPI scaling
root = tkinter .Tk ()
vertRes = root .winfo_screenheight ()
root .destroy ()

#  Set up the graph's properties
fig = plt .figure (dpi = vertRes // 1080 * 100)
fig .patch .set_facecolor (colourBackground)
ax = fig .add_subplot (111)

#  Colours and stuff
plt .setp (ax .spines .values (), color = colourBox)
plt .tick_params (axis = 'x', colors = colourNumbering)
plt .tick_params (axis = 'y', colors = colourNumbering)
updateTheGraph ()

#  Bring the graph up for the first time
plt .ion ()
plt .show ()

#  Run the ping-logging function
runThePingsPre ()

input ("Press [enter] to exit...")
