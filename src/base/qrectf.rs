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
use super::qrect::QRect;
use super::qmarginsf::QMarginsF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QRectF::NewQRectF();
  fn _ZN6QRectFC1Ev(qthis: *mut c_void) ;
  // proto:  void QRectF::moveBottomRight(const QPointF & p);
  fn _ZN6QRectF15moveBottomRightERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRectF::moveTo(qreal x, qreal y);
  fn _ZN6QRectF6moveToEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  double QRectF::top();
  fn _ZNK6QRectF3topEv(qthis: *mut c_void) -> c_double;
  // proto:  QPointF QRectF::bottomLeft();
  fn _ZNK6QRectF10bottomLeftEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::setHeight(qreal h);
  fn _ZN6QRectF9setHeightEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QRectF::setSize(const QSizeF & s);
  fn _ZN6QRectF7setSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRectF::NewQRectF(const QPointF & topleft, const QPointF & bottomRight);
  fn _ZN6QRectFC1ERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QRectF::moveTo(const QPointF & p);
  fn _ZN6QRectF6moveToERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRect QRectF::toAlignedRect();
  fn _ZNK6QRectF13toAlignedRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::setRight(qreal pos);
  fn _ZN6QRectF8setRightEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QRectF::setBottomLeft(const QPointF & p);
  fn _ZN6QRectF13setBottomLeftERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPointF QRectF::topRight();
  fn _ZNK6QRectF8topRightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizeF QRectF::size();
  fn _ZNK6QRectF4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::adjust(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZN6QRectF6adjustEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  void QRectF::moveRight(qreal pos);
  fn _ZN6QRectF9moveRightEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QRectF::y();
  fn _ZNK6QRectF1yEv(qthis: *mut c_void) ;
  // proto:  QPointF QRectF::bottomRight();
  fn _ZNK6QRectF11bottomRightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::setBottom(qreal pos);
  fn _ZN6QRectF9setBottomEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QRectF::moveBottomLeft(const QPointF & p);
  fn _ZN6QRectF14moveBottomLeftERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRectF::moveBottom(qreal pos);
  fn _ZN6QRectF10moveBottomEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QRectF::getRect(qreal * x, qreal * y, qreal * w, qreal * h);
  fn _ZNK6QRectF7getRectEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) ;
  // proto:  double QRectF::x();
  fn _ZNK6QRectF1xEv(qthis: *mut c_void) ;
  // proto:  double QRectF::bottom();
  fn _ZNK6QRectF6bottomEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QRectF::isNull();
  fn _ZNK6QRectF6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QRectF::NewQRectF(const QPointF & topleft, const QSizeF & size);
  fn _ZN6QRectFC1ERK7QPointFRK6QSizeF(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QRectF::setWidth(qreal w);
  fn _ZN6QRectF8setWidthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QRectF::height();
  fn _ZNK6QRectF6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRectF::translate(const QPointF & p);
  fn _ZN6QRectF9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRectF::moveCenter(const QPointF & p);
  fn _ZN6QRectF10moveCenterERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QRectF::contains(const QRectF & r);
  fn _ZNK6QRectF8containsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QRectF QRectF::marginsRemoved(const QMarginsF & margins);
  fn _ZNK6QRectF14marginsRemovedERK9QMarginsF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRectF::contains(qreal x, qreal y);
  fn _ZNK6QRectF8containsEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> int8_t;
  // proto:  void QRectF::setX(qreal pos);
  fn _ZN6QRectF4setXEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QRectF::setRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN6QRectF7setRectEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  QPointF QRectF::center();
  fn _ZNK6QRectF6centerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::setLeft(qreal pos);
  fn _ZN6QRectF7setLeftEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QRectF QRectF::intersected(const QRectF & other);
  fn _ZNK6QRectF11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPointF QRectF::topLeft();
  fn _ZNK6QRectF7topLeftEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QRectF::left();
  fn _ZNK6QRectF4leftEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRectF::setY(qreal pos);
  fn _ZN6QRectF4setYEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QRectF::moveTopLeft(const QPointF & p);
  fn _ZN6QRectF11moveTopLeftERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QRectF::width();
  fn _ZNK6QRectF5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRectF::setTop(qreal pos);
  fn _ZN6QRectF6setTopEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  bool QRectF::isValid();
  fn _ZNK6QRectF7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QRectF::translate(qreal dx, qreal dy);
  fn _ZN6QRectF9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QRectF::NewQRectF(qreal left, qreal top, qreal width, qreal height);
  fn _ZN6QRectFC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  QRect QRectF::toRect();
  fn _ZNK6QRectF6toRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::moveLeft(qreal pos);
  fn _ZN6QRectF8moveLeftEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QRectF::setTopLeft(const QPointF & p);
  fn _ZN6QRectF10setTopLeftERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRectF::setBottomRight(const QPointF & p);
  fn _ZN6QRectF14setBottomRightERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QRectF::marginsAdded(const QMarginsF & margins);
  fn _ZNK6QRectF12marginsAddedERK9QMarginsF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QRectF::translated(const QPointF & p);
  fn _ZNK6QRectF10translatedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QRectF::normalized();
  fn _ZNK6QRectF10normalizedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRectF::getCoords(qreal * x1, qreal * y1, qreal * x2, qreal * y2);
  fn _ZNK6QRectF9getCoordsEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double) ;
  // proto:  void QRectF::setTopRight(const QPointF & p);
  fn _ZN6QRectF11setTopRightERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QRectF::contains(const QPointF & p);
  fn _ZNK6QRectF8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QRectF::intersects(const QRectF & r);
  fn _ZNK6QRectF10intersectsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QRectF::moveTop(qreal pos);
  fn _ZN6QRectF7moveTopEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QRectF::setCoords(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZN6QRectF9setCoordsEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  QRectF QRectF::translated(qreal dx, qreal dy);
  fn _ZNK6QRectF10translatedEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  bool QRectF::isEmpty();
  fn _ZNK6QRectF7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QRectF::moveTopRight(const QPointF & p);
  fn _ZN6QRectF12moveTopRightERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QRectF::united(const QRectF & other);
  fn _ZNK6QRectF6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  double QRectF::right();
  fn _ZNK6QRectF5rightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QRectF::NewQRectF(const QRect & rect);
  fn _ZN6QRectFC1ERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRectF QRectF::adjusted(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZNK6QRectF8adjustedEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) -> *mut c_void;
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
  pub fn moveBottomRight<RetType, T: QRectF_moveBottomRight<RetType>>(&mut self, value: T) -> RetType {
    return value.moveBottomRight(self);
    // return 1;
  }
}

