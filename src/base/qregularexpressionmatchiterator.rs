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
  fn _ZNK31QRegularExpressionMatchIterator7hasNextEv() -> i32;
  fn _ZNK31QRegularExpressionMatchIterator7isValidEv() -> i32;
  fn _ZNK31QRegularExpressionMatchIterator8peekNextEv() -> i32;
  fn _ZN31QRegularExpressionMatchIteratorC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK31QRegularExpressionMatchIterator17regularExpressionEv() -> i32;
  fn _ZN31QRegularExpressionMatchIteratorC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN31QRegularExpressionMatchIteratorD0Ev() -> i32;
  fn _ZN31QRegularExpressionMatchIterator4nextEv() -> i32;
  fn _ZN31QRegularExpressionMatchIterator4swapERS_(arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QRegularExpressionMatchIterator)=1
pub struct QRegularExpressionMatchIterator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn hasNext<T: QRegularExpressionMatchIterator_hasNext>(&mut self, value: T) -> i32 {
    value.hasNext(self);
    return 1;
  }
}

pub trait QRegularExpressionMatchIterator_hasNext {
  fn hasNext(self, this: &mut QRegularExpressionMatchIterator) -> i32;
}

// proto: bool QRegularExpressionMatchIterator::hasNext();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_hasNext for () {
  fn hasNext(self, this: &mut QRegularExpressionMatchIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator7hasNextEv()};
    unsafe {_ZNK31QRegularExpressionMatchIterator7hasNextEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn isValid<T: QRegularExpressionMatchIterator_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QRegularExpressionMatchIterator_isValid {
  fn isValid(self, this: &mut QRegularExpressionMatchIterator) -> i32;
}

// proto: bool QRegularExpressionMatchIterator::isValid();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_isValid for () {
  fn isValid(self, this: &mut QRegularExpressionMatchIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator7isValidEv()};
    unsafe {_ZNK31QRegularExpressionMatchIterator7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn peekNext<T: QRegularExpressionMatchIterator_peekNext>(&mut self, value: T) -> i32 {
    value.peekNext(self);
    return 1;
  }
}

pub trait QRegularExpressionMatchIterator_peekNext {
  fn peekNext(self, this: &mut QRegularExpressionMatchIterator) -> i32;
}

// proto: QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_peekNext for () {
  fn peekNext(self, this: &mut QRegularExpressionMatchIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator8peekNextEv()};
    unsafe {_ZNK31QRegularExpressionMatchIterator8peekNextEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn NewQRegularExpressionMatchIterator<T: QRegularExpressionMatchIterator_NewQRegularExpressionMatchIterator>(value: T) -> QRegularExpressionMatchIterator {
    let rsthis = value.NewQRegularExpressionMatchIterator();
    return rsthis;
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_NewQRegularExpressionMatchIterator {
  fn NewQRegularExpressionMatchIterator(self) -> QRegularExpressionMatchIterator;
}

// proto: void QRegularExpressionMatchIterator::NewQRegularExpressionMatchIterator();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_NewQRegularExpressionMatchIterator for () {
  fn NewQRegularExpressionMatchIterator(self) -> QRegularExpressionMatchIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorC1Ev()};
    unsafe {_ZN31QRegularExpressionMatchIteratorC1Ev(qthis)};
    let rsthis = QRegularExpressionMatchIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn regularExpression<T: QRegularExpressionMatchIterator_regularExpression>(&mut self, value: T) -> i32 {
    value.regularExpression(self);
    return 1;
  }
}

pub trait QRegularExpressionMatchIterator_regularExpression {
  fn regularExpression(self, this: &mut QRegularExpressionMatchIterator) -> i32;
}

// proto: QRegularExpression QRegularExpressionMatchIterator::regularExpression();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_regularExpression for () {
  fn regularExpression(self, this: &mut QRegularExpressionMatchIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator17regularExpressionEv()};
    unsafe {_ZNK31QRegularExpressionMatchIterator17regularExpressionEv()};
    return 1;
  }
}

// proto: void QRegularExpressionMatchIterator::NewQRegularExpressionMatchIterator(const QRegularExpressionMatchIterator & iterator);
impl<'a> /*trait*/ QRegularExpressionMatchIterator_NewQRegularExpressionMatchIterator for (&'a  QRegularExpressionMatchIterator) {
  fn NewQRegularExpressionMatchIterator(self) -> QRegularExpressionMatchIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN31QRegularExpressionMatchIteratorC1ERKS_(qthis, arg0)};
    let rsthis = QRegularExpressionMatchIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn FreeQRegularExpressionMatchIterator<T: QRegularExpressionMatchIterator_FreeQRegularExpressionMatchIterator>(&mut self, value: T) -> i32 {
    value.FreeQRegularExpressionMatchIterator(self);
    return 1;
  }
}

pub trait QRegularExpressionMatchIterator_FreeQRegularExpressionMatchIterator {
  fn FreeQRegularExpressionMatchIterator(self, this: &mut QRegularExpressionMatchIterator) -> i32;
}

// proto: void QRegularExpressionMatchIterator::FreeQRegularExpressionMatchIterator();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_FreeQRegularExpressionMatchIterator for () {
  fn FreeQRegularExpressionMatchIterator(self, this: &mut QRegularExpressionMatchIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorD0Ev()};
    unsafe {_ZN31QRegularExpressionMatchIteratorD0Ev()};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn next<T: QRegularExpressionMatchIterator_next>(&mut self, value: T) -> i32 {
    value.next(self);
    return 1;
  }
}

pub trait QRegularExpressionMatchIterator_next {
  fn next(self, this: &mut QRegularExpressionMatchIterator) -> i32;
}

// proto: QRegularExpressionMatch QRegularExpressionMatchIterator::next();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_next for () {
  fn next(self, this: &mut QRegularExpressionMatchIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIterator4nextEv()};
    unsafe {_ZN31QRegularExpressionMatchIterator4nextEv()};
    return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn swap<T: QRegularExpressionMatchIterator_swap>(&mut self, value: T) -> i32 {
    value.swap(self);
    return 1;
  }
}

pub trait QRegularExpressionMatchIterator_swap {
  fn swap(self, this: &mut QRegularExpressionMatchIterator) -> i32;
}

// proto: void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
impl<'a> /*trait*/ QRegularExpressionMatchIterator_swap for (&'a mut QRegularExpressionMatchIterator) {
  fn swap(self, this: &mut QRegularExpressionMatchIterator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIterator4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN31QRegularExpressionMatchIterator4swapERS_(arg0)};
    return 1;
  }
}

