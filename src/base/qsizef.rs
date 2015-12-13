// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN6QSizeF7rheightEv() -> i32;
  fn _ZN6QSizeF6rwidthEv() -> i32;
  fn _ZNK6QSizeF10transposedEv() -> i32;
  fn _ZNK6QSizeF7isValidEv() -> i32;
  fn _ZN6QSizeF9setHeightEd(arg0: c_double) -> i32;
  fn _ZN6QSizeFC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK6QSizeF5widthEv() -> i32;
  fn _ZNK6QSizeF6isNullEv() -> i32;
  fn _ZNK6QSizeF9boundedToERKS_(arg0: *const c_void) -> i32;
  fn _ZNK6QSizeF6heightEv() -> i32;
  fn _ZN6QSizeF9transposeEv() -> i32;
  fn _ZN6QSizeFC1ERK5QSize(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK6QSizeF10expandedToERKS_(arg0: *const c_void) -> i32;
  fn _ZNK6QSizeF7isEmptyEv() -> i32;
  fn _ZN6QSizeF8setWidthEd(arg0: c_double) -> i32;
  fn _ZNK6QSizeF6toSizeEv() -> i32;
  fn _ZN6QSizeFC1Edd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> i32;
}

// body block begin
// class sizeof(QSizeF)=16
pub struct QSizeF {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSizeF {
  pub fn rheight<T: QSizeF_rheight>(&mut self, value: T) -> i32 {
    value.rheight(self);
    return 1;
  }
}

pub trait QSizeF_rheight {
  fn rheight(self, this: &mut QSizeF) -> i32;
}

// proto: qreal & QSizeF::rheight();
impl<'a> /*trait*/ QSizeF_rheight for () {
  fn rheight(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF7rheightEv()};
    unsafe {_ZN6QSizeF7rheightEv()};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn rwidth<T: QSizeF_rwidth>(&mut self, value: T) -> i32 {
    value.rwidth(self);
    return 1;
  }
}

pub trait QSizeF_rwidth {
  fn rwidth(self, this: &mut QSizeF) -> i32;
}

// proto: qreal & QSizeF::rwidth();
impl<'a> /*trait*/ QSizeF_rwidth for () {
  fn rwidth(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF6rwidthEv()};
    unsafe {_ZN6QSizeF6rwidthEv()};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn transposed<T: QSizeF_transposed>(&mut self, value: T) -> i32 {
    value.transposed(self);
    return 1;
  }
}

pub trait QSizeF_transposed {
  fn transposed(self, this: &mut QSizeF) -> i32;
}

// proto: QSizeF QSizeF::transposed();
impl<'a> /*trait*/ QSizeF_transposed for () {
  fn transposed(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF10transposedEv()};
    unsafe {_ZNK6QSizeF10transposedEv()};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn isValid<T: QSizeF_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QSizeF_isValid {
  fn isValid(self, this: &mut QSizeF) -> i32;
}

// proto: bool QSizeF::isValid();
impl<'a> /*trait*/ QSizeF_isValid for () {
  fn isValid(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF7isValidEv()};
    unsafe {_ZNK6QSizeF7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn setHeight<T: QSizeF_setHeight>(&mut self, value: T) -> i32 {
    value.setHeight(self);
    return 1;
  }
}

pub trait QSizeF_setHeight {
  fn setHeight(self, this: &mut QSizeF) -> i32;
}

// proto: void QSizeF::setHeight(qreal h);
impl<'a> /*trait*/ QSizeF_setHeight for (f64) {
  fn setHeight(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF9setHeightEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QSizeF9setHeightEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn NewQSizeF<T: QSizeF_NewQSizeF>(value: T) -> QSizeF {
    let rsthis = value.NewQSizeF();
    return rsthis;
    // return 1;
  }
}

pub trait QSizeF_NewQSizeF {
  fn NewQSizeF(self) -> QSizeF;
}

// proto: void QSizeF::NewQSizeF();
impl<'a> /*trait*/ QSizeF_NewQSizeF for () {
  fn NewQSizeF(self) -> QSizeF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeFC1Ev()};
    unsafe {_ZN6QSizeFC1Ev(qthis)};
    let rsthis = QSizeF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn width<T: QSizeF_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QSizeF_width {
  fn width(self, this: &mut QSizeF) -> i32;
}

