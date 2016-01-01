// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtWidgets/qtooltip.h
// dst-file: /src/widgets/qtooltip.rs
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
use std::ops::Deref;
use super::super::gui::qfont::QFont; // 771
use super::super::gui::qpalette::QPalette; // 771
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qstring::QString; // 771
use super::qwidget::QWidget; // 773
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QToolTip_Class_Size() -> c_int;
  // proto: static void QToolTip::setFont(const QFont & );
  fn _ZN8QToolTip7setFontERK5QFont(arg0: *mut c_void);
  // proto: static QPalette QToolTip::palette();
  fn _ZN8QToolTip7paletteEv() -> *mut c_void;
  // proto: static void QToolTip::hideText();
  fn demth_ZN8QToolTip8hideTextEv();
  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect);
  fn _ZN8QToolTip8showTextERK6QPointRK7QStringP7QWidgetRK5QRect(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  void QToolTip::QToolTip();
  fn dector_ZN8QToolTipC1Ev() -> *mut c_void;
  fn _ZN8QToolTipC1Ev(qthis: u64 /* *mut c_void*/);
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
} // <= ext block end

// body block begin =>
// class sizeof(QToolTip)=1
#[derive(Default)]
pub struct QToolTip {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QToolTip {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QToolTip {
    return QToolTip{qclsinst: qthis, ..Default::default()};
  }
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
impl<'a> /*trait*/ QToolTip_setFont_s<()> for (&'a QFont) {
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
    let mut ret1 = QPalette::inheritFrom(ret as u64);
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
     unsafe {demth_ZN8QToolTip8hideTextEv()};
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
impl<'a> /*trait*/ QToolTip_showText_s<()> for (&'a QPoint, &'a QString, &'a QWidget, &'a QRect) {
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
  pub fn new<T: QToolTip_new>(value: T) -> QToolTip {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QToolTip_new {
  fn new(self) -> QToolTip;
}

  // proto:  void QToolTip::QToolTip();
impl<'a> /*trait*/ QToolTip_new for () {
  fn new(self) -> QToolTip {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTipC1Ev()};
    let ctysz: c_int = unsafe{QToolTip_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN8QToolTipC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN8QToolTipC1Ev()} as u64;
    let rsthis = QToolTip{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w, const QRect & rect, int msecShowTime);
impl<'a> /*trait*/ QToolTip_showText_s<()> for (&'a QPoint, &'a QString, &'a QWidget, &'a QRect, i32) {
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
    let mut ret1 = QString::inheritFrom(ret as u64);
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
    let mut ret1 = QFont::inheritFrom(ret as u64);
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
impl<'a> /*trait*/ QToolTip_setPalette_s<()> for (&'a QPalette) {
  fn setPalette_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolTip10setPaletteERK8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolTip10setPaletteERK8QPalette(arg0)};
    // return 1;
  }
}

  // proto: static void QToolTip::showText(const QPoint & pos, const QString & text, QWidget * w);
impl<'a> /*trait*/ QToolTip_showText_s<()> for (&'a QPoint, &'a QString, &'a QWidget) {
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

// <= body block end

