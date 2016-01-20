// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsgsimpletexturenode.h
// dst-file: /src/quick/qsgsimpletexturenode.rs
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
use super::qsgtexture::QSGTexture; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGSimpleTextureNode_Class_Size() -> c_int;
  // proto:  void QSGSimpleTextureNode::setSourceRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN20QSGSimpleTextureNode13setSourceRectEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QSGSimpleTextureNode::setOwnsTexture(bool owns);
  fn _ZN20QSGSimpleTextureNode14setOwnsTextureEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QSGSimpleTextureNode::ownsTexture();
  fn _ZNK20QSGSimpleTextureNode11ownsTextureEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSGSimpleTextureNode::~QSGSimpleTextureNode();
  fn _ZN20QSGSimpleTextureNodeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSGSimpleTextureNode::setRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN20QSGSimpleTextureNode7setRectEdddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  QRectF QSGSimpleTextureNode::sourceRect();
  fn _ZNK20QSGSimpleTextureNode10sourceRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSGTexture * QSGSimpleTextureNode::texture();
  fn _ZNK20QSGSimpleTextureNode7textureEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGSimpleTextureNode::setRect(const QRectF & rect);
  fn _ZN20QSGSimpleTextureNode7setRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGSimpleTextureNode::setTexture(QSGTexture * texture);
  fn _ZN20QSGSimpleTextureNode10setTextureEP10QSGTexture(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSGSimpleTextureNode::setSourceRect(const QRectF & r);
  fn _ZN20QSGSimpleTextureNode13setSourceRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRectF QSGSimpleTextureNode::rect();
  fn _ZNK20QSGSimpleTextureNode4rectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGSimpleTextureNode::QSGSimpleTextureNode();
  fn _ZN20QSGSimpleTextureNodeC2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QSGSimpleTextureNode)=1
#[derive(Default)]
pub struct QSGSimpleTextureNode {
  qbase: QSGGeometryNode,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSGSimpleTextureNode {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGSimpleTextureNode {
    return QSGSimpleTextureNode{qbase: QSGGeometryNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSGSimpleTextureNode {
  type Target = QSGGeometryNode;

  fn deref(&self) -> &QSGGeometryNode {
    return & self.qbase;
  }
}
impl AsRef<QSGGeometryNode> for QSGSimpleTextureNode {
  fn as_ref(& self) -> & QSGGeometryNode {
    return & self.qbase;
  }
}
  // proto:  void QSGSimpleTextureNode::setSourceRect(qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QSGSimpleTextureNode {
  pub fn setSourceRect<RetType, T: QSGSimpleTextureNode_setSourceRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSourceRect(self);
    // return 1;
  }
}

pub trait QSGSimpleTextureNode_setSourceRect<RetType> {
  fn setSourceRect(self , rsthis: & QSGSimpleTextureNode) -> RetType;
}

  // proto:  void QSGSimpleTextureNode::setSourceRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QSGSimpleTextureNode_setSourceRect<()> for (f64, f64, f64, f64) {
  fn setSourceRect(self , rsthis: & QSGSimpleTextureNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGSimpleTextureNode13setSourceRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN20QSGSimpleTextureNode13setSourceRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QSGSimpleTextureNode::setOwnsTexture(bool owns);
impl /*struct*/ QSGSimpleTextureNode {
  pub fn setOwnsTexture<RetType, T: QSGSimpleTextureNode_setOwnsTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOwnsTexture(self);
    // return 1;
  }
}

pub trait QSGSimpleTextureNode_setOwnsTexture<RetType> {
  fn setOwnsTexture(self , rsthis: & QSGSimpleTextureNode) -> RetType;
}

  // proto:  void QSGSimpleTextureNode::setOwnsTexture(bool owns);
impl<'a> /*trait*/ QSGSimpleTextureNode_setOwnsTexture<()> for (i8) {
  fn setOwnsTexture(self , rsthis: & QSGSimpleTextureNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGSimpleTextureNode14setOwnsTextureEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN20QSGSimpleTextureNode14setOwnsTextureEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSGSimpleTextureNode::ownsTexture();
impl /*struct*/ QSGSimpleTextureNode {
  pub fn ownsTexture<RetType, T: QSGSimpleTextureNode_ownsTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ownsTexture(self);
    // return 1;
  }
}

pub trait QSGSimpleTextureNode_ownsTexture<RetType> {
  fn ownsTexture(self , rsthis: & QSGSimpleTextureNode) -> RetType;
}

  // proto:  bool QSGSimpleTextureNode::ownsTexture();
impl<'a> /*trait*/ QSGSimpleTextureNode_ownsTexture<i8> for () {
  fn ownsTexture(self , rsthis: & QSGSimpleTextureNode) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QSGSimpleTextureNode11ownsTextureEv()};
    let mut ret = unsafe {_ZNK20QSGSimpleTextureNode11ownsTextureEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSGSimpleTextureNode::~QSGSimpleTextureNode();
impl /*struct*/ QSGSimpleTextureNode {
  pub fn free<RetType, T: QSGSimpleTextureNode_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGSimpleTextureNode_free<RetType> {
  fn free(self , rsthis: & QSGSimpleTextureNode) -> RetType;
}

  // proto:  void QSGSimpleTextureNode::~QSGSimpleTextureNode();
impl<'a> /*trait*/ QSGSimpleTextureNode_free<()> for () {
  fn free(self , rsthis: & QSGSimpleTextureNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGSimpleTextureNodeD2Ev()};
     unsafe {_ZN20QSGSimpleTextureNodeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSGSimpleTextureNode::setRect(qreal x, qreal y, qreal w, qreal h);
impl /*struct*/ QSGSimpleTextureNode {
  pub fn setRect<RetType, T: QSGSimpleTextureNode_setRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRect(self);
    // return 1;
  }
}

pub trait QSGSimpleTextureNode_setRect<RetType> {
  fn setRect(self , rsthis: & QSGSimpleTextureNode) -> RetType;
}

  // proto:  void QSGSimpleTextureNode::setRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QSGSimpleTextureNode_setRect<()> for (f64, f64, f64, f64) {
  fn setRect(self , rsthis: & QSGSimpleTextureNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGSimpleTextureNode7setRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN20QSGSimpleTextureNode7setRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  QRectF QSGSimpleTextureNode::sourceRect();
impl /*struct*/ QSGSimpleTextureNode {
  pub fn sourceRect<RetType, T: QSGSimpleTextureNode_sourceRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sourceRect(self);
    // return 1;
  }
}

pub trait QSGSimpleTextureNode_sourceRect<RetType> {
  fn sourceRect(self , rsthis: & QSGSimpleTextureNode) -> RetType;
}

  // proto:  QRectF QSGSimpleTextureNode::sourceRect();
impl<'a> /*trait*/ QSGSimpleTextureNode_sourceRect<QRectF> for () {
  fn sourceRect(self , rsthis: & QSGSimpleTextureNode) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QSGSimpleTextureNode10sourceRectEv()};
    let mut ret = unsafe {_ZNK20QSGSimpleTextureNode10sourceRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSGTexture * QSGSimpleTextureNode::texture();
impl /*struct*/ QSGSimpleTextureNode {
  pub fn texture<RetType, T: QSGSimpleTextureNode_texture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.texture(self);
    // return 1;
  }
}

pub trait QSGSimpleTextureNode_texture<RetType> {
  fn texture(self , rsthis: & QSGSimpleTextureNode) -> RetType;
}

  // proto:  QSGTexture * QSGSimpleTextureNode::texture();
impl<'a> /*trait*/ QSGSimpleTextureNode_texture<QSGTexture> for () {
  fn texture(self , rsthis: & QSGSimpleTextureNode) -> QSGTexture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QSGSimpleTextureNode7textureEv()};
    let mut ret = unsafe {_ZNK20QSGSimpleTextureNode7textureEv(rsthis.qclsinst)};
    let mut ret1 = QSGTexture::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGSimpleTextureNode::setRect(const QRectF & rect);
impl<'a> /*trait*/ QSGSimpleTextureNode_setRect<()> for (&'a QRectF) {
  fn setRect(self , rsthis: & QSGSimpleTextureNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGSimpleTextureNode7setRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QSGSimpleTextureNode7setRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGSimpleTextureNode::setTexture(QSGTexture * texture);
impl /*struct*/ QSGSimpleTextureNode {
  pub fn setTexture<RetType, T: QSGSimpleTextureNode_setTexture<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTexture(self);
    // return 1;
  }
}

pub trait QSGSimpleTextureNode_setTexture<RetType> {
  fn setTexture(self , rsthis: & QSGSimpleTextureNode) -> RetType;
}

