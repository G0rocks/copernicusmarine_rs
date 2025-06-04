/// copernicusmarine_rs.
/// Author: G0rocks
/// Date: 2025-06-04
/// Description
/// Enables the use of copernicus marine toolbox through rust

// Dependencies
//-------------------------------------------------------------------------------------------------------------------------
use time;   // For start and end times
use std::process::Command; // To run commands through the command line
// netcdf?


// Enums
//-------------------------------------------------------------------------------------------------------------------------



// Functions
//-------------------------------------------------------------------------------------------------------------------------
/// Function to get a subset from the copernicus database using the copernicus marine toolbox
pub fn subset(
    dataset_id: String,
    variables: Vec<String>,
    start_datetime: time::UtcDateTime,
    end_datetime: time::UtcDateTime,
    minimum_longitude: f64,
    maximum_longitude: f64,
    minimum_latitude: f64,
    maximum_latitude: f64) -> u8 {

    // Make argument
    let mut args: Vec<String> = vec![
        "subset".to_string(),
        "--dataset-id".to_string(),
        dataset_id];

    // Make variable argument
    // Loop and for each variable in variables, add "--variable " + variable[i].to_string();
    for variable in variables {
        args.push("--variable".to_string());
        args.push(variable);
    }

    args.push("--start-datetime".to_string());
    args.push(start_datetime.to_string());
    args.push("--end-datetime".to_string());
    args.push(end_datetime.to_string());
    args.push("--minimum-longitude".to_string());
    args.push(minimum_longitude.to_string());
    args.push("--maximum-longitude".to_string());
    args.push(maximum_longitude.to_string());
    args.push("--minimum-latitude".to_string());
    args.push(minimum_latitude.to_string());
    args.push("--maximum-latitude".to_string());
    args.push(maximum_latitude.to_string());
    
    let output = Command::new("copernicusmarine")
        .args(&args)
        .output()
        .expect("Failed to execute command \"copernicusmarine subset\" with the given arguments");

    println!("Status: {}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    return 5;
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
