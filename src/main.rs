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
use std ::str ::FromStr;
use std ::borrow ::Cow;
use std ::time ::Duration;
//  From Crates.io
use json;
use chrono;
use ping_rs;
use colored ::*;

//  Global Constants
const PATH_PREFS: &str = "./settings.json";
const PATH_LOGS_DIR: &str = "./data";
const PATH_DATA_PREFIX: &str = "pingResults-";
const PATH_DATA_EXT: &str = ".csv";

//  Global Variables

//  Local Constants

//  Local Variables

//  Structures
struct Settings
{
	dateStart:  chrono ::DateTime <chrono ::Local>,
	pingDelay:  Duration,			//  10000
	pingCount:  u64,				//  5
	pingDest:  std ::net ::IpAddr,	//  1.1.1.1
	pingTimeout:  Duration,			//  1000
	pingBuffer:  u64,				//  1000
	pingUpperBound:  u64,			//  200
	graphTitle:  String,			//  "Ping Results"
	graphXLabel:  String,			//  "Pings"
	graphYLabel:  String,			//  "Time (ms)"
	colourFail:  [u8; 4],			//  FB092CFF
	colourUpper:  [u8; 4],			//  FD9050FF
	colourPass:  [u8; 4],			//  FFFFFFFF
	colourBackground:  u32,			//  1E2028FF
	colourBox:  u32,				//  FFFFFFFF
	colourMajDivs:  u32,			//  FFFFFFFF
	colourPlotline:  u32,			//  19C4F1FF
	colourTitle:  u32,				//  46E415FF
	colourLabels:  u32,				//  FD9050FF
	colourNumbering:  u32,			//  8572F8FF
}

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

# [allow (unreachable_code)]
fn main ()
{
	//  Read & parse the settings JSON from disk.
	let settings = settingsRead (PATH_PREFS);

	//  Create & open the output file.
	let mut logFile = logOpen ();
	//  Write the header regarding this test.
	writeHeader (&mut logFile, &settings);

	//  Core Loop
	//#  Capture ctrl+c.
	//  Determine whether to run indefinitely, or how many pings to do if not.
	if (settings .pingCount == 0)
	{
		loop
		{
			handlePing (&mut logFile, &settings);
		}
	}
	else
	{
		for _ in 0 .. settings .pingCount
		{
			handlePing (&mut logFile, &settings);
		}
	}

	//  Wrap everything up.
	//  No need to close the file - this will happen automatically once the handle goes out-of-scope.
}

///  Handle a ping.
fn handlePing (logFile: &mut File, settings: &Settings)
{
	//  Perform the ping.
	let data = [8; 8];
	let res = ping_rs ::send_ping (&settings .pingDest, settings .pingTimeout, &data, None);

	//  Handle the results.
	writeResult (logFile, settings, &res);
	graphResult (settings, &res);
	std ::thread ::sleep (settings .pingDelay);
}


