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
  fn _ZNK5QRect5rightEv() -> i32;
  fn _ZN5QRect6moveToERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZN5QRect11moveTopLeftERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZN5QRect9moveRightEi(arg0: c_int) -> i32;
  fn _ZN5QRectC1ERK6QPointS2_(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK5QRect10translatedEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK5QRect6centerEv() -> i32;
  fn _ZN5QRect12moveTopRightERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZN5QRect7setLeftEi(arg0: c_int) -> i32;
  fn _ZNK5QRect4leftEv() -> i32;
  fn _ZNK5QRect11intersectedERKS_(arg0: *const c_void) -> i32;
  fn _ZNK5QRect8containsEiib(arg0: c_int, arg1: c_int, arg2: int8_t) -> i32;
  fn _ZNK5QRect11bottomRightEv() -> i32;
  fn _ZNK5QRect7isValidEv() -> i32;
  fn _ZNK5QRect4sizeEv() -> i32;
  fn _ZNK5QRect6unitedERKS_(arg0: *const c_void) -> i32;
  fn _ZN5QRect6adjustEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZNK5QRect6isNullEv() -> i32;
  fn _ZN5QRect9setBottomEi(arg0: c_int) -> i32;
  fn _ZN5QRect7setSizeERK5QSize(arg0: *const c_void) -> i32;
  fn _ZNK5QRect1yEv() -> i32;
  fn _ZNK5QRect1xEv() -> i32;
  fn _ZNK5QRect8adjustedEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZN5QRectC1ERK6QPointRK5QSize(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void) -> i32;
  fn _ZNK5QRect6heightEv() -> i32;
  fn _ZN5QRectC1Eiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZN5QRect14moveBottomLeftERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZNK5QRect3topEv() -> i32;
  fn _ZN5QRect6moveToEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK5QRect7getRectEPiS0_S0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  fn _ZNK5QRect8containsERKS_b(arg0: *const c_void, arg1: int8_t) -> i32;
  fn _ZNK5QRect14marginsRemovedERK8QMargins(arg0: *const c_void) -> i32;
  fn _ZN5QRect9translateEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZNK5QRect7topLeftEv() -> i32;
  fn _ZNK5QRect8containsERK6QPointb(arg0: *const c_void, arg1: int8_t) -> i32;
  fn _ZNK5QRect5widthEv() -> i32;
  fn _ZN5QRect7setRectEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZN5QRect10moveCenterERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZNK5QRect10intersectsERKS_(arg0: *const c_void) -> i32;
  fn _ZN5QRect11setTopRightERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZN5QRect9setCoordsEiiii(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> i32;
  fn _ZN5QRect9translateERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZN5QRect10moveBottomEi(arg0: c_int) -> i32;
  fn _ZN5QRect13setBottomLeftERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZNK5QRect9getCoordsEPiS0_S0_S0_(arg0: *mut c_int, arg1: *mut c_int, arg2: *mut c_int, arg3: *mut c_int) -> i32;
  fn _ZNK5QRect8topRightEv() -> i32;
  fn _ZN5QRect14setBottomRightERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZN5QRect9setHeightEi(arg0: c_int) -> i32;
  fn _ZNK5QRect7isEmptyEv() -> i32;
  fn _ZNK5QRect8containsEii(arg0: c_int, arg1: c_int) -> i32;
  fn _ZN5QRect15moveBottomRightERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZNK5QRect10bottomLeftEv() -> i32;
  fn _ZN5QRect6setTopEi(arg0: c_int) -> i32;
  fn _ZNK5QRect6bottomEv() -> i32;
  fn _ZN5QRectC1Ev(qthis: *mut c_void) -> i32;
  fn _ZNK5QRect12marginsAddedERK8QMargins(arg0: *const c_void) -> i32;
  fn _ZNK5QRect10normalizedEv() -> i32;
  fn _ZN5QRect8setWidthEi(arg0: c_int) -> i32;
  fn _ZN5QRect4setYEi(arg0: c_int) -> i32;
  fn _ZN5QRect7moveTopEi(arg0: c_int) -> i32;
  fn _ZN5QRect4setXEi(arg0: c_int) -> i32;
  fn _ZN5QRect8setRightEi(arg0: c_int) -> i32;
  fn _ZN5QRect10setTopLeftERK6QPoint(arg0: *const c_void) -> i32;
  fn _ZN5QRect8moveLeftEi(arg0: c_int) -> i32;
  fn _ZNK5QRect10translatedERK6QPoint(arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QRect)=16
pub struct QRect {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRect {
  pub fn right<T: QRect_right>(&mut self, value: T) -> i32 {
    value.right(self);
    return 1;
  }
}

pub trait QRect_right {
  fn right(self, this: &mut QRect) -> i32;
}

// proto: int QRect::right();
impl<'a> /*trait*/ QRect_right for () {
  fn right(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect5rightEv()};
    unsafe {_ZNK5QRect5rightEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveTo<T: QRect_moveTo>(&mut self, value: T) -> i32 {
    value.moveTo(self);
    return 1;
  }
}

pub trait QRect_moveTo {
  fn moveTo(self, this: &mut QRect) -> i32;
}

// proto: void QRect::moveTo(const QPoint & p);
impl<'a> /*trait*/ QRect_moveTo for (&'a  QPoint) {
  fn moveTo(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6moveToERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect6moveToERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveTopLeft<T: QRect_moveTopLeft>(&mut self, value: T) -> i32 {
    value.moveTopLeft(self);
    return 1;
  }
}

pub trait QRect_moveTopLeft {
  fn moveTopLeft(self, this: &mut QRect) -> i32;
}

// proto: void QRect::moveTopLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_moveTopLeft for (&'a  QPoint) {
  fn moveTopLeft(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect11moveTopLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect11moveTopLeftERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveRight<T: QRect_moveRight>(&mut self, value: T) -> i32 {
    value.moveRight(self);
    return 1;
  }
}

pub trait QRect_moveRight {
  fn moveRight(self, this: &mut QRect) -> i32;
}

// proto: void QRect::moveRight(int pos);
impl<'a> /*trait*/ QRect_moveRight for (i32) {
  fn moveRight(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9moveRightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect9moveRightEi(arg0)};
    return 1;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QRectC1ERK6QPointS2_(qthis, arg0, arg1)};
    let rsthis = QRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn translated<T: QRect_translated>(&mut self, value: T) -> i32 {
    value.translated(self);
    return 1;
  }
}

pub trait QRect_translated {
  fn translated(self, this: &mut QRect) -> i32;
}

// proto: QRect QRect::translated(int dx, int dy);
impl<'a> /*trait*/ QRect_translated for (i32, i32) {
  fn translated(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK5QRect10translatedEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn center<T: QRect_center>(&mut self, value: T) -> i32 {
    value.center(self);
    return 1;
  }
}

pub trait QRect_center {
  fn center(self, this: &mut QRect) -> i32;
}

// proto: QPoint QRect::center();
impl<'a> /*trait*/ QRect_center for () {
  fn center(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6centerEv()};
    unsafe {_ZNK5QRect6centerEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveTopRight<T: QRect_moveTopRight>(&mut self, value: T) -> i32 {
    value.moveTopRight(self);
    return 1;
  }
}

pub trait QRect_moveTopRight {
  fn moveTopRight(self, this: &mut QRect) -> i32;
}

// proto: void QRect::moveTopRight(const QPoint & p);
impl<'a> /*trait*/ QRect_moveTopRight for (&'a  QPoint) {
  fn moveTopRight(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect12moveTopRightERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect12moveTopRightERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setLeft<T: QRect_setLeft>(&mut self, value: T) -> i32 {
    value.setLeft(self);
    return 1;
  }
}

pub trait QRect_setLeft {
  fn setLeft(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setLeft(int pos);
impl<'a> /*trait*/ QRect_setLeft for (i32) {
  fn setLeft(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7setLeftEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect7setLeftEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn left<T: QRect_left>(&mut self, value: T) -> i32 {
    value.left(self);
    return 1;
  }
}

pub trait QRect_left {
  fn left(self, this: &mut QRect) -> i32;
}

// proto: int QRect::left();
impl<'a> /*trait*/ QRect_left for () {
  fn left(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect4leftEv()};
    unsafe {_ZNK5QRect4leftEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn intersected<T: QRect_intersected>(&mut self, value: T) -> i32 {
    value.intersected(self);
    return 1;
  }
}

pub trait QRect_intersected {
  fn intersected(self, this: &mut QRect) -> i32;
}

// proto: QRect QRect::intersected(const QRect & other);
impl<'a> /*trait*/ QRect_intersected for (&'a  QRect) {
  fn intersected(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect11intersectedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QRect11intersectedERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn contains<T: QRect_contains>(&mut self, value: T) -> i32 {
    value.contains(self);
    return 1;
  }
}

pub trait QRect_contains {
  fn contains(self, this: &mut QRect) -> i32;
}

// proto: bool QRect::contains(int x, int y, bool proper);
impl<'a> /*trait*/ QRect_contains for (i32, i32, i8) {
  fn contains(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsEiib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as int8_t;
    unsafe {_ZNK5QRect8containsEiib(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn bottomRight<T: QRect_bottomRight>(&mut self, value: T) -> i32 {
    value.bottomRight(self);
    return 1;
  }
}

pub trait QRect_bottomRight {
  fn bottomRight(self, this: &mut QRect) -> i32;
}

// proto: QPoint QRect::bottomRight();
impl<'a> /*trait*/ QRect_bottomRight for () {
  fn bottomRight(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect11bottomRightEv()};
    unsafe {_ZNK5QRect11bottomRightEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn isValid<T: QRect_isValid>(&mut self, value: T) -> i32 {
    value.isValid(self);
    return 1;
  }
}

pub trait QRect_isValid {
  fn isValid(self, this: &mut QRect) -> i32;
}

// proto: bool QRect::isValid();
impl<'a> /*trait*/ QRect_isValid for () {
  fn isValid(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7isValidEv()};
    unsafe {_ZNK5QRect7isValidEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn size<T: QRect_size>(&mut self, value: T) -> i32 {
    value.size(self);
    return 1;
  }
}

pub trait QRect_size {
  fn size(self, this: &mut QRect) -> i32;
}

// proto: QSize QRect::size();
impl<'a> /*trait*/ QRect_size for () {
  fn size(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect4sizeEv()};
    unsafe {_ZNK5QRect4sizeEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn united<T: QRect_united>(&mut self, value: T) -> i32 {
    value.united(self);
    return 1;
  }
}

pub trait QRect_united {
  fn united(self, this: &mut QRect) -> i32;
}

// proto: QRect QRect::united(const QRect & other);
impl<'a> /*trait*/ QRect_united for (&'a  QRect) {
  fn united(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6unitedERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QRect6unitedERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn adjust<T: QRect_adjust>(&mut self, value: T) -> i32 {
    value.adjust(self);
    return 1;
  }
}

pub trait QRect_adjust {
  fn adjust(self, this: &mut QRect) -> i32;
}

// proto: void QRect::adjust(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QRect_adjust for (i32, i32, i32, i32) {
  fn adjust(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6adjustEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QRect6adjustEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn isNull<T: QRect_isNull>(&mut self, value: T) -> i32 {
    value.isNull(self);
    return 1;
  }
}

pub trait QRect_isNull {
  fn isNull(self, this: &mut QRect) -> i32;
}

// proto: bool QRect::isNull();
impl<'a> /*trait*/ QRect_isNull for () {
  fn isNull(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6isNullEv()};
    unsafe {_ZNK5QRect6isNullEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setBottom<T: QRect_setBottom>(&mut self, value: T) -> i32 {
    value.setBottom(self);
    return 1;
  }
}

pub trait QRect_setBottom {
  fn setBottom(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setBottom(int pos);
impl<'a> /*trait*/ QRect_setBottom for (i32) {
  fn setBottom(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9setBottomEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect9setBottomEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setSize<T: QRect_setSize>(&mut self, value: T) -> i32 {
    value.setSize(self);
    return 1;
  }
}

pub trait QRect_setSize {
  fn setSize(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setSize(const QSize & s);
impl<'a> /*trait*/ QRect_setSize for (&'a  QSize) {
  fn setSize(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7setSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect7setSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn y<T: QRect_y>(&mut self, value: T) -> i32 {
    value.y(self);
    return 1;
  }
}

pub trait QRect_y {
  fn y(self, this: &mut QRect) -> i32;
}

// proto: int QRect::y();
impl<'a> /*trait*/ QRect_y for () {
  fn y(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect1yEv()};
    unsafe {_ZNK5QRect1yEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn x<T: QRect_x>(&mut self, value: T) -> i32 {
    value.x(self);
    return 1;
  }
}

pub trait QRect_x {
  fn x(self, this: &mut QRect) -> i32;
}

// proto: int QRect::x();
impl<'a> /*trait*/ QRect_x for () {
  fn x(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect1xEv()};
    unsafe {_ZNK5QRect1xEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn adjusted<T: QRect_adjusted>(&mut self, value: T) -> i32 {
    value.adjusted(self);
    return 1;
  }
}

pub trait QRect_adjusted {
  fn adjusted(self, this: &mut QRect) -> i32;
}

// proto: QRect QRect::adjusted(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QRect_adjusted for (i32, i32, i32, i32) {
  fn adjusted(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8adjustedEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZNK5QRect8adjustedEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QRect::NewQRect(const QPoint & topleft, const QSize & size);
impl<'a> /*trait*/ QRect_NewQRect for (&'a  QPoint, &'a  QSize) {
  fn NewQRect(self) -> QRect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRectC1ERK6QPointRK5QSize()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN5QRectC1ERK6QPointRK5QSize(qthis, arg0, arg1)};
    let rsthis = QRect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QRect {
  pub fn height<T: QRect_height>(&mut self, value: T) -> i32 {
    value.height(self);
    return 1;
  }
}

pub trait QRect_height {
  fn height(self, this: &mut QRect) -> i32;
}

// proto: int QRect::height();
impl<'a> /*trait*/ QRect_height for () {
  fn height(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6heightEv()};
    unsafe {_ZNK5QRect6heightEv()};
    return 1;
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
  pub fn moveBottomLeft<T: QRect_moveBottomLeft>(&mut self, value: T) -> i32 {
    value.moveBottomLeft(self);
    return 1;
  }
}

pub trait QRect_moveBottomLeft {
  fn moveBottomLeft(self, this: &mut QRect) -> i32;
}

// proto: void QRect::moveBottomLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_moveBottomLeft for (&'a  QPoint) {
  fn moveBottomLeft(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect14moveBottomLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect14moveBottomLeftERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn top<T: QRect_top>(&mut self, value: T) -> i32 {
    value.top(self);
    return 1;
  }
}

pub trait QRect_top {
  fn top(self, this: &mut QRect) -> i32;
}

// proto: int QRect::top();
impl<'a> /*trait*/ QRect_top for () {
  fn top(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect3topEv()};
    unsafe {_ZNK5QRect3topEv()};
    return 1;
  }
}

// proto: void QRect::moveTo(int x, int t);
impl<'a> /*trait*/ QRect_moveTo for (i32, i32) {
  fn moveTo(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6moveToEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN5QRect6moveToEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn getRect<T: QRect_getRect>(&mut self, value: T) -> i32 {
    value.getRect(self);
    return 1;
  }
}

pub trait QRect_getRect {
  fn getRect(self, this: &mut QRect) -> i32;
}

// proto: void QRect::getRect(int * x, int * y, int * w, int * h);
impl<'a> /*trait*/ QRect_getRect for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getRect(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7getRectEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK5QRect7getRectEPiS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: bool QRect::contains(const QRect & r, bool proper);
impl<'a> /*trait*/ QRect_contains for (&'a  QRect, i8) {
  fn contains(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsERKS_b()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZNK5QRect8containsERKS_b(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn marginsRemoved<T: QRect_marginsRemoved>(&mut self, value: T) -> i32 {
    value.marginsRemoved(self);
    return 1;
  }
}

pub trait QRect_marginsRemoved {
  fn marginsRemoved(self, this: &mut QRect) -> i32;
}

// proto: QRect QRect::marginsRemoved(const QMargins & margins);
impl<'a> /*trait*/ QRect_marginsRemoved for (&'a  QMargins) {
  fn marginsRemoved(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect14marginsRemovedERK8QMargins()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QRect14marginsRemovedERK8QMargins(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn translate<T: QRect_translate>(&mut self, value: T) -> i32 {
    value.translate(self);
    return 1;
  }
}

pub trait QRect_translate {
  fn translate(self, this: &mut QRect) -> i32;
}

// proto: void QRect::translate(int dx, int dy);
impl<'a> /*trait*/ QRect_translate for (i32, i32) {
  fn translate(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN5QRect9translateEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn topLeft<T: QRect_topLeft>(&mut self, value: T) -> i32 {
    value.topLeft(self);
    return 1;
  }
}

pub trait QRect_topLeft {
  fn topLeft(self, this: &mut QRect) -> i32;
}

// proto: QPoint QRect::topLeft();
impl<'a> /*trait*/ QRect_topLeft for () {
  fn topLeft(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7topLeftEv()};
    unsafe {_ZNK5QRect7topLeftEv()};
    return 1;
  }
}

// proto: bool QRect::contains(const QPoint & p, bool proper);
impl<'a> /*trait*/ QRect_contains for (&'a  QPoint, i8) {
  fn contains(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsERK6QPointb()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as int8_t;
    unsafe {_ZNK5QRect8containsERK6QPointb(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn width<T: QRect_width>(&mut self, value: T) -> i32 {
    value.width(self);
    return 1;
  }
}

pub trait QRect_width {
  fn width(self, this: &mut QRect) -> i32;
}

// proto: int QRect::width();
impl<'a> /*trait*/ QRect_width for () {
  fn width(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect5widthEv()};
    unsafe {_ZNK5QRect5widthEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setRect<T: QRect_setRect>(&mut self, value: T) -> i32 {
    value.setRect(self);
    return 1;
  }
}

pub trait QRect_setRect {
  fn setRect(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setRect(int x, int y, int w, int h);
impl<'a> /*trait*/ QRect_setRect for (i32, i32, i32, i32) {
  fn setRect(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7setRectEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QRect7setRectEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveCenter<T: QRect_moveCenter>(&mut self, value: T) -> i32 {
    value.moveCenter(self);
    return 1;
  }
}

pub trait QRect_moveCenter {
  fn moveCenter(self, this: &mut QRect) -> i32;
}

// proto: void QRect::moveCenter(const QPoint & p);
impl<'a> /*trait*/ QRect_moveCenter for (&'a  QPoint) {
  fn moveCenter(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect10moveCenterERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect10moveCenterERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn intersects<T: QRect_intersects>(&mut self, value: T) -> i32 {
    value.intersects(self);
    return 1;
  }
}

pub trait QRect_intersects {
  fn intersects(self, this: &mut QRect) -> i32;
}

// proto: bool QRect::intersects(const QRect & r);
impl<'a> /*trait*/ QRect_intersects for (&'a  QRect) {
  fn intersects(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10intersectsERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QRect10intersectsERKS_(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setTopRight<T: QRect_setTopRight>(&mut self, value: T) -> i32 {
    value.setTopRight(self);
    return 1;
  }
}

pub trait QRect_setTopRight {
  fn setTopRight(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setTopRight(const QPoint & p);
impl<'a> /*trait*/ QRect_setTopRight for (&'a  QPoint) {
  fn setTopRight(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect11setTopRightERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect11setTopRightERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setCoords<T: QRect_setCoords>(&mut self, value: T) -> i32 {
    value.setCoords(self);
    return 1;
  }
}

pub trait QRect_setCoords {
  fn setCoords(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setCoords(int x1, int y1, int x2, int y2);
impl<'a> /*trait*/ QRect_setCoords for (i32, i32, i32, i32) {
  fn setCoords(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9setCoordsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN5QRect9setCoordsEiiii(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: void QRect::translate(const QPoint & p);
impl<'a> /*trait*/ QRect_translate for (&'a  QPoint) {
  fn translate(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect9translateERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveBottom<T: QRect_moveBottom>(&mut self, value: T) -> i32 {
    value.moveBottom(self);
    return 1;
  }
}

pub trait QRect_moveBottom {
  fn moveBottom(self, this: &mut QRect) -> i32;
}

// proto: void QRect::moveBottom(int pos);
impl<'a> /*trait*/ QRect_moveBottom for (i32) {
  fn moveBottom(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect10moveBottomEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect10moveBottomEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setBottomLeft<T: QRect_setBottomLeft>(&mut self, value: T) -> i32 {
    value.setBottomLeft(self);
    return 1;
  }
}

pub trait QRect_setBottomLeft {
  fn setBottomLeft(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setBottomLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_setBottomLeft for (&'a  QPoint) {
  fn setBottomLeft(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect13setBottomLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect13setBottomLeftERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn getCoords<T: QRect_getCoords>(&mut self, value: T) -> i32 {
    value.getCoords(self);
    return 1;
  }
}

pub trait QRect_getCoords {
  fn getCoords(self, this: &mut QRect) -> i32;
}

// proto: void QRect::getCoords(int * x1, int * y1, int * x2, int * y2);
impl<'a> /*trait*/ QRect_getCoords for (&'a mut i32, &'a mut i32, &'a mut i32, &'a mut i32) {
  fn getCoords(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect9getCoordsEPiS0_S0_S0_()};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1  as *mut c_int;
    let arg2 = self.2  as *mut c_int;
    let arg3 = self.3  as *mut c_int;
    unsafe {_ZNK5QRect9getCoordsEPiS0_S0_S0_(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn topRight<T: QRect_topRight>(&mut self, value: T) -> i32 {
    value.topRight(self);
    return 1;
  }
}

pub trait QRect_topRight {
  fn topRight(self, this: &mut QRect) -> i32;
}

// proto: QPoint QRect::topRight();
impl<'a> /*trait*/ QRect_topRight for () {
  fn topRight(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8topRightEv()};
    unsafe {_ZNK5QRect8topRightEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setBottomRight<T: QRect_setBottomRight>(&mut self, value: T) -> i32 {
    value.setBottomRight(self);
    return 1;
  }
}

pub trait QRect_setBottomRight {
  fn setBottomRight(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setBottomRight(const QPoint & p);
impl<'a> /*trait*/ QRect_setBottomRight for (&'a  QPoint) {
  fn setBottomRight(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect14setBottomRightERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect14setBottomRightERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setHeight<T: QRect_setHeight>(&mut self, value: T) -> i32 {
    value.setHeight(self);
    return 1;
  }
}

pub trait QRect_setHeight {
  fn setHeight(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setHeight(int h);
impl<'a> /*trait*/ QRect_setHeight for (i32) {
  fn setHeight(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect9setHeightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect9setHeightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn isEmpty<T: QRect_isEmpty>(&mut self, value: T) -> i32 {
    value.isEmpty(self);
    return 1;
  }
}

pub trait QRect_isEmpty {
  fn isEmpty(self, this: &mut QRect) -> i32;
}

// proto: bool QRect::isEmpty();
impl<'a> /*trait*/ QRect_isEmpty for () {
  fn isEmpty(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect7isEmptyEv()};
    unsafe {_ZNK5QRect7isEmptyEv()};
    return 1;
  }
}

// proto: bool QRect::contains(int x, int y);
impl<'a> /*trait*/ QRect_contains for (i32, i32) {
  fn contains(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect8containsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZNK5QRect8containsEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveBottomRight<T: QRect_moveBottomRight>(&mut self, value: T) -> i32 {
    value.moveBottomRight(self);
    return 1;
  }
}

pub trait QRect_moveBottomRight {
  fn moveBottomRight(self, this: &mut QRect) -> i32;
}

// proto: void QRect::moveBottomRight(const QPoint & p);
impl<'a> /*trait*/ QRect_moveBottomRight for (&'a  QPoint) {
  fn moveBottomRight(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect15moveBottomRightERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect15moveBottomRightERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn bottomLeft<T: QRect_bottomLeft>(&mut self, value: T) -> i32 {
    value.bottomLeft(self);
    return 1;
  }
}

pub trait QRect_bottomLeft {
  fn bottomLeft(self, this: &mut QRect) -> i32;
}

// proto: QPoint QRect::bottomLeft();
impl<'a> /*trait*/ QRect_bottomLeft for () {
  fn bottomLeft(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10bottomLeftEv()};
    unsafe {_ZNK5QRect10bottomLeftEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setTop<T: QRect_setTop>(&mut self, value: T) -> i32 {
    value.setTop(self);
    return 1;
  }
}

pub trait QRect_setTop {
  fn setTop(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setTop(int pos);
impl<'a> /*trait*/ QRect_setTop for (i32) {
  fn setTop(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect6setTopEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect6setTopEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn bottom<T: QRect_bottom>(&mut self, value: T) -> i32 {
    value.bottom(self);
    return 1;
  }
}

pub trait QRect_bottom {
  fn bottom(self, this: &mut QRect) -> i32;
}

// proto: int QRect::bottom();
impl<'a> /*trait*/ QRect_bottom for () {
  fn bottom(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect6bottomEv()};
    unsafe {_ZNK5QRect6bottomEv()};
    return 1;
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
  pub fn marginsAdded<T: QRect_marginsAdded>(&mut self, value: T) -> i32 {
    value.marginsAdded(self);
    return 1;
  }
}

pub trait QRect_marginsAdded {
  fn marginsAdded(self, this: &mut QRect) -> i32;
}

// proto: QRect QRect::marginsAdded(const QMargins & margins);
impl<'a> /*trait*/ QRect_marginsAdded for (&'a  QMargins) {
  fn marginsAdded(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect12marginsAddedERK8QMargins()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QRect12marginsAddedERK8QMargins(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn normalized<T: QRect_normalized>(&mut self, value: T) -> i32 {
    value.normalized(self);
    return 1;
  }
}

pub trait QRect_normalized {
  fn normalized(self, this: &mut QRect) -> i32;
}

// proto: QRect QRect::normalized();
impl<'a> /*trait*/ QRect_normalized for () {
  fn normalized(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10normalizedEv()};
    unsafe {_ZNK5QRect10normalizedEv()};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setWidth<T: QRect_setWidth>(&mut self, value: T) -> i32 {
    value.setWidth(self);
    return 1;
  }
}

pub trait QRect_setWidth {
  fn setWidth(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setWidth(int w);
impl<'a> /*trait*/ QRect_setWidth for (i32) {
  fn setWidth(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect8setWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect8setWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setY<T: QRect_setY>(&mut self, value: T) -> i32 {
    value.setY(self);
    return 1;
  }
}

pub trait QRect_setY {
  fn setY(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setY(int y);
impl<'a> /*trait*/ QRect_setY for (i32) {
  fn setY(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect4setYEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect4setYEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveTop<T: QRect_moveTop>(&mut self, value: T) -> i32 {
    value.moveTop(self);
    return 1;
  }
}

pub trait QRect_moveTop {
  fn moveTop(self, this: &mut QRect) -> i32;
}

// proto: void QRect::moveTop(int pos);
impl<'a> /*trait*/ QRect_moveTop for (i32) {
  fn moveTop(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect7moveTopEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect7moveTopEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setX<T: QRect_setX>(&mut self, value: T) -> i32 {
    value.setX(self);
    return 1;
  }
}

pub trait QRect_setX {
  fn setX(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setX(int x);
impl<'a> /*trait*/ QRect_setX for (i32) {
  fn setX(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect4setXEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect4setXEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setRight<T: QRect_setRight>(&mut self, value: T) -> i32 {
    value.setRight(self);
    return 1;
  }
}

pub trait QRect_setRight {
  fn setRight(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setRight(int pos);
impl<'a> /*trait*/ QRect_setRight for (i32) {
  fn setRight(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect8setRightEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect8setRightEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn setTopLeft<T: QRect_setTopLeft>(&mut self, value: T) -> i32 {
    value.setTopLeft(self);
    return 1;
  }
}

pub trait QRect_setTopLeft {
  fn setTopLeft(self, this: &mut QRect) -> i32;
}

// proto: void QRect::setTopLeft(const QPoint & p);
impl<'a> /*trait*/ QRect_setTopLeft for (&'a  QPoint) {
  fn setTopLeft(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect10setTopLeftERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN5QRect10setTopLeftERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QRect {
  pub fn moveLeft<T: QRect_moveLeft>(&mut self, value: T) -> i32 {
    value.moveLeft(self);
    return 1;
  }
}

pub trait QRect_moveLeft {
  fn moveLeft(self, this: &mut QRect) -> i32;
}

// proto: void QRect::moveLeft(int pos);
impl<'a> /*trait*/ QRect_moveLeft for (i32) {
  fn moveLeft(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QRect8moveLeftEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN5QRect8moveLeftEi(arg0)};
    return 1;
  }
}

// proto: QRect QRect::translated(const QPoint & p);
impl<'a> /*trait*/ QRect_translated for (&'a  QPoint) {
  fn translated(self, this: &mut QRect) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QRect10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK5QRect10translatedERK6QPoint(arg0)};
    return 1;
  }
}

