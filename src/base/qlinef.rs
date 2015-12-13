// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qline::QLine;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN6QLineF9translateEdd(arg0: c_double, arg1: c_double) -> i32;
  fn _ZN6QLineF9setPointsERK7QPointFS2_(arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN6QLineF5setP2ERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QLineF10translatedEdd(arg0: c_double, arg1: c_double) -> i32;
  fn _ZN6QLineF9setLengthEd(arg0: c_double) -> i32;
  fn _ZNK6QLineF2x1Ev() -> i32;
  fn _ZNK6QLineF5angleEv() -> i32;
  fn _ZN6QLineFC1ERK7QPointFS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK6QLineF6lengthEv() -> i32;
  fn _ZN6QLineFC1ERK5QLine(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN6QLineF8setAngleEd(arg0: c_double) -> i32;
  fn _ZNK6QLineF2x2Ev() -> i32;
  fn _ZN6QLineF9translateERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QLineF2dxEv() -> i32;
  fn _ZN6QLineFC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK6QLineF2p1Ev() -> i32;
  fn _ZNK6QLineF12normalVectorEv() -> i32;
  fn _ZNK6QLineF6toLineEv() -> i32;
  fn _ZNK6QLineF7pointAtEd(arg0: c_double) -> i32;
  fn _ZNK6QLineF2p2Ev() -> i32;
  fn _ZNK6QLineF2y2Ev() -> i32;
  fn _ZN6QLineFC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  fn _ZNK6QLineF2dyEv() -> i32;
  fn _ZNK6QLineF10unitVectorEv() -> i32;
  fn _ZNK6QLineF6isNullEv() -> i32;
  fn _ZNK6QLineF2y1Ev() -> i32;
  fn _ZNK6QLineF7angleToERKS_(arg0: *const c_void) -> i32;
  fn _ZNK6QLineF10translatedERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZN6QLineF7setLineEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  fn _ZN6QLineF9fromPolarEdd(arg0: c_double, arg1: c_double) -> i32;
  fn _ZN6QLineF5setP1ERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QLineF5angleERKS_(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QLineF)=32
pub struct QLineF {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLineF {
  pub fn translate<T: QLineF_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QLineF_translate {
  fn translate(self, this: &mut QLineF) -> i32;
}

// proto: void QLineF::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QLineF_translate for (f64, f64) {
  fn translate(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN6QLineF9translateEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setPoints<T: QLineF_setPoints>(&mut self, value: T) -> i32 {
    value.setPoints(self);
    return 1;
  }
}

pub trait QLineF_setPoints {
  fn setPoints(self, this: &mut QLineF) -> i32;
}

// proto: void QLineF::setPoints(const QPointF & p1, const QPointF & p2);
impl<'a> /*trait*/ QLineF_setPoints for (&'a  QPointF, &'a  QPointF) {
  fn setPoints(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9setPointsERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN6QLineF9setPointsERK7QPointFS2_(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setP2<T: QLineF_setP2>(&mut self, value: T) -> i32 {
    value.setP2(self);
    return 1;
  }
}

pub trait QLineF_setP2 {
  fn setP2(self, this: &mut QLineF) -> i32;
}

// proto: void QLineF::setP2(const QPointF & p2);
impl<'a> /*trait*/ QLineF_setP2 for (&'a  QPointF) {
  fn setP2(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF5setP2ERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QLineF5setP2ERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn translated<T: QLineF_translated>(&mut self, value: T) -> i32 {
    value.translated(self);
    return 1;
  }
}

pub trait QLineF_translated {
  fn translated(self, this: &mut QLineF) -> i32;
}

// proto: QLineF QLineF::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QLineF_translated for (f64, f64) {
  fn translated(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZNK6QLineF10translatedEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setLength<T: QLineF_setLength>(&mut self, value: T) -> i32 {
    value.setLength(self);
    return 1;
  }
}

pub trait QLineF_setLength {
  fn setLength(self, this: &mut QLineF) -> i32;
}

// proto: void QLineF::setLength(qreal len);
impl<'a> /*trait*/ QLineF_setLength for (f64) {
  fn setLength(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9setLengthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QLineF9setLengthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn x1<T: QLineF_x1>(&mut self, value: T) -> i32 {
    value.x1(self);
    return 1;
  }
}

pub trait QLineF_x1 {
  fn x1(self, this: &mut QLineF) -> i32;
}

// proto: double QLineF::x1();
impl<'a> /*trait*/ QLineF_x1 for () {
  fn x1(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2x1Ev()};
    unsafe {_ZNK6QLineF2x1Ev()};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn angle<T: QLineF_angle>(&mut self, value: T) -> i32 {
    value.angle(self);
    return 1;
  }
}

pub trait QLineF_angle {
  fn angle(self, this: &mut QLineF) -> i32;
}

// proto: double QLineF::angle();
impl<'a> /*trait*/ QLineF_angle for () {
  fn angle(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF5angleEv()};
    unsafe {_ZNK6QLineF5angleEv()};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn NewQLineF<T: QLineF_NewQLineF>(value: T) -> QLineF {
    let rsthis = value.NewQLineF();
    return rsthis;
    // return 1;
  }
}

pub trait QLineF_NewQLineF {
  fn NewQLineF(self) -> QLineF;
}

// proto: void QLineF::NewQLineF(const QPointF & pt1, const QPointF & pt2);
impl<'a> /*trait*/ QLineF_NewQLineF for (&'a  QPointF, &'a  QPointF) {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1ERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN6QLineFC1ERK7QPointFS2_(qthis, arg0, arg1)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn length<T: QLineF_length>(&mut self, value: T) -> i32 {
    value.length(self);
    return 1;
  }
}

pub trait QLineF_length {
  fn length(self, this: &mut QLineF) -> i32;
}

// proto: double QLineF::length();
impl<'a> /*trait*/ QLineF_length for () {
  fn length(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF6lengthEv()};
    unsafe {_ZNK6QLineF6lengthEv()};
    return 1;
  }
}

