// auto generated, do not modify.
// created: Wed Jan 20 00:44:03 2016
// src-file: /QtQuick/designersupport.h
// dst-file: /src/quick/designersupport.rs
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
use super::qquickview::QQuickView; // 773
use super::qquickitem::QQuickItem; // 773
use super::super::core::qstring::QString; // 771
use super::qquickwindow::QQuickWindow; // 773
use super::super::qml::qqmlcontext::QQmlContext; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::core::qsize::QSize; // 771
use super::super::gui::qimage::QImage; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn DesignerSupport_Class_Size() -> c_int;
  // proto: static void DesignerSupport::setRootItem(QQuickView * view, QQuickItem * item);
  fn _ZN15DesignerSupport11setRootItemEP10QQuickViewP10QQuickItem(arg0: *mut c_void, arg1: *mut c_void);
  // proto: static bool DesignerSupport::hasAnchor(QQuickItem * item, const QString & name);
  fn _ZN15DesignerSupport9hasAnchorEP10QQuickItemRK7QString(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static void DesignerSupport::polishItems(QQuickWindow * window);
  fn _ZN15DesignerSupport11polishItemsEP12QQuickWindow(arg0: *mut c_void);
  // proto: static bool DesignerSupport::isDirty(QQuickItem * referencedItem, DesignerSupport::DirtyType dirtyType);
  fn _ZN15DesignerSupport7isDirtyEP10QQuickItemNS_9DirtyTypeE(arg0: *mut c_void, arg1: c_int) -> c_char;
  // proto: static void DesignerSupport::refreshExpressions(QQmlContext * context);
  fn _ZN15DesignerSupport18refreshExpressionsEP11QQmlContext(arg0: *mut c_void);
  // proto: static QPair<QString, QObject *> DesignerSupport::anchorLineTarget(QQuickItem * item, const QString & name, QQmlContext * context);
  fn _ZN15DesignerSupport16anchorLineTargetEP10QQuickItemRK7QStringP11QQmlContext(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto: static bool DesignerSupport::isAnchoredTo(QQuickItem * fromItem, QQuickItem * toItem);
  fn _ZN15DesignerSupport12isAnchoredToEP10QQuickItemS1_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto: static void DesignerSupport::resetDirty(QQuickItem * referencedItem);
  fn _ZN15DesignerSupport10resetDirtyEP10QQuickItem(arg0: *mut c_void);
  // proto:  QImage DesignerSupport::renderImageForItem(QQuickItem * referencedItem, const QRectF & boundingRect, const QSize & imageSize);
  fn _ZN15DesignerSupport18renderImageForItemEP10QQuickItemRK6QRectFRK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void DesignerSupport::refFromEffectItem(QQuickItem * referencedItem, bool hide);
  fn _ZN15DesignerSupport17refFromEffectItemEP10QQuickItemb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(DesignerSupport)=1
#[derive(Default)]
pub struct DesignerSupport {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ DesignerSupport {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> DesignerSupport {
    return DesignerSupport{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static void DesignerSupport::setRootItem(QQuickView * view, QQuickItem * item);
impl /*struct*/ DesignerSupport {
  pub fn setRootItem_s<RetType, T: DesignerSupport_setRootItem_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setRootItem_s();
    // return 1;
  }
}

pub trait DesignerSupport_setRootItem_s<RetType> {
  fn setRootItem_s(self ) -> RetType;
}

  // proto: static void DesignerSupport::setRootItem(QQuickView * view, QQuickItem * item);
impl<'a> /*trait*/ DesignerSupport_setRootItem_s<()> for (&'a QQuickView, &'a QQuickItem) {
  fn setRootItem_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15DesignerSupport11setRootItemEP10QQuickViewP10QQuickItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN15DesignerSupport11setRootItemEP10QQuickViewP10QQuickItem(arg0, arg1)};
    // return 1;
  }
}

  // proto: static bool DesignerSupport::hasAnchor(QQuickItem * item, const QString & name);
impl /*struct*/ DesignerSupport {
  pub fn hasAnchor_s<RetType, T: DesignerSupport_hasAnchor_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasAnchor_s();
    // return 1;
  }
}

pub trait DesignerSupport_hasAnchor_s<RetType> {
  fn hasAnchor_s(self ) -> RetType;
}

  // proto: static bool DesignerSupport::hasAnchor(QQuickItem * item, const QString & name);
impl<'a> /*trait*/ DesignerSupport_hasAnchor_s<i8> for (&'a QQuickItem, &'a QString) {
  fn hasAnchor_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15DesignerSupport9hasAnchorEP10QQuickItemRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15DesignerSupport9hasAnchorEP10QQuickItemRK7QString(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void DesignerSupport::polishItems(QQuickWindow * window);
impl /*struct*/ DesignerSupport {
  pub fn polishItems_s<RetType, T: DesignerSupport_polishItems_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.polishItems_s();
    // return 1;
  }
}

pub trait DesignerSupport_polishItems_s<RetType> {
  fn polishItems_s(self ) -> RetType;
}

  // proto: static void DesignerSupport::polishItems(QQuickWindow * window);
impl<'a> /*trait*/ DesignerSupport_polishItems_s<()> for (&'a QQuickWindow) {
  fn polishItems_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15DesignerSupport11polishItemsEP12QQuickWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15DesignerSupport11polishItemsEP12QQuickWindow(arg0)};
    // return 1;
  }
}

  // proto: static bool DesignerSupport::isDirty(QQuickItem * referencedItem, DesignerSupport::DirtyType dirtyType);
