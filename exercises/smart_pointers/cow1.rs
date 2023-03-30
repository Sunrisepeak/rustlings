// cow1.rs

// This exercise explores the Cow, or Clone-On-Write type.
// Cow is a clone-on-write smart pointer.
// It can enclose and provide immutable access to borrowed data, and clone the data lazily when mutation or ownership is required.
// The type is designed to work with general borrowed data via the Borrow trait.
//
// This exercise is meant to show you what to expect when passing data to Cow.
// Fix the unit tests by checking for Cow::Owned(_) and Cow::Borrowed(_) at the TODO markers.


use std::borrow::Cow;

fn abs_all<'a, 'b>(input: &'a mut Cow<'b, [i32]>) -> &'a mut Cow<'b, [i32]> {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            // Clones into a vector if not already owned.
            input.to_mut()[i] = -v;
        }
    }
    input
}

fn print_and_cmp<'a, 'b>(input1: &'a Cow<'b, [i32]>, input2: &'a Cow<'b, [i32]>) {
    for i in 0..input1.len() {
        println!("({}\t{})", input1[i], input2[i]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() -> Result<(), &'static str> {
        // Clone occurs because `input` needs to be mutated.
        let slice = [-1, 0, 1];
        let mut input = Cow::from(&slice[..]);
        let old_input = input.clone();
        let new_input = abs_all(&mut input);

        print_and_cmp(&old_input, &new_input);

        Ok(())
    }

    #[test]
    fn reference_no_mutation() -> Result<(), &'static str> {
        // No clone occurs because `input` doesn't need to be mutated.
        let slice = [0, 1, -1];
        let mut input = Cow::from(&slice[..]);
        match abs_all(&mut input) {
            Cow::Owned(_) => Ok(()),
            _ => Err("Expected owned value"),
        }
    }

    #[test]
    fn owned_no_mutation() -> Result<(), &'static str> {
        // We can also pass `slice` without `&` so Cow owns it directly.
        // In this case no mutation occurs and thus also no clone,
        // but the result is still owned because it always was.
        let slice = vec![0, 1, 2];
        let mut input = Cow::from(slice);
        let old_input = input.clone();
        let new_input = abs_all(&mut input);

        print_and_cmp(&old_input, &new_input);

        Ok(())
    }

    #[test]
    fn owned_mutation() -> Result<(), &'static str> {
        // Of course this is also the case if a mutation does occur.
        // In this case the call to `to_mut()` returns a reference to
        // the same data as before.
        let slice = vec![-1, 0, 1];
        let mut input = Cow::from(slice);
        let old_input = input.clone();
        let new_input = abs_all(&mut input);

        print_and_cmp(&old_input, &new_input);

        Ok(())
    }
}
