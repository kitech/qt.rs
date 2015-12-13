// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qpalette::QPalette;
use super::qrect::QRect;
use super::qpixmap::QPixmap;
use super::qfontmetrics::QFontMetrics;
use super::qstring::QString;
use super::qapplication::QApplication;
use super::qpainter::QPainter;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStyle::NewQStyle(const QStyle & );
  fn _ZN6QStyleC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyle::unpolish(QWidget * );
  fn _ZN6QStyle8unpolishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyle::FreeQStyle();
  fn _ZN6QStyleD0Ev(qthis: *mut c_void) ;
  // proto:  void QStyle::polish(QPalette & );
  fn _ZN6QStyle6polishER8QPalette(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyle::NewQStyle();
  fn _ZN6QStyleC1Ev(qthis: *mut c_void) ;
  // proto:  QRect QStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
  fn _ZNK6QStyle14itemPixmapRectERK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QRect QStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
  fn _ZNK6QStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: int8_t, arg4: *mut c_void) -> *mut c_void;
  // proto:  const QStyle * QStyle::proxy();
  fn _ZNK6QStyle5proxyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPalette QStyle::standardPalette();
  fn _ZNK6QStyle15standardPaletteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QStyle::metaObject();
  fn _ZNK6QStyle10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QStyle::polish(QApplication * );
  fn _ZN6QStyle6polishEP12QApplication(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
  fn _ZNK6QStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: *mut c_void) ;
  // proto:  void QStyle::polish(QWidget * );
  fn _ZN6QStyle6polishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static int QStyle::sliderPositionFromValue(int min, int max, int val, int space, bool upsideDown);
  fn _ZN6QStyle23sliderPositionFromValueEiiiib(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: int8_t) -> c_int;
  // proto: static int QStyle::sliderValueFromPosition(int min, int max, int pos, int space, bool upsideDown);
  fn _ZN6QStyle23sliderValueFromPositionEiiiib(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: int8_t) -> c_int;
  // proto:  void QStyle::unpolish(QApplication * );
  fn _ZN6QStyle8unpolishEP12QApplication(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QStyle)=1
pub struct QStyle {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyle {
  pub fn NewQStyle<T: QStyle_NewQStyle>(value: T) -> QStyle {
    let rsthis = value.NewQStyle();
    return rsthis;
    // return 1;
  }
}

pub trait QStyle_NewQStyle {
  fn NewQStyle(self) -> QStyle;
}

// proto: void QStyle::NewQStyle(const QStyle & );
impl<'a> /*trait*/ QStyle_NewQStyle for (&'a  QStyle) {
  fn NewQStyle(self) -> QStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyleC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QStyleC1ERKS_(qthis, arg0)};
    let rsthis = QStyle{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn unpolish<T: QStyle_unpolish>(&mut self, value: T)  {
     value.unpolish(self);
    // return 1;
  }
}

pub trait QStyle_unpolish {
  fn unpolish(self, rsthis: &mut QStyle) ;
}

// proto:  void QStyle::unpolish(QWidget * );
impl<'a> /*trait*/ QStyle_unpolish for (&'a mut QWidget) {
  fn unpolish(self, rsthis: &mut QStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle8unpolishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QStyle8unpolishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn FreeQStyle<T: QStyle_FreeQStyle>(&mut self, value: T)  {
     value.FreeQStyle(self);
    // return 1;
  }
}

pub trait QStyle_FreeQStyle {
  fn FreeQStyle(self, rsthis: &mut QStyle) ;
}

// proto:  void QStyle::FreeQStyle();
impl<'a> /*trait*/ QStyle_FreeQStyle for () {
  fn FreeQStyle(self, rsthis: &mut QStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyleD0Ev()};
     unsafe {_ZN6QStyleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn polish<T: QStyle_polish>(&mut self, value: T)  {
     value.polish(self);
    // return 1;
  }
}

pub trait QStyle_polish {
  fn polish(self, rsthis: &mut QStyle) ;
}

// proto:  void QStyle::polish(QPalette & );
impl<'a> /*trait*/ QStyle_polish for (&'a mut QPalette) {
  fn polish(self, rsthis: &mut QStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle6polishER8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QStyle6polishER8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QStyle::NewQStyle();
impl<'a> /*trait*/ QStyle_NewQStyle for () {
  fn NewQStyle(self) -> QStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyleC1Ev()};
    unsafe {_ZN6QStyleC1Ev(qthis)};
    let rsthis = QStyle{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn itemPixmapRect<T: QStyle_itemPixmapRect>(&mut self, value: T) -> QRect {
    return value.itemPixmapRect(self);
    // return 1;
  }
}

pub trait QStyle_itemPixmapRect {
  fn itemPixmapRect(self, rsthis: &mut QStyle) -> QRect;
}

// proto:  QRect QStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
impl<'a> /*trait*/ QStyle_itemPixmapRect for (&'a  QRect, i32, &'a  QPixmap) {
  fn itemPixmapRect(self, rsthis: &mut QStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle14itemPixmapRectERK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QStyle14itemPixmapRectERK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn itemTextRect<T: QStyle_itemTextRect>(&mut self, value: T) -> QRect {
    return value.itemTextRect(self);
    // return 1;
  }
}

pub trait QStyle_itemTextRect {
  fn itemTextRect(self, rsthis: &mut QStyle) -> QRect;
}

// proto:  QRect QStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
impl<'a> /*trait*/ QStyle_itemTextRect for (&'a  QFontMetrics, &'a  QRect, i32, i8, &'a  QString) {
  fn itemTextRect(self, rsthis: &mut QStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as int8_t;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn proxy<T: QStyle_proxy>(&mut self, value: T) -> QStyle {
    return value.proxy(self);
    // return 1;
  }
}

pub trait QStyle_proxy {
  fn proxy(self, rsthis: &mut QStyle) -> QStyle;
}

// proto:  const QStyle * QStyle::proxy();
impl<'a> /*trait*/ QStyle_proxy for () {
  fn proxy(self, rsthis: &mut QStyle) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle5proxyEv()};
    let mut ret = unsafe {_ZNK6QStyle5proxyEv(rsthis.qclsinst)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn standardPalette<T: QStyle_standardPalette>(&mut self, value: T) -> QPalette {
    return value.standardPalette(self);
    // return 1;
  }
}

