// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfont::QFont;
use super::qpalette::QPalette;
use super::qpoint::QPoint;
use super::qstring::QString;
use super::qwidget::QWidget;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static void QToolTip::setFont(const QFont & );
  fn _ZN8QToolTip7setFontERK5QFont(arg0: *mut c_void) ;
  // proto: static QPalette QToolTip::palette();
  fn _ZN8QToolTip7paletteEv() -> *mut c_void;
  // proto: static void QToolTip::hideText();
  fn _ZN8QToolTip8hideTextEv() ;
  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect);
  fn _ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRect(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) ;
  // proto:  void QToolTip::NewQToolTip();
  fn _ZN8QToolTipC1Ev(qthis: *mut c_void) ;
  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect, int msecShowTime);
  fn _ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRecti(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: c_int) ;
  // proto: static QString QToolTip::text();
  fn _ZN8QToolTip4textEv() -> *mut c_void;
  // proto: static QFont QToolTip::font();
  fn _ZN8QToolTip4fontEv() -> *mut c_void;
  // proto: static void QToolTip::setPalette(const QPalette & );
  fn _ZN8QToolTip10setPaletteERK8QPalette(arg0: *mut c_void) ;
  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w);
  fn _ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto: static bool QToolTip::isVisible();
  fn _ZN8QToolTip9isVisibleEv() -> int8_t;
}

// body block begin
// class sizeof(QToolTip)=1
pub struct QToolTip {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QToolTip {
  pub fn setFont<T: QToolTip_setFont>(&mut self, value: T)  {
     value.setFont(self);
    // return 1;
  }
}

pub trait QToolTip_setFont {
  fn setFont(self, rsthis: &mut QToolTip) ;
}

// proto: static void QToolTip::setFont(const QFont & );
impl<'a> /*trait*/ QToolTip_setFont for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QToolTip)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolTip7setFontERK5QFont(arg0)};
    // return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn palette<T: QToolTip_palette>(&mut self, value: T) -> QPalette {
    return value.palette(self);
    // return 1;
  }
}

pub trait QToolTip_palette {
  fn palette(self, rsthis: &mut QToolTip) -> QPalette;
}

// proto: static QPalette QToolTip::palette();
impl<'a> /*trait*/ QToolTip_palette for () {
  fn palette(self, rsthis: &mut QToolTip) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip7paletteEv()};
    let mut ret = unsafe {_ZN8QToolTip7paletteEv()};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn hideText<T: QToolTip_hideText>(&mut self, value: T)  {
     value.hideText(self);
    // return 1;
  }
}

pub trait QToolTip_hideText {
  fn hideText(self, rsthis: &mut QToolTip) ;
}

// proto: static void QToolTip::hideText();
impl<'a> /*trait*/ QToolTip_hideText for () {
  fn hideText(self, rsthis: &mut QToolTip)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip8hideTextEv()};
     unsafe {_ZN8QToolTip8hideTextEv()};
    // return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn showText<T: QToolTip_showText>(&mut self, value: T)  {
     value.showText(self);
    // return 1;
  }
}

pub trait QToolTip_showText {
  fn showText(self, rsthis: &mut QToolTip) ;
}

// proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect);
impl<'a> /*trait*/ QToolTip_showText for (&'a  QPoint, &'a  QString, &'a mut QWidget, &'a  QRect) {
  fn showText(self, rsthis: &mut QToolTip)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRect()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRect(arg0, arg1, arg2, arg3)};
    // return 1;
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

// proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect, int msecShowTime);
impl<'a> /*trait*/ QToolTip_showText for (&'a  QPoint, &'a  QString, &'a mut QWidget, &'a  QRect, i32) {
  fn showText(self, rsthis: &mut QToolTip)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4  as c_int;
     unsafe {_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRecti(arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn text<T: QToolTip_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QToolTip_text {
  fn text(self, rsthis: &mut QToolTip) -> QString;
}

// proto: static QString QToolTip::text();
impl<'a> /*trait*/ QToolTip_text for () {
  fn text(self, rsthis: &mut QToolTip) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip4textEv()};
    let mut ret = unsafe {_ZN8QToolTip4textEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn font<T: QToolTip_font>(&mut self, value: T) -> QFont {
    return value.font(self);
    // return 1;
  }
}

pub trait QToolTip_font {
  fn font(self, rsthis: &mut QToolTip) -> QFont;
}

// proto: static QFont QToolTip::font();
impl<'a> /*trait*/ QToolTip_font for () {
  fn font(self, rsthis: &mut QToolTip) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip4fontEv()};
    let mut ret = unsafe {_ZN8QToolTip4fontEv()};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn setPalette<T: QToolTip_setPalette>(&mut self, value: T)  {
     value.setPalette(self);
    // return 1;
  }
}

pub trait QToolTip_setPalette {
  fn setPalette(self, rsthis: &mut QToolTip) ;
}

// proto: static void QToolTip::setPalette(const QPalette & );
impl<'a> /*trait*/ QToolTip_setPalette for (&'a  QPalette) {
  fn setPalette(self, rsthis: &mut QToolTip)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolTip10setPaletteERK8QPalette(arg0)};
    // return 1;
  }
}

// proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w);
impl<'a> /*trait*/ QToolTip_showText for (&'a  QPoint, &'a  QString, &'a mut QWidget) {
  fn showText(self, rsthis: &mut QToolTip)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidget(arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QToolTip {
  pub fn isVisible<T: QToolTip_isVisible>(&mut self, value: T) -> i8 {
    return value.isVisible(self);
    // return 1;
  }
}

pub trait QToolTip_isVisible {
  fn isVisible(self, rsthis: &mut QToolTip) -> i8;
}

// proto: static bool QToolTip::isVisible();
impl<'a> /*trait*/ QToolTip_isVisible for () {
  fn isVisible(self, rsthis: &mut QToolTip) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip9isVisibleEv()};
    let mut ret = unsafe {_ZN8QToolTip9isVisibleEv()};
    return ret as i8;
    // return 1;
  }
}