///  Read the settings JSON from disk at the location specified.
fn settingsRead (pathToOpen: &str) -> Settings
{
	//#  Generate a new file if the existing one is corrupt / non-existant.
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
	let parsed = json ::parse (&settingsStr) .unwrap ();
	let testStr = parsed ["pingSettings"] ["pingDest"] .as_str () .unwrap () .to_owned ();
	let colourUpperStr = parsed ["colourSettings"] ["colourUpper"] .as_str () .unwrap ();
	let settings = Settings
	{
		dateStart:  chrono ::Local ::now (),
		pingDelay:  Duration ::from_millis (parsed ["pingSettings"] ["pingDelay"] .as_u64 () .unwrap ()),
		pingCount:  parsed ["pingSettings"] ["pingCount"] .as_u64 () .unwrap (),
		pingDest:  std ::net ::IpAddr ::from_str (parsed ["pingSettings"] ["pingDest"] .as_str () .unwrap ()) .unwrap (),
		pingTimeout:  Duration ::from_millis (parsed ["pingSettings"] ["pingTimeout"] .as_u64 () .unwrap ()),
		pingBuffer:  parsed ["pingSettings"] ["pingBuffer"] .as_u64 () .unwrap (),
		pingUpperBound:  parsed ["pingSettings"] ["pingUpperBound"] .as_u64 () .unwrap (),
		graphTitle:  parsed ["graphText"] ["graphTitle"] .as_str () .unwrap () .to_owned (),
		graphXLabel:  parsed ["graphText"] ["graphXLabel"] .as_str () .unwrap () .to_owned (),
		graphYLabel:  parsed ["graphText"] ["graphYLabel"] .as_str () .unwrap () .to_owned (),
		colourFail:  u32 ::from_str_radix (parsed ["colourSettings"] ["colourFail"] .as_str () .unwrap (), 16) .unwrap () .to_be_bytes (),
		colourUpper:  u32 ::from_str_radix (parsed ["colourSettings"] ["colourUpper"] .as_str () .unwrap (), 16) .unwrap () .to_be_bytes (),
		colourPass:  u32 ::from_str_radix (parsed ["colourSettings"] ["colourPass"] .as_str () .unwrap (), 16) .unwrap () .to_be_bytes (),
		colourBackground:  u32 ::from_str_radix (parsed ["colourSettings"] ["colourBackground"] .as_str () .unwrap (), 16) .unwrap (),
		colourBox:  u32 ::from_str_radix (parsed ["colourSettings"] ["colourBox"] .as_str () .unwrap (), 16) .unwrap (),
		colourMajDivs:  u32 ::from_str_radix (parsed ["colourSettings"] ["colourMajDivs"] .as_str () .unwrap (), 16) .unwrap (),
		colourPlotline:  u32 ::from_str_radix (parsed ["colourSettings"] ["colourPlotline"] .as_str () .unwrap (), 16) .unwrap (),
		colourTitle:  u32 ::from_str_radix (parsed ["colourSettings"] ["colourTitle"] .as_str () .unwrap (), 16) .unwrap (),
		colourLabels:  u32 ::from_str_radix (parsed ["colourSettings"] ["colourLabels"] .as_str () .unwrap (), 16) .unwrap (),
		colourNumbering:  u32 ::from_str_radix (parsed ["colourSettings"] ["colourNumbering"] .as_str () .unwrap (), 16) .unwrap (),
	};
	return settings;
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
fn writeHeader (logFile: &mut File, settings: &Settings)
{
	logFile .write_all ("-=-  Ping Graph  -=-\n" .as_bytes ()) .unwrap ();
	logFile .write_all (format! ("-=>  Started:  {}\n", settings .dateStart .format ("%Y-%m-%d_%H-%M-%S")) .as_bytes ()) .unwrap ();
	logFile .write_all (format! ("-=>  Address:  {}\n", settings .pingDest) .as_bytes ()) .unwrap ();
	logFile .write_all (format! ("-=>  Period:  {}s\n", settings .pingDelay .as_secs ()) .as_bytes ()) .unwrap ();
	logFile .write_all (format! ("-=>  Timeout:  {}ms\n\n", settings .pingTimeout .as_millis ()) .as_bytes ()) .unwrap ();
}

enum ResCond {Fail, Upper, Pass}
///  Write a ping result to the given file.
fn writeResult (logFile: &mut File, settings: &Settings, result: &Result <ping_rs ::PingReply, ping_rs ::PingError>)
{
	//  Generate the timestamp.
	let dateString = chrono ::Local ::now () .format ("%Y-%m-%d_%H-%M-%S");

	//  Determine the condition of the result.
	let resCond: ResCond = match result
	{
		Err (error) => ResCond ::Fail,
		Ok (res) => match (u64 ::from (res .rtt))
		{
			x if x > settings .pingUpperBound => ResCond ::Upper,
			_ => ResCond ::Pass,
		},
	};

	//  Format the results for printing.
	let logLine: String;
	match result
	{
		Err (error) => logLine = format! ("[{}]:  <F>  {:?}\n", dateString, error),
		Ok (res) => match resCond
		{
			ResCond ::Fail => unreachable! ("Error!  'result' == Ok (), but 'resCond' == Fail!"),
			ResCond ::Upper => logLine = format! ("[{}]:  <U>  {} - {}ms\n", dateString, res .address, res .rtt),
			ResCond ::Pass => logLine = format! ("[{}]:  <P>  {} - {}ms\n", dateString, res .address, res .rtt),
		}
	};

	//  Write the results to the console and log file.
	//std ::io ::stdout () .write (logLine .as_bytes ()) .unwrap (),
	match resCond
	{
		ResCond ::Fail => std ::io ::stdout () .write (format! ("{}", logLine .truecolor
			(settings .colourFail [0], settings .colourFail [1], settings .colourFail [2]))
			.as_bytes ()) .unwrap (),
		ResCond ::Upper => std ::io ::stdout () .write (format! ("{}", logLine .truecolor
			(settings .colourUpper [0], settings .colourUpper [1], settings .colourUpper [2]))
			.as_bytes ()) .unwrap (),
		ResCond ::Pass => std ::io ::stdout () .write (logLine .as_bytes ()) .unwrap (),
	};
	logFile .write_all (logLine .as_bytes ()) .unwrap ();
}

///  Graph a ping result.
fn graphResult (settings: &Settings, result: &Result <ping_rs ::PingReply, ping_rs ::PingError>)
{
	//  Get the total elapsed time since starting the pings.
	let elapsed = (chrono ::Local ::now () - settings .dateStart) .num_milliseconds ();
	//  Determine whether to plot a successful or failed ping.
	/*match result
	{
		Err () => plot (0),
		Ok () => plot (1),
	}*/
	return;
}

//  *--</Main Code>--*  //



//  *--<Callbacks>--*  //

//  *--</Callbacks>--*  //
