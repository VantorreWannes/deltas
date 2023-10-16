#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench_patch {
    use test::Bencher;


    #[bench]
    fn bench_pow(b: &mut Bencher) {
        b.iter(|| {
            
        });
    }
}