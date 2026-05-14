
// need a way to compute how far two vectors are. in a vector db, every search compares a query vector stored against the stored vectors to find knn or some subset of that.

// we are going to also use creusot to practice programming that's correect by construction. treat this as a mathematics exercise. it's daunting, but it's the right way to move forward in databases.

extern crate creusot_std;
use creusot_std::prelude::*;


// method1: ecludian_distance_squared
// with creusot annotations
#[requires()] // we need to promise that slides of a and b are the same length. it is too much headache to allow mismatched slices. 
#[ensures()] // verify that the result will never be negative



pub fn euclidian_distance_squared(a: &[f32], b: &[f32]) -> f32 {


}