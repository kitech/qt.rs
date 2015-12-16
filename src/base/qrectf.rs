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
  pub fn moveBottomRight<T: QRectF_moveBottomRight>(&mut self, value: T)  {
     value.moveBottomRight(self);
    // return 1;
  }
}

pub trait QRectF_moveBottomRight {
  fn moveBottomRight(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::moveBottomRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveBottomRight for (&'a  QPointF) {
  fn moveBottomRight(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF15moveBottomRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF15moveBottomRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTo<T: QRectF_moveTo>(&mut self, value: T)  {
     value.moveTo(self);
    // return 1;
  }
}

pub trait QRectF_moveTo {
  fn moveTo(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::moveTo(qreal x, qreal y);
impl<'a> /*trait*/ QRectF_moveTo for (f64, f64) {
  fn moveTo(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6moveToEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN6QRectF6moveToEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn top<T: QRectF_top>(&mut self, value: T) -> f64 {
    return value.top(self);
    // return 1;
  }
}

pub trait QRectF_top {
  fn top(self, rsthis: &mut QRectF) -> f64;
}

// proto:  double QRectF::top();
impl<'a> /*trait*/ QRectF_top for () {
  fn top(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF3topEv()};
    let mut ret = unsafe {_ZNK6QRectF3topEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn bottomLeft<T: QRectF_bottomLeft>(&mut self, value: T) -> QPointF {
    return value.bottomLeft(self);
    // return 1;
  }
}

pub trait QRectF_bottomLeft {
  fn bottomLeft(self, rsthis: &mut QRectF) -> QPointF;
}

// proto:  QPointF QRectF::bottomLeft();
impl<'a> /*trait*/ QRectF_bottomLeft for () {
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
  pub fn setHeight<T: QRectF_setHeight>(&mut self, value: T)  {
     value.setHeight(self);
    // return 1;
  }
}

pub trait QRectF_setHeight {
  fn setHeight(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setHeight(qreal h);
impl<'a> /*trait*/ QRectF_setHeight for (f64) {
  fn setHeight(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setSize<T: QRectF_setSize>(&mut self, value: T)  {
     value.setSize(self);
    // return 1;
  }
}

pub trait QRectF_setSize {
  fn setSize(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setSize(const QSizeF & s);
impl<'a> /*trait*/ QRectF_setSize for (&'a  QSizeF) {
  fn setSize(self, rsthis: &mut QRectF)  {
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
impl<'a> /*trait*/ QRectF_moveTo for (&'a  QPointF) {
  fn moveTo(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6moveToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF6moveToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn toAlignedRect<T: QRectF_toAlignedRect>(&mut self, value: T) -> QRect {
    return value.toAlignedRect(self);
    // return 1;
  }
}

pub trait QRectF_toAlignedRect {
  fn toAlignedRect(self, rsthis: &mut QRectF) -> QRect;
}

// proto:  QRect QRectF::toAlignedRect();
impl<'a> /*trait*/ QRectF_toAlignedRect for () {
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
  pub fn setRight<T: QRectF_setRight>(&mut self, value: T)  {
     value.setRight(self);
    // return 1;
  }
}

pub trait QRectF_setRight {
  fn setRight(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setRight(qreal pos);
impl<'a> /*trait*/ QRectF_setRight for (f64) {
  fn setRight(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8setRightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF8setRightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setBottomLeft<T: QRectF_setBottomLeft>(&mut self, value: T)  {
     value.setBottomLeft(self);
    // return 1;
  }
}

pub trait QRectF_setBottomLeft {
  fn setBottomLeft(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setBottomLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_setBottomLeft for (&'a  QPointF) {
  fn setBottomLeft(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF13setBottomLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF13setBottomLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn topRight<T: QRectF_topRight>(&mut self, value: T) -> QPointF {
    return value.topRight(self);
    // return 1;
  }
}

pub trait QRectF_topRight {
  fn topRight(self, rsthis: &mut QRectF) -> QPointF;
}

// proto:  QPointF QRectF::topRight();
impl<'a> /*trait*/ QRectF_topRight for () {
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
  pub fn size<T: QRectF_size>(&mut self, value: T) -> QSizeF {
    return value.size(self);
    // return 1;
  }
}

pub trait QRectF_size {
  fn size(self, rsthis: &mut QRectF) -> QSizeF;
}

// proto:  QSizeF QRectF::size();
impl<'a> /*trait*/ QRectF_size for () {
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
  pub fn adjust<T: QRectF_adjust>(&mut self, value: T)  {
     value.adjust(self);
    // return 1;
  }
}

pub trait QRectF_adjust {
  fn adjust(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::adjust(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_adjust for (f64, f64, f64, f64) {
  fn adjust(self, rsthis: &mut QRectF)  {
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
  pub fn moveRight<T: QRectF_moveRight>(&mut self, value: T)  {
     value.moveRight(self);
    // return 1;
  }
}

pub trait QRectF_moveRight {
  fn moveRight(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::moveRight(qreal pos);
impl<'a> /*trait*/ QRectF_moveRight for (f64) {
  fn moveRight(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9moveRightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF9moveRightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn y<T: QRectF_y>(&mut self, value: T)  {
     value.y(self);
    // return 1;
  }
}

pub trait QRectF_y {
  fn y(self, rsthis: &mut QRectF) ;
}

// proto:  double QRectF::y();
impl<'a> /*trait*/ QRectF_y for () {
  fn y(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF1yEv()};
     unsafe {_ZNK6QRectF1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn bottomRight<T: QRectF_bottomRight>(&mut self, value: T) -> QPointF {
    return value.bottomRight(self);
    // return 1;
  }
}

pub trait QRectF_bottomRight {
  fn bottomRight(self, rsthis: &mut QRectF) -> QPointF;
}

// proto:  QPointF QRectF::bottomRight();
impl<'a> /*trait*/ QRectF_bottomRight for () {
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
  pub fn setBottom<T: QRectF_setBottom>(&mut self, value: T)  {
     value.setBottom(self);
    // return 1;
  }
}

pub trait QRectF_setBottom {
  fn setBottom(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setBottom(qreal pos);
impl<'a> /*trait*/ QRectF_setBottom for (f64) {
  fn setBottom(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9setBottomEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF9setBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveBottomLeft<T: QRectF_moveBottomLeft>(&mut self, value: T)  {
     value.moveBottomLeft(self);
    // return 1;
  }
}

pub trait QRectF_moveBottomLeft {
  fn moveBottomLeft(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::moveBottomLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveBottomLeft for (&'a  QPointF) {
  fn moveBottomLeft(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF14moveBottomLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF14moveBottomLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveBottom<T: QRectF_moveBottom>(&mut self, value: T)  {
     value.moveBottom(self);
    // return 1;
  }
}

pub trait QRectF_moveBottom {
  fn moveBottom(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::moveBottom(qreal pos);
impl<'a> /*trait*/ QRectF_moveBottom for (f64) {
  fn moveBottom(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10moveBottomEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF10moveBottomEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn getRect<T: QRectF_getRect>(&mut self, value: T)  {
     value.getRect(self);
    // return 1;
  }
}

pub trait QRectF_getRect {
  fn getRect(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::getRect(qreal * x, qreal * y, qreal * w, qreal * h);
impl<'a> /*trait*/ QRectF_getRect for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getRect(self, rsthis: &mut QRectF)  {
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
  pub fn x<T: QRectF_x>(&mut self, value: T)  {
     value.x(self);
    // return 1;
  }
}

pub trait QRectF_x {
  fn x(self, rsthis: &mut QRectF) ;
}

// proto:  double QRectF::x();
impl<'a> /*trait*/ QRectF_x for () {
  fn x(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF1xEv()};
     unsafe {_ZNK6QRectF1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn bottom<T: QRectF_bottom>(&mut self, value: T) -> f64 {
    return value.bottom(self);
    // return 1;
  }
}

pub trait QRectF_bottom {
  fn bottom(self, rsthis: &mut QRectF) -> f64;
}

// proto:  double QRectF::bottom();
impl<'a> /*trait*/ QRectF_bottom for () {
  fn bottom(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6bottomEv()};
    let mut ret = unsafe {_ZNK6QRectF6bottomEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn isNull<T: QRectF_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QRectF_isNull {
  fn isNull(self, rsthis: &mut QRectF) -> i8;
}

// proto:  bool QRectF::isNull();
impl<'a> /*trait*/ QRectF_isNull for () {
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
  pub fn setWidth<T: QRectF_setWidth>(&mut self, value: T)  {
     value.setWidth(self);
    // return 1;
  }
}

pub trait QRectF_setWidth {
  fn setWidth(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setWidth(qreal w);
impl<'a> /*trait*/ QRectF_setWidth for (f64) {
  fn setWidth(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn height<T: QRectF_height>(&mut self, value: T) -> f64 {
    return value.height(self);
    // return 1;
  }
}

pub trait QRectF_height {
  fn height(self, rsthis: &mut QRectF) -> f64;
}

// proto:  double QRectF::height();
impl<'a> /*trait*/ QRectF_height for () {
  fn height(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF6heightEv()};
    let mut ret = unsafe {_ZNK6QRectF6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn translate<T: QRectF_translate>(&mut self, value: T)  {
     value.translate(self);
    // return 1;
  }
}

pub trait QRectF_translate {
  fn translate(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::translate(const QPointF & p);
impl<'a> /*trait*/ QRectF_translate for (&'a  QPointF) {
  fn translate(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveCenter<T: QRectF_moveCenter>(&mut self, value: T)  {
     value.moveCenter(self);
    // return 1;
  }
}

pub trait QRectF_moveCenter {
  fn moveCenter(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::moveCenter(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveCenter for (&'a  QPointF) {
  fn moveCenter(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10moveCenterERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF10moveCenterERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn contains<T: QRectF_contains>(&mut self, value: T) -> i8 {
    return value.contains(self);
    // return 1;
  }
}

pub trait QRectF_contains {
  fn contains(self, rsthis: &mut QRectF) -> i8;
}

// proto:  bool QRectF::contains(const QRectF & r);
impl<'a> /*trait*/ QRectF_contains for (&'a  QRectF) {
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
  pub fn marginsRemoved<T: QRectF_marginsRemoved>(&mut self, value: T) -> QRectF {
    return value.marginsRemoved(self);
    // return 1;
  }
}

pub trait QRectF_marginsRemoved {
  fn marginsRemoved(self, rsthis: &mut QRectF) -> QRectF;
}

// proto:  QRectF QRectF::marginsRemoved(const QMarginsF & margins);
impl<'a> /*trait*/ QRectF_marginsRemoved for (&'a  QMarginsF) {
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
impl<'a> /*trait*/ QRectF_contains for (f64, f64) {
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
  pub fn setX<T: QRectF_setX>(&mut self, value: T)  {
     value.setX(self);
    // return 1;
  }
}

pub trait QRectF_setX {
  fn setX(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setX(qreal pos);
impl<'a> /*trait*/ QRectF_setX for (f64) {
  fn setX(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF4setXEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF4setXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setRect<T: QRectF_setRect>(&mut self, value: T)  {
     value.setRect(self);
    // return 1;
  }
}

pub trait QRectF_setRect {
  fn setRect(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QRectF_setRect for (f64, f64, f64, f64) {
  fn setRect(self, rsthis: &mut QRectF)  {
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
  pub fn center<T: QRectF_center>(&mut self, value: T) -> QPointF {
    return value.center(self);
    // return 1;
  }
}

pub trait QRectF_center {
  fn center(self, rsthis: &mut QRectF) -> QPointF;
}

// proto:  QPointF QRectF::center();
impl<'a> /*trait*/ QRectF_center for () {
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
  pub fn setLeft<T: QRectF_setLeft>(&mut self, value: T)  {
     value.setLeft(self);
    // return 1;
  }
}

pub trait QRectF_setLeft {
  fn setLeft(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setLeft(qreal pos);
impl<'a> /*trait*/ QRectF_setLeft for (f64) {
  fn setLeft(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7setLeftEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF7setLeftEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn intersected<T: QRectF_intersected>(&mut self, value: T) -> QRectF {
    return value.intersected(self);
    // return 1;
  }
}

pub trait QRectF_intersected {
  fn intersected(self, rsthis: &mut QRectF) -> QRectF;
}

// proto:  QRectF QRectF::intersected(const QRectF & other);
impl<'a> /*trait*/ QRectF_intersected for (&'a  QRectF) {
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
  pub fn topLeft<T: QRectF_topLeft>(&mut self, value: T) -> QPointF {
    return value.topLeft(self);
    // return 1;
  }
}

pub trait QRectF_topLeft {
  fn topLeft(self, rsthis: &mut QRectF) -> QPointF;
}

// proto:  QPointF QRectF::topLeft();
impl<'a> /*trait*/ QRectF_topLeft for () {
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
  pub fn left<T: QRectF_left>(&mut self, value: T) -> f64 {
    return value.left(self);
    // return 1;
  }
}

pub trait QRectF_left {
  fn left(self, rsthis: &mut QRectF) -> f64;
}

// proto:  double QRectF::left();
impl<'a> /*trait*/ QRectF_left for () {
  fn left(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF4leftEv()};
    let mut ret = unsafe {_ZNK6QRectF4leftEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setY<T: QRectF_setY>(&mut self, value: T)  {
     value.setY(self);
    // return 1;
  }
}

pub trait QRectF_setY {
  fn setY(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setY(qreal pos);
impl<'a> /*trait*/ QRectF_setY for (f64) {
  fn setY(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF4setYEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF4setYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTopLeft<T: QRectF_moveTopLeft>(&mut self, value: T)  {
     value.moveTopLeft(self);
    // return 1;
  }
}

pub trait QRectF_moveTopLeft {
  fn moveTopLeft(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::moveTopLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTopLeft for (&'a  QPointF) {
  fn moveTopLeft(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF11moveTopLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF11moveTopLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn width<T: QRectF_width>(&mut self, value: T) -> f64 {
    return value.width(self);
    // return 1;
  }
}

pub trait QRectF_width {
  fn width(self, rsthis: &mut QRectF) -> f64;
}

// proto:  double QRectF::width();
impl<'a> /*trait*/ QRectF_width for () {
  fn width(self, rsthis: &mut QRectF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF5widthEv()};
    let mut ret = unsafe {_ZNK6QRectF5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setTop<T: QRectF_setTop>(&mut self, value: T)  {
     value.setTop(self);
    // return 1;
  }
}

pub trait QRectF_setTop {
  fn setTop(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setTop(qreal pos);
impl<'a> /*trait*/ QRectF_setTop for (f64) {
  fn setTop(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF6setTopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF6setTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn isValid<T: QRectF_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QRectF_isValid {
  fn isValid(self, rsthis: &mut QRectF) -> i8;
}

// proto:  bool QRectF::isValid();
impl<'a> /*trait*/ QRectF_isValid for () {
  fn isValid(self, rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7isValidEv()};
    let mut ret = unsafe {_ZNK6QRectF7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QRectF::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QRectF_translate for (f64, f64) {
  fn translate(self, rsthis: &mut QRectF)  {
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
  pub fn toRect<T: QRectF_toRect>(&mut self, value: T) -> QRect {
    return value.toRect(self);
    // return 1;
  }
}

pub trait QRectF_toRect {
  fn toRect(self, rsthis: &mut QRectF) -> QRect;
}

// proto:  QRect QRectF::toRect();
impl<'a> /*trait*/ QRectF_toRect for () {
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
  pub fn moveLeft<T: QRectF_moveLeft>(&mut self, value: T)  {
     value.moveLeft(self);
    // return 1;
  }
}

pub trait QRectF_moveLeft {
  fn moveLeft(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::moveLeft(qreal pos);
impl<'a> /*trait*/ QRectF_moveLeft for (f64) {
  fn moveLeft(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF8moveLeftEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF8moveLeftEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setTopLeft<T: QRectF_setTopLeft>(&mut self, value: T)  {
     value.setTopLeft(self);
    // return 1;
  }
}

pub trait QRectF_setTopLeft {
  fn setTopLeft(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setTopLeft(const QPointF & p);
impl<'a> /*trait*/ QRectF_setTopLeft for (&'a  QPointF) {
  fn setTopLeft(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF10setTopLeftERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF10setTopLeftERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setBottomRight<T: QRectF_setBottomRight>(&mut self, value: T)  {
     value.setBottomRight(self);
    // return 1;
  }
}

pub trait QRectF_setBottomRight {
  fn setBottomRight(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setBottomRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_setBottomRight for (&'a  QPointF) {
  fn setBottomRight(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF14setBottomRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF14setBottomRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn marginsAdded<T: QRectF_marginsAdded>(&mut self, value: T) -> QRectF {
    return value.marginsAdded(self);
    // return 1;
  }
}

pub trait QRectF_marginsAdded {
  fn marginsAdded(self, rsthis: &mut QRectF) -> QRectF;
}

// proto:  QRectF QRectF::marginsAdded(const QMarginsF & margins);
impl<'a> /*trait*/ QRectF_marginsAdded for (&'a  QMarginsF) {
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
  pub fn translated<T: QRectF_translated>(&mut self, value: T) -> QRectF {
    return value.translated(self);
    // return 1;
  }
}

pub trait QRectF_translated {
  fn translated(self, rsthis: &mut QRectF) -> QRectF;
}

// proto:  QRectF QRectF::translated(const QPointF & p);
impl<'a> /*trait*/ QRectF_translated for (&'a  QPointF) {
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
  pub fn normalized<T: QRectF_normalized>(&mut self, value: T) -> QRectF {
    return value.normalized(self);
    // return 1;
  }
}

pub trait QRectF_normalized {
  fn normalized(self, rsthis: &mut QRectF) -> QRectF;
}

// proto:  QRectF QRectF::normalized();
impl<'a> /*trait*/ QRectF_normalized for () {
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
  pub fn getCoords<T: QRectF_getCoords>(&mut self, value: T)  {
     value.getCoords(self);
    // return 1;
  }
}

pub trait QRectF_getCoords {
  fn getCoords(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::getCoords(qreal * x1, qreal * y1, qreal * x2, qreal * y2);
impl<'a> /*trait*/ QRectF_getCoords for (&'a mut f64, &'a mut f64, &'a mut f64, &'a mut f64) {
  fn getCoords(self, rsthis: &mut QRectF)  {
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
  pub fn setTopRight<T: QRectF_setTopRight>(&mut self, value: T)  {
     value.setTopRight(self);
    // return 1;
  }
}

pub trait QRectF_setTopRight {
  fn setTopRight(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setTopRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_setTopRight for (&'a  QPointF) {
  fn setTopRight(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF11setTopRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF11setTopRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QRectF::contains(const QPointF & p);
impl<'a> /*trait*/ QRectF_contains for (&'a  QPointF) {
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
  pub fn intersects<T: QRectF_intersects>(&mut self, value: T) -> i8 {
    return value.intersects(self);
    // return 1;
  }
}

pub trait QRectF_intersects {
  fn intersects(self, rsthis: &mut QRectF) -> i8;
}

// proto:  bool QRectF::intersects(const QRectF & r);
impl<'a> /*trait*/ QRectF_intersects for (&'a  QRectF) {
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
  pub fn moveTop<T: QRectF_moveTop>(&mut self, value: T)  {
     value.moveTop(self);
    // return 1;
  }
}

pub trait QRectF_moveTop {
  fn moveTop(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::moveTop(qreal pos);
impl<'a> /*trait*/ QRectF_moveTop for (f64) {
  fn moveTop(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF7moveTopEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QRectF7moveTopEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn setCoords<T: QRectF_setCoords>(&mut self, value: T)  {
     value.setCoords(self);
    // return 1;
  }
}

pub trait QRectF_setCoords {
  fn setCoords(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::setCoords(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_setCoords for (f64, f64, f64, f64) {
  fn setCoords(self, rsthis: &mut QRectF)  {
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
impl<'a> /*trait*/ QRectF_translated for (f64, f64) {
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
  pub fn isEmpty<T: QRectF_isEmpty>(&mut self, value: T) -> i8 {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QRectF_isEmpty {
  fn isEmpty(self, rsthis: &mut QRectF) -> i8;
}

// proto:  bool QRectF::isEmpty();
impl<'a> /*trait*/ QRectF_isEmpty for () {
  fn isEmpty(self, rsthis: &mut QRectF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QRectF7isEmptyEv()};
    let mut ret = unsafe {_ZNK6QRectF7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn moveTopRight<T: QRectF_moveTopRight>(&mut self, value: T)  {
     value.moveTopRight(self);
    // return 1;
  }
}

pub trait QRectF_moveTopRight {
  fn moveTopRight(self, rsthis: &mut QRectF) ;
}

// proto:  void QRectF::moveTopRight(const QPointF & p);
impl<'a> /*trait*/ QRectF_moveTopRight for (&'a  QPointF) {
  fn moveTopRight(self, rsthis: &mut QRectF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QRectF12moveTopRightERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QRectF12moveTopRightERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRectF {
  pub fn united<T: QRectF_united>(&mut self, value: T) -> QRectF {
    return value.united(self);
    // return 1;
  }
}

pub trait QRectF_united {
  fn united(self, rsthis: &mut QRectF) -> QRectF;
}

// proto:  QRectF QRectF::united(const QRectF & other);
impl<'a> /*trait*/ QRectF_united for (&'a  QRectF) {
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
  pub fn right<T: QRectF_right>(&mut self, value: T) -> f64 {
    return value.right(self);
    // return 1;
  }
}

pub trait QRectF_right {
  fn right(self, rsthis: &mut QRectF) -> f64;
}

// proto:  double QRectF::right();
impl<'a> /*trait*/ QRectF_right for () {
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
  pub fn adjusted<T: QRectF_adjusted>(&mut self, value: T) -> QRectF {
    return value.adjusted(self);
    // return 1;
  }
}

pub trait QRectF_adjusted {
  fn adjusted(self, rsthis: &mut QRectF) -> QRectF;
}

// proto:  QRectF QRectF::adjusted(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QRectF_adjusted for (f64, f64, f64, f64) {
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

