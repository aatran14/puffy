 use tpuf_v1::euclidian_distance_squared;

  fn main() {
      let a = vec![1.0, 2.0, 3.0];
      let b = vec![4.0, 5.0, 6.0];
      let d = euclidian_distance_squared(&a, &b);
      println!("huzzah: {}", d);
  }