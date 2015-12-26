// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
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
use super::qstyleoption::QStyleOptionComplex; // 773
use super::super::core::qrect::QRect; // 771
use super::super::gui::qpalette::QPalette; // 771
use super::super::core::qstring::QString; // 771
use super::qstyleoption::QStyleOption; // 773
use super::super::gui::qpixmap::QPixmap; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStylePainter_Class_Size() -> c_int;
  // proto:  void QStylePainter::QStylePainter(const QStylePainter & );
  fn dector_ZN13QStylePainterC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QStylePainterC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStylePainter::drawItemPixmap(const QRect & r, int flags, const QPixmap & pixmap);
  fn _ZN13QStylePainter14drawItemPixmapERK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
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
  // proto:  void QStylePainter::QStylePainter(const QStylePainter & );
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

  // proto:  void QStylePainter::QStylePainter(const QStylePainter & );
impl<'a> /*trait*/ QStylePainter_New for (&'a QStylePainter) {
  fn New(self) -> QStylePainter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QStylePainterC1ERKS_()};
    let ctysz: c_int = unsafe{QStylePainter_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QStylePainterC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QStylePainterC1ERKS_(arg0)};
    let rsthis = QStylePainter{/**/qbase: QPainter::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
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

// <= body block end

