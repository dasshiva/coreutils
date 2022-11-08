use std::env;
use std::path::Path;
use std::panic;

mod copy;

fn main() {
    panic::set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            println!("{s:?}");
        }
        else {
            println!("{info}");
        }
    }));
    let args: Vec<String> = env::args().collect();
    let prog = match Path::new(&args[0]).file_name() {
        Some(x) => {
            match x.to_str() {
                Some(s) => s,
                None => panic!("Unreachable"),
            }
        }
        None => panic!("Unreachable!"), 
    };

    match prog {
        "cp" => copy::cp(&args),
         _   => panic!("No such program {} ", prog),
    }
}
