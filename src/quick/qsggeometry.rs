// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qsggeometry.h
// dst-file: /src/quick/qsggeometry.rs
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
use super::super::core::qrect::QRectF; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSGGeometry_Class_Size() -> c_int;
  // proto:  int QSGGeometry::attributeCount();
  fn _ZNK11QSGGeometry14attributeCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static void QSGGeometry::updateTexturedRectGeometry(QSGGeometry * g, const QRectF & rect, const QRectF & sourceRect);
  fn _ZN11QSGGeometry26updateTexturedRectGeometryEPS_RK6QRectFS3_(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QSGGeometry::~QSGGeometry();
  fn _ZN11QSGGeometryD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QSGGeometry::sizeOfVertex();
  fn _ZNK11QSGGeometry12sizeOfVertexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSGGeometry::setLineWidth(float w);
  fn _ZN11QSGGeometry12setLineWidthEf(qthis: u64 /* *mut c_void*/, arg0: c_float);
  // proto:  uint * QSGGeometry::indexDataAsUInt();
  fn _ZN11QSGGeometry15indexDataAsUIntEv(qthis: u64 /* *mut c_void*/) -> *mut c_uint;
  // proto:  int QSGGeometry::sizeOfIndex();
  fn _ZNK11QSGGeometry11sizeOfIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSGGeometry::setDrawingMode(GLenum mode);
  fn _ZN11QSGGeometry14setDrawingModeEj(qthis: u64 /* *mut c_void*/, arg0: c_uint);
  // proto:  void QSGGeometry::markIndexDataDirty();
  fn _ZN11QSGGeometry18markIndexDataDirtyEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QSGGeometry::indexCount();
  fn _ZNK11QSGGeometry10indexCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QSGGeometry::indexType();
  fn _ZNK11QSGGeometry9indexTypeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void * QSGGeometry::indexData();
  fn _ZN11QSGGeometry9indexDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  GLenum QSGGeometry::drawingMode();
  fn _ZNK11QSGGeometry11drawingModeEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  int QSGGeometry::vertexCount();
  fn _ZNK11QSGGeometry11vertexCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSGGeometry::allocate(int vertexCount, int indexCount);
  fn _ZN11QSGGeometry8allocateEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto: static void QSGGeometry::updateRectGeometry(QSGGeometry * g, const QRectF & rect);
  fn _ZN11QSGGeometry18updateRectGeometryEPS_RK6QRectF(arg0: *mut c_void, arg1: *mut c_void);
  // proto:  quint16 * QSGGeometry::indexDataAsUShort();
  fn _ZN11QSGGeometry17indexDataAsUShortEv(qthis: u64 /* *mut c_void*/) -> *mut c_ushort;
  // proto:  void * QSGGeometry::vertexData();
  fn _ZN11QSGGeometry10vertexDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSGGeometry::markVertexDataDirty();
  fn _ZN11QSGGeometry19markVertexDataDirtyEv(qthis: u64 /* *mut c_void*/);
  // proto:  float QSGGeometry::lineWidth();
  fn _ZNK11QSGGeometry9lineWidthEv(qthis: u64 /* *mut c_void*/) -> c_float;
} // <= ext block end

// body block begin =>
// class sizeof(QSGGeometry)=128
#[derive(Default)]
pub struct QSGGeometry {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSGGeometry {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSGGeometry {
    return QSGGeometry{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QSGGeometry::attributeCount();
impl /*struct*/ QSGGeometry {
  pub fn attributeCount<RetType, T: QSGGeometry_attributeCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.attributeCount(self);
    // return 1;
  }
}

pub trait QSGGeometry_attributeCount<RetType> {
  fn attributeCount(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  int QSGGeometry::attributeCount();
impl<'a> /*trait*/ QSGGeometry_attributeCount<i32> for () {
  fn attributeCount(self , rsthis: & QSGGeometry) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZNK11QSGGeometry14attributeCountEv()};
    let mut ret = unsafe {_ZNK11QSGGeometry14attributeCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static void QSGGeometry::updateTexturedRectGeometry(QSGGeometry * g, const QRectF & rect, const QRectF & sourceRect);
impl /*struct*/ QSGGeometry {
  pub fn updateTexturedRectGeometry_s<RetType, T: QSGGeometry_updateTexturedRectGeometry_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.updateTexturedRectGeometry_s();
    // return 1;
  }
}

pub trait QSGGeometry_updateTexturedRectGeometry_s<RetType> {
  fn updateTexturedRectGeometry_s(self ) -> RetType;
}

