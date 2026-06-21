use crate::entity::EntityPosition;

pub struct PathRequest {
    destination: Option<EntityPosition>,
    minimum_distance: Option<f64>,
    maximum_distance: Option<f64>,
    variance: Option<f64>,
    on_complete: Option<Box<dyn FnOnce() + Send>>,
}

impl PathRequest {
    pub const fn reset() -> Self {
        Self {
            destination: None,
            minimum_distance: None,
            maximum_distance: None,
            variance: None,
            on_complete: None,
        }
    }

    pub fn with_minimum_distance(mut self, minimum_distance: f64) -> Self {
        self.minimum_distance = Some(minimum_distance);
        self
    }

    pub fn with_maximum_distance(mut self, maximum_distance: f64) -> Self {
        self.maximum_distance = Some(maximum_distance);
        self
    }

    pub fn with_variance(mut self, variance: f64) -> Self {
        self.variance = Some(variance);
        self
    }

    pub fn on_complete(mut self, on_complete: impl FnOnce() + Send + 'static) -> Self {
        self.on_complete = Some(Box::new(on_complete));
        self
    }

    pub(crate) const fn get_destination(&self) -> Option<EntityPosition> {
        self.destination
    }

    pub(crate) const fn get_minimum_distance(&self) -> Option<f64> {
        self.minimum_distance
    }

    pub(crate) const fn get_maximum_distance(&self) -> Option<f64> {
        self.maximum_distance
    }

    pub(crate) const fn get_variance(&self) -> Option<f64> {
        self.variance
    }

    pub(crate) fn take_on_complete(&mut self) -> Option<Box<dyn FnOnce() + Send>> {
        self.on_complete.take()
    }
}

impl From<EntityPosition> for PathRequest {
    fn from(destination: EntityPosition) -> Self {
        Self {
            destination: Some(destination),
            minimum_distance: None,
            maximum_distance: None,
            variance: None,
            on_complete: None,
        }
    }
}
