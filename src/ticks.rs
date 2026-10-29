/// Round values for an axis's ticks, from [`nice_ticks`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ticks {
    /// The axis's top value: the highest value rounded up to a whole step.
    pub max: f64,
    /// The value between two ticks: 1, 2 or 5 × 10ⁿ.
    pub step: f64,
    /// How many steps the axis has (`max / step`), for the `divisions` of
    /// [`YAxisParams`](crate::YAxisParams) and the `y_divisions` of [`GridParams`](crate::GridParams).
    pub divisions: usize,
    /// The decimal places the step needs, for [`YAxisParams`](crate::YAxisParams)'s `decimals`.
    pub decimals: u32,
}

/// Picks round tick values for an axis from 0 to `max`, with about `target` divisions: the step is
/// 1, 2 or 5 × 10ⁿ, and the top value is `max` rounded up to a whole step.
///
/// The top value is the axis's scale, so the caller scales its points by it too. A `target` of 0
/// counts as 1, and a `max` that is not positive and finite as 1, so the axis still has ticks.
///
/// ```
/// use vertigo_graphs::nice_ticks;
///
/// let ticks = nice_ticks(51.0, 5);
/// assert_eq!((ticks.max, ticks.step, ticks.divisions, ticks.decimals), (60.0, 10.0, 6, 0));
///
/// let ticks = nice_ticks(2.38, 5);
/// assert_eq!((ticks.max, ticks.step, ticks.divisions, ticks.decimals), (2.5, 0.5, 5, 1));
/// ```
pub fn nice_ticks(max: f64, target: usize) -> Ticks {
    let max = if max.is_finite() && max > 0.0 {
        max
    } else {
        1.0
    };
    let raw = max / target.max(1) as f64;
    let exp = raw.log10().floor() as i32;
    // Rounds the raw step to 1, 2, 5 or 10 × 10^exp, at the geometric means between them
    let (factor, exp) = match shift(raw, -exp) {
        error if error >= 50f64.sqrt() => (1.0, exp + 1),
        error if error >= 10f64.sqrt() => (5.0, exp),
        error if error >= 2f64.sqrt() => (2.0, exp),
        _ => (1.0, exp),
    };
    let step = shift(factor, exp);
    // Less a hair, so a `max` on a step does not get a step more from floating point error
    let divisions = ((max / step - 1e-9).ceil() as usize).max(1);
    Ticks {
        max: shift(divisions as f64 * factor, exp),
        step,
        divisions,
        decimals: exp.min(0).unsigned_abs(),
    }
}

/// `value × 10^exp`, dividing for a negative `exp` so that 0.5, 0.02 or 0.1 come out as the
/// nearest `f64`, as if written.
fn shift(value: f64, exp: i32) -> f64 {
    if exp >= 0 {
        value * 10f64.powi(exp)
    } else {
        value / 10f64.powi(-exp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ticks(max: f64, target: usize) -> (f64, f64, usize, u32) {
        let ticks = nice_ticks(max, target);
        (ticks.max, ticks.step, ticks.divisions, ticks.decimals)
    }

    #[test]
    fn steps_are_one_two_or_five_times_a_power_of_ten() {
        assert_eq!(ticks(51.0, 5), (60.0, 10.0, 6, 0));
        assert_eq!(ticks(56.1, 5), (60.0, 10.0, 6, 0));
        assert_eq!(ticks(8.25, 5), (10.0, 2.0, 5, 0));
        assert_eq!(ticks(730.0, 5), (800.0, 200.0, 4, 0));
        assert_eq!(ticks(1_000.0, 5), (1_000.0, 200.0, 5, 0));
    }

    #[test]
    fn small_steps_need_decimals() {
        assert_eq!(ticks(2.38, 5), (2.5, 0.5, 5, 1));
        assert_eq!(ticks(0.09, 5), (0.1, 0.02, 5, 2));
        assert_eq!(ticks(0.07, 5), (0.07, 0.01, 7, 2));
    }

    #[test]
    fn max_on_a_step_is_the_top() {
        assert_eq!(ticks(50.0, 5), (50.0, 10.0, 5, 0));
        assert_eq!(ticks(3.0, 5), (3.0, 0.5, 6, 1));
        assert_eq!(ticks(0.3, 5), (0.3, 0.05, 6, 2));
    }

    #[test]
    fn target_sets_how_many_divisions() {
        assert_eq!(ticks(100.0, 10), (100.0, 10.0, 10, 0));
        assert_eq!(ticks(100.0, 2), (100.0, 50.0, 2, 0));
        assert_eq!(ticks(100.0, 0), (100.0, 100.0, 1, 0));
    }

    #[test]
    fn no_range_still_gets_ticks() {
        for max in [0.0, -5.0, f64::NAN, f64::INFINITY] {
            assert_eq!(ticks(max, 5), (1.0, 0.2, 5, 1), "{max}");
        }
    }
}
