// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qabstracttextdocumentlayout.h
// dst-file: /src/gui/qabstracttextdocumentlayout.rs
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
use super::qtextdocument::*; // 773
use super::qtextformat::*; // 773
use super::super::core::qsize::*; // 771
use super::qpainter::*; // 773
use super::super::core::qrect::*; // 771
use super::super::core::qobject::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qpoint::*; // 771
use super::qpaintdevice::*; // 773
use super::super::core::qstring::*; // 771
// use super::qabstracttextdocumentlayout::QTextObjectInterface; // 773
use super::qtextobject::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextObjectInterface_Class_Size() -> c_int;
  // proto:  void QTextObjectInterface::~QTextObjectInterface();
  fn C_ZN20QTextObjectInterfaceD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
  fn C_ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
  fn C_ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: *mut c_void);
  fn QAbstractTextDocumentLayout_Class_Size() -> c_int;
  // proto:  const QMetaObject * QAbstractTextDocumentLayout::metaObject();
  fn C_ZNK27QAbstractTextDocumentLayout10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractTextDocumentLayout::registerHandler(int objectType, QObject * component);
  fn C_ZN27QAbstractTextDocumentLayout15registerHandlerEiP7QObject(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  int QAbstractTextDocumentLayout::pageCount();
  fn C_ZNK27QAbstractTextDocumentLayout9pageCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QAbstractTextDocumentLayout::~QAbstractTextDocumentLayout();
  fn C_ZN27QAbstractTextDocumentLayoutD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractTextDocumentLayout::setPaintDevice(QPaintDevice * device);
  fn C_ZN27QAbstractTextDocumentLayout14setPaintDeviceEP12QPaintDevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTextDocument * QAbstractTextDocumentLayout::document();
  fn C_ZNK27QAbstractTextDocumentLayout8documentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractTextDocumentLayout::unregisterHandler(int objectType, QObject * component);
  fn C_ZN27QAbstractTextDocumentLayout17unregisterHandlerEiP7QObject(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QAbstractTextDocumentLayout::QAbstractTextDocumentLayout(QTextDocument * doc);
  fn C_ZN27QAbstractTextDocumentLayoutC2EP13QTextDocument(arg0: *mut c_void) -> u64;
  // proto:  QSizeF QAbstractTextDocumentLayout::documentSize();
  fn C_ZNK27QAbstractTextDocumentLayout12documentSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPaintDevice * QAbstractTextDocumentLayout::paintDevice();
  fn C_ZNK27QAbstractTextDocumentLayout11paintDeviceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QAbstractTextDocumentLayout::anchorAt(const QPointF & pos);
  fn C_ZNK27QAbstractTextDocumentLayout8anchorAtERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTextObjectInterface * QAbstractTextDocumentLayout::handlerForObject(int objectType);
  fn C_ZNK27QAbstractTextDocumentLayout16handlerForObjectEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QRectF QAbstractTextDocumentLayout::frameBoundingRect(QTextFrame * frame);
  fn C_ZNK27QAbstractTextDocumentLayout17frameBoundingRectEP10QTextFrame(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRectF QAbstractTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
  fn C_ZNK27QAbstractTextDocumentLayout17blockBoundingRectERK10QTextBlock(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  fn QAbstractTextDocumentLayout_SlotProxy_connect__ZN27QAbstractTextDocumentLayout16pageCountChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractTextDocumentLayout_SlotProxy_connect__ZN27QAbstractTextDocumentLayout19documentSizeChangedERK6QSizeF(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractTextDocumentLayout_SlotProxy_connect__ZN27QAbstractTextDocumentLayout11updateBlockERK10QTextBlock(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTextObjectInterface)=8
#[derive(Default)]
pub struct QTextObjectInterface {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAbstractTextDocumentLayout)=1
#[derive(Default)]
pub struct QAbstractTextDocumentLayout {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _updateBlock: QAbstractTextDocumentLayout_updateBlock_signal,
  pub _pageCountChanged: QAbstractTextDocumentLayout_pageCountChanged_signal,
  pub _update: QAbstractTextDocumentLayout_update_signal,
  pub _documentSizeChanged: QAbstractTextDocumentLayout_documentSizeChanged_signal,
}

impl /*struct*/ QTextObjectInterface {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextObjectInterface {
    return QTextObjectInterface{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTextObjectInterface::~QTextObjectInterface();
impl /*struct*/ QTextObjectInterface {
  pub fn free<RetType, T: QTextObjectInterface_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_free<RetType> {
  fn free(self , rsthis: & QTextObjectInterface) -> RetType;
}

  // proto:  void QTextObjectInterface::~QTextObjectInterface();
impl<'a> /*trait*/ QTextObjectInterface_free<()> for () {
  fn free(self , rsthis: & QTextObjectInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterfaceD2Ev()};
     unsafe {C_ZN20QTextObjectInterfaceD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl /*struct*/ QTextObjectInterface {
  pub fn intrinsicSize<RetType, T: QTextObjectInterface_intrinsicSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intrinsicSize(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_intrinsicSize<RetType> {
  fn intrinsicSize(self , rsthis: & QTextObjectInterface) -> RetType;
}

  // proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl<'a> /*trait*/ QTextObjectInterface_intrinsicSize<QSizeF> for (&'a QTextDocument, i32, &'a QTextFormat) {
  fn intrinsicSize(self , rsthis: & QTextObjectInterface) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl /*struct*/ QTextObjectInterface {
  pub fn drawObject<RetType, T: QTextObjectInterface_drawObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawObject(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_drawObject<RetType> {
  fn drawObject(self , rsthis: & QTextObjectInterface) -> RetType;
}

  // proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl<'a> /*trait*/ QTextObjectInterface_drawObject<()> for (&'a QPainter, &'a QRectF, &'a QTextDocument, i32, &'a QTextFormat) {
  fn drawObject(self , rsthis: & QTextObjectInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
     unsafe {C_ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractTextDocumentLayout {
    return QAbstractTextDocumentLayout{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractTextDocumentLayout {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAbstractTextDocumentLayout {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QAbstractTextDocumentLayout::metaObject();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn metaObject<RetType, T: QAbstractTextDocumentLayout_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  const QMetaObject * QAbstractTextDocumentLayout::metaObject();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QAbstractTextDocumentLayout) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout10metaObjectEv()};
    let mut ret = unsafe {C_ZNK27QAbstractTextDocumentLayout10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::registerHandler(int objectType, QObject * component);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn registerHandler<RetType, T: QAbstractTextDocumentLayout_registerHandler<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.registerHandler(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_registerHandler<RetType> {
  fn registerHandler(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::registerHandler(int objectType, QObject * component);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_registerHandler<()> for (i32, &'a QObject) {
  fn registerHandler(self , rsthis: & QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayout15registerHandlerEiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN27QAbstractTextDocumentLayout15registerHandlerEiP7QObject(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QAbstractTextDocumentLayout::pageCount();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn pageCount<RetType, T: QAbstractTextDocumentLayout_pageCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pageCount(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_pageCount<RetType> {
  fn pageCount(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  int QAbstractTextDocumentLayout::pageCount();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_pageCount<i32> for () {
  fn pageCount(self , rsthis: & QAbstractTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout9pageCountEv()};
    let mut ret = unsafe {C_ZNK27QAbstractTextDocumentLayout9pageCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::~QAbstractTextDocumentLayout();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn free<RetType, T: QAbstractTextDocumentLayout_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_free<RetType> {
  fn free(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::~QAbstractTextDocumentLayout();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_free<()> for () {
  fn free(self , rsthis: & QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayoutD2Ev()};
     unsafe {C_ZN27QAbstractTextDocumentLayoutD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::setPaintDevice(QPaintDevice * device);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn setPaintDevice<RetType, T: QAbstractTextDocumentLayout_setPaintDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPaintDevice(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_setPaintDevice<RetType> {
  fn setPaintDevice(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::setPaintDevice(QPaintDevice * device);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_setPaintDevice<()> for (&'a QPaintDevice) {
  fn setPaintDevice(self , rsthis: & QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayout14setPaintDeviceEP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN27QAbstractTextDocumentLayout14setPaintDeviceEP12QPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextDocument * QAbstractTextDocumentLayout::document();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn document<RetType, T: QAbstractTextDocumentLayout_document<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_document<RetType> {
  fn document(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QTextDocument * QAbstractTextDocumentLayout::document();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_document<QTextDocument> for () {
  fn document(self , rsthis: & QAbstractTextDocumentLayout) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout8documentEv()};
    let mut ret = unsafe {C_ZNK27QAbstractTextDocumentLayout8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::unregisterHandler(int objectType, QObject * component);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn unregisterHandler<RetType, T: QAbstractTextDocumentLayout_unregisterHandler<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unregisterHandler(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_unregisterHandler<RetType> {
  fn unregisterHandler(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::unregisterHandler(int objectType, QObject * component);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_unregisterHandler<()> for (i32, Option<&'a QObject>) {
  fn unregisterHandler(self , rsthis: & QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayout17unregisterHandlerEiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
     unsafe {C_ZN27QAbstractTextDocumentLayout17unregisterHandlerEiP7QObject(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::QAbstractTextDocumentLayout(QTextDocument * doc);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn new<T: QAbstractTextDocumentLayout_new>(value: T) -> QAbstractTextDocumentLayout {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_new {
  fn new(self) -> QAbstractTextDocumentLayout;
}

  // proto:  void QAbstractTextDocumentLayout::QAbstractTextDocumentLayout(QTextDocument * doc);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_new for (&'a QTextDocument) {
  fn new(self) -> QAbstractTextDocumentLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayoutC2EP13QTextDocument()};
    let ctysz: c_int = unsafe{QAbstractTextDocumentLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN27QAbstractTextDocumentLayoutC2EP13QTextDocument(arg0)};
    let rsthis = QAbstractTextDocumentLayout{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSizeF QAbstractTextDocumentLayout::documentSize();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn documentSize<RetType, T: QAbstractTextDocumentLayout_documentSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentSize(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_documentSize<RetType> {
  fn documentSize(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QSizeF QAbstractTextDocumentLayout::documentSize();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_documentSize<QSizeF> for () {
  fn documentSize(self , rsthis: & QAbstractTextDocumentLayout) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout12documentSizeEv()};
    let mut ret = unsafe {C_ZNK27QAbstractTextDocumentLayout12documentSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPaintDevice * QAbstractTextDocumentLayout::paintDevice();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn paintDevice<RetType, T: QAbstractTextDocumentLayout_paintDevice<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.paintDevice(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_paintDevice<RetType> {
  fn paintDevice(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QPaintDevice * QAbstractTextDocumentLayout::paintDevice();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_paintDevice<QPaintDevice> for () {
  fn paintDevice(self , rsthis: & QAbstractTextDocumentLayout) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout11paintDeviceEv()};
    let mut ret = unsafe {C_ZNK27QAbstractTextDocumentLayout11paintDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QPaintDevice::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QAbstractTextDocumentLayout::anchorAt(const QPointF & pos);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn anchorAt<RetType, T: QAbstractTextDocumentLayout_anchorAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.anchorAt(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_anchorAt<RetType> {
  fn anchorAt(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QString QAbstractTextDocumentLayout::anchorAt(const QPointF & pos);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_anchorAt<QString> for (&'a QPointF) {
  fn anchorAt(self , rsthis: & QAbstractTextDocumentLayout) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout8anchorAtERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK27QAbstractTextDocumentLayout8anchorAtERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextObjectInterface * QAbstractTextDocumentLayout::handlerForObject(int objectType);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn handlerForObject<RetType, T: QAbstractTextDocumentLayout_handlerForObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.handlerForObject(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_handlerForObject<RetType> {
  fn handlerForObject(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QTextObjectInterface * QAbstractTextDocumentLayout::handlerForObject(int objectType);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_handlerForObject<QTextObjectInterface> for (i32) {
  fn handlerForObject(self , rsthis: & QAbstractTextDocumentLayout) -> QTextObjectInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout16handlerForObjectEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK27QAbstractTextDocumentLayout16handlerForObjectEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextObjectInterface::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QAbstractTextDocumentLayout::frameBoundingRect(QTextFrame * frame);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn frameBoundingRect<RetType, T: QAbstractTextDocumentLayout_frameBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameBoundingRect(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_frameBoundingRect<RetType> {
  fn frameBoundingRect(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QRectF QAbstractTextDocumentLayout::frameBoundingRect(QTextFrame * frame);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_frameBoundingRect<QRectF> for (&'a QTextFrame) {
  fn frameBoundingRect(self , rsthis: & QAbstractTextDocumentLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout17frameBoundingRectEP10QTextFrame()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK27QAbstractTextDocumentLayout17frameBoundingRectEP10QTextFrame(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QAbstractTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn blockBoundingRect<RetType, T: QAbstractTextDocumentLayout_blockBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockBoundingRect(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_blockBoundingRect<RetType> {
  fn blockBoundingRect(self , rsthis: & QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QRectF QAbstractTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_blockBoundingRect<QRectF> for (&'a QTextBlock) {
  fn blockBoundingRect(self , rsthis: & QAbstractTextDocumentLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout17blockBoundingRectERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK27QAbstractTextDocumentLayout17blockBoundingRectERK10QTextBlock(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QAbstractTextDocumentLayout_updateBlock
pub struct QAbstractTextDocumentLayout_updateBlock_signal{poi:u64}
impl /* struct */ QAbstractTextDocumentLayout {
  pub fn updateBlock(&self) -> QAbstractTextDocumentLayout_updateBlock_signal {
     return QAbstractTextDocumentLayout_updateBlock_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractTextDocumentLayout_updateBlock_signal {
  pub fn connect<T: QAbstractTextDocumentLayout_updateBlock_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractTextDocumentLayout_updateBlock_signal_connect {
  fn connect(self, sigthis: QAbstractTextDocumentLayout_updateBlock_signal);
}

#[derive(Default)] // for QAbstractTextDocumentLayout_pageCountChanged
pub struct QAbstractTextDocumentLayout_pageCountChanged_signal{poi:u64}
impl /* struct */ QAbstractTextDocumentLayout {
  pub fn pageCountChanged(&self) -> QAbstractTextDocumentLayout_pageCountChanged_signal {
     return QAbstractTextDocumentLayout_pageCountChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractTextDocumentLayout_pageCountChanged_signal {
  pub fn connect<T: QAbstractTextDocumentLayout_pageCountChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractTextDocumentLayout_pageCountChanged_signal_connect {
  fn connect(self, sigthis: QAbstractTextDocumentLayout_pageCountChanged_signal);
}

#[derive(Default)] // for QAbstractTextDocumentLayout_update
pub struct QAbstractTextDocumentLayout_update_signal{poi:u64}
impl /* struct */ QAbstractTextDocumentLayout {
  pub fn update(&self) -> QAbstractTextDocumentLayout_update_signal {
     return QAbstractTextDocumentLayout_update_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractTextDocumentLayout_update_signal {
  pub fn connect<T: QAbstractTextDocumentLayout_update_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractTextDocumentLayout_update_signal_connect {
  fn connect(self, sigthis: QAbstractTextDocumentLayout_update_signal);
}

#[derive(Default)] // for QAbstractTextDocumentLayout_documentSizeChanged
pub struct QAbstractTextDocumentLayout_documentSizeChanged_signal{poi:u64}
impl /* struct */ QAbstractTextDocumentLayout {
  pub fn documentSizeChanged(&self) -> QAbstractTextDocumentLayout_documentSizeChanged_signal {
     return QAbstractTextDocumentLayout_documentSizeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractTextDocumentLayout_documentSizeChanged_signal {
  pub fn connect<T: QAbstractTextDocumentLayout_documentSizeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractTextDocumentLayout_documentSizeChanged_signal_connect {
  fn connect(self, sigthis: QAbstractTextDocumentLayout_documentSizeChanged_signal);
}

// pageCountChanged(int)
extern fn QAbstractTextDocumentLayout_pageCountChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QAbstractTextDocumentLayout_pageCountChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractTextDocumentLayout_pageCountChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QAbstractTextDocumentLayout_pageCountChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractTextDocumentLayout_pageCountChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractTextDocumentLayout_SlotProxy_connect__ZN27QAbstractTextDocumentLayout16pageCountChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractTextDocumentLayout_pageCountChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QAbstractTextDocumentLayout_pageCountChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractTextDocumentLayout_pageCountChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractTextDocumentLayout_SlotProxy_connect__ZN27QAbstractTextDocumentLayout16pageCountChangedEi(arg0, arg1, arg2)};
  }
}
// documentSizeChanged(const class QSizeF &)
extern fn QAbstractTextDocumentLayout_documentSizeChanged_signal_connect_cb_1(rsfptr:fn(QSizeF), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QSizeF::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QAbstractTextDocumentLayout_documentSizeChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(QSizeF)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QSizeF::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractTextDocumentLayout_documentSizeChanged_signal_connect for fn(QSizeF) {
  fn connect(self, sigthis: QAbstractTextDocumentLayout_documentSizeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractTextDocumentLayout_documentSizeChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractTextDocumentLayout_SlotProxy_connect__ZN27QAbstractTextDocumentLayout19documentSizeChangedERK6QSizeF(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractTextDocumentLayout_documentSizeChanged_signal_connect for Box<Fn(QSizeF)> {
  fn connect(self, sigthis: QAbstractTextDocumentLayout_documentSizeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractTextDocumentLayout_documentSizeChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractTextDocumentLayout_SlotProxy_connect__ZN27QAbstractTextDocumentLayout19documentSizeChangedERK6QSizeF(arg0, arg1, arg2)};
  }
}
// updateBlock(const class QTextBlock &)
extern fn QAbstractTextDocumentLayout_updateBlock_signal_connect_cb_2(rsfptr:fn(QTextBlock), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTextBlock::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QAbstractTextDocumentLayout_updateBlock_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(QTextBlock)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTextBlock::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractTextDocumentLayout_updateBlock_signal_connect for fn(QTextBlock) {
  fn connect(self, sigthis: QAbstractTextDocumentLayout_updateBlock_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractTextDocumentLayout_updateBlock_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractTextDocumentLayout_SlotProxy_connect__ZN27QAbstractTextDocumentLayout11updateBlockERK10QTextBlock(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractTextDocumentLayout_updateBlock_signal_connect for Box<Fn(QTextBlock)> {
  fn connect(self, sigthis: QAbstractTextDocumentLayout_updateBlock_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractTextDocumentLayout_updateBlock_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractTextDocumentLayout_SlotProxy_connect__ZN27QAbstractTextDocumentLayout11updateBlockERK10QTextBlock(arg0, arg1, arg2)};
  }
}
// <= body block end

