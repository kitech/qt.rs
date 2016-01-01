// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
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
  fn QStylePainter_Class_Size() -> c_int;
  // proto:  void QStylePainter::QStylePainter(QWidget * w);
  fn dector_ZN13QStylePainterC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn demth_ZN13QStylePainterC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStylePainter::QStylePainter(QPaintDevice * pd, QWidget * w);
  fn dector_ZN13QStylePainterC1EP12QPaintDeviceP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn demth_ZN13QStylePainterC1EP12QPaintDeviceP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QStylePainter::QStylePainter();
  fn dector_ZN13QStylePainterC1Ev() -> *mut c_void;
  fn demth_ZN13QStylePainterC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QStylePainter::begin(QPaintDevice * pd, QWidget * w);
  fn demth_ZN13QStylePainter5beginEP12QPaintDeviceP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  void QStylePainter::QStylePainter(const QStylePainter & );
  fn dector_ZN13QStylePainterC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QStylePainterC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QStylePainter::begin(QWidget * w);
  fn demth_ZN13QStylePainter5beginEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QStylePainter::drawItemPixmap(const QRect & r, int flags, const QPixmap & pixmap);
  fn demth_ZN13QStylePainter14drawItemPixmapERK5QRectiRK7QPixmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  QStyle * QStylePainter::style();
  fn demth_ZNK13QStylePainter5styleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QStylePainter)=1
#[derive(Default)]
pub struct QStylePainter {
  qbase: QPainter,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStylePainter {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStylePainter {
    return QStylePainter{qbase: QPainter::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
  pub fn new<T: QStylePainter_new>(value: T) -> QStylePainter {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStylePainter_new {
  fn new(self) -> QStylePainter;
}

  // proto:  void QStylePainter::QStylePainter(QWidget * w);
impl<'a> /*trait*/ QStylePainter_new for (&'a QWidget) {
  fn new(self) -> QStylePainter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1EP7QWidget()};
    let ctysz: c_int = unsafe{QStylePainter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QStylePainterC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN13QStylePainterC1EP7QWidget(arg0)} as u64;
    let rsthis = QStylePainter{qbase: QPainter::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStylePainter::QStylePainter(QPaintDevice * pd, QWidget * w);
impl<'a> /*trait*/ QStylePainter_new for (&'a QPaintDevice, &'a QWidget) {
  fn new(self) -> QStylePainter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1EP12QPaintDeviceP7QWidget()};
    let ctysz: c_int = unsafe{QStylePainter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN13QStylePainterC1EP12QPaintDeviceP7QWidget(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN13QStylePainterC1EP12QPaintDeviceP7QWidget(arg0, arg1)} as u64;
    let rsthis = QStylePainter{qbase: QPainter::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStylePainter::QStylePainter();
impl<'a> /*trait*/ QStylePainter_new for () {
  fn new(self) -> QStylePainter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1Ev()};
    let ctysz: c_int = unsafe{QStylePainter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN13QStylePainterC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN13QStylePainterC1Ev()} as u64;
    let rsthis = QStylePainter{qbase: QPainter::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {demth_ZN13QStylePainter5beginEP12QPaintDeviceP7QWidget(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QStylePainter::QStylePainter(const QStylePainter & );
impl<'a> /*trait*/ QStylePainter_new for (&'a QStylePainter) {
  fn new(self) -> QStylePainter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1ERKS_()};
    let ctysz: c_int = unsafe{QStylePainter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QStylePainterC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN13QStylePainterC1ERKS_(arg0)} as u64;
    let rsthis = QStylePainter{qbase: QPainter::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
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
    let mut ret = unsafe {demth_ZN13QStylePainter5beginEP7QWidget(rsthis.qclsinst, arg0)};
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
     unsafe {demth_ZN13QStylePainter14drawItemPixmapERK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
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
    let mut ret = unsafe {demth_ZNK13QStylePainter5styleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

