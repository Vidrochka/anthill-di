use crate::injector::Injector;

pub trait Injection {
    fn build_injection(injector: &mut Injector) -> Self;
}

impl<T> Injection for Box<T> where T: Injection {
    fn build_injection(injector: &mut Injector) -> Self {
        Box::new(T::build_injection(injector))
    }
}

macro_rules! injection {
    ($type:ty, $build_call:expr) => {
        impl Injection for $type {
            fn build_injection(_: &mut Injector) -> Self {
                $build_call
            }
        }
    }
}

injection!(String, "".to_string());
injection!(&str, "");
injection!(u8, 0);
injection!(i8, 0);
injection!(u16, 0);
injection!(i16, 0);
injection!(u32, 0);
injection!(i32, 0);
injection!(u64, 0);
injection!(i64, 0);
injection!(u128, 0);
injection!(i128, 0);
injection!(usize, 0);
injection!(isize, 0);
injection!(f32, 0.0);
injection!(f64, 0.0);