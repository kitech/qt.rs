// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QTextBoundaryFinder::isAtBoundary();
  fn _ZNK19QTextBoundaryFinder12isAtBoundaryEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTextBoundaryFinder::toNextBoundary();
  fn _ZN19QTextBoundaryFinder14toNextBoundaryEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBoundaryFinder::toEnd();
  fn _ZN19QTextBoundaryFinder5toEndEv(qthis: *mut c_void) ;
  // proto:  void QTextBoundaryFinder::NewQTextBoundaryFinder(const QTextBoundaryFinder & other);
  fn _ZN19QTextBoundaryFinderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTextBoundaryFinder::setPosition(int position);
  fn _ZN19QTextBoundaryFinder11setPositionEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTextBoundaryFinder::NewQTextBoundaryFinder();
  fn _ZN19QTextBoundaryFinderC1Ev(qthis: *mut c_void) ;
  // proto:  int QTextBoundaryFinder::toPreviousBoundary();
  fn _ZN19QTextBoundaryFinder18toPreviousBoundaryEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextBoundaryFinder::isValid();
  fn _ZNK19QTextBoundaryFinder7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTextBoundaryFinder::FreeQTextBoundaryFinder();
  fn _ZN19QTextBoundaryFinderD0Ev(qthis: *mut c_void) ;
  // proto:  QString QTextBoundaryFinder::string();
  fn _ZNK19QTextBoundaryFinder6stringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextBoundaryFinder::toStart();
  fn _ZN19QTextBoundaryFinder7toStartEv(qthis: *mut c_void) ;
  // proto:  int QTextBoundaryFinder::position();
  fn _ZNK19QTextBoundaryFinder8positionEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QTextBoundaryFinder)=48
