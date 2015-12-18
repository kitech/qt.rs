// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QPolygon::setPoint(int index, int x, int y);
  fn _ZN8QPolygon8setPointEiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) ;
  // proto:  void QPolygon::FreeQPolygon();
  fn _ZN8QPolygonD0Ev(qthis: *mut c_void) ;
  // proto:  void QPolygon::putPoints(int index, int nPoints, const QPolygon & from, int fromIndex);
  fn _ZN8QPolygon9putPointsEiiRKS_i(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: c_int) ;
  // proto:  QPolygon QPolygon::translated(const QPoint & offset);
  fn _ZNK8QPolygon10translatedERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygon QPolygon::subtracted(const QPolygon & r);
  fn _ZNK8QPolygon10subtractedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygon QPolygon::intersected(const QPolygon & r);
  fn _ZNK8QPolygon11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPolygon::setPoint(int index, const QPoint & p);
  fn _ZN8QPolygon8setPointEiRK6QPoint(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QPolygon::translate(int dx, int dy);
  fn _ZN8QPolygon9translateEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QPolygon::putPoints(int index, int nPoints, int firstx, int firsty);
  fn _ZN8QPolygon9putPointsEiiiiz(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QPolygon::setPoints(int nPoints, int firstx, int firsty);
  fn _ZN8QPolygon9setPointsEiiiz(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int) ;
  // proto:  void QPolygon::translate(const QPoint & offset);
  fn _ZN8QPolygon9translateERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPolygon::swap(QPolygon & other);
  fn _ZN8QPolygon4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPolygon::NewQPolygon(const QPolygon & a);
  fn _ZN8QPolygonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPolygon::NewQPolygon(int nPoints, const int * points);
  fn _ZN8QPolygonC1EiPKi(qthis: *mut c_void, arg0: c_int, arg1: *const c_int) ;
  // proto:  QPolygon QPolygon::united(const QPolygon & r);
  fn _ZNK8QPolygon6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPolygon QPolygon::translated(int dx, int dy);
  fn _ZNK8QPolygon10translatedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QPolygon::putPoints(int index, int nPoints, const int * points);
  fn _ZN8QPolygon9putPointsEiiPKi(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *const c_int) ;
  // proto:  void QPolygon::setPoints(int nPoints, const int * points);
  fn _ZN8QPolygon9setPointsEiPKi(qthis: *mut c_void, arg0: c_int, arg1: *const c_int) ;
  // proto:  void QPolygon::NewQPolygon(int size);
  fn _ZN8QPolygonC1Ei(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QPolygon::NewQPolygon();
  fn _ZN8QPolygonC1Ev(qthis: *mut c_void) ;
  // proto:  void QPolygon::NewQPolygon(const QRect & r, bool closed);
  fn _ZN8QPolygonC1ERK5QRectb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) ;
}

// body block begin
// class sizeof(QPolygon)=1
pub struct QPolygon {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPolygon {
  pub fn setPoint<RetType, T: QPolygon_setPoint<RetType>>(&mut self, value: T) -> RetType {
    return value.setPoint(self);
    // return 1;
  }
}

pub trait QPolygon_setPoint<RetType> {
  fn setPoint(self, rsthis: &mut QPolygon) -> RetType;
}

// proto:  void QPolygon::setPoint(int index, int x, int y);
impl<'a> /*trait*/ QPolygon_setPoint<()> for (i32, i32, i32) {
  fn setPoint(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon8setPointEiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPolygon8setPointEiii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn FreeQPolygon<RetType, T: QPolygon_FreeQPolygon<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQPolygon(self);
    // return 1;
  }
}

pub trait QPolygon_FreeQPolygon<RetType> {
  fn FreeQPolygon(self, rsthis: &mut QPolygon) -> RetType;
}

// proto:  void QPolygon::FreeQPolygon();
impl<'a> /*trait*/ QPolygon_FreeQPolygon<()> for () {
  fn FreeQPolygon(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonD0Ev()};
     unsafe {_ZN8QPolygonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn putPoints<RetType, T: QPolygon_putPoints<RetType>>(&mut self, value: T) -> RetType {
    return value.putPoints(self);
    // return 1;
  }
}

pub trait QPolygon_putPoints<RetType> {
  fn putPoints(self, rsthis: &mut QPolygon) -> RetType;
}

// proto:  void QPolygon::putPoints(int index, int nPoints, const QPolygon & from, int fromIndex);
impl<'a> /*trait*/ QPolygon_putPoints<()> for (i32, i32, &'a  QPolygon, i32) {
  fn putPoints(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9putPointsEiiRKS_i()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPolygon9putPointsEiiRKS_i(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn translated<RetType, T: QPolygon_translated<RetType>>(&mut self, value: T) -> RetType {
    return value.translated(self);
    // return 1;
  }
}

pub trait QPolygon_translated<RetType> {
  fn translated(self, rsthis: &mut QPolygon) -> RetType;
}

// proto:  QPolygon QPolygon::translated(const QPoint & offset);
impl<'a> /*trait*/ QPolygon_translated<QPolygon> for (&'a  QPoint) {
  fn translated(self, rsthis: &mut QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPolygon10translatedERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn subtracted<RetType, T: QPolygon_subtracted<RetType>>(&mut self, value: T) -> RetType {
    return value.subtracted(self);
    // return 1;
  }
}

pub trait QPolygon_subtracted<RetType> {
  fn subtracted(self, rsthis: &mut QPolygon) -> RetType;
}

// proto:  QPolygon QPolygon::subtracted(const QPolygon & r);
impl<'a> /*trait*/ QPolygon_subtracted<QPolygon> for (&'a  QPolygon) {
  fn subtracted(self, rsthis: &mut QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10subtractedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPolygon10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn intersected<RetType, T: QPolygon_intersected<RetType>>(&mut self, value: T) -> RetType {
    return value.intersected(self);
    // return 1;
  }
}

pub trait QPolygon_intersected<RetType> {
  fn intersected(self, rsthis: &mut QPolygon) -> RetType;
}

// proto:  QPolygon QPolygon::intersected(const QPolygon & r);
impl<'a> /*trait*/ QPolygon_intersected<QPolygon> for (&'a  QPolygon) {
  fn intersected(self, rsthis: &mut QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPolygon11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPolygon::setPoint(int index, const QPoint & p);
impl<'a> /*trait*/ QPolygon_setPoint<()> for (i32, &'a  QPoint) {
  fn setPoint(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon8setPointEiRK6QPoint()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QPolygon8setPointEiRK6QPoint(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn translate<RetType, T: QPolygon_translate<RetType>>(&mut self, value: T) -> RetType {
    return value.translate(self);
    // return 1;
  }
}

pub trait QPolygon_translate<RetType> {
  fn translate(self, rsthis: &mut QPolygon) -> RetType;
}

// proto:  void QPolygon::translate(int dx, int dy);
impl<'a> /*trait*/ QPolygon_translate<()> for (i32, i32) {
  fn translate(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN8QPolygon9translateEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QPolygon::putPoints(int index, int nPoints, int firstx, int firsty);
impl<'a> /*trait*/ QPolygon_putPoints<()> for (i32, i32, i32, i32) {
  fn putPoints(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9putPointsEiiiiz()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN8QPolygon9putPointsEiiiiz(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn setPoints<RetType, T: QPolygon_setPoints<RetType>>(&mut self, value: T) -> RetType {
    return value.setPoints(self);
    // return 1;
  }
}

pub trait QPolygon_setPoints<RetType> {
  fn setPoints(self, rsthis: &mut QPolygon) -> RetType;
}

// proto:  void QPolygon::setPoints(int nPoints, int firstx, int firsty);
impl<'a> /*trait*/ QPolygon_setPoints<()> for (i32, i32, i32) {
  fn setPoints(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9setPointsEiiiz()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN8QPolygon9setPointsEiiiz(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QPolygon::translate(const QPoint & offset);
impl<'a> /*trait*/ QPolygon_translate<()> for (&'a  QPoint) {
  fn translate(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPolygon9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn swap<RetType, T: QPolygon_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QPolygon_swap<RetType> {
  fn swap(self, rsthis: &mut QPolygon) -> RetType;
}

// proto:  void QPolygon::swap(QPolygon & other);
impl<'a> /*trait*/ QPolygon_swap<()> for (&'a mut QPolygon) {
  fn swap(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QPolygon4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn NewQPolygon<T: QPolygon_NewQPolygon>(value: T) -> QPolygon {
    let rsthis = value.NewQPolygon();
    return rsthis;
    // return 1;
  }
}

pub trait QPolygon_NewQPolygon {
  fn NewQPolygon(self) -> QPolygon;
}

// proto: void QPolygon::NewQPolygon(const QPolygon & a);
impl<'a> /*trait*/ QPolygon_NewQPolygon for (&'a  QPolygon) {
  fn NewQPolygon(self) -> QPolygon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QPolygonC1ERKS_(qthis, arg0)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPolygon::NewQPolygon(int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_NewQPolygon for (i32, &'a  i32) {
  fn NewQPolygon(self) -> QPolygon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1EiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_int;
    unsafe {_ZN8QPolygonC1EiPKi(qthis, arg0, arg1)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPolygon {
  pub fn united<RetType, T: QPolygon_united<RetType>>(&mut self, value: T) -> RetType {
    return value.united(self);
    // return 1;
  }
}

pub trait QPolygon_united<RetType> {
  fn united(self, rsthis: &mut QPolygon) -> RetType;
}

// proto:  QPolygon QPolygon::united(const QPolygon & r);
impl<'a> /*trait*/ QPolygon_united<QPolygon> for (&'a  QPolygon) {
  fn united(self, rsthis: &mut QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QPolygon6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPolygon QPolygon::translated(int dx, int dy);
impl<'a> /*trait*/ QPolygon_translated<QPolygon> for (i32, i32) {
  fn translated(self, rsthis: &mut QPolygon) -> QPolygon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QPolygon10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK8QPolygon10translatedEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPolygon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPolygon::putPoints(int index, int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_putPoints<()> for (i32, i32, &'a  i32) {
  fn putPoints(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9putPointsEiiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *const c_int;
     unsafe {_ZN8QPolygon9putPointsEiiPKi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  void QPolygon::setPoints(int nPoints, const int * points);
impl<'a> /*trait*/ QPolygon_setPoints<()> for (i32, &'a  i32) {
  fn setPoints(self, rsthis: &mut QPolygon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygon9setPointsEiPKi()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *const c_int;
     unsafe {_ZN8QPolygon9setPointsEiPKi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto: void QPolygon::NewQPolygon(int size);
impl<'a> /*trait*/ QPolygon_NewQPolygon for (i32) {
  fn NewQPolygon(self) -> QPolygon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN8QPolygonC1Ei(qthis, arg0)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPolygon::NewQPolygon();
impl<'a> /*trait*/ QPolygon_NewQPolygon for () {
  fn NewQPolygon(self) -> QPolygon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1Ev()};
    unsafe {_ZN8QPolygonC1Ev(qthis)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QPolygon::NewQPolygon(const QRect & r, bool closed);
impl<'a> /*trait*/ QPolygon_NewQPolygon for (&'a  QRect, i8) {
  fn NewQPolygon(self) -> QPolygon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QPolygonC1ERK5QRectb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN8QPolygonC1ERK5QRectb(qthis, arg0, arg1)};
    let rsthis = QPolygon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

