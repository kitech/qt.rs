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
  // proto:  qreal & QSizeF::rheight();
  fn _ZN6QSizeF7rheightEv(qthis: *mut c_void) ;
  // proto:  qreal & QSizeF::rwidth();
  fn _ZN6QSizeF6rwidthEv(qthis: *mut c_void) ;
  // proto:  QSizeF QSizeF::transposed();
  fn _ZNK6QSizeF10transposedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QSizeF::isValid();
  fn _ZNK6QSizeF7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSizeF::setHeight(qreal h);
  fn _ZN6QSizeF9setHeightEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QSizeF::NewQSizeF();
  fn _ZN6QSizeFC1Ev(qthis: *mut c_void) ;
  // proto:  double QSizeF::width();
  fn _ZNK6QSizeF5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QSizeF::isNull();
  fn _ZNK6QSizeF6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSizeF QSizeF::boundedTo(const QSizeF & );
  fn _ZNK6QSizeF9boundedToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  double QSizeF::height();
  fn _ZNK6QSizeF6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QSizeF::transpose();
  fn _ZN6QSizeF9transposeEv(qthis: *mut c_void) ;
  // proto:  void QSizeF::NewQSizeF(const QSize & sz);
  fn _ZN6QSizeFC1ERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSizeF QSizeF::expandedTo(const QSizeF & );
  fn _ZNK6QSizeF10expandedToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QSizeF::isEmpty();
  fn _ZNK6QSizeF7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSizeF::setWidth(qreal w);
  fn _ZN6QSizeF8setWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QSize QSizeF::toSize();
  fn _ZNK6QSizeF6toSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSizeF::NewQSizeF(qreal w, qreal h);
  fn _ZN6QSizeFC1Edd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
}

