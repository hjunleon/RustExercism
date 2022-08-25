// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::cmp;

pub fn my_min(a: i32,b: i32) -> i32 {
    if a > b {
        return b;
    }
    a
}

pub fn production_rate_per_hour(r_speed: u8) -> f64 {
    let speed: u32 = r_speed as u32;
    let unit_work: u32 = 221;
    // let mut total_work: f32 = 0.;
    // let mut multiplier: f32 = 1.0; 
    // let mut cur_range: u32 = 1;
    if (speed <= 4){
        return (unit_work * speed) as f64; 
    } else if speed <= 8 {
        return (unit_work * speed) as f64 * 0.9
    }
    (unit_work * speed) as f64 * 0.77
    
    // unimplemented!("calculate hourly production rate at speed: {}", speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    println!("{} / {} = {}", production_rate_per_hour(speed), 60., production_rate_per_hour(speed) / 60.);
    (production_rate_per_hour(speed) / 60.) as u32
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
}
