//system monitor specifically for component temps
//import sysinfo crate
use sysinfo::{Components, RefreshKind};
use crossterm::terminal::{Clear, ClearType};
use std::io::{stdout, Write};
use crossterm::queue;

fn main() {
    //basic test stuff to start, try reading the component temps on my device
    let mut sys = sysinfo::System::new_all();

    //try looping this bit, expect that it will print new set per lines
    loop {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All)).unwrap();
        stdout.flush().unwrap();
        //iterate each component individually now
        sys.refresh_specifics(RefreshKind::nothing());
        let components = Components::new_with_refreshed_list();
        //print to terminal
        for component in &components {
            println!("{component:?}");
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
