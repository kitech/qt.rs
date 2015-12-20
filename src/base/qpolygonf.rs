// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrectf::QRectF;
use super::qpolygon::QPolygon;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QRectF QPolygonF::boundingRect();
  fn _ZNK9QPolygonF12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPolygonF QPolygonF::intersected(const QPolygonF & r);
  fn _ZNK9QPolygonF11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::QPolygonF(const QPolygon & a);
  fn _ZN9QPolygonFC1ERK8QPolygon(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPolygonF::QPolygonF(const QRectF & r);
  fn _ZN9QPolygonFC1ERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPolygon QPolygonF::toPolygon();
  fn _ZNK9QPolygonF9toPolygonEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::~QPolygonF();
  fn _ZN9QPolygonFD0Ev(qthis: *mut c_void);
  // proto:  void QPolygonF::QPolygonF(int size);
  fn _ZN9QPolygonFC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  QPolygonF QPolygonF::subtracted(const QPolygonF & r);
  fn _ZNK9QPolygonF10subtractedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::QPolygonF();
  fn _ZN9QPolygonFC1Ev(qthis: *mut c_void);
  // proto:  void QPolygonF::translate(const QPointF & offset);
  fn _ZN9QPolygonF9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPolygonF::swap(QPolygonF & other);
  fn _ZN9QPolygonF4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPolygonF QPolygonF::translated(const QPointF & offset);
  fn _ZNK9QPolygonF10translatedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygonF::translate(qreal dx, qreal dy);
  fn _ZN9QPolygonF9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QPolygonF::QPolygonF(const QPolygonF & a);
  fn _ZN9QPolygonFC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPolygonF QPolygonF::translated(qreal dx, qreal dy);
  fn _ZNK9QPolygonF10translatedEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  bool QPolygonF::isClosed();
  fn _ZNK9QPolygonF8isClosedEv(qthis: *mut c_void) -> c_char;
  // proto:  QPolygonF QPolygonF::united(const QPolygonF & r);
  fn _ZNK9QPolygonF6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QPolygonF)=1
pub struct QPolygonF {
  pub qclsinst: *mut c_void,
}

  // proto:  QRectF QPolygonF::boundingRect();
impl /*struct*/ QPolygonF {
  pub fn boundingRect<RetType, T: QPolygonF_boundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QPolygonF_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QPolygonF) -> RetType;
}

  // proto:  QRectF QPolygonF::boundingRect();
