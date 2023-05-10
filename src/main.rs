#[allow(unused, non_snake_case, non_upper_case_globals, non_camel_case_types)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;
use postgres::{Client, NoTls};

fn main() {
    let mut error;
    let mut client =
        Client::connect("host=192.168.2.2 user=postgres password=measurement dbname=air_quality", NoTls).unwrap();

    unsafe {
        sensirion_i2c_hal_init();

        // Clean up potential SCD40 states
        scd4x_wake_up();
        scd4x_stop_periodic_measurement();
        scd4x_reinit();

        // Start Measurement

        error = scd4x_start_periodic_measurement();
        if error != 0 {
            println!("Error executing scd4x_start_periodic_measurement(): {error}");
        }
    }
    println!("Waiting for first measurement... (5 sec)");

    loop {
        // Read Measurement if data is available
        unsafe { sensirion_i2c_hal_sleep_usec(5000000) };
        let mut co2 = 0;
        let mut temperature = 0.0;
        let mut humidity = 0.0;
        error = unsafe { scd4x_read_measurement(&mut co2, &mut temperature, &mut humidity) };
        if error != 0 {
            println!("Error executing scd4x_read_measurement(): {error}");
        } else if co2 == 0 {
            println!("Invalid sample detected, skipping.");
        } else {
            let res = client.execute("INSERT INTO measurements (time, ppm, temperature, humidity) VALUES (CURRENT_TIMESTAMP, $1, $2, $3)", &[&(co2 as i32), &temperature, &humidity]);
            if let Err(e) = res {
                eprintln!("Failed to insert into database {e:?}");
            } else {
                println!("Measurement added to the database successfully.");
            }
            println!("CO2: {co2} ppm");
            println!("Temperature: {temperature:.2} °C");
            println!("Humidity: {humidity:.2} RH");
        }
    }
}
