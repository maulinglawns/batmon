use std::fs;
use std::env;
use std::path::Path;
use std::process::Command;
use std::process::exit;
use chrono;

const AC: &str = "/sys/class/power_supply/AC/online";
const BATFULL: &str = "/sys/class/power_supply/BAT0/charge_full";
const BATNOW: &str = "/sys/class/power_supply/BAT0/charge_now";

fn help() {
    println!("
Usage: batmon [-h] <CRITICAL>

Power off system if battery is lower than CRITICAL percent.

Flags:
  -h        Show this help then exit.

Example:
batmon 15
");
}

fn ac_conn(ac_path: &str) -> bool {
    // Check if AC is connected
    let ac: String = fs::read_to_string(ac_path)
        .unwrap()
        .parse()
        .unwrap();
        
    if ac.trim_end() == "1" {
        return true;
    }

    false
}

fn bat_percent(batf_path: &str, batn_path: &str) -> i32 {
    // Calculate percent left of battery
    let bat_full: String = fs::read_to_string(batf_path)
        .unwrap()
        .parse()
        .unwrap();
    let bat_now: String = fs::read_to_string(batn_path)
        .unwrap()
        .parse()
        .unwrap();
    
    // Convert Strings to floats
    let bat_full = bat_full.trim_end().parse::<f64>().unwrap();
    let bat_now = bat_now.trim_end().parse::<f64>().unwrap();
    
    // Calculate percent
    let bat_perc_float = (bat_now/bat_full)*100.0;
    
    // Return percent as int
    bat_perc_float as i32
}


fn main() {
    let critical: i32;
    
    // Sanity check our paths
    let paths_vec: [&str; 3] = [AC, BATFULL, BATNOW];

    for p in &paths_vec {
        if ! Path::new(p).exists() {
            println!("The path: \"{}\" does not exist. Exiting.", p);
            exit(1);
        }
    }
    
    // Save 'now' as timestamp for logging
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    
    // Check arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No argument given.");
        exit(1);
    }
    // Show help then exit
    if args[1] == "-h" {
        help();
        exit(0);
    }
    // Convert argument to int
    if args[1].parse::<i32>().is_ok() {
        critical = args[1].parse::<i32>().unwrap();
    } else {
        println!("\"{}\" is not an integer.", args[1]);
        println!("Usage: batmon [-h] <CRITICAL>");
        exit(1);
    }
    // Only values between 0 and 100 allowed
    if critical < 0 || critical > 100 {
        println!("Only values between 0 and 100 allowed. Exiting.");
        exit(1);
    }
    
    println!("Critical level set at {} percent.", critical);
    
    // If power cable is connected, we can skip the rest!
    if ac_conn(AC) {
        println!("{} - Power connected. It's all good.", now);
        exit(0);
    }
    
    // If we run on battery, check level against critical
    let bat_now = bat_percent(BATFULL, BATNOW);
    if bat_now < critical {
        println!("{} - Battery at {} percent. Powering off.", now, bat_now);
        Command::new("/sbin/shutdown")
                 .arg("-h")
                 .arg("now")
                 .spawn()
                 .expect("Failed to shutdown system");
    }
}
