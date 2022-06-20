use criterion::criterion_main;
mod transient;
mod singleton;
mod context_dependent;

criterion_main! {
    transient::set::benches,
    transient::get::benches,
    transient::delete::benches,
    singleton::set::benches,
    singleton::get::benches,
    singleton::delete::benches,
    context_dependent::set::benches,
    context_dependent::get::benches,
}