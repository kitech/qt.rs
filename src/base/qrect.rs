// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qsize::QSize;
use super::qmargins::QMargins;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int QRect::right();
  fn _ZNK5QRect5rightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRect::moveTo(const QPoint & p);
  fn _ZN5QRect6moveToERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRect::moveTopLeft(const QPoint & p);
  fn _ZN5QRect11moveTopLeftERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRect::moveRight(int pos);
  fn _ZN5QRect9moveRightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QRect::NewQRect(const QPoint & topleft, const QPoint & bottomright);
  fn _ZN5QRectC1ERK6QPointS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QRect QRect::translated(int dx, int dy);
  fn _ZNK5QRect10translatedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QPoint QRect::center();
  fn _ZNK5QRect6centerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRect::moveTopRight(const QPoint & p);
  fn _ZN5QRect12moveTopRightERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRect::setLeft(int pos);
  fn _ZN5QRect7setLeftEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QRect::left();
  fn _ZNK5QRect4leftEv(qthis: *mut c_void) -> c_int;
  // proto:  QRect QRect::intersected(const QRect & other);
  fn _ZNK5QRect11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRect::contains(int x, int y, bool proper);
  fn _ZNK5QRect8containsEiib(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: int8_t) -> int8_t;
  // proto:  QPoint QRect::bottomRight();
  fn _ZNK5QRect11bottomRightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QRect::isValid();
  fn _ZNK5QRect7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSize QRect::size();
  fn _ZNK5QRect4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QRect::united(const QRect & other);
  fn _ZNK5QRect6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRect::adjust(int x1, int y1, int x2, int y2);
  fn _ZN5QRect6adjustEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  bool QRect::isNull();
  fn _ZNK5QRect6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QRect::setBottom(int pos);
  fn _ZN5QRect9setBottomEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QRect::setSize(const QSize & s);
  fn _ZN5QRect7setSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QRect::y();
  fn _ZNK5QRect1yEv(qthis: *mut c_void) ;
  // proto:  int QRect::x();
  fn _ZNK5QRect1xEv(qthis: *mut c_void) ;
  // proto:  QRect QRect::adjusted(int x1, int y1, int x2, int y2);
  fn _ZNK5QRect8adjustedEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto:  void QRect::NewQRect(const QPoint & topleft, const QSize & size);
  fn _ZN5QRectC1ERK6QPointRK5QSize(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  int QRect::height();
  fn _ZNK5QRect6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRect::NewQRect(int left, int top, int width, int height);
  fn _ZN5QRectC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QRect::moveBottomLeft(const QPoint & p);
  fn _ZN5QRect14moveBottomLeftERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QRect::top();
  fn _ZNK5QRect3topEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRect::moveTo(int x, int t);
  fn _ZN5QRect6moveToEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QRect::getRect(int * x, int * y, int * w, int * h);
  fn _ZNK5QRect7getRectEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) ;
  // proto:  bool QRect::contains(const QRect & r, bool proper);
  fn _ZNK5QRect8containsERKS_b(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) -> int8_t;
  // proto:  QRect QRect::marginsRemoved(const QMargins & margins);
  fn _ZNK5QRect14marginsRemovedERK8QMargins(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRect::translate(int dx, int dy);
  fn _ZN5QRect9translateEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QPoint QRect::topLeft();
  fn _ZNK5QRect7topLeftEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QRect::contains(const QPoint & p, bool proper);
  fn _ZNK5QRect8containsERK6QPointb(qthis: *mut c_void, arg0: *mut c_void, arg1: int8_t) -> int8_t;
  // proto:  int QRect::width();
  fn _ZNK5QRect5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRect::setRect(int x, int y, int w, int h);
  fn _ZN5QRect7setRectEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QRect::moveCenter(const QPoint & p);
  fn _ZN5QRect10moveCenterERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QRect::intersects(const QRect & r);
  fn _ZNK5QRect10intersectsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QRect::setTopRight(const QPoint & p);
  fn _ZN5QRect11setTopRightERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRect::setCoords(int x1, int y1, int x2, int y2);
  fn _ZN5QRect9setCoordsEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QRect::translate(const QPoint & p);
  fn _ZN5QRect9translateERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRect::moveBottom(int pos);
  fn _ZN5QRect10moveBottomEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QRect::setBottomLeft(const QPoint & p);
  fn _ZN5QRect13setBottomLeftERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRect::getCoords(int * x1, int * y1, int * x2, int * y2);
  fn _ZNK5QRect9getCoordsEPiS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) ;
  // proto:  QPoint QRect::topRight();
  fn _ZNK5QRect8topRightEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRect::setBottomRight(const QPoint & p);
  fn _ZN5QRect14setBottomRightERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRect::setHeight(int h);
  fn _ZN5QRect9setHeightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QRect::isEmpty();
  fn _ZNK5QRect7isEmptyEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QRect::contains(int x, int y);
  fn _ZNK5QRect8containsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> int8_t;
  // proto:  void QRect::moveBottomRight(const QPoint & p);
  fn _ZN5QRect15moveBottomRightERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPoint QRect::bottomLeft();
  fn _ZNK5QRect10bottomLeftEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRect::setTop(int pos);
  fn _ZN5QRect6setTopEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QRect::bottom();
  fn _ZNK5QRect6bottomEv(qthis: *mut c_void) -> c_int;
  // proto:  void QRect::NewQRect();
  fn _ZN5QRectC1Ev(qthis: *mut c_void) ;
  // proto:  QRect QRect::marginsAdded(const QMargins & margins);
  fn _ZNK5QRect12marginsAddedERK8QMargins(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRect QRect::normalized();
  fn _ZNK5QRect10normalizedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRect::setWidth(int w);
  fn _ZN5QRect8setWidthEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QRect::setY(int y);
  fn _ZN5QRect4setYEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QRect::moveTop(int pos);
  fn _ZN5QRect7moveTopEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QRect::setX(int x);
  fn _ZN5QRect4setXEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QRect::setRight(int pos);
  fn _ZN5QRect8setRightEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QRect::setTopLeft(const QPoint & p);
  fn _ZN5QRect10setTopLeftERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QRect::moveLeft(int pos);
  fn _ZN5QRect8moveLeftEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QRect QRect::translated(const QPoint & p);
  fn _ZNK5QRect10translatedERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QRect)=16