  // proto: static void QSGGeometry::updateTexturedRectGeometry(QSGGeometry * g, const QRectF & rect, const QRectF & sourceRect);
impl<'a> /*trait*/ QSGGeometry_updateTexturedRectGeometry_s<()> for (&'a QSGGeometry, &'a QRectF, &'a QRectF) {
  fn updateTexturedRectGeometry_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry26updateTexturedRectGeometryEPS_RK6QRectFS3_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN11QSGGeometry26updateTexturedRectGeometryEPS_RK6QRectFS3_(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QSGGeometry::~QSGGeometry();
impl /*struct*/ QSGGeometry {
  pub fn free<RetType, T: QSGGeometry_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSGGeometry_free<RetType> {
  fn free(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  void QSGGeometry::~QSGGeometry();
impl<'a> /*trait*/ QSGGeometry_free<()> for () {
  fn free(self , rsthis: & QSGGeometry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometryD2Ev()};
     unsafe {_ZN11QSGGeometryD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QSGGeometry::sizeOfVertex();
impl /*struct*/ QSGGeometry {
  pub fn sizeOfVertex<RetType, T: QSGGeometry_sizeOfVertex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeOfVertex(self);
    // return 1;
  }
}

pub trait QSGGeometry_sizeOfVertex<RetType> {
  fn sizeOfVertex(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  int QSGGeometry::sizeOfVertex();
impl<'a> /*trait*/ QSGGeometry_sizeOfVertex<i32> for () {
  fn sizeOfVertex(self , rsthis: & QSGGeometry) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZNK11QSGGeometry12sizeOfVertexEv()};
    let mut ret = unsafe {_ZNK11QSGGeometry12sizeOfVertexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSGGeometry::setLineWidth(float w);
impl /*struct*/ QSGGeometry {
  pub fn setLineWidth<RetType, T: QSGGeometry_setLineWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLineWidth(self);
    // return 1;
  }
}

pub trait QSGGeometry_setLineWidth<RetType> {
  fn setLineWidth(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  void QSGGeometry::setLineWidth(float w);
impl<'a> /*trait*/ QSGGeometry_setLineWidth<()> for (f32) {
  fn setLineWidth(self , rsthis: & QSGGeometry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry12setLineWidthEf()};
    let arg0 = self  as c_float;
     unsafe {_ZN11QSGGeometry12setLineWidthEf(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  uint * QSGGeometry::indexDataAsUInt();
impl /*struct*/ QSGGeometry {
  pub fn indexDataAsUInt<RetType, T: QSGGeometry_indexDataAsUInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexDataAsUInt(self);
    // return 1;
  }
}

pub trait QSGGeometry_indexDataAsUInt<RetType> {
  fn indexDataAsUInt(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  uint * QSGGeometry::indexDataAsUInt();
impl<'a> /*trait*/ QSGGeometry_indexDataAsUInt<*mut u32> for () {
  fn indexDataAsUInt(self , rsthis: & QSGGeometry) -> *mut u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry15indexDataAsUIntEv()};
    let mut ret = unsafe {_ZN11QSGGeometry15indexDataAsUIntEv(rsthis.qclsinst)};
    return ret as *mut u32;
    // return 1;
  }
}

  // proto:  int QSGGeometry::sizeOfIndex();
impl /*struct*/ QSGGeometry {
  pub fn sizeOfIndex<RetType, T: QSGGeometry_sizeOfIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeOfIndex(self);
    // return 1;
  }
}

pub trait QSGGeometry_sizeOfIndex<RetType> {
  fn sizeOfIndex(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  int QSGGeometry::sizeOfIndex();
impl<'a> /*trait*/ QSGGeometry_sizeOfIndex<i32> for () {
  fn sizeOfIndex(self , rsthis: & QSGGeometry) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZNK11QSGGeometry11sizeOfIndexEv()};
    let mut ret = unsafe {_ZNK11QSGGeometry11sizeOfIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSGGeometry::setDrawingMode(GLenum mode);
impl /*struct*/ QSGGeometry {
  pub fn setDrawingMode<RetType, T: QSGGeometry_setDrawingMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDrawingMode(self);
    // return 1;
  }
}

pub trait QSGGeometry_setDrawingMode<RetType> {
  fn setDrawingMode(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  void QSGGeometry::setDrawingMode(GLenum mode);
impl<'a> /*trait*/ QSGGeometry_setDrawingMode<()> for (u32) {
  fn setDrawingMode(self , rsthis: & QSGGeometry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry14setDrawingModeEj()};
    let arg0 = self  as c_uint;
     unsafe {_ZN11QSGGeometry14setDrawingModeEj(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSGGeometry::markIndexDataDirty();
impl /*struct*/ QSGGeometry {
  pub fn markIndexDataDirty<RetType, T: QSGGeometry_markIndexDataDirty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.markIndexDataDirty(self);
    // return 1;
  }
}

pub trait QSGGeometry_markIndexDataDirty<RetType> {
  fn markIndexDataDirty(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  void QSGGeometry::markIndexDataDirty();
impl<'a> /*trait*/ QSGGeometry_markIndexDataDirty<()> for () {
  fn markIndexDataDirty(self , rsthis: & QSGGeometry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry18markIndexDataDirtyEv()};
     unsafe {_ZN11QSGGeometry18markIndexDataDirtyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QSGGeometry::indexCount();
impl /*struct*/ QSGGeometry {
  pub fn indexCount<RetType, T: QSGGeometry_indexCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexCount(self);
    // return 1;
  }
}

pub trait QSGGeometry_indexCount<RetType> {
  fn indexCount(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  int QSGGeometry::indexCount();
impl<'a> /*trait*/ QSGGeometry_indexCount<i32> for () {
  fn indexCount(self , rsthis: & QSGGeometry) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZNK11QSGGeometry10indexCountEv()};
    let mut ret = unsafe {_ZNK11QSGGeometry10indexCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QSGGeometry::indexType();
impl /*struct*/ QSGGeometry {
  pub fn indexType<RetType, T: QSGGeometry_indexType<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexType(self);
    // return 1;
  }
}

pub trait QSGGeometry_indexType<RetType> {
  fn indexType(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  int QSGGeometry::indexType();
impl<'a> /*trait*/ QSGGeometry_indexType<i32> for () {
  fn indexType(self , rsthis: & QSGGeometry) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZNK11QSGGeometry9indexTypeEv()};
    let mut ret = unsafe {_ZNK11QSGGeometry9indexTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void * QSGGeometry::indexData();
impl /*struct*/ QSGGeometry {
  pub fn indexData<RetType, T: QSGGeometry_indexData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexData(self);
    // return 1;
  }
}

pub trait QSGGeometry_indexData<RetType> {
  fn indexData(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  void * QSGGeometry::indexData();
impl<'a> /*trait*/ QSGGeometry_indexData<*mut c_void> for () {
  fn indexData(self , rsthis: & QSGGeometry) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry9indexDataEv()};
    let mut ret = unsafe {_ZN11QSGGeometry9indexDataEv(rsthis.qclsinst)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  GLenum QSGGeometry::drawingMode();
impl /*struct*/ QSGGeometry {
  pub fn drawingMode<RetType, T: QSGGeometry_drawingMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawingMode(self);
    // return 1;
  }
}

pub trait QSGGeometry_drawingMode<RetType> {
  fn drawingMode(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  GLenum QSGGeometry::drawingMode();
impl<'a> /*trait*/ QSGGeometry_drawingMode<u32> for () {
  fn drawingMode(self , rsthis: & QSGGeometry) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZNK11QSGGeometry11drawingModeEv()};
    let mut ret = unsafe {_ZNK11QSGGeometry11drawingModeEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  int QSGGeometry::vertexCount();
impl /*struct*/ QSGGeometry {
  pub fn vertexCount<RetType, T: QSGGeometry_vertexCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.vertexCount(self);
    // return 1;
  }
}

pub trait QSGGeometry_vertexCount<RetType> {
  fn vertexCount(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  int QSGGeometry::vertexCount();
impl<'a> /*trait*/ QSGGeometry_vertexCount<i32> for () {
  fn vertexCount(self , rsthis: & QSGGeometry) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZNK11QSGGeometry11vertexCountEv()};
    let mut ret = unsafe {_ZNK11QSGGeometry11vertexCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QSGGeometry::allocate(int vertexCount, int indexCount);
impl /*struct*/ QSGGeometry {
  pub fn allocate<RetType, T: QSGGeometry_allocate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allocate(self);
    // return 1;
  }
}

pub trait QSGGeometry_allocate<RetType> {
  fn allocate(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  void QSGGeometry::allocate(int vertexCount, int indexCount);
impl<'a> /*trait*/ QSGGeometry_allocate<()> for (i32, i32) {
  fn allocate(self , rsthis: & QSGGeometry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry8allocateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN11QSGGeometry8allocateEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QSGGeometry::updateRectGeometry(QSGGeometry * g, const QRectF & rect);
impl /*struct*/ QSGGeometry {
  pub fn updateRectGeometry_s<RetType, T: QSGGeometry_updateRectGeometry_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.updateRectGeometry_s();
    // return 1;
  }
}

pub trait QSGGeometry_updateRectGeometry_s<RetType> {
  fn updateRectGeometry_s(self ) -> RetType;
}

  // proto: static void QSGGeometry::updateRectGeometry(QSGGeometry * g, const QRectF & rect);
impl<'a> /*trait*/ QSGGeometry_updateRectGeometry_s<()> for (&'a QSGGeometry, &'a QRectF) {
  fn updateRectGeometry_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry18updateRectGeometryEPS_RK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QSGGeometry18updateRectGeometryEPS_RK6QRectF(arg0, arg1)};
    // return 1;
  }
}

  // proto:  quint16 * QSGGeometry::indexDataAsUShort();
impl /*struct*/ QSGGeometry {
  pub fn indexDataAsUShort<RetType, T: QSGGeometry_indexDataAsUShort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexDataAsUShort(self);
    // return 1;
  }
}

pub trait QSGGeometry_indexDataAsUShort<RetType> {
  fn indexDataAsUShort(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  quint16 * QSGGeometry::indexDataAsUShort();
impl<'a> /*trait*/ QSGGeometry_indexDataAsUShort<*mut u16> for () {
  fn indexDataAsUShort(self , rsthis: & QSGGeometry) -> *mut u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry17indexDataAsUShortEv()};
    let mut ret = unsafe {_ZN11QSGGeometry17indexDataAsUShortEv(rsthis.qclsinst)};
    return ret as *mut u16;
    // return 1;
  }
}

  // proto:  void * QSGGeometry::vertexData();
impl /*struct*/ QSGGeometry {
  pub fn vertexData<RetType, T: QSGGeometry_vertexData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.vertexData(self);
    // return 1;
  }
}

pub trait QSGGeometry_vertexData<RetType> {
  fn vertexData(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  void * QSGGeometry::vertexData();
impl<'a> /*trait*/ QSGGeometry_vertexData<*mut c_void> for () {
  fn vertexData(self , rsthis: & QSGGeometry) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry10vertexDataEv()};
    let mut ret = unsafe {_ZN11QSGGeometry10vertexDataEv(rsthis.qclsinst)};
    return ret as *mut c_void;
    // return 1;
  }
}

  // proto:  void QSGGeometry::markVertexDataDirty();
impl /*struct*/ QSGGeometry {
  pub fn markVertexDataDirty<RetType, T: QSGGeometry_markVertexDataDirty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.markVertexDataDirty(self);
    // return 1;
  }
}

pub trait QSGGeometry_markVertexDataDirty<RetType> {
  fn markVertexDataDirty(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  void QSGGeometry::markVertexDataDirty();
impl<'a> /*trait*/ QSGGeometry_markVertexDataDirty<()> for () {
  fn markVertexDataDirty(self , rsthis: & QSGGeometry) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZN11QSGGeometry19markVertexDataDirtyEv()};
     unsafe {_ZN11QSGGeometry19markVertexDataDirtyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  float QSGGeometry::lineWidth();
impl /*struct*/ QSGGeometry {
  pub fn lineWidth<RetType, T: QSGGeometry_lineWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineWidth(self);
    // return 1;
  }
}

pub trait QSGGeometry_lineWidth<RetType> {
  fn lineWidth(self , rsthis: & QSGGeometry) -> RetType;
}

  // proto:  float QSGGeometry::lineWidth();
impl<'a> /*trait*/ QSGGeometry_lineWidth<f32> for () {
  fn lineWidth(self , rsthis: & QSGGeometry) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 128)};
    // unsafe{_ZNK11QSGGeometry9lineWidthEv()};
    let mut ret = unsafe {_ZNK11QSGGeometry9lineWidthEv(rsthis.qclsinst)};
    return ret as f32;
    // return 1;
  }
}

// <= body block end

