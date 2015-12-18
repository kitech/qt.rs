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
  pub fn isAtBoundary<RetType, T: QTextBoundaryFinder_isAtBoundary<RetType>>(&mut self, value: T) -> RetType {
    return value.isAtBoundary(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_isAtBoundary<RetType> {
  fn isAtBoundary(self, rsthis: &mut QTextBoundaryFinder) -> RetType;
}

// proto:  bool QTextBoundaryFinder::isAtBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_isAtBoundary<i8> for () {
  fn isAtBoundary(self, rsthis: &mut QTextBoundaryFinder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder12isAtBoundaryEv()};
    let mut ret = unsafe {_ZNK19QTextBoundaryFinder12isAtBoundaryEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn toNextBoundary<RetType, T: QTextBoundaryFinder_toNextBoundary<RetType>>(&mut self, value: T) -> RetType {
    return value.toNextBoundary(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toNextBoundary<RetType> {
  fn toNextBoundary(self, rsthis: &mut QTextBoundaryFinder) -> RetType;
}

// proto:  int QTextBoundaryFinder::toNextBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_toNextBoundary<i32> for () {
  fn toNextBoundary(self, rsthis: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder14toNextBoundaryEv()};
    let mut ret = unsafe {_ZN19QTextBoundaryFinder14toNextBoundaryEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn toEnd<RetType, T: QTextBoundaryFinder_toEnd<RetType>>(&mut self, value: T) -> RetType {
    return value.toEnd(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toEnd<RetType> {
  fn toEnd(self, rsthis: &mut QTextBoundaryFinder) -> RetType;
}

// proto:  void QTextBoundaryFinder::toEnd();
impl<'a> /*trait*/ QTextBoundaryFinder_toEnd<()> for () {
  fn toEnd(self, rsthis: &mut QTextBoundaryFinder) -> () {
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
  pub fn setPosition<RetType, T: QTextBoundaryFinder_setPosition<RetType>>(&mut self, value: T) -> RetType {
    return value.setPosition(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_setPosition<RetType> {
  fn setPosition(self, rsthis: &mut QTextBoundaryFinder) -> RetType;
}

// proto:  void QTextBoundaryFinder::setPosition(int position);
impl<'a> /*trait*/ QTextBoundaryFinder_setPosition<()> for (i32) {
  fn setPosition(self, rsthis: &mut QTextBoundaryFinder) -> () {
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
  pub fn toPreviousBoundary<RetType, T: QTextBoundaryFinder_toPreviousBoundary<RetType>>(&mut self, value: T) -> RetType {
    return value.toPreviousBoundary(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toPreviousBoundary<RetType> {
  fn toPreviousBoundary(self, rsthis: &mut QTextBoundaryFinder) -> RetType;
}

// proto:  int QTextBoundaryFinder::toPreviousBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_toPreviousBoundary<i32> for () {
  fn toPreviousBoundary(self, rsthis: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder18toPreviousBoundaryEv()};
    let mut ret = unsafe {_ZN19QTextBoundaryFinder18toPreviousBoundaryEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn isValid<RetType, T: QTextBoundaryFinder_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_isValid<RetType> {
  fn isValid(self, rsthis: &mut QTextBoundaryFinder) -> RetType;
}

// proto:  bool QTextBoundaryFinder::isValid();
impl<'a> /*trait*/ QTextBoundaryFinder_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QTextBoundaryFinder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder7isValidEv()};
    let mut ret = unsafe {_ZNK19QTextBoundaryFinder7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn FreeQTextBoundaryFinder<RetType, T: QTextBoundaryFinder_FreeQTextBoundaryFinder<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQTextBoundaryFinder(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_FreeQTextBoundaryFinder<RetType> {
  fn FreeQTextBoundaryFinder(self, rsthis: &mut QTextBoundaryFinder) -> RetType;
}

// proto:  void QTextBoundaryFinder::FreeQTextBoundaryFinder();
impl<'a> /*trait*/ QTextBoundaryFinder_FreeQTextBoundaryFinder<()> for () {
  fn FreeQTextBoundaryFinder(self, rsthis: &mut QTextBoundaryFinder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinderD0Ev()};
     unsafe {_ZN19QTextBoundaryFinderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn string<RetType, T: QTextBoundaryFinder_string<RetType>>(&mut self, value: T) -> RetType {
    return value.string(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_string<RetType> {
  fn string(self, rsthis: &mut QTextBoundaryFinder) -> RetType;
}

// proto:  QString QTextBoundaryFinder::string();
impl<'a> /*trait*/ QTextBoundaryFinder_string<QString> for () {
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
  pub fn toStart<RetType, T: QTextBoundaryFinder_toStart<RetType>>(&mut self, value: T) -> RetType {
    return value.toStart(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_toStart<RetType> {
  fn toStart(self, rsthis: &mut QTextBoundaryFinder) -> RetType;
}

// proto:  void QTextBoundaryFinder::toStart();
impl<'a> /*trait*/ QTextBoundaryFinder_toStart<()> for () {
  fn toStart(self, rsthis: &mut QTextBoundaryFinder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder7toStartEv()};
     unsafe {_ZN19QTextBoundaryFinder7toStartEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn position<RetType, T: QTextBoundaryFinder_position<RetType>>(&mut self, value: T) -> RetType {
    return value.position(self);
    // return 1;
  }
}

pub trait QTextBoundaryFinder_position<RetType> {
  fn position(self, rsthis: &mut QTextBoundaryFinder) -> RetType;
}

// proto:  int QTextBoundaryFinder::position();
impl<'a> /*trait*/ QTextBoundaryFinder_position<i32> for () {
  fn position(self, rsthis: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder8positionEv()};
    let mut ret = unsafe {_ZNK19QTextBoundaryFinder8positionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

