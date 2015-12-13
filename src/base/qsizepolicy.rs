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
  // proto:  bool QSizePolicy::hasHeightForWidth();
  fn _ZNK11QSizePolicy17hasHeightForWidthEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QSizePolicy::retainSizeWhenHidden();
  fn _ZNK11QSizePolicy20retainSizeWhenHiddenEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QSizePolicy::hasWidthForHeight();
  fn _ZNK11QSizePolicy17hasWidthForHeightEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSizePolicy::transpose();
  fn _ZN11QSizePolicy9transposeEv(qthis: *mut c_void) ;
  // proto:  void QSizePolicy::setWidthForHeight(bool b);
  fn _ZN11QSizePolicy17setWidthForHeightEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QSizePolicy::setVerticalStretch(int stretchFactor);
  fn _ZN11QSizePolicy18setVerticalStretchEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSizePolicy::setHeightForWidth(bool b);
  fn _ZN11QSizePolicy17setHeightForWidthEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QSizePolicy::setRetainSizeWhenHidden(bool retainSize);
  fn _ZN11QSizePolicy23setRetainSizeWhenHiddenEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QSizePolicy::horizontalStretch();
  fn _ZNK11QSizePolicy17horizontalStretchEv(qthis: *mut c_void) -> c_int;
  // proto:  void QSizePolicy::setHorizontalStretch(int stretchFactor);
  fn _ZN11QSizePolicy20setHorizontalStretchEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSizePolicy::NewQSizePolicy(int i);
  fn _ZN11QSizePolicyC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSizePolicy::NewQSizePolicy();
  fn _ZN11QSizePolicyC1Ev(qthis: *mut c_void) ;
  // proto:  int QSizePolicy::verticalStretch();
  fn _ZNK11QSizePolicy15verticalStretchEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QSizePolicy)=4
pub struct QSizePolicy {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSizePolicy {
  pub fn hasHeightForWidth<T: QSizePolicy_hasHeightForWidth>(&mut self, value: T) -> i8 {
    return value.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QSizePolicy_hasHeightForWidth {
  fn hasHeightForWidth(self, rsthis: &mut QSizePolicy) -> i8;
}

// proto:  bool QSizePolicy::hasHeightForWidth();
impl<'a> /*trait*/ QSizePolicy_hasHeightForWidth for () {
  fn hasHeightForWidth(self, rsthis: &mut QSizePolicy) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK11QSizePolicy17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn retainSizeWhenHidden<T: QSizePolicy_retainSizeWhenHidden>(&mut self, value: T) -> i8 {
    return value.retainSizeWhenHidden(self);
    // return 1;
  }
}

pub trait QSizePolicy_retainSizeWhenHidden {
  fn retainSizeWhenHidden(self, rsthis: &mut QSizePolicy) -> i8;
}

// proto:  bool QSizePolicy::retainSizeWhenHidden();
impl<'a> /*trait*/ QSizePolicy_retainSizeWhenHidden for () {
  fn retainSizeWhenHidden(self, rsthis: &mut QSizePolicy) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy20retainSizeWhenHiddenEv()};
    let mut ret = unsafe {_ZNK11QSizePolicy20retainSizeWhenHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn hasWidthForHeight<T: QSizePolicy_hasWidthForHeight>(&mut self, value: T) -> i8 {
    return value.hasWidthForHeight(self);
    // return 1;
  }
}

pub trait QSizePolicy_hasWidthForHeight {
  fn hasWidthForHeight(self, rsthis: &mut QSizePolicy) -> i8;
}

// proto:  bool QSizePolicy::hasWidthForHeight();
impl<'a> /*trait*/ QSizePolicy_hasWidthForHeight for () {
  fn hasWidthForHeight(self, rsthis: &mut QSizePolicy) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy17hasWidthForHeightEv()};
    let mut ret = unsafe {_ZNK11QSizePolicy17hasWidthForHeightEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn transpose<T: QSizePolicy_transpose>(&mut self, value: T)  {
     value.transpose(self);
    // return 1;
  }
}

pub trait QSizePolicy_transpose {
  fn transpose(self, rsthis: &mut QSizePolicy) ;
}

// proto:  void QSizePolicy::transpose();
impl<'a> /*trait*/ QSizePolicy_transpose for () {
  fn transpose(self, rsthis: &mut QSizePolicy)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy9transposeEv()};
     unsafe {_ZN11QSizePolicy9transposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn setWidthForHeight<T: QSizePolicy_setWidthForHeight>(&mut self, value: T)  {
     value.setWidthForHeight(self);
    // return 1;
  }
}

pub trait QSizePolicy_setWidthForHeight {
  fn setWidthForHeight(self, rsthis: &mut QSizePolicy) ;
}

// proto:  void QSizePolicy::setWidthForHeight(bool b);
impl<'a> /*trait*/ QSizePolicy_setWidthForHeight for (i8) {
  fn setWidthForHeight(self, rsthis: &mut QSizePolicy)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy17setWidthForHeightEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QSizePolicy17setWidthForHeightEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn setVerticalStretch<T: QSizePolicy_setVerticalStretch>(&mut self, value: T)  {
     value.setVerticalStretch(self);
    // return 1;
  }
}