// proto: void QLineF::NewQLineF(const QLine & line);
impl<'a> /*trait*/ QLineF_NewQLineF for (&'a  QLine) {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1ERK5QLine()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QLineFC1ERK5QLine(qthis, arg0)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setAngle<T: QLineF_setAngle>(&mut self, value: T) -> i32 {
    value.setAngle(self);
    return 1;
  }
}

pub trait QLineF_setAngle {
  fn setAngle(self, this: &mut QLineF) -> i32;
}

// proto: void QLineF::setAngle(qreal angle);
impl<'a> /*trait*/ QLineF_setAngle for (f64) {
  fn setAngle(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF8setAngleEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QLineF8setAngleEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn x2<T: QLineF_x2>(&mut self, value: T) -> i32 {
    value.x2(self);
    return 1;
  }
}

pub trait QLineF_x2 {
  fn x2(self, this: &mut QLineF) -> i32;
}

// proto: double QLineF::x2();
impl<'a> /*trait*/ QLineF_x2 for () {
  fn x2(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2x2Ev()};
    unsafe {_ZNK6QLineF2x2Ev()};
    return 1;
  }
}

// proto: void QLineF::translate(const QPointF & p);
impl<'a> /*trait*/ QLineF_translate for (&'a  QPointF) {
  fn translate(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QLineF9translateERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn dx<T: QLineF_dx>(&mut self, value: T) -> i32 {
    value.dx(self);
    return 1;
  }
}

pub trait QLineF_dx {
  fn dx(self, this: &mut QLineF) -> i32;
}

// proto: double QLineF::dx();
impl<'a> /*trait*/ QLineF_dx for () {
  fn dx(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2dxEv()};
    unsafe {_ZNK6QLineF2dxEv()};
    return 1;
  }
}

// proto: void QLineF::NewQLineF();
impl<'a> /*trait*/ QLineF_NewQLineF for () {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1Ev()};
    unsafe {_ZN6QLineFC1Ev(qthis)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn p1<T: QLineF_p1>(&mut self, value: T) -> i32 {
    value.p1(self);
    return 1;
  }
}

pub trait QLineF_p1 {
  fn p1(self, this: &mut QLineF) -> i32;
}

// proto: QPointF QLineF::p1();
impl<'a> /*trait*/ QLineF_p1 for () {
  fn p1(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2p1Ev()};
    unsafe {_ZNK6QLineF2p1Ev()};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn normalVector<T: QLineF_normalVector>(&mut self, value: T) -> i32 {
    value.normalVector(self);
    return 1;
  }
}

pub trait QLineF_normalVector {
  fn normalVector(self, this: &mut QLineF) -> i32;
}

// proto: QLineF QLineF::normalVector();
impl<'a> /*trait*/ QLineF_normalVector for () {
  fn normalVector(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF12normalVectorEv()};
    unsafe {_ZNK6QLineF12normalVectorEv()};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn toLine<T: QLineF_toLine>(&mut self, value: T) -> i32 {
    value.toLine(self);
    return 1;
  }
}

pub trait QLineF_toLine {
  fn toLine(self, this: &mut QLineF) -> i32;
}

// proto: QLine QLineF::toLine();
impl<'a> /*trait*/ QLineF_toLine for () {
  fn toLine(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF6toLineEv()};
    unsafe {_ZNK6QLineF6toLineEv()};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn pointAt<T: QLineF_pointAt>(&mut self, value: T) -> i32 {
    value.pointAt(self);
    return 1;
  }
}

pub trait QLineF_pointAt {
  fn pointAt(self, this: &mut QLineF) -> i32;
}

// proto: QPointF QLineF::pointAt(qreal t);
impl<'a> /*trait*/ QLineF_pointAt for (f64) {
  fn pointAt(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF7pointAtEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK6QLineF7pointAtEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn p2<T: QLineF_p2>(&mut self, value: T) -> i32 {
    value.p2(self);
    return 1;
  }
}

