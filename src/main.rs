/*  *--<Preface>--*  //

 -=-  Author Details  -=-
 Blair Edwards
 Just 'cause I felt like it.

 -=-  Dates  -=-
 Started 2022-07-30

 -=-  Description  -=-
 This programme attempts to perform continuous network monitoring by repeatedly pinging a destination and recording the results in a CSV file.  An optional real-time graph will also be created with the live results.
 Input is through configuration of the 'settings.json' file and through ICMP Ping responses.
 Output is text through STDOUT, file IO, ICMP Ping requests and through a GUI graph.

 -=-  Task  -=-
 -=>  Implement JSON reading & read the programme's preferences from a JSON file.
 -=>  Continuously ping as many resources as requested.
 -=>  Log the results in a CSV file.
 -=>  Graph the results.

 -=-  Notes  -=-
 -=>  I've developed my own commenting notation for things that "aren't done" one way or another.  Such as:
	 -  //#  TODO
	 -  //?  Not sure / query
	 -  //!  Important note / relevant as technology advances
 -=>  Logging with `_LOG ()` takes a 'logLevel' argument, which roughly indicates:
	 -  0:  Critical, major errors, should Always be printed.
	 -  1:	Important info particularly critical functions, minor / user errors.
	 -  2:	Useful info / general programme flow.
	 -  3:	Debug info, steps throughout a function.
	 -  4:  Useful spam - printed often such as in a loop.
	 -  5:  Debug spam - printed often such as in a loop.
 -=>

//  *--</Preface>--*  */



//  *--<Preparations>--*  //

//  Local Compiler Pragmas
#! [allow (unused_variables)]
#! [allow (non_snake_case)]
#! [allow (unused_parens)]
#! [allow (dead_code)]
#! [allow (unused_imports)]

//  Imports
//use std :: io ::{stdin, stdout, Write};
//use std ::time;
use std ::fs;
use std ::fs ::File;
use std ::io ::prelude;
use std ::io ::Read;
use std ::io ::Write;
use std ::path ::Path;
use chrono;

//  Global Constants
const PATH_PREFS: &str = "./settings.json";
const PATH_LOGS_DIR: &str = "./data";
const PATH_DATA_PREFIX: &str = "pingResults-";
const PATH_DATA_EXT: &str = ".csv";

//  Global Variables

//  Local Constants

//  Local Variables

//  Structures

//  *--</Preparations>--*  //



//  *--<Macros>--*  //

//  TODO:  implement macros.rs.
/*macro_rules! macroName
{
	($a: expr, $b: expr) =>
	{
		$a + $b
	}
	($a: expr) =>
	{
		$a
	}
}*/

//  *--</Macros>--*  //



//  *--<Main Code>--*  //

fn main ()
{
	//  Read & parse the settings JSON from disk.
	let settingsJson = settingsRead (PATH_PREFS);
	print! ("{:#}\n", settingsJson);
	//  Create & open the output file.
	let mut logFile = logOpen ();
	//  Write the header regarding this test.
	writeHeader (&mut logFile);

	//  Core Loop
	let sleep_dur = std ::time ::Duration ::from_millis (250);
	//#  Capture ctrl+c.
	//loop
	for _ in 0 .. 10
	{
		writeResult (&mut logFile);
		std ::thread ::sleep (sleep_dur);
	}

	//  Wrap everything up.
	//  No need to close the file - this will happen automatically once the handle goes out-of-scope.
}


///  Read the settings JSON from disk at the location specified.
fn settingsRead (pathToOpen: &str) -> json ::JsonValue
{
	//  Load the preferences JSON.
	let settingsPath = Path ::new (pathToOpen);
	print! ("Reading settings from:  {}\n", settingsPath .display ());

	//  Open the passed file in read-only mode.
	let mut settingsFile = match File ::open (&settingsPath)
	{
		Err (reason) => panic! ("Failed to open {}:  {}", settingsPath .display (), reason),
		Ok (file) => file,
	};

	//  Read the JSON into a String.
	let mut settingsStr = String ::new ();
	match settingsFile .read_to_string (&mut settingsStr)
	{
		Err (reason) => panic! ("Failed to read {}:  {}", settingsPath .display (), reason),
		Ok (_) => print! ("Read successful.\n"),
	}
	//  Parse & return the JSON.
	return json ::parse (&settingsStr) .unwrap ();
}

///  Open a new log file, which is automatically named using the current date.
fn logOpen () -> File  //  Return a file-buffer?
{
	//  Create the data directory if it doesn't exist.
	match std ::fs ::create_dir (PATH_LOGS_DIR)
	{
		Err (reason) if reason .kind () == std ::io ::ErrorKind ::AlreadyExists => (),
		Err (reason) => panic! ("Could not create directory {}:  {}", PATH_LOGS_DIR, reason),
		Ok (_) => (),
	}
	//  Construct the date-string.
	let dateString = chrono ::Local ::now () .format ("%Y-%m-%d_%H-%M-%S");
	//  Create the logfile path.
	let logPathStr = format! ("{}/{}{}{}", PATH_LOGS_DIR, PATH_DATA_PREFIX, dateString, PATH_DATA_EXT);
	let logPath = Path ::new (&logPathStr);
	print! ("Creating log file:  {}\n", logPath .display ());

	//  Open the passed file in write-only mode.
	let logFile = match File ::create (&logPath)
	{
		Err (reason) => panic! ("Failed to open {}:  {}", logPath .display (), reason),
		Ok (file) => file,
	};

	return logFile;
}

///  Write the test header to the given file.
fn writeHeader (logFile: &mut File)
{
	logFile .write_all ("-=-  Ping Graph  -=-\n" .as_bytes ()) .unwrap ();
	logFile .write_all ("-=>  Address:  127.0.0.1\n" .as_bytes ()) .unwrap ();
	logFile .write_all ("-=>  Period:  10s\n\n" .as_bytes ()) .unwrap ();
}

///  Write a ping result to the given file.
fn writeResult (logFile: &mut File)
{
	let logLine = format! ("<pings>\n");
	std ::io ::stdout () .write(logLine .as_bytes ()) .unwrap ();
	logFile .write_all (logLine .as_bytes ()) .unwrap ();
}

//  *--</Main Code>--*  //



//  *--<Callbacks>--*  //

//  *--</Callbacks>--*  //