pub trait QSizePolicy_setVerticalStretch {
  fn setVerticalStretch(self, rsthis: &mut QSizePolicy) ;
}

// proto:  void QSizePolicy::setVerticalStretch(int stretchFactor);
impl<'a> /*trait*/ QSizePolicy_setVerticalStretch for (i32) {
  fn setVerticalStretch(self, rsthis: &mut QSizePolicy)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy18setVerticalStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QSizePolicy18setVerticalStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn setHeightForWidth<T: QSizePolicy_setHeightForWidth>(&mut self, value: T)  {
     value.setHeightForWidth(self);
    // return 1;
  }
}

pub trait QSizePolicy_setHeightForWidth {
  fn setHeightForWidth(self, rsthis: &mut QSizePolicy) ;
}

// proto:  void QSizePolicy::setHeightForWidth(bool b);
impl<'a> /*trait*/ QSizePolicy_setHeightForWidth for (i8) {
  fn setHeightForWidth(self, rsthis: &mut QSizePolicy)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy17setHeightForWidthEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QSizePolicy17setHeightForWidthEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn setRetainSizeWhenHidden<T: QSizePolicy_setRetainSizeWhenHidden>(&mut self, value: T)  {
     value.setRetainSizeWhenHidden(self);
    // return 1;
  }
}

pub trait QSizePolicy_setRetainSizeWhenHidden {
  fn setRetainSizeWhenHidden(self, rsthis: &mut QSizePolicy) ;
}

// proto:  void QSizePolicy::setRetainSizeWhenHidden(bool retainSize);
impl<'a> /*trait*/ QSizePolicy_setRetainSizeWhenHidden for (i8) {
  fn setRetainSizeWhenHidden(self, rsthis: &mut QSizePolicy)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy23setRetainSizeWhenHiddenEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QSizePolicy23setRetainSizeWhenHiddenEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn horizontalStretch<T: QSizePolicy_horizontalStretch>(&mut self, value: T) -> i32 {
    return value.horizontalStretch(self);
    // return 1;
  }
}

pub trait QSizePolicy_horizontalStretch {
  fn horizontalStretch(self, rsthis: &mut QSizePolicy) -> i32;
}

// proto:  int QSizePolicy::horizontalStretch();
impl<'a> /*trait*/ QSizePolicy_horizontalStretch for () {
  fn horizontalStretch(self, rsthis: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy17horizontalStretchEv()};
    let mut ret = unsafe {_ZNK11QSizePolicy17horizontalStretchEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn setHorizontalStretch<T: QSizePolicy_setHorizontalStretch>(&mut self, value: T)  {
     value.setHorizontalStretch(self);
    // return 1;
  }
}

pub trait QSizePolicy_setHorizontalStretch {
  fn setHorizontalStretch(self, rsthis: &mut QSizePolicy) ;
}

// proto:  void QSizePolicy::setHorizontalStretch(int stretchFactor);
impl<'a> /*trait*/ QSizePolicy_setHorizontalStretch for (i32) {
  fn setHorizontalStretch(self, rsthis: &mut QSizePolicy)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy20setHorizontalStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QSizePolicy20setHorizontalStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn NewQSizePolicy<T: QSizePolicy_NewQSizePolicy>(value: T) -> QSizePolicy {
    let rsthis = value.NewQSizePolicy();
    return rsthis;
    // return 1;
  }
}

pub trait QSizePolicy_NewQSizePolicy {
  fn NewQSizePolicy(self) -> QSizePolicy;
}

// proto: void QSizePolicy::NewQSizePolicy(int i);
impl<'a> /*trait*/ QSizePolicy_NewQSizePolicy for (i32) {
  fn NewQSizePolicy(self) -> QSizePolicy {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicyC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QSizePolicyC1Ei(qthis, arg0)};
    let rsthis = QSizePolicy{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QSizePolicy::NewQSizePolicy();
impl<'a> /*trait*/ QSizePolicy_NewQSizePolicy for () {
  fn NewQSizePolicy(self) -> QSizePolicy {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicyC1Ev()};
    unsafe {_ZN11QSizePolicyC1Ev(qthis)};
    let rsthis = QSizePolicy{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn verticalStretch<T: QSizePolicy_verticalStretch>(&mut self, value: T) -> i32 {
    return value.verticalStretch(self);
    // return 1;
  }
}

pub trait QSizePolicy_verticalStretch {
  fn verticalStretch(self, rsthis: &mut QSizePolicy) -> i32;
}

// proto:  int QSizePolicy::verticalStretch();
impl<'a> /*trait*/ QSizePolicy_verticalStretch for () {
  fn verticalStretch(self, rsthis: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy15verticalStretchEv()};
    let mut ret = unsafe {_ZNK11QSizePolicy15verticalStretchEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

