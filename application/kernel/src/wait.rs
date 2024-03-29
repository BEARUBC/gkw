use std::ops::Deref;
use std::sync::Arc;
use std::sync::Condvar;
use std::sync::Mutex;

use anyhow::anyhow;

#[derive(Default)]
pub struct Wait<T>(Arc<(Mutex<T>, Condvar)>);

impl<T> Wait<T>
where
    T: PartialEq,
{
    pub fn wait(self, value: T) -> anyhow::Result<()> {
        let (lock, cond) = &*self;
        let mut resume = lock.lock().map_err(|_| anyhow!(""))?;
        while *resume != value {
            resume = cond.wait(resume).map_err(|_| anyhow!(""))?;
        }
        Ok(())
    }
}

#[cfg(not(feature = "simulation"))]
impl<T> Wait<T>
where
    T: 'static + Send,
{
    pub fn new(value: T) -> Self {
        let lock = Mutex::new(value);
        let cond = Condvar::default();
        let pair = Arc::new((lock, cond));
        Self(pair)
    }
}

impl<T> Wait<T> {
    pub fn set(&mut self, value: T) -> anyhow::Result<()> {
        let (lock, cond) = &**self;
        let mut resume = lock.lock().map_err(|_| anyhow!(""))?;
        *resume = value;
        cond.notify_one();
        Ok(())
    }
}

impl<T> Clone for Wait<T> {
    fn clone(&self) -> Self {
        let pair = self.0.clone();
        Self(pair)
    }
}

impl<T> Deref for Wait<T> {
    type Target = (Mutex<T>, Condvar);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
