//! Contains various value types.

use num::{rational::Ratio, Zero};

use crate::{
    preintegral::Integrable,
    time::Instant,
    timeline::{Timeline, TimelineError},
};

/// Represents beat event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Beat(pub Ratio<usize>);

impl Integrable<usize> for Beat {
    type Output = Ratio<usize>;

    fn integrate_within(&self, self_time: usize, target_time: usize) -> Self::Output {
        self.0 * (target_time - self_time) as usize
    }

    fn accumlate(lhs: Self::Output, rhs: Self::Output) -> Self::Output {
        lhs + rhs
    }

    fn zero() -> Self::Output {
        Ratio::zero()
    }
}

/// Represents tempo event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tempo(pub Ratio<usize>);

/// Represents rhythm change event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RhythmChange(pub Beat, pub Tempo);

pub fn merge_beats_and_tempo(
    beats: Timeline<usize, Beat>,
    tempos: Timeline<Instant, Tempo>,
) -> Result<Timeline<Instant, RhythmChange>, TimelineError> {
    let beats: Timeline<_, _> = beats
        .into_pairs()
        .map(|(u, b)| (Instant::new_parts(u, 0, 1), b))
        .collect();

    let mut merged_pairs = beats.merge(tempos)?.into_pairs();
    let Some((must_zero, (Some(first_beat), Some(first_tempo)))) = merged_pairs.next() else {
        return Err(TimelineError::NotZeroAligned);
    };
    if must_zero != Instant::zero() {
        return Err(TimelineError::NotZeroAligned);
    }

    let mut timeline = Timeline::new();
    timeline.append(must_zero, RhythmChange(first_beat, first_tempo));
    let (timeline, _, _) = merged_pairs.fold(
        (timeline, first_beat, first_tempo),
        |(mut tl, lb, lt), (i, (b, t))| {
            let next_beat = b.unwrap_or(lb);
            let next_tempo = t.unwrap_or(lt);
            tl.append(i, RhythmChange(next_beat, next_tempo));
            (tl, next_beat, next_tempo)
        },
    );

    Ok(timeline)
}

#[cfg(test)]
mod tests {
    #[test]
    fn rhythm_change_merge_works() {

    }
}
