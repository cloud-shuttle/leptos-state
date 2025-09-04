use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos_state::v1::*;

fn benchmark_store_creation(c: &mut Criterion) {
    #[derive(Clone, Debug, PartialEq, Default)]
    struct SimpleStore {
        count: i32,
    }

    impl StoreState for SimpleStore {}

    impl Store for SimpleStore {
        fn create() -> Self {
            Self::default()
        }
        
        fn create_with_state(state: Self) -> Self {
            state
        }
        
        fn update<F>(&mut self, f: F) 
        where 
            F: FnOnce(&mut Self) {
            f(self);
        }
        
        fn get(&self) -> &Self {
            self
        }
        
        fn get_mut(&mut self) -> &mut Self {
            self
        }
    }

    c.bench_function("simple_store_creation", |b| {
        b.iter(|| {
            let _store = SimpleStore::create();
        });
    });

    c.bench_function("simple_store_update", |b| {
        let mut store = SimpleStore::create();
        b.iter(|| {
            store.update(|state| state.count += 1);
        });
    });
}

criterion_group!(benches, benchmark_store_creation);
criterion_main!(benches);
