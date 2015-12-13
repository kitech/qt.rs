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
  // proto: bool QSizePolicy::hasHeightForWidth();
  fn _ZNK11QSizePolicy17hasHeightForWidthEv() -> i32;
  // proto: bool QSizePolicy::retainSizeWhenHidden();
  fn _ZNK11QSizePolicy20retainSizeWhenHiddenEv() -> i32;
  // proto: bool QSizePolicy::hasWidthForHeight();
  fn _ZNK11QSizePolicy17hasWidthForHeightEv() -> i32;
  // proto: void QSizePolicy::transpose();
  fn _ZN11QSizePolicy9transposeEv() -> i32;
  // proto: void QSizePolicy::setWidthForHeight(bool b);
  fn _ZN11QSizePolicy17setWidthForHeightEb(arg0: int8_t) -> i32;
  // proto: void QSizePolicy::setVerticalStretch(int stretchFactor);
  fn _ZN11QSizePolicy18setVerticalStretchEi(arg0: c_int) -> i32;
  // proto: void QSizePolicy::setHeightForWidth(bool b);
  fn _ZN11QSizePolicy17setHeightForWidthEb(arg0: int8_t) -> i32;
  // proto: void QSizePolicy::setRetainSizeWhenHidden(bool retainSize);
  fn _ZN11QSizePolicy23setRetainSizeWhenHiddenEb(arg0: int8_t) -> i32;
  // proto: int QSizePolicy::horizontalStretch();
  fn _ZNK11QSizePolicy17horizontalStretchEv() -> i32;
  // proto: void QSizePolicy::setHorizontalStretch(int stretchFactor);
  fn _ZN11QSizePolicy20setHorizontalStretchEi(arg0: c_int) -> i32;
  // proto: void QSizePolicy::NewQSizePolicy(int i);
  fn _ZN11QSizePolicyC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
  // proto: void QSizePolicy::NewQSizePolicy();
  fn _ZN11QSizePolicyC1Ev(qthis: *mut c_void) -> i32;
  // proto: int QSizePolicy::verticalStretch();
  fn _ZNK11QSizePolicy15verticalStretchEv() -> i32;
}

// body block begin
// class sizeof(QSizePolicy)=4
pub struct QSizePolicy {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSizePolicy {
  pub fn hasHeightForWidth<T: QSizePolicy_hasHeightForWidth>(&mut self, value: T) -> i32 {
    value.hasHeightForWidth(self);
    return 1;
  }
}

pub trait QSizePolicy_hasHeightForWidth {
  fn hasHeightForWidth(self, this: &mut QSizePolicy) -> i32;
}

// proto: bool QSizePolicy::hasHeightForWidth();
impl<'a> /*trait*/ QSizePolicy_hasHeightForWidth for () {
  fn hasHeightForWidth(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy17hasHeightForWidthEv()};
    unsafe {_ZNK11QSizePolicy17hasHeightForWidthEv()};
    return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn retainSizeWhenHidden<T: QSizePolicy_retainSizeWhenHidden>(&mut self, value: T) -> i32 {
    value.retainSizeWhenHidden(self);
    return 1;
  }
}

pub trait QSizePolicy_retainSizeWhenHidden {
  fn retainSizeWhenHidden(self, this: &mut QSizePolicy) -> i32;
}

// proto: bool QSizePolicy::retainSizeWhenHidden();
impl<'a> /*trait*/ QSizePolicy_retainSizeWhenHidden for () {
  fn retainSizeWhenHidden(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy20retainSizeWhenHiddenEv()};
    unsafe {_ZNK11QSizePolicy20retainSizeWhenHiddenEv()};
    return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn hasWidthForHeight<T: QSizePolicy_hasWidthForHeight>(&mut self, value: T) -> i32 {
    value.hasWidthForHeight(self);
    return 1;
  }
}

pub trait QSizePolicy_hasWidthForHeight {
  fn hasWidthForHeight(self, this: &mut QSizePolicy) -> i32;
}

// proto: bool QSizePolicy::hasWidthForHeight();
impl<'a> /*trait*/ QSizePolicy_hasWidthForHeight for () {
  fn hasWidthForHeight(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy17hasWidthForHeightEv()};
    unsafe {_ZNK11QSizePolicy17hasWidthForHeightEv()};
    return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn transpose<T: QSizePolicy_transpose>(&mut self, value: T) -> i32 {
    value.transpose(self);
    return 1;
  }
}

pub trait QSizePolicy_transpose {
  fn transpose(self, this: &mut QSizePolicy) -> i32;
}