// body block begin
// class sizeof(QSizeF)=16
pub struct QSizeF {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSizeF {
  pub fn rheight<T: QSizeF_rheight>(&mut self, value: T)  {
     value.rheight(self);
    // return 1;
  }
}

pub trait QSizeF_rheight {
  fn rheight(self, rsthis: &mut QSizeF) ;
}

// proto:  qreal & QSizeF::rheight();
impl<'a> /*trait*/ QSizeF_rheight for () {
  fn rheight(self, rsthis: &mut QSizeF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF7rheightEv()};
     unsafe {_ZN6QSizeF7rheightEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn rwidth<T: QSizeF_rwidth>(&mut self, value: T)  {
     value.rwidth(self);
    // return 1;
  }
}

pub trait QSizeF_rwidth {
  fn rwidth(self, rsthis: &mut QSizeF) ;
}

// proto:  qreal & QSizeF::rwidth();
impl<'a> /*trait*/ QSizeF_rwidth for () {
  fn rwidth(self, rsthis: &mut QSizeF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF6rwidthEv()};
     unsafe {_ZN6QSizeF6rwidthEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn transposed<T: QSizeF_transposed>(&mut self, value: T) -> QSizeF {
    return value.transposed(self);
    // return 1;
  }
}

pub trait QSizeF_transposed {
  fn transposed(self, rsthis: &mut QSizeF) -> QSizeF;
}

// proto:  QSizeF QSizeF::transposed();
impl<'a> /*trait*/ QSizeF_transposed for () {
  fn transposed(self, rsthis: &mut QSizeF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF10transposedEv()};
    let mut ret = unsafe {_ZNK6QSizeF10transposedEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn isValid<T: QSizeF_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QSizeF_isValid {
  fn isValid(self, rsthis: &mut QSizeF) -> i8;
}

// proto:  bool QSizeF::isValid();
impl<'a> /*trait*/ QSizeF_isValid for () {
  fn isValid(self, rsthis: &mut QSizeF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF7isValidEv()};
    let mut ret = unsafe {_ZNK6QSizeF7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn setHeight<T: QSizeF_setHeight>(&mut self, value: T)  {
     value.setHeight(self);
    // return 1;
  }
}

pub trait QSizeF_setHeight {
  fn setHeight(self, rsthis: &mut QSizeF) ;
}

// proto:  void QSizeF::setHeight(qreal h);
impl<'a> /*trait*/ QSizeF_setHeight for (f64) {
  fn setHeight(self, rsthis: &mut QSizeF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QSizeF9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn width<T: QSizeF_width>(&mut self, value: T) -> f64 {
    return value.width(self);
    // return 1;
  }
}

pub trait QSizeF_width {
  fn width(self, rsthis: &mut QSizeF) -> f64;
}

// proto:  double QSizeF::width();
impl<'a> /*trait*/ QSizeF_width for () {
  fn width(self, rsthis: &mut QSizeF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF5widthEv()};
    let mut ret = unsafe {_ZNK6QSizeF5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn isNull<T: QSizeF_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QSizeF_isNull {
  fn isNull(self, rsthis: &mut QSizeF) -> i8;
}

// proto:  bool QSizeF::isNull();
impl<'a> /*trait*/ QSizeF_isNull for () {
  fn isNull(self, rsthis: &mut QSizeF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6isNullEv()};
    let mut ret = unsafe {_ZNK6QSizeF6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn boundedTo<T: QSizeF_boundedTo>(&mut self, value: T) -> QSizeF {
    return value.boundedTo(self);
    // return 1;
  }
}

pub trait QSizeF_boundedTo {
  fn boundedTo(self, rsthis: &mut QSizeF) -> QSizeF;
}

// proto:  QSizeF QSizeF::boundedTo(const QSizeF & );
impl<'a> /*trait*/ QSizeF_boundedTo for (&'a  QSizeF) {
  fn boundedTo(self, rsthis: &mut QSizeF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF9boundedToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QSizeF9boundedToERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn height<T: QSizeF_height>(&mut self, value: T) -> f64 {
    return value.height(self);
    // return 1;
  }
}

pub trait QSizeF_height {
  fn height(self, rsthis: &mut QSizeF) -> f64;
}

// proto:  double QSizeF::height();
impl<'a> /*trait*/ QSizeF_height for () {
  fn height(self, rsthis: &mut QSizeF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6heightEv()};
    let mut ret = unsafe {_ZNK6QSizeF6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn transpose<T: QSizeF_transpose>(&mut self, value: T)  {
     value.transpose(self);
    // return 1;
  }
}

pub trait QSizeF_transpose {
  fn transpose(self, rsthis: &mut QSizeF) ;
}

// proto:  void QSizeF::transpose();
impl<'a> /*trait*/ QSizeF_transpose for () {
  fn transpose(self, rsthis: &mut QSizeF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF9transposeEv()};
     unsafe {_ZN6QSizeF9transposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QSizeF::NewQSizeF(const QSize & sz);
impl<'a> /*trait*/ QSizeF_NewQSizeF for (&'a  QSize) {
  fn NewQSizeF(self) -> QSizeF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeFC1ERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QSizeFC1ERK5QSize(qthis, arg0)};
    let rsthis = QSizeF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn expandedTo<T: QSizeF_expandedTo>(&mut self, value: T) -> QSizeF {
    return value.expandedTo(self);
    // return 1;
  }
}

pub trait QSizeF_expandedTo {
  fn expandedTo(self, rsthis: &mut QSizeF) -> QSizeF;
}

// proto:  QSizeF QSizeF::expandedTo(const QSizeF & );
impl<'a> /*trait*/ QSizeF_expandedTo for (&'a  QSizeF) {
  fn expandedTo(self, rsthis: &mut QSizeF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF10expandedToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QSizeF10expandedToERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn isEmpty<T: QSizeF_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QSizeF_isEmpty {
  fn isEmpty(self, rsthis: &mut QSizeF) -> i8;
}

// proto:  bool QSizeF::isEmpty();
impl<'a> /*trait*/ QSizeF_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QSizeF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF7isEmptyEv()};
    let mut ret = unsafe {_ZNK6QSizeF7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn setWidth<T: QSizeF_setWidth>(&mut self, value: T)  {
     value.setWidth(self);
    // return 1;
  }
}

pub trait QSizeF_setWidth {
  fn setWidth(self, rsthis: &mut QSizeF) ;
}

// proto:  void QSizeF::setWidth(qreal w);
impl<'a> /*trait*/ QSizeF_setWidth for (f64) {
  fn setWidth(self, rsthis: &mut QSizeF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QSizeF8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSizeF {
  pub fn toSize<T: QSizeF_toSize>(&mut self, value: T) -> QSize {
    return value.toSize(self);
    // return 1;
  }
}

pub trait QSizeF_toSize {
  fn toSize(self, rsthis: &mut QSizeF) -> QSize;
}

// proto:  QSize QSizeF::toSize();
impl<'a> /*trait*/ QSizeF_toSize for () {
  fn toSize(self, rsthis: &mut QSizeF) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6toSizeEv()};
    let mut ret = unsafe {_ZNK6QSizeF6toSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
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

