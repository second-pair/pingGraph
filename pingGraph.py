#! /usr/bin/python

#  Author:  Blair Edwards 2018
#  The aim here is to plot a graph of ping results, to get an idea of overall latency
#  I'm mainly using it to test my internet, but it could be used for any host

#  TODO:  Account for Windows-ness and run a different ping command; plus how to RegEx?

import os

#  Prime the system comamnd
hostname = "1.1.1.1"
#  grep parses RegEx a bit differently than I'm used to, but this seems to work great
grepExp = 'time=[0-9]\{1,\}\.\{0,1\}[0-9]\{1,\} ms'

#  Run the command and capture the system's response
response = os .system ('ping -c 1 ' + hostname + ' | grep "' + grepExp + '"')

print (response)
