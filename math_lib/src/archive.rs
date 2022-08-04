pub fn control(input: String) -> ControlFlow<()> {
    const OPT_YES: &str = "y";
    const OPT_QUIT: &str = "q";
    const OPT_ALL: &str = "a";

    // println!("input: {}", input);
    match Some(input) {
        Some(yes) if yes == OPT_YES => {
            println!("{}", yes);
            ControlFlow::Continue(())
        }
        Some(no) if no == OPT_QUIT => {
            println!("{}", no);
            ControlFlow::Break(())
        }
        Some(all) if all == OPT_ALL => {
            println!("{}", all);
            ControlFlow::Continue(())
        }
        None => panic!(),
        _ => ControlFlow::Continue(()),
    }
}
// Some(num) if num.trim().parse::<i64>().is_ok() => {
//     let square_root = Sqrt::custom_sqrt(num.trim().parse::<i64>().unwrap());
//     println!("sqrt of {} is {}", num, square_root);
//     ControlFlow::Continue(())
// }
pub struct CLI;

impl CLI {
    fn stdin_read_line_cli() -> String {
        loop {
            println!("{}", "Enter the number to root for!".yellow());
            let mut input: String = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read your input number!");

            match Some(input.trim()) {
                Some(exit) if Loop::input_exit(exit) == ControlFlow::Break(()) => {
                    println!("{} ", "\nExiting. Thank you!\n".bright_green().bold());
                    break;
                }
                Some(_) => continue,
                None => continue,
            };

            // if let ControlFlow::Break(_) == continue_or_break_flow(input) {
            // break;
            // };
        }

        // input
    }

    fn continue_or_break_flow(input: String) -> ControlFlow<()> {
        match Some(input) {
            Some(exit) if exit == "q" => {
                println!("{}", "{exiting... thank you!}".green());
                ControlFlow::Break(())
            }
            Some(_) => ControlFlow::Continue(()),
            None => panic!(),
        }
    }
}

// fn loop_demo(i: i32) {
//     let something = loop {
//         i *= 2;
//         if i > 100 {
//             break i;
//         }
//     };
// }
