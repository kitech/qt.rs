// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qpaintdevice::QPaintDevice;
use super::qstyleoptioncomplex::QStyleOptionComplex;
use super::qrect::QRect;
use super::qpalette::QPalette;
use super::qstring::QString;
use super::qpixmap::QPixmap;
use super::qstyle::QStyle;
use super::qstyleoption::QStyleOption;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
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
}

// body block begin
// class sizeof(QStylePainter)=1
pub struct QStylePainter {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStylePainter::QStylePainter(QWidget * w);
impl /*struct*/ QStylePainter {
  pub fn NewQStylePainter<T: QStylePainter_NewQStylePainter>(value: T) -> QStylePainter {
    let rsthis = value.NewQStylePainter();
    return rsthis;
    // return 1;
  }
}

pub trait QStylePainter_NewQStylePainter {
  fn NewQStylePainter(self) -> QStylePainter;
}

  // proto:  void QStylePainter::QStylePainter(QWidget * w);
impl<'a> /*trait*/ QStylePainter_NewQStylePainter for (QWidget) {
  fn NewQStylePainter(self) -> QStylePainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStylePainterC1EP7QWidget(qthis, arg0)};
    let rsthis = QStylePainter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStylePainter::QStylePainter(QPaintDevice * pd, QWidget * w);
impl<'a> /*trait*/ QStylePainter_NewQStylePainter for (QPaintDevice, QWidget) {
  fn NewQStylePainter(self) -> QStylePainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1EP12QPaintDeviceP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN13QStylePainterC1EP12QPaintDeviceP7QWidget(qthis, arg0, arg1)};
    let rsthis = QStylePainter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStylePainter::QStylePainter();
impl<'a> /*trait*/ QStylePainter_NewQStylePainter for () {
  fn NewQStylePainter(self) -> QStylePainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1Ev()};
    unsafe {_ZN13QStylePainterC1Ev(qthis)};
    let rsthis = QStylePainter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QStylePainter::begin(QPaintDevice * pd, QWidget * w);
impl /*struct*/ QStylePainter {
  pub fn begin<RetType, T: QStylePainter_begin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QStylePainter_begin<RetType> {
  fn begin(self , rsthis: &mut QStylePainter) -> RetType;
}

  // proto:  bool QStylePainter::begin(QPaintDevice * pd, QWidget * w);
impl<'a> /*trait*/ QStylePainter_begin<i8> for (QPaintDevice, QWidget) {
  fn begin(self , rsthis: &mut QStylePainter) -> i8 {
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
impl<'a> /*trait*/ QStylePainter_NewQStylePainter for (QStylePainter) {
  fn NewQStylePainter(self) -> QStylePainter {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QStylePainterC1ERKS_(qthis, arg0)};
    let rsthis = QStylePainter{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QStylePainter::begin(QWidget * w);
impl<'a> /*trait*/ QStylePainter_begin<i8> for (QWidget) {
  fn begin(self , rsthis: &mut QStylePainter) -> i8 {
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
  pub fn drawItemPixmap<RetType, T: QStylePainter_drawItemPixmap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawItemPixmap(self);
    // return 1;
  }
}

pub trait QStylePainter_drawItemPixmap<RetType> {
  fn drawItemPixmap(self , rsthis: &mut QStylePainter) -> RetType;
}

  // proto:  void QStylePainter::drawItemPixmap(const QRect & r, int flags, const QPixmap & pixmap);
impl<'a> /*trait*/ QStylePainter_drawItemPixmap<()> for (QRect, i32, QPixmap) {
  fn drawItemPixmap(self , rsthis: &mut QStylePainter) -> () {
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
  pub fn style<RetType, T: QStylePainter_style<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.style(self);
    // return 1;
  }
}

pub trait QStylePainter_style<RetType> {
  fn style(self , rsthis: &mut QStylePainter) -> RetType;
}

  // proto:  QStyle * QStylePainter::style();
impl<'a> /*trait*/ QStylePainter_style<QStyle> for () {
  fn style(self , rsthis: &mut QStylePainter) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStylePainter5styleEv()};
    let mut ret = unsafe {_ZNK13QStylePainter5styleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