impl /*struct*/ DesignerSupport {
  pub fn isDirty_s<RetType, T: DesignerSupport_isDirty_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isDirty_s();
    // return 1;
  }
}

pub trait DesignerSupport_isDirty_s<RetType> {
  fn isDirty_s(self ) -> RetType;
}

  // proto: static bool DesignerSupport::isDirty(QQuickItem * referencedItem, DesignerSupport::DirtyType dirtyType);
impl<'a> /*trait*/ DesignerSupport_isDirty_s<i8> for (&'a QQuickItem, i32) {
  fn isDirty_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15DesignerSupport7isDirtyEP10QQuickItemNS_9DirtyTypeE()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN15DesignerSupport7isDirtyEP10QQuickItemNS_9DirtyTypeE(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void DesignerSupport::refreshExpressions(QQmlContext * context);
impl /*struct*/ DesignerSupport {
  pub fn refreshExpressions_s<RetType, T: DesignerSupport_refreshExpressions_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.refreshExpressions_s();
    // return 1;
  }
}

pub trait DesignerSupport_refreshExpressions_s<RetType> {
  fn refreshExpressions_s(self ) -> RetType;
}

  // proto: static void DesignerSupport::refreshExpressions(QQmlContext * context);
impl<'a> /*trait*/ DesignerSupport_refreshExpressions_s<()> for (&'a QQmlContext) {
  fn refreshExpressions_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15DesignerSupport18refreshExpressionsEP11QQmlContext()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15DesignerSupport18refreshExpressionsEP11QQmlContext(arg0)};
    // return 1;
  }
}

  // proto: static QPair<QString, QObject *> DesignerSupport::anchorLineTarget(QQuickItem * item, const QString & name, QQmlContext * context);
impl /*struct*/ DesignerSupport {
  pub fn anchorLineTarget_s<RetType, T: DesignerSupport_anchorLineTarget_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.anchorLineTarget_s();
    // return 1;
  }
}

pub trait DesignerSupport_anchorLineTarget_s<RetType> {
  fn anchorLineTarget_s(self ) -> RetType;
}

  // proto: static QPair<QString, QObject *> DesignerSupport::anchorLineTarget(QQuickItem * item, const QString & name, QQmlContext * context);
