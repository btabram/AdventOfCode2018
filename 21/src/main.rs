use std::collections::HashSet;

type ErrorHolder = Box<std::error::Error>;

fn main() -> Result<(), ErrorHolder> {
    // *Part 1* logic (calculate solution later on with part 2)
    //
    // Only command 28 makes use of the [0], since we can only effect
    // [0] the program up until the point where command 28 runs is out of our
    // control so don't bother trying to understand it.
    // State when we first run command 28: [x, 28, 1, 13270004, 1, 1]
    //
    // Carrying on:
    // IP is now 28 so set [2] to [3]==[0] (false with x = 0)
    // IP is now 29 so add [2] to [1]
    // IP is now 30 so set [1] to 5
    // IP is now 6...
    //
    // Now if [3]==[0] was true then the program would have had IP 31 after
    // command 29 and finished. This is surely the quickest way to halt the
    // program by changing only the inital value of [0].
    //
    // So the answer to part 1 us 13270004!



    // *Part 2*
    //
    // We need to understand the program now, so running through it:
    //
    // IP is [1]
    // IP is now 0 so set [3] to 123
    // IP is now 1 so set [3] to [3]&456 (binary and)
    // IP is now 2 so set [3] to [3]==72 (true for this code)
    // IP is now 3 so add [3] to [1]
    // IP is now 5 so set [3] to 0
    //
    // Everything so far is the check that inputs are being interpreted as
    // numbers and not strings. Once complete, there's no resulting changes.
    //
    // IP is now 6 so set [5] to [3]|65536
    // IP is now 7 so set [3] to 15028787
    // IP is now 8 so set [2] to [5]&255
    // IP is now 9 so add [2] to [3]
    // IP is now 10 so set [3] to [3]&16777215
    // IP is now 11 so multiply [3] by 65899
    // IP is now 12 so set [3] to [3]&16777215
    // IP is now 13 so set [2] to 256>[5] (false)
    // IP is now 14 so add [2] to [1]
    // IP is now 15 so add 1 to [1]
    // IP is now 17 so set [2] to 0
    // >> LOOP <<
    // IP is now 18 so set [4] to [2]+1
    // IP is now 19 so multiply [4] by 256
    // IP is now 20 so set [4] to [4]>[5] (false for now)
    // IP is now 21 so add [4] to [1]
    // IP is now 22 so add 1 to [1]
    // IP is now 24 so add 1 to [2]
    // IP is now 25 so set [1] to 17
    // >> LOOP <<
    // IP is now 18 so...

    // The above loop doesn't involve [0] (the one thing we can control) so it's
    // not helpful towards the solution but we still need to get past it.
    // The only test is that [4]>[5] where [4]=256*([2]+1) and every loop [2]
    // increases by 1. The condition will be true when [2] = 256 at the start
    // of the loop. So we start with [x, 18, 256, 6196817, 0, 65536] and do:
    //
    // IP is now 18 so set [4] to [2]+1
    // IP is now 19 so multiply [4] by 256
    // IP is now 20 so set [4] to [4]>[5] (true)
    // IP is now 21 so add [4] to [1]
    // IP is now 23 so set [1] to 25
    // IP is now 26 so set [5] to [2]
    // IP is now 27 so set [1] to 7
    //
    // IP is now 8 so set [2] to [5]&255
    // IP is now 9 so add [2] to [3]
    // IP is now 10 so set [3] to [3]&16777215
    // IP is now 11 so multiply [3] by 65899
    // IP is now 12 so set [3] to [3]&16777215
    // IP is now 13 so set [2] to 256>[5] (false)
    // IP is now 14 so add [2] to [1]
    // IP is now 15 so add 1 to [1]
    // IP is now 17 so set [2] to 0
    //
    // IP is now 18 so set [4] to [2]+1
    // IP is now 19 so multiply [4] by 256
    // IP is now 20 so set [4] to [4]>[5] (false for now)
    // IP is now 21 so add [4] to [1]
    // IP is now 22 so add 1 to [1]
    // IP is now 24 so add 1 to [2]
    // IP is now 25 so set [1] to 17
    //
    // IP is now 18 so set [4] to [2]+1
    // IP is now 19 so multiply [4] by 256
    // IP is now 20 so set [4] to [4]>[5] (now true)
    // IP is now 21 so add [4] to [1]
    // IP is now 23 so set [1] to 25
    // IP is now 26 so set [5] to [2]
    // IP is now 27 so set [1] to 7
    //
    // IP is now 8 so set [2] to [5]&255
    // IP is now 9 so add [2] to [3]
    // IP is now 10 so set [3] to [3]&16777215
    // IP is now 11 so multiply [3] by 65899
    // IP is now 12 so set [3] to [3]&16777215
    // IP is now 13 so set [2] to 256>[5] (now true)
    // IP is now 14 so add [2] to [1]
    // IP is now 16 so set [1] to 27
    //
    // Now IP 28 where we have to consider [0], the state is:
    // [x, 28, 1, 13270004, 1, 1]

    // Loop 1 (18-25):
    //
    // loop {
        // if 256*([2]+1) > [5] {
            // break to command 26
        // }
        // [2]++
    // }
    //
    // Command 26 sets [5] to [2] and goes to the start of loop 2 (see below)

    // Loop 2 (8-17):
    //
    // loop {
        // [3] += [5]&255
        // [3] = [3]&16777215
        // [3] *= 65899
        // [3] = [3]&16777215
        // if 256>[5] {
            // break to command 28
        // }
        // [2] = 0
        // Do loop 1
    // }
    //
    // Command 28 offers a chance to exit if [3] == [0],
    // otherwise [5] = [3]|65536 and [3] = 15028787, then start loop 2

    // Optimised reproduction of the program from when it initially passes the
    // check that inputs are numbers and not strings:
    let mut set = HashSet::new();
    let mut first_value = None;
    let mut prev_value = None;

    //let mut two = 0;
    let mut three = 15028787;
    let mut five = 0|65536;
    loop {
        loop {
            three += five&255;
            three = three&16777215;
            three *= 65899;
            three = three&16777215;
            if 256 > five {
                break;
            }
            //two = 0;

            // We can remove this loop (and variable 2 entirely), see below
            /*
            loop {
                if 256*(two+1) > five {
                    five = two;
                    break;
                }
                two += 1;
            }
            */
            five = ((five as f64) / 256.0).floor() as i32;
        }

        // This is the point in the loop where we can exit if [0] == [3]
        //
        // Part 1. Setting [0] equal to the first value of [3] will allow us
        // to exit the program the quickest.
        if first_value == None {
            first_value = Some(three);
        }
        // Part 2. Check for repeated [3] values. Assuming that the values
        // repeat then setting [0] to the last value before repetition will run
        // the program for the longest whilst still halting.
        if !set.insert(three) {
            break;
        }
        prev_value = Some(three);

        five = three|65536;
        three = 15028787;
    }

    println!("For the shortest running time set register [0] to {}.",
             first_value.unwrap());
    println!("For the longest running time set register [0] to {}.",
             prev_value.unwrap());

    Ok(())
}
