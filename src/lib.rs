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
use std::process::Command; // To run commands through the command line
use netcdf; // For working with and using the netcdf files retrieved from the copernicus server
// use zarrs; // For working with and using the .zarr files retrieved from the copernicus server
// use pyo3;   // For calling python things


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
    /// This is the function you use to get some specific data
    pub fn subset(
        &self,
        dataset_id: String,
        variables: Vec<String>,
        start_datetime: time::UtcDateTime,
        end_datetime: time::UtcDateTime,
        minimum_longitude: f64,
        maximum_longitude: f64,
        minimum_latitude: f64,
        maximum_latitude: f64) -> u8 { //netcdf::File {

        // Validate that minimum longitude and latitude is less or equal to maximum
        if minimum_latitude > maximum_latitude {
            panic!("Minimum latitude must be less than or equal to maximum latitude");
        }
        if minimum_longitude > maximum_longitude {
            panic!("Minimum longitude must be less than or equal to maximum longitude");
        }

        println!("Getting copernicus subset");

        
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
        args.push(utc_date_time_to_copernicus_string(start_datetime));
        args.push("--end-datetime".to_string());
        args.push(utc_date_time_to_copernicus_string(end_datetime));
        args.push("--minimum-longitude".to_string());
        args.push(minimum_longitude.to_string());
        args.push("--maximum-longitude".to_string());
        args.push(maximum_longitude.to_string());
        args.push("--minimum-latitude".to_string());
        args.push(minimum_latitude.to_string());
        args.push("--maximum-latitude".to_string());
        args.push(maximum_latitude.to_string());
        args.push("--output-directory".to_string());
        args.push(self.output_path.clone());
        
        // File name
        let mut filename = String::new();
        filename.push_str(&dataset_id);
        filename.push('_');
        filename.push_str(&variables[0]);
        filename.push('_');
        filename.push_str(&minimum_latitude.to_string());
        filename.push('_');
        filename.push_str(&maximum_latitude.to_string());
        filename.push('_');
        filename.push_str(&minimum_longitude.to_string());
        filename.push('_');
        filename.push_str(&maximum_longitude.to_string());
        filename.push('_');

        // Date for filename
        let mut datestring = start_datetime.year().to_string();
        datestring.push('-');
        datestring.push_str(&(start_datetime.month() as u8).to_string());
        datestring.push('-');
        datestring.push_str(&start_datetime.day().to_string());
        filename.push_str(&datestring);
        // filename.push_str(".zarr");

        // Make filepath
        let filepath = self.output_path.as_str().to_owned() + "/" + &filename;
        println!("file path: {}", &filepath);

        // set output filename
        args.push("--output-filename".to_string());
        args.push(filename);

        println!("Querying server");
        
        // Run the command to get the data
        let output = Command::new("copernicusmarine")
            .args(&args)
            .output()
            .expect("Failed to execute command \"copernicusmarine subset\" with the given arguments");

        // println!("Status: {}", output.status);
        // println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

        // Find the file where the data is stored
        // Print hello with pyo3?
        // pyo3::marker::Python::with_gil(|py| {
        //     let s = c"print(\"Hello user, I'm Python\")";
        //     py.run(s, None, None).expect("Well that didn't work");
        // });

        let netcdf_file = netcdf::open(filename);
        println!("Netcdf file: {:?}", netcdf_file);
        // Move into output path directory
        // std::env::set_current_dir(std::path::Path::new(&self.output_path)).expect("Error changing directories");
        // Print working directory
        // println!("Changed directory to: {:?}", std::env::current_dir().expect("Could not get current directory"));
        // Make zarr store
        // let zarr_store: zarrs::storage::ReadableWritableListableStorage = std::sync::Arc::new(zarrs::storage::store::MemoryStore::new());
        // let zarr_store: zarrs::filesystem::FilesystemStore = zarrs::filesystem::FilesystemStore::new(std::env::current_dir().expect("Could not get current dir")).expect("Could not make zarrs::filesystem::FilesystemStore");
        // Make store key
        // let store_key = zarrs::storage::StoreKey::new("eastward_wind").expect("Could not make store key");
        // println!("Zarr store keys: {:?}", zarr_store.list());
        // println!("Zarr store : {:?}", zarr_store);


        // Return the file
        return 5; //netcdf_file;
    }
}

// Helper functions
//-------------------------------------------------------------------------------------------------------------------------


/// Writes the datetime variable to the "YYYY-MM-DDTHH:MM:SS" format where the 'T' is literally just a 'T'
pub fn utc_date_time_to_copernicus_string(datetime: time::UtcDateTime) -> String {
    // Init empty string
    let mut out_string = String::new();

    // Assemble string in correct format
    out_string = out_string + &datetime.year().to_string();
    out_string.push_str("-");
    out_string = out_string + &datetime.month().to_string();
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