pub trait QRectF_moveBottomRight<RetType> {
  fn moveBottomRight(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::moveBottomRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveBottomRight<()> for (&'a  QPointF) {
  fn moveBottomRight(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF15moveBottomRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF15moveBottomRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTo<RetType, T: QRectF_moveTo<RetType>>(&mut self, value: T) -> RetType {
    return value.moveTo(self);
    // return 1;
  }
}

pub trait QRectF_moveTo<RetType> {
  fn moveTo(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::moveTo(qreal x, qreal y);
impl<'a> /*trait*/ QRectF_moveTo<()> for (f64, f64) {
  fn moveTo(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6moveToEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN6QRectF6moveToEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn top<RetType, T: QRectF_top<RetType>>(&mut self, value: T) -> RetType {
    return value.top(self);
    // return 1;
  }
}

pub trait QRectF_top<RetType> {
  fn top(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  double QRectF::top();
impl<'a> /*trait*/ QRectF_top<f64> for () {
  fn top(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF3topEv()};
    let mut ret = unsafe {_ZNK6QRectF3topEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn bottomLeft<RetType, T: QRectF_bottomLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.bottomLeft(self);
    // return 1;
  }
}

pub trait QRectF_bottomLeft<RetType> {
  fn bottomLeft(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QPointF QRectF::bottomLeft();
impl<'a> /*trait*/ QRectF_bottomLeft<QPointF> for () {
  fn bottomLeft(self, rsthis: &mut QRectF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10bottomLeftEv()};
    let mut ret = unsafe {_ZNK6QRectF10bottomLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setHeight<RetType, T: QRectF_setHeight<RetType>>(&mut self, value: T) -> RetType {
    return value.setHeight(self);
    // return 1;
  }
}

pub trait QRectF_setHeight<RetType> {
  fn setHeight(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setHeight(qreal h);
impl<'a> /*trait*/ QRectF_setHeight<()> for (f64) {
  fn setHeight(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setSize<RetType, T: QRectF_setSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setSize(self);
    // return 1;
  }
}

pub trait QRectF_setSize<RetType> {
  fn setSize(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setSize(const QSizeF & s);
impl<'a> /*trait*/ QRectF_setSize<()> for (&'a  QSizeF) {
  fn setSize(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7setSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF7setSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QRectF::NewQRectF(const QPointF & topleft, const QPointF & bottomRight);
impl<'a> /*trait*/ QRectF_NewQRectF for (&'a  QPointF, &'a  QPointF) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1ERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN6QRectFC1ERK7QPointFS2_(qthis, arg0, arg1)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QRectF::moveTo(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTo<()> for (&'a  QPointF) {
  fn moveTo(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6moveToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF6moveToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn toAlignedRect<RetType, T: QRectF_toAlignedRect<RetType>>(&mut self, value: T) -> RetType {
    return value.toAlignedRect(self);
    // return 1;
  }
}

pub trait QRectF_toAlignedRect<RetType> {
  fn toAlignedRect(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QRect QRectF::toAlignedRect();
impl<'a> /*trait*/ QRectF_toAlignedRect<QRect> for () {
  fn toAlignedRect(self, rsthis: &mut QRectF) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF13toAlignedRectEv()};
    let mut ret = unsafe {_ZNK6QRectF13toAlignedRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setRight<RetType, T: QRectF_setRight<RetType>>(&mut self, value: T) -> RetType {
    return value.setRight(self);
    // return 1;
  }
}

pub trait QRectF_setRight<RetType> {
  fn setRight(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setRight(qreal pos);
impl<'a> /*trait*/ QRectF_setRight<()> for (f64) {
  fn setRight(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8setRightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF8setRightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setBottomLeft<RetType, T: QRectF_setBottomLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.setBottomLeft(self);
    // return 1;
  }
}

pub trait QRectF_setBottomLeft<RetType> {
  fn setBottomLeft(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setBottomLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_setBottomLeft<()> for (&'a  QPointF) {
  fn setBottomLeft(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF13setBottomLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF13setBottomLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn topRight<RetType, T: QRectF_topRight<RetType>>(&mut self, value: T) -> RetType {
    return value.topRight(self);
    // return 1;
  }
}

pub trait QRectF_topRight<RetType> {
  fn topRight(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QPointF QRectF::topRight();
impl<'a> /*trait*/ QRectF_topRight<QPointF> for () {
  fn topRight(self, rsthis: &mut QRectF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8topRightEv()};
    let mut ret = unsafe {_ZNK6QRectF8topRightEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn size<RetType, T: QRectF_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QRectF_size<RetType> {
  fn size(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QSizeF QRectF::size();
impl<'a> /*trait*/ QRectF_size<QSizeF> for () {
  fn size(self, rsthis: &mut QRectF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF4sizeEv()};
    let mut ret = unsafe {_ZNK6QRectF4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn adjust<RetType, T: QRectF_adjust<RetType>>(&mut self, value: T) -> RetType {
    return value.adjust(self);
    // return 1;
  }
}

pub trait QRectF_adjust<RetType> {
  fn adjust(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::adjust(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_adjust<()> for (f64, f64, f64, f64) {
  fn adjust(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6adjustEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QRectF6adjustEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveRight<RetType, T: QRectF_moveRight<RetType>>(&mut self, value: T) -> RetType {
    return value.moveRight(self);
    // return 1;
  }
}

pub trait QRectF_moveRight<RetType> {
  fn moveRight(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::moveRight(qreal pos);
impl<'a> /*trait*/ QRectF_moveRight<()> for (f64) {
  fn moveRight(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9moveRightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF9moveRightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn y<RetType, T: QRectF_y<RetType>>(&mut self, value: T) -> RetType {
    return value.y(self);
    // return 1;
  }
}

pub trait QRectF_y<RetType> {
  fn y(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  double QRectF::y();
impl<'a> /*trait*/ QRectF_y<()> for () {
  fn y(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF1yEv()};
     unsafe {_ZNK6QRectF1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn bottomRight<RetType, T: QRectF_bottomRight<RetType>>(&mut self, value: T) -> RetType {
    return value.bottomRight(self);
    // return 1;
  }
}

pub trait QRectF_bottomRight<RetType> {
  fn bottomRight(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QPointF QRectF::bottomRight();
impl<'a> /*trait*/ QRectF_bottomRight<QPointF> for () {
  fn bottomRight(self, rsthis: &mut QRectF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF11bottomRightEv()};
    let mut ret = unsafe {_ZNK6QRectF11bottomRightEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setBottom<RetType, T: QRectF_setBottom<RetType>>(&mut self, value: T) -> RetType {
    return value.setBottom(self);
    // return 1;
  }
}

pub trait QRectF_setBottom<RetType> {
  fn setBottom(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setBottom(qreal pos);
impl<'a> /*trait*/ QRectF_setBottom<()> for (f64) {
  fn setBottom(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setBottomEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF9setBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveBottomLeft<RetType, T: QRectF_moveBottomLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.moveBottomLeft(self);
    // return 1;
  }
}

pub trait QRectF_moveBottomLeft<RetType> {
  fn moveBottomLeft(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::moveBottomLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveBottomLeft<()> for (&'a  QPointF) {
  fn moveBottomLeft(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF14moveBottomLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF14moveBottomLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveBottom<RetType, T: QRectF_moveBottom<RetType>>(&mut self, value: T) -> RetType {
    return value.moveBottom(self);
    // return 1;
  }
}

pub trait QRectF_moveBottom<RetType> {
  fn moveBottom(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::moveBottom(qreal pos);
impl<'a> /*trait*/ QRectF_moveBottom<()> for (f64) {
  fn moveBottom(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10moveBottomEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF10moveBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn getRect<RetType, T: QRectF_getRect<RetType>>(&mut self, value: T) -> RetType {
    return value.getRect(self);
    // return 1;
  }
}

pub trait QRectF_getRect<RetType> {
  fn getRect(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::getRect(qreal * x, qreal * y, qreal * w, qreal * h);
impl<'a> /*trait*/ QRectF_getRect<()> for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getRect(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7getRectEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
     unsafe {_ZNK6QRectF7getRectEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn x<RetType, T: QRectF_x<RetType>>(&mut self, value: T) -> RetType {
    return value.x(self);
    // return 1;
  }
}

pub trait QRectF_x<RetType> {
  fn x(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  double QRectF::x();
impl<'a> /*trait*/ QRectF_x<()> for () {
  fn x(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF1xEv()};
     unsafe {_ZNK6QRectF1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn bottom<RetType, T: QRectF_bottom<RetType>>(&mut self, value: T) -> RetType {
    return value.bottom(self);
    // return 1;
  }
}

pub trait QRectF_bottom<RetType> {
  fn bottom(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  double QRectF::bottom();
impl<'a> /*trait*/ QRectF_bottom<f64> for () {
  fn bottom(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6bottomEv()};
    let mut ret = unsafe {_ZNK6QRectF6bottomEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn isNull<RetType, T: QRectF_isNull<RetType>>(&mut self, value: T) -> RetType {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QRectF_isNull<RetType> {
  fn isNull(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  bool QRectF::isNull();
impl<'a> /*trait*/ QRectF_isNull<i8> for () {
  fn isNull(self, rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6isNullEv()};
    let mut ret = unsafe {_ZNK6QRectF6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QRectF::NewQRectF(const QPointF & topleft, const QSizeF & size);
impl<'a> /*trait*/ QRectF_NewQRectF for (&'a  QPointF, &'a  QSizeF) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1ERK7QPointFRK6QSizeF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN6QRectFC1ERK7QPointFRK6QSizeF(qthis, arg0, arg1)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setWidth<RetType, T: QRectF_setWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setWidth(self);
    // return 1;
  }
}

pub trait QRectF_setWidth<RetType> {
  fn setWidth(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setWidth(qreal w);
impl<'a> /*trait*/ QRectF_setWidth<()> for (f64) {
  fn setWidth(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn height<RetType, T: QRectF_height<RetType>>(&mut self, value: T) -> RetType {
    return value.height(self);
    // return 1;
  }
}

pub trait QRectF_height<RetType> {
  fn height(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  double QRectF::height();
impl<'a> /*trait*/ QRectF_height<f64> for () {
  fn height(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6heightEv()};
    let mut ret = unsafe {_ZNK6QRectF6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn translate<RetType, T: QRectF_translate<RetType>>(&mut self, value: T) -> RetType {
    return value.translate(self);
    // return 1;
  }
}

pub trait QRectF_translate<RetType> {
  fn translate(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::translate(const QPointF & p);
impl<'a> /*trait*/ QRectF_translate<()> for (&'a  QPointF) {
  fn translate(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveCenter<RetType, T: QRectF_moveCenter<RetType>>(&mut self, value: T) -> RetType {
    return value.moveCenter(self);
    // return 1;
  }
}

pub trait QRectF_moveCenter<RetType> {
  fn moveCenter(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::moveCenter(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveCenter<()> for (&'a  QPointF) {
  fn moveCenter(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10moveCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF10moveCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn contains<RetType, T: QRectF_contains<RetType>>(&mut self, value: T) -> RetType {
    return value.contains(self);
    // return 1;
  }
}

pub trait QRectF_contains<RetType> {
  fn contains(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  bool QRectF::contains(const QRectF & r);
impl<'a> /*trait*/ QRectF_contains<i8> for (&'a  QRectF) {
  fn contains(self, rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8containsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF8containsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn marginsRemoved<RetType, T: QRectF_marginsRemoved<RetType>>(&mut self, value: T) -> RetType {
    return value.marginsRemoved(self);
    // return 1;
  }
}

pub trait QRectF_marginsRemoved<RetType> {
  fn marginsRemoved(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QRectF QRectF::marginsRemoved(const QMarginsF & margins);
impl<'a> /*trait*/ QRectF_marginsRemoved<QRectF> for (&'a  QMarginsF) {
  fn marginsRemoved(self, rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF14marginsRemovedERK9QMarginsF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF14marginsRemovedERK9QMarginsF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QRectF::contains(qreal x, qreal y);
impl<'a> /*trait*/ QRectF_contains<i8> for (f64, f64) {
  fn contains(self, rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8containsEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK6QRectF8containsEdd(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setX<RetType, T: QRectF_setX<RetType>>(&mut self, value: T) -> RetType {
    return value.setX(self);
    // return 1;
  }
}

pub trait QRectF_setX<RetType> {
  fn setX(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setX(qreal pos);
impl<'a> /*trait*/ QRectF_setX<()> for (f64) {
  fn setX(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF4setXEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF4setXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setRect<RetType, T: QRectF_setRect<RetType>>(&mut self, value: T) -> RetType {
    return value.setRect(self);
    // return 1;
  }
}

pub trait QRectF_setRect<RetType> {
  fn setRect(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QRectF_setRect<()> for (f64, f64, f64, f64) {
  fn setRect(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7setRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QRectF7setRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn center<RetType, T: QRectF_center<RetType>>(&mut self, value: T) -> RetType {
    return value.center(self);
    // return 1;
  }
}

pub trait QRectF_center<RetType> {
  fn center(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QPointF QRectF::center();
impl<'a> /*trait*/ QRectF_center<QPointF> for () {
  fn center(self, rsthis: &mut QRectF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6centerEv()};
    let mut ret = unsafe {_ZNK6QRectF6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setLeft<RetType, T: QRectF_setLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.setLeft(self);
    // return 1;
  }
}

pub trait QRectF_setLeft<RetType> {
  fn setLeft(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setLeft(qreal pos);
impl<'a> /*trait*/ QRectF_setLeft<()> for (f64) {
  fn setLeft(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7setLeftEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF7setLeftEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn intersected<RetType, T: QRectF_intersected<RetType>>(&mut self, value: T) -> RetType {
    return value.intersected(self);
    // return 1;
  }
}

pub trait QRectF_intersected<RetType> {
  fn intersected(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QRectF QRectF::intersected(const QRectF & other);
impl<'a> /*trait*/ QRectF_intersected<QRectF> for (&'a  QRectF) {
  fn intersected(self, rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn topLeft<RetType, T: QRectF_topLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.topLeft(self);
    // return 1;
  }
}

pub trait QRectF_topLeft<RetType> {
  fn topLeft(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QPointF QRectF::topLeft();
impl<'a> /*trait*/ QRectF_topLeft<QPointF> for () {
  fn topLeft(self, rsthis: &mut QRectF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7topLeftEv()};
    let mut ret = unsafe {_ZNK6QRectF7topLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn left<RetType, T: QRectF_left<RetType>>(&mut self, value: T) -> RetType {
    return value.left(self);
    // return 1;
  }
}

pub trait QRectF_left<RetType> {
  fn left(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  double QRectF::left();
impl<'a> /*trait*/ QRectF_left<f64> for () {
  fn left(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF4leftEv()};
    let mut ret = unsafe {_ZNK6QRectF4leftEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setY<RetType, T: QRectF_setY<RetType>>(&mut self, value: T) -> RetType {
    return value.setY(self);
    // return 1;
  }
}

pub trait QRectF_setY<RetType> {
  fn setY(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setY(qreal pos);
impl<'a> /*trait*/ QRectF_setY<()> for (f64) {
  fn setY(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF4setYEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF4setYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTopLeft<RetType, T: QRectF_moveTopLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.moveTopLeft(self);
    // return 1;
  }
}

pub trait QRectF_moveTopLeft<RetType> {
  fn moveTopLeft(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::moveTopLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTopLeft<()> for (&'a  QPointF) {
  fn moveTopLeft(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF11moveTopLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF11moveTopLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn width<RetType, T: QRectF_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QRectF_width<RetType> {
  fn width(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  double QRectF::width();
impl<'a> /*trait*/ QRectF_width<f64> for () {
  fn width(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF5widthEv()};
    let mut ret = unsafe {_ZNK6QRectF5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setTop<RetType, T: QRectF_setTop<RetType>>(&mut self, value: T) -> RetType {
    return value.setTop(self);
    // return 1;
  }
}

pub trait QRectF_setTop<RetType> {
  fn setTop(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setTop(qreal pos);
impl<'a> /*trait*/ QRectF_setTop<()> for (f64) {
  fn setTop(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6setTopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF6setTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn isValid<RetType, T: QRectF_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QRectF_isValid<RetType> {
  fn isValid(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  bool QRectF::isValid();
impl<'a> /*trait*/ QRectF_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7isValidEv()};
    let mut ret = unsafe {_ZNK6QRectF7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QRectF::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QRectF_translate<()> for (f64, f64) {
  fn translate(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN6QRectF9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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
  pub fn toRect<RetType, T: QRectF_toRect<RetType>>(&mut self, value: T) -> RetType {
    return value.toRect(self);
    // return 1;
  }
}

pub trait QRectF_toRect<RetType> {
  fn toRect(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QRect QRectF::toRect();
impl<'a> /*trait*/ QRectF_toRect<QRect> for () {
  fn toRect(self, rsthis: &mut QRectF) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6toRectEv()};
    let mut ret = unsafe {_ZNK6QRectF6toRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveLeft<RetType, T: QRectF_moveLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.moveLeft(self);
    // return 1;
  }
}

pub trait QRectF_moveLeft<RetType> {
  fn moveLeft(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::moveLeft(qreal pos);
impl<'a> /*trait*/ QRectF_moveLeft<()> for (f64) {
  fn moveLeft(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8moveLeftEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF8moveLeftEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setTopLeft<RetType, T: QRectF_setTopLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.setTopLeft(self);
    // return 1;
  }
}

pub trait QRectF_setTopLeft<RetType> {
  fn setTopLeft(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setTopLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_setTopLeft<()> for (&'a  QPointF) {
  fn setTopLeft(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10setTopLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF10setTopLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setBottomRight<RetType, T: QRectF_setBottomRight<RetType>>(&mut self, value: T) -> RetType {
    return value.setBottomRight(self);
    // return 1;
  }
}

pub trait QRectF_setBottomRight<RetType> {
  fn setBottomRight(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setBottomRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_setBottomRight<()> for (&'a  QPointF) {
  fn setBottomRight(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF14setBottomRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF14setBottomRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn marginsAdded<RetType, T: QRectF_marginsAdded<RetType>>(&mut self, value: T) -> RetType {
    return value.marginsAdded(self);
    // return 1;
  }
}

pub trait QRectF_marginsAdded<RetType> {
  fn marginsAdded(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QRectF QRectF::marginsAdded(const QMarginsF & margins);
impl<'a> /*trait*/ QRectF_marginsAdded<QRectF> for (&'a  QMarginsF) {
  fn marginsAdded(self, rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF12marginsAddedERK9QMarginsF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF12marginsAddedERK9QMarginsF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn translated<RetType, T: QRectF_translated<RetType>>(&mut self, value: T) -> RetType {
    return value.translated(self);
    // return 1;
  }
}

pub trait QRectF_translated<RetType> {
  fn translated(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QRectF QRectF::translated(const QPointF & p);
impl<'a> /*trait*/ QRectF_translated<QRectF> for (&'a  QPointF) {
  fn translated(self, rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF10translatedERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn normalized<RetType, T: QRectF_normalized<RetType>>(&mut self, value: T) -> RetType {
    return value.normalized(self);
    // return 1;
  }
}

pub trait QRectF_normalized<RetType> {
  fn normalized(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QRectF QRectF::normalized();
impl<'a> /*trait*/ QRectF_normalized<QRectF> for () {
  fn normalized(self, rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10normalizedEv()};
    let mut ret = unsafe {_ZNK6QRectF10normalizedEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn getCoords<RetType, T: QRectF_getCoords<RetType>>(&mut self, value: T) -> RetType {
    return value.getCoords(self);
    // return 1;
  }
}

pub trait QRectF_getCoords<RetType> {
  fn getCoords(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::getCoords(qreal * x1, qreal * y1, qreal * x2, qreal * y2);
impl<'a> /*trait*/ QRectF_getCoords<()> for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getCoords(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF9getCoordsEPdS0_S0_S0_()};
    let arg0 = self.0  as *mut c_double;
    let arg1 = self.1  as *mut c_double;
    let arg2 = self.2  as *mut c_double;
    let arg3 = self.3  as *mut c_double;
     unsafe {_ZNK6QRectF9getCoordsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setTopRight<RetType, T: QRectF_setTopRight<RetType>>(&mut self, value: T) -> RetType {
    return value.setTopRight(self);
    // return 1;
  }
}

pub trait QRectF_setTopRight<RetType> {
  fn setTopRight(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setTopRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_setTopRight<()> for (&'a  QPointF) {
  fn setTopRight(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF11setTopRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF11setTopRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QRectF::contains(const QPointF & p);
impl<'a> /*trait*/ QRectF_contains<i8> for (&'a  QPointF) {
  fn contains(self, rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn intersects<RetType, T: QRectF_intersects<RetType>>(&mut self, value: T) -> RetType {
    return value.intersects(self);
    // return 1;
  }
}

pub trait QRectF_intersects<RetType> {
  fn intersects(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  bool QRectF::intersects(const QRectF & r);
impl<'a> /*trait*/ QRectF_intersects<i8> for (&'a  QRectF) {
  fn intersects(self, rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTop<RetType, T: QRectF_moveTop<RetType>>(&mut self, value: T) -> RetType {
    return value.moveTop(self);
    // return 1;
  }
}

pub trait QRectF_moveTop<RetType> {
  fn moveTop(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::moveTop(qreal pos);
impl<'a> /*trait*/ QRectF_moveTop<()> for (f64) {
  fn moveTop(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7moveTopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF7moveTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setCoords<RetType, T: QRectF_setCoords<RetType>>(&mut self, value: T) -> RetType {
    return value.setCoords(self);
    // return 1;
  }
}

pub trait QRectF_setCoords<RetType> {
  fn setCoords(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::setCoords(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_setCoords<()> for (f64, f64, f64, f64) {
  fn setCoords(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setCoordsEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QRectF9setCoordsEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  QRectF QRectF::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QRectF_translated<QRectF> for (f64, f64) {
  fn translated(self, rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK6QRectF10translatedEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn isEmpty<RetType, T: QRectF_isEmpty<RetType>>(&mut self, value: T) -> RetType {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QRectF_isEmpty<RetType> {
  fn isEmpty(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  bool QRectF::isEmpty();
impl<'a> /*trait*/ QRectF_isEmpty<i8> for () {
  fn isEmpty(self, rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7isEmptyEv()};
    let mut ret = unsafe {_ZNK6QRectF7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTopRight<RetType, T: QRectF_moveTopRight<RetType>>(&mut self, value: T) -> RetType {
    return value.moveTopRight(self);
    // return 1;
  }
}

pub trait QRectF_moveTopRight<RetType> {
  fn moveTopRight(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  void QRectF::moveTopRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTopRight<()> for (&'a  QPointF) {
  fn moveTopRight(self, rsthis: &mut QRectF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF12moveTopRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF12moveTopRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn united<RetType, T: QRectF_united<RetType>>(&mut self, value: T) -> RetType {
    return value.united(self);
    // return 1;
  }
}

pub trait QRectF_united<RetType> {
  fn united(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QRectF QRectF::united(const QRectF & other);
impl<'a> /*trait*/ QRectF_united<QRectF> for (&'a  QRectF) {
  fn united(self, rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QRectF6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn right<RetType, T: QRectF_right<RetType>>(&mut self, value: T) -> RetType {
    return value.right(self);
    // return 1;
  }
}

pub trait QRectF_right<RetType> {
  fn right(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  double QRectF::right();
impl<'a> /*trait*/ QRectF_right<f64> for () {
  fn right(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF5rightEv()};
    let mut ret = unsafe {_ZNK6QRectF5rightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QRectF::NewQRectF(const QRect & rect);
impl<'a> /*trait*/ QRectF_NewQRectF for (&'a  QRect) {
  fn NewQRectF(self) -> QRectF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectFC1ERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QRectFC1ERK5QRect(qthis, arg0)};
    let rsthis = QRectF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn adjusted<RetType, T: QRectF_adjusted<RetType>>(&mut self, value: T) -> RetType {
    return value.adjusted(self);
    // return 1;
  }
}

pub trait QRectF_adjusted<RetType> {
  fn adjusted(self, rsthis: &mut QRectF) -> RetType;
}

// proto:  QRectF QRectF::adjusted(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_adjusted<QRectF> for (f64, f64, f64, f64) {
  fn adjusted(self, rsthis: &mut QRectF) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF8adjustedEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let mut ret = unsafe {_ZNK6QRectF8adjustedEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

