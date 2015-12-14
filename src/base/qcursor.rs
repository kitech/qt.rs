// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qscreen::QScreen;
use super::qpixmap::QPixmap;
use super::qbitmap::QBitmap;
use super::qpoint::QPoint;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static void QCursor::setPos(QScreen * screen, int x, int y);
  fn _ZN7QCursor6setPosEP7QScreenii(arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  QPixmap QCursor::pixmap();
  fn _ZNK7QCursor6pixmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCursor::NewQCursor(const QBitmap & bitmap, const QBitmap & mask, int hotX, int hotY);
  fn _ZN7QCursorC1ERK7QBitmapS2_ii(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int) ;
  // proto:  void QCursor::NewQCursor(const QPixmap & pixmap, int hotX, int hotY);
  fn _ZN7QCursorC1ERK7QPixmapii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QCursor::FreeQCursor();
  fn _ZN7QCursorD0Ev(qthis: *mut c_void) ;
  // proto:  const QBitmap * QCursor::mask();
  fn _ZNK7QCursor4maskEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCursor::NewQCursor(const QCursor & cursor);
  fn _ZN7QCursorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static void QCursor::setPos(int x, int y);
  fn _ZN7QCursor6setPosEii(arg0: c_int, arg1: c_int) ;
  // proto: static void QCursor::setPos(QScreen * screen, const QPoint & p);
  fn _ZN7QCursor6setPosEP7QScreenRK6QPoint(arg0: *mut c_void, arg1: *mut c_void) ;
  // proto: static void QCursor::setPos(const QPoint & p);
  fn _ZN7QCursor6setPosERK6QPoint(arg0: *mut c_void) ;
  // proto:  const QBitmap * QCursor::bitmap();
  fn _ZNK7QCursor6bitmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QPoint QCursor::pos(const QScreen * screen);
  fn _ZN7QCursor3posEPK7QScreen(arg0: *mut c_void) -> *mut c_void;
  // proto: static QPoint QCursor::pos();
  fn _ZN7QCursor3posEv() -> *mut c_void;
  // proto:  void QCursor::NewQCursor();
  fn _ZN7QCursorC1Ev(qthis: *mut c_void) ;
  // proto:  QPoint QCursor::hotSpot();
  fn _ZNK7QCursor7hotSpotEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QCursor)=8
pub struct QCursor {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCursor {
  pub fn setPos<T: QCursor_setPos>(&mut self, value: T)  {
     value.setPos(self);
    // return 1;
  }
}

pub trait QCursor_setPos {
  fn setPos(self, rsthis: &mut QCursor) ;
}

// proto: static void QCursor::setPos(QScreen * screen, int x, int y);
impl<'a> /*trait*/ QCursor_setPos for (&'a mut QScreen, i32, i32) {
  fn setPos(self, rsthis: &mut QCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosEP7QScreenii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN7QCursor6setPosEP7QScreenii(arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QCursor {
  pub fn pixmap<T: QCursor_pixmap>(&mut self, value: T) -> QPixmap {
    return value.pixmap(self);
    // return 1;
  }
}

pub trait QCursor_pixmap {
  fn pixmap(self, rsthis: &mut QCursor) -> QPixmap;
}

// proto:  QPixmap QCursor::pixmap();
impl<'a> /*trait*/ QCursor_pixmap for () {
  fn pixmap(self, rsthis: &mut QCursor) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor6pixmapEv()};
    let mut ret = unsafe {_ZNK7QCursor6pixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCursor {
  pub fn NewQCursor<T: QCursor_NewQCursor>(value: T) -> QCursor {
    let rsthis = value.NewQCursor();
    return rsthis;
    // return 1;
  }
}

pub trait QCursor_NewQCursor {
  fn NewQCursor(self) -> QCursor;
}

// proto: void QCursor::NewQCursor(const QBitmap & bitmap, const QBitmap & mask, int hotX, int hotY);
impl<'a> /*trait*/ QCursor_NewQCursor for (&'a  QBitmap, &'a  QBitmap, i32, i32) {
  fn NewQCursor(self) -> QCursor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorC1ERK7QBitmapS2_ii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    unsafe {_ZN7QCursorC1ERK7QBitmapS2_ii(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QCursor::NewQCursor(const QPixmap & pixmap, int hotX, int hotY);
impl<'a> /*trait*/ QCursor_NewQCursor for (&'a  QPixmap, i32, i32) {
  fn NewQCursor(self) -> QCursor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorC1ERK7QPixmapii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    unsafe {_ZN7QCursorC1ERK7QPixmapii(qthis, arg0, arg1, arg2)};
    let rsthis = QCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCursor {
  pub fn FreeQCursor<T: QCursor_FreeQCursor>(&mut self, value: T)  {
     value.FreeQCursor(self);
    // return 1;
  }
}

pub trait QCursor_FreeQCursor {
  fn FreeQCursor(self, rsthis: &mut QCursor) ;
}

// proto:  void QCursor::FreeQCursor();
impl<'a> /*trait*/ QCursor_FreeQCursor for () {
  fn FreeQCursor(self, rsthis: &mut QCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorD0Ev()};
     unsafe {_ZN7QCursorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCursor {
  pub fn mask<T: QCursor_mask>(&mut self, value: T) -> QBitmap {
    return value.mask(self);
    // return 1;
  }
}

pub trait QCursor_mask {
  fn mask(self, rsthis: &mut QCursor) -> QBitmap;
}

// proto:  const QBitmap * QCursor::mask();
impl<'a> /*trait*/ QCursor_mask for () {
  fn mask(self, rsthis: &mut QCursor) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor4maskEv()};
    let mut ret = unsafe {_ZNK7QCursor4maskEv(rsthis.qclsinst)};
    let mut ret1 = QBitmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QCursor::NewQCursor(const QCursor & cursor);
impl<'a> /*trait*/ QCursor_NewQCursor for (&'a  QCursor) {
  fn NewQCursor(self) -> QCursor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QCursorC1ERKS_(qthis, arg0)};
    let rsthis = QCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static void QCursor::setPos(int x, int y);
impl<'a> /*trait*/ QCursor_setPos for (i32, i32) {
  fn setPos(self, rsthis: &mut QCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QCursor6setPosEii(arg0, arg1)};
    // return 1;
  }
}

// proto: static void QCursor::setPos(QScreen * screen, const QPoint & p);
impl<'a> /*trait*/ QCursor_setPos for (&'a mut QScreen, &'a  QPoint) {
  fn setPos(self, rsthis: &mut QCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosEP7QScreenRK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QCursor6setPosEP7QScreenRK6QPoint(arg0, arg1)};
    // return 1;
  }
}

