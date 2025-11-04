use std::thread::sleep;
use std::time::Duration;
//system monitor specifically for component temps
//import sysinfo crate
use sysinfo::*;

//create async function for the loop
async fn temp_display() {

    //create new system for sysinfo
    let mut sys = System::new_all();

    //implement the loop
    loop {
        //clear all lines
        print!("{esc}c", esc = 27 as char);

        //iterate the components
        sys.refresh_all();
        let components = Components::new_with_refreshed_list();

        for component in &components {
            println!("{component:?}");
        }
        sleep(Duration::from_secs(1));
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() {
    //call our async function
    temp_display().await;
}
