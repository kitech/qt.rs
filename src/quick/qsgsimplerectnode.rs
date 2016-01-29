// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgsimplerectnode.h
// dst-file: /src/quick/qsgsimplerectnode.rs
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
use super::qsgnode::QSGGeometryNode; // 773
use std::ops::Deref;
use super::super::core::qrect::QRectF; // 771
use super::super::gui::qcolor::QColor; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGSimpleRectNode_Class_Size() -> c_int;
  // proto:  void QSGSimpleRectNode::QSGSimpleRectNode();
  fn _ZN17QSGSimpleRectNodeC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QRectF QSGSimpleRectNode::rect();
  fn _ZNK17QSGSimpleRectNode4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGSimpleRectNode::QSGSimpleRectNode(const QRectF & rect, const QColor & color);
  fn _ZN17QSGSimpleRectNodeC2ERK6QRectFRK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QColor QSGSimpleRectNode::color();
  fn _ZNK17QSGSimpleRectNode5colorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGSimpleRectNode::setColor(const QColor & color);
  fn _ZN17QSGSimpleRectNode8setColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGSimpleRectNode::setRect(const QRectF & rect);
  fn _ZN17QSGSimpleRectNode7setRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGSimpleRectNode::setRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN17QSGSimpleRectNode7setRectEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QSGSimpleRectNode)=1
#[derive(Default)]
pub struct QSGSimpleRectNode {
  qbase: QSGGeometryNode,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSGSimpleRectNode {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGSimpleRectNode {
    return QSGSimpleRectNode{qbase: QSGGeometryNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGSimpleRectNode {
  type Target = QSGGeometryNode;

  fn deref(&self) -> &QSGGeometryNode {
    return & self.qbase;
  }
}
impl AsRef<QSGGeometryNode> for QSGSimpleRectNode {
  fn as_ref(& self) -> & QSGGeometryNode {
    return & self.qbase;
  }
}
  // proto:  void QSGSimpleRectNode::QSGSimpleRectNode();
impl /*struct*/ QSGSimpleRectNode {
  pub fn new<T: QSGSimpleRectNode_new>(value: T) -> QSGSimpleRectNode {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGSimpleRectNode_new {
  fn new(self) -> QSGSimpleRectNode;
}

  // proto:  void QSGSimpleRectNode::QSGSimpleRectNode();
impl<'a> /*trait*/ QSGSimpleRectNode_new for () {
  fn new(self) -> QSGSimpleRectNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGSimpleRectNodeC2Ev()};
    let ctysz: c_int = unsafe{QSGSimpleRectNode_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN17QSGSimpleRectNodeC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGSimpleRectNode{qbase: QSGGeometryNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRectF QSGSimpleRectNode::rect();
impl /*struct*/ QSGSimpleRectNode {
  pub fn rect<RetType, T: QSGSimpleRectNode_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QSGSimpleRectNode_rect<RetType> {
  fn rect(self , rsthis: & QSGSimpleRectNode) -> RetType;
}

  // proto:  QRectF QSGSimpleRectNode::rect();
impl<'a> /*trait*/ QSGSimpleRectNode_rect<QRectF> for () {
  fn rect(self , rsthis: & QSGSimpleRectNode) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSGSimpleRectNode4rectEv()};
    let mut ret = unsafe {_ZNK17QSGSimpleRectNode4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGSimpleRectNode::QSGSimpleRectNode(const QRectF & rect, const QColor & color);
impl<'a> /*trait*/ QSGSimpleRectNode_new for (&'a QRectF, &'a QColor) {
  fn new(self) -> QSGSimpleRectNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGSimpleRectNodeC2ERK6QRectFRK6QColor()};
    let ctysz: c_int = unsafe{QSGSimpleRectNode_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN17QSGSimpleRectNodeC2ERK6QRectFRK6QColor(qthis_ph, arg0, arg1)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGSimpleRectNode{qbase: QSGGeometryNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QColor QSGSimpleRectNode::color();
impl /*struct*/ QSGSimpleRectNode {
  pub fn color<RetType, T: QSGSimpleRectNode_color<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QSGSimpleRectNode_color<RetType> {
  fn color(self , rsthis: & QSGSimpleRectNode) -> RetType;
}

  // proto:  QColor QSGSimpleRectNode::color();
impl<'a> /*trait*/ QSGSimpleRectNode_color<QColor> for () {
  fn color(self , rsthis: & QSGSimpleRectNode) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSGSimpleRectNode5colorEv()};
    let mut ret = unsafe {_ZNK17QSGSimpleRectNode5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGSimpleRectNode::setColor(const QColor & color);
impl /*struct*/ QSGSimpleRectNode {
  pub fn setColor<RetType, T: QSGSimpleRectNode_setColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QSGSimpleRectNode_setColor<RetType> {
  fn setColor(self , rsthis: & QSGSimpleRectNode) -> RetType;
}

  // proto:  void QSGSimpleRectNode::setColor(const QColor & color);
impl<'a> /*trait*/ QSGSimpleRectNode_setColor<()> for (&'a QColor) {
  fn setColor(self , rsthis: & QSGSimpleRectNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGSimpleRectNode8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSGSimpleRectNode8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGSimpleRectNode::setRect(const QRectF & rect);
impl /*struct*/ QSGSimpleRectNode {
  pub fn setRect<RetType, T: QSGSimpleRectNode_setRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRect(self);
    // return 1;
  }
}

pub trait QSGSimpleRectNode_setRect<RetType> {
  fn setRect(self , rsthis: & QSGSimpleRectNode) -> RetType;
}

  // proto:  void QSGSimpleRectNode::setRect(const QRectF & rect);
impl<'a> /*trait*/ QSGSimpleRectNode_setRect<()> for (&'a QRectF) {
  fn setRect(self , rsthis: & QSGSimpleRectNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGSimpleRectNode7setRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSGSimpleRectNode7setRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGSimpleRectNode::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QSGSimpleRectNode_setRect<()> for (f64, f64, f64, f64) {
  fn setRect(self , rsthis: & QSGSimpleRectNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSGSimpleRectNode7setRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN17QSGSimpleRectNode7setRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// <= body block end