// proto: static void QCursor::setPos(const QPoint & p);
impl<'a> /*trait*/ QCursor_setPos for (&'a  QPoint) {
  fn setPos(self, rsthis: &mut QCursor)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QCursor6setPosERK6QPoint(arg0)};
    // return 1;
  }
}

impl /*struct*/ QCursor {
  pub fn bitmap<T: QCursor_bitmap>(&mut self, value: T) -> QBitmap {
    return value.bitmap(self);
    // return 1;
  }
}

pub trait QCursor_bitmap {
  fn bitmap(self, rsthis: &mut QCursor) -> QBitmap;
}

// proto:  const QBitmap * QCursor::bitmap();
impl<'a> /*trait*/ QCursor_bitmap for () {
  fn bitmap(self, rsthis: &mut QCursor) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor6bitmapEv()};
    let mut ret = unsafe {_ZNK7QCursor6bitmapEv(rsthis.qclsinst)};
    let mut ret1 = QBitmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QCursor {
  pub fn pos<T: QCursor_pos>(&mut self, value: T) -> QPoint {
    return value.pos(self);
    // return 1;
  }
}

pub trait QCursor_pos {
  fn pos(self, rsthis: &mut QCursor) -> QPoint;
}

// proto: static QPoint QCursor::pos(const QScreen * screen);
impl<'a> /*trait*/ QCursor_pos for (&'a  QScreen) {
  fn pos(self, rsthis: &mut QCursor) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor3posEPK7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QCursor3posEPK7QScreen(arg0)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QPoint QCursor::pos();
impl<'a> /*trait*/ QCursor_pos for () {
  fn pos(self, rsthis: &mut QCursor) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor3posEv()};
    let mut ret = unsafe {_ZN7QCursor3posEv()};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QCursor::NewQCursor();
impl<'a> /*trait*/ QCursor_NewQCursor for () {
  fn NewQCursor(self) -> QCursor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorC1Ev()};
    unsafe {_ZN7QCursorC1Ev(qthis)};
    let rsthis = QCursor{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QCursor {
  pub fn hotSpot<T: QCursor_hotSpot>(&mut self, value: T) -> QPoint {
    return value.hotSpot(self);
    // return 1;
  }
}

pub trait QCursor_hotSpot {
  fn hotSpot(self, rsthis: &mut QCursor) -> QPoint;
}

// proto:  QPoint QCursor::hotSpot();
impl<'a> /*trait*/ QCursor_hotSpot for () {
  fn hotSpot(self, rsthis: &mut QCursor) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor7hotSpotEv()};
    let mut ret = unsafe {_ZNK7QCursor7hotSpotEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

