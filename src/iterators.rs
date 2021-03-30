/*
    Iterators in Rust

    Recall that every for loop is internally an iterator:
*/

pub fn example_for() {
    unimplemented!()
}

pub fn example_iter() {
    unimplemented!()
}

// More explicitly...
pub fn example_iter_explicit1() {
    unimplemented!()
}

// And we can also use .next() directly, not in a loop
pub fn example_iter_explicit2() {
    unimplemented!()
}

/*
    Data structures, in particular Vec, have methods to iterate over them:
    - iter()
    - iter_mut()
    - into_iter()

    into_iter() is defined by the IntoIterator trait:
    this is what makes for loops work!
*/

pub fn example_iter_mut() {
    unimplemented!()
}

pub fn example_into_iter() {
    // Conceptually same as example_for
    unimplemented!()
}

// Other data structures...

/*
    Programming with Iterators!

    Very rich API:
    https://doc.rust-lang.org/std/iter/trait.Iterator.html

    Iterators can be used to write code in a more functional style.

    EXERCISES:
    1. Check if all elements in a vector are below a threshold
    2. Return a vector of only the elements below the threshold
    3. Copy the nth element n times
    4. Pad/truncate a vector to exactly a certain length
    5. Sum all the squares less than n
*/

pub fn all_below(_v: Vec<usize>, _thresh: usize) -> bool {
    unimplemented!()
}

pub fn filter_below(_v: Vec<usize>, _thresh: usize) -> Vec<usize> {
    unimplemented!()
}

pub fn copy_increasing(_v: Vec<usize>) -> Vec<usize> {
    unimplemented!()
}

pub fn pad_truncate(_v: Vec<usize>) -> Vec<usize> {
    unimplemented!()
}

pub fn sum_squares_lt(_n: usize) -> usize {
    unimplemented!()
}

/*
    We can also write functions that manipulate iterators directly.

    Exercise: implement some of the above using iterators as input/output.

    But what should the return values be?

    - Here the impl Trait syntax comes in handy again.
*/

/*
    Writing iterators for your own data types

    There are two ways:

    - Easy way (if it suits your purposes):
      write a method returning `impl Iterator`

    - Slightly harder way: manually implement the Iterator trait

      Warning: you don't implement Iterator for your type!
      You implement it for a dedicated iterator type.
      Why is this?

    Example:
*/

#[derive(Debug)]
pub struct SongName(String);

#[derive(Debug)]
pub struct SongUserProfile {
    username: String,
    liked_songs: Vec<SongName>,
    disliked_songs: Vec<SongName>,
    listens: usize,
    days_active: usize,
}
impl SongUserProfile {
    // pub fn play_songs(&self) -> impl Iterator<Item = SongName> {
    //     // What should go here?
    //     unimplemented!()
    // }
}

/*
    Second way: implementing a dedicated iterator object

    This is what standard library traits do! See for example:
    - The std::slice::Iter struct:
      https://doc.rust-lang.org/std/slice/struct.Iter.html
    - The std::collections::hash_map::Iter struct:
      https://doc.rust-lang.org/std/collections/hash_map/struct.Iter.html

    These are NOT the same as Iterator, types, not traits.
    They are just called Iter out of convenience.
*/
