// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfont::QFont;
use super::qpoint::QPoint;
use super::qstring::QString;
use super::qwidget::QWidget;
use super::qrect::QRect;
use super::qpalette::QPalette;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QToolTip::setFont(const QFont & );
  fn _ZN8QToolTip7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: QPalette QToolTip::palette();
  fn _ZN8QToolTip7paletteEv() -> i32;
  // proto: void QToolTip::hideText();
  fn _ZN8QToolTip8hideTextEv() -> i32;
  // proto: void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect);
  fn _ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRect(arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void, arg3: *const c_void) -> i32;
  // proto: void QToolTip::NewQToolTip();
  fn _ZN8QToolTipC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect, int msecShowTime);
  fn _ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRecti(arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void, arg3: *const c_void, arg4: c_int) -> i32;
  // proto: QString QToolTip::text();
  fn _ZN8QToolTip4textEv() -> i32;
  // proto: QFont QToolTip::font();
  fn _ZN8QToolTip4fontEv() -> i32;
  // proto: void QToolTip::setPalette(const QPalette & );
  fn _ZN8QToolTip10setPaletteERK8QPalette(arg0: *const c_void) -> i32;
  // proto: void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w);
  fn _ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidget(arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: bool QToolTip::isVisible();
  fn _ZN8QToolTip9isVisibleEv() -> i32;
}

// body block begin
// class sizeof(QToolTip)=1
pub struct QToolTip {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QToolTip {
  pub fn setFont<T: QToolTip_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QToolTip_setFont {
  fn setFont(self, this: &mut QToolTip) -> i32;
}

// proto: void QToolTip::setFont(const QFont & );
impl<'a> /*trait*/ QToolTip_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QToolTip) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QToolTip7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn palette<T: QToolTip_palette>(&mut self, value: T) -> i32 {
    value.palette(self);
    return 1;
  }
}

pub trait QToolTip_palette {
  fn palette(self, this: &mut QToolTip) -> i32;
}

// proto: QPalette QToolTip::palette();
impl<'a> /*trait*/ QToolTip_palette for () {
  fn palette(self, this: &mut QToolTip) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip7paletteEv()};
    unsafe {_ZN8QToolTip7paletteEv()};
    return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn hideText<T: QToolTip_hideText>(&mut self, value: T) -> i32 {
    value.hideText(self);
    return 1;
  }
}

pub trait QToolTip_hideText {
  fn hideText(self, this: &mut QToolTip) -> i32;
}

// proto: void QToolTip::hideText();
impl<'a> /*trait*/ QToolTip_hideText for () {
  fn hideText(self, this: &mut QToolTip) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip8hideTextEv()};
    unsafe {_ZN8QToolTip8hideTextEv()};
    return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn showText<T: QToolTip_showText>(&mut self, value: T) -> i32 {
    value.showText(self);
    return 1;
  }
}

pub trait QToolTip_showText {
  fn showText(self, this: &mut QToolTip) -> i32;
}

// proto: void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect);
impl<'a> /*trait*/ QToolTip_showText for (&'a  QPoint, &'a  QString, &'a mut QWidget, &'a  QRect) {
  fn showText(self, this: &mut QToolTip) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRect()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRect(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn NewQToolTip<T: QToolTip_NewQToolTip>(value: T) -> QToolTip {
    let rsthis = value.NewQToolTip();
    return rsthis;
    // return 1;
  }
}

pub trait QToolTip_NewQToolTip {
  fn NewQToolTip(self) -> QToolTip;
}

// proto: void QToolTip::NewQToolTip();
impl<'a> /*trait*/ QToolTip_NewQToolTip for () {
  fn NewQToolTip(self) -> QToolTip {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTipC1Ev()};
    unsafe {_ZN8QToolTipC1Ev(qthis)};
    let rsthis = QToolTip{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect, int msecShowTime);
impl<'a> /*trait*/ QToolTip_showText for (&'a  QPoint, &'a  QString, &'a mut QWidget, &'a  QRect, i32) {
  fn showText(self, this: &mut QToolTip) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRecti()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    let arg4 = self.4  as c_int;
    unsafe {_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRecti(arg0, arg1, arg2, arg3, arg4)};
    return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn text<T: QToolTip_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QToolTip_text {
  fn text(self, this: &mut QToolTip) -> i32;
}

// proto: QString QToolTip::text();
impl<'a> /*trait*/ QToolTip_text for () {
  fn text(self, this: &mut QToolTip) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip4textEv()};
    unsafe {_ZN8QToolTip4textEv()};
    return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn font<T: QToolTip_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QToolTip_font {
  fn font(self, this: &mut QToolTip) -> i32;
}

// proto: QFont QToolTip::font();
impl<'a> /*trait*/ QToolTip_font for () {
  fn font(self, this: &mut QToolTip) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip4fontEv()};
    unsafe {_ZN8QToolTip4fontEv()};
    return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn setPalette<T: QToolTip_setPalette>(&mut self, value: T) -> i32 {
    value.setPalette(self);
    return 1;
  }
}

pub trait QToolTip_setPalette {
  fn setPalette(self, this: &mut QToolTip) -> i32;
}

// proto: void QToolTip::setPalette(const QPalette & );
impl<'a> /*trait*/ QToolTip_setPalette for (&'a  QPalette) {
  fn setPalette(self, this: &mut QToolTip) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN8QToolTip10setPaletteERK8QPalette(arg0)};
    return 1;
  }
}

// proto: void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w);
impl<'a> /*trait*/ QToolTip_showText for (&'a  QPoint, &'a  QString, &'a mut QWidget) {
  fn showText(self, this: &mut QToolTip) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidget(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn isVisible<T: QToolTip_isVisible>(&mut self, value: T) -> i32 {
    value.isVisible(self);
    return 1;
  }
}

pub trait QToolTip_isVisible {
  fn isVisible(self, this: &mut QToolTip) -> i32;
}

// proto: bool QToolTip::isVisible();
impl<'a> /*trait*/ QToolTip_isVisible for () {
  fn isVisible(self, this: &mut QToolTip) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip9isVisibleEv()};
    unsafe {_ZN8QToolTip9isVisibleEv()};
    return 1;
  }
}

