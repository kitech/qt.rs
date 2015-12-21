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
  fn _ZN8QToolTip7setFontERK5QFont(arg0: *mut c_void);
  // proto: static QPalette QToolTip::palette();
  fn _ZN8QToolTip7paletteEv() -> *mut c_void;
  // proto: static void QToolTip::hideText();
  fn _ZN8QToolTip8hideTextEv();
  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect);
  fn _ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRect(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  void QToolTip::QToolTip();
  fn _ZN8QToolTipC1Ev(qthis: *mut c_void);
  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect, int msecShowTime);
  fn _ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRecti(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void, arg4: c_int);
  // proto: static QString QToolTip::text();
  fn _ZN8QToolTip4textEv() -> *mut c_void;
  // proto: static QFont QToolTip::font();
  fn _ZN8QToolTip4fontEv() -> *mut c_void;
  // proto: static void QToolTip::setPalette(const QPalette & );
  fn _ZN8QToolTip10setPaletteERK8QPalette(arg0: *mut c_void);
  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w);
  fn _ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto: static bool QToolTip::isVisible();
  fn _ZN8QToolTip9isVisibleEv() -> c_char;
}

// body block begin
// class sizeof(QToolTip)=1
pub struct QToolTip {
  pub qclsinst: *mut c_void,
}

  // proto: static void QToolTip::setFont(const QFont & );
impl /*struct*/ QToolTip {
  pub fn setFont_s<RetType, T: QToolTip_setFont_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFont_s();
    // return 1;
  }
}

pub trait QToolTip_setFont_s<RetType> {
  fn setFont_s(self ) -> RetType;
}

  // proto: static void QToolTip::setFont(const QFont & );
impl<'a> /*trait*/ QToolTip_setFont_s<()> for (QFont) {
  fn setFont_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolTip7setFontERK5QFont(arg0)};
    // return 1;
  }
}

  // proto: static QPalette QToolTip::palette();
impl /*struct*/ QToolTip {
  pub fn palette_s<RetType, T: QToolTip_palette_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.palette_s();
    // return 1;
  }
}

pub trait QToolTip_palette_s<RetType> {
  fn palette_s(self ) -> RetType;
}

  // proto: static QPalette QToolTip::palette();
impl<'a> /*trait*/ QToolTip_palette_s<QPalette> for () {
  fn palette_s(self ) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip7paletteEv()};
    let mut ret = unsafe {_ZN8QToolTip7paletteEv()};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static void QToolTip::hideText();
impl /*struct*/ QToolTip {
  pub fn hideText_s<RetType, T: QToolTip_hideText_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hideText_s();
    // return 1;
  }
}

pub trait QToolTip_hideText_s<RetType> {
  fn hideText_s(self ) -> RetType;
}

  // proto: static void QToolTip::hideText();
impl<'a> /*trait*/ QToolTip_hideText_s<()> for () {
  fn hideText_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip8hideTextEv()};
     unsafe {_ZN8QToolTip8hideTextEv()};
    // return 1;
  }
}

  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect);
impl /*struct*/ QToolTip {
  pub fn showText_s<RetType, T: QToolTip_showText_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.showText_s();
    // return 1;
  }
}

pub trait QToolTip_showText_s<RetType> {
  fn showText_s(self ) -> RetType;
}

  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect);
impl<'a> /*trait*/ QToolTip_showText_s<()> for (QPoint, QString, QWidget, QRect) {
  fn showText_s(self ) -> () {
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

  // proto:  void QToolTip::QToolTip();
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

  // proto:  void QToolTip::QToolTip();
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
impl<'a> /*trait*/ QToolTip_showText_s<()> for (QPoint, QString, QWidget, QRect, i32) {
  fn showText_s(self ) -> () {
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

  // proto: static QString QToolTip::text();
impl /*struct*/ QToolTip {
  pub fn text_s<RetType, T: QToolTip_text_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.text_s();
    // return 1;
  }
}

pub trait QToolTip_text_s<RetType> {
  fn text_s(self ) -> RetType;
}

  // proto: static QString QToolTip::text();
impl<'a> /*trait*/ QToolTip_text_s<QString> for () {
  fn text_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip4textEv()};
    let mut ret = unsafe {_ZN8QToolTip4textEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static QFont QToolTip::font();
impl /*struct*/ QToolTip {
  pub fn font_s<RetType, T: QToolTip_font_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.font_s();
    // return 1;
  }
}

pub trait QToolTip_font_s<RetType> {
  fn font_s(self ) -> RetType;
}

  // proto: static QFont QToolTip::font();
impl<'a> /*trait*/ QToolTip_font_s<QFont> for () {
  fn font_s(self ) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip4fontEv()};
    let mut ret = unsafe {_ZN8QToolTip4fontEv()};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto: static void QToolTip::setPalette(const QPalette & );
impl /*struct*/ QToolTip {
  pub fn setPalette_s<RetType, T: QToolTip_setPalette_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPalette_s();
    // return 1;
  }
}

pub trait QToolTip_setPalette_s<RetType> {
  fn setPalette_s(self ) -> RetType;
}

  // proto: static void QToolTip::setPalette(const QPalette & );
impl<'a> /*trait*/ QToolTip_setPalette_s<()> for (QPalette) {
  fn setPalette_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolTip10setPaletteERK8QPalette(arg0)};
    // return 1;
  }
}

  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w);
impl<'a> /*trait*/ QToolTip_showText_s<()> for (QPoint, QString, QWidget) {
  fn showText_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidget(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto: static bool QToolTip::isVisible();
impl /*struct*/ QToolTip {
  pub fn isVisible_s<RetType, T: QToolTip_isVisible_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isVisible_s();
    // return 1;
  }
}

pub trait QToolTip_isVisible_s<RetType> {
  fn isVisible_s(self ) -> RetType;
}

  // proto: static bool QToolTip::isVisible();
impl<'a> /*trait*/ QToolTip_isVisible_s<i8> for () {
  fn isVisible_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip9isVisibleEv()};
    let mut ret = unsafe {_ZN8QToolTip9isVisibleEv()};
    return ret as i8;
    // return 1;
  }
}