impl<'a> /*trait*/ DesignerSupport_anchorLineTarget_s<()> for (&'a QQuickItem, &'a QString, &'a QQmlContext) {
  fn anchorLineTarget_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15DesignerSupport16anchorLineTargetEP10QQuickItemRK7QStringP11QQmlContext()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN15DesignerSupport16anchorLineTargetEP10QQuickItemRK7QStringP11QQmlContext(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto: static bool DesignerSupport::isAnchoredTo(QQuickItem * fromItem, QQuickItem * toItem);
impl /*struct*/ DesignerSupport {
  pub fn isAnchoredTo_s<RetType, T: DesignerSupport_isAnchoredTo_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isAnchoredTo_s();
    // return 1;
  }
}

pub trait DesignerSupport_isAnchoredTo_s<RetType> {
  fn isAnchoredTo_s(self ) -> RetType;
}

  // proto: static bool DesignerSupport::isAnchoredTo(QQuickItem * fromItem, QQuickItem * toItem);
impl<'a> /*trait*/ DesignerSupport_isAnchoredTo_s<i8> for (&'a QQuickItem, &'a QQuickItem) {
  fn isAnchoredTo_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15DesignerSupport12isAnchoredToEP10QQuickItemS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15DesignerSupport12isAnchoredToEP10QQuickItemS1_(arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void DesignerSupport::resetDirty(QQuickItem * referencedItem);
impl /*struct*/ DesignerSupport {
  pub fn resetDirty_s<RetType, T: DesignerSupport_resetDirty_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.resetDirty_s();
    // return 1;
  }
}

pub trait DesignerSupport_resetDirty_s<RetType> {
  fn resetDirty_s(self ) -> RetType;
}

  // proto: static void DesignerSupport::resetDirty(QQuickItem * referencedItem);
impl<'a> /*trait*/ DesignerSupport_resetDirty_s<()> for (&'a QQuickItem) {
  fn resetDirty_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15DesignerSupport10resetDirtyEP10QQuickItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15DesignerSupport10resetDirtyEP10QQuickItem(arg0)};
    // return 1;
  }
}

  // proto:  QImage DesignerSupport::renderImageForItem(QQuickItem * referencedItem, const QRectF & boundingRect, const QSize & imageSize);
impl /*struct*/ DesignerSupport {
  pub fn renderImageForItem<RetType, T: DesignerSupport_renderImageForItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.renderImageForItem(self);
    // return 1;
  }
}

pub trait DesignerSupport_renderImageForItem<RetType> {
  fn renderImageForItem(self , rsthis: & DesignerSupport) -> RetType;
}

  // proto:  QImage DesignerSupport::renderImageForItem(QQuickItem * referencedItem, const QRectF & boundingRect, const QSize & imageSize);
impl<'a> /*trait*/ DesignerSupport_renderImageForItem<QImage> for (&'a QQuickItem, &'a QRectF, &'a QSize) {
  fn renderImageForItem(self , rsthis: & DesignerSupport) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15DesignerSupport18renderImageForItemEP10QQuickItemRK6QRectFRK5QSize()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15DesignerSupport18renderImageForItemEP10QQuickItemRK6QRectFRK5QSize(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QImage::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void DesignerSupport::refFromEffectItem(QQuickItem * referencedItem, bool hide);
impl /*struct*/ DesignerSupport {
  pub fn refFromEffectItem<RetType, T: DesignerSupport_refFromEffectItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.refFromEffectItem(self);
    // return 1;
  }
}

pub trait DesignerSupport_refFromEffectItem<RetType> {
  fn refFromEffectItem(self , rsthis: & DesignerSupport) -> RetType;
}

  // proto:  void DesignerSupport::refFromEffectItem(QQuickItem * referencedItem, bool hide);
impl<'a> /*trait*/ DesignerSupport_refFromEffectItem<()> for (&'a QQuickItem, i8) {
  fn refFromEffectItem(self , rsthis: & DesignerSupport) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15DesignerSupport17refFromEffectItemEP10QQuickItemb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {_ZN15DesignerSupport17refFromEffectItemEP10QQuickItemb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

