use std::{
    cell::{Ref, RefCell},
    ops::RangeInclusive,
    rc::Rc,
};

use crate::{Pose, Time};

/// Aggregate of [`ColumnData`] that can only be added and not deleted.
/// Length of all [`ColumnData`] are equal, making this effectively a 2D table.
pub type DataSet<T = f32> = Vec<ColumnData<T>>;

/// Easily accessible wrapper around [`TimeTable`] to be shared by multiple widgets
pub struct DataStore(pub(crate) Rc<RefCell<TimeTable<Pose>>>);

impl Default for DataStore {
    fn default() -> Self {
        // Temporary, create dummy data for showing
        use crate::math::{cos, sin};
        use std::f32::consts::PI;

        let n = 10000;
        let dt = 0.01;
        let t: Vec<f32> = (0..n).map(|n| n as f32 * dt).collect();
        let pose: Vec<Pose> = t
            .iter()
            .map(|&t| Pose::new(30.0 * sin(t), 20.0 * cos(t), 2.0 * PI * sin(t)))
            .collect();

        Self(Rc::new(RefCell::new(TimeTable::new(t, pose))))
    }
}

impl DataStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clone(&self) -> Self {
        DataStore(Rc::clone(&self.0))
    }

    pub fn borrow(&self) -> Ref<TimeTable<Pose>> {
        self.0.borrow()
    }
}

/// Single column of data
#[derive(Debug, Default, Clone)]
pub struct ColumnData<T> {
    data: Vec<T>,
    name: Option<String>,
}