  // proto:  void QSGSimpleTextureNode::setTexture(QSGTexture * texture);
impl<'a> /*trait*/ QSGSimpleTextureNode_setTexture<()> for (&'a QSGTexture) {
  fn setTexture(self , rsthis: & QSGSimpleTextureNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGSimpleTextureNode10setTextureEP10QSGTexture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QSGSimpleTextureNode10setTextureEP10QSGTexture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGSimpleTextureNode::setSourceRect(const QRectF & r);
impl<'a> /*trait*/ QSGSimpleTextureNode_setSourceRect<()> for (&'a QRectF) {
  fn setSourceRect(self , rsthis: & QSGSimpleTextureNode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGSimpleTextureNode13setSourceRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN20QSGSimpleTextureNode13setSourceRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QSGSimpleTextureNode::rect();
impl /*struct*/ QSGSimpleTextureNode {
  pub fn rect<RetType, T: QSGSimpleTextureNode_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QSGSimpleTextureNode_rect<RetType> {
  fn rect(self , rsthis: & QSGSimpleTextureNode) -> RetType;
}

  // proto:  QRectF QSGSimpleTextureNode::rect();
impl<'a> /*trait*/ QSGSimpleTextureNode_rect<QRectF> for () {
  fn rect(self , rsthis: & QSGSimpleTextureNode) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK20QSGSimpleTextureNode4rectEv()};
    let mut ret = unsafe {_ZNK20QSGSimpleTextureNode4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSGSimpleTextureNode::QSGSimpleTextureNode();
impl /*struct*/ QSGSimpleTextureNode {
  pub fn new<T: QSGSimpleTextureNode_new>(value: T) -> QSGSimpleTextureNode {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSGSimpleTextureNode_new {
  fn new(self) -> QSGSimpleTextureNode;
}

  // proto:  void QSGSimpleTextureNode::QSGSimpleTextureNode();
impl<'a> /*trait*/ QSGSimpleTextureNode_new for () {
  fn new(self) -> QSGSimpleTextureNode {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QSGSimpleTextureNodeC2Ev()};
    let ctysz: c_int = unsafe{QSGSimpleTextureNode_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN20QSGSimpleTextureNodeC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QSGSimpleTextureNode{qbase: QSGGeometryNode::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

