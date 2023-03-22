#[cfg(test)]
mod tests {
    use crate::{
        statistics::normalization::{
            clip_value, scale_value_centered, scale_value_down, scale_value_min_max, scale_value_up,
        },
        testing::comparison::FloatComparison,
    };

    static TEST_PRECISION_F64: f64 = 0.01;

    #[test]
    fn test_clip_value() {
        assert_eq!(clip_value(0.0, 0.0, 1.0), 0.0);
        assert_eq!(clip_value(1.0, 0.0, 1.0), 1.0);
        assert_eq!(clip_value(-2.0, 0.0, 1.0), 0.0);
        assert_eq!(clip_value(2.0, 0.0, 1.0), 1.0);
    }

    #[test]
    fn test_scale_value_up() {
        assert!(scale_value_up(-100.0, 70.0, 100.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_up(0.0, 70.0, 100.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_up(15.0, 70.0, 100.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_up(30.0, 70.0, 100.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_up(50.0, 70.0, 100.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_up(60.0, 70.0, 100.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_up(70.0, 70.0, 100.0).compare_with_precision(0.0, TEST_PRECISION_F64));

        assert!(scale_value_up(75.0, 70.0, 100.0).compare_with_precision(0.16, TEST_PRECISION_F64));
        assert!(scale_value_up(85.0, 70.0, 100.0).compare_with_precision(0.5, TEST_PRECISION_F64));
        assert!(scale_value_up(95.0, 70.0, 100.0).compare_with_precision(0.83, TEST_PRECISION_F64));
        assert!(scale_value_up(100.0, 70.0, 100.0).compare_with_precision(1.0, TEST_PRECISION_F64));
        assert!(scale_value_up(200.0, 70.0, 100.0).compare_with_precision(1.0, TEST_PRECISION_F64));
    }

    #[test]
    fn test_scale_value_down() {
        assert!(scale_value_down(-200.0, 30.0, 0.0).compare_with_precision(1.0, TEST_PRECISION_F64));
        assert!(scale_value_down(0.0, 30.0, 0.0).compare_with_precision(1.0, TEST_PRECISION_F64));
        assert!(scale_value_down(10.0, 30.0, 0.0).compare_with_precision(0.66, TEST_PRECISION_F64));
        assert!(scale_value_down(15.0, 30.0, 0.0).compare_with_precision(0.5, TEST_PRECISION_F64));
        assert!(scale_value_down(25.0, 30.0, 0.0).compare_with_precision(0.166, TEST_PRECISION_F64));
        assert!(scale_value_down(30.0, 30.0, 0.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_down(35.0, 30.0, 0.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_down(50.0, 30.0, 0.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_down(65.0, 30.0, 0.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_down(70.0, 30.0, 0.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_down(75.0, 30.0, 0.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_down(95.0, 30.0, 0.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_down(100.0, 30.0, 0.0).compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_down(200.0, 30.0, 0.0).compare_with_precision(0.0, TEST_PRECISION_F64));
    }

    #[test]
    fn test_scale_value_centered() {
        assert!(scale_value_centered(0.0, 50.0, 30.0, 70.0)
            .compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_centered(15.0, 50.0, 30.0, 70.0)
            .compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_centered(25.0, 50.0, 30.0, 70.0)
            .compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_centered(30.0, 50.0, 30.0, 70.0)
            .compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_centered(40.0, 50.0, 30.0, 70.0)
            .compare_with_precision(0.5, TEST_PRECISION_F64));
        assert!(scale_value_centered(50.0, 50.0, 30.0, 70.0)
            .compare_with_precision(1.0, TEST_PRECISION_F64));
        assert!(scale_value_centered(60.0, 50.0, 30.0, 70.0)
            .compare_with_precision(0.5, TEST_PRECISION_F64));
        assert!(scale_value_centered(70.0, 50.0, 30.0, 70.0)
            .compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_centered(75.0, 50.0, 30.0, 70.0)
            .compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_centered(85.0, 50.0, 30.0, 70.0)
            .compare_with_precision(0.0, TEST_PRECISION_F64));
        assert!(scale_value_centered(100.0, 50.0, 30.0, 70.0)
            .compare_with_precision(0.0, TEST_PRECISION_F64));
    }

    #[test]
    fn test_scale_value_min_max() {
        assert!(
            scale_value_min_max(0.0, 0.0, 100.0).compare_with_precision(-1.0, TEST_PRECISION_F64)
        );
        assert!(
            scale_value_min_max(15.0, 0.0, 100.0).compare_with_precision(-0.7, TEST_PRECISION_F64)
        );
        assert!(
            scale_value_min_max(25.0, 0.0, 100.0).compare_with_precision(-0.5, TEST_PRECISION_F64)
        );
        assert!(
            scale_value_min_max(30.0, 0.0, 100.0).compare_with_precision(-0.4, TEST_PRECISION_F64)
        );
        assert!(
            scale_value_min_max(40.0, 0.0, 100.0).compare_with_precision(-0.2, TEST_PRECISION_F64)
        );
        assert!(
            scale_value_min_max(50.0, 0.0, 100.0).compare_with_precision(0.0, TEST_PRECISION_F64)
        );
        assert!(
            scale_value_min_max(60.0, 0.0, 100.0).compare_with_precision(0.2, TEST_PRECISION_F64)
        );
        assert!(
            scale_value_min_max(70.0, 0.0, 100.0).compare_with_precision(0.4, TEST_PRECISION_F64)
        );
        assert!(
            scale_value_min_max(75.0, 0.0, 100.0).compare_with_precision(0.5, TEST_PRECISION_F64)
        );
        assert!(
            scale_value_min_max(85.0, 0.0, 100.0).compare_with_precision(0.7, TEST_PRECISION_F64)
        );
        assert!(
            scale_value_min_max(100.0, 0.0, 100.0).compare_with_precision(1.0, TEST_PRECISION_F64)
        );
    }
}
