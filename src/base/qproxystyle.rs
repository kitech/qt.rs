// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qstring::QString;
use super::qapplication::QApplication;
use super::qpalette::QPalette;
use super::qstyle::QStyle;
use super::qpainter::QPainter;
use super::qrect::QRect;
use super::qpixmap::QPixmap;
use super::qfontmetrics::QFontMetrics;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QProxyStyle::unpolish(QWidget * widget);
  fn _ZN11QProxyStyle8unpolishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProxyStyle::NewQProxyStyle(const QString & key);
  fn _ZN11QProxyStyleC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProxyStyle::unpolish(QApplication * app);
  fn _ZN11QProxyStyle8unpolishEP12QApplication(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QPalette QProxyStyle::standardPalette();
  fn _ZNK11QProxyStyle15standardPaletteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProxyStyle::setBaseStyle(QStyle * style);
  fn _ZN11QProxyStyle12setBaseStyleEP6QStyle(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProxyStyle::polish(QPalette & pal);
  fn _ZN11QProxyStyle6polishER8QPalette(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProxyStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
  fn _ZNK11QProxyStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: *mut c_void) ;
  // proto:  void QProxyStyle::FreeQProxyStyle();
  fn _ZN11QProxyStyleD0Ev(qthis: *mut c_void) ;
  // proto:  QStyle * QProxyStyle::baseStyle();
  fn _ZNK11QProxyStyle9baseStyleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProxyStyle::NewQProxyStyle(const QProxyStyle & );
  fn _ZN11QProxyStyleC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProxyStyle::polish(QApplication * app);
  fn _ZN11QProxyStyle6polishEP12QApplication(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProxyStyle::polish(QWidget * widget);
  fn _ZN11QProxyStyle6polishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRect QProxyStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
  fn _ZNK11QProxyStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: int8_t, arg4: *mut c_void) -> *mut c_void;
  // proto:  QRect QProxyStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
  fn _ZNK11QProxyStyle14itemPixmapRectERK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QProxyStyle::metaObject();
  fn _ZNK11QProxyStyle10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QProxyStyle::NewQProxyStyle(QStyle * style);
  fn _ZN11QProxyStyleC1EP6QStyle(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QProxyStyle)=1
pub struct QProxyStyle {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QProxyStyle {
  pub fn unpolish<T: QProxyStyle_unpolish>(&mut self, value: T)  {
     value.unpolish(self);
    // return 1;
  }
}

pub trait QProxyStyle_unpolish {
  fn unpolish(self, rsthis: &mut QProxyStyle) ;
}

// proto:  void QProxyStyle::unpolish(QWidget * widget);
impl<'a> /*trait*/ QProxyStyle_unpolish for (&'a mut QWidget) {
  fn unpolish(self, rsthis: &mut QProxyStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle8unpolishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle8unpolishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProxyStyle {
  pub fn NewQProxyStyle<T: QProxyStyle_NewQProxyStyle>(value: T) -> QProxyStyle {
    let rsthis = value.NewQProxyStyle();
    return rsthis;
    // return 1;
  }
}

pub trait QProxyStyle_NewQProxyStyle {
  fn NewQProxyStyle(self) -> QProxyStyle;
}

// proto: void QProxyStyle::NewQProxyStyle(const QString & key);
impl<'a> /*trait*/ QProxyStyle_NewQProxyStyle for (&'a  QString) {
  fn NewQProxyStyle(self) -> QProxyStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QProxyStyleC1ERK7QString(qthis, arg0)};
    let rsthis = QProxyStyle{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QProxyStyle::unpolish(QApplication * app);
impl<'a> /*trait*/ QProxyStyle_unpolish for (&'a mut QApplication) {
  fn unpolish(self, rsthis: &mut QProxyStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle8unpolishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle8unpolishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProxyStyle {
  pub fn standardPalette<T: QProxyStyle_standardPalette>(&mut self, value: T) -> QPalette {
    return value.standardPalette(self);
    // return 1;
  }
}

pub trait QProxyStyle_standardPalette {
  fn standardPalette(self, rsthis: &mut QProxyStyle) -> QPalette;
}

// proto:  QPalette QProxyStyle::standardPalette();
impl<'a> /*trait*/ QProxyStyle_standardPalette for () {
  fn standardPalette(self, rsthis: &mut QProxyStyle) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle15standardPaletteEv()};
    let mut ret = unsafe {_ZNK11QProxyStyle15standardPaletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProxyStyle {
  pub fn setBaseStyle<T: QProxyStyle_setBaseStyle>(&mut self, value: T)  {
     value.setBaseStyle(self);
    // return 1;
  }
}

pub trait QProxyStyle_setBaseStyle {
  fn setBaseStyle(self, rsthis: &mut QProxyStyle) ;
}

// proto:  void QProxyStyle::setBaseStyle(QStyle * style);
impl<'a> /*trait*/ QProxyStyle_setBaseStyle for (&'a mut QStyle) {
  fn setBaseStyle(self, rsthis: &mut QProxyStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle12setBaseStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle12setBaseStyleEP6QStyle(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProxyStyle {
  pub fn polish<T: QProxyStyle_polish>(&mut self, value: T)  {
     value.polish(self);
    // return 1;
  }
}

pub trait QProxyStyle_polish {
  fn polish(self, rsthis: &mut QProxyStyle) ;
}

// proto:  void QProxyStyle::polish(QPalette & pal);
impl<'a> /*trait*/ QProxyStyle_polish for (&'a mut QPalette) {
  fn polish(self, rsthis: &mut QProxyStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle6polishER8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle6polishER8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProxyStyle {
  pub fn drawItemPixmap<T: QProxyStyle_drawItemPixmap>(&mut self, value: T)  {
     value.drawItemPixmap(self);
    // return 1;
  }
}

pub trait QProxyStyle_drawItemPixmap {
  fn drawItemPixmap(self, rsthis: &mut QProxyStyle) ;
}

// proto:  void QProxyStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
impl<'a> /*trait*/ QProxyStyle_drawItemPixmap for (&'a mut QPainter, &'a  QRect, i32, &'a  QPixmap) {
  fn drawItemPixmap(self, rsthis: &mut QProxyStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZNK11QProxyStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QProxyStyle {
  pub fn FreeQProxyStyle<T: QProxyStyle_FreeQProxyStyle>(&mut self, value: T)  {
     value.FreeQProxyStyle(self);
    // return 1;
  }
}

pub trait QProxyStyle_FreeQProxyStyle {
  fn FreeQProxyStyle(self, rsthis: &mut QProxyStyle) ;
}

// proto:  void QProxyStyle::FreeQProxyStyle();
impl<'a> /*trait*/ QProxyStyle_FreeQProxyStyle for () {
  fn FreeQProxyStyle(self, rsthis: &mut QProxyStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleD0Ev()};
     unsafe {_ZN11QProxyStyleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProxyStyle {
  pub fn baseStyle<T: QProxyStyle_baseStyle>(&mut self, value: T) -> QStyle {
    return value.baseStyle(self);
    // return 1;
  }
}

pub trait QProxyStyle_baseStyle {
  fn baseStyle(self, rsthis: &mut QProxyStyle) -> QStyle;
}

// proto:  QStyle * QProxyStyle::baseStyle();
impl<'a> /*trait*/ QProxyStyle_baseStyle for () {
  fn baseStyle(self, rsthis: &mut QProxyStyle) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle9baseStyleEv()};
    let mut ret = unsafe {_ZNK11QProxyStyle9baseStyleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QProxyStyle::NewQProxyStyle(const QProxyStyle & );
impl<'a> /*trait*/ QProxyStyle_NewQProxyStyle for (&'a  QProxyStyle) {
  fn NewQProxyStyle(self) -> QProxyStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QProxyStyleC1ERKS_(qthis, arg0)};
    let rsthis = QProxyStyle{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QProxyStyle::polish(QApplication * app);
impl<'a> /*trait*/ QProxyStyle_polish for (&'a mut QApplication) {
  fn polish(self, rsthis: &mut QProxyStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle6polishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle6polishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QProxyStyle::polish(QWidget * widget);
impl<'a> /*trait*/ QProxyStyle_polish for (&'a mut QWidget) {
  fn polish(self, rsthis: &mut QProxyStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle6polishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle6polishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProxyStyle {
  pub fn itemTextRect<T: QProxyStyle_itemTextRect>(&mut self, value: T) -> QRect {
    return value.itemTextRect(self);
    // return 1;
  }
}

pub trait QProxyStyle_itemTextRect {
  fn itemTextRect(self, rsthis: &mut QProxyStyle) -> QRect;
}

// proto:  QRect QProxyStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
impl<'a> /*trait*/ QProxyStyle_itemTextRect for (&'a  QFontMetrics, &'a  QRect, i32, i8, &'a  QString) {
  fn itemTextRect(self, rsthis: &mut QProxyStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as int8_t;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QProxyStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProxyStyle {
  pub fn itemPixmapRect<T: QProxyStyle_itemPixmapRect>(&mut self, value: T) -> QRect {
    return value.itemPixmapRect(self);
    // return 1;
  }
}

pub trait QProxyStyle_itemPixmapRect {
  fn itemPixmapRect(self, rsthis: &mut QProxyStyle) -> QRect;
}

// proto:  QRect QProxyStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
impl<'a> /*trait*/ QProxyStyle_itemPixmapRect for (&'a  QRect, i32, &'a  QPixmap) {
  fn itemPixmapRect(self, rsthis: &mut QProxyStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle14itemPixmapRectERK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QProxyStyle14itemPixmapRectERK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProxyStyle {
  pub fn metaObject<T: QProxyStyle_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QProxyStyle_metaObject {
  fn metaObject(self, rsthis: &mut QProxyStyle) ;
}

// proto:  const QMetaObject * QProxyStyle::metaObject();
impl<'a> /*trait*/ QProxyStyle_metaObject for () {
  fn metaObject(self, rsthis: &mut QProxyStyle)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle10metaObjectEv()};
     unsafe {_ZNK11QProxyStyle10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QProxyStyle::NewQProxyStyle(QStyle * style);
impl<'a> /*trait*/ QProxyStyle_NewQProxyStyle for (&'a mut QStyle) {
  fn NewQProxyStyle(self) -> QProxyStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleC1EP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QProxyStyleC1EP6QStyle(qthis, arg0)};
    let rsthis = QProxyStyle{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

