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
  // proto:  void QElapsedTimer::start();
  fn _ZN13QElapsedTimer5startEv(qthis: *mut c_void) ;
  // proto:  long long QElapsedTimer::nsecsElapsed();
  fn _ZNK13QElapsedTimer12nsecsElapsedEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QElapsedTimer::invalidate();
  fn _ZN13QElapsedTimer10invalidateEv(qthis: *mut c_void) ;
  // proto: static bool QElapsedTimer::isMonotonic();
  fn _ZN13QElapsedTimer11isMonotonicEv() -> int8_t;
  // proto:  void QElapsedTimer::NewQElapsedTimer();
  fn _ZN13QElapsedTimerC1Ev(qthis: *mut c_void) ;
  // proto:  long long QElapsedTimer::msecsTo(const QElapsedTimer & other);
  fn _ZNK13QElapsedTimer7msecsToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_longlong;
  // proto:  long long QElapsedTimer::msecsSinceReference();
  fn _ZNK13QElapsedTimer19msecsSinceReferenceEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QElapsedTimer::hasExpired(qint64 timeout);
  fn _ZNK13QElapsedTimer10hasExpiredEx(qthis: *mut c_void, arg0: c_longlong) -> int8_t;
  // proto:  long long QElapsedTimer::restart();
  fn _ZN13QElapsedTimer7restartEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QElapsedTimer::isValid();
  fn _ZNK13QElapsedTimer7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  long long QElapsedTimer::secsTo(const QElapsedTimer & other);
  fn _ZNK13QElapsedTimer6secsToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_longlong;
  // proto:  long long QElapsedTimer::elapsed();
  fn _ZNK13QElapsedTimer7elapsedEv(qthis: *mut c_void) -> c_longlong;
}

