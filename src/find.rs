use {
  crate::args::{Algorithm, Alphabet},
  num::pow,
  std::{
    sync::{
      atomic::{AtomicBool, Ordering},
      Arc,
    },
    thread,
    time::Instant,
  },
};

pub const NUMERIC: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn alphabet_length(alphabet: Alphabet) -> usize {
  match alphabet {
    Alphabet::NUMERIC => NUMERIC.len(),
  }
}

pub fn find(
  plaintext_min_length: usize,
  plaintext_max_length: usize,
  _algorithm: Algorithm,
  alphabet: Alphabet,
  threads: usize,
  hash: String,
) {
  let now = Instant::now();
  let alphabet_length = alphabet_length(alphabet.clone());
  (plaintext_min_length..plaintext_max_length + 1).for_each(|plaintext_length| {
    let subspace = pow(alphabet_length, plaintext_length);
    let subspace_per_thread = subspace / threads;
    let _rest = subspace % threads;

    println!(
      "Subspace {}/(per thread {}) for current plaintext length {} ",
      subspace, subspace_per_thread, plaintext_length
    );

    let found = Arc::new(AtomicBool::new(false));
    let hash = Arc::new(hash.clone());

    let mut range_per_thread = 0;
    let mut handles = vec![];
    (0..threads).for_each(|_thread| {
      let found = found.clone();
      let hash = hash.clone();
      let handle = thread::spawn(move || {
        find_hash(
          plaintext_max_length,
          alphabet_length,
          plaintext_length,
          subspace_per_thread,
          range_per_thread,
          hash,
          found,
        );
      });
      handles.push(handle);
      range_per_thread += subspace_per_thread;
    });

    for handle in handles {
      handle.join().unwrap();
    }
  });
  println!("End time {:?}", now.elapsed());
}

fn find_hash(
  max_length: usize,
  alphabet_length: usize,
  plaintext_length: usize,
  subspace_per_thread: usize,
  range_per_thread: usize,
  hash: Arc<String>,
  found: Arc<AtomicBool>,
) {
  let mut plaintext = String::with_capacity(max_length);
  (range_per_thread..range_per_thread + subspace_per_thread).for_each(|index_range| {
    if found.load(Ordering::Relaxed) {
      return;
    }
    let mut index = index_range;
    (0..plaintext_length).for_each(|_| {
      plaintext.push(NUMERIC[index % alphabet_length]);
      index = index / alphabet_length;
    });
    let digest = format!("{:x}", md5::compute(&plaintext));
    if digest == *hash {
      println!("Found {:?} plaintext {}", digest, plaintext);
      found.store(true, Ordering::Relaxed);
    }
    plaintext.clear();
  });
}