impl<'a> /*trait*/ QPolygonF_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: &mut QPolygonF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF12boundingRectEv()};
    let mut ret = unsafe {_ZNK9QPolygonF12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPolygonF QPolygonF::intersected(const QPolygonF & r);
impl /*struct*/ QPolygonF {
  pub fn intersected<RetType, T: QPolygonF_intersected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QPolygonF_intersected<RetType> {
  fn intersected(self , rsthis: &mut QPolygonF) -> RetType;
}

  // proto:  QPolygonF QPolygonF::intersected(const QPolygonF & r);
impl<'a> /*trait*/ QPolygonF_intersected<QPolygonF> for (QPolygonF) {
  fn intersected(self , rsthis: &mut QPolygonF) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QPolygonF11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF(const QPolygon & a);
impl /*struct*/ QPolygonF {
  pub fn NewQPolygonF<T: QPolygonF_NewQPolygonF>(value: T) -> QPolygonF {
    let rsthis = value.NewQPolygonF();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygonF_NewQPolygonF {
  fn NewQPolygonF(self) -> QPolygonF;
}

  // proto:  void QPolygonF::QPolygonF(const QPolygon & a);
impl<'a> /*trait*/ QPolygonF_NewQPolygonF for (QPolygon) {
  fn NewQPolygonF(self) -> QPolygonF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1ERK8QPolygon()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QPolygonFC1ERK8QPolygon(qthis, arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF(const QRectF & r);
impl<'a> /*trait*/ QPolygonF_NewQPolygonF for (QRectF) {
  fn NewQPolygonF(self) -> QPolygonF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1ERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QPolygonFC1ERK6QRectF(qthis, arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPolygon QPolygonF::toPolygon();
impl /*struct*/ QPolygonF {
  pub fn toPolygon<RetType, T: QPolygonF_toPolygon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toPolygon(self);
    // return 1;
  }
}

pub trait QPolygonF_toPolygon<RetType> {
  fn toPolygon(self , rsthis: &mut QPolygonF) -> RetType;
}

  // proto:  QPolygon QPolygonF::toPolygon();
impl<'a> /*trait*/ QPolygonF_toPolygon<QPolygon> for () {
  fn toPolygon(self , rsthis: &mut QPolygonF) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF9toPolygonEv()};
    let mut ret = unsafe {_ZNK9QPolygonF9toPolygonEv(rsthis.qclsinst)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::~QPolygonF();
impl /*struct*/ QPolygonF {
  pub fn FreeQPolygonF<RetType, T: QPolygonF_FreeQPolygonF<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPolygonF(self);
    // return 1;
  }
}

pub trait QPolygonF_FreeQPolygonF<RetType> {
  fn FreeQPolygonF(self , rsthis: &mut QPolygonF) -> RetType;
}

  // proto:  void QPolygonF::~QPolygonF();
impl<'a> /*trait*/ QPolygonF_FreeQPolygonF<()> for () {
  fn FreeQPolygonF(self , rsthis: &mut QPolygonF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFD0Ev()};
     unsafe {_ZN9QPolygonFD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF(int size);
impl<'a> /*trait*/ QPolygonF_NewQPolygonF for (i32) {
  fn NewQPolygonF(self) -> QPolygonF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN9QPolygonFC1Ei(qthis, arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPolygonF QPolygonF::subtracted(const QPolygonF & r);
impl /*struct*/ QPolygonF {
  pub fn subtracted<RetType, T: QPolygonF_subtracted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.subtracted(self);
    // return 1;
  }
}

pub trait QPolygonF_subtracted<RetType> {
  fn subtracted(self , rsthis: &mut QPolygonF) -> RetType;
}

  // proto:  QPolygonF QPolygonF::subtracted(const QPolygonF & r);
impl<'a> /*trait*/ QPolygonF_subtracted<QPolygonF> for (QPolygonF) {
  fn subtracted(self , rsthis: &mut QPolygonF) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF10subtractedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QPolygonF10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF();
impl<'a> /*trait*/ QPolygonF_NewQPolygonF for () {
  fn NewQPolygonF(self) -> QPolygonF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1Ev()};
    unsafe {_ZN9QPolygonFC1Ev(qthis)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPolygonF::translate(const QPointF & offset);
impl /*struct*/ QPolygonF {
  pub fn translate<RetType, T: QPolygonF_translate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QPolygonF_translate<RetType> {
  fn translate(self , rsthis: &mut QPolygonF) -> RetType;
}

  // proto:  void QPolygonF::translate(const QPointF & offset);
impl<'a> /*trait*/ QPolygonF_translate<()> for (QPointF) {
  fn translate(self , rsthis: &mut QPolygonF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonF9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QPolygonF9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPolygonF::swap(QPolygonF & other);
impl /*struct*/ QPolygonF {
  pub fn swap<RetType, T: QPolygonF_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPolygonF_swap<RetType> {
  fn swap(self , rsthis: &mut QPolygonF) -> RetType;
}

  // proto:  void QPolygonF::swap(QPolygonF & other);
impl<'a> /*trait*/ QPolygonF_swap<()> for (QPolygonF) {
  fn swap(self , rsthis: &mut QPolygonF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonF4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QPolygonF4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QPolygonF::translated(const QPointF & offset);
impl /*struct*/ QPolygonF {
  pub fn translated<RetType, T: QPolygonF_translated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QPolygonF_translated<RetType> {
  fn translated(self , rsthis: &mut QPolygonF) -> RetType;
}

  // proto:  QPolygonF QPolygonF::translated(const QPointF & offset);
impl<'a> /*trait*/ QPolygonF_translated<QPolygonF> for (QPointF) {
  fn translated(self , rsthis: &mut QPolygonF) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QPolygonF10translatedERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPolygonF::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QPolygonF_translate<()> for (f64, f64) {
  fn translate(self , rsthis: &mut QPolygonF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonF9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN9QPolygonF9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPolygonF::QPolygonF(const QPolygonF & a);
impl<'a> /*trait*/ QPolygonF_NewQPolygonF for (QPolygonF) {
  fn NewQPolygonF(self) -> QPolygonF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QPolygonFC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QPolygonFC1ERKS_(qthis, arg0)};
    let rsthis = QPolygonF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPolygonF QPolygonF::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QPolygonF_translated<QPolygonF> for (f64, f64) {
  fn translated(self , rsthis: &mut QPolygonF) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK9QPolygonF10translatedEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPolygonF::isClosed();
impl /*struct*/ QPolygonF {
  pub fn isClosed<RetType, T: QPolygonF_isClosed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isClosed(self);
    // return 1;
  }
}

pub trait QPolygonF_isClosed<RetType> {
  fn isClosed(self , rsthis: &mut QPolygonF) -> RetType;
}

  // proto:  bool QPolygonF::isClosed();
impl<'a> /*trait*/ QPolygonF_isClosed<i8> for () {
  fn isClosed(self , rsthis: &mut QPolygonF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF8isClosedEv()};
    let mut ret = unsafe {_ZNK9QPolygonF8isClosedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QPolygonF QPolygonF::united(const QPolygonF & r);
impl /*struct*/ QPolygonF {
  pub fn united<RetType, T: QPolygonF_united<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.united(self);
    // return 1;
  }
}

pub trait QPolygonF_united<RetType> {
  fn united(self , rsthis: &mut QPolygonF) -> RetType;
}

  // proto:  QPolygonF QPolygonF::united(const QPolygonF & r);
impl<'a> /*trait*/ QPolygonF_united<QPolygonF> for (QPolygonF) {
  fn united(self , rsthis: &mut QPolygonF) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QPolygonF6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QPolygonF6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

