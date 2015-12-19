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

// proto:  bool QRegularExpressionMatchIterator::hasNext();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn hasNext<RetType, T: QRegularExpressionMatchIterator_hasNext<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hasNext(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_hasNext<RetType> {
  fn hasNext(self , rsthis: &mut QRegularExpressionMatchIterator) -> RetType;
}

// proto:  bool QRegularExpressionMatchIterator::hasNext();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_hasNext<i8> for () {
  fn hasNext(self , rsthis: &mut QRegularExpressionMatchIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator7hasNextEv()};
    let mut ret = unsafe {_ZNK31QRegularExpressionMatchIterator7hasNextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QRegularExpressionMatchIterator::isValid();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn isValid<RetType, T: QRegularExpressionMatchIterator_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_isValid<RetType> {
  fn isValid(self , rsthis: &mut QRegularExpressionMatchIterator) -> RetType;
}

// proto:  bool QRegularExpressionMatchIterator::isValid();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QRegularExpressionMatchIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK31QRegularExpressionMatchIterator7isValidEv()};
    let mut ret = unsafe {_ZNK31QRegularExpressionMatchIterator7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn peekNext<RetType, T: QRegularExpressionMatchIterator_peekNext<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.peekNext(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_peekNext<RetType> {
  fn peekNext(self , rsthis: &mut QRegularExpressionMatchIterator) -> RetType;
}

// proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::peekNext();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_peekNext<QRegularExpressionMatch> for () {
  fn peekNext(self , rsthis: &mut QRegularExpressionMatchIterator) -> QRegularExpressionMatch {
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

// proto:  QRegularExpression QRegularExpressionMatchIterator::regularExpression();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn regularExpression<RetType, T: QRegularExpressionMatchIterator_regularExpression<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.regularExpression(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_regularExpression<RetType> {
  fn regularExpression(self , rsthis: &mut QRegularExpressionMatchIterator) -> RetType;
}

// proto:  QRegularExpression QRegularExpressionMatchIterator::regularExpression();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_regularExpression<QRegularExpression> for () {
  fn regularExpression(self , rsthis: &mut QRegularExpressionMatchIterator) -> QRegularExpression {
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

// proto:  void QRegularExpressionMatchIterator::FreeQRegularExpressionMatchIterator();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn FreeQRegularExpressionMatchIterator<RetType, T: QRegularExpressionMatchIterator_FreeQRegularExpressionMatchIterator<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQRegularExpressionMatchIterator(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_FreeQRegularExpressionMatchIterator<RetType> {
  fn FreeQRegularExpressionMatchIterator(self , rsthis: &mut QRegularExpressionMatchIterator) -> RetType;
}

// proto:  void QRegularExpressionMatchIterator::FreeQRegularExpressionMatchIterator();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_FreeQRegularExpressionMatchIterator<()> for () {
  fn FreeQRegularExpressionMatchIterator(self , rsthis: &mut QRegularExpressionMatchIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIteratorD0Ev()};
     unsafe {_ZN31QRegularExpressionMatchIteratorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::next();
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn next<RetType, T: QRegularExpressionMatchIterator_next<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.next(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_next<RetType> {
  fn next(self , rsthis: &mut QRegularExpressionMatchIterator) -> RetType;
}

// proto:  QRegularExpressionMatch QRegularExpressionMatchIterator::next();
impl<'a> /*trait*/ QRegularExpressionMatchIterator_next<QRegularExpressionMatch> for () {
  fn next(self , rsthis: &mut QRegularExpressionMatchIterator) -> QRegularExpressionMatch {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIterator4nextEv()};
    let mut ret = unsafe {_ZN31QRegularExpressionMatchIterator4nextEv(rsthis.qclsinst)};
    let mut ret1 = QRegularExpressionMatch{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
impl /*struct*/ QRegularExpressionMatchIterator {
  pub fn swap<RetType, T: QRegularExpressionMatchIterator_swap<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegularExpressionMatchIterator_swap<RetType> {
  fn swap(self , rsthis: &mut QRegularExpressionMatchIterator) -> RetType;
}

// proto:  void QRegularExpressionMatchIterator::swap(QRegularExpressionMatchIterator & other);
impl<'a> /*trait*/ QRegularExpressionMatchIterator_swap<()> for (&'a mut QRegularExpressionMatchIterator) {
  fn swap(self , rsthis: &mut QRegularExpressionMatchIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN31QRegularExpressionMatchIterator4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN31QRegularExpressionMatchIterator4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

