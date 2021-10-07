use core::cell::UnsafeCell;
use core::hint;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock<T: ?Sized> {
	token: AtomicBool,
	value: UnsafeCell<T>,
}

unsafe impl<T: ?Sized> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
	pub const fn new(v: T) -> Self {
		SpinLock {
			token: AtomicBool::new(true),
			value: UnsafeCell::new(v),
		}
	}

	// false means locked
	// true means unlocked
	pub fn lock(&'_ self) -> SpinLockGuard<'_, T> {
		while self.token.load(Ordering::SeqCst) != true {
			hint::spin_loop();
		}
		self.token.store(false, Ordering::SeqCst);
		unsafe {
			SpinLockGuard {
				token: &self.token,
				value: &mut *self.value.get(),
			}
		}
	}
}

pub struct SpinLockGuard<'a, T: 'a + ?Sized> {
	token: &'a AtomicBool,
	value: &'a mut T,
}

impl<'a, T: ?Sized> Drop for SpinLockGuard<'a, T> {
	fn drop(&mut self) {
		self.token.store(true, Ordering::SeqCst);
	}
}

impl<'a, T: ?Sized> Deref for SpinLockGuard<'a, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.value
	}
}

impl<'a, T: ?Sized> DerefMut for SpinLockGuard<'a, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.value
	}
}
