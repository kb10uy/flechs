use flechs::{preintegral::Preintegral, timeline, value::Beat};
use num::rational::Ratio;

#[test]
fn check_beats_timeline() {
    let tl = timeline! {
        [0]: Beat(Ratio::new(4, 1)),
        [4]: Beat(Ratio::new(7, 1)),
        [6]: Beat(Ratio::new(4, 1)),
        [8]: Beat(Ratio::new(7, 2)),
    };
    let pitl = Preintegral::new(tl);

    assert_eq!(pitl.fetch(0), Ratio::new(0, 1), "beat calculation works");
    assert_eq!(pitl.fetch(2), Ratio::new(8, 1), "beat calculation works");
    assert_eq!(pitl.fetch(4), Ratio::new(16, 1), "beat calculation works");
    assert_eq!(pitl.fetch(5), Ratio::new(23, 1), "beat calculation works");
    assert_eq!(pitl.fetch(6), Ratio::new(30, 1), "beat calculation works");
    assert_eq!(pitl.fetch(8), Ratio::new(38, 1), "beat calculation works");
    assert_eq!(pitl.fetch(9), Ratio::new(83, 2), "beat calculation works");
    assert_eq!(pitl.fetch(10), Ratio::new(90, 2), "beat calculation works");
}
