// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtWidgets/qstylepainter.h
// dst-file: /src/widgets/qstylepainter.rs
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
use super::super::gui::qpainter::QPainter; // 771
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::gui::qpaintdevice::QPaintDevice; // 771
use super::qstyleoption::QStyleOptionComplex; // 773
use super::super::core::qrect::QRect; // 771
use super::super::gui::qpalette::QPalette; // 771
use super::super::core::qstring::QString; // 771
use super::super::gui::qpixmap::QPixmap; // 771
use super::qstyle::QStyle; // 773
use super::qstyleoption::QStyleOption; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  void QStylePainter::QStylePainter(QWidget * w);
  fn _ZN13QStylePainterC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStylePainter::QStylePainter(QPaintDevice * pd, QWidget * w);
  fn _ZN13QStylePainterC1EP12QPaintDeviceP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QStylePainter::QStylePainter();
  fn _ZN13QStylePainterC1Ev(qthis: *mut c_void);
  // proto:  bool QStylePainter::begin(QPaintDevice * pd, QWidget * w);
  fn _ZN13QStylePainter5beginEP12QPaintDeviceP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QStylePainter::QStylePainter(const QStylePainter & );
  fn _ZN13QStylePainterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QStylePainter::begin(QWidget * w);
  fn _ZN13QStylePainter5beginEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QStylePainter::drawItemPixmap(const QRect & r, int flags, const QPixmap & pixmap);
  fn _ZN13QStylePainter14drawItemPixmapERK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  QStyle * QStylePainter::style();
  fn _ZNK13QStylePainter5styleEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QStylePainter)=1
pub struct QStylePainter {
  qbase: QPainter,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStylePainter {
  pub fn inheritFrom(qthis: *mut c_void) -> QStylePainter {
    return QStylePainter{qbase: QPainter::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStylePainter {
  type Target = QPainter;

  fn deref(&self) -> &QPainter {
    return & self.qbase;
  }
}
impl AsRef<QPainter> for QStylePainter {
  fn as_ref(& self) -> & QPainter {
    return & self.qbase;
  }
}
  // proto:  void QStylePainter::QStylePainter(QWidget * w);
impl /*struct*/ QStylePainter {
  pub fn New<T: QStylePainter_New>(value: T) -> QStylePainter {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QStylePainter_New {
  fn New(self) -> QStylePainter;
}

  // proto:  void QStylePainter::QStylePainter(QWidget * w);
impl<'a> /*trait*/ QStylePainter_New for (&'a QWidget) {
  fn New(self) -> QStylePainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStylePainterC1EP7QWidget(qthis, arg0)};
    let rsthis = QStylePainter{/**/qbase: QPainter::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStylePainter::QStylePainter(QPaintDevice * pd, QWidget * w);
impl<'a> /*trait*/ QStylePainter_New for (&'a QPaintDevice, &'a QWidget) {
  fn New(self) -> QStylePainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1EP12QPaintDeviceP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QStylePainterC1EP12QPaintDeviceP7QWidget(qthis, arg0, arg1)};
    let rsthis = QStylePainter{/**/qbase: QPainter::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStylePainter::QStylePainter();
impl<'a> /*trait*/ QStylePainter_New for () {
  fn New(self) -> QStylePainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1Ev()};
    unsafe {_ZN13QStylePainterC1Ev(qthis)};
    let rsthis = QStylePainter{/**/qbase: QPainter::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QStylePainter::begin(QPaintDevice * pd, QWidget * w);
impl /*struct*/ QStylePainter {
  pub fn begin<RetType, T: QStylePainter_begin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QStylePainter_begin<RetType> {
  fn begin(self , rsthis: & QStylePainter) -> RetType;
}

  // proto:  bool QStylePainter::begin(QPaintDevice * pd, QWidget * w);
impl<'a> /*trait*/ QStylePainter_begin<i8> for (&'a QPaintDevice, &'a QWidget) {
  fn begin(self , rsthis: & QStylePainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainter5beginEP12QPaintDeviceP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QStylePainter5beginEP12QPaintDeviceP7QWidget(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QStylePainter::QStylePainter(const QStylePainter & );
impl<'a> /*trait*/ QStylePainter_New for (&'a QStylePainter) {
  fn New(self) -> QStylePainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStylePainterC1ERKS_(qthis, arg0)};
    let rsthis = QStylePainter{/**/qbase: QPainter::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QStylePainter::begin(QWidget * w);
impl<'a> /*trait*/ QStylePainter_begin<i8> for (&'a QWidget) {
  fn begin(self , rsthis: & QStylePainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainter5beginEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QStylePainter5beginEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QStylePainter::drawItemPixmap(const QRect & r, int flags, const QPixmap & pixmap);
impl /*struct*/ QStylePainter {
  pub fn drawItemPixmap<RetType, T: QStylePainter_drawItemPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawItemPixmap(self);
    // return 1;
  }
}

pub trait QStylePainter_drawItemPixmap<RetType> {
  fn drawItemPixmap(self , rsthis: & QStylePainter) -> RetType;
}

  // proto:  void QStylePainter::drawItemPixmap(const QRect & r, int flags, const QPixmap & pixmap);
impl<'a> /*trait*/ QStylePainter_drawItemPixmap<()> for (&'a QRect, i32, &'a QPixmap) {
  fn drawItemPixmap(self , rsthis: & QStylePainter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainter14drawItemPixmapERK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QStylePainter14drawItemPixmapERK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QStyle * QStylePainter::style();
impl /*struct*/ QStylePainter {
  pub fn style<RetType, T: QStylePainter_style<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.style(self);
    // return 1;
  }
}

pub trait QStylePainter_style<RetType> {
  fn style(self , rsthis: & QStylePainter) -> RetType;
}

  // proto:  QStyle * QStylePainter::style();
impl<'a> /*trait*/ QStylePainter_style<QStyle> for () {
  fn style(self , rsthis: & QStylePainter) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStylePainter5styleEv()};
    let mut ret = unsafe {_ZNK13QStylePainter5styleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

