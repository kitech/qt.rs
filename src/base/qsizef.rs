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

// proto:  qreal & QSizeF::rheight();
impl /*struct*/ QSizeF {
  pub fn rheight<RetType, T: QSizeF_rheight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rheight(self);
    // return 1;
  }
}

pub trait QSizeF_rheight<RetType> {
  fn rheight(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  qreal & QSizeF::rheight();
impl<'a> /*trait*/ QSizeF_rheight<()> for () {
  fn rheight(self , rsthis: &mut QSizeF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF7rheightEv()};
     unsafe {_ZN6QSizeF7rheightEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  qreal & QSizeF::rwidth();
impl /*struct*/ QSizeF {
  pub fn rwidth<RetType, T: QSizeF_rwidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.rwidth(self);
    // return 1;
  }
}

pub trait QSizeF_rwidth<RetType> {
  fn rwidth(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  qreal & QSizeF::rwidth();
impl<'a> /*trait*/ QSizeF_rwidth<()> for () {
  fn rwidth(self , rsthis: &mut QSizeF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF6rwidthEv()};
     unsafe {_ZN6QSizeF6rwidthEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QSizeF QSizeF::transposed();
impl /*struct*/ QSizeF {
  pub fn transposed<RetType, T: QSizeF_transposed<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.transposed(self);
    // return 1;
  }
}

pub trait QSizeF_transposed<RetType> {
  fn transposed(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  QSizeF QSizeF::transposed();
impl<'a> /*trait*/ QSizeF_transposed<QSizeF> for () {
  fn transposed(self , rsthis: &mut QSizeF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF10transposedEv()};
    let mut ret = unsafe {_ZNK6QSizeF10transposedEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QSizeF::isValid();
impl /*struct*/ QSizeF {
  pub fn isValid<RetType, T: QSizeF_isValid<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QSizeF_isValid<RetType> {
  fn isValid(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  bool QSizeF::isValid();
impl<'a> /*trait*/ QSizeF_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QSizeF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF7isValidEv()};
    let mut ret = unsafe {_ZNK6QSizeF7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QSizeF::setHeight(qreal h);
impl /*struct*/ QSizeF {
  pub fn setHeight<RetType, T: QSizeF_setHeight<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QSizeF_setHeight<RetType> {
  fn setHeight(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  void QSizeF::setHeight(qreal h);
impl<'a> /*trait*/ QSizeF_setHeight<()> for (f64) {
  fn setHeight(self , rsthis: &mut QSizeF) -> () {
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

// proto:  double QSizeF::width();
impl /*struct*/ QSizeF {
  pub fn width<RetType, T: QSizeF_width<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QSizeF_width<RetType> {
  fn width(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  double QSizeF::width();
impl<'a> /*trait*/ QSizeF_width<f64> for () {
  fn width(self , rsthis: &mut QSizeF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF5widthEv()};
    let mut ret = unsafe {_ZNK6QSizeF5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  bool QSizeF::isNull();
impl /*struct*/ QSizeF {
  pub fn isNull<RetType, T: QSizeF_isNull<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QSizeF_isNull<RetType> {
  fn isNull(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  bool QSizeF::isNull();
impl<'a> /*trait*/ QSizeF_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QSizeF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6isNullEv()};
    let mut ret = unsafe {_ZNK6QSizeF6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QSizeF QSizeF::boundedTo(const QSizeF & );
impl /*struct*/ QSizeF {
  pub fn boundedTo<RetType, T: QSizeF_boundedTo<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.boundedTo(self);
    // return 1;
  }
}

pub trait QSizeF_boundedTo<RetType> {
  fn boundedTo(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  QSizeF QSizeF::boundedTo(const QSizeF & );
impl<'a> /*trait*/ QSizeF_boundedTo<QSizeF> for (&'a  QSizeF) {
  fn boundedTo(self , rsthis: &mut QSizeF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF9boundedToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QSizeF9boundedToERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  double QSizeF::height();
impl /*struct*/ QSizeF {
  pub fn height<RetType, T: QSizeF_height<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QSizeF_height<RetType> {
  fn height(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  double QSizeF::height();
impl<'a> /*trait*/ QSizeF_height<f64> for () {
  fn height(self , rsthis: &mut QSizeF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6heightEv()};
    let mut ret = unsafe {_ZNK6QSizeF6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QSizeF::transpose();
impl /*struct*/ QSizeF {
  pub fn transpose<RetType, T: QSizeF_transpose<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.transpose(self);
    // return 1;
  }
}

pub trait QSizeF_transpose<RetType> {
  fn transpose(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  void QSizeF::transpose();
impl<'a> /*trait*/ QSizeF_transpose<()> for () {
  fn transpose(self , rsthis: &mut QSizeF) -> () {
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

// proto:  QSizeF QSizeF::expandedTo(const QSizeF & );
impl /*struct*/ QSizeF {
  pub fn expandedTo<RetType, T: QSizeF_expandedTo<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.expandedTo(self);
    // return 1;
  }
}

pub trait QSizeF_expandedTo<RetType> {
  fn expandedTo(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  QSizeF QSizeF::expandedTo(const QSizeF & );
impl<'a> /*trait*/ QSizeF_expandedTo<QSizeF> for (&'a  QSizeF) {
  fn expandedTo(self , rsthis: &mut QSizeF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF10expandedToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QSizeF10expandedToERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QSizeF::isEmpty();
impl /*struct*/ QSizeF {
  pub fn isEmpty<RetType, T: QSizeF_isEmpty<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QSizeF_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  bool QSizeF::isEmpty();
impl<'a> /*trait*/ QSizeF_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QSizeF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF7isEmptyEv()};
    let mut ret = unsafe {_ZNK6QSizeF7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QSizeF::setWidth(qreal w);
impl /*struct*/ QSizeF {
  pub fn setWidth<RetType, T: QSizeF_setWidth<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QSizeF_setWidth<RetType> {
  fn setWidth(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  void QSizeF::setWidth(qreal w);
impl<'a> /*trait*/ QSizeF_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: &mut QSizeF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QSizeF8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QSize QSizeF::toSize();
impl /*struct*/ QSizeF {
  pub fn toSize<RetType, T: QSizeF_toSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.toSize(self);
    // return 1;
  }
}

pub trait QSizeF_toSize<RetType> {
  fn toSize(self , rsthis: &mut QSizeF) -> RetType;
}

// proto:  QSize QSizeF::toSize();
impl<'a> /*trait*/ QSizeF_toSize<QSize> for () {
  fn toSize(self , rsthis: &mut QSizeF) -> QSize {
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

