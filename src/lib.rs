/// copernicusmarine_rs.
/// Author: G0rocks
/// Date: 2025-06-04
/// Description
/// Enables the use of copernicus marine toolbox through rust

// Todo
//-------------------------------------------------------------------------------------------------------------------------
// 1. Add way to chose where the files are stored and fetched. Use absolute or relative path



// Dependencies
//-------------------------------------------------------------------------------------------------------------------------
use time;   // For start and end times
use std::thread; // For sleeping between attempts to get data from the copernicus marine servers
use core::panic;    // For panicking when something goes wrong
// If windows
#[cfg(target_os = "windows")]
use std::os::windows::process::ExitStatusExt; // To get exit code from commands
// If unix
#[cfg(target_os = "linux")]
use std::os::unix::process::ExitStatusExt; // To get exit code from commands

use std::process::Command; // To run commands through the command line
use netcdf; // For working with and using the netcdf files retrieved from the copernicus server

// Definitions
//-------------------------------------------------------------------------------------------------------------------------
/// Maximum number of attempts to get data from the copernicus marine servers
// const MAX_ATTEMPTS: u16 = 1000;

// Enums
//-------------------------------------------------------------------------------------------------------------------------


// Structs
//-------------------------------------------------------------------------------------------------------------------------
/// A struct that keeps the copernicus relevant info, like the output path. This way when we run functions we can also get this information for those function but only need to set this info once instead of passing it everytime
#[derive(Debug, Clone)]
pub struct Copernicus {
    /// The output path where the files are stored and retrieved
    pub output_path: String,
}

// Functions
//-------------------------------------------------------------------------------------------------------------------------
impl Copernicus {
    pub fn new(output_path: String) -> Copernicus {
        Copernicus {
            output_path: output_path
        }
    }

    /// Function to get a subset from the copernicus database using the copernicus marine toolbox
    /// This is the function you use to get some specific data.
    /// Note even though you put many variables, only the first variable is used in the file name
    pub fn subset(
        &self,
        dataset_id: String,
        variables: Vec<String>,
        start_datetime: time::UtcDateTime,
        end_datetime: time::UtcDateTime,
        minimum_longitude: f64,
        maximum_longitude: f64,
        minimum_latitude: f64,
        maximum_latitude: f64) -> netcdf::File {

        // Validate that minimum longitude and latitude is less or equal to maximum
        if minimum_latitude > maximum_latitude {
            panic!("Minimum latitude must be less than or equal to maximum latitude");
        }
        if minimum_longitude > maximum_longitude {
            panic!("Minimum longitude must be less than or equal to maximum longitude");
        }

        // println!("Getting copernicus subset");

        
        // Make argument
        let mut args: Vec<String> = vec![
            "subset".to_string(),
            "--dataset-id".to_string(),
            dataset_id.clone()];

        // Make variable argument
        // Loop and for each variable in variables, add "--variable " + variable[i].to_string();
        for variable in variables.clone() {
            args.push("--variable".to_string());
            args.push(variable);
        }

        args.push("--start-datetime".to_string());
        args.push(utc_date_time_to_string(start_datetime));
        args.push("--end-datetime".to_string());
        args.push(utc_date_time_to_string(end_datetime));
        args.push("--minimum-longitude".to_string());
        args.push(minimum_longitude.to_string());
        args.push("--maximum-longitude".to_string());
        args.push(5000.to_string());
        args.push("--minimum-latitude".to_string());
        args.push(minimum_latitude.to_string());
        args.push("--maximum-latitude".to_string());
        args.push(maximum_latitude.to_string());
        args.push("--output-directory".to_string());
        args.push(self.output_path.clone());
        args.push("--overwrite".to_string());   // To overwrite file if it already exists to conserve computer space
        
        
        // File name
        let mut filename = String::new();
        filename.push_str(&string_first_n_chars(dataset_id, 5));
        filename.push('_');
        filename.push_str(&variables[0]);
        filename.push('_');
        filename.push_str(&string_first_n_chars(minimum_latitude.to_string(), 5));
        filename.push('_');
        filename.push_str(&string_first_n_chars(maximum_latitude.to_string(), 5));
        filename.push('_');
        filename.push_str(&string_first_n_chars(minimum_longitude.to_string(), 5));
        filename.push('_');
        filename.push_str(&&string_first_n_chars(maximum_longitude.to_string(), 5));
        filename.push('_');

        // Date for filename
        let datestring = utc_date_time_to_string(start_datetime);
        filename.push_str(&datestring);

        // Make filepath
        // let filepath = self.output_path.as_str().to_owned() + "/" + &filename;

        // set output filename
        args.push("--output-filename".to_string());
        args.push(filename.clone());

        // println!("Querying server");
        // Run the command to query server for the data
        // If fails, retry 2 more times
        // Todo: If file alredy exists, overwrite or similar
        // Init output so it exists outside of the loop
        let mut output: std::process::Output = std::process::Output {
            status: std::process::ExitStatus::from_raw(100),
            stdout: vec![],
            stderr: vec!["Initialzed output for copernicusmarine toolbox subset command. If you see this something went wrong in an unpredicted way.".as_bytes().to_vec()].concat(),
        };
        let mut attempt_counter = 0;
        loop {
            attempt_counter += 1;
        // for i in 0..MAX_ATTEMPTS {
            // Running command
            let start_time = time::UtcDateTime::now();
            output = Command::new("copernicusmarine")
                .args(&args)
                .output()
                .expect(format!("Failed to execute the given command\ncopernicusmarine with arguments: {:?}", args).as_str());

            // Get exit code, if no exit code, set to -1 and assume failure
            let exit_code = match output.status.code() {
                Some(code) => code,
                None => -1,
            };

            // If the command was successful, break the loop
            if exit_code == 0 {
                break;
            }
            let end_time = time::UtcDateTime::now();
            let duration = end_time - start_time;
            // Print error message
            println!("Error getting data from copernicusmarine toolbox subset command, attempt {}. Query finished in {:?}. Exit code: {}. Output message: {}", attempt_counter, duration, exit_code, String::from_utf8_lossy(&output.stderr));
            // println!("Error getting data from copernicusmarine toolbox subset command, attempt {}/{}. Exit code: {}. Query finished in {:?}", i+1, MAX_ATTEMPTS, exit_code, duration);

            // Before trying again, wait 1 minute as per instructions from the devs: https://github.com/mercator-ocean/copernicus-marine-toolbox/issues/392#issuecomment-3136220183
            println!("Waiting {} seconds before trying again...", attempt_counter);
            thread::sleep(std::time::Duration::from_secs(attempt_counter));
        }

        // println!("Status: {}", _output.status.code().unwrap());
        // println!("Stdout: {}", String::from_utf8_lossy(&_output.stdout));
        // println!("Stderr: {}", String::from_utf8_lossy(&_output.stderr));

        // Move into output path directory
        let start_dir = std::env::current_dir().expect("Could not get current directory");
        std::env::set_current_dir(std::path::Path::new(&self.output_path)).expect("Error changing directories");
        // Find the file where the data is stored
        let nc_filename = filename.as_str().to_owned() + ".nc";

        // Get netcdf root for netcdf file
        let netcdf_file = netcdf::open(nc_filename.clone()).expect(format!("Could not get netcdf file:\n{}\ncopernicusmarinetoolbox subset error {}", &nc_filename, String::from_utf8_lossy(&output.stderr)).as_str());

        // Move back into starting directory
        std::env::set_current_dir(start_dir).expect("Error changing directories");

        // Return file
        return netcdf_file;
    }

}