pub struct QTextBoundaryFinder {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn isAtBoundary<T: QTextBoundaryFinder_isAtBoundary>(&mut self, value: T) -> i8 {
    return value.isAtBoundary(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_isAtBoundary {
  fn isAtBoundary(self, rsthis: &mut QTextBoundaryFinder) -> i8;
}

// proto:  bool QTextBoundaryFinder::isAtBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_isAtBoundary for () {
  fn isAtBoundary(self, rsthis: &mut QTextBoundaryFinder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder12isAtBoundaryEv()};
    let mut ret = unsafe {_ZNK19QTextBoundaryFinder12isAtBoundaryEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn toNextBoundary<T: QTextBoundaryFinder_toNextBoundary>(&mut self, value: T) -> i32 {
    return value.toNextBoundary(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toNextBoundary {
  fn toNextBoundary(self, rsthis: &mut QTextBoundaryFinder) -> i32;
}

// proto:  int QTextBoundaryFinder::toNextBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_toNextBoundary for () {
  fn toNextBoundary(self, rsthis: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder14toNextBoundaryEv()};
    let mut ret = unsafe {_ZN19QTextBoundaryFinder14toNextBoundaryEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn toEnd<T: QTextBoundaryFinder_toEnd>(&mut self, value: T)  {
     value.toEnd(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toEnd {
  fn toEnd(self, rsthis: &mut QTextBoundaryFinder) ;
}

// proto:  void QTextBoundaryFinder::toEnd();
impl<'a> /*trait*/ QTextBoundaryFinder_toEnd for () {
  fn toEnd(self, rsthis: &mut QTextBoundaryFinder)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder5toEndEv()};
     unsafe {_ZN19QTextBoundaryFinder5toEndEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn NewQTextBoundaryFinder<T: QTextBoundaryFinder_NewQTextBoundaryFinder>(value: T) -> QTextBoundaryFinder {
    let rsthis = value.NewQTextBoundaryFinder();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBoundaryFinder_NewQTextBoundaryFinder {
  fn NewQTextBoundaryFinder(self) -> QTextBoundaryFinder;
}

// proto: void QTextBoundaryFinder::NewQTextBoundaryFinder(const QTextBoundaryFinder & other);
impl<'a> /*trait*/ QTextBoundaryFinder_NewQTextBoundaryFinder for (&'a  QTextBoundaryFinder) {
  fn NewQTextBoundaryFinder(self) -> QTextBoundaryFinder {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QTextBoundaryFinderC1ERKS_(qthis, arg0)};
    let rsthis = QTextBoundaryFinder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn setPosition<T: QTextBoundaryFinder_setPosition>(&mut self, value: T)  {
     value.setPosition(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_setPosition {
  fn setPosition(self, rsthis: &mut QTextBoundaryFinder) ;
}

// proto:  void QTextBoundaryFinder::setPosition(int position);
impl<'a> /*trait*/ QTextBoundaryFinder_setPosition for (i32) {
  fn setPosition(self, rsthis: &mut QTextBoundaryFinder)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder11setPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN19QTextBoundaryFinder11setPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QTextBoundaryFinder::NewQTextBoundaryFinder();
impl<'a> /*trait*/ QTextBoundaryFinder_NewQTextBoundaryFinder for () {
  fn NewQTextBoundaryFinder(self) -> QTextBoundaryFinder {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinderC1Ev()};
    unsafe {_ZN19QTextBoundaryFinderC1Ev(qthis)};
    let rsthis = QTextBoundaryFinder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn toPreviousBoundary<T: QTextBoundaryFinder_toPreviousBoundary>(&mut self, value: T) -> i32 {
    return value.toPreviousBoundary(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toPreviousBoundary {
  fn toPreviousBoundary(self, rsthis: &mut QTextBoundaryFinder) -> i32;
}

// proto:  int QTextBoundaryFinder::toPreviousBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_toPreviousBoundary for () {
  fn toPreviousBoundary(self, rsthis: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder18toPreviousBoundaryEv()};
    let mut ret = unsafe {_ZN19QTextBoundaryFinder18toPreviousBoundaryEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn isValid<T: QTextBoundaryFinder_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_isValid {
  fn isValid(self, rsthis: &mut QTextBoundaryFinder) -> i8;
}

// proto:  bool QTextBoundaryFinder::isValid();
impl<'a> /*trait*/ QTextBoundaryFinder_isValid for () {
  fn isValid(self, rsthis: &mut QTextBoundaryFinder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder7isValidEv()};
    let mut ret = unsafe {_ZNK19QTextBoundaryFinder7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn FreeQTextBoundaryFinder<T: QTextBoundaryFinder_FreeQTextBoundaryFinder>(&mut self, value: T)  {
     value.FreeQTextBoundaryFinder(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_FreeQTextBoundaryFinder {
  fn FreeQTextBoundaryFinder(self, rsthis: &mut QTextBoundaryFinder) ;
}

// proto:  void QTextBoundaryFinder::FreeQTextBoundaryFinder();
impl<'a> /*trait*/ QTextBoundaryFinder_FreeQTextBoundaryFinder for () {
  fn FreeQTextBoundaryFinder(self, rsthis: &mut QTextBoundaryFinder)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinderD0Ev()};
     unsafe {_ZN19QTextBoundaryFinderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn string<T: QTextBoundaryFinder_string>(&mut self, value: T) -> QString {
    return value.string(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_string {
  fn string(self, rsthis: &mut QTextBoundaryFinder) -> QString;
}

// proto:  QString QTextBoundaryFinder::string();
impl<'a> /*trait*/ QTextBoundaryFinder_string for () {
  fn string(self, rsthis: &mut QTextBoundaryFinder) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder6stringEv()};
    let mut ret = unsafe {_ZNK19QTextBoundaryFinder6stringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn toStart<T: QTextBoundaryFinder_toStart>(&mut self, value: T)  {
     value.toStart(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toStart {
  fn toStart(self, rsthis: &mut QTextBoundaryFinder) ;
}

// proto:  void QTextBoundaryFinder::toStart();
impl<'a> /*trait*/ QTextBoundaryFinder_toStart for () {
  fn toStart(self, rsthis: &mut QTextBoundaryFinder)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder7toStartEv()};
     unsafe {_ZN19QTextBoundaryFinder7toStartEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn position<T: QTextBoundaryFinder_position>(&mut self, value: T) -> i32 {
    return value.position(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_position {
  fn position(self, rsthis: &mut QTextBoundaryFinder) -> i32;
}

// proto:  int QTextBoundaryFinder::position();
impl<'a> /*trait*/ QTextBoundaryFinder_position for () {
  fn position(self, rsthis: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder8positionEv()};
    let mut ret = unsafe {_ZNK19QTextBoundaryFinder8positionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

