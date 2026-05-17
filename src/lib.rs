#![cfg_attr(not(creusot), feature(stmt_expr_attributes))]
#![cfg_attr(not(creusot), feature(proc_macro_hygiene))]
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
pub fn brute_force_topk(query: &[f32], vectors: &[Vector], k: usize) -> Vec<QueryResult> {
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

// #[derive(Serialize, Deserialize)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize))]
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


// manifest 
 pub struct ManifestEntry {
      pub file_id: Uuid,
      pub seq_no: u64,
  }

  pub struct Manifest {
      pub namespace: Uuid,
      pub files: Vec<ManifestEntry>,
  }

// this annotation says that if the manifest is empty OR the new entry seq_no must be bigger than the last one in the list. 
#[requires(manifest.files@.len() == 0 || entry.seq_no@ > manifest.files@[manifest.files@.len() - 1].seq_no@)]  
pub fn manifest_add(manifest: &mut Manifest, entry: ManifestEntry) {
    manifest.files.push(entry);
}

// wal buffer
pub struct WalBuffer {
    pub next_seq: u64, // unsigned64, next_seq is the counter that increments with each write.
    pub pending: Vec<WalEntry>, // pending holds entries waiting to be flushed into S3
}

// buffer_write which takes a &mut WalBuffer, a Uuid and Vec<f32> to create a WalEntry
// this WalEntry in addition to the current next_seq, which is then pushed into pending and increments next_seq
// the goal is to get the server to assign sequence numbers. the caller will never touch them.

// #[ensures(buf.next_seq@ == old(buf.next_seq@) + 1)] // apparently creusot doesn't use old()
#[requires(buf.next_seq@ < u64::MAX@)]
#[ensures((^buf).next_seq@ == (*buf).next_seq@ + 1)]
pub fn buffer_write(buf: &mut WalBuffer, id: Uuid, values: Vec<f32>) {
    let entry = WalEntry {
    seq_no: buf.next_seq,
    id: id,
    values: values,
    };
    buf.pending.push(entry);
    buf.next_seq += 1;
}

// buffer_flush into s3
  #[requires(buf.pending@.len() > 0)]
  #[trusted] // remove when S3 works
  #[cfg(not(creusot))]
pub async fn buffer_flush(buf: &mut WalBuffer, manifest: &mut Manifest, client: &aws_sdk_s3::Client, bucket: &str, namespace: Uuid) {
//   pub fn buffer_flush(buf: &mut WalBuffer, manifest: &mut Manifest) {
    let file_id = Uuid::new_v4();

    let bytes = serialize_wal(&buf.pending);
    let key = format!("ns/{}/wal/{}.bin", namespace, file_id);    client.put_object()
        .bucket(bucket)
        .key(&key)
        .body(bytes.into())
        .send()
        .await
    .unwrap();
    
    let seq_no = buf.pending.last().unwrap().seq_no;
    manifest_add(manifest, ManifestEntry { file_id, seq_no});
    buf.pending.clear();


    manifest_add(manifest, ManifestEntry { file_id, seq_no });
    buf.pending.clear();
}

  // 
#[cfg(not(creusot))]
pub fn query(query: &[f32], buf: &WalBuffer, flushed: &[Vector], k: usize) -> Vec<QueryResult> {
    let mut all: Vec<Vector> = Vec::new();

    for v in flushed { // loop through flushed, clone each into all
        all.push(Vector { id: v.id, values: v.values.clone() });
    }

    for entry in &buf.pending { // loop through buf.pending, convert each WalEntry to Vector, push into all
        all.push(Vector { id: entry.id, values: entry.values.clone() });
    }


    brute_force_topk(query, &all, k)
}

// serialization to the wal. simple cedes complex.
#[cfg(not(creusot))]
pub fn serialize_wal(entries: &[WalEntry]) -> Vec<u8> {
    serde_json::to_vec(entries).unwrap()
}

#[cfg(not(creusot))]
pub fn deserialize_wal(bytes: &[u8]) -> Vec<WalEntry> {
      serde_json::from_slice(bytes).unwrap()
  }

// the model is that we have multiple WAL files sitting on S3. Each one is just a batch of vectors. Compaction would merge them into one sorted file (an SS-table) so queries read fewer files.
// simple: dump all entires into one Vec, sort by id, deduplicate. This is one pass and easy to understand. O(n log n)
// complex: tiered compaction. multiple levels, size ratios, partial merges. keep the hot data in smaller levels. Better at scale.

// > simple should be good enough for 100ms per roundtrip. both achieve similar order of magnitude of functionality while simple remaining well... simple.


#[cfg(not(creusot))]
pub fn compact(wal_files: &[Vec<WalEntry>]) -> Vec<WalEntry> {
    // 1. flatten all the entries into one Vec<WalEntry>
    // 2. deducplicate by id. keep the ntry with the highest seq_no for each UUID.
    // 3. sort by seq_no. 

    // 2. probably means use a HashMap for the deudplicate step.
    let mut all: Vec<WalEntry> = Vec::new();
    
    // for i in 0..wal_files.len(){
    //     for j in 0..wal_fiels[i].len() {
    //         all.push(WalEntry { seq_no: wal_files[i][j].seq_no, id: wal_files[i][j].id, values: wal_files[i][j].values.clone() });        
    //     }
    // }

    // realized that version did not present the loop as simple as it really is. below is better.
    for file in wal_files {
        for entry in file {
            all.push(WalEntry {seq_no: entry.seq_no, id: entry.id, values: entry.values.clone()})
        }
    }

    let mut map: HashMap<Uuid, WalEntry> = HashMap::new();
        for entry in all {
            if let Some(existing) = map.get(&entry.id) {
                if entry.seq_no > existing.seq_no {
                    map.insert(entry.id, entry);
                }
            } else {
            map.insert(entry.id, entry);
            }
    }
    // all
    let mut result: Vec<WalEntry> = map.into_values().collect();
    result.sort_by_key(|e| e.seq_no);
    result
    // map.into_values().collect() // ultimately to pull deudplicated entries otu of the map and into the Vec
}

// buffer_flush needs to actually PUT bytes into S3. It currently does not. Right now, it just clears the buffer
// 1. serilaize pending entries to bytes (which we have via serialize_wal)
// 2. PUT those bytes to a prefix on S3
// 3. update the manifest

// The things to note are that S3 calls are sync- they use await. Meaning buffer_flush needs to become async fn. The same thought applies to main().