// body block begin
// class sizeof(QElapsedTimer)=16
pub struct QElapsedTimer {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QElapsedTimer {
  pub fn start<RetType, T: QElapsedTimer_start<RetType>>(&mut self, value: T) -> RetType {
    return value.start(self);
    // return 1;
  }
}

pub trait QElapsedTimer_start<RetType> {
  fn start(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto:  void QElapsedTimer::start();
impl<'a> /*trait*/ QElapsedTimer_start<()> for () {
  fn start(self, rsthis: &mut QElapsedTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer5startEv()};
     unsafe {_ZN13QElapsedTimer5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn nsecsElapsed<RetType, T: QElapsedTimer_nsecsElapsed<RetType>>(&mut self, value: T) -> RetType {
    return value.nsecsElapsed(self);
    // return 1;
  }
}

pub trait QElapsedTimer_nsecsElapsed<RetType> {
  fn nsecsElapsed(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto:  long long QElapsedTimer::nsecsElapsed();
impl<'a> /*trait*/ QElapsedTimer_nsecsElapsed<i64> for () {
  fn nsecsElapsed(self, rsthis: &mut QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer12nsecsElapsedEv()};
    let mut ret = unsafe {_ZNK13QElapsedTimer12nsecsElapsedEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn invalidate<RetType, T: QElapsedTimer_invalidate<RetType>>(&mut self, value: T) -> RetType {
    return value.invalidate(self);
    // return 1;
  }
}

pub trait QElapsedTimer_invalidate<RetType> {
  fn invalidate(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto:  void QElapsedTimer::invalidate();
impl<'a> /*trait*/ QElapsedTimer_invalidate<()> for () {
  fn invalidate(self, rsthis: &mut QElapsedTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer10invalidateEv()};
     unsafe {_ZN13QElapsedTimer10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn isMonotonic<RetType, T: QElapsedTimer_isMonotonic<RetType>>(&mut self, value: T) -> RetType {
    return value.isMonotonic(self);
    // return 1;
  }
}

pub trait QElapsedTimer_isMonotonic<RetType> {
  fn isMonotonic(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto: static bool QElapsedTimer::isMonotonic();
impl<'a> /*trait*/ QElapsedTimer_isMonotonic<i8> for () {
  fn isMonotonic(self, rsthis: &mut QElapsedTimer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer11isMonotonicEv()};
    let mut ret = unsafe {_ZN13QElapsedTimer11isMonotonicEv()};
    return ret as i8;
    // return 1;
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
  pub fn msecsTo<RetType, T: QElapsedTimer_msecsTo<RetType>>(&mut self, value: T) -> RetType {
    return value.msecsTo(self);
    // return 1;
  }
}

pub trait QElapsedTimer_msecsTo<RetType> {
  fn msecsTo(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto:  long long QElapsedTimer::msecsTo(const QElapsedTimer & other);
impl<'a> /*trait*/ QElapsedTimer_msecsTo<i64> for (&'a  QElapsedTimer) {
  fn msecsTo(self, rsthis: &mut QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer7msecsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QElapsedTimer7msecsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn msecsSinceReference<RetType, T: QElapsedTimer_msecsSinceReference<RetType>>(&mut self, value: T) -> RetType {
    return value.msecsSinceReference(self);
    // return 1;
  }
}

pub trait QElapsedTimer_msecsSinceReference<RetType> {
  fn msecsSinceReference(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto:  long long QElapsedTimer::msecsSinceReference();
impl<'a> /*trait*/ QElapsedTimer_msecsSinceReference<i64> for () {
  fn msecsSinceReference(self, rsthis: &mut QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer19msecsSinceReferenceEv()};
    let mut ret = unsafe {_ZNK13QElapsedTimer19msecsSinceReferenceEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn hasExpired<RetType, T: QElapsedTimer_hasExpired<RetType>>(&mut self, value: T) -> RetType {
    return value.hasExpired(self);
    // return 1;
  }
}

pub trait QElapsedTimer_hasExpired<RetType> {
  fn hasExpired(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto:  bool QElapsedTimer::hasExpired(qint64 timeout);
impl<'a> /*trait*/ QElapsedTimer_hasExpired<i8> for (i64) {
  fn hasExpired(self, rsthis: &mut QElapsedTimer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer10hasExpiredEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZNK13QElapsedTimer10hasExpiredEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn restart<RetType, T: QElapsedTimer_restart<RetType>>(&mut self, value: T) -> RetType {
    return value.restart(self);
    // return 1;
  }
}

pub trait QElapsedTimer_restart<RetType> {
  fn restart(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto:  long long QElapsedTimer::restart();
impl<'a> /*trait*/ QElapsedTimer_restart<i64> for () {
  fn restart(self, rsthis: &mut QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer7restartEv()};
    let mut ret = unsafe {_ZN13QElapsedTimer7restartEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn isValid<RetType, T: QElapsedTimer_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QElapsedTimer_isValid<RetType> {
  fn isValid(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto:  bool QElapsedTimer::isValid();
impl<'a> /*trait*/ QElapsedTimer_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QElapsedTimer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer7isValidEv()};
    let mut ret = unsafe {_ZNK13QElapsedTimer7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn secsTo<RetType, T: QElapsedTimer_secsTo<RetType>>(&mut self, value: T) -> RetType {
    return value.secsTo(self);
    // return 1;
  }
}

pub trait QElapsedTimer_secsTo<RetType> {
  fn secsTo(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto:  long long QElapsedTimer::secsTo(const QElapsedTimer & other);
impl<'a> /*trait*/ QElapsedTimer_secsTo<i64> for (&'a  QElapsedTimer) {
  fn secsTo(self, rsthis: &mut QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer6secsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QElapsedTimer6secsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QElapsedTimer {
  pub fn elapsed<RetType, T: QElapsedTimer_elapsed<RetType>>(&mut self, value: T) -> RetType {
    return value.elapsed(self);
    // return 1;
  }
}

pub trait QElapsedTimer_elapsed<RetType> {
  fn elapsed(self, rsthis: &mut QElapsedTimer) -> RetType;
}

// proto:  long long QElapsedTimer::elapsed();
impl<'a> /*trait*/ QElapsedTimer_elapsed<i64> for () {
  fn elapsed(self, rsthis: &mut QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer7elapsedEv()};
    let mut ret = unsafe {_ZNK13QElapsedTimer7elapsedEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