impl<T> ColumnData<T> {
    pub fn from_vec(data: impl Into<Vec<T>>) -> Self {
        Self {
            data: data.into(),
            name: None,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn add(&mut self, element: T) {
        self.data.push(element);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_between(&self, range: RangeInclusive<usize>) -> &[T] {
        &self.data[range]
    }
}

/// Basic time vector for finding indices to look up within [`TimeSeries`] and [`TimeTable`]
#[derive(Debug, Default)]
struct Timeline {
    /// Cache stores previously found index to avoid unecessary iteration when finding time index
    cache: Option<(Time, usize)>,
    /// Actual vector
    vec: Vec<Time>,
}

impl Into<Timeline> for Vec<Time> {
    fn into(self) -> Timeline {
        Timeline {
            vec: self,
            ..Default::default()
        }
    }
}

impl Timeline {
    /// Tolerance to compare two input time for their equality
    const EPSILON: f32 = 0.0005;

    pub fn new(time_vec: impl Into<Vec<Time>>) -> Self {
        Self {
            vec: time_vec.into(),
            ..Default::default()
        }
    }

    /// Adds a time element to the end
    pub fn add(&mut self, time: Time) {
        self.vec.push(time);
    }

    /// Checks if time input has changed from last index search
    /// If time input is sufficiently close, assume same index can be used without calling [`get_index`]
    ///
    /// [`get_index`]: Self::get_index
    fn time_changed(&self, time: Time) -> bool {
        self.cache
            .map_or(true, |(prev, _)| (time - prev).abs() > Self::EPSILON)
    }

    /// Find the index that corresponds to the given time in seconds.
    ///
    /// Returns index of first time that is greater or equal to the specified time.
    fn get_index(&self, time: Time) -> Option<usize> {
        if self.time_changed(time) {
            self.vec.iter().position(|&t| t >= time).map(|index| {
                // self.cache = Some((time, index));
                index
            })
        } else {
            // unwrap here is ok, since time_changed always ensures cache is not None
            Some(self.cache.unwrap().1)
        }
    }

    /// Similar to [`get_index`], but only returns time index that is smaller than the input time.
    /// This is useful when making sure the returned time index never exceeds the given time, as
    /// in [`get_range`]
    ///
    /// [`get_index`]: Self::get_index
    /// [`get_range`]: Self::get_range
    fn get_index_under(&self, time: Time) -> Option<usize> {
        if self.time_changed(time) {
            self.vec
                .iter()
                .position(|&t| t > time)
                .map(|idx| (idx - 1).max(0))
                .map(|index| {
                    // self.cache = Some((time, index));
                    index
                })
        } else {
            // unwrap here is ok, since time_changed always ensures cache is not None
            Some(self.cache.unwrap().1)
        }
    }

    /// Returns range indices that is within the time range specified
    pub fn get_range(&self, start: Time, end: Time) -> Option<RangeInclusive<usize>> {
        if start < end {
            if let Some(start) = self.get_index(start) {
                if let Some(end) = self.get_index_under(end) {
                    return Some(start..=end);
                }
            }
        }
        None
    }

    pub fn get_range_raw(&self, start: Time, end: Time) -> Option<Vec<Time>> {
        self.get_range(start, end)
            .map(|range| self.vec[range].to_vec())
    }

    /// Length of the time vector
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

#[derive(Debug, Default)]
pub struct TimeTable<T> {
    time: Timeline,
    data: DataSet<T>,
}

impl<T> Into<TimeTable<T>> for TimeSeries<T> {
    fn into(self) -> TimeTable<T> {
        TimeTable {
            time: self.time,
            data: vec![self.data],
        }
    }
}

impl<T: Clone> TimeTable<T> {
    #[allow(dead_code)]
    pub fn from_timeseries(timeseries: TimeSeries<T>) -> Self {
        Self {
            time: timeseries.time,
            data: vec![timeseries.data],
        }
    }
}

impl<T: Clone> TimeTable<T> {
    pub fn new(time: Vec<Time>, data: Vec<T>) -> Self {
        TimeSeries::new(time, data).into()
    }

    #[allow(dead_code)]
    pub fn get_column(&self, column: usize) -> Option<ColumnData<T>> {
        self.data.get(column).map(|val| val.clone())
    }

    pub fn get_at_time(&self, column: usize, time: Time) -> Option<T> {
        if let Some(idx) = self.time.get_index(time) {
            self.data
                .get(column)
                .and_then(|vec| vec.get(idx).clone())
                .map(|el| el.to_owned())
        } else {
            None
        }
    }

    pub fn get_time_range(&self, start: Time, end: Time) -> Option<Vec<Time>> {
        self.time.get_range_raw(start, end)
    }

    /// Returns slice of data that is within the time range specified
    pub fn get_range(&self, column: usize, start: Time, end: Time) -> Option<Vec<T>> {
        if let Some(range) = self.time.get_range(start, end) {
            self.data
                .get(column)
                .map(|vec| vec.get_between(range).to_owned())
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct TimeSeries<T> {
    time: Timeline,
    data: ColumnData<T>,
}

impl<T: Clone> TimeSeries<T> {
    pub fn new(time: impl Into<Vec<Time>>, data: impl Into<Vec<T>>) -> Self {
        let time = Timeline::new(time.into());
        let data = ColumnData::from_vec(data);

        if time.len() != data.len() {
            panic!("Size of time and data are different!");
        }
        Self { time, data }
    }

    pub fn empty() -> Self {
        Self {
            time: Timeline::default(),
            data: ColumnData {
                data: Vec::new(),
                name: None,
            },
        }
    }

    pub fn add(&mut self, time: Time, element: T) {
        self.time.add(time);
        self.data.add(element);
    }

    /// Get data element for a given time
    #[allow(dead_code)]
    pub fn get_at_time(&self, time: Time) -> Option<T> {
        self.time
            .get_index(time)
            .and_then(|idx| self.data.get(idx))
            .map(|val| val.to_owned())
    }

    /// Returns slice of data that is within the time range specified
    #[allow(dead_code)]
    pub fn get_range(&self, start: Time, end: Time) -> Option<&[T]> {
        self.time
            .get_range(start, end)
            .map(|range| self.data.get_between(range))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{cos, sin};
    use crate::Pose;
    use std::f32::consts::PI;

    fn dummy_pose() -> TimeSeries<Pose> {
        let n = 5;
        let dt = 0.01;
        let t: Vec<f32> = (0..n).map(|n| n as f32 * dt).collect();
        let pose: Vec<Pose> = t
            .iter()
            .map(|&t| Pose::new(30.0 * sin(t), 20.0 * cos(t), 2.0 * PI * sin(t)))
            .collect();

        TimeSeries::new(t, pose)
    }
    fn dummy_f32() -> TimeSeries<f32> {
        let n = 5;
        let dt = 1.0;
        let t: Vec<f32> = (0..n).map(|n| n as f32 * dt).collect();
        let data: Vec<f32> = t.iter().map(|&t| t * 3.0).collect();

        TimeSeries::new(t, data)
    }

    #[test]
    fn add_timeseries() {
        let mut ts = TimeSeries::<f32>::empty();
        ts.add(0.5, 5.0);
        ts.add(0.8, 15.0);
        dbg!(&ts);
    }

    #[test]
    fn check_index() {
        // dbg!(&ts);
        let ts = dummy_pose();

        assert_eq!(2, ts.time.get_index(0.02).unwrap()); // finding exactly matching time
        assert_eq!(2, ts.time.get_index(0.02).unwrap()); // running again should give same result
        assert_eq!(2, ts.time.get_index(0.015).unwrap()); // finding next closest time stamp
    }

    #[test]
    fn check_range() {
        let ts = dummy_f32();
        assert_eq!(1, ts.time.get_index(1.0).unwrap());
        assert_eq!(3, ts.time.get_index(2.1).unwrap());
        assert_eq!(3, ts.time.get_index(2.9).unwrap());
        assert_eq!(3, ts.time.get_index(3.0).unwrap());
        assert_eq!(&[3.0, 6.0], ts.get_range(1.0, 2.9).unwrap());
        assert_eq!(&[3.0, 6.0, 9.0], ts.get_range(1.0, 3.0).unwrap());
    }

    #[test]
    fn series_to_table() {
        let ts = dummy_f32();
        let _table: TimeTable<f32> = ts.into();
    }
}

// TimeTable data format options:
// 1. Generational-arena -> Arena<TimeSeries>
//  pros: easy to push and manage TimeSeries
//  cons: Dependency, TimeSeries cannot contain different data types
// 2. Vec<Box<dyn TimeSeries>>
//  pros: No dependency
//  cons: Use of trait object
// 3. ndarray?
