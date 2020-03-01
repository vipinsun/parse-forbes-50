//Parses the forbes50 article expressed as a text file and creates a csv file from it
//Each section in the input (which is a text file converted from the article) 
//is parsed and a row created. A row per section.
//The section boundaries are —\r\n
//There are some generalized functions for parsing, uses serde and csv crate
//fs opening and closing
//nom crate for some other stuff


extern crate csv;
#[macro_use]
extern crate nom;
extern crate serde_derive;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use csv::WriterBuilder;
//use std::io::prelude::*;

//use std::error::Error;
//use std::process;
// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Proj {
    company: String,
    location: String,
    description: String,
    keyleader: String,
	platform: String,
}

named!(get_keyleader<&str,&str>,
		ws!(
			alt!(
					
					tag!("Key leader: ") | tag!("Key leadership: ") | tag!("Key Executives: ") | tag!("Key Executive: ")
					
			)
		)
);
named!(get_bcp<&str,&str>,
	ws!(
    	alt!(
			tag!("Blockchain platforms: ") | tag!("Blockchain platform: ")| tag!("Blockchain: ")
		)
	)
);

fn outputcsv(outfilename: &str, projs: &Vec<&str>) -> io::Result<()> {
	let _ofile = File::create(&outfilename)?;
	//Now we parse the projs and create columns in Proj (which is the Serde_Record)
		let mut wtr = WriterBuilder::new()
		.terminator(csv::Terminator::CRLF)
		.from_path(outfilename)?;

	//println!("size of projs {}", projs.len());
	for i in projs{
		let intv: Vec<&str> = i.split("\r\n")
							  .filter(|s| s.trim().len() >0) //get rid of blank lines
							  .collect();
		let company: String = intv[0].to_string();
		let location: String = intv[1].to_string();
		let description: String = intv[2].to_string();
//Blockchain platforms: Blockchain:
	    let platform = match get_bcp(intv[3]) { //remove hard coding
        	Ok(s1) => s1.0.to_string(),
        	Err(_s) => ("").to_string(),
    	};
	//Key leader: Executive:
	    let keyleader =  match get_keyleader(intv[4]) { //remove 
        	Ok(s1) => s1.0.to_string(),
        	Err(_s) => ("").to_string(),
    	};

		wtr.serialize(Proj {
				company: company,
				location: location,
				description: description,
				platform: platform,
				keyleader: keyleader,
			})?;
				//let mut proj=new Proj;

/*  company: String, //First line to EOL
    location: String, //Second line to EOL
    description: String,//From second line to EOL 
    keyleader: String, //Key leader: to EOL
	platform: String, //Blockchain platforms: to EOL then all below.
	platform1: Option <String>,
	platform2: Option <String>,
	platform3: Option <String>,
	platform4: Option <String>,
	*/
		
	}
	wtr.flush()?;
	Ok(())
}

//Take @infilename, open and read it into a string (type is inferred from first usage) 
// Projects is a vector of strings that results
//outputcsv is a function that generates the outf from the projects vector
//
//The @splitstr contains the separator between projects
//The @outf contains the name of the outputfile
//
// Errors result in io::Result<()>
//suggestions for improvement
// Have functions that just read and return the string
// Or just a vector of projects
// The operations on this can be generalized removing hardcoding

fn chop_parsely(infilename: &str, splitstr: &str, outf: &str) ->  io::Result<()> {
    //let file = File::open(&infilename)?;

    //let reader = io::BufReader::new(file);

	//let data: Vec<u32> = data_str.lines_any().filter_map(|s| s.trim().parse()).collect();
    let instr = fs::read_to_string(&infilename)?;
    let projects: Vec<&str> = instr.split(splitstr).collect();
	outputcsv(outf, &projects)
    //instr.split(splitchar);
    
//    Ok(())
}
fn usage (){
	println!("Please run as ParseForbes inputfile outputfile")
}

fn main() {
	let argsv: Vec<String> = env::args().skip(1).collect();
	let mut infilename="".to_string();
	let mut outfilename="".to_string();
    if argsv.len() > 0 {
		println!{"'{:?}'", argsv}; 
		for arg in &argsv {
			println!("'{}'", arg);
		}
		
		match argsv.get(0) {
			Some(x) =>  infilename=x.to_string(),
			None => {println!("There is no input filename"); usage();},
			
		}
		
		match argsv.get(1) {
			Some(x) =>  outfilename=x.to_string(),
			None => {println!("There is no output filename"); usage();},
			
		}
		
	    let _res=chop_parsely(&infilename, "—\r\n", &outfilename);
	    
	}
    else 
    {
		usage()
	}
}
