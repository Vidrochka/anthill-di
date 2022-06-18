use criterion::criterion_main;
mod transient;
mod singleton;
mod scoped;

criterion_main! {
    transient::single_set::benches,
    transient::single_set_get::benches,
    singleton::single_set::benches,
    singleton::single_set_get::benches,
    scoped::single_set::benches,
    scoped::single_set_get::benches,
}