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
  fn _ZNK19QTextBoundaryFinder12isAtBoundaryEv() -> i32;
  fn _ZN19QTextBoundaryFinder14toNextBoundaryEv() -> i32;
  fn _ZN19QTextBoundaryFinder5toEndEv() -> i32;
  fn _ZN19QTextBoundaryFinderC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN19QTextBoundaryFinder11setPositionEi(arg0: c_int) -> i32;
  fn _ZN19QTextBoundaryFinderC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN19QTextBoundaryFinder18toPreviousBoundaryEv() -> i32;
  fn _ZNK19QTextBoundaryFinder7isValidEv() -> i32;
  fn _ZN19QTextBoundaryFinderD0Ev() -> i32;
  fn _ZNK19QTextBoundaryFinder6stringEv() -> i32;
  fn _ZN19QTextBoundaryFinder7toStartEv() -> i32;
  fn _ZNK19QTextBoundaryFinder8positionEv() -> i32;
}

// body block begin
// class sizeof(QTextBoundaryFinder)=48
pub struct QTextBoundaryFinder {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn isAtBoundary<T: QTextBoundaryFinder_isAtBoundary>(&mut self, value: T) -> i32 {
    value.isAtBoundary(self);
    return 1;
  }
}

pub trait QTextBoundaryFinder_isAtBoundary {
  fn isAtBoundary(self, this: &mut QTextBoundaryFinder) -> i32;
}

// proto: bool QTextBoundaryFinder::isAtBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_isAtBoundary for () {
  fn isAtBoundary(self, this: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder12isAtBoundaryEv()};
    unsafe {_ZNK19QTextBoundaryFinder12isAtBoundaryEv()};
    return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn toNextBoundary<T: QTextBoundaryFinder_toNextBoundary>(&mut self, value: T) -> i32 {
    value.toNextBoundary(self);
    return 1;
  }
}

pub trait QTextBoundaryFinder_toNextBoundary {
  fn toNextBoundary(self, this: &mut QTextBoundaryFinder) -> i32;
}

// proto: int QTextBoundaryFinder::toNextBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_toNextBoundary for () {
  fn toNextBoundary(self, this: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder14toNextBoundaryEv()};
    unsafe {_ZN19QTextBoundaryFinder14toNextBoundaryEv()};
    return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn toEnd<T: QTextBoundaryFinder_toEnd>(&mut self, value: T) -> i32 {
    value.toEnd(self);
    return 1;
  }
}

pub trait QTextBoundaryFinder_toEnd {
  fn toEnd(self, this: &mut QTextBoundaryFinder) -> i32;
}

// proto: void QTextBoundaryFinder::toEnd();
impl<'a> /*trait*/ QTextBoundaryFinder_toEnd for () {
  fn toEnd(self, this: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder5toEndEv()};
    unsafe {_ZN19QTextBoundaryFinder5toEndEv()};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QTextBoundaryFinderC1ERKS_(qthis, arg0)};
    let rsthis = QTextBoundaryFinder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn setPosition<T: QTextBoundaryFinder_setPosition>(&mut self, value: T) -> i32 {
    value.setPosition(self);
    return 1;
  }
}

pub trait QTextBoundaryFinder_setPosition {
  fn setPosition(self, this: &mut QTextBoundaryFinder) -> i32;
}

// proto: void QTextBoundaryFinder::setPosition(int position);
impl<'a> /*trait*/ QTextBoundaryFinder_setPosition for (i32) {
  fn setPosition(self, this: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder11setPositionEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QTextBoundaryFinder11setPositionEi(arg0)};
    return 1;
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
    value.toPreviousBoundary(self);
    return 1;
  }
}

pub trait QTextBoundaryFinder_toPreviousBoundary {
  fn toPreviousBoundary(self, this: &mut QTextBoundaryFinder) -> i32;
}

// proto: int QTextBoundaryFinder::toPreviousBoundary();
impl<'a> /*trait*/ QTextBoundaryFinder_toPreviousBoundary for () {
  fn toPreviousBoundary(self, this: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder18toPreviousBoundaryEv()};
    unsafe {_ZN19QTextBoundaryFinder18toPreviousBoundaryEv()};
    return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn isValid<T: QTextBoundaryFinder_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QTextBoundaryFinder_isValid {
  fn isValid(self, this: &mut QTextBoundaryFinder) -> i32;
}

// proto: bool QTextBoundaryFinder::isValid();
impl<'a> /*trait*/ QTextBoundaryFinder_isValid for () {
  fn isValid(self, this: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder7isValidEv()};
    unsafe {_ZNK19QTextBoundaryFinder7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn FreeQTextBoundaryFinder<T: QTextBoundaryFinder_FreeQTextBoundaryFinder>(&mut self, value: T) -> i32 {
    value.FreeQTextBoundaryFinder(self);
    return 1;
  }
}

pub trait QTextBoundaryFinder_FreeQTextBoundaryFinder {
  fn FreeQTextBoundaryFinder(self, this: &mut QTextBoundaryFinder) -> i32;
}

// proto: void QTextBoundaryFinder::FreeQTextBoundaryFinder();
impl<'a> /*trait*/ QTextBoundaryFinder_FreeQTextBoundaryFinder for () {
  fn FreeQTextBoundaryFinder(self, this: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinderD0Ev()};
    unsafe {_ZN19QTextBoundaryFinderD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn string<T: QTextBoundaryFinder_string>(&mut self, value: T) -> i32 {
    value.string(self);
    return 1;
  }
}

pub trait QTextBoundaryFinder_string {
  fn string(self, this: &mut QTextBoundaryFinder) -> i32;
}

// proto: QString QTextBoundaryFinder::string();
impl<'a> /*trait*/ QTextBoundaryFinder_string for () {
  fn string(self, this: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder6stringEv()};
    unsafe {_ZNK19QTextBoundaryFinder6stringEv()};
    return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn toStart<T: QTextBoundaryFinder_toStart>(&mut self, value: T) -> i32 {
    value.toStart(self);
    return 1;
  }
}

pub trait QTextBoundaryFinder_toStart {
  fn toStart(self, this: &mut QTextBoundaryFinder) -> i32;
}

// proto: void QTextBoundaryFinder::toStart();
impl<'a> /*trait*/ QTextBoundaryFinder_toStart for () {
  fn toStart(self, this: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN19QTextBoundaryFinder7toStartEv()};
    unsafe {_ZN19QTextBoundaryFinder7toStartEv()};
    return 1;
  }
}

impl /*struct*/ QTextBoundaryFinder {
  pub fn position<T: QTextBoundaryFinder_position>(&mut self, value: T) -> i32 {
    value.position(self);
    return 1;
  }
}

pub trait QTextBoundaryFinder_position {
  fn position(self, this: &mut QTextBoundaryFinder) -> i32;
}

// proto: int QTextBoundaryFinder::position();
impl<'a> /*trait*/ QTextBoundaryFinder_position for () {
  fn position(self, this: &mut QTextBoundaryFinder) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK19QTextBoundaryFinder8positionEv()};
    unsafe {_ZNK19QTextBoundaryFinder8positionEv()};
    return 1;
  }
}