// Helper functions
//-------------------------------------------------------------------------------------------------------------------------
/// Returns a substring of the first 5 characters of a string if there are more than 5 characters, otherwise returns the whole string
pub fn string_first_n_chars(string_in: String, num_chars: usize) -> String {
    // Get first 5 characters
    if string_in.chars().count() > num_chars {
        return string_in[..num_chars].to_string();
    }
    // Otherwise return whole input string
    return string_in;
}



/// Makes the seconds since 1990-01-01 00:00:00 into a time::UtcdDateTime value
/// Since according to datasheet the time is measured in seconds since that date: https://documentation.marine.copernicus.eu/PUM/CMEMS-WIND-PUM-012-004-006.pdf
pub fn secs_since_1990_01_01_0_to_utcdatetime(secs: i64) -> time::UtcDateTime {
    let start = time::UtcDateTime::new(time::Date::from_calendar_date(1990,time::Month::January,1).expect("Could not make time::Date"), time::Time::from_hms(0,0,0).expect("Could not make time::Time"));

    let dur = time::Duration::new(secs, 0);

    return start.checked_add(dur).expect("Could not add time::Duration to time::UtcDateTime");
}


/// Writes the datetime variable to the "YYYY-MM-DDTHH:MM:SS" format where the 'T' is literally just a 'T'
pub fn utc_date_time_to_string(datetime: time::UtcDateTime) -> String {
    // Init empty string
    let mut out_string = String::new();

    // Assemble string in correct format
    out_string = out_string + &datetime.year().to_string();
    out_string.push_str("-");
    // If month is less than 10, add a leading zero
    if (datetime.month() as u8) < 10 {
        out_string.push_str("0");
    }
    out_string = out_string + &(datetime.month() as u8).to_string();
    out_string.push_str("-");
    out_string = out_string + &datetime.day().to_string();
    out_string.push_str("T");

    out_string = out_string + &datetime.hour().to_string();
    out_string.push_str(":");
    out_string = out_string + &datetime.minute().to_string();
    out_string.push_str(":");
    out_string = out_string + &datetime.second().to_string();
    
    // Return output
    return out_string;
}


// Tests
//-------------------------------------------------------------------------------------------------------------------------
// #[cfg(test)]
// mod tests {
//     use super::*;
// 
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
