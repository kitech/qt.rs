// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QLine::isNull();
  fn _ZNK5QLine6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  QLine QLine::translated(const QPoint & p);
  fn _ZNK5QLine10translatedERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QLine::setP2(const QPoint & p2);
  fn _ZN5QLine5setP2ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QLine::x2();
  fn _ZNK5QLine2x2Ev(qthis: *mut c_void) -> c_int;
  // proto:  void QLine::NewQLine(const QPoint & pt1, const QPoint & pt2);
  fn _ZN5QLineC1ERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QLine::setP1(const QPoint & p1);
  fn _ZN5QLine5setP1ERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLine::translate(const QPoint & p);
  fn _ZN5QLine9translateERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QLine::dx();
  fn _ZNK5QLine2dxEv(qthis: *mut c_void) -> c_int;
  // proto:  int QLine::y2();
  fn _ZNK5QLine2y2Ev(qthis: *mut c_void) -> c_int;
  // proto:  int QLine::dy();
  fn _ZNK5QLine2dyEv(qthis: *mut c_void) -> c_int;
  // proto:  int QLine::y1();
  fn _ZNK5QLine2y1Ev(qthis: *mut c_void) -> c_int;
  // proto:  QPoint QLine::p1();
  fn _ZNK5QLine2p1Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPoint QLine::p2();
  fn _ZNK5QLine2p2Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLine::NewQLine(int x1, int y1, int x2, int y2);
  fn _ZN5QLineC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QLine::translate(int dx, int dy);
  fn _ZN5QLine9translateEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QLine QLine::translated(int dx, int dy);
  fn _ZNK5QLine10translatedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QLine::setPoints(const QPoint & p1, const QPoint & p2);
  fn _ZN5QLine9setPointsERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QLine::setLine(int x1, int y1, int x2, int y2);
  fn _ZN5QLine7setLineEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  int QLine::x1();
  fn _ZNK5QLine2x1Ev(qthis: *mut c_void) -> c_int;
  // proto:  void QLine::NewQLine();
  fn _ZN5QLineC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QLine)=16
pub struct QLine {
  pub qclsinst: *mut c_void,
}

