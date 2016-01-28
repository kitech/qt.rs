// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qrubberband.h
// dst-file: /src/widgets/qrubberband.rs
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
use super::qwidget::*; // 773
use std::ops::Deref;
use super::super::core::qsize::*; // 771
use super::super::core::qpoint::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qrect::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRubberBand_Class_Size() -> c_int;
  // proto:  void QRubberBand::resize(const QSize & s);
  fn C_ZN11QRubberBand6resizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QRubberBand::setGeometry(int x, int y, int w, int h);
  fn C_ZN11QRubberBand11setGeometryEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QRubberBand::move(const QPoint & p);
  fn C_ZN11QRubberBand4moveERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QRubberBand::~QRubberBand();
  fn C_ZN11QRubberBandD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QRubberBand::move(int x, int y);
  fn C_ZN11QRubberBand4moveEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  const QMetaObject * QRubberBand::metaObject();
  fn C_ZNK11QRubberBand10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRubberBand::setGeometry(const QRect & r);
  fn C_ZN11QRubberBand11setGeometryERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QRubberBand::resize(int w, int h);
  fn C_ZN11QRubberBand6resizeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QRubberBand)=1
#[derive(Default)]
pub struct QRubberBand {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QRubberBand {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRubberBand {
    return QRubberBand{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QRubberBand {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QRubberBand {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QRubberBand::resize(const QSize & s);
impl /*struct*/ QRubberBand {
  pub fn resize<RetType, T: QRubberBand_resize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QRubberBand_resize<RetType> {
  fn resize(self , rsthis: & QRubberBand) -> RetType;
}

  // proto:  void QRubberBand::resize(const QSize & s);
impl<'a> /*trait*/ QRubberBand_resize<()> for (&'a QSize) {
  fn resize(self , rsthis: & QRubberBand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand6resizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QRubberBand6resizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRubberBand::setGeometry(int x, int y, int w, int h);
impl /*struct*/ QRubberBand {
  pub fn setGeometry<RetType, T: QRubberBand_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QRubberBand_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QRubberBand) -> RetType;
}

  // proto:  void QRubberBand::setGeometry(int x, int y, int w, int h);
impl<'a> /*trait*/ QRubberBand_setGeometry<()> for (i32, i32, i32, i32) {
  fn setGeometry(self , rsthis: & QRubberBand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand11setGeometryEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {C_ZN11QRubberBand11setGeometryEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QRubberBand::move(const QPoint & p);
impl /*struct*/ QRubberBand {
  pub fn move_<RetType, T: QRubberBand_move_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.move_(self);
    // return 1;
  }
}

pub trait QRubberBand_move_<RetType> {
  fn move_(self , rsthis: & QRubberBand) -> RetType;
}

  // proto:  void QRubberBand::move(const QPoint & p);
impl<'a> /*trait*/ QRubberBand_move_<()> for (&'a QPoint) {
  fn move_(self , rsthis: & QRubberBand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand4moveERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QRubberBand4moveERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRubberBand::~QRubberBand();
impl /*struct*/ QRubberBand {
  pub fn free<RetType, T: QRubberBand_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QRubberBand_free<RetType> {
  fn free(self , rsthis: & QRubberBand) -> RetType;
}

  // proto:  void QRubberBand::~QRubberBand();
impl<'a> /*trait*/ QRubberBand_free<()> for () {
  fn free(self , rsthis: & QRubberBand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBandD2Ev()};
     unsafe {C_ZN11QRubberBandD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRubberBand::move(int x, int y);
impl<'a> /*trait*/ QRubberBand_move_<()> for (i32, i32) {
  fn move_(self , rsthis: & QRubberBand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand4moveEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN11QRubberBand4moveEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QRubberBand::metaObject();
impl /*struct*/ QRubberBand {
  pub fn metaObject<RetType, T: QRubberBand_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRubberBand_metaObject<RetType> {
  fn metaObject(self , rsthis: & QRubberBand) -> RetType;
}

  // proto:  const QMetaObject * QRubberBand::metaObject();
impl<'a> /*trait*/ QRubberBand_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QRubberBand) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QRubberBand10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QRubberBand10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRubberBand::setGeometry(const QRect & r);
impl<'a> /*trait*/ QRubberBand_setGeometry<()> for (&'a QRect) {
  fn setGeometry(self , rsthis: & QRubberBand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand11setGeometryERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QRubberBand11setGeometryERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRubberBand::resize(int w, int h);
impl<'a> /*trait*/ QRubberBand_resize<()> for (i32, i32) {
  fn resize(self , rsthis: & QRubberBand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QRubberBand6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN11QRubberBand6resizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

