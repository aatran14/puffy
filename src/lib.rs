#![cfg_attr(not(creusot), feature(stmt_expr_attributes))]
#![cfg_attr(not(creusot), feature(proc_macro_hygiene))]
#![allow(unused_imports)] // don't forget this future me.
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;

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

#[cfg_attr(not(creusot), derive(Serialize, Deserialize))]
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
#[ensures(result == true ==> forall<j: Int> 1 <= j && j < log@.len() ==> log@[j].seq_no@ > log@[j - 1].seq_no@)]
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
#[cfg_attr(not(creusot), derive(Serialize, Deserialize))]
 pub struct ManifestEntry {
      pub file_id: Uuid,
      pub seq_no: u64,
  }

#[cfg_attr(not(creusot), derive(Serialize, Deserialize))]
  pub struct Manifest {
      pub namespace: Uuid,
      pub files: Vec<ManifestEntry>,
  }

impl Invariant for Manifest { // manifest entries are always ordered by seq_no. the same prop that manifest_add #[requires] currently encorces manally.
    #[logic]
    fn invariant(self) -> bool {
        pearlite! {
        forall<i: Int, j: Int>
            0 <= i && i < j && j < self.files@.len() ==>
            self.files@[i].seq_no@ < self.files@[j].seq_no@
          }
      }
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


impl Invariant for WalBuffer {  // creusot to assume theinvariant holds when receiving a &mut WalBuffer and check that it sitlll holds when the borrow ends
      #[logic]
      fn invariant(self) -> bool {
          pearlite! {
              (forall<i: Int, j: Int>
                  0 <= i && i < j && j < self.pending@.len() ==>
                  self.pending@[i].seq_no@ < self.pending@[j].seq_no@)
              &&
              (self.pending@.len() > 0 ==>
                  self.next_seq@ > self.pending@[self.pending@.len() - 1].seq_no@)
          }
      }
  }

// buffer_write which takes a &mut WalBuffer, a Uuid and Vec<f32> to create a WalEntry
// this WalEntry in addition to the current next_seq, which is then pushed into pending and increments next_seq
// the goal is to get the server to assign sequence numbers. the caller will never touch them.

// #[ensures(buf.next_seq@ == old(buf.next_seq@) + 1)] // apparently creusot doesn't use old()
#[requires(buf.next_seq@ < u64::MAX@)]
#[ensures((^buf).next_seq@ == (*buf).next_seq@ + 1)]
#[ensures((^buf).pending@.len() == (*buf).pending@.len() + 1)]
pub fn buffer_write(buf: &mut WalBuffer, id: Uuid, values: Vec<f32>) {
    let entry = WalEntry {
    seq_no: buf.next_seq,
    id: id,
    values: values,
    };
    buf.pending.push(entry);
    buf.next_seq += 1;
}

#[requires(buf.pending@.len() > 0)]
#[ensures((^buf).pending@.len() == 0)]
#[ensures((^buf).next_seq@ == (*buf).next_seq@)]
pub fn buffer_clear(buf: &mut WalBuffer) -> u64 {
    let seq_no = buf.pending[buf.pending.len() - 1].seq_no;
    let next = buf.next_seq;
    buf.pending = Vec::new();
    buf.next_seq = next;
    seq_no
}

// buffer_flush into s3
  #[requires(buf.pending@.len() > 0)]
  #[trusted] // remove when S3 works
  #[cfg(not(creusot))]
pub async fn buffer_flush(buf: &mut WalBuffer, manifest: &mut Manifest, client: &aws_sdk_s3::Client, bucket: &str, namespace: Uuid) {
//   pub fn buffer_flush(buf: &mut WalBuffer, manifest: &mut Manifest) {
    let file_id = Uuid::new_v4();

    let bytes = serialize_wal(&buf.pending);
    let key = format!("ns/{}/wal/{}.bin", namespace, file_id);
        client.put_object()
        .bucket(bucket)
        .key(&key)
        .body(bytes.into())
        .send()
        .await
        .unwrap();
    let seq_no = buffer_clear(buf);
    manifest_add(manifest, ManifestEntry { file_id, seq_no });

    // let seq_no = buf.pending.last().unwrap().seq_no;
    // manifest_add(manifest, ManifestEntry { file_id, seq_no });
    // buf.pending.clear();
}

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

// fetch from the WAL
 #[cfg(not(creusot))]
pub async fn fetch_wal(client: &aws_sdk_s3::Client, bucket: &str, key: &str) -> Vec<WalEntry> {
    let resp = client.get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .unwrap();
    let bytes = resp.body.collect().await.unwrap().into_bytes();
    deserialize_wal(&bytes)
  }

// major brick building here

#[cfg(not(creusot))]
pub async fn manifest_save(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    namespace: Uuid,
    manifest: &Manifest,
    expected_etag: Option<&str>,
) -> Result<String, String> {
    // this function will serialize manifest to JSON
    // PUT to s3 with If-Match: expected_etag
    // if no etag (first write), use If-None-Match: *
    // return new etag on success, error on conflict
    let body = serde_json::to_vec(manifest).unwrap();
    let key = format!("ns/{}/manifest.json", namespace);

    let mut req = client.put_object()
        .bucket(bucket)
        .key(&key)
        .body(body.into());

    match expected_etag {
        Some(etag) => { req = req.if_match(etag); }
        None => { req = req.if_none_match("*"); }
    }

    match req.send().await {
        Ok(output) => {
            let new_etag = output.e_tag().unwrap_or("").to_string();
            Ok(new_etag)
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(not(creusot))]
pub async fn manifest_read(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    namespace: Uuid,
    cached_etag: Option<&str>,
) -> Option<(Manifest, String)> {
    // GET ns/{namespace}/manifest.json with If-None-Match
    // if S3 returns 304 (not modified), return None. cached version is still valid
    // if S3 returns the object, parse JSON into Manifest and return (manifest, etag)
    let key = format!("ns/{}/manifest.json", namespace);

    let mut req = client.get_object()
        .bucket(bucket)
        .key(&key);

    if let Some(etag) = cached_etag {
        req = req.if_none_match(etag);
    }

    match req.send().await {
        Ok(resp) => {
            let etag = resp.e_tag().unwrap_or("").to_string();
            let bytes = resp.body.collect().await.unwrap().into_bytes();
            let manifest: Manifest = serde_json::from_slice(&bytes).unwrap();
            Some((manifest, etag))
        }
        Err(_) => None, // 304 Not Modified or error
    }
}

#[cfg_attr(not(creusot), derive(Serialize, Deserialize))]
pub struct WriteRequest {
    pub id: Uuid,
    pub values: Vec<f32>,
}

#[cfg_attr(not(creusot), derive(Serialize, Deserialize))]
pub struct QueryRequest {
    pub vector: Vec<f32>,
    pub k: usize,
}

#[cfg(not(creusot))]
pub struct AppState {
    pub buf: tokio::sync::Mutex<WalBuffer>,
    pub manifest: tokio::sync::Mutex<Manifest>,
    pub manifest_etag: tokio::sync::Mutex<Option<String>>,
    pub flushed: tokio::sync::Mutex<Vec<Vector>>,
    pub client: aws_sdk_s3::Client,
    pub bucket: String,
    pub namespace: Uuid,
}

#[cfg(not(creusot))]
pub async fn handle_query(
    state: axum::extract::State<std::sync::Arc<AppState>>,
    axum::Json(payload): axum::Json<QueryRequest>,
) -> impl axum::response::IntoResponse {
    // read manifest (consistent read via GET-if-not-match with cached etag)
    // brute force search in-memory WAL data
    // return results as JSON
    let cached_etag = state.manifest_etag.lock().await.clone();
    if let Some((new_manifest, new_etag)) = manifest_read(&state.client, &state.bucket, state.namespace, 
    //cache_etag.deref())
    cached_etag.as_deref()).await() { // manifest changed, meaning few new WAL data from S3
        let mut flushed = state.flushed.lock().await;
        flushed.clear();
        for file in &new_manifest.files { 
            let key = format!("ns/{}/wal/{}.bin", state.namespace, file.file_id);
            let entries = fetch_wal(&state.client, &state.bucket, &key).await;
              for entry in entries {
                  flushed.push(Vector { id: entry.id, values: entry.values });
              }
        //let buf = state.buf.lock().await;
    // let flushed = state.flushed.lock().await;
    // let results = query(&payload.vector, &buf, &flushed, payload.k);
    // axum::Json(results)
    }
    drop(flushed);
    *state.manifest.lock().await = new_manifest;
    *state.manifest_etag.lock().await = Some(new_etag);
}


// NOT crash recovery. nodes are stateless. This is the cold query path:
// when a query arrives for a namespace that isn't cached, fetch its WAL data from S3
#[cfg(not(creusot))]
pub async fn cold_fetch(
    client: &aws_sdk_s3::Client,
    bucket: &str,
    namespace: Uuid,
) -> (Vec<Vector>, Manifest, String) {
    // read manifest from S3 (first read, no cached etag)
    // for each WAL file in manifest.files, fetch_wal from S3
    // convert WalEntries to Vectors
    // return (vectors, manifest, etag)
    let (manifest, etag) = manifest_read(client, bucket, namespace, None)
        .await
        .expect("no manifest found on S3");

    let mut vectors: Vec<Vector> = Vec::new();
    for file in &manifest.files {
        let key = format!("ns/{}/wal/{}.bin", namespace, file.file_id);
        let entries = fetch_wal(client, bucket, &key).await;
        for entry in entries {
            vectors.push(Vector { id: entry.id, values: entry.values });
        }
    }

    (vectors, manifest, etag)
}


// #[requires(buf.pending@.len() > 0)]
// #[requires(manifest.files@.len() == 0 ||buf.pending@[buf.pending@.len() - 1].seq_no@ > manifest.files@[manifest.files@.len() - 1].seq_no@)] // the last pending entry's seq_no is greater than the last manifest entry's seq_no.
// #[ensures((^buf).pending@.len() == 0)]
// #[ensures((^manifest).files@.len() == (*manifest).files@.len() + 1)]
// #[ensures((^buf).next_seq@ == (*buf).next_seq@)] // removed: testing if this is the stuck goal
// pub fn buffer_commit(buf: &mut WalBuffer, manifest: &mut Manifest, file_id: Uuid) {
//     let seq_no = buf.pending[buf.pending.len() - 1].seq_no;
//     let next = buf.next_seq;
//     buf.pending = Vec::new();
//     buf.next_seq = next;
//     manifest_add(manifest, ManifestEntry {file_id, seq_no});
// }

// buffer_commit was a hot mess. it took &mut WalBuffer and &mut Manifest in the same function. Two mutable borrrows meant Cruesot had to resolve both at function return. The prover was losing facts about one while reasoning the other. I referenced the docs which warn this, but even with the postconditions, the prover couldn't hold onto the buf states across the manfiest-add call.
// As a result, I think a rule of thumb is to do one &mut per verified function when possible.


#[cfg(not(creusot))]
pub async fn handle_write(
    state: axum::extract::State<Arc<AppState>>,
    axum::Json(payload): axum::Json<WriteRequest>,
) -> impl axum::response::IntoResponse {
    let mut buf = state.buf.lock().await;
    buffer_write(&mut buf, payload.id, payload.values);

    // flush when pending hits threshold
    if buf.pending.len() >= 10 {
        let mut manifest = state.manifest.lock().await;
        
        // testing copy the pending entries into flushed
        let mut flushed = state.flushed.lock().await;
        for entry in &buf.pending {
        flushed.push(Vector { id: entry.id, values: entry.values.clone() });
        }
        drop(flushed);
        buffer_flush(&mut buf, &mut manifest, &state.client, &state.bucket, state.namespace).await;

        let mut etag = state.manifest_etag.lock().await;
        match manifest_save(&state.client, &state.bucket, state.namespace, &manifest, etag.as_deref()).await {
            Ok(new_etag) => { *etag = Some(new_etag); }
            Err(e) => { eprintln!("manifest CAS failed: {}", e); }
        }
    }

    axum::http::StatusCode::OK
}