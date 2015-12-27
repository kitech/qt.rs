// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtWidgets/qgraphicslayoutitem.h
// dst-file: /src/widgets/qgraphicslayoutitem.rs
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
use super::qsizepolicy::QSizePolicy; // 773
use super::super::core::qsize::QSizeF; // 771
use super::super::core::qrect::QRectF; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsLayoutItem_Class_Size() -> c_int;
  // proto:  void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy & policy);
  fn _ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QGraphicsLayoutItem * QGraphicsLayoutItem::parentLayoutItem();
  fn _ZNK19QGraphicsLayoutItem16parentLayoutItemEv(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QGraphicsLayoutItem::minimumWidth();
  fn demth_ZNK19QGraphicsLayoutItem12minimumWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QGraphicsItem * QGraphicsLayoutItem::graphicsItem();
  fn _ZNK19QGraphicsLayoutItem12graphicsItemEv(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QGraphicsLayoutItem::preferredWidth();
  fn demth_ZNK19QGraphicsLayoutItem14preferredWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QGraphicsLayoutItem::ownedByLayout();
  fn _ZNK19QGraphicsLayoutItem13ownedByLayoutEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSizeF QGraphicsLayoutItem::preferredSize();
  fn _ZNK19QGraphicsLayoutItem13preferredSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGraphicsLayoutItem::geometry();
  fn _ZNK19QGraphicsLayoutItem8geometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsLayoutItem::QGraphicsLayoutItem(QGraphicsLayoutItem * parent, bool isLayout);
  fn dector_ZN19QGraphicsLayoutItemC1EPS_b(arg0: *mut c_void, arg1: c_char) -> *mut c_void;
  fn _ZN19QGraphicsLayoutItemC1EPS_b(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  qreal QGraphicsLayoutItem::minimumHeight();
  fn demth_ZNK19QGraphicsLayoutItem13minimumHeightEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QGraphicsLayoutItem::preferredHeight();
  fn demth_ZNK19QGraphicsLayoutItem15preferredHeightEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QSizeF QGraphicsLayoutItem::maximumSize();
  fn _ZNK19QGraphicsLayoutItem11maximumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSizePolicy QGraphicsLayoutItem::sizePolicy();
  fn _ZNK19QGraphicsLayoutItem10sizePolicyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QGraphicsLayoutItem::maximumHeight();
  fn demth_ZNK19QGraphicsLayoutItem13maximumHeightEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsLayoutItem::setGeometry(const QRectF & rect);
  fn _ZN19QGraphicsLayoutItem11setGeometryERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsLayoutItem::setPreferredWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem17setPreferredWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsLayoutItem::setMaximumSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QGraphicsLayoutItem::maximumWidth();
  fn demth_ZNK19QGraphicsLayoutItem12maximumWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsLayoutItem::setMinimumSize(qreal w, qreal h);
  fn demth_ZN19QGraphicsLayoutItem14setMinimumSizeEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  void QGraphicsLayoutItem::setMaximumHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem16setMaximumHeightEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsLayoutItem::setMinimumSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsLayoutItem::setPreferredSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsLayoutItem::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_(qthis: u64 /* *mut c_void*/, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double);
  // proto:  void QGraphicsLayoutItem::setParentLayoutItem(QGraphicsLayoutItem * parent);
  fn _ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsLayoutItem::setMinimumWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem15setMinimumWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsLayoutItem::setMaximumWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem15setMaximumWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsLayoutItem::updateGeometry();
  fn _ZN19QGraphicsLayoutItem14updateGeometryEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsLayoutItem::setPreferredHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem18setPreferredHeightEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QSizeF QGraphicsLayoutItem::minimumSize();
  fn _ZNK19QGraphicsLayoutItem11minimumSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGraphicsLayoutItem::contentsRect();
  fn _ZNK19QGraphicsLayoutItem12contentsRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGraphicsLayoutItem::isLayout();
  fn _ZNK19QGraphicsLayoutItem8isLayoutEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGraphicsLayoutItem::setPreferredSize(qreal w, qreal h);
  fn demth_ZN19QGraphicsLayoutItem16setPreferredSizeEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  void QGraphicsLayoutItem::~QGraphicsLayoutItem();
  fn _ZN19QGraphicsLayoutItemD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsLayoutItem::setMinimumHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem16setMinimumHeightEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsLayoutItem::setMaximumSize(qreal w, qreal h);
  fn demth_ZN19QGraphicsLayoutItem14setMaximumSizeEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsLayoutItem)=1
#[derive(Default)]
pub struct QGraphicsLayoutItem {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsLayoutItem {
    return QGraphicsLayoutItem{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy & policy);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setSizePolicy<RetType, T: QGraphicsLayoutItem_setSizePolicy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSizePolicy(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setSizePolicy<RetType> {
  fn setSizePolicy(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy & policy);
impl<'a> /*trait*/ QGraphicsLayoutItem_setSizePolicy<()> for (&'a QSizePolicy) {
  fn setSizePolicy(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsLayoutItem * QGraphicsLayoutItem::parentLayoutItem();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn parentLayoutItem<RetType, T: QGraphicsLayoutItem_parentLayoutItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentLayoutItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_parentLayoutItem<RetType> {
  fn parentLayoutItem(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  QGraphicsLayoutItem * QGraphicsLayoutItem::parentLayoutItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_parentLayoutItem<()> for () {
  fn parentLayoutItem(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem16parentLayoutItemEv()};
     unsafe {_ZNK19QGraphicsLayoutItem16parentLayoutItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::minimumWidth();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumWidth<RetType, T: QGraphicsLayoutItem_minimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumWidth<RetType> {
  fn minimumWidth(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::minimumWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumWidth<f64> for () {
  fn minimumWidth(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12minimumWidthEv()};
    let mut ret = unsafe {demth_ZNK19QGraphicsLayoutItem12minimumWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsLayoutItem::graphicsItem();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn graphicsItem<RetType, T: QGraphicsLayoutItem_graphicsItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.graphicsItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_graphicsItem<RetType> {
  fn graphicsItem(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsLayoutItem::graphicsItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_graphicsItem<()> for () {
  fn graphicsItem(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12graphicsItemEv()};
     unsafe {_ZNK19QGraphicsLayoutItem12graphicsItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::preferredWidth();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredWidth<RetType, T: QGraphicsLayoutItem_preferredWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preferredWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredWidth<RetType> {
  fn preferredWidth(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::preferredWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredWidth<f64> for () {
  fn preferredWidth(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem14preferredWidthEv()};
    let mut ret = unsafe {demth_ZNK19QGraphicsLayoutItem14preferredWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QGraphicsLayoutItem::ownedByLayout();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn ownedByLayout<RetType, T: QGraphicsLayoutItem_ownedByLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ownedByLayout(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_ownedByLayout<RetType> {
  fn ownedByLayout(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  bool QGraphicsLayoutItem::ownedByLayout();
impl<'a> /*trait*/ QGraphicsLayoutItem_ownedByLayout<i8> for () {
  fn ownedByLayout(self , rsthis: & QGraphicsLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13ownedByLayoutEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem13ownedByLayoutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSizeF QGraphicsLayoutItem::preferredSize();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredSize<RetType, T: QGraphicsLayoutItem_preferredSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preferredSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredSize<RetType> {
  fn preferredSize(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  QSizeF QGraphicsLayoutItem::preferredSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredSize<QSizeF> for () {
  fn preferredSize(self , rsthis: & QGraphicsLayoutItem) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13preferredSizeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem13preferredSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsLayoutItem::geometry();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn geometry<RetType, T: QGraphicsLayoutItem_geometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_geometry<RetType> {
  fn geometry(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  QRectF QGraphicsLayoutItem::geometry();
impl<'a> /*trait*/ QGraphicsLayoutItem_geometry<QRectF> for () {
  fn geometry(self , rsthis: & QGraphicsLayoutItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem8geometryEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::QGraphicsLayoutItem(QGraphicsLayoutItem * parent, bool isLayout);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn New<T: QGraphicsLayoutItem_New>(value: T) -> QGraphicsLayoutItem {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_New {
  fn New(self) -> QGraphicsLayoutItem;
}

  // proto:  void QGraphicsLayoutItem::QGraphicsLayoutItem(QGraphicsLayoutItem * parent, bool isLayout);
impl<'a> /*trait*/ QGraphicsLayoutItem_New for (&'a QGraphicsLayoutItem, i8) {
  fn New(self) -> QGraphicsLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItemC1EPS_b()};
    let ctysz: c_int = unsafe{QGraphicsLayoutItem_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
    // unsafe {_ZN19QGraphicsLayoutItemC1EPS_b(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN19QGraphicsLayoutItemC1EPS_b(arg0, arg1)} as u64;
    let rsthis = QGraphicsLayoutItem{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::minimumHeight();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumHeight<RetType, T: QGraphicsLayoutItem_minimumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumHeight<RetType> {
  fn minimumHeight(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::minimumHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumHeight<f64> for () {
  fn minimumHeight(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13minimumHeightEv()};
    let mut ret = unsafe {demth_ZNK19QGraphicsLayoutItem13minimumHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::preferredHeight();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredHeight<RetType, T: QGraphicsLayoutItem_preferredHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preferredHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredHeight<RetType> {
  fn preferredHeight(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::preferredHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredHeight<f64> for () {
  fn preferredHeight(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem15preferredHeightEv()};
    let mut ret = unsafe {demth_ZNK19QGraphicsLayoutItem15preferredHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QSizeF QGraphicsLayoutItem::maximumSize();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumSize<RetType, T: QGraphicsLayoutItem_maximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  QSizeF QGraphicsLayoutItem::maximumSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumSize<QSizeF> for () {
  fn maximumSize(self , rsthis: & QGraphicsLayoutItem) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizePolicy QGraphicsLayoutItem::sizePolicy();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn sizePolicy<RetType, T: QGraphicsLayoutItem_sizePolicy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizePolicy(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_sizePolicy<RetType> {
  fn sizePolicy(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  QSizePolicy QGraphicsLayoutItem::sizePolicy();
impl<'a> /*trait*/ QGraphicsLayoutItem_sizePolicy<QSizePolicy> for () {
  fn sizePolicy(self , rsthis: & QGraphicsLayoutItem) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem10sizePolicyEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem10sizePolicyEv(rsthis.qclsinst)};
    let mut ret1 = QSizePolicy::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::maximumHeight();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumHeight<RetType, T: QGraphicsLayoutItem_maximumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumHeight<RetType> {
  fn maximumHeight(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::maximumHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumHeight<f64> for () {
  fn maximumHeight(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13maximumHeightEv()};
    let mut ret = unsafe {demth_ZNK19QGraphicsLayoutItem13maximumHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setGeometry(const QRectF & rect);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setGeometry<RetType, T: QGraphicsLayoutItem_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsLayoutItem_setGeometry<()> for (&'a QRectF) {
  fn setGeometry(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setPreferredWidth(qreal width);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredWidth<RetType, T: QGraphicsLayoutItem_setPreferredWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPreferredWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredWidth<RetType> {
  fn setPreferredWidth(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setPreferredWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredWidth<()> for (f64) {
  fn setPreferredWidth(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem17setPreferredWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem17setPreferredWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMaximumSize(const QSizeF & size);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumSize<RetType, T: QGraphicsLayoutItem_setMaximumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumSize<RetType> {
  fn setMaximumSize(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMaximumSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumSize<()> for (&'a QSizeF) {
  fn setMaximumSize(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::maximumWidth();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumWidth<RetType, T: QGraphicsLayoutItem_maximumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumWidth<RetType> {
  fn maximumWidth(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::maximumWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumWidth<f64> for () {
  fn maximumWidth(self , rsthis: & QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12maximumWidthEv()};
    let mut ret = unsafe {demth_ZNK19QGraphicsLayoutItem12maximumWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMinimumSize(qreal w, qreal h);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumSize<RetType, T: QGraphicsLayoutItem_setMinimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumSize<RetType> {
  fn setMinimumSize(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMinimumSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumSize<()> for (f64, f64) {
  fn setMinimumSize(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMinimumSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {demth_ZN19QGraphicsLayoutItem14setMinimumSizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMaximumHeight(qreal height);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumHeight<RetType, T: QGraphicsLayoutItem_setMaximumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumHeight<RetType> {
  fn setMaximumHeight(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMaximumHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumHeight<()> for (f64) {
  fn setMaximumHeight(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setMaximumHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem16setMaximumHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMinimumSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumSize<()> for (&'a QSizeF) {
  fn setMinimumSize(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setPreferredSize(const QSizeF & size);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredSize<RetType, T: QGraphicsLayoutItem_setPreferredSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPreferredSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredSize<RetType> {
  fn setPreferredSize(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setPreferredSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredSize<()> for (&'a QSizeF) {
  fn setPreferredSize(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn getContentsMargins<RetType, T: QGraphicsLayoutItem_getContentsMargins<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_getContentsMargins<RetType> {
  fn getContentsMargins(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsLayoutItem_getContentsMargins<()> for (&'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>) {
  fn getContentsMargins(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_()};
    let arg0 = self.0.as_ptr()  as *mut c_double;
    let arg1 = self.1.as_ptr()  as *mut c_double;
    let arg2 = self.2.as_ptr()  as *mut c_double;
    let arg3 = self.3.as_ptr()  as *mut c_double;
     unsafe {_ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setParentLayoutItem(QGraphicsLayoutItem * parent);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setParentLayoutItem<RetType, T: QGraphicsLayoutItem_setParentLayoutItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setParentLayoutItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setParentLayoutItem<RetType> {
  fn setParentLayoutItem(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setParentLayoutItem(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsLayoutItem_setParentLayoutItem<()> for (&'a QGraphicsLayoutItem) {
  fn setParentLayoutItem(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMinimumWidth(qreal width);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumWidth<RetType, T: QGraphicsLayoutItem_setMinimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumWidth<RetType> {
  fn setMinimumWidth(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMinimumWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumWidth<()> for (f64) {
  fn setMinimumWidth(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem15setMinimumWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem15setMinimumWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMaximumWidth(qreal width);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumWidth<RetType, T: QGraphicsLayoutItem_setMaximumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumWidth<RetType> {
  fn setMaximumWidth(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMaximumWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumWidth<()> for (f64) {
  fn setMaximumWidth(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem15setMaximumWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem15setMaximumWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::updateGeometry();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn updateGeometry<RetType, T: QGraphicsLayoutItem_updateGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_updateGeometry<RetType> {
  fn updateGeometry(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::updateGeometry();
impl<'a> /*trait*/ QGraphicsLayoutItem_updateGeometry<()> for () {
  fn updateGeometry(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14updateGeometryEv()};
     unsafe {_ZN19QGraphicsLayoutItem14updateGeometryEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setPreferredHeight(qreal height);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredHeight<RetType, T: QGraphicsLayoutItem_setPreferredHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPreferredHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredHeight<RetType> {
  fn setPreferredHeight(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setPreferredHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredHeight<()> for (f64) {
  fn setPreferredHeight(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem18setPreferredHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem18setPreferredHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSizeF QGraphicsLayoutItem::minimumSize();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumSize<RetType, T: QGraphicsLayoutItem_minimumSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  QSizeF QGraphicsLayoutItem::minimumSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumSize<QSizeF> for () {
  fn minimumSize(self , rsthis: & QGraphicsLayoutItem) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsLayoutItem::contentsRect();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn contentsRect<RetType, T: QGraphicsLayoutItem_contentsRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contentsRect(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_contentsRect<RetType> {
  fn contentsRect(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  QRectF QGraphicsLayoutItem::contentsRect();
impl<'a> /*trait*/ QGraphicsLayoutItem_contentsRect<QRectF> for () {
  fn contentsRect(self , rsthis: & QGraphicsLayoutItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12contentsRectEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem12contentsRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsLayoutItem::isLayout();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn isLayout<RetType, T: QGraphicsLayoutItem_isLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isLayout(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_isLayout<RetType> {
  fn isLayout(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  bool QGraphicsLayoutItem::isLayout();
impl<'a> /*trait*/ QGraphicsLayoutItem_isLayout<i8> for () {
  fn isLayout(self , rsthis: & QGraphicsLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem8isLayoutEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem8isLayoutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setPreferredSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredSize<()> for (f64, f64) {
  fn setPreferredSize(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setPreferredSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {demth_ZN19QGraphicsLayoutItem16setPreferredSizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::~QGraphicsLayoutItem();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn Free<RetType, T: QGraphicsLayoutItem_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::~QGraphicsLayoutItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItemD0Ev()};
     unsafe {_ZN19QGraphicsLayoutItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMinimumHeight(qreal height);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumHeight<RetType, T: QGraphicsLayoutItem_setMinimumHeight<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumHeight<RetType> {
  fn setMinimumHeight(self , rsthis: & QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMinimumHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumHeight<()> for (f64) {
  fn setMinimumHeight(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setMinimumHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem16setMinimumHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMaximumSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumSize<()> for (f64, f64) {
  fn setMaximumSize(self , rsthis: & QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMaximumSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {demth_ZN19QGraphicsLayoutItem14setMaximumSizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

