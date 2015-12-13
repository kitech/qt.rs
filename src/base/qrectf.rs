// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qsizef::QSizeF;
use super::qmarginsf::QMarginsF;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZN6QRectFC1Ev(qthis: *mut c_void) -> i32;
  fn _ZN6QRectF15moveBottomRightERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZN6QRectF6moveToEdd(arg0: c_double, arg1: c_double) -> i32;
  fn _ZNK6QRectF3topEv() -> i32;
  fn _ZNK6QRectF10bottomLeftEv() -> i32;
  fn _ZN6QRectF9setHeightEd(arg0: c_double) -> i32;
  fn _ZN6QRectF7setSizeERK6QSizeF(arg0: *const c_void) -> i32;
  fn _ZN6QRectFC1ERK7QPointFS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN6QRectF6moveToERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF13toAlignedRectEv() -> i32;
  fn _ZN6QRectF8setRightEd(arg0: c_double) -> i32;
  fn _ZN6QRectF13setBottomLeftERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF8topRightEv() -> i32;
  fn _ZNK6QRectF4sizeEv() -> i32;
  fn _ZN6QRectF6adjustEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  fn _ZN6QRectF9moveRightEd(arg0: c_double) -> i32;
  fn _ZNK6QRectF1yEv() -> i32;
  fn _ZNK6QRectF11bottomRightEv() -> i32;
  fn _ZN6QRectF9setBottomEd(arg0: c_double) -> i32;
  fn _ZN6QRectF14moveBottomLeftERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZN6QRectF10moveBottomEd(arg0: c_double) -> i32;
  fn _ZNK6QRectF7getRectEPdS0_S0_S0_(arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  fn _ZNK6QRectF1xEv() -> i32;
  fn _ZNK6QRectF6bottomEv() -> i32;
  fn _ZNK6QRectF6isNullEv() -> i32;
  fn _ZN6QRectFC1ERK7QPointFRK6QSizeF(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZN6QRectF8setWidthEd(arg0: c_double) -> i32;
  fn _ZNK6QRectF6heightEv() -> i32;
  fn _ZN6QRectF9translateERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZN6QRectF10moveCenterERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF8containsERKS_(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF14marginsRemovedERK9QMarginsF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF8containsEdd(arg0: c_double, arg1: c_double) -> i32;
  fn _ZN6QRectF4setXEd(arg0: c_double) -> i32;
  fn _ZN6QRectF7setRectEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  fn _ZNK6QRectF6centerEv() -> i32;
  fn _ZN6QRectF7setLeftEd(arg0: c_double) -> i32;
  fn _ZNK6QRectF11intersectedERKS_(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF7topLeftEv() -> i32;
  fn _ZNK6QRectF4leftEv() -> i32;
  fn _ZN6QRectF4setYEd(arg0: c_double) -> i32;
  fn _ZN6QRectF11moveTopLeftERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF5widthEv() -> i32;
  fn _ZN6QRectF6setTopEd(arg0: c_double) -> i32;
  fn _ZNK6QRectF7isValidEv() -> i32;
  fn _ZN6QRectF9translateEdd(arg0: c_double, arg1: c_double) -> i32;
  fn _ZN6QRectFC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  fn _ZNK6QRectF6toRectEv() -> i32;
  fn _ZN6QRectF8moveLeftEd(arg0: c_double) -> i32;
  fn _ZN6QRectF10setTopLeftERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZN6QRectF14setBottomRightERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF12marginsAddedERK9QMarginsF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF10translatedERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF10normalizedEv() -> i32;
  fn _ZNK6QRectF9getCoordsEPdS0_S0_S0_(arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) -> i32;
  fn _ZN6QRectF11setTopRightERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF8containsERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF10intersectsERKS_(arg0: *const c_void) -> i32;
  fn _ZN6QRectF7moveTopEd(arg0: c_double) -> i32;
  fn _ZN6QRectF9setCoordsEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
  fn _ZNK6QRectF10translatedEdd(arg0: c_double, arg1: c_double) -> i32;
  fn _ZNK6QRectF7isEmptyEv() -> i32;
  fn _ZN6QRectF12moveTopRightERK7QPointF(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF6unitedERKS_(arg0: *const c_void) -> i32;
  fn _ZNK6QRectF5rightEv() -> i32;
  fn _ZN6QRectFC1ERK5QRect(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZNK6QRectF8adjustedEdddd(arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> i32;
}

// body block begin
// class sizeof(QRectF)=32
pub struct QRectF {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRectF {
  pub fn NewQRectF<T: QRectF_NewQRectF>(value: T) -> QRectF {
    let rsthis = value.NewQRectF();
    return rsthis;
    // return 1;
  }
}

pub trait QRectF_NewQRectF {
  fn NewQRectF(self) -> QRectF;
}

// proto: void QRectF::NewQRectF();
impl<'a> /*trait*/ QRectF_NewQRectF for () {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1Ev()};
    unsafe {_ZN6QRectFC1Ev(qthis)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveBottomRight<T: QRectF_moveBottomRight>(&mut self, value: T) -> i32 {
    value.moveBottomRight(self);
    return 1;
  }
}

pub trait QRectF_moveBottomRight {
  fn moveBottomRight(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::moveBottomRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveBottomRight for (&'a  QPointF) {
  fn moveBottomRight(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF15moveBottomRightERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF15moveBottomRightERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTo<T: QRectF_moveTo>(&mut self, value: T) -> i32 {
    value.moveTo(self);
    return 1;
  }
}

pub trait QRectF_moveTo {
  fn moveTo(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::moveTo(qreal x, qreal y);
impl<'a> /*trait*/ QRectF_moveTo for (f64, f64) {
  fn moveTo(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6moveToEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN6QRectF6moveToEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn top<T: QRectF_top>(&mut self, value: T) -> i32 {
    value.top(self);
    return 1;
  }
}

pub trait QRectF_top {
  fn top(self, this: &mut QRectF) -> i32;
}

// proto: double QRectF::top();
impl<'a> /*trait*/ QRectF_top for () {
  fn top(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF3topEv()};
    unsafe {_ZNK6QRectF3topEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn bottomLeft<T: QRectF_bottomLeft>(&mut self, value: T) -> i32 {
    value.bottomLeft(self);
    return 1;
  }
}

pub trait QRectF_bottomLeft {
  fn bottomLeft(self, this: &mut QRectF) -> i32;
}

// proto: QPointF QRectF::bottomLeft();
impl<'a> /*trait*/ QRectF_bottomLeft for () {
  fn bottomLeft(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10bottomLeftEv()};
    unsafe {_ZNK6QRectF10bottomLeftEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setHeight<T: QRectF_setHeight>(&mut self, value: T) -> i32 {
    value.setHeight(self);
    return 1;
  }
}

pub trait QRectF_setHeight {
  fn setHeight(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setHeight(qreal h);
impl<'a> /*trait*/ QRectF_setHeight for (f64) {
  fn setHeight(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setHeightEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF9setHeightEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setSize<T: QRectF_setSize>(&mut self, value: T) -> i32 {
    value.setSize(self);
    return 1;
  }
}

pub trait QRectF_setSize {
  fn setSize(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setSize(const QSizeF & s);
impl<'a> /*trait*/ QRectF_setSize for (&'a  QSizeF) {
  fn setSize(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7setSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF7setSizeERK6QSizeF(arg0)};
    return 1;
  }
}

// proto: void QRectF::NewQRectF(const QPointF & topleft, const QPointF & bottomRight);
impl<'a> /*trait*/ QRectF_NewQRectF for (&'a  QPointF, &'a  QPointF) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1ERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN6QRectFC1ERK7QPointFS2_(qthis, arg0, arg1)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QRectF::moveTo(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTo for (&'a  QPointF) {
  fn moveTo(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6moveToERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF6moveToERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn toAlignedRect<T: QRectF_toAlignedRect>(&mut self, value: T) -> i32 {
    value.toAlignedRect(self);
    return 1;
  }
}

pub trait QRectF_toAlignedRect {
  fn toAlignedRect(self, this: &mut QRectF) -> i32;
}

// proto: QRect QRectF::toAlignedRect();
impl<'a> /*trait*/ QRectF_toAlignedRect for () {
  fn toAlignedRect(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF13toAlignedRectEv()};
    unsafe {_ZNK6QRectF13toAlignedRectEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setRight<T: QRectF_setRight>(&mut self, value: T) -> i32 {
    value.setRight(self);
    return 1;
  }
}

pub trait QRectF_setRight {
  fn setRight(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setRight(qreal pos);
impl<'a> /*trait*/ QRectF_setRight for (f64) {
  fn setRight(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8setRightEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF8setRightEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setBottomLeft<T: QRectF_setBottomLeft>(&mut self, value: T) -> i32 {
    value.setBottomLeft(self);
    return 1;
  }
}

pub trait QRectF_setBottomLeft {
  fn setBottomLeft(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setBottomLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_setBottomLeft for (&'a  QPointF) {
  fn setBottomLeft(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF13setBottomLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF13setBottomLeftERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn topRight<T: QRectF_topRight>(&mut self, value: T) -> i32 {
    value.topRight(self);
    return 1;
  }
}

pub trait QRectF_topRight {
  fn topRight(self, this: &mut QRectF) -> i32;
}

// proto: QPointF QRectF::topRight();
impl<'a> /*trait*/ QRectF_topRight for () {
  fn topRight(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8topRightEv()};
    unsafe {_ZNK6QRectF8topRightEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn size<T: QRectF_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QRectF_size {
  fn size(self, this: &mut QRectF) -> i32;
}

// proto: QSizeF QRectF::size();
impl<'a> /*trait*/ QRectF_size for () {
  fn size(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF4sizeEv()};
    unsafe {_ZNK6QRectF4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn adjust<T: QRectF_adjust>(&mut self, value: T) -> i32 {
    value.adjust(self);
    return 1;
  }
}

pub trait QRectF_adjust {
  fn adjust(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::adjust(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_adjust for (f64, f64, f64, f64) {
  fn adjust(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6adjustEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QRectF6adjustEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveRight<T: QRectF_moveRight>(&mut self, value: T) -> i32 {
    value.moveRight(self);
    return 1;
  }
}

pub trait QRectF_moveRight {
  fn moveRight(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::moveRight(qreal pos);
impl<'a> /*trait*/ QRectF_moveRight for (f64) {
  fn moveRight(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9moveRightEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF9moveRightEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn y<T: QRectF_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QRectF_y {
  fn y(self, this: &mut QRectF) -> i32;
}

// proto: double QRectF::y();
impl<'a> /*trait*/ QRectF_y for () {
  fn y(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF1yEv()};
    unsafe {_ZNK6QRectF1yEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn bottomRight<T: QRectF_bottomRight>(&mut self, value: T) -> i32 {
    value.bottomRight(self);
    return 1;
  }
}

pub trait QRectF_bottomRight {
  fn bottomRight(self, this: &mut QRectF) -> i32;
}

// proto: QPointF QRectF::bottomRight();
impl<'a> /*trait*/ QRectF_bottomRight for () {
  fn bottomRight(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF11bottomRightEv()};
    unsafe {_ZNK6QRectF11bottomRightEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setBottom<T: QRectF_setBottom>(&mut self, value: T) -> i32 {
    value.setBottom(self);
    return 1;
  }
}

pub trait QRectF_setBottom {
  fn setBottom(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setBottom(qreal pos);
impl<'a> /*trait*/ QRectF_setBottom for (f64) {
  fn setBottom(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setBottomEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF9setBottomEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveBottomLeft<T: QRectF_moveBottomLeft>(&mut self, value: T) -> i32 {
    value.moveBottomLeft(self);
    return 1;
  }
}

pub trait QRectF_moveBottomLeft {
  fn moveBottomLeft(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::moveBottomLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveBottomLeft for (&'a  QPointF) {
  fn moveBottomLeft(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF14moveBottomLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF14moveBottomLeftERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveBottom<T: QRectF_moveBottom>(&mut self, value: T) -> i32 {
    value.moveBottom(self);
    return 1;
  }
}

pub trait QRectF_moveBottom {
  fn moveBottom(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::moveBottom(qreal pos);
impl<'a> /*trait*/ QRectF_moveBottom for (f64) {
  fn moveBottom(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10moveBottomEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF10moveBottomEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn getRect<T: QRectF_getRect>(&mut self, value: T) -> i32 {
    value.getRect(self);
    return 1;
  }
}

pub trait QRectF_getRect {
  fn getRect(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::getRect(qreal * x, qreal * y, qreal * w, qreal * h);
impl<'a> /*trait*/ QRectF_getRect for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getRect(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7getRectEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK6QRectF7getRectEPdS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn x<T: QRectF_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QRectF_x {
  fn x(self, this: &mut QRectF) -> i32;
}

// proto: double QRectF::x();
impl<'a> /*trait*/ QRectF_x for () {
  fn x(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF1xEv()};
    unsafe {_ZNK6QRectF1xEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn bottom<T: QRectF_bottom>(&mut self, value: T) -> i32 {
    value.bottom(self);
    return 1;
  }
}

pub trait QRectF_bottom {
  fn bottom(self, this: &mut QRectF) -> i32;
}

// proto: double QRectF::bottom();
impl<'a> /*trait*/ QRectF_bottom for () {
  fn bottom(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6bottomEv()};
    unsafe {_ZNK6QRectF6bottomEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn isNull<T: QRectF_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QRectF_isNull {
  fn isNull(self, this: &mut QRectF) -> i32;
}

// proto: bool QRectF::isNull();
impl<'a> /*trait*/ QRectF_isNull for () {
  fn isNull(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6isNullEv()};
    unsafe {_ZNK6QRectF6isNullEv()};
    return 1;
  }
}

// proto: void QRectF::NewQRectF(const QPointF & topleft, const QSizeF & size);
impl<'a> /*trait*/ QRectF_NewQRectF for (&'a  QPointF, &'a  QSizeF) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1ERK7QPointFRK6QSizeF()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN6QRectFC1ERK7QPointFRK6QSizeF(qthis, arg0, arg1)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setWidth<T: QRectF_setWidth>(&mut self, value: T) -> i32 {
    value.setWidth(self);
    return 1;
  }
}

pub trait QRectF_setWidth {
  fn setWidth(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setWidth(qreal w);
impl<'a> /*trait*/ QRectF_setWidth for (f64) {
  fn setWidth(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8setWidthEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF8setWidthEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn height<T: QRectF_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QRectF_height {
  fn height(self, this: &mut QRectF) -> i32;
}

// proto: double QRectF::height();
impl<'a> /*trait*/ QRectF_height for () {
  fn height(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6heightEv()};
    unsafe {_ZNK6QRectF6heightEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn translate<T: QRectF_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QRectF_translate {
  fn translate(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::translate(const QPointF & p);
impl<'a> /*trait*/ QRectF_translate for (&'a  QPointF) {
  fn translate(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF9translateERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveCenter<T: QRectF_moveCenter>(&mut self, value: T) -> i32 {
    value.moveCenter(self);
    return 1;
  }
}

pub trait QRectF_moveCenter {
  fn moveCenter(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::moveCenter(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveCenter for (&'a  QPointF) {
  fn moveCenter(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10moveCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF10moveCenterERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn contains<T: QRectF_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QRectF_contains {
  fn contains(self, this: &mut QRectF) -> i32;
}

// proto: bool QRectF::contains(const QRectF & r);
impl<'a> /*trait*/ QRectF_contains for (&'a  QRectF) {
  fn contains(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8containsERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QRectF8containsERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn marginsRemoved<T: QRectF_marginsRemoved>(&mut self, value: T) -> i32 {
    value.marginsRemoved(self);
    return 1;
  }
}

pub trait QRectF_marginsRemoved {
  fn marginsRemoved(self, this: &mut QRectF) -> i32;
}

// proto: QRectF QRectF::marginsRemoved(const QMarginsF & margins);
impl<'a> /*trait*/ QRectF_marginsRemoved for (&'a  QMarginsF) {
  fn marginsRemoved(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF14marginsRemovedERK9QMarginsF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QRectF14marginsRemovedERK9QMarginsF(arg0)};
    return 1;
  }
}

// proto: bool QRectF::contains(qreal x, qreal y);
impl<'a> /*trait*/ QRectF_contains for (f64, f64) {
  fn contains(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8containsEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZNK6QRectF8containsEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setX<T: QRectF_setX>(&mut self, value: T) -> i32 {
    value.setX(self);
    return 1;
  }
}

pub trait QRectF_setX {
  fn setX(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setX(qreal pos);
impl<'a> /*trait*/ QRectF_setX for (f64) {
  fn setX(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF4setXEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF4setXEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setRect<T: QRectF_setRect>(&mut self, value: T) -> i32 {
    value.setRect(self);
    return 1;
  }
}

pub trait QRectF_setRect {
  fn setRect(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QRectF_setRect for (f64, f64, f64, f64) {
  fn setRect(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7setRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QRectF7setRectEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn center<T: QRectF_center>(&mut self, value: T) -> i32 {
    value.center(self);
    return 1;
  }
}

pub trait QRectF_center {
  fn center(self, this: &mut QRectF) -> i32;
}

// proto: QPointF QRectF::center();
impl<'a> /*trait*/ QRectF_center for () {
  fn center(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6centerEv()};
    unsafe {_ZNK6QRectF6centerEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setLeft<T: QRectF_setLeft>(&mut self, value: T) -> i32 {
    value.setLeft(self);
    return 1;
  }
}

pub trait QRectF_setLeft {
  fn setLeft(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setLeft(qreal pos);
impl<'a> /*trait*/ QRectF_setLeft for (f64) {
  fn setLeft(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7setLeftEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF7setLeftEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn intersected<T: QRectF_intersected>(&mut self, value: T) -> i32 {
    value.intersected(self);
    return 1;
  }
}

pub trait QRectF_intersected {
  fn intersected(self, this: &mut QRectF) -> i32;
}

// proto: QRectF QRectF::intersected(const QRectF & other);
impl<'a> /*trait*/ QRectF_intersected for (&'a  QRectF) {
  fn intersected(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF11intersectedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QRectF11intersectedERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn topLeft<T: QRectF_topLeft>(&mut self, value: T) -> i32 {
    value.topLeft(self);
    return 1;
  }
}

pub trait QRectF_topLeft {
  fn topLeft(self, this: &mut QRectF) -> i32;
}

// proto: QPointF QRectF::topLeft();
impl<'a> /*trait*/ QRectF_topLeft for () {
  fn topLeft(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7topLeftEv()};
    unsafe {_ZNK6QRectF7topLeftEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn left<T: QRectF_left>(&mut self, value: T) -> i32 {
    value.left(self);
    return 1;
  }
}

pub trait QRectF_left {
  fn left(self, this: &mut QRectF) -> i32;
}

// proto: double QRectF::left();
impl<'a> /*trait*/ QRectF_left for () {
  fn left(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF4leftEv()};
    unsafe {_ZNK6QRectF4leftEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setY<T: QRectF_setY>(&mut self, value: T) -> i32 {
    value.setY(self);
    return 1;
  }
}

pub trait QRectF_setY {
  fn setY(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setY(qreal pos);
impl<'a> /*trait*/ QRectF_setY for (f64) {
  fn setY(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF4setYEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF4setYEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTopLeft<T: QRectF_moveTopLeft>(&mut self, value: T) -> i32 {
    value.moveTopLeft(self);
    return 1;
  }
}

pub trait QRectF_moveTopLeft {
  fn moveTopLeft(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::moveTopLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTopLeft for (&'a  QPointF) {
  fn moveTopLeft(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF11moveTopLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF11moveTopLeftERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn width<T: QRectF_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QRectF_width {
  fn width(self, this: &mut QRectF) -> i32;
}

// proto: double QRectF::width();
impl<'a> /*trait*/ QRectF_width for () {
  fn width(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF5widthEv()};
    unsafe {_ZNK6QRectF5widthEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setTop<T: QRectF_setTop>(&mut self, value: T) -> i32 {
    value.setTop(self);
    return 1;
  }
}

pub trait QRectF_setTop {
  fn setTop(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setTop(qreal pos);
impl<'a> /*trait*/ QRectF_setTop for (f64) {
  fn setTop(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6setTopEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF6setTopEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn isValid<T: QRectF_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QRectF_isValid {
  fn isValid(self, this: &mut QRectF) -> i32;
}

// proto: bool QRectF::isValid();
impl<'a> /*trait*/ QRectF_isValid for () {
  fn isValid(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7isValidEv()};
    unsafe {_ZNK6QRectF7isValidEv()};
    return 1;
  }
}

// proto: void QRectF::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QRectF_translate for (f64, f64) {
  fn translate(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN6QRectF9translateEdd(arg0, arg1)};
    return 1;
  }
}

// proto: void QRectF::NewQRectF(qreal left, qreal top, qreal width, qreal height);
impl<'a> /*trait*/ QRectF_NewQRectF for (f64, f64, f64, f64) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1Edddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QRectFC1Edddd(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn toRect<T: QRectF_toRect>(&mut self, value: T) -> i32 {
    value.toRect(self);
    return 1;
  }
}

pub trait QRectF_toRect {
  fn toRect(self, this: &mut QRectF) -> i32;
}

// proto: QRect QRectF::toRect();
impl<'a> /*trait*/ QRectF_toRect for () {
  fn toRect(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6toRectEv()};
    unsafe {_ZNK6QRectF6toRectEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveLeft<T: QRectF_moveLeft>(&mut self, value: T) -> i32 {
    value.moveLeft(self);
    return 1;
  }
}

pub trait QRectF_moveLeft {
  fn moveLeft(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::moveLeft(qreal pos);
impl<'a> /*trait*/ QRectF_moveLeft for (f64) {
  fn moveLeft(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8moveLeftEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF8moveLeftEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setTopLeft<T: QRectF_setTopLeft>(&mut self, value: T) -> i32 {
    value.setTopLeft(self);
    return 1;
  }
}

pub trait QRectF_setTopLeft {
  fn setTopLeft(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setTopLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_setTopLeft for (&'a  QPointF) {
  fn setTopLeft(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10setTopLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF10setTopLeftERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setBottomRight<T: QRectF_setBottomRight>(&mut self, value: T) -> i32 {
    value.setBottomRight(self);
    return 1;
  }
}

pub trait QRectF_setBottomRight {
  fn setBottomRight(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setBottomRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_setBottomRight for (&'a  QPointF) {
  fn setBottomRight(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF14setBottomRightERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF14setBottomRightERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn marginsAdded<T: QRectF_marginsAdded>(&mut self, value: T) -> i32 {
    value.marginsAdded(self);
    return 1;
  }
}

pub trait QRectF_marginsAdded {
  fn marginsAdded(self, this: &mut QRectF) -> i32;
}

// proto: QRectF QRectF::marginsAdded(const QMarginsF & margins);
impl<'a> /*trait*/ QRectF_marginsAdded for (&'a  QMarginsF) {
  fn marginsAdded(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF12marginsAddedERK9QMarginsF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QRectF12marginsAddedERK9QMarginsF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn translated<T: QRectF_translated>(&mut self, value: T) -> i32 {
    value.translated(self);
    return 1;
  }
}

pub trait QRectF_translated {
  fn translated(self, this: &mut QRectF) -> i32;
}

// proto: QRectF QRectF::translated(const QPointF & p);
impl<'a> /*trait*/ QRectF_translated for (&'a  QPointF) {
  fn translated(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QRectF10translatedERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn normalized<T: QRectF_normalized>(&mut self, value: T) -> i32 {
    value.normalized(self);
    return 1;
  }
}

pub trait QRectF_normalized {
  fn normalized(self, this: &mut QRectF) -> i32;
}

// proto: QRectF QRectF::normalized();
impl<'a> /*trait*/ QRectF_normalized for () {
  fn normalized(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10normalizedEv()};
    unsafe {_ZNK6QRectF10normalizedEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn getCoords<T: QRectF_getCoords>(&mut self, value: T) -> i32 {
    value.getCoords(self);
    return 1;
  }
}

pub trait QRectF_getCoords {
  fn getCoords(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::getCoords(qreal * x1, qreal * y1, qreal * x2, qreal * y2);
impl<'a> /*trait*/ QRectF_getCoords for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getCoords(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF9getCoordsEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
    unsafe {_ZNK6QRectF9getCoordsEPdS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setTopRight<T: QRectF_setTopRight>(&mut self, value: T) -> i32 {
    value.setTopRight(self);
    return 1;
  }
}

pub trait QRectF_setTopRight {
  fn setTopRight(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setTopRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_setTopRight for (&'a  QPointF) {
  fn setTopRight(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF11setTopRightERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF11setTopRightERK7QPointF(arg0)};
    return 1;
  }
}

// proto: bool QRectF::contains(const QPointF & p);
impl<'a> /*trait*/ QRectF_contains for (&'a  QPointF) {
  fn contains(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QRectF8containsERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn intersects<T: QRectF_intersects>(&mut self, value: T) -> i32 {
    value.intersects(self);
    return 1;
  }
}

pub trait QRectF_intersects {
  fn intersects(self, this: &mut QRectF) -> i32;
}

// proto: bool QRectF::intersects(const QRectF & r);
impl<'a> /*trait*/ QRectF_intersects for (&'a  QRectF) {
  fn intersects(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10intersectsERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QRectF10intersectsERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTop<T: QRectF_moveTop>(&mut self, value: T) -> i32 {
    value.moveTop(self);
    return 1;
  }
}

pub trait QRectF_moveTop {
  fn moveTop(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::moveTop(qreal pos);
impl<'a> /*trait*/ QRectF_moveTop for (f64) {
  fn moveTop(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7moveTopEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN6QRectF7moveTopEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setCoords<T: QRectF_setCoords>(&mut self, value: T) -> i32 {
    value.setCoords(self);
    return 1;
  }
}

pub trait QRectF_setCoords {
  fn setCoords(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::setCoords(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_setCoords for (f64, f64, f64, f64) {
  fn setCoords(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setCoordsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QRectF9setCoordsEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: QRectF QRectF::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QRectF_translated for (f64, f64) {
  fn translated(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZNK6QRectF10translatedEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn isEmpty<T: QRectF_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QRectF_isEmpty {
  fn isEmpty(self, this: &mut QRectF) -> i32;
}

// proto: bool QRectF::isEmpty();
impl<'a> /*trait*/ QRectF_isEmpty for () {
  fn isEmpty(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7isEmptyEv()};
    unsafe {_ZNK6QRectF7isEmptyEv()};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTopRight<T: QRectF_moveTopRight>(&mut self, value: T) -> i32 {
    value.moveTopRight(self);
    return 1;
  }
}

pub trait QRectF_moveTopRight {
  fn moveTopRight(self, this: &mut QRectF) -> i32;
}

// proto: void QRectF::moveTopRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTopRight for (&'a  QPointF) {
  fn moveTopRight(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF12moveTopRightERK7QPointF()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectF12moveTopRightERK7QPointF(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn united<T: QRectF_united>(&mut self, value: T) -> i32 {
    value.united(self);
    return 1;
  }
}

pub trait QRectF_united {
  fn united(self, this: &mut QRectF) -> i32;
}

// proto: QRectF QRectF::united(const QRectF & other);
impl<'a> /*trait*/ QRectF_united for (&'a  QRectF) {
  fn united(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6unitedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK6QRectF6unitedERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn right<T: QRectF_right>(&mut self, value: T) -> i32 {
    value.right(self);
    return 1;
  }
}

pub trait QRectF_right {
  fn right(self, this: &mut QRectF) -> i32;
}

// proto: double QRectF::right();
impl<'a> /*trait*/ QRectF_right for () {
  fn right(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF5rightEv()};
    unsafe {_ZNK6QRectF5rightEv()};
    return 1;
  }
}

// proto: void QRectF::NewQRectF(const QRect & rect);
impl<'a> /*trait*/ QRectF_NewQRectF for (&'a  QRect) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1ERK5QRect()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN6QRectFC1ERK5QRect(qthis, arg0)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn adjusted<T: QRectF_adjusted>(&mut self, value: T) -> i32 {
    value.adjusted(self);
    return 1;
  }
}

pub trait QRectF_adjusted {
  fn adjusted(self, this: &mut QRectF) -> i32;
}

// proto: QRectF QRectF::adjusted(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_adjusted for (f64, f64, f64, f64) {
  fn adjusted(self, this: &mut QRectF) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8adjustedEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZNK6QRectF8adjustedEdddd(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

