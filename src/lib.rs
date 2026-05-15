  #![cfg_attr(not(creusot), feature(stmt_expr_attributes))]
  #![cfg_attr(not(creusot), feature(proc_macro_hygiene))]

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

use std::collections::BinaryHeap;


// If structs are correct, then follow through with brute_force_topk function
#[requires(k@ > 0)]
#[requires(vectors@.len() > 0)]
#[trusted]
#[cfg(not(creusot))]
fn brute_force_topk(query: &[f32], vectors: &[Vector], k: usize) -> Vec<QueryResult> {
    let mut heap = BinaryHeap::new();

    for i in 0..vectors.len() {
        let dist = euclidian_distance_squared(query, &vectors[i].values);
        // push into heap
        heap.push(Scored {distance: dist, id: vectors[i].id});
    }

    let mut results: Vec<QueryResult> = Vec::new();
    let count = k.min(heap.len());
    let mut j: usize = 0;
    while j < count {
        let s: Scored = heap.pop().unwrap();
        results.push(QueryResult {id: s.id, distance: s.distance});
        j += 1;
    }

    results

    
}

#[cfg(not(creusot))]
struct Scored {
    distance: f32,
    id: Uuid,
}

#[cfg(not(creusot))]
impl PartialEq for Scored {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

#[cfg(not(creusot))]
impl Eq for Scored {}

#[cfg(not(creusot))]
impl PartialOrd for Scored { 
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))

    }
}

#[cfg(not(creusot))]
impl Ord for Scored {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.partial_cmp(&self.distance)
        .unwrap_or(std::cmp::Ordering::Equal)
    }
}

// WAL - Write Ahead Log


pub struct WalEntry {
    pub seq_no: u64, // unsigned64bit, helpful for monotonically increasing counter, which we can further prove via Creusot is always ordered. The id and the values are the vector data being written.
    pub id: Uuid,
    pub values: Vec<f32>,
}

// #[ensures(log@.len() == old(log@.len()) + 1)]
// ensure seq_no
#[requires(log@.len() == 0 || entry.seq_no > log@[log@.len() - 1].seq_no)] // &mut Vec<WalEntry> beucase it modifies the log (push)
pub fn wal_append(log: &mut Vec<WalEntry>, entry: WalEntry) {
    log.push(entry);
}


// Interesting! Creusot couldn't fully verify the wal_replay without more help. I did see that onl 8/9 goals proved, but one failed.
// alpha: Need a way to see what is true as each step of the while loop.
// pub fn wal_replay(log: &mut &[WalEntry]) -> bool { // &[WalEntry] because it only reads the log (chcecks if it sorted)
pub fn wal_replay(log: &[WalEntry]) -> bool {
    let mut i: usize = 1; // what even is usize and why do we even use it.


// alpha: prover needs to know what's true at each step of the loop
// for every integer j where 1 <= j <= where we've checked so far 
// it's true that entry j has a bigger seq_no than entry j-1
#[invariant(i@ >=1)]
#[invariant(forall<j: Int> 1 <= j && j < i@ ==> log@[j].seq_no@ > log@[j - 1].seq_no@)]
    while i < log.len() {
        if log[i].seq_no <= log[i-1].seq_no {
            return false;
        }
        i += 1;
    }
    true
}