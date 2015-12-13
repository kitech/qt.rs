// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN13QElapsedTimer5startEv() -> i32;
  fn _ZNK13QElapsedTimer12nsecsElapsedEv() -> i32;
  fn _ZN13QElapsedTimer10invalidateEv() -> i32;
  fn _ZN13QElapsedTimer11isMonotonicEv() -> i32;
  fn _ZN13QElapsedTimerC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK13QElapsedTimer7msecsToERKS_(arg0: *const c_void) -> i32;
  fn _ZNK13QElapsedTimer19msecsSinceReferenceEv() -> i32;
  fn _ZNK13QElapsedTimer10hasExpiredEx(arg0: c_longlong) -> i32;
  fn _ZN13QElapsedTimer7restartEv() -> i32;
  fn _ZNK13QElapsedTimer7isValidEv() -> i32;
  fn _ZNK13QElapsedTimer6secsToERKS_(arg0: *const c_void) -> i32;
  fn _ZNK13QElapsedTimer7elapsedEv() -> i32;
}

// body block begin
// class sizeof(QElapsedTimer)=16
pub struct QElapsedTimer {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QElapsedTimer {
  pub fn start<T: QElapsedTimer_start>(&mut self, value: T) -> i32 {
    value.start(self);
    return 1;
  }
}

pub trait QElapsedTimer_start {
  fn start(self, this: &mut QElapsedTimer) -> i32;
}

// proto: void QElapsedTimer::start();
impl<'a> /*trait*/ QElapsedTimer_start for () {
  fn start(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer5startEv()};
    unsafe {_ZN13QElapsedTimer5startEv()};
    return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn nsecsElapsed<T: QElapsedTimer_nsecsElapsed>(&mut self, value: T) -> i32 {
    value.nsecsElapsed(self);
    return 1;
  }
}

pub trait QElapsedTimer_nsecsElapsed {
  fn nsecsElapsed(self, this: &mut QElapsedTimer) -> i32;
}

// proto: long long QElapsedTimer::nsecsElapsed();
impl<'a> /*trait*/ QElapsedTimer_nsecsElapsed for () {
  fn nsecsElapsed(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer12nsecsElapsedEv()};
    unsafe {_ZNK13QElapsedTimer12nsecsElapsedEv()};
    return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn invalidate<T: QElapsedTimer_invalidate>(&mut self, value: T) -> i32 {
    value.invalidate(self);
    return 1;
  }
}

pub trait QElapsedTimer_invalidate {
  fn invalidate(self, this: &mut QElapsedTimer) -> i32;
}

// proto: void QElapsedTimer::invalidate();
impl<'a> /*trait*/ QElapsedTimer_invalidate for () {
  fn invalidate(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer10invalidateEv()};
    unsafe {_ZN13QElapsedTimer10invalidateEv()};
    return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn isMonotonic<T: QElapsedTimer_isMonotonic>(&mut self, value: T) -> i32 {
    value.isMonotonic(self);
    return 1;
  }
}

pub trait QElapsedTimer_isMonotonic {
  fn isMonotonic(self, this: &mut QElapsedTimer) -> i32;
}

// proto: bool QElapsedTimer::isMonotonic();
impl<'a> /*trait*/ QElapsedTimer_isMonotonic for () {
  fn isMonotonic(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer11isMonotonicEv()};
    unsafe {_ZN13QElapsedTimer11isMonotonicEv()};
    return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn NewQElapsedTimer<T: QElapsedTimer_NewQElapsedTimer>(value: T) -> QElapsedTimer {
    let rsthis = value.NewQElapsedTimer();
    return rsthis;
    // return 1;
  }
}

pub trait QElapsedTimer_NewQElapsedTimer {
  fn NewQElapsedTimer(self) -> QElapsedTimer;
}

// proto: void QElapsedTimer::NewQElapsedTimer();
impl<'a> /*trait*/ QElapsedTimer_NewQElapsedTimer for () {
  fn NewQElapsedTimer(self) -> QElapsedTimer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimerC1Ev()};
    unsafe {_ZN13QElapsedTimerC1Ev(qthis)};
    let rsthis = QElapsedTimer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn msecsTo<T: QElapsedTimer_msecsTo>(&mut self, value: T) -> i32 {
    value.msecsTo(self);
    return 1;
  }
}

pub trait QElapsedTimer_msecsTo {
  fn msecsTo(self, this: &mut QElapsedTimer) -> i32;
}

// proto: long long QElapsedTimer::msecsTo(const QElapsedTimer & other);
impl<'a> /*trait*/ QElapsedTimer_msecsTo for (&'a  QElapsedTimer) {
  fn msecsTo(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer7msecsToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QElapsedTimer7msecsToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn msecsSinceReference<T: QElapsedTimer_msecsSinceReference>(&mut self, value: T) -> i32 {
    value.msecsSinceReference(self);
    return 1;
  }
}

pub trait QElapsedTimer_msecsSinceReference {
  fn msecsSinceReference(self, this: &mut QElapsedTimer) -> i32;
}

// proto: long long QElapsedTimer::msecsSinceReference();
impl<'a> /*trait*/ QElapsedTimer_msecsSinceReference for () {
  fn msecsSinceReference(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer19msecsSinceReferenceEv()};
    unsafe {_ZNK13QElapsedTimer19msecsSinceReferenceEv()};
    return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn hasExpired<T: QElapsedTimer_hasExpired>(&mut self, value: T) -> i32 {
    value.hasExpired(self);
    return 1;
  }
}

pub trait QElapsedTimer_hasExpired {
  fn hasExpired(self, this: &mut QElapsedTimer) -> i32;
}

// proto: bool QElapsedTimer::hasExpired(qint64 timeout);
impl<'a> /*trait*/ QElapsedTimer_hasExpired for (i64) {
  fn hasExpired(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer10hasExpiredEx()};
    let arg0 = self  as c_longlong;
    unsafe {_ZNK13QElapsedTimer10hasExpiredEx(arg0)};
    return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn restart<T: QElapsedTimer_restart>(&mut self, value: T) -> i32 {
    value.restart(self);
    return 1;
  }
}

pub trait QElapsedTimer_restart {
  fn restart(self, this: &mut QElapsedTimer) -> i32;
}

// proto: long long QElapsedTimer::restart();
impl<'a> /*trait*/ QElapsedTimer_restart for () {
  fn restart(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer7restartEv()};
    unsafe {_ZN13QElapsedTimer7restartEv()};
    return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn isValid<T: QElapsedTimer_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QElapsedTimer_isValid {
  fn isValid(self, this: &mut QElapsedTimer) -> i32;
}

// proto: bool QElapsedTimer::isValid();
impl<'a> /*trait*/ QElapsedTimer_isValid for () {
  fn isValid(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer7isValidEv()};
    unsafe {_ZNK13QElapsedTimer7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn secsTo<T: QElapsedTimer_secsTo>(&mut self, value: T) -> i32 {
    value.secsTo(self);
    return 1;
  }
}

pub trait QElapsedTimer_secsTo {
  fn secsTo(self, this: &mut QElapsedTimer) -> i32;
}

// proto: long long QElapsedTimer::secsTo(const QElapsedTimer & other);
impl<'a> /*trait*/ QElapsedTimer_secsTo for (&'a  QElapsedTimer) {
  fn secsTo(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer6secsToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK13QElapsedTimer6secsToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn elapsed<T: QElapsedTimer_elapsed>(&mut self, value: T) -> i32 {
    value.elapsed(self);
    return 1;
  }
}

pub trait QElapsedTimer_elapsed {
  fn elapsed(self, this: &mut QElapsedTimer) -> i32;
}

// proto: long long QElapsedTimer::elapsed();
impl<'a> /*trait*/ QElapsedTimer_elapsed for () {
  fn elapsed(self, this: &mut QElapsedTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer7elapsedEv()};
    unsafe {_ZNK13QElapsedTimer7elapsedEv()};
    return 1;
  }
}