pub trait QLineF_p2 {
  fn p2(self, this: &mut QLineF) -> i32;
}

// proto: QPointF QLineF::p2();
impl<'a> /*trait*/ QLineF_p2 for () {
  fn p2(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2p2Ev()};
    unsafe {_ZNK6QLineF2p2Ev()};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn y2<T: QLineF_y2>(&mut self, value: T) -> i32 {
    value.y2(self);
    return 1;
  }
}

pub trait QLineF_y2 {
  fn y2(self, this: &mut QLineF) -> i32;
}

// proto: double QLineF::y2();
impl<'a> /*trait*/ QLineF_y2 for () {
  fn y2(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2y2Ev()};
    unsafe {_ZNK6QLineF2y2Ev()};
    return 1;
  }
}

// proto: void QLineF::NewQLineF(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QLineF_NewQLineF for (f64, f64, f64, f64) {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1Edddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QLineFC1Edddd(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn dy<T: QLineF_dy>(&mut self, value: T) -> i32 {
    value.dy(self);
    return 1;
  }
}

pub trait QLineF_dy {
  fn dy(self, this: &mut QLineF) -> i32;
}

// proto: double QLineF::dy();
impl<'a> /*trait*/ QLineF_dy for () {
  fn dy(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2dyEv()};
    unsafe {_ZNK6QLineF2dyEv()};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn unitVector<T: QLineF_unitVector>(&mut self, value: T) -> i32 {
    value.unitVector(self);
    return 1;
  }
}

pub trait QLineF_unitVector {
  fn unitVector(self, this: &mut QLineF) -> i32;
}

// proto: QLineF QLineF::unitVector();
impl<'a> /*trait*/ QLineF_unitVector for () {
  fn unitVector(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF10unitVectorEv()};
    unsafe {_ZNK6QLineF10unitVectorEv()};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn isNull<T: QLineF_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QLineF_isNull {
  fn isNull(self, this: &mut QLineF) -> i32;
}

// proto: bool QLineF::isNull();
impl<'a> /*trait*/ QLineF_isNull for () {
  fn isNull(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF6isNullEv()};
    unsafe {_ZNK6QLineF6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn y1<T: QLineF_y1>(&mut self, value: T) -> i32 {
    value.y1(self);
    return 1;
  }
}

pub trait QLineF_y1 {
  fn y1(self, this: &mut QLineF) -> i32;
}

// proto: double QLineF::y1();
impl<'a> /*trait*/ QLineF_y1 for () {
  fn y1(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2y1Ev()};
    unsafe {_ZNK6QLineF2y1Ev()};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn angleTo<T: QLineF_angleTo>(&mut self, value: T) -> i32 {
    value.angleTo(self);
    return 1;
  }
}

pub trait QLineF_angleTo {
  fn angleTo(self, this: &mut QLineF) -> i32;
}

// proto: double QLineF::angleTo(const QLineF & l);
impl<'a> /*trait*/ QLineF_angleTo for (&'a  QLineF) {
  fn angleTo(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF7angleToERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QLineF7angleToERKS_(arg0)};
    return 1;
  }
}

// proto: QLineF QLineF::translated(const QPointF & p);
impl<'a> /*trait*/ QLineF_translated for (&'a  QPointF) {
  fn translated(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QLineF10translatedERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setLine<T: QLineF_setLine>(&mut self, value: T) -> i32 {
    value.setLine(self);
    return 1;
  }
}

pub trait QLineF_setLine {
  fn setLine(self, this: &mut QLineF) -> i32;
}

// proto: void QLineF::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QLineF_setLine for (f64, f64, f64, f64) {
  fn setLine(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF7setLineEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QLineF7setLineEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn fromPolar<T: QLineF_fromPolar>(&mut self, value: T) -> i32 {
    value.fromPolar(self);
    return 1;
  }
}

pub trait QLineF_fromPolar {
  fn fromPolar(self, this: &mut QLineF) -> i32;
}

// proto: QLineF QLineF::fromPolar(qreal length, qreal angle);
impl<'a> /*trait*/ QLineF_fromPolar for (f64, f64) {
  fn fromPolar(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9fromPolarEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN6QLineF9fromPolarEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setP1<T: QLineF_setP1>(&mut self, value: T) -> i32 {
    value.setP1(self);
    return 1;
  }
}

pub trait QLineF_setP1 {
  fn setP1(self, this: &mut QLineF) -> i32;
}

// proto: void QLineF::setP1(const QPointF & p1);
impl<'a> /*trait*/ QLineF_setP1 for (&'a  QPointF) {
  fn setP1(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF5setP1ERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QLineF5setP1ERK7QPointF(arg0)};
    return 1;
  }
}

// proto: double QLineF::angle(const QLineF & l);
impl<'a> /*trait*/ QLineF_angle for (&'a  QLineF) {
  fn angle(self, this: &mut QLineF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF5angleERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QLineF5angleERKS_(arg0)};
    return 1;
  }
}

