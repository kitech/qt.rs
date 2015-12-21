// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qcursor.h
// dst-file: /src/gui/qcursor.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qscreen::QScreen; // 773
use super::qpixmap::QPixmap; // 773
use super::qbitmap::QBitmap; // 773
use super::super::core::qpoint::QPoint; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto: static void QCursor::setPos(QScreen * screen, int x, int y);
  fn _ZN7QCursor6setPosEP7QScreenii(arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  QPixmap QCursor::pixmap();
  fn _ZNK7QCursor6pixmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCursor::QCursor(const QBitmap & bitmap, const QBitmap & mask, int hotX, int hotY);
  fn _ZN7QCursorC1ERK7QBitmapS2_ii(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int);
  // proto:  void QCursor::QCursor(const QPixmap & pixmap, int hotX, int hotY);
  fn _ZN7QCursorC1ERK7QPixmapii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QCursor::~QCursor();
  fn _ZN7QCursorD0Ev(qthis: *mut c_void);
  // proto:  const QBitmap * QCursor::mask();
  fn _ZNK7QCursor4maskEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCursor::QCursor(const QCursor & cursor);
  fn _ZN7QCursorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static void QCursor::setPos(int x, int y);
  fn _ZN7QCursor6setPosEii(arg0: c_int, arg1: c_int);
  // proto: static void QCursor::setPos(QScreen * screen, const QPoint & p);
  fn _ZN7QCursor6setPosEP7QScreenRK6QPoint(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static void QCursor::setPos(const QPoint & p);
  fn _ZN7QCursor6setPosERK6QPoint(arg0: *mut c_void);
  // proto:  const QBitmap * QCursor::bitmap();
  fn _ZNK7QCursor6bitmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QPoint QCursor::pos(const QScreen * screen);
  fn _ZN7QCursor3posEPK7QScreen(arg0: *mut c_void) -> *mut c_void;
  // proto: static QPoint QCursor::pos();
  fn _ZN7QCursor3posEv() -> *mut c_void;
  // proto:  void QCursor::QCursor();
  fn _ZN7QCursorC1Ev(qthis: *mut c_void);
  // proto:  QPoint QCursor::hotSpot();
  fn _ZNK7QCursor7hotSpotEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QCursor)=8
pub struct QCursor {
  pub qclsinst: *mut c_void,
}

  // proto: static void QCursor::setPos(QScreen * screen, int x, int y);
impl /*struct*/ QCursor {
  pub fn setPos_s<RetType, T: QCursor_setPos_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPos_s();
    // return 1;
  }
}

pub trait QCursor_setPos_s<RetType> {
  fn setPos_s(self ) -> RetType;
}

  // proto: static void QCursor::setPos(QScreen * screen, int x, int y);
impl<'a> /*trait*/ QCursor_setPos_s<()> for (QScreen, i32, i32) {
  fn setPos_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosEP7QScreenii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN7QCursor6setPosEP7QScreenii(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QPixmap QCursor::pixmap();
impl /*struct*/ QCursor {
  pub fn pixmap<RetType, T: QCursor_pixmap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pixmap(self);
    // return 1;
  }
}

pub trait QCursor_pixmap<RetType> {
  fn pixmap(self , rsthis: &mut QCursor) -> RetType;
}

  // proto:  QPixmap QCursor::pixmap();
impl<'a> /*trait*/ QCursor_pixmap<QPixmap> for () {
  fn pixmap(self , rsthis: &mut QCursor) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor6pixmapEv()};
    let mut ret = unsafe {_ZNK7QCursor6pixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCursor::QCursor(const QBitmap & bitmap, const QBitmap & mask, int hotX, int hotY);
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

  // proto:  void QCursor::QCursor(const QBitmap & bitmap, const QBitmap & mask, int hotX, int hotY);
impl<'a> /*trait*/ QCursor_NewQCursor for (QBitmap, QBitmap, i32, i32) {
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

  // proto:  void QCursor::QCursor(const QPixmap & pixmap, int hotX, int hotY);
impl<'a> /*trait*/ QCursor_NewQCursor for (QPixmap, i32, i32) {
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

  // proto:  void QCursor::~QCursor();
impl /*struct*/ QCursor {
  pub fn FreeQCursor<RetType, T: QCursor_FreeQCursor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQCursor(self);
    // return 1;
  }
}

