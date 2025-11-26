/// copernicusmarine_rs.
/// Author: G0rocks
/// Date: 2025-06-04
/// Description
/// Enables the use of copernicus marine toolbox through rust

// Dependencies
//-------------------------------------------------------------------------------------------------------------------------
use time;   // For start and end times
use std::thread; // For sleeping between attempts to get data from the copernicus marine servers
use std::process::Command; // To run commands through the command line
use netcdf; // For working with and using the netcdf files retrieved from the copernicus server
use std::{io}; // To use errors

// Definitions
//-------------------------------------------------------------------------------------------------------------------------
/// Maximum number of attempts to get data from the copernicus marine servers
// const MAX_ATTEMPTS: u16 = 1000;

// Enums
//-------------------------------------------------------------------------------------------------------------------------
/// An enum that contains a list of Copernicus variables. For use with the get_dataset_id function
/// Note to developers when adding more variables, the variables are in alphabetical order within their own category (see comments) which are also in alphabetical order
#[derive(Debug, Clone)]
pub enum CopernicusVariable {
    // Ocean Current
    EastwardSeaWaterVelocity,
    NorthwardSeaWaterVelocity,
    // Wind
    EastwardWind,
    NorthwardWind,
}

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
        maximum_latitude: f64,
        minimum_depth:  Option<f64>,
        maximum_depth:  Option<f64>) -> Result<netcdf::File, io::Error> {

        // Validate that minimum longitude and latitude is less or equal to maximum
        if minimum_latitude > maximum_latitude {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Minimum latitude must be less than or equal to maximum latitude"));
        }
        if minimum_longitude > maximum_longitude {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Minimum longitude must be less than or equal to maximum longitude"));
        }

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

        // Add all arguments to the args vector
        args.push("--start-datetime".to_string());
        args.push(utc_date_time_to_string(start_datetime));
        args.push("--end-datetime".to_string());
        args.push(utc_date_time_to_string(end_datetime));
        args.push("--minimum-longitude".to_string());
        args.push(minimum_longitude.to_string());
        args.push("--maximum-longitude".to_string());
        args.push(minimum_longitude.to_string());
        args.push("--minimum-latitude".to_string());
        args.push(minimum_latitude.to_string());
        args.push("--maximum-latitude".to_string());
        args.push(maximum_latitude.to_string());
        // If minimum_depth is specified, add minimum_depth argument
        if minimum_depth.is_some() {
            args.push("--minimum-depth".to_string());
            args.push(minimum_depth.unwrap().to_string());
        }
        // If minimum_depth is specified, add minimum_depth argument
        if maximum_depth.is_some() {
            args.push("--maximum-depth".to_string());
            args.push(maximum_depth.unwrap().to_string());
        }
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
        let datestring = datestring.replace(":",""); // Remove colons from the date string to make it a valid filename
        filename.push_str(&datestring);

        // Make filepath
        // let filepath = self.output_path.as_str().to_owned() + "/" + &filename;

        // set output filename
        args.push("--output-filename".to_string());
        args.push(filename.clone());

        // Run the command to query server for the data
        // If fails, keep trying until successful or we get a non-timeout error
        // Init output so it exists outside of the loop
        let mut output: std::process::Output;

        let mut attempt_counter = 0;
        loop {
            // Running command
            let start_time = time::UtcDateTime::now();
            output = Command::new("copernicusmarine")
                .args(&args)
                .output()
                .expect(format!("Failed to execute the given command\ncopernicusmarine with arguments: {:?}\nMake sure you have the copernicusmarine toolbox installed\nError", args).as_str());

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
            // Assume if duration is shorter than 100 seconds that the error is not a timeout, but an error that will not be solved by trying again. Break the loop. 
            if duration < time::Duration::seconds(100) {
                return Err(io::Error::new(io::ErrorKind::Other, std::format!("Error getting data from copernicusmarine toolbox subset command. Query finished in {:?} < 100 seconds so assume not a timeout and stopping program.\nExit code: {}. Output message: {}\n\nQuery: copernicusmarine {:?}", duration, exit_code, String::from_utf8_lossy(&output.stderr), args)));
            }

            // Before trying again, wait 1 minute as per instructions from the devs: https://github.com/mercator-ocean/copernicus-marine-toolbox/issues/392#issuecomment-3136220183
            println!("Waiting {} seconds before trying again...", attempt_counter);
            thread::sleep(std::time::Duration::from_secs(attempt_counter));

            // Increment attempt_counter
            attempt_counter += 1;
        }

        // Debugging prints
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
        return Ok(netcdf_file);
    }

    /// Function that asks the copernicus marine server for data using the subset command and automatically scales and offsets the value
    /// If the returned values contain a fillvalue, then returns an error
    /// Only uses scale_factor, add_offset and fill_value if they exist, otherwise not.
    /// If you have problems with this command, consider using the subset command directly
    ///
    /// # Arguments
    ///
    /// * `variables` - A vector with the variables within the dataset that are being asked for
    ///
    /// Returns a vector of vectors, the first vector contains the data for the first variable and so on and so forth.
    /// If the value in the vector is 'None' then the value is a fill_value from the netcdf file.
    pub fn get_f64_values(&self,
        dataset_id: String,
        variables: Vec<String>,
        start_datetime: time::UtcDateTime,
        end_datetime: time::UtcDateTime,
        minimum_longitude: f64,
        maximum_longitude: f64,
        minimum_latitude: f64,
        maximum_latitude: f64,
        minimum_depth:  Option<f64>,
        maximum_depth:  Option<f64>,) -> Result<Vec<Vec<Option<f64>>>, io::Error> {

        // Get netcdf file from Copernicus
        let netcdf_file = match self.subset(dataset_id.clone(), variables.clone(), start_datetime, end_datetime, minimum_longitude, maximum_longitude, minimum_latitude, maximum_latitude, minimum_depth, maximum_depth) {
            Ok(file) => file,
            Err(e) => return Err(io::Error::from(e)),
        };

        // Get netcdf root from netcdf file
        let netcdf_root =  netcdf_file.root().expect("Could not get netcdf root from netcdf file");

        // Init data vectors
        let mut data_vectors = Vec::new();

        // Get the data for each variable
        for variable in variables {
            // Get netcdf_variables from netcdf root
            let netcdf_variable = netcdf_root.variable(variable.as_str()).expect(format!("No variable '{}' found in dataset '{}'", variable, dataset_id).as_str());

            // Get data vectors from variables
            let data_vector_direct: Vec<f64> = netcdf_variable.get_values(netcdf::Extents::All).expect("Failed to read eastward wind");
            let mut data_vector: Vec<Option<f64>> = Vec::new();
            for i in 0..data_vector_direct.len() {
                data_vector.push(Some(data_vector_direct[i]));
            }

            // Check if a fill value attribute exists
            let fill_value_attr_option = netcdf_variable.attribute("_FillValue");
            if fill_value_attr_option.is_some() {
                // Get fill value
                let fill_value_attr_val = fill_value_attr_option.unwrap().value().expect("Could not get fill value");
                let fill_value = match fill_value_attr_val {
                    netcdf::AttributeValue::Double(v) => v as f64,
                    netcdf::AttributeValue::Short(v) => v as f64,
                    netcdf::AttributeValue::Float(v) => v as f64,
                    _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "fill_value was not a usable number")),
                };

                // Check if any of the data is the fill value, if it is, set the entry in the data_vector to None
                for i in 0..data_vector.len() {
                    if data_vector[i].unwrap() == fill_value {
                        data_vector[i] = None;
                    }
                }
            }   // End if

            // Check if a scale factor attribute exists
            let scale_factor_attr_option = netcdf_variable.attribute("scale_factor");
            if scale_factor_attr_option.is_some() {
                // get scale factor
                let scale_factor_attr_val = scale_factor_attr_option.unwrap().value().expect("Could not get scale factor value");
                let scale_factor = match scale_factor_attr_val {
                    netcdf::AttributeValue::Double(v) => v as f64,
                    _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "scale_factor was not a double"))
                };
                // Scale data
                for i in 0..data_vector.len() {
                    if data_vector[i].is_some() {
                        data_vector[i] = Some(data_vector[i].unwrap()*scale_factor);
                    }
                }
            }   // End if

            // Check if a scale factor attribute exists
            let add_offset_attr_option = netcdf_variable.attribute("add_offset");
            if add_offset_attr_option.is_some() {
                // get add offset
                let add_offset_attr_val = add_offset_attr_option.unwrap().value().expect("Could not get add_offset value");
                let add_offset = match add_offset_attr_val {
                    netcdf::AttributeValue::Double(v) => v as f64,
                    _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "add_offset was not a double"))
                };
                // Offset data
                for i in 0..data_vector.len() {
                    if data_vector[i].is_some() {
                        data_vector[i] = Some(data_vector[i].unwrap() + add_offset);
                    }
                }
            }   // End if

            // Add data_vector to data_vectors
            data_vectors.push(data_vector);
        } // End for

        // Return data_vector after scaling and offseting
        return Ok(data_vectors);
    }

    /// Function that clears all netcdf files in the output path
    /// Warning: Use with caution!
    pub fn delete_all_netcdf_files_in_output_path(&self) -> Result<(), io::Error> {
        // Get list of files in output path
        let files = std::fs::read_dir(&self.output_path).expect("Could not read output path directory");

        // Loop through each file and delete all netcdf files
        for file in files {
            // get path to file from DirEntry
            let file = file.expect("Could not get path").path();
            // Get extension
            let extension = file.extension();

            // If extension exists and is "nc", delete the file
            if extension.is_some() {
                if extension.unwrap() == "nc" {
                    // Delete file
                    std::fs::remove_file(file).expect("Could not delete netcdf file");
                }
            }
        }

        // Return ok
        return Ok(());
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
/// Since according to datasheet the time is measured in seconds since that date: <https://documentation.marine.copernicus.eu/PUM/CMEMS-WIND-PUM-012-004-006.pdf>
pub fn secs_since_1990_01_01_0_to_utcdatetime(secs: i64) -> time::UtcDateTime {
    let start = time::UtcDateTime::new(time::Date::from_calendar_date(1990,time::Month::January,1).expect("Could not make time::Date"), time::Time::from_hms(0,0,0).expect("Could not make time::Time"));

    let dur = time::Duration::new(secs, 0);

    return start.checked_add(dur).expect("Could not add time::Duration to time::UtcDateTime");
}


/// Writes the datetime variable to the "YYYY-MM-DDTHH:MM:SS" format where the 'T' is literally just a 'T' and is there to ensure different operating system compatibility
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
    // If day is less than 10, add a leading zero
    if (datetime.day() as u8) < 10 {
        out_string.push_str("0");
    }
    out_string = out_string + &datetime.day().to_string();
    out_string.push_str("T");

    // If hour is less than 10, add a leading zero
    if (datetime.hour() as u8) < 10 {
        out_string.push_str("0");
    }
    out_string = out_string + &datetime.hour().to_string();
    out_string.push_str(":");
    // If minute is less than 10, add a leading zero
    if (datetime.minute() as u8) < 10 {
        out_string.push_str("0");
    }
    out_string = out_string + &datetime.minute().to_string();
    out_string.push_str(":");
    // If second is less than 10, add a leading zero
    if (datetime.second() as u8) < 10 {
        out_string.push_str("0");
    }
    out_string = out_string + &datetime.second().to_string();
    
    // Return output
    return out_string;
}


