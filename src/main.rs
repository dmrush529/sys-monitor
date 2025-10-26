//system monitor specifically for component temps
//import sysinfo crate
use sysinfo::*;

fn main() {
    //create new system entry with sysinfo
    let mut sys = System::new_all();

    //loop to constantly display temp data
    loop {
        //clear all lines printed by the last print
        println!("{esc}c", esc = 27 as char);
        //iterate each component and refresh their info
        sys.refresh_all();
        let components = Components::new_with_refreshed_list();
        //print to terminal
        for component in &components {
            println!("{component:?}");
        };
        //wait one second before doing it again
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

}
