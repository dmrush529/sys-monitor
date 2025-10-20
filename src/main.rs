//system monitor specifically for component temps
//import sysinfo crate
use sysinfo::{Components, RefreshKind};
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Write};
use crossterm::queue;

fn main() {
    //create new sysinfo entry
    let mut sys = sysinfo::System::new_all();

    //loop to constantly display temp data
    loop {
        //create stdout
        let mut stdout = stdout();
        //clear all lines printed by the last print
        queue!(stdout, Clear(ClearType::All)).unwrap();
        stdout.flush().unwrap();
        //iterate each component and refresh their info
        sys.refresh_specifics(RefreshKind::nothing());
        let components = Components::new_with_refreshed_list();
        //print to terminal
        for component in &components {
            println!("{component:?}");
        };
        //wait one second before doing it again
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