pub trait QCursor_FreeQCursor<RetType> {
  fn FreeQCursor(self , rsthis: &mut QCursor) -> RetType;
}

  // proto:  void QCursor::~QCursor();
impl<'a> /*trait*/ QCursor_FreeQCursor<()> for () {
  fn FreeQCursor(self , rsthis: &mut QCursor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursorD0Ev()};
     unsafe {_ZN7QCursorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QBitmap * QCursor::mask();
impl /*struct*/ QCursor {
  pub fn mask<RetType, T: QCursor_mask<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mask(self);
    // return 1;
  }
}

pub trait QCursor_mask<RetType> {
  fn mask(self , rsthis: &mut QCursor) -> RetType;
}

  // proto:  const QBitmap * QCursor::mask();
impl<'a> /*trait*/ QCursor_mask<QBitmap> for () {
  fn mask(self , rsthis: &mut QCursor) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor4maskEv()};
    let mut ret = unsafe {_ZNK7QCursor4maskEv(rsthis.qclsinst)};
    let mut ret1 = QBitmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCursor::QCursor(const QCursor & cursor);
impl<'a> /*trait*/ QCursor_NewQCursor for (QCursor) {
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
impl<'a> /*trait*/ QCursor_setPos_s<()> for (i32, i32) {
  fn setPos_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QCursor6setPosEii(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QCursor::setPos(QScreen * screen, const QPoint & p);
impl<'a> /*trait*/ QCursor_setPos_s<()> for (QScreen, QPoint) {
  fn setPos_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosEP7QScreenRK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QCursor6setPosEP7QScreenRK6QPoint(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QCursor::setPos(const QPoint & p);
impl<'a> /*trait*/ QCursor_setPos_s<()> for (QPoint) {
  fn setPos_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor6setPosERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QCursor6setPosERK6QPoint(arg0)};
    // return 1;
  }
}

  // proto:  const QBitmap * QCursor::bitmap();
impl /*struct*/ QCursor {
  pub fn bitmap<RetType, T: QCursor_bitmap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.bitmap(self);
    // return 1;
  }
}

pub trait QCursor_bitmap<RetType> {
  fn bitmap(self , rsthis: &mut QCursor) -> RetType;
}

  // proto:  const QBitmap * QCursor::bitmap();
impl<'a> /*trait*/ QCursor_bitmap<QBitmap> for () {
  fn bitmap(self , rsthis: &mut QCursor) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor6bitmapEv()};
    let mut ret = unsafe {_ZNK7QCursor6bitmapEv(rsthis.qclsinst)};
    let mut ret1 = QBitmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QPoint QCursor::pos(const QScreen * screen);
impl /*struct*/ QCursor {
  pub fn pos_s<RetType, T: QCursor_pos_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.pos_s();
    // return 1;
  }
}

pub trait QCursor_pos_s<RetType> {
  fn pos_s(self ) -> RetType;
}

  // proto: static QPoint QCursor::pos(const QScreen * screen);
impl<'a> /*trait*/ QCursor_pos_s<QPoint> for (QScreen) {
  fn pos_s(self ) -> QPoint {
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
impl<'a> /*trait*/ QCursor_pos_s<QPoint> for () {
  fn pos_s(self ) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QCursor3posEv()};
    let mut ret = unsafe {_ZN7QCursor3posEv()};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCursor::QCursor();
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

  // proto:  QPoint QCursor::hotSpot();
impl /*struct*/ QCursor {
  pub fn hotSpot<RetType, T: QCursor_hotSpot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hotSpot(self);
    // return 1;
  }
}

pub trait QCursor_hotSpot<RetType> {
  fn hotSpot(self , rsthis: &mut QCursor) -> RetType;
}

  // proto:  QPoint QCursor::hotSpot();
impl<'a> /*trait*/ QCursor_hotSpot<QPoint> for () {
  fn hotSpot(self , rsthis: &mut QCursor) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QCursor7hotSpotEv()};
    let mut ret = unsafe {_ZNK7QCursor7hotSpotEv(rsthis.qclsinst)};
    let mut ret1 = QPoint{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// <= body block end