// proto:  bool QLine::isNull();
impl /*struct*/ QLine {
  pub fn isNull<RetType, T: QLine_isNull<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QLine_isNull<RetType> {
  fn isNull(self , rsthis: &mut QLine) -> RetType;
}

// proto:  bool QLine::isNull();
impl<'a> /*trait*/ QLine_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine6isNullEv()};
    let mut ret = unsafe {_ZNK5QLine6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QLine QLine::translated(const QPoint & p);
impl /*struct*/ QLine {
  pub fn translated<RetType, T: QLine_translated<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QLine_translated<RetType> {
  fn translated(self , rsthis: &mut QLine) -> RetType;
}

// proto:  QLine QLine::translated(const QPoint & p);
impl<'a> /*trait*/ QLine_translated<QLine> for (&'a  QPoint) {
  fn translated(self , rsthis: &mut QLine) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QLine10translatedERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QLine::setP2(const QPoint & p2);
impl /*struct*/ QLine {
  pub fn setP2<RetType, T: QLine_setP2<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setP2(self);
    // return 1;
  }
}

pub trait QLine_setP2<RetType> {
  fn setP2(self , rsthis: &mut QLine) -> RetType;
}

// proto:  void QLine::setP2(const QPoint & p2);
impl<'a> /*trait*/ QLine_setP2<()> for (&'a  QPoint) {
  fn setP2(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine5setP2ERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine5setP2ERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QLine::x2();
impl /*struct*/ QLine {
  pub fn x2<RetType, T: QLine_x2<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.x2(self);
    // return 1;
  }
}

pub trait QLine_x2<RetType> {
  fn x2(self , rsthis: &mut QLine) -> RetType;
}

// proto:  int QLine::x2();
impl<'a> /*trait*/ QLine_x2<i32> for () {
  fn x2(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2x2Ev()};
    let mut ret = unsafe {_ZNK5QLine2x2Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QLine {
  pub fn NewQLine<T: QLine_NewQLine>(value: T) -> QLine {
    let rsthis = value.NewQLine();
    return rsthis;
    // return 1;
  }
}

pub trait QLine_NewQLine {
  fn NewQLine(self) -> QLine;
}

// proto: void QLine::NewQLine(const QPoint & pt1, const QPoint & pt2);
impl<'a> /*trait*/ QLine_NewQLine for (&'a  QPoint, &'a  QPoint) {
  fn NewQLine(self) -> QLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLineC1ERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QLineC1ERK6QPointS2_(qthis, arg0, arg1)};
    let rsthis = QLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QLine::setP1(const QPoint & p1);
impl /*struct*/ QLine {
  pub fn setP1<RetType, T: QLine_setP1<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setP1(self);
    // return 1;
  }
}

pub trait QLine_setP1<RetType> {
  fn setP1(self , rsthis: &mut QLine) -> RetType;
}

// proto:  void QLine::setP1(const QPoint & p1);
impl<'a> /*trait*/ QLine_setP1<()> for (&'a  QPoint) {
  fn setP1(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine5setP1ERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine5setP1ERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QLine::translate(const QPoint & p);
impl /*struct*/ QLine {
  pub fn translate<RetType, T: QLine_translate<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QLine_translate<RetType> {
  fn translate(self , rsthis: &mut QLine) -> RetType;
}

// proto:  void QLine::translate(const QPoint & p);
impl<'a> /*trait*/ QLine_translate<()> for (&'a  QPoint) {
  fn translate(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  int QLine::dx();
impl /*struct*/ QLine {
  pub fn dx<RetType, T: QLine_dx<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.dx(self);
    // return 1;
  }
}

pub trait QLine_dx<RetType> {
  fn dx(self , rsthis: &mut QLine) -> RetType;
}

// proto:  int QLine::dx();
impl<'a> /*trait*/ QLine_dx<i32> for () {
  fn dx(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2dxEv()};
    let mut ret = unsafe {_ZNK5QLine2dxEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QLine::y2();
impl /*struct*/ QLine {
  pub fn y2<RetType, T: QLine_y2<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.y2(self);
    // return 1;
  }
}

pub trait QLine_y2<RetType> {
  fn y2(self , rsthis: &mut QLine) -> RetType;
}

// proto:  int QLine::y2();
impl<'a> /*trait*/ QLine_y2<i32> for () {
  fn y2(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2y2Ev()};
    let mut ret = unsafe {_ZNK5QLine2y2Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QLine::dy();
impl /*struct*/ QLine {
  pub fn dy<RetType, T: QLine_dy<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.dy(self);
    // return 1;
  }
}

pub trait QLine_dy<RetType> {
  fn dy(self , rsthis: &mut QLine) -> RetType;
}

// proto:  int QLine::dy();
impl<'a> /*trait*/ QLine_dy<i32> for () {
  fn dy(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2dyEv()};
    let mut ret = unsafe {_ZNK5QLine2dyEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QLine::y1();
impl /*struct*/ QLine {
  pub fn y1<RetType, T: QLine_y1<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.y1(self);
    // return 1;
  }
}

pub trait QLine_y1<RetType> {
  fn y1(self , rsthis: &mut QLine) -> RetType;
}

// proto:  int QLine::y1();
impl<'a> /*trait*/ QLine_y1<i32> for () {
  fn y1(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2y1Ev()};
    let mut ret = unsafe {_ZNK5QLine2y1Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  QPoint QLine::p1();
impl /*struct*/ QLine {
  pub fn p1<RetType, T: QLine_p1<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.p1(self);
    // return 1;
  }
}

pub trait QLine_p1<RetType> {
  fn p1(self , rsthis: &mut QLine) -> RetType;
}

// proto:  QPoint QLine::p1();
impl<'a> /*trait*/ QLine_p1<QPoint> for () {
  fn p1(self , rsthis: &mut QLine) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2p1Ev()};
    let mut ret = unsafe {_ZNK5QLine2p1Ev(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QPoint QLine::p2();
impl /*struct*/ QLine {
  pub fn p2<RetType, T: QLine_p2<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.p2(self);
    // return 1;
  }
}

pub trait QLine_p2<RetType> {
  fn p2(self , rsthis: &mut QLine) -> RetType;
}

// proto:  QPoint QLine::p2();
impl<'a> /*trait*/ QLine_p2<QPoint> for () {
  fn p2(self , rsthis: &mut QLine) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2p2Ev()};
    let mut ret = unsafe {_ZNK5QLine2p2Ev(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QLine::NewQLine(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QLine_NewQLine for (i32, i32, i32, i32) {
  fn NewQLine(self) -> QLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLineC1Eiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QLineC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QLine::translate(int dx, int dy);
impl<'a> /*trait*/ QLine_translate<()> for (i32, i32) {
  fn translate(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN5QLine9translateEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QLine QLine::translated(int dx, int dy);
impl<'a> /*trait*/ QLine_translated<QLine> for (i32, i32) {
  fn translated(self , rsthis: &mut QLine) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK5QLine10translatedEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QLine::setPoints(const QPoint & p1, const QPoint & p2);
impl /*struct*/ QLine {
  pub fn setPoints<RetType, T: QLine_setPoints<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setPoints(self);
    // return 1;
  }
}

pub trait QLine_setPoints<RetType> {
  fn setPoints(self , rsthis: &mut QLine) -> RetType;
}

// proto:  void QLine::setPoints(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QLine_setPoints<()> for (&'a  QPoint, &'a  QPoint) {
  fn setPoints(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine9setPointsERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN5QLine9setPointsERK6QPointS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QLine::setLine(int x1, int y1, int x2, int y2);
impl /*struct*/ QLine {
  pub fn setLine<RetType, T: QLine_setLine<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setLine(self);
    // return 1;
  }
}

pub trait QLine_setLine<RetType> {
  fn setLine(self , rsthis: &mut QLine) -> RetType;
}

// proto:  void QLine::setLine(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QLine_setLine<()> for (i32, i32, i32, i32) {
  fn setLine(self , rsthis: &mut QLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLine7setLineEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN5QLine7setLineEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  int QLine::x1();
impl /*struct*/ QLine {
  pub fn x1<RetType, T: QLine_x1<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.x1(self);
    // return 1;
  }
}

pub trait QLine_x1<RetType> {
  fn x1(self , rsthis: &mut QLine) -> RetType;
}

// proto:  int QLine::x1();
impl<'a> /*trait*/ QLine_x1<i32> for () {
  fn x1(self , rsthis: &mut QLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QLine2x1Ev()};
    let mut ret = unsafe {_ZNK5QLine2x1Ev(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QLine::NewQLine();
impl<'a> /*trait*/ QLine_NewQLine for () {
  fn NewQLine(self) -> QLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QLineC1Ev()};
    unsafe {_ZN5QLineC1Ev(qthis)};
    let rsthis = QLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

