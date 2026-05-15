use tpuf_v1::{WalBuffer, WalEntry, buffer_write, buffer_flush, Manifest, query, Vector, serialize_wal, deserialize_wal};
use uuid::Uuid;

fn main() {
    let mut buf = WalBuffer {
        next_seq: 0,
        pending: Vec::new(),
    };
    
   

    buffer_write(&mut buf, Uuid::new_v4(), vec![1.0, 0.0, 0.0]);
    buffer_write(&mut buf, Uuid::new_v4(), vec![0.0, 1.0, 0.0]);
    buffer_write(&mut buf, Uuid::new_v4(), vec![0.0, 0.0, 1.0]);

    let bytes = serialize_wal(&buf.pending);
    let back = deserialize_wal(&bytes);
    println!("serialized {} entries, {} bytes", back.len(), bytes.len());

    let mut manifest = Manifest {
        namespace: Uuid::new_v4(),
        files: Vec::new(),
    };

    // save vectors before flushing (in order to simulatw what S3 would store)
    let flushed: Vec<Vector> = buf.pending.iter().map(|e| Vector {
        id: e.id,
        values: e.values.clone(),
    }).collect();

    buffer_flush(&mut buf, &mut manifest);

    let q = vec![0.9, 0.1, 0.0];
    let results = query(&q, &buf, &flushed, 2);

    println!("query: {:?}", q);
    for r in &results {
        println!("  id: {}, distance: {}", r.id, r.distance);
    }
    
  }
