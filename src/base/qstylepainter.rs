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
use super::qrect::QRect;
use super::qpixmap::QPixmap;
use super::qstyle::QStyle;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStylePainter::NewQStylePainter(QWidget * w);
  fn _ZN13QStylePainterC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStylePainter::NewQStylePainter(QPaintDevice * pd, QWidget * w);
  fn _ZN13QStylePainterC1EP12QPaintDeviceP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QStylePainter::NewQStylePainter();
  fn _ZN13QStylePainterC1Ev(qthis: *mut c_void) ;
  // proto:  bool QStylePainter::begin(QPaintDevice * pd, QWidget * w);
  fn _ZN13QStylePainter5beginEP12QPaintDeviceP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> int8_t;
  // proto:  void QStylePainter::NewQStylePainter(const QStylePainter & );
  fn _ZN13QStylePainterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QStylePainter::begin(QWidget * w);
  fn _ZN13QStylePainter5beginEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QStylePainter::drawItemPixmap(const QRect & r, int flags, const QPixmap & pixmap);
  fn _ZN13QStylePainter14drawItemPixmapERK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) ;
  // proto:  QStyle * QStylePainter::style();
  fn _ZNK13QStylePainter5styleEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QStylePainter)=1
pub struct QStylePainter {
  pub qclsinst: *mut c_void,
}

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

// proto: void QStylePainter::NewQStylePainter(QWidget * w);
impl<'a> /*trait*/ QStylePainter_NewQStylePainter for (&'a mut QWidget) {
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

// proto: void QStylePainter::NewQStylePainter(QPaintDevice * pd, QWidget * w);
impl<'a> /*trait*/ QStylePainter_NewQStylePainter for (&'a mut QPaintDevice, &'a mut QWidget) {
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

// proto: void QStylePainter::NewQStylePainter();
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

impl /*struct*/ QStylePainter {
  pub fn begin<T: QStylePainter_begin>(&mut self, value: T) -> i8 {
    return value.begin(self);
    // return 1;
  }
}

pub trait QStylePainter_begin {
  fn begin(self, rsthis: &mut QStylePainter) -> i8;
}

// proto:  bool QStylePainter::begin(QPaintDevice * pd, QWidget * w);
impl<'a> /*trait*/ QStylePainter_begin for (&'a mut QPaintDevice, &'a mut QWidget) {
  fn begin(self, rsthis: &mut QStylePainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainter5beginEP12QPaintDeviceP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QStylePainter5beginEP12QPaintDeviceP7QWidget(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QStylePainter::NewQStylePainter(const QStylePainter & );
impl<'a> /*trait*/ QStylePainter_NewQStylePainter for (&'a  QStylePainter) {
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
impl<'a> /*trait*/ QStylePainter_begin for (&'a mut QWidget) {
  fn begin(self, rsthis: &mut QStylePainter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainter5beginEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN13QStylePainter5beginEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStylePainter {
  pub fn drawItemPixmap<T: QStylePainter_drawItemPixmap>(&mut self, value: T)  {
     value.drawItemPixmap(self);
    // return 1;
  }
}

pub trait QStylePainter_drawItemPixmap {
  fn drawItemPixmap(self, rsthis: &mut QStylePainter) ;
}

// proto:  void QStylePainter::drawItemPixmap(const QRect & r, int flags, const QPixmap & pixmap);
impl<'a> /*trait*/ QStylePainter_drawItemPixmap for (&'a  QRect, i32, &'a  QPixmap) {
  fn drawItemPixmap(self, rsthis: &mut QStylePainter)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainter14drawItemPixmapERK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN13QStylePainter14drawItemPixmapERK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QStylePainter {
  pub fn style<T: QStylePainter_style>(&mut self, value: T) -> QStyle {
    return value.style(self);
    // return 1;
  }
}

pub trait QStylePainter_style {
  fn style(self, rsthis: &mut QStylePainter) -> QStyle;
}

// proto:  QStyle * QStylePainter::style();
impl<'a> /*trait*/ QStylePainter_style for () {
  fn style(self, rsthis: &mut QStylePainter) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QStylePainter5styleEv()};
    let mut ret = unsafe {_ZNK13QStylePainter5styleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