pub trait QStyle_standardPalette {
  fn standardPalette(self, rsthis: &mut QStyle) -> QPalette;
}

// proto:  QPalette QStyle::standardPalette();
impl<'a> /*trait*/ QStyle_standardPalette for () {
  fn standardPalette(self, rsthis: &mut QStyle) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle15standardPaletteEv()};
    let mut ret = unsafe {_ZNK6QStyle15standardPaletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn metaObject<T: QStyle_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QStyle_metaObject {
  fn metaObject(self, rsthis: &mut QStyle) ;
}

// proto:  const QMetaObject * QStyle::metaObject();
impl<'a> /*trait*/ QStyle_metaObject for () {
  fn metaObject(self, rsthis: &mut QStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle10metaObjectEv()};
     unsafe {_ZNK6QStyle10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QStyle::polish(QApplication * );
impl<'a> /*trait*/ QStyle_polish for (&'a mut QApplication) {
  fn polish(self, rsthis: &mut QStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle6polishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QStyle6polishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn drawItemPixmap<T: QStyle_drawItemPixmap>(&mut self, value: T)  {
     value.drawItemPixmap(self);
    // return 1;
  }
}

pub trait QStyle_drawItemPixmap {
  fn drawItemPixmap(self, rsthis: &mut QStyle) ;
}

// proto:  void QStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
impl<'a> /*trait*/ QStyle_drawItemPixmap for (&'a mut QPainter, &'a  QRect, i32, &'a  QPixmap) {
  fn drawItemPixmap(self, rsthis: &mut QStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZNK6QStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QStyle::polish(QWidget * );
impl<'a> /*trait*/ QStyle_polish for (&'a mut QWidget) {
  fn polish(self, rsthis: &mut QStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle6polishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QStyle6polishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn sliderPositionFromValue<T: QStyle_sliderPositionFromValue>(&mut self, value: T) -> i32 {
    return value.sliderPositionFromValue(self);
    // return 1;
  }
}

pub trait QStyle_sliderPositionFromValue {
  fn sliderPositionFromValue(self, rsthis: &mut QStyle) -> i32;
}

// proto: static int QStyle::sliderPositionFromValue(int min, int max, int val, int space, bool upsideDown);
impl<'a> /*trait*/ QStyle_sliderPositionFromValue for (i32, i32, i32, i32, i8) {
  fn sliderPositionFromValue(self, rsthis: &mut QStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle23sliderPositionFromValueEiiiib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as int8_t;
    let mut ret = unsafe {_ZN6QStyle23sliderPositionFromValueEiiiib(arg0, arg1, arg2, arg3, arg4)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStyle {
  pub fn sliderValueFromPosition<T: QStyle_sliderValueFromPosition>(&mut self, value: T) -> i32 {
    return value.sliderValueFromPosition(self);
    // return 1;
  }
}

pub trait QStyle_sliderValueFromPosition {
  fn sliderValueFromPosition(self, rsthis: &mut QStyle) -> i32;
}

// proto: static int QStyle::sliderValueFromPosition(int min, int max, int pos, int space, bool upsideDown);
impl<'a> /*trait*/ QStyle_sliderValueFromPosition for (i32, i32, i32, i32, i8) {
  fn sliderValueFromPosition(self, rsthis: &mut QStyle) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle23sliderValueFromPositionEiiiib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as int8_t;
    let mut ret = unsafe {_ZN6QStyle23sliderValueFromPositionEiiiib(arg0, arg1, arg2, arg3, arg4)};
    return ret as i32;
    // return 1;
  }
}

// proto:  void QStyle::unpolish(QApplication * );
impl<'a> /*trait*/ QStyle_unpolish for (&'a mut QApplication) {
  fn unpolish(self, rsthis: &mut QStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle8unpolishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QStyle8unpolishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

