// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/qquickpainteditem.h
// dst-file: /src/quick/qquickpainteditem.rs
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
use super::qquickitem::QQuickItem; // 773
use std::ops::Deref;
use super::qsgtextureprovider::QSGTextureProvider; // 773
use super::super::core::qsize::QSize; // 771
use super::super::gui::qcolor::QColor; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::core::qrect::QRect; // 771
use super::super::gui::qpainter::QPainter; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QQuickPaintedItem_Class_Size() -> c_int;
  // proto:  bool QQuickPaintedItem::antialiasing();
  fn _ZNK17QQuickPaintedItem12antialiasingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSGTextureProvider * QQuickPaintedItem::textureProvider();
  fn _ZNK17QQuickPaintedItem15textureProviderEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QQuickPaintedItem::metaObject();
  fn _ZNK17QQuickPaintedItem10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QQuickPaintedItem::QQuickPaintedItem(const QQuickPaintedItem & );
  fn _ZN17QQuickPaintedItemC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickPaintedItem::setMipmap(bool enable);
  fn _ZN17QQuickPaintedItem9setMipmapEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QQuickPaintedItem::QQuickPaintedItem(QQuickItem * parent);
  fn _ZN17QQuickPaintedItemC2EP10QQuickItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSize QQuickPaintedItem::contentsSize();
  fn _ZNK17QQuickPaintedItem12contentsSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickPaintedItem::setAntialiasing(bool enable);
  fn _ZN17QQuickPaintedItem15setAntialiasingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  qreal QQuickPaintedItem::contentsScale();
  fn _ZNK17QQuickPaintedItem13contentsScaleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QQuickPaintedItem::isTextureProvider();
  fn _ZNK17QQuickPaintedItem17isTextureProviderEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQuickPaintedItem::~QQuickPaintedItem();
  fn _ZN17QQuickPaintedItemD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QColor QQuickPaintedItem::fillColor();
  fn _ZNK17QQuickPaintedItem9fillColorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickPaintedItem::setOpaquePainting(bool opaque);
  fn _ZN17QQuickPaintedItem17setOpaquePaintingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QQuickPaintedItem::mipmap();
  fn _ZNK17QQuickPaintedItem6mipmapEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQuickPaintedItem::setFillColor(const QColor & );
  fn _ZN17QQuickPaintedItem12setFillColorERK6QColor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickPaintedItem::resetContentsSize();
  fn _ZN17QQuickPaintedItem17resetContentsSizeEv(qthis: u64 /* *mut c_void*/);
  // proto:  QRectF QQuickPaintedItem::contentsBoundingRect();
  fn _ZNK17QQuickPaintedItem20contentsBoundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QQuickPaintedItem::update(const QRect & rect);
  fn _ZN17QQuickPaintedItem6updateERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QQuickPaintedItem::opaquePainting();
  fn _ZNK17QQuickPaintedItem14opaquePaintingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QQuickPaintedItem::setContentsSize(const QSize & );
  fn _ZN17QQuickPaintedItem15setContentsSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickPaintedItem::paint(QPainter * painter);
  fn _ZN17QQuickPaintedItem5paintEP8QPainter(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QQuickPaintedItem::setContentsScale(qreal );
  fn _ZN17QQuickPaintedItem16setContentsScaleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  fn QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem19renderTargetChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem20contentsScaleChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem19contentsSizeChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem16fillColorChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QQuickPaintedItem)=1
#[derive(Default)]
pub struct QQuickPaintedItem {
  qbase: QQuickItem,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _contentsScaleChanged: QQuickPaintedItem_contentsScaleChanged_signal,
  pub _contentsSizeChanged: QQuickPaintedItem_contentsSizeChanged_signal,
  pub _fillColorChanged: QQuickPaintedItem_fillColorChanged_signal,
  pub _renderTargetChanged: QQuickPaintedItem_renderTargetChanged_signal,
}

