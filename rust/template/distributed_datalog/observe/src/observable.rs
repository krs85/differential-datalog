use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;

use crate::observer::ObserverBox;
use crate::subscription::Subscription;
use crate::subscription::UpdatesSubscription;

/// A trait for objects that can be observed.
pub trait Observable<T, E>: Debug
where
    T: Send,
    E: Send,
{
    /// An Observer subscribes to an Observable to listen to data
    /// emitted by the latter. The Observer stops listening when the
    /// Subscription returned is canceled.
    ///
    /// A return value of `None` indicates that the `Observable` is
    /// unable to accept any more `Observer`s.
    fn subscribe(&mut self, observer: ObserverBox<T, E>) -> Option<Box<dyn Subscription>>;
}

/// A very simple observable that supports subscription of a single
/// observer.
#[derive(Debug)]
pub struct UpdatesObservable<T, E> {
    /// A reference to the `Observer` we subscribed to.
    pub observer: Arc<Mutex<Option<ObserverBox<T, E>>>>,
}

impl<T, E> Observable<T, E> for UpdatesObservable<T, E>
where
    T: Debug + Send + 'static,
    E: Debug + Send + 'static,
{
    /// An observer subscribes to the delta stream from an outlet.
    fn subscribe(&mut self, observer: ObserverBox<T, E>) -> Option<Box<dyn Subscription>> {
        let mut guard = self.observer.lock().unwrap();
        match *guard {
            Some(_) => None,
            None => {
                *guard = Some(observer);
                Some(Box::new(UpdatesSubscription {
                    observer: self.observer.clone(),
                }))
            }
        }
    }
}