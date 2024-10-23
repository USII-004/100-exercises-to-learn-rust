// Rewrite the factorial function using a `while` loop.
pub fn factorial(mut n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.
<<<<<<< HEAD
    let mut result: u32 = 1;

=======

    let mut result = 1;
>>>>>>> b3b0ba0c84a3d1a6106786bcf684ddc30831219a
    while n > 0 {
        result *= n;
        n -= 1;
    }
<<<<<<< HEAD

    result 
=======
    result
>>>>>>> b3b0ba0c84a3d1a6106786bcf684ddc30831219a
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
