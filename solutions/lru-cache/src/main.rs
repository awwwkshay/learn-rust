use lru_cache::LruCache;

fn main() {
    let mut cache = LruCache::new(2).expect("positive capacity");
    println!("{:?}", cache.insert("alpha", 10));
    println!("{:?}", cache.insert("beta", 20));
    println!("alpha: {:?}", cache.get(&"alpha"));
    println!("{:?}", cache.insert("gamma", 30));
    println!("beta: {:?}", cache.get(&"beta"));
}
