#  Ping Graph
This is a simple python script to ping a destination and graph the returned timing results in real-time.
This is still _under development_ and is missing some stuff.
There's a `settings.json` file that stores the runtime configuration.

##  Dependencies
-  [Python 3](https://www.python.org/)
-  matplotlib _(available through pip)_

##  Running
This script will happily run through the Python 3 interpreter, given all the dependencies are met.
The easiest way to install these is using [`pip`](https://pypi.org/project/pip/).
For example, running `pip install matplotlib` will install the `matplotlib` module.

##  Making/Distributing
I've set up a build system for native distribution.  
**Note:**  I haven't tried this with anything other than windows.

###  Dependencies
-  pyinstaller _(available through pip)_
-  make _(optional)_

###  Method
If you've got `make` installed, just run `make`.  `make clean` will clean the build.  Otherwise:  
Simply running `pyinstaller pingGraph.py` will do the business for your machine, though adding `--onefile` is necessary to end up with a single, distributable binary.
Additionally, you'll need to copy `settings.json` to the same directory as the binary.
The commands I've used in the `makefile` (and that plays nicely with the `.gitignore` is:  
`pyinstaller --onefile --specpath build --distpath bin pingGraph.py`
