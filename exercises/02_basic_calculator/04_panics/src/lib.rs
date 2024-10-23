/// Given the start and end points of a journey, and the time it took to complete the journey,
/// calculate the average speed of the journey.
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: Panic with a custom message if `time_elapsed` is 0
    if time_elapsed == 0 {
<<<<<<< HEAD
        panic!("The journey took no time at all, that's impossible!");
    }

=======
        panic!("The journey took no time at all, that's impossible!")
    }
    
>>>>>>> b3b0ba0c84a3d1a6106786bcf684ddc30831219a
    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 With the `#[should_panic]` annotation we can assert that we expect the code
    //    under test to panic. We can also check the panic message by using `expected`.
    //    This is all part of Rust's built-in test framework!
    #[should_panic(expected = "The journey took no time at all, that's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
