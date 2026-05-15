
// need a way to compute how far two vectors are. in a vector db, every search compares a query vector stored against the stored vectors to find knn or some subset of that.

// we are going to also use creusot to practice programming that's correect by construction. treat this as a mathematics exercise. it's daunting, but it's the right way to move forward in databases.

extern crate creusot_std;
use creusot_std::prelude::*;



// m0: ecludian_distance_squared
// with creusot annotations
#[requires( // we need to promise that slides of a and b are the same length. it is too much headache to allow mismatched slices. 
    a@.len() == b@.len()
)] 

// #[ensures(result >= 0.0)] // verify that the result will never be negative
//  #774 on Creusot PR says Parital Ord does not have support. Come back to this later. perhaps i should write the proof for partial ord f32 floats for creusot.
pub fn euclidian_distance_squared(a: &[f32], b: &[f32]) -> f32 {
    let mut sum: f32 = 0.0;
    let mut i: usize = 0;

    // #[invariant(sum >= 0.0)] // creusot has no float comparison. Clear to me in hindsight.
    while i < a.len() {
        let d = a[i] - b[i];
        sum += d * d; // distance squared
        i += 1;
    }

    sum

}

// m1: brute_force_topk

use uuid::Uuid; // because you should always use them. UUIDs are fixed-size and cheap to compare. Strings are heap-allocated and variable length. So like wtf.

pub struct Vector {
    pub id: Uuid,
    pub values: Vec<f32>,
}

pub struct QueryResult {
    pub id: Uuid,
    pub distance: f32,
}

// If structs are correct, then follow through with brute_force_topk function
fn brute_force_topk(query: &[f32], vectors: &[Vector], k: usize) -> Vec<QueryResult> {
    let mut results: Vec<QueryResult> = Vec::new();
    for i in 0..vectors.len() {
        let dist = euclidian_distance_squared(query, &vectors[i].values);
        results.push(QueryResult {
            id: vectors[i].id,
            distance: dist,
        })
      

        // sort via heap maybe? reasonable but f32 is not Ord. Probably need a wrapper struct. I imagine RaBitQ makes this much easier.
        
    }
}

use std::collections::BinaryHeap;

struct Scored {
    distance: f32,
    id: Uuid,
}

impl PartialEq for Scored {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for Scored {}

impl PartialOrd for Scored { 
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))

    }
}

impl Ord for Scored {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.partial_cmp(&self.distance)
        .unwrap_or(std::cmp::Ordering::Equal)
    }
}

