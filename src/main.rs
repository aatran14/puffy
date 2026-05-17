use tpuf_v1::{WalBuffer, WalEntry, buffer_write, buffer_flush, Manifest, query, Vector, serialize_wal, deserialize_wal, compact};
use uuid::Uuid;

#[tokio::main]
async fn main() {
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

    // buffer_flush(&mut buf, &mut manifest);
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);
    let namespace = manifest.namespace;

    let bucket = std::env::var("PUFFY_BUCKET").expect("set PUFFY_BUCKET env var");
    buffer_flush(&mut buf, &mut manifest, &client, &bucket, namespace).await;
    // pls work

    let q = vec![0.9, 0.1, 0.0];
    let results = query(&q, &buf, &flushed, 2);

    println!("query: {:?}", q);
    for r in &results {
        println!("  id: {}, distance: {}", r.id, r.distance);
    }

    // test compaction. id1 appears in both WAL files, should dedup to seq_no 2
    let id1 = Uuid::new_v4();
    let wal1 = vec![
        WalEntry { seq_no: 0, id: id1, values: vec![1.0, 0.0] },
        WalEntry { seq_no: 1, id: Uuid::new_v4(), values: vec![0.0, 1.0] },
    ];
    let wal2 = vec![
        WalEntry { seq_no: 2, id: id1, values: vec![9.0, 9.0] },
    ];

    let compacted = compact(&[wal1, wal2]);
    println!("compacted: {} entries", compacted.len());
    for e in &compacted {
        println!("  seq: {}, id: {}, values: {:?}", e.seq_no, e.id, e.values);
    }
}
