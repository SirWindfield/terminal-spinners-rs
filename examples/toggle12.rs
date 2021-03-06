use std::{thread, time::Duration};
use terminal_spinners::{SpinnerBuilder, TOGGLE12};
fn main() {
    let text = "Loading unicorns";
    let handle = SpinnerBuilder::new().spinner(&TOGGLE12).text(text).start();
    thread::sleep(Duration::from_secs(3));
    handle.done();
}