pub struct QRect {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRect {
  pub fn right<RetType, T: QRect_right<RetType>>(&mut self, value: T) -> RetType {
    return value.right(self);
    // return 1;
  }
}

pub trait QRect_right<RetType> {
  fn right(self, rsthis: &mut QRect) -> RetType;
}

// proto:  int QRect::right();
impl<'a> /*trait*/ QRect_right<i32> for () {
  fn right(self, rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect5rightEv()};
    let mut ret = unsafe {_ZNK5QRect5rightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveTo<RetType, T: QRect_moveTo<RetType>>(&mut self, value: T) -> RetType {
    return value.moveTo(self);
    // return 1;
  }
}

pub trait QRect_moveTo<RetType> {
  fn moveTo(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::moveTo(const QPoint & p);
impl<'a> /*trait*/ QRect_moveTo<()> for (&'a  QPoint) {
  fn moveTo(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6moveToERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect6moveToERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveTopLeft<RetType, T: QRect_moveTopLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.moveTopLeft(self);
    // return 1;
  }
}

pub trait QRect_moveTopLeft<RetType> {
  fn moveTopLeft(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::moveTopLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_moveTopLeft<()> for (&'a  QPoint) {
  fn moveTopLeft(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect11moveTopLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect11moveTopLeftERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveRight<RetType, T: QRect_moveRight<RetType>>(&mut self, value: T) -> RetType {
    return value.moveRight(self);
    // return 1;
  }
}

pub trait QRect_moveRight<RetType> {
  fn moveRight(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::moveRight(int pos);
impl<'a> /*trait*/ QRect_moveRight<()> for (i32) {
  fn moveRight(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9moveRightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect9moveRightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn NewQRect<T: QRect_NewQRect>(value: T) -> QRect {
    let rsthis = value.NewQRect();
    return rsthis;
    // return 1;
  }
}

pub trait QRect_NewQRect {
  fn NewQRect(self) -> QRect;
}

// proto: void QRect::NewQRect(const QPoint & topleft, const QPoint & bottomright);
impl<'a> /*trait*/ QRect_NewQRect for (&'a  QPoint, &'a  QPoint) {
  fn NewQRect(self) -> QRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRectC1ERK6QPointS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QRectC1ERK6QPointS2_(qthis, arg0, arg1)};
    let rsthis = QRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn translated<RetType, T: QRect_translated<RetType>>(&mut self, value: T) -> RetType {
    return value.translated(self);
    // return 1;
  }
}

pub trait QRect_translated<RetType> {
  fn translated(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QRect QRect::translated(int dx, int dy);
impl<'a> /*trait*/ QRect_translated<QRect> for (i32, i32) {
  fn translated(self, rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK5QRect10translatedEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn center<RetType, T: QRect_center<RetType>>(&mut self, value: T) -> RetType {
    return value.center(self);
    // return 1;
  }
}

pub trait QRect_center<RetType> {
  fn center(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QPoint QRect::center();
impl<'a> /*trait*/ QRect_center<QPoint> for () {
  fn center(self, rsthis: &mut QRect) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6centerEv()};
    let mut ret = unsafe {_ZNK5QRect6centerEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveTopRight<RetType, T: QRect_moveTopRight<RetType>>(&mut self, value: T) -> RetType {
    return value.moveTopRight(self);
    // return 1;
  }
}

pub trait QRect_moveTopRight<RetType> {
  fn moveTopRight(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::moveTopRight(const QPoint & p);
impl<'a> /*trait*/ QRect_moveTopRight<()> for (&'a  QPoint) {
  fn moveTopRight(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect12moveTopRightERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect12moveTopRightERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setLeft<RetType, T: QRect_setLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.setLeft(self);
    // return 1;
  }
}

pub trait QRect_setLeft<RetType> {
  fn setLeft(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setLeft(int pos);
impl<'a> /*trait*/ QRect_setLeft<()> for (i32) {
  fn setLeft(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7setLeftEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect7setLeftEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn left<RetType, T: QRect_left<RetType>>(&mut self, value: T) -> RetType {
    return value.left(self);
    // return 1;
  }
}

pub trait QRect_left<RetType> {
  fn left(self, rsthis: &mut QRect) -> RetType;
}

// proto:  int QRect::left();
impl<'a> /*trait*/ QRect_left<i32> for () {
  fn left(self, rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect4leftEv()};
    let mut ret = unsafe {_ZNK5QRect4leftEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn intersected<RetType, T: QRect_intersected<RetType>>(&mut self, value: T) -> RetType {
    return value.intersected(self);
    // return 1;
  }
}

pub trait QRect_intersected<RetType> {
  fn intersected(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QRect QRect::intersected(const QRect & other);
impl<'a> /*trait*/ QRect_intersected<QRect> for (&'a  QRect) {
  fn intersected(self, rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn contains<RetType, T: QRect_contains<RetType>>(&mut self, value: T) -> RetType {
    return value.contains(self);
    // return 1;
  }
}

pub trait QRect_contains<RetType> {
  fn contains(self, rsthis: &mut QRect) -> RetType;
}

// proto:  bool QRect::contains(int x, int y, bool proper);
impl<'a> /*trait*/ QRect_contains<i8> for (i32, i32, i8) {
  fn contains(self, rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsEiib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as int8_t;
    let mut ret = unsafe {_ZNK5QRect8containsEiib(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn bottomRight<RetType, T: QRect_bottomRight<RetType>>(&mut self, value: T) -> RetType {
    return value.bottomRight(self);
    // return 1;
  }
}

pub trait QRect_bottomRight<RetType> {
  fn bottomRight(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QPoint QRect::bottomRight();
impl<'a> /*trait*/ QRect_bottomRight<QPoint> for () {
  fn bottomRight(self, rsthis: &mut QRect) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect11bottomRightEv()};
    let mut ret = unsafe {_ZNK5QRect11bottomRightEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn isValid<RetType, T: QRect_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QRect_isValid<RetType> {
  fn isValid(self, rsthis: &mut QRect) -> RetType;
}

// proto:  bool QRect::isValid();
impl<'a> /*trait*/ QRect_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7isValidEv()};
    let mut ret = unsafe {_ZNK5QRect7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn size<RetType, T: QRect_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QRect_size<RetType> {
  fn size(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QSize QRect::size();
impl<'a> /*trait*/ QRect_size<QSize> for () {
  fn size(self, rsthis: &mut QRect) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect4sizeEv()};
    let mut ret = unsafe {_ZNK5QRect4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn united<RetType, T: QRect_united<RetType>>(&mut self, value: T) -> RetType {
    return value.united(self);
    // return 1;
  }
}

pub trait QRect_united<RetType> {
  fn united(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QRect QRect::united(const QRect & other);
impl<'a> /*trait*/ QRect_united<QRect> for (&'a  QRect) {
  fn united(self, rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn adjust<RetType, T: QRect_adjust<RetType>>(&mut self, value: T) -> RetType {
    return value.adjust(self);
    // return 1;
  }
}

pub trait QRect_adjust<RetType> {
  fn adjust(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::adjust(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QRect_adjust<()> for (i32, i32, i32, i32) {
  fn adjust(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6adjustEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN5QRect6adjustEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn isNull<RetType, T: QRect_isNull<RetType>>(&mut self, value: T) -> RetType {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QRect_isNull<RetType> {
  fn isNull(self, rsthis: &mut QRect) -> RetType;
}

// proto:  bool QRect::isNull();
impl<'a> /*trait*/ QRect_isNull<i8> for () {
  fn isNull(self, rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6isNullEv()};
    let mut ret = unsafe {_ZNK5QRect6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setBottom<RetType, T: QRect_setBottom<RetType>>(&mut self, value: T) -> RetType {
    return value.setBottom(self);
    // return 1;
  }
}

pub trait QRect_setBottom<RetType> {
  fn setBottom(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setBottom(int pos);
impl<'a> /*trait*/ QRect_setBottom<()> for (i32) {
  fn setBottom(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9setBottomEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect9setBottomEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setSize<RetType, T: QRect_setSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setSize(self);
    // return 1;
  }
}

pub trait QRect_setSize<RetType> {
  fn setSize(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setSize(const QSize & s);
impl<'a> /*trait*/ QRect_setSize<()> for (&'a  QSize) {
  fn setSize(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7setSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect7setSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn y<RetType, T: QRect_y<RetType>>(&mut self, value: T) -> RetType {
    return value.y(self);
    // return 1;
  }
}

pub trait QRect_y<RetType> {
  fn y(self, rsthis: &mut QRect) -> RetType;
}

// proto:  int QRect::y();
impl<'a> /*trait*/ QRect_y<()> for () {
  fn y(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect1yEv()};
     unsafe {_ZNK5QRect1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn x<RetType, T: QRect_x<RetType>>(&mut self, value: T) -> RetType {
    return value.x(self);
    // return 1;
  }
}

pub trait QRect_x<RetType> {
  fn x(self, rsthis: &mut QRect) -> RetType;
}

// proto:  int QRect::x();
impl<'a> /*trait*/ QRect_x<()> for () {
  fn x(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect1xEv()};
     unsafe {_ZNK5QRect1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn adjusted<RetType, T: QRect_adjusted<RetType>>(&mut self, value: T) -> RetType {
    return value.adjusted(self);
    // return 1;
  }
}

pub trait QRect_adjusted<RetType> {
  fn adjusted(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QRect QRect::adjusted(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QRect_adjusted<QRect> for (i32, i32, i32, i32) {
  fn adjusted(self, rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8adjustedEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZNK5QRect8adjustedEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QRect::NewQRect(const QPoint & topleft, const QSize & size);
impl<'a> /*trait*/ QRect_NewQRect for (&'a  QPoint, &'a  QSize) {
  fn NewQRect(self) -> QRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRectC1ERK6QPointRK5QSize()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN5QRectC1ERK6QPointRK5QSize(qthis, arg0, arg1)};
    let rsthis = QRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn height<RetType, T: QRect_height<RetType>>(&mut self, value: T) -> RetType {
    return value.height(self);
    // return 1;
  }
}

pub trait QRect_height<RetType> {
  fn height(self, rsthis: &mut QRect) -> RetType;
}

// proto:  int QRect::height();
impl<'a> /*trait*/ QRect_height<i32> for () {
  fn height(self, rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6heightEv()};
    let mut ret = unsafe {_ZNK5QRect6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QRect::NewQRect(int left, int top, int width, int height);
impl<'a> /*trait*/ QRect_NewQRect for (i32, i32, i32, i32) {
  fn NewQRect(self) -> QRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRectC1Eiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QRectC1Eiiii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveBottomLeft<RetType, T: QRect_moveBottomLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.moveBottomLeft(self);
    // return 1;
  }
}

pub trait QRect_moveBottomLeft<RetType> {
  fn moveBottomLeft(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::moveBottomLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_moveBottomLeft<()> for (&'a  QPoint) {
  fn moveBottomLeft(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect14moveBottomLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect14moveBottomLeftERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn top<RetType, T: QRect_top<RetType>>(&mut self, value: T) -> RetType {
    return value.top(self);
    // return 1;
  }
}

pub trait QRect_top<RetType> {
  fn top(self, rsthis: &mut QRect) -> RetType;
}

// proto:  int QRect::top();
impl<'a> /*trait*/ QRect_top<i32> for () {
  fn top(self, rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect3topEv()};
    let mut ret = unsafe {_ZNK5QRect3topEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QRect::moveTo(int x, int t);
impl<'a> /*trait*/ QRect_moveTo<()> for (i32, i32) {
  fn moveTo(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6moveToEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN5QRect6moveToEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn getRect<RetType, T: QRect_getRect<RetType>>(&mut self, value: T) -> RetType {
    return value.getRect(self);
    // return 1;
  }
}

pub trait QRect_getRect<RetType> {
  fn getRect(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::getRect(int * x, int * y, int * w, int * h);
impl<'a> /*trait*/ QRect_getRect<()> for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getRect(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7getRectEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZNK5QRect7getRectEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  bool QRect::contains(const QRect & r, bool proper);
impl<'a> /*trait*/ QRect_contains<i8> for (&'a  QRect, i8) {
  fn contains(self, rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsERKS_b()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
    let mut ret = unsafe {_ZNK5QRect8containsERKS_b(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn marginsRemoved<RetType, T: QRect_marginsRemoved<RetType>>(&mut self, value: T) -> RetType {
    return value.marginsRemoved(self);
    // return 1;
  }
}

pub trait QRect_marginsRemoved<RetType> {
  fn marginsRemoved(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QRect QRect::marginsRemoved(const QMargins & margins);
impl<'a> /*trait*/ QRect_marginsRemoved<QRect> for (&'a  QMargins) {
  fn marginsRemoved(self, rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect14marginsRemovedERK8QMargins()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect14marginsRemovedERK8QMargins(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn translate<RetType, T: QRect_translate<RetType>>(&mut self, value: T) -> RetType {
    return value.translate(self);
    // return 1;
  }
}

pub trait QRect_translate<RetType> {
  fn translate(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::translate(int dx, int dy);
impl<'a> /*trait*/ QRect_translate<()> for (i32, i32) {
  fn translate(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN5QRect9translateEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn topLeft<RetType, T: QRect_topLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.topLeft(self);
    // return 1;
  }
}

pub trait QRect_topLeft<RetType> {
  fn topLeft(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QPoint QRect::topLeft();
impl<'a> /*trait*/ QRect_topLeft<QPoint> for () {
  fn topLeft(self, rsthis: &mut QRect) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7topLeftEv()};
    let mut ret = unsafe {_ZNK5QRect7topLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QRect::contains(const QPoint & p, bool proper);
impl<'a> /*trait*/ QRect_contains<i8> for (&'a  QPoint, i8) {
  fn contains(self, rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsERK6QPointb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as int8_t;
    let mut ret = unsafe {_ZNK5QRect8containsERK6QPointb(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn width<RetType, T: QRect_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QRect_width<RetType> {
  fn width(self, rsthis: &mut QRect) -> RetType;
}

// proto:  int QRect::width();
impl<'a> /*trait*/ QRect_width<i32> for () {
  fn width(self, rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect5widthEv()};
    let mut ret = unsafe {_ZNK5QRect5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setRect<RetType, T: QRect_setRect<RetType>>(&mut self, value: T) -> RetType {
    return value.setRect(self);
    // return 1;
  }
}

pub trait QRect_setRect<RetType> {
  fn setRect(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setRect(int x, int y, int w, int h);
impl<'a> /*trait*/ QRect_setRect<()> for (i32, i32, i32, i32) {
  fn setRect(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7setRectEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN5QRect7setRectEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveCenter<RetType, T: QRect_moveCenter<RetType>>(&mut self, value: T) -> RetType {
    return value.moveCenter(self);
    // return 1;
  }
}

pub trait QRect_moveCenter<RetType> {
  fn moveCenter(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::moveCenter(const QPoint & p);
impl<'a> /*trait*/ QRect_moveCenter<()> for (&'a  QPoint) {
  fn moveCenter(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect10moveCenterERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect10moveCenterERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn intersects<RetType, T: QRect_intersects<RetType>>(&mut self, value: T) -> RetType {
    return value.intersects(self);
    // return 1;
  }
}

pub trait QRect_intersects<RetType> {
  fn intersects(self, rsthis: &mut QRect) -> RetType;
}

// proto:  bool QRect::intersects(const QRect & r);
impl<'a> /*trait*/ QRect_intersects<i8> for (&'a  QRect) {
  fn intersects(self, rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setTopRight<RetType, T: QRect_setTopRight<RetType>>(&mut self, value: T) -> RetType {
    return value.setTopRight(self);
    // return 1;
  }
}

pub trait QRect_setTopRight<RetType> {
  fn setTopRight(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setTopRight(const QPoint & p);
impl<'a> /*trait*/ QRect_setTopRight<()> for (&'a  QPoint) {
  fn setTopRight(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect11setTopRightERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect11setTopRightERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setCoords<RetType, T: QRect_setCoords<RetType>>(&mut self, value: T) -> RetType {
    return value.setCoords(self);
    // return 1;
  }
}

pub trait QRect_setCoords<RetType> {
  fn setCoords(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setCoords(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QRect_setCoords<()> for (i32, i32, i32, i32) {
  fn setCoords(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9setCoordsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN5QRect9setCoordsEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QRect::translate(const QPoint & p);
impl<'a> /*trait*/ QRect_translate<()> for (&'a  QPoint) {
  fn translate(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveBottom<RetType, T: QRect_moveBottom<RetType>>(&mut self, value: T) -> RetType {
    return value.moveBottom(self);
    // return 1;
  }
}

pub trait QRect_moveBottom<RetType> {
  fn moveBottom(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::moveBottom(int pos);
impl<'a> /*trait*/ QRect_moveBottom<()> for (i32) {
  fn moveBottom(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect10moveBottomEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect10moveBottomEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setBottomLeft<RetType, T: QRect_setBottomLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.setBottomLeft(self);
    // return 1;
  }
}

pub trait QRect_setBottomLeft<RetType> {
  fn setBottomLeft(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setBottomLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_setBottomLeft<()> for (&'a  QPoint) {
  fn setBottomLeft(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect13setBottomLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect13setBottomLeftERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn getCoords<RetType, T: QRect_getCoords<RetType>>(&mut self, value: T) -> RetType {
    return value.getCoords(self);
    // return 1;
  }
}

pub trait QRect_getCoords<RetType> {
  fn getCoords(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::getCoords(int * x1, int * y1, int * x2, int * y2);
impl<'a> /*trait*/ QRect_getCoords<()> for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getCoords(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect9getCoordsEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
     unsafe {_ZNK5QRect9getCoordsEPiS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn topRight<RetType, T: QRect_topRight<RetType>>(&mut self, value: T) -> RetType {
    return value.topRight(self);
    // return 1;
  }
}

pub trait QRect_topRight<RetType> {
  fn topRight(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QPoint QRect::topRight();
impl<'a> /*trait*/ QRect_topRight<QPoint> for () {
  fn topRight(self, rsthis: &mut QRect) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8topRightEv()};
    let mut ret = unsafe {_ZNK5QRect8topRightEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setBottomRight<RetType, T: QRect_setBottomRight<RetType>>(&mut self, value: T) -> RetType {
    return value.setBottomRight(self);
    // return 1;
  }
}

pub trait QRect_setBottomRight<RetType> {
  fn setBottomRight(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setBottomRight(const QPoint & p);
impl<'a> /*trait*/ QRect_setBottomRight<()> for (&'a  QPoint) {
  fn setBottomRight(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect14setBottomRightERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect14setBottomRightERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setHeight<RetType, T: QRect_setHeight<RetType>>(&mut self, value: T) -> RetType {
    return value.setHeight(self);
    // return 1;
  }
}

pub trait QRect_setHeight<RetType> {
  fn setHeight(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setHeight(int h);
impl<'a> /*trait*/ QRect_setHeight<()> for (i32) {
  fn setHeight(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9setHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect9setHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn isEmpty<RetType, T: QRect_isEmpty<RetType>>(&mut self, value: T) -> RetType {
    return value.isEmpty(self);
    // return 1;
  }
}

pub trait QRect_isEmpty<RetType> {
  fn isEmpty(self, rsthis: &mut QRect) -> RetType;
}

// proto:  bool QRect::isEmpty();
impl<'a> /*trait*/ QRect_isEmpty<i8> for () {
  fn isEmpty(self, rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7isEmptyEv()};
    let mut ret = unsafe {_ZNK5QRect7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  bool QRect::contains(int x, int y);
impl<'a> /*trait*/ QRect_contains<i8> for (i32, i32) {
  fn contains(self, rsthis: &mut QRect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK5QRect8containsEii(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveBottomRight<RetType, T: QRect_moveBottomRight<RetType>>(&mut self, value: T) -> RetType {
    return value.moveBottomRight(self);
    // return 1;
  }
}

pub trait QRect_moveBottomRight<RetType> {
  fn moveBottomRight(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::moveBottomRight(const QPoint & p);
impl<'a> /*trait*/ QRect_moveBottomRight<()> for (&'a  QPoint) {
  fn moveBottomRight(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect15moveBottomRightERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect15moveBottomRightERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn bottomLeft<RetType, T: QRect_bottomLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.bottomLeft(self);
    // return 1;
  }
}

pub trait QRect_bottomLeft<RetType> {
  fn bottomLeft(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QPoint QRect::bottomLeft();
impl<'a> /*trait*/ QRect_bottomLeft<QPoint> for () {
  fn bottomLeft(self, rsthis: &mut QRect) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10bottomLeftEv()};
    let mut ret = unsafe {_ZNK5QRect10bottomLeftEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setTop<RetType, T: QRect_setTop<RetType>>(&mut self, value: T) -> RetType {
    return value.setTop(self);
    // return 1;
  }
}

pub trait QRect_setTop<RetType> {
  fn setTop(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setTop(int pos);
impl<'a> /*trait*/ QRect_setTop<()> for (i32) {
  fn setTop(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6setTopEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect6setTopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn bottom<RetType, T: QRect_bottom<RetType>>(&mut self, value: T) -> RetType {
    return value.bottom(self);
    // return 1;
  }
}

pub trait QRect_bottom<RetType> {
  fn bottom(self, rsthis: &mut QRect) -> RetType;
}

// proto:  int QRect::bottom();
impl<'a> /*trait*/ QRect_bottom<i32> for () {
  fn bottom(self, rsthis: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6bottomEv()};
    let mut ret = unsafe {_ZNK5QRect6bottomEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QRect::NewQRect();
impl<'a> /*trait*/ QRect_NewQRect for () {
  fn NewQRect(self) -> QRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRectC1Ev()};
    unsafe {_ZN5QRectC1Ev(qthis)};
    let rsthis = QRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn marginsAdded<RetType, T: QRect_marginsAdded<RetType>>(&mut self, value: T) -> RetType {
    return value.marginsAdded(self);
    // return 1;
  }
}

pub trait QRect_marginsAdded<RetType> {
  fn marginsAdded(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QRect QRect::marginsAdded(const QMargins & margins);
impl<'a> /*trait*/ QRect_marginsAdded<QRect> for (&'a  QMargins) {
  fn marginsAdded(self, rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect12marginsAddedERK8QMargins()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect12marginsAddedERK8QMargins(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn normalized<RetType, T: QRect_normalized<RetType>>(&mut self, value: T) -> RetType {
    return value.normalized(self);
    // return 1;
  }
}

pub trait QRect_normalized<RetType> {
  fn normalized(self, rsthis: &mut QRect) -> RetType;
}

// proto:  QRect QRect::normalized();
impl<'a> /*trait*/ QRect_normalized<QRect> for () {
  fn normalized(self, rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10normalizedEv()};
    let mut ret = unsafe {_ZNK5QRect10normalizedEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setWidth<RetType, T: QRect_setWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.setWidth(self);
    // return 1;
  }
}

pub trait QRect_setWidth<RetType> {
  fn setWidth(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setWidth(int w);
impl<'a> /*trait*/ QRect_setWidth<()> for (i32) {
  fn setWidth(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect8setWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect8setWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setY<RetType, T: QRect_setY<RetType>>(&mut self, value: T) -> RetType {
    return value.setY(self);
    // return 1;
  }
}

pub trait QRect_setY<RetType> {
  fn setY(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setY(int y);
impl<'a> /*trait*/ QRect_setY<()> for (i32) {
  fn setY(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect4setYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect4setYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveTop<RetType, T: QRect_moveTop<RetType>>(&mut self, value: T) -> RetType {
    return value.moveTop(self);
    // return 1;
  }
}

pub trait QRect_moveTop<RetType> {
  fn moveTop(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::moveTop(int pos);
impl<'a> /*trait*/ QRect_moveTop<()> for (i32) {
  fn moveTop(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7moveTopEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect7moveTopEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setX<RetType, T: QRect_setX<RetType>>(&mut self, value: T) -> RetType {
    return value.setX(self);
    // return 1;
  }
}

pub trait QRect_setX<RetType> {
  fn setX(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setX(int x);
impl<'a> /*trait*/ QRect_setX<()> for (i32) {
  fn setX(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect4setXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect4setXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setRight<RetType, T: QRect_setRight<RetType>>(&mut self, value: T) -> RetType {
    return value.setRight(self);
    // return 1;
  }
}

pub trait QRect_setRight<RetType> {
  fn setRight(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setRight(int pos);
impl<'a> /*trait*/ QRect_setRight<()> for (i32) {
  fn setRight(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect8setRightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect8setRightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setTopLeft<RetType, T: QRect_setTopLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.setTopLeft(self);
    // return 1;
  }
}

pub trait QRect_setTopLeft<RetType> {
  fn setTopLeft(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::setTopLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_setTopLeft<()> for (&'a  QPoint) {
  fn setTopLeft(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect10setTopLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QRect10setTopLeftERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveLeft<RetType, T: QRect_moveLeft<RetType>>(&mut self, value: T) -> RetType {
    return value.moveLeft(self);
    // return 1;
  }
}

pub trait QRect_moveLeft<RetType> {
  fn moveLeft(self, rsthis: &mut QRect) -> RetType;
}

// proto:  void QRect::moveLeft(int pos);
impl<'a> /*trait*/ QRect_moveLeft<()> for (i32) {
  fn moveLeft(self, rsthis: &mut QRect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect8moveLeftEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QRect8moveLeftEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QRect QRect::translated(const QPoint & p);
impl<'a> /*trait*/ QRect_translated<QRect> for (&'a  QPoint) {
  fn translated(self, rsthis: &mut QRect) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QRect10translatedERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

