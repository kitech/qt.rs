// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qregularexpressionmatch::QRegularExpressionMatch;
use super::qregularexpression::QRegularExpression;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QRegularExpressionMatchIterator::hasNext();
  fn _ZNK31QRegularExpressionMatchIterator7hasNextEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QRegularExpressionMatchIterator::isValid();
  fn _ZNK31QRegularExpressionMatchIterator7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
  fn _ZNK31QRegularExpressionMatchIterator8peekNextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpressionMatchIterator::NewQRegularExpressionMatchIterator();
  fn _ZN31QRegularExpressionMatchIteratorC1Ev(qthis: *mut c_void) ;
  // proto:  QRegularExpression QRegularExpressionMatchIterator::regularExpression();
  fn _ZNK31QRegularExpressionMatchIterator17regularExpressionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpressionMatchIterator::NewQRegularExpressionMatchIterator(const QRegularExpressionMatchIterator & iterator);
  fn _ZN31QRegularExpressionMatchIteratorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRegularExpressionMatchIterator::FreeQRegularExpressionMatchIterator();
  fn _ZN31QRegularExpressionMatchIteratorD0Ev(qthis: *mut c_void) ;
  // proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::next();
  fn _ZN31QRegularExpressionMatchIterator4nextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
  fn _ZN31QRegularExpressionMatchIterator4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QRegularExpressionMatchIterator)=1
pub struct QRegularExpressionMatchIterator {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn hasNext<T: QRegularExpressionMatchIterator_hasNext>(&mut self, value: T) -> i8 {
    return value.hasNext(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_hasNext {
  fn hasNext(self, rsthis: &mut QRegularExpressionMatchIterator) -> i8;
}

// proto:  bool QRegularExpressionMatchIterator::hasNext();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_hasNext for () {
  fn hasNext(self, rsthis: &mut QRegularExpressionMatchIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator7hasNextEv()};
    let mut ret = unsafe {_ZNK31QRegularExpressionMatchIterator7hasNextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn isValid<T: QRegularExpressionMatchIterator_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_isValid {
  fn isValid(self, rsthis: &mut QRegularExpressionMatchIterator) -> i8;
}

// proto:  bool QRegularExpressionMatchIterator::isValid();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_isValid for () {
  fn isValid(self, rsthis: &mut QRegularExpressionMatchIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator7isValidEv()};
    let mut ret = unsafe {_ZNK31QRegularExpressionMatchIterator7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn peekNext<T: QRegularExpressionMatchIterator_peekNext>(&mut self, value: T) -> QRegularExpressionMatch {
    return value.peekNext(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_peekNext {
  fn peekNext(self, rsthis: &mut QRegularExpressionMatchIterator) -> QRegularExpressionMatch;
}

// proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_peekNext for () {
  fn peekNext(self, rsthis: &mut QRegularExpressionMatchIterator) -> QRegularExpressionMatch {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator8peekNextEv()};
    let mut ret = unsafe {_ZNK31QRegularExpressionMatchIterator8peekNextEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpressionMatch{qclsinst: ret};
    return ret1;
    // return 1;
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
  pub fn regularExpression<T: QRegularExpressionMatchIterator_regularExpression>(&mut self, value: T) -> QRegularExpression {
    return value.regularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_regularExpression {
  fn regularExpression(self, rsthis: &mut QRegularExpressionMatchIterator) -> QRegularExpression;
}

// proto:  QRegularExpression QRegularExpressionMatchIterator::regularExpression();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_regularExpression for () {
  fn regularExpression(self, rsthis: &mut QRegularExpressionMatchIterator) -> QRegularExpression {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator17regularExpressionEv()};
    let mut ret = unsafe {_ZNK31QRegularExpressionMatchIterator17regularExpressionEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpression{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QRegularExpressionMatchIterator::NewQRegularExpressionMatchIterator(const QRegularExpressionMatchIterator & iterator);
impl<'a> /*trait*/ QRegularExpressionMatchIterator_NewQRegularExpressionMatchIterator for (&'a  QRegularExpressionMatchIterator) {
  fn NewQRegularExpressionMatchIterator(self) -> QRegularExpressionMatchIterator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN31QRegularExpressionMatchIteratorC1ERKS_(qthis, arg0)};
    let rsthis = QRegularExpressionMatchIterator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn FreeQRegularExpressionMatchIterator<T: QRegularExpressionMatchIterator_FreeQRegularExpressionMatchIterator>(&mut self, value: T)  {
     value.FreeQRegularExpressionMatchIterator(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_FreeQRegularExpressionMatchIterator {
  fn FreeQRegularExpressionMatchIterator(self, rsthis: &mut QRegularExpressionMatchIterator) ;
}

// proto:  void QRegularExpressionMatchIterator::FreeQRegularExpressionMatchIterator();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_FreeQRegularExpressionMatchIterator for () {
  fn FreeQRegularExpressionMatchIterator(self, rsthis: &mut QRegularExpressionMatchIterator)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorD0Ev()};
     unsafe {_ZN31QRegularExpressionMatchIteratorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn next<T: QRegularExpressionMatchIterator_next>(&mut self, value: T) -> QRegularExpressionMatch {
    return value.next(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_next {
  fn next(self, rsthis: &mut QRegularExpressionMatchIterator) -> QRegularExpressionMatch;
}

// proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::next();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_next for () {
  fn next(self, rsthis: &mut QRegularExpressionMatchIterator) -> QRegularExpressionMatch {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIterator4nextEv()};
    let mut ret = unsafe {_ZN31QRegularExpressionMatchIterator4nextEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpressionMatch{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn swap<T: QRegularExpressionMatchIterator_swap>(&mut self, value: T)  {
     value.swap(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_swap {
  fn swap(self, rsthis: &mut QRegularExpressionMatchIterator) ;
}

// proto:  void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
impl<'a> /*trait*/ QRegularExpressionMatchIterator_swap for (&'a mut QRegularExpressionMatchIterator) {
  fn swap(self, rsthis: &mut QRegularExpressionMatchIterator)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIterator4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN31QRegularExpressionMatchIterator4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