/// A function that gets the dataset id for a given variable and temporal extent
/// Note: As of 2025-09-17 almost all dates are hardcoded into this function since it is not known if the CMEMS datasets will be updated in the future or if they will remain as they are now. Please fix this if needed in future
pub fn get_dataset_id(variable: CopernicusVariable, start_datetime: time::UtcDateTime, end_datetime: time::UtcDateTime) -> Result<String, io::Error> {
    // Sanity check that end_datetime is indeed after start_datetime
    if start_datetime > end_datetime {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "start_datetime must be before end_datetime or the same as end_datetime"));
    }

    // Init dataset_id
    let dataset_id: String;

    // Match variable and for each variable match the temporal extent to get the correct dataset id
    match variable {
        CopernicusVariable::EastwardSeaWaterVelocity | CopernicusVariable::NorthwardSeaWaterVelocity => {
            // If start_datetime is before first possible date in GLOBAL_MULTIYEAR_PHY_001_030 then return error, see https://data.marine.copernicus.eu/viewer/expert?view=dataset&dataset=GLOBAL_MULTIYEAR_PHY_001_030
            if start_datetime <= time::UtcDateTime::new(time::Date::from_calendar_date(1993,time::Month::January,1).unwrap(), time::Time::from_hms(0, 0, 0).unwrap()) {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "start_datetime is before 1993-01-01 00:00:00, which is the earliest date for ocean current data in GLOBAL_MULTIYEAR_PHY_001_030"));
            }
            // If end_datetime is after last possible date in GLOBAL_ANALYSISFORECAST_PHY_001_024, then return error, see https://data.marine.copernicus.eu/viewer/expert?view=dataset&dataset=GLOBAL_ANALYSISFORECAST_PHY_001_024
            // Forecast goes 10 days into future
            if end_datetime >= (time::UtcDateTime::now() + time::Duration::days(10)) {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "end_datetime is after the latest date for ocean current data in GLOBAL_ANALYSISFORECAST_PHY_001_024, which has a forecast up to 10 days into the future from now"));
            }
            // Now if the start_datetime is before the first possible date in GLOBAL_ANALYSISFORECAST_PHY_001_024
            if start_datetime < time::UtcDateTime::new(time::Date::from_calendar_date(2020,time::Month::November,1).unwrap(), time::Time::from_hms(0, 0, 0).unwrap()) {
                // Check if end_datetime is after the last date in GLOBAL_MULTIYEAR_PHY_001_030, if so, return error since we have to use more than one dataset to cover the temporal extent
                if end_datetime > time::UtcDateTime::new(time::Date::from_calendar_date(2025,time::Month::August,26).unwrap(), time::Time::from_hms(0, 0, 0).unwrap()) {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "The temporal extent from start_datetime to end_datetime is too large and would require using more than one dataset. Please use a smaller temporal extent."));
                }
                // Otherwise return only the ocean current dataset_id for GLOBAL_MULTIYEAR_PHY_001_030
                dataset_id = "cmems_mod_glo_phy_my_0.083deg_P1D-m".to_string();
            } // Otherwise return only the ocean current dataset_id for GLOBAL_ANALYSISFORECAST_PHY_001_024
            else {
                dataset_id = "cmems_mod_glo_phy_anfc_0.083deg_PT1H-m".to_string();
            }
        },
        CopernicusVariable::EastwardWind | CopernicusVariable::NorthwardWind => {
            // If start_datetime is before first possible date in WIND_GLO_PHY_L4_MY_012_006 then return error, see https://data.marine.copernicus.eu/viewer/expert?view=dataset&dataset=WIND_GLO_PHY_L4_MY_012_006
            if start_datetime <= time::UtcDateTime::new(time::Date::from_calendar_date(1994,time::Month::June,1).unwrap(), time::Time::from_hms(0, 0, 0).unwrap()) {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "start_datetime is before 1994-06-01 00:00:00, which is the earliest date for wind data in WIND_GLO_PHY_L4_MY_012_006"));
            }
            // If end_datetime is after last possible date in WIND_GLO_PHY_L4_NRT_012_004, then return error, see https://data.marine.copernicus.eu/viewer/expert?view=dataset&dataset=WIND_GLO_PHY_L4_NRT_012_004
            // No forecast, updated daily at 15:00
            if end_datetime >= time::UtcDateTime::now() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "end_datetime is after the latest date for wind data in WIND_GLO_PHY_L4_NRT_012_004, which is updated daily at 15:00 UTC"));
            }
            // Now if the start_datetime is before the first possible date in WIND_GLO_PHY_L4_NRT_012_004
            if start_datetime < time::UtcDateTime::new(time::Date::from_calendar_date(2023,time::Month::April,27).unwrap(), time::Time::from_hms(0, 0, 0).unwrap()) {
                // Check if end_datetime is after the last date in WIND_GLO_PHY_L4_MY_012_006, if so, return error since we have to use more than one dataset to cover the temporal extent
                if end_datetime > time::UtcDateTime::new(time::Date::from_calendar_date(2025,time::Month::May,21).unwrap(), time::Time::from_hms(0, 0, 0).unwrap()) {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "The temporal extent from start_datetime to end_datetime is too large and would require using more than one dataset. Please use a smaller temporal extent."));
                }
                // Otherwise return only the ocean current dataset_id for WIND_GLO_PHY_L4_MY_012_006
                dataset_id = "cmems_obs-wind_glo_phy_my_l4_0.125deg_PT1H".to_string();
            } // Otherwise return only the ocean current dataset_id for WIND_GLO_PHY_L4_MY_012_006
            else {
                dataset_id = "cmems_obs-wind_glo_phy_nrt_l4_0.125deg_PT1H".to_string();
            }
        }
    }   // End match

    // Return dataset_id
    return Ok(dataset_id);
}

// Tests
//-------------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_dataset_id() {
        let date = time::UtcDateTime::new(time::Date::from_calendar_date(2024,time::Month::January,1).unwrap(), time::Time::from_hms(0, 0, 0).unwrap());
        let result = match get_dataset_id(CopernicusVariable::EastwardSeaWaterVelocity, date, date) {
            Ok(id) => id,
            Err(e) => panic!("Error getting dataset id: {}", e),
        };
        println!("Result: {:?}", result);
        // assert_eq!(result, 4);
    }
}
