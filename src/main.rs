extern crate printwrap;
//use std::{ffi::OsStr,process,env,fs,path::Path};
use std::{process,env,fs,path::Path};
use regex::Regex;
use std::process::ExitCode;

fn usage()
{
	printwrap::print_wrap(5,0,"Usage:");
	printwrap::print_wrap(5,0,"    rename <from> <to> <path> [<path> ...]");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"Rename will rename all occurences of <from> to <to> in the file names of each file for which the path is provided. Rename will *only* modify the final part of the path itself. Any preceeding directories will not be changed. By default <from> is interpreted as a literal string. To use a regular expression, use the -r option. Rename will not replace an existing file.");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"Rename will return 0 (zero) if any files were successfully renamed, and 1 otherwise.");
	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"For example:");
	printwrap::print_wrap(10,0,"rename less more /some/lesser/path/to/a/file/called/more-or-less");
	printwrap::print_wrap(8,0,"will result in the new name:");
	printwrap::print_wrap(10,0,"/some/lesser/path/to/a/file/called/more-or-more");
	printwrap::print_wrap(8,0,"and");
	printwrap::print_wrap(10,0,"rename -r \"or[a-z] uch /some/lesser/path/to/a/file/called/more-or-less");
	printwrap::print_wrap(8,0,"will result in the new name:");
	printwrap::print_wrap(10,0,"/some/lesser/path/to/a/file/called/much-or-less	");

	printwrap::print_wrap(5,0,"");
	printwrap::print_wrap(5,0,"Options:");
	printwrap::print_wrap(10,24,"-h | --help  This usage information.");
	printwrap::print_wrap(10,24,"-r           Indicates the <from> parmaeter will be a regular expression. The standard regular expression must be compatible with the rust implementation of regular expressions.");

	printwrap::print_wrap(10,24,"-v           Verbose. Print more verbose messages as rename runs.");
	process::exit(1);
}

fn rename(from:&String, to:&String, is_regex:bool, verbose:bool, file:&Path) -> u32
{
	let mut success=0;

	if file.exists()
	{
		let parent = file.parent().unwrap().to_str().unwrap();
		let filename = file.file_name().unwrap().to_str().unwrap();

		let newfilename:String;
		if is_regex
		{
			let regex = Regex::new(from).unwrap();
			newfilename=String::from(regex.replace_all(filename,to)); 
		}
		else
		{
			newfilename = filename.replace(from, to);
		}
		if newfilename != filename
		{
			let newpath;
			if parent != ""
			{
				newpath=format!("{}/{}", parent, newfilename);
			}
			else
			{
				newpath=newfilename;
			}
			if Path::new(newpath.as_str()).exists()
			{
				println!("ERROR: Cannot rename \"{}\" to existing file \"{}\"", file.display(), newpath);
			}
			else
			{
				match fs::rename(file, newpath.as_str())
				{
					Err(error) => {println!("ERROR: Could not rename file \"{}\" to \"{}\"\n{}", file.display(), newpath,error);process::exit(1)},
					Ok(file) => file,
				}
				if verbose
				{
					success=1;
					println!("\"{}\" => \"{}\"", file.display(), newpath);
				}
			}
		}
	}
	else
	{
		println!("File \"{}\" does not exist or is not accessible.", file.display());
	}
	return success;
}

fn main() -> ExitCode
{
	let args: Vec<String> = env::args().collect();
	let start=1;
	let end=args.len();
	let mut regex = false;
	let mut verbose = false;
	let mut from:String=String::from("");
	let mut to:String=String::from("");
	let mut bfrom=false;
	let mut bto=false;
	let mut renamed=0;
	let mut enough_parameters=false;

	if end < 4 // args +1 including arg 0
	{
	}

	for i in start..end
	{
		match args[i].as_ref()
		{
			"-h" | "--help" =>
				{
				usage();
				}
			"-r" =>
				{
					// regex
					regex=true;
				}
			"-v" =>
				{
					verbose=true;
				}
			_ =>
				{
					if !bfrom
					{
						from = String::from(&args[i]);
						bfrom=true;
					}
					else if !bto
					{
						to = String::from(&args[i]);
						bto=true;
					}
					else
					{
						// file to process
						let file = Path::new(&args[i]);
						enough_parameters=true;
						renamed=renamed + rename(&from, &to, regex, verbose, file);
					}
				}
		}
	}
	if ! enough_parameters
	{
		println!("Not enough parameters provided to rename:");
		if !bfrom
		{
			println!("No \"from\" value provided.")
		}
		if !bto
		{
			println!("No \"to\" value provided.")
		}
		if bfrom && bto
		{
			println!("No file-to-rename provided.")
		}
		println!("rename -h for more information.");
	}
	if verbose
	{
		println!("Files renamed:{}", renamed);
	}
	if renamed > 0
	{
		return ExitCode::SUCCESS;
	}
	return ExitCode::FAILURE
}
