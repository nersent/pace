use super::feature::Feature;

pub trait FeatureBuilder<T: Feature> {
    fn next(&mut self) -> T;
}