// proto: double QSizeF::width();
impl<'a> /*trait*/ QSizeF_width for () {
  fn width(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF5widthEv()};
    unsafe {_ZNK6QSizeF5widthEv()};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn isNull<T: QSizeF_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QSizeF_isNull {
  fn isNull(self, this: &mut QSizeF) -> i32;
}

// proto: bool QSizeF::isNull();
impl<'a> /*trait*/ QSizeF_isNull for () {
  fn isNull(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6isNullEv()};
    unsafe {_ZNK6QSizeF6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn boundedTo<T: QSizeF_boundedTo>(&mut self, value: T) -> i32 {
    value.boundedTo(self);
    return 1;
  }
}

pub trait QSizeF_boundedTo {
  fn boundedTo(self, this: &mut QSizeF) -> i32;
}

// proto: QSizeF QSizeF::boundedTo(const QSizeF & );
impl<'a> /*trait*/ QSizeF_boundedTo for (&'a  QSizeF) {
  fn boundedTo(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF9boundedToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QSizeF9boundedToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn height<T: QSizeF_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QSizeF_height {
  fn height(self, this: &mut QSizeF) -> i32;
}

// proto: double QSizeF::height();
impl<'a> /*trait*/ QSizeF_height for () {
  fn height(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6heightEv()};
    unsafe {_ZNK6QSizeF6heightEv()};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn transpose<T: QSizeF_transpose>(&mut self, value: T) -> i32 {
    value.transpose(self);
    return 1;
  }
}

pub trait QSizeF_transpose {
  fn transpose(self, this: &mut QSizeF) -> i32;
}

// proto: void QSizeF::transpose();
impl<'a> /*trait*/ QSizeF_transpose for () {
  fn transpose(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF9transposeEv()};
    unsafe {_ZN6QSizeF9transposeEv()};
    return 1;
  }
}

// proto: void QSizeF::NewQSizeF(const QSize & sz);
impl<'a> /*trait*/ QSizeF_NewQSizeF for (&'a  QSize) {
  fn NewQSizeF(self) -> QSizeF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeFC1ERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QSizeFC1ERK5QSize(qthis, arg0)};
    let rsthis = QSizeF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn expandedTo<T: QSizeF_expandedTo>(&mut self, value: T) -> i32 {
    value.expandedTo(self);
    return 1;
  }
}

pub trait QSizeF_expandedTo {
  fn expandedTo(self, this: &mut QSizeF) -> i32;
}

// proto: QSizeF QSizeF::expandedTo(const QSizeF & );
impl<'a> /*trait*/ QSizeF_expandedTo for (&'a  QSizeF) {
  fn expandedTo(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF10expandedToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QSizeF10expandedToERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn isEmpty<T: QSizeF_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QSizeF_isEmpty {
  fn isEmpty(self, this: &mut QSizeF) -> i32;
}

// proto: bool QSizeF::isEmpty();
impl<'a> /*trait*/ QSizeF_isEmpty for () {
  fn isEmpty(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF7isEmptyEv()};
    unsafe {_ZNK6QSizeF7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn setWidth<T: QSizeF_setWidth>(&mut self, value: T) -> i32 {
    value.setWidth(self);
    return 1;
  }
}

pub trait QSizeF_setWidth {
  fn setWidth(self, this: &mut QSizeF) -> i32;
}

// proto: void QSizeF::setWidth(qreal w);
impl<'a> /*trait*/ QSizeF_setWidth for (f64) {
  fn setWidth(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF8setWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QSizeF8setWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn toSize<T: QSizeF_toSize>(&mut self, value: T) -> i32 {
    value.toSize(self);
    return 1;
  }
}

pub trait QSizeF_toSize {
  fn toSize(self, this: &mut QSizeF) -> i32;
}

// proto: QSize QSizeF::toSize();
impl<'a> /*trait*/ QSizeF_toSize for () {
  fn toSize(self, this: &mut QSizeF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6toSizeEv()};
    unsafe {_ZNK6QSizeF6toSizeEv()};
    return 1;
  }
}

// proto: void QSizeF::NewQSizeF(qreal w, qreal h);
impl<'a> /*trait*/ QSizeF_NewQSizeF for (f64, f64) {
  fn NewQSizeF(self) -> QSizeF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeFC1Edd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN6QSizeFC1Edd(qthis, arg0, arg1)};
    let rsthis = QSizeF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