// proto: void QSizePolicy::transpose();
impl<'a> /*trait*/ QSizePolicy_transpose for () {
  fn transpose(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy9transposeEv()};
    unsafe {_ZN11QSizePolicy9transposeEv()};
    return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn setWidthForHeight<T: QSizePolicy_setWidthForHeight>(&mut self, value: T) -> i32 {
    value.setWidthForHeight(self);
    return 1;
  }
}

pub trait QSizePolicy_setWidthForHeight {
  fn setWidthForHeight(self, this: &mut QSizePolicy) -> i32;
}

// proto: void QSizePolicy::setWidthForHeight(bool b);
impl<'a> /*trait*/ QSizePolicy_setWidthForHeight for (i8) {
  fn setWidthForHeight(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy17setWidthForHeightEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QSizePolicy17setWidthForHeightEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn setVerticalStretch<T: QSizePolicy_setVerticalStretch>(&mut self, value: T) -> i32 {
    value.setVerticalStretch(self);
    return 1;
  }
}

pub trait QSizePolicy_setVerticalStretch {
  fn setVerticalStretch(self, this: &mut QSizePolicy) -> i32;
}

// proto: void QSizePolicy::setVerticalStretch(int stretchFactor);
impl<'a> /*trait*/ QSizePolicy_setVerticalStretch for (i32) {
  fn setVerticalStretch(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy18setVerticalStretchEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QSizePolicy18setVerticalStretchEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn setHeightForWidth<T: QSizePolicy_setHeightForWidth>(&mut self, value: T) -> i32 {
    value.setHeightForWidth(self);
    return 1;
  }
}

pub trait QSizePolicy_setHeightForWidth {
  fn setHeightForWidth(self, this: &mut QSizePolicy) -> i32;
}

// proto: void QSizePolicy::setHeightForWidth(bool b);
impl<'a> /*trait*/ QSizePolicy_setHeightForWidth for (i8) {
  fn setHeightForWidth(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy17setHeightForWidthEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QSizePolicy17setHeightForWidthEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn setRetainSizeWhenHidden<T: QSizePolicy_setRetainSizeWhenHidden>(&mut self, value: T) -> i32 {
    value.setRetainSizeWhenHidden(self);
    return 1;
  }
}

pub trait QSizePolicy_setRetainSizeWhenHidden {
  fn setRetainSizeWhenHidden(self, this: &mut QSizePolicy) -> i32;
}

// proto: void QSizePolicy::setRetainSizeWhenHidden(bool retainSize);
impl<'a> /*trait*/ QSizePolicy_setRetainSizeWhenHidden for (i8) {
  fn setRetainSizeWhenHidden(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy23setRetainSizeWhenHiddenEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN11QSizePolicy23setRetainSizeWhenHiddenEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn horizontalStretch<T: QSizePolicy_horizontalStretch>(&mut self, value: T) -> i32 {
    value.horizontalStretch(self);
    return 1;
  }
}

pub trait QSizePolicy_horizontalStretch {
  fn horizontalStretch(self, this: &mut QSizePolicy) -> i32;
}

// proto: int QSizePolicy::horizontalStretch();
impl<'a> /*trait*/ QSizePolicy_horizontalStretch for () {
  fn horizontalStretch(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy17horizontalStretchEv()};
    unsafe {_ZNK11QSizePolicy17horizontalStretchEv()};
    return 1;
  }
}

impl /*struct*/ QSizePolicy {
  pub fn setHorizontalStretch<T: QSizePolicy_setHorizontalStretch>(&mut self, value: T) -> i32 {
    value.setHorizontalStretch(self);
    return 1;
  }
}

pub trait QSizePolicy_setHorizontalStretch {
  fn setHorizontalStretch(self, this: &mut QSizePolicy) -> i32;
}

// proto: void QSizePolicy::setHorizontalStretch(int stretchFactor);
impl<'a> /*trait*/ QSizePolicy_setHorizontalStretch for (i32) {
  fn setHorizontalStretch(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSizePolicy20setHorizontalStretchEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN11QSizePolicy20setHorizontalStretchEi(arg0)};
    return 1;
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
    value.verticalStretch(self);
    return 1;
  }
}

pub trait QSizePolicy_verticalStretch {
  fn verticalStretch(self, this: &mut QSizePolicy) -> i32;
}

// proto: int QSizePolicy::verticalStretch();
impl<'a> /*trait*/ QSizePolicy_verticalStretch for () {
  fn verticalStretch(self, this: &mut QSizePolicy) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QSizePolicy15verticalStretchEv()};
    unsafe {_ZNK11QSizePolicy15verticalStretchEv()};
    return 1;
  }
}

