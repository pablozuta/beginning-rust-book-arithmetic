fn main() {
    // sum of two integer numbers
    println!("The sum is {}.", 80 + 34);
    /* the compiler, realizing that this expression contains only constant values, evaluates directly that expression, obtaining the integer number 114, and stores into the executable program such number in binary format. */

    // It is also allowed to write
    println!("{} + {} = {}", 34, 80, 80 +34);

    // All the integer arithmetic operators of C language can be used.
    println!("El resultado de todo esto es: {}", (23 - 6) % 5 + 20 * 30 / (3 + 4));
    /* 17 % 5 is the “remainder of the integer division” operation, and it has 2 as a result,that is, the remainder of the operation 17 / 5. */

    // compute the sum between two numbers with fractional parts
    println!("The sum of floats is: {}", 80.3 + 34.8);

    // Also expressions containing floating-point numbers can be evaluated:
   println!("{}", (23. - 6.) % 5. + 20. * 30. / (3. + 4.));

   // differing from C language, in Rust you cannot simply mix integer
   // numbers and floating-point numbers , you have to do this:
   println!("{}", 2.7 + 1.);

   // % operator behaves in Rust like in C language
   println!("{} {}", -12 % 10, -1.2 % 1.);

   // this is a valid program
   println!("{}", "These
       are
          three lines");

}
