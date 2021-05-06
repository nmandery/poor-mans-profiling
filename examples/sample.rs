use std::time::Duration;

use serde::Serialize;

use poor_mans_profiling::Span;

#[derive(Serialize)]
struct MyStruct {
    n_secs: u64,
}

fn main() {
    println!("start");
    let n_secs = 2;
    {
        let _s = Span::new_with_message(MyStruct { n_secs }, "Sleeping");
        std::thread::sleep(Duration::from_secs(n_secs));
    }
    println!("end");

    // this will print something like
    //
    // start
    // {"t_start":"2021-05-06T17:57:33.985703608Z","t_end":"2021-05-06T17:57:35.985855186Z","message":"Sleeping","data":{"n_secs":2}}
    // end
}