impl /*struct*/ QQuickPaintedItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QQuickPaintedItem {
    return QQuickPaintedItem{qbase: QQuickItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QQuickPaintedItem {
  type Target = QQuickItem;

  fn deref(&self) -> &QQuickItem {
    return & self.qbase;
  }
}
impl AsRef<QQuickItem> for QQuickPaintedItem {
  fn as_ref(& self) -> & QQuickItem {
    return & self.qbase;
  }
}
  // proto:  bool QQuickPaintedItem::antialiasing();
impl /*struct*/ QQuickPaintedItem {
  pub fn antialiasing<RetType, T: QQuickPaintedItem_antialiasing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.antialiasing(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_antialiasing<RetType> {
  fn antialiasing(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  bool QQuickPaintedItem::antialiasing();
impl<'a> /*trait*/ QQuickPaintedItem_antialiasing<i8> for () {
  fn antialiasing(self , rsthis: & QQuickPaintedItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQuickPaintedItem12antialiasingEv()};
    let mut ret = unsafe {_ZNK17QQuickPaintedItem12antialiasingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSGTextureProvider * QQuickPaintedItem::textureProvider();
impl /*struct*/ QQuickPaintedItem {
  pub fn textureProvider<RetType, T: QQuickPaintedItem_textureProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textureProvider(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_textureProvider<RetType> {
  fn textureProvider(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  QSGTextureProvider * QQuickPaintedItem::textureProvider();
impl<'a> /*trait*/ QQuickPaintedItem_textureProvider<QSGTextureProvider> for () {
  fn textureProvider(self , rsthis: & QQuickPaintedItem) -> QSGTextureProvider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQuickPaintedItem15textureProviderEv()};
    let mut ret = unsafe {_ZNK17QQuickPaintedItem15textureProviderEv(rsthis.qclsinst)};
    let mut ret1 = QSGTextureProvider::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QQuickPaintedItem::metaObject();
impl /*struct*/ QQuickPaintedItem {
  pub fn metaObject<RetType, T: QQuickPaintedItem_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_metaObject<RetType> {
  fn metaObject(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  const QMetaObject * QQuickPaintedItem::metaObject();
impl<'a> /*trait*/ QQuickPaintedItem_metaObject<()> for () {
  fn metaObject(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQuickPaintedItem10metaObjectEv()};
     unsafe {_ZNK17QQuickPaintedItem10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::QQuickPaintedItem(const QQuickPaintedItem & );
impl /*struct*/ QQuickPaintedItem {
  pub fn new<T: QQuickPaintedItem_new>(value: T) -> QQuickPaintedItem {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QQuickPaintedItem_new {
  fn new(self) -> QQuickPaintedItem;
}

  // proto:  void QQuickPaintedItem::QQuickPaintedItem(const QQuickPaintedItem & );
impl<'a> /*trait*/ QQuickPaintedItem_new for (&'a QQuickPaintedItem) {
  fn new(self) -> QQuickPaintedItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItemC2ERKS_()};
    let ctysz: c_int = unsafe{QQuickPaintedItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QQuickPaintedItemC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickPaintedItem{qbase: QQuickItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::setMipmap(bool enable);
impl /*struct*/ QQuickPaintedItem {
  pub fn setMipmap<RetType, T: QQuickPaintedItem_setMipmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMipmap(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_setMipmap<RetType> {
  fn setMipmap(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  void QQuickPaintedItem::setMipmap(bool enable);
impl<'a> /*trait*/ QQuickPaintedItem_setMipmap<()> for (i8) {
  fn setMipmap(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItem9setMipmapEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN17QQuickPaintedItem9setMipmapEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::QQuickPaintedItem(QQuickItem * parent);
impl<'a> /*trait*/ QQuickPaintedItem_new for (&'a QQuickItem) {
  fn new(self) -> QQuickPaintedItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItemC2EP10QQuickItem()};
    let ctysz: c_int = unsafe{QQuickPaintedItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QQuickPaintedItemC2EP10QQuickItem(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QQuickPaintedItem{qbase: QQuickItem::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QQuickPaintedItem::contentsSize();
impl /*struct*/ QQuickPaintedItem {
  pub fn contentsSize<RetType, T: QQuickPaintedItem_contentsSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentsSize(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_contentsSize<RetType> {
  fn contentsSize(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  QSize QQuickPaintedItem::contentsSize();
impl<'a> /*trait*/ QQuickPaintedItem_contentsSize<QSize> for () {
  fn contentsSize(self , rsthis: & QQuickPaintedItem) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQuickPaintedItem12contentsSizeEv()};
    let mut ret = unsafe {_ZNK17QQuickPaintedItem12contentsSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::setAntialiasing(bool enable);
impl /*struct*/ QQuickPaintedItem {
  pub fn setAntialiasing<RetType, T: QQuickPaintedItem_setAntialiasing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAntialiasing(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_setAntialiasing<RetType> {
  fn setAntialiasing(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  void QQuickPaintedItem::setAntialiasing(bool enable);
impl<'a> /*trait*/ QQuickPaintedItem_setAntialiasing<()> for (i8) {
  fn setAntialiasing(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItem15setAntialiasingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN17QQuickPaintedItem15setAntialiasingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QQuickPaintedItem::contentsScale();
impl /*struct*/ QQuickPaintedItem {
  pub fn contentsScale<RetType, T: QQuickPaintedItem_contentsScale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentsScale(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_contentsScale<RetType> {
  fn contentsScale(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  qreal QQuickPaintedItem::contentsScale();
impl<'a> /*trait*/ QQuickPaintedItem_contentsScale<f64> for () {
  fn contentsScale(self , rsthis: & QQuickPaintedItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQuickPaintedItem13contentsScaleEv()};
    let mut ret = unsafe {_ZNK17QQuickPaintedItem13contentsScaleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QQuickPaintedItem::isTextureProvider();
impl /*struct*/ QQuickPaintedItem {
  pub fn isTextureProvider<RetType, T: QQuickPaintedItem_isTextureProvider<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTextureProvider(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_isTextureProvider<RetType> {
  fn isTextureProvider(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  bool QQuickPaintedItem::isTextureProvider();
impl<'a> /*trait*/ QQuickPaintedItem_isTextureProvider<i8> for () {
  fn isTextureProvider(self , rsthis: & QQuickPaintedItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQuickPaintedItem17isTextureProviderEv()};
    let mut ret = unsafe {_ZNK17QQuickPaintedItem17isTextureProviderEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::~QQuickPaintedItem();
impl /*struct*/ QQuickPaintedItem {
  pub fn free<RetType, T: QQuickPaintedItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_free<RetType> {
  fn free(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  void QQuickPaintedItem::~QQuickPaintedItem();
impl<'a> /*trait*/ QQuickPaintedItem_free<()> for () {
  fn free(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItemD2Ev()};
     unsafe {_ZN17QQuickPaintedItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QColor QQuickPaintedItem::fillColor();
impl /*struct*/ QQuickPaintedItem {
  pub fn fillColor<RetType, T: QQuickPaintedItem_fillColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fillColor(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_fillColor<RetType> {
  fn fillColor(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  QColor QQuickPaintedItem::fillColor();
impl<'a> /*trait*/ QQuickPaintedItem_fillColor<QColor> for () {
  fn fillColor(self , rsthis: & QQuickPaintedItem) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQuickPaintedItem9fillColorEv()};
    let mut ret = unsafe {_ZNK17QQuickPaintedItem9fillColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::setOpaquePainting(bool opaque);
impl /*struct*/ QQuickPaintedItem {
  pub fn setOpaquePainting<RetType, T: QQuickPaintedItem_setOpaquePainting<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOpaquePainting(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_setOpaquePainting<RetType> {
  fn setOpaquePainting(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  void QQuickPaintedItem::setOpaquePainting(bool opaque);
impl<'a> /*trait*/ QQuickPaintedItem_setOpaquePainting<()> for (i8) {
  fn setOpaquePainting(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItem17setOpaquePaintingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN17QQuickPaintedItem17setOpaquePaintingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QQuickPaintedItem::mipmap();
impl /*struct*/ QQuickPaintedItem {
  pub fn mipmap<RetType, T: QQuickPaintedItem_mipmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mipmap(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_mipmap<RetType> {
  fn mipmap(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  bool QQuickPaintedItem::mipmap();
impl<'a> /*trait*/ QQuickPaintedItem_mipmap<i8> for () {
  fn mipmap(self , rsthis: & QQuickPaintedItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQuickPaintedItem6mipmapEv()};
    let mut ret = unsafe {_ZNK17QQuickPaintedItem6mipmapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::setFillColor(const QColor & );
impl /*struct*/ QQuickPaintedItem {
  pub fn setFillColor<RetType, T: QQuickPaintedItem_setFillColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFillColor(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_setFillColor<RetType> {
  fn setFillColor(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  void QQuickPaintedItem::setFillColor(const QColor & );
impl<'a> /*trait*/ QQuickPaintedItem_setFillColor<()> for (&'a QColor) {
  fn setFillColor(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItem12setFillColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QQuickPaintedItem12setFillColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::resetContentsSize();
impl /*struct*/ QQuickPaintedItem {
  pub fn resetContentsSize<RetType, T: QQuickPaintedItem_resetContentsSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetContentsSize(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_resetContentsSize<RetType> {
  fn resetContentsSize(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  void QQuickPaintedItem::resetContentsSize();
impl<'a> /*trait*/ QQuickPaintedItem_resetContentsSize<()> for () {
  fn resetContentsSize(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItem17resetContentsSizeEv()};
     unsafe {_ZN17QQuickPaintedItem17resetContentsSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QQuickPaintedItem::contentsBoundingRect();
impl /*struct*/ QQuickPaintedItem {
  pub fn contentsBoundingRect<RetType, T: QQuickPaintedItem_contentsBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentsBoundingRect(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_contentsBoundingRect<RetType> {
  fn contentsBoundingRect(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  QRectF QQuickPaintedItem::contentsBoundingRect();
impl<'a> /*trait*/ QQuickPaintedItem_contentsBoundingRect<QRectF> for () {
  fn contentsBoundingRect(self , rsthis: & QQuickPaintedItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQuickPaintedItem20contentsBoundingRectEv()};
    let mut ret = unsafe {_ZNK17QQuickPaintedItem20contentsBoundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::update(const QRect & rect);
impl /*struct*/ QQuickPaintedItem {
  pub fn update<RetType, T: QQuickPaintedItem_update<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_update<RetType> {
  fn update(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  void QQuickPaintedItem::update(const QRect & rect);
impl<'a> /*trait*/ QQuickPaintedItem_update<()> for (&'a QRect) {
  fn update(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItem6updateERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QQuickPaintedItem6updateERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QQuickPaintedItem::opaquePainting();
impl /*struct*/ QQuickPaintedItem {
  pub fn opaquePainting<RetType, T: QQuickPaintedItem_opaquePainting<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.opaquePainting(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_opaquePainting<RetType> {
  fn opaquePainting(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  bool QQuickPaintedItem::opaquePainting();
impl<'a> /*trait*/ QQuickPaintedItem_opaquePainting<i8> for () {
  fn opaquePainting(self , rsthis: & QQuickPaintedItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QQuickPaintedItem14opaquePaintingEv()};
    let mut ret = unsafe {_ZNK17QQuickPaintedItem14opaquePaintingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::setContentsSize(const QSize & );
impl /*struct*/ QQuickPaintedItem {
  pub fn setContentsSize<RetType, T: QQuickPaintedItem_setContentsSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContentsSize(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_setContentsSize<RetType> {
  fn setContentsSize(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  void QQuickPaintedItem::setContentsSize(const QSize & );
impl<'a> /*trait*/ QQuickPaintedItem_setContentsSize<()> for (&'a QSize) {
  fn setContentsSize(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItem15setContentsSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QQuickPaintedItem15setContentsSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::paint(QPainter * painter);
impl /*struct*/ QQuickPaintedItem {
  pub fn paint<RetType, T: QQuickPaintedItem_paint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_paint<RetType> {
  fn paint(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  void QQuickPaintedItem::paint(QPainter * painter);
impl<'a> /*trait*/ QQuickPaintedItem_paint<()> for (&'a QPainter) {
  fn paint(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItem5paintEP8QPainter()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QQuickPaintedItem5paintEP8QPainter(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QQuickPaintedItem::setContentsScale(qreal );
impl /*struct*/ QQuickPaintedItem {
  pub fn setContentsScale<RetType, T: QQuickPaintedItem_setContentsScale<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContentsScale(self);
    // return 1;
  }
}

pub trait QQuickPaintedItem_setContentsScale<RetType> {
  fn setContentsScale(self , rsthis: & QQuickPaintedItem) -> RetType;
}

  // proto:  void QQuickPaintedItem::setContentsScale(qreal );
impl<'a> /*trait*/ QQuickPaintedItem_setContentsScale<()> for (f64) {
  fn setContentsScale(self , rsthis: & QQuickPaintedItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QQuickPaintedItem16setContentsScaleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QQuickPaintedItem16setContentsScaleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QQuickPaintedItem_contentsScaleChanged
pub struct QQuickPaintedItem_contentsScaleChanged_signal{poi:u64}
impl /* struct */ QQuickPaintedItem {
  pub fn contentsScaleChanged(&self) -> QQuickPaintedItem_contentsScaleChanged_signal {
     return QQuickPaintedItem_contentsScaleChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickPaintedItem_contentsScaleChanged_signal {
  pub fn connect<T: QQuickPaintedItem_contentsScaleChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickPaintedItem_contentsScaleChanged_signal_connect {
  fn connect(self, sigthis: QQuickPaintedItem_contentsScaleChanged_signal);
}

#[derive(Default)] // for QQuickPaintedItem_contentsSizeChanged
pub struct QQuickPaintedItem_contentsSizeChanged_signal{poi:u64}
impl /* struct */ QQuickPaintedItem {
  pub fn contentsSizeChanged(&self) -> QQuickPaintedItem_contentsSizeChanged_signal {
     return QQuickPaintedItem_contentsSizeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickPaintedItem_contentsSizeChanged_signal {
  pub fn connect<T: QQuickPaintedItem_contentsSizeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickPaintedItem_contentsSizeChanged_signal_connect {
  fn connect(self, sigthis: QQuickPaintedItem_contentsSizeChanged_signal);
}

#[derive(Default)] // for QQuickPaintedItem_fillColorChanged
pub struct QQuickPaintedItem_fillColorChanged_signal{poi:u64}
impl /* struct */ QQuickPaintedItem {
  pub fn fillColorChanged(&self) -> QQuickPaintedItem_fillColorChanged_signal {
     return QQuickPaintedItem_fillColorChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickPaintedItem_fillColorChanged_signal {
  pub fn connect<T: QQuickPaintedItem_fillColorChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickPaintedItem_fillColorChanged_signal_connect {
  fn connect(self, sigthis: QQuickPaintedItem_fillColorChanged_signal);
}

#[derive(Default)] // for QQuickPaintedItem_renderTargetChanged
pub struct QQuickPaintedItem_renderTargetChanged_signal{poi:u64}
impl /* struct */ QQuickPaintedItem {
  pub fn renderTargetChanged(&self) -> QQuickPaintedItem_renderTargetChanged_signal {
     return QQuickPaintedItem_renderTargetChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QQuickPaintedItem_renderTargetChanged_signal {
  pub fn connect<T: QQuickPaintedItem_renderTargetChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QQuickPaintedItem_renderTargetChanged_signal_connect {
  fn connect(self, sigthis: QQuickPaintedItem_renderTargetChanged_signal);
}

// renderTargetChanged()
extern fn QQuickPaintedItem_renderTargetChanged_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickPaintedItem_renderTargetChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickPaintedItem_renderTargetChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickPaintedItem_renderTargetChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickPaintedItem_renderTargetChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem19renderTargetChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickPaintedItem_renderTargetChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickPaintedItem_renderTargetChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickPaintedItem_renderTargetChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem19renderTargetChangedEv(arg0, arg1, arg2)};
  }
}
// contentsScaleChanged()
extern fn QQuickPaintedItem_contentsScaleChanged_signal_connect_cb_1(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickPaintedItem_contentsScaleChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickPaintedItem_contentsScaleChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickPaintedItem_contentsScaleChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickPaintedItem_contentsScaleChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem20contentsScaleChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickPaintedItem_contentsScaleChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickPaintedItem_contentsScaleChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickPaintedItem_contentsScaleChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem20contentsScaleChangedEv(arg0, arg1, arg2)};
  }
}
// contentsSizeChanged()
extern fn QQuickPaintedItem_contentsSizeChanged_signal_connect_cb_2(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickPaintedItem_contentsSizeChanged_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickPaintedItem_contentsSizeChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickPaintedItem_contentsSizeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickPaintedItem_contentsSizeChanged_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem19contentsSizeChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickPaintedItem_contentsSizeChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickPaintedItem_contentsSizeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickPaintedItem_contentsSizeChanged_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem19contentsSizeChangedEv(arg0, arg1, arg2)};
  }
}
// fillColorChanged()
extern fn QQuickPaintedItem_fillColorChanged_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QQuickPaintedItem_fillColorChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QQuickPaintedItem_fillColorChanged_signal_connect for fn() {
  fn connect(self, sigthis: QQuickPaintedItem_fillColorChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickPaintedItem_fillColorChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem16fillColorChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QQuickPaintedItem_fillColorChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QQuickPaintedItem_fillColorChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QQuickPaintedItem_fillColorChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QQuickPaintedItem_SlotProxy_connect__ZN17QQuickPaintedItem16fillColorChangedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

