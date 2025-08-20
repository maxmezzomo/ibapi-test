pub trait Guard: Send + 'static {
    fn to_guard(&self) -> Box<dyn Guard>
    where
        Self: Sized + Clone,
    {
        Box::new(self.clone())
    }
}

// pub trait GuardedResultExt {
//     fn into_guarded_result<T>(self, result: T) -> GuardedResult<T>;
// }

// impl GuardedResultExt for SubscriptionGuard {
//     fn into_guarded_result<T>(self, result: T) -> GuardedResult<T> {
//         (result, self)
//     }

// }

impl<T: Send + 'static> Guard for ibapi::subscriptions::Subscription<T> {}

pub struct SubscriptionGuard(Box<dyn Guard>);

impl SubscriptionGuard {
    pub fn to_guarded_result<T>(self, result: T) -> GuardedResult<T> {
        GuardedResult(result, self)
    }
}

impl<T: Send + 'static> From<&ibapi::subscriptions::Subscription<T>> for SubscriptionGuard {
    fn from(value: &ibapi::subscriptions::Subscription<T>) -> Self {
        SubscriptionGuard(value.to_guard())
    }
}

pub struct GuardedResult<T>(T, SubscriptionGuard);

impl<T> GuardedResult<T> {
    pub fn to_result(self) -> T {
        self.0
    }
}
