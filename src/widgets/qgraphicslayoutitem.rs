// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy & policy);
  fn _ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QGraphicsLayoutItem * QGraphicsLayoutItem::parentLayoutItem();
  fn _ZNK19QGraphicsLayoutItem16parentLayoutItemEv(qthis: *mut c_void);
  // proto:  qreal QGraphicsLayoutItem::minimumWidth();
  fn _ZNK19QGraphicsLayoutItem12minimumWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  QGraphicsItem * QGraphicsLayoutItem::graphicsItem();
  fn _ZNK19QGraphicsLayoutItem12graphicsItemEv(qthis: *mut c_void);
  // proto:  qreal QGraphicsLayoutItem::preferredWidth();
  fn _ZNK19QGraphicsLayoutItem14preferredWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QGraphicsLayoutItem::ownedByLayout();
  fn _ZNK19QGraphicsLayoutItem13ownedByLayoutEv(qthis: *mut c_void) -> c_char;
  // proto:  QSizeF QGraphicsLayoutItem::preferredSize();
  fn _ZNK19QGraphicsLayoutItem13preferredSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsLayoutItem::geometry();
  fn _ZNK19QGraphicsLayoutItem8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsLayoutItem::QGraphicsLayoutItem(QGraphicsLayoutItem * parent, bool isLayout);
  fn _ZN19QGraphicsLayoutItemC1EPS_b(qthis: *mut c_void, arg0: *mut c_void, arg1: c_char);
  // proto:  qreal QGraphicsLayoutItem::minimumHeight();
  fn _ZNK19QGraphicsLayoutItem13minimumHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QGraphicsLayoutItem::preferredHeight();
  fn _ZNK19QGraphicsLayoutItem15preferredHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  QSizeF QGraphicsLayoutItem::maximumSize();
  fn _ZNK19QGraphicsLayoutItem11maximumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSizePolicy QGraphicsLayoutItem::sizePolicy();
  fn _ZNK19QGraphicsLayoutItem10sizePolicyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QGraphicsLayoutItem::maximumHeight();
  fn _ZNK19QGraphicsLayoutItem13maximumHeightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsLayoutItem::setGeometry(const QRectF & rect);
  fn _ZN19QGraphicsLayoutItem11setGeometryERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsLayoutItem::setPreferredWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem17setPreferredWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsLayoutItem::setMaximumSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QGraphicsLayoutItem::maximumWidth();
  fn _ZNK19QGraphicsLayoutItem12maximumWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsLayoutItem::setMinimumSize(qreal w, qreal h);
  fn _ZN19QGraphicsLayoutItem14setMinimumSizeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QGraphicsLayoutItem::setMaximumHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem16setMaximumHeightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsLayoutItem::setMinimumSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsLayoutItem::setPreferredSize(const QSizeF & size);
  fn _ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsLayoutItem::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
  fn _ZNK19QGraphicsLayoutItem18getContentsMarginsEPdS0_S0_S0_(qthis: *mut c_void, arg0: *mut c_double, arg1: *mut c_double, arg2: *mut c_double, arg3: *mut c_double);
  // proto:  void QGraphicsLayoutItem::setParentLayoutItem(QGraphicsLayoutItem * parent);
  fn _ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsLayoutItem::setMinimumWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem15setMinimumWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsLayoutItem::setMaximumWidth(qreal width);
  fn _ZN19QGraphicsLayoutItem15setMaximumWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsLayoutItem::updateGeometry();
  fn _ZN19QGraphicsLayoutItem14updateGeometryEv(qthis: *mut c_void);
  // proto:  void QGraphicsLayoutItem::setPreferredHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem18setPreferredHeightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QSizeF QGraphicsLayoutItem::minimumSize();
  fn _ZNK19QGraphicsLayoutItem11minimumSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRectF QGraphicsLayoutItem::contentsRect();
  fn _ZNK19QGraphicsLayoutItem12contentsRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsLayoutItem::isLayout();
  fn _ZNK19QGraphicsLayoutItem8isLayoutEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGraphicsLayoutItem::setPreferredSize(qreal w, qreal h);
  fn _ZN19QGraphicsLayoutItem16setPreferredSizeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QGraphicsLayoutItem::~QGraphicsLayoutItem();
  fn _ZN19QGraphicsLayoutItemD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsLayoutItem::setMinimumHeight(qreal height);
  fn _ZN19QGraphicsLayoutItem16setMinimumHeightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsLayoutItem::setMaximumSize(qreal w, qreal h);
  fn _ZN19QGraphicsLayoutItem14setMaximumSizeEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsLayoutItem)=1
pub struct QGraphicsLayoutItem {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsLayoutItem {
  pub fn inheritFrom(qthis: *mut c_void) -> QGraphicsLayoutItem {
    return QGraphicsLayoutItem{qclsinst: qthis};
  }
}
  // proto:  void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy & policy);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setSizePolicy<RetType, T: QGraphicsLayoutItem_setSizePolicy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSizePolicy(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setSizePolicy<RetType> {
  fn setSizePolicy(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setSizePolicy(const QSizePolicy & policy);
impl<'a> /*trait*/ QGraphicsLayoutItem_setSizePolicy<()> for (QSizePolicy) {
  fn setSizePolicy(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem13setSizePolicyERK11QSizePolicy(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QGraphicsLayoutItem * QGraphicsLayoutItem::parentLayoutItem();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn parentLayoutItem<RetType, T: QGraphicsLayoutItem_parentLayoutItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parentLayoutItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_parentLayoutItem<RetType> {
  fn parentLayoutItem(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  QGraphicsLayoutItem * QGraphicsLayoutItem::parentLayoutItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_parentLayoutItem<()> for () {
  fn parentLayoutItem(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem16parentLayoutItemEv()};
     unsafe {_ZNK19QGraphicsLayoutItem16parentLayoutItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::minimumWidth();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumWidth<RetType, T: QGraphicsLayoutItem_minimumWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumWidth<RetType> {
  fn minimumWidth(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::minimumWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumWidth<f64> for () {
  fn minimumWidth(self , rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12minimumWidthEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem12minimumWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsLayoutItem::graphicsItem();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn graphicsItem<RetType, T: QGraphicsLayoutItem_graphicsItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.graphicsItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_graphicsItem<RetType> {
  fn graphicsItem(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsLayoutItem::graphicsItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_graphicsItem<()> for () {
  fn graphicsItem(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12graphicsItemEv()};
     unsafe {_ZNK19QGraphicsLayoutItem12graphicsItemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::preferredWidth();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredWidth<RetType, T: QGraphicsLayoutItem_preferredWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.preferredWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredWidth<RetType> {
  fn preferredWidth(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::preferredWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredWidth<f64> for () {
  fn preferredWidth(self , rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem14preferredWidthEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem14preferredWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QGraphicsLayoutItem::ownedByLayout();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn ownedByLayout<RetType, T: QGraphicsLayoutItem_ownedByLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ownedByLayout(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_ownedByLayout<RetType> {
  fn ownedByLayout(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  bool QGraphicsLayoutItem::ownedByLayout();
impl<'a> /*trait*/ QGraphicsLayoutItem_ownedByLayout<i8> for () {
  fn ownedByLayout(self , rsthis: &mut QGraphicsLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13ownedByLayoutEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem13ownedByLayoutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSizeF QGraphicsLayoutItem::preferredSize();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredSize<RetType, T: QGraphicsLayoutItem_preferredSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.preferredSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredSize<RetType> {
  fn preferredSize(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  QSizeF QGraphicsLayoutItem::preferredSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredSize<QSizeF> for () {
  fn preferredSize(self , rsthis: &mut QGraphicsLayoutItem) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13preferredSizeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem13preferredSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsLayoutItem::geometry();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn geometry<RetType, T: QGraphicsLayoutItem_geometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_geometry<RetType> {
  fn geometry(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  QRectF QGraphicsLayoutItem::geometry();
impl<'a> /*trait*/ QGraphicsLayoutItem_geometry<QRectF> for () {
  fn geometry(self , rsthis: &mut QGraphicsLayoutItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem8geometryEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::QGraphicsLayoutItem(QGraphicsLayoutItem * parent, bool isLayout);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn NewQGraphicsLayoutItem<T: QGraphicsLayoutItem_NewQGraphicsLayoutItem>(value: T) -> QGraphicsLayoutItem {
    let rsthis = value.NewQGraphicsLayoutItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_NewQGraphicsLayoutItem {
  fn NewQGraphicsLayoutItem(self) -> QGraphicsLayoutItem;
}

  // proto:  void QGraphicsLayoutItem::QGraphicsLayoutItem(QGraphicsLayoutItem * parent, bool isLayout);
impl<'a> /*trait*/ QGraphicsLayoutItem_NewQGraphicsLayoutItem for (QGraphicsLayoutItem, i8) {
  fn NewQGraphicsLayoutItem(self) -> QGraphicsLayoutItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItemC1EPS_b()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
    unsafe {_ZN19QGraphicsLayoutItemC1EPS_b(qthis, arg0, arg1)};
    let rsthis = QGraphicsLayoutItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::minimumHeight();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumHeight<RetType, T: QGraphicsLayoutItem_minimumHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumHeight<RetType> {
  fn minimumHeight(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::minimumHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumHeight<f64> for () {
  fn minimumHeight(self , rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13minimumHeightEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem13minimumHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::preferredHeight();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn preferredHeight<RetType, T: QGraphicsLayoutItem_preferredHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.preferredHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_preferredHeight<RetType> {
  fn preferredHeight(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::preferredHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_preferredHeight<f64> for () {
  fn preferredHeight(self , rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem15preferredHeightEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem15preferredHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QSizeF QGraphicsLayoutItem::maximumSize();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumSize<RetType, T: QGraphicsLayoutItem_maximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumSize<RetType> {
  fn maximumSize(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  QSizeF QGraphicsLayoutItem::maximumSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumSize<QSizeF> for () {
  fn maximumSize(self , rsthis: &mut QGraphicsLayoutItem) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem11maximumSizeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem11maximumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSizePolicy QGraphicsLayoutItem::sizePolicy();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn sizePolicy<RetType, T: QGraphicsLayoutItem_sizePolicy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizePolicy(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_sizePolicy<RetType> {
  fn sizePolicy(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  QSizePolicy QGraphicsLayoutItem::sizePolicy();
impl<'a> /*trait*/ QGraphicsLayoutItem_sizePolicy<QSizePolicy> for () {
  fn sizePolicy(self , rsthis: &mut QGraphicsLayoutItem) -> QSizePolicy {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem10sizePolicyEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem10sizePolicyEv(rsthis.qclsinst)};
    let mut ret1 = QSizePolicy::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::maximumHeight();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumHeight<RetType, T: QGraphicsLayoutItem_maximumHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumHeight<RetType> {
  fn maximumHeight(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::maximumHeight();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumHeight<f64> for () {
  fn maximumHeight(self , rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem13maximumHeightEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem13maximumHeightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setGeometry(const QRectF & rect);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setGeometry<RetType, T: QGraphicsLayoutItem_setGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsLayoutItem_setGeometry<()> for (QRectF) {
  fn setGeometry(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setPreferredWidth(qreal width);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredWidth<RetType, T: QGraphicsLayoutItem_setPreferredWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPreferredWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredWidth<RetType> {
  fn setPreferredWidth(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setPreferredWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredWidth<()> for (f64) {
  fn setPreferredWidth(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem17setPreferredWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem17setPreferredWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMaximumSize(const QSizeF & size);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumSize<RetType, T: QGraphicsLayoutItem_setMaximumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMaximumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumSize<RetType> {
  fn setMaximumSize(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMaximumSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumSize<()> for (QSizeF) {
  fn setMaximumSize(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem14setMaximumSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsLayoutItem::maximumWidth();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn maximumWidth<RetType, T: QGraphicsLayoutItem_maximumWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_maximumWidth<RetType> {
  fn maximumWidth(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  qreal QGraphicsLayoutItem::maximumWidth();
impl<'a> /*trait*/ QGraphicsLayoutItem_maximumWidth<f64> for () {
  fn maximumWidth(self , rsthis: &mut QGraphicsLayoutItem) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12maximumWidthEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem12maximumWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMinimumSize(qreal w, qreal h);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumSize<RetType, T: QGraphicsLayoutItem_setMinimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMinimumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumSize<RetType> {
  fn setMinimumSize(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMinimumSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumSize<()> for (f64, f64) {
  fn setMinimumSize(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMinimumSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem14setMinimumSizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMaximumHeight(qreal height);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumHeight<RetType, T: QGraphicsLayoutItem_setMaximumHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMaximumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumHeight<RetType> {
  fn setMaximumHeight(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMaximumHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumHeight<()> for (f64) {
  fn setMaximumHeight(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setMaximumHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem16setMaximumHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMinimumSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumSize<()> for (QSizeF) {
  fn setMinimumSize(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem14setMinimumSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setPreferredSize(const QSizeF & size);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredSize<RetType, T: QGraphicsLayoutItem_setPreferredSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPreferredSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredSize<RetType> {
  fn setPreferredSize(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setPreferredSize(const QSizeF & size);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredSize<()> for (QSizeF) {
  fn setPreferredSize(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem16setPreferredSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn getContentsMargins<RetType, T: QGraphicsLayoutItem_getContentsMargins<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.getContentsMargins(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_getContentsMargins<RetType> {
  fn getContentsMargins(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::getContentsMargins(qreal * left, qreal * top, qreal * right, qreal * bottom);
impl<'a> /*trait*/ QGraphicsLayoutItem_getContentsMargins<()> for (&'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>, &'a mut Vec<f64>) {
  fn getContentsMargins(self , rsthis: &mut QGraphicsLayoutItem) -> () {
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
  pub fn setParentLayoutItem<RetType, T: QGraphicsLayoutItem_setParentLayoutItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setParentLayoutItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setParentLayoutItem<RetType> {
  fn setParentLayoutItem(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setParentLayoutItem(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsLayoutItem_setParentLayoutItem<()> for (QGraphicsLayoutItem) {
  fn setParentLayoutItem(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsLayoutItem19setParentLayoutItemEPS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMinimumWidth(qreal width);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumWidth<RetType, T: QGraphicsLayoutItem_setMinimumWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMinimumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumWidth<RetType> {
  fn setMinimumWidth(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMinimumWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumWidth<()> for (f64) {
  fn setMinimumWidth(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem15setMinimumWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem15setMinimumWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMaximumWidth(qreal width);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMaximumWidth<RetType, T: QGraphicsLayoutItem_setMaximumWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMaximumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMaximumWidth<RetType> {
  fn setMaximumWidth(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMaximumWidth(qreal width);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumWidth<()> for (f64) {
  fn setMaximumWidth(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem15setMaximumWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem15setMaximumWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::updateGeometry();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn updateGeometry<RetType, T: QGraphicsLayoutItem_updateGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.updateGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_updateGeometry<RetType> {
  fn updateGeometry(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::updateGeometry();
impl<'a> /*trait*/ QGraphicsLayoutItem_updateGeometry<()> for () {
  fn updateGeometry(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14updateGeometryEv()};
     unsafe {_ZN19QGraphicsLayoutItem14updateGeometryEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setPreferredHeight(qreal height);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setPreferredHeight<RetType, T: QGraphicsLayoutItem_setPreferredHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPreferredHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setPreferredHeight<RetType> {
  fn setPreferredHeight(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setPreferredHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredHeight<()> for (f64) {
  fn setPreferredHeight(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem18setPreferredHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem18setPreferredHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSizeF QGraphicsLayoutItem::minimumSize();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn minimumSize<RetType, T: QGraphicsLayoutItem_minimumSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSize(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_minimumSize<RetType> {
  fn minimumSize(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  QSizeF QGraphicsLayoutItem::minimumSize();
impl<'a> /*trait*/ QGraphicsLayoutItem_minimumSize<QSizeF> for () {
  fn minimumSize(self , rsthis: &mut QGraphicsLayoutItem) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem11minimumSizeEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem11minimumSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsLayoutItem::contentsRect();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn contentsRect<RetType, T: QGraphicsLayoutItem_contentsRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contentsRect(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_contentsRect<RetType> {
  fn contentsRect(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  QRectF QGraphicsLayoutItem::contentsRect();
impl<'a> /*trait*/ QGraphicsLayoutItem_contentsRect<QRectF> for () {
  fn contentsRect(self , rsthis: &mut QGraphicsLayoutItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem12contentsRectEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem12contentsRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGraphicsLayoutItem::isLayout();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn isLayout<RetType, T: QGraphicsLayoutItem_isLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isLayout(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_isLayout<RetType> {
  fn isLayout(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  bool QGraphicsLayoutItem::isLayout();
impl<'a> /*trait*/ QGraphicsLayoutItem_isLayout<i8> for () {
  fn isLayout(self , rsthis: &mut QGraphicsLayoutItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsLayoutItem8isLayoutEv()};
    let mut ret = unsafe {_ZNK19QGraphicsLayoutItem8isLayoutEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setPreferredSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setPreferredSize<()> for (f64, f64) {
  fn setPreferredSize(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setPreferredSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem16setPreferredSizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::~QGraphicsLayoutItem();
impl /*struct*/ QGraphicsLayoutItem {
  pub fn FreeQGraphicsLayoutItem<RetType, T: QGraphicsLayoutItem_FreeQGraphicsLayoutItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsLayoutItem(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_FreeQGraphicsLayoutItem<RetType> {
  fn FreeQGraphicsLayoutItem(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::~QGraphicsLayoutItem();
impl<'a> /*trait*/ QGraphicsLayoutItem_FreeQGraphicsLayoutItem<()> for () {
  fn FreeQGraphicsLayoutItem(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItemD0Ev()};
     unsafe {_ZN19QGraphicsLayoutItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMinimumHeight(qreal height);
impl /*struct*/ QGraphicsLayoutItem {
  pub fn setMinimumHeight<RetType, T: QGraphicsLayoutItem_setMinimumHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMinimumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsLayoutItem_setMinimumHeight<RetType> {
  fn setMinimumHeight(self , rsthis: &mut QGraphicsLayoutItem) -> RetType;
}

  // proto:  void QGraphicsLayoutItem::setMinimumHeight(qreal height);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMinimumHeight<()> for (f64) {
  fn setMinimumHeight(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem16setMinimumHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem16setMinimumHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLayoutItem::setMaximumSize(qreal w, qreal h);
impl<'a> /*trait*/ QGraphicsLayoutItem_setMaximumSize<()> for (f64, f64) {
  fn setMaximumSize(self , rsthis: &mut QGraphicsLayoutItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsLayoutItem14setMaximumSizeEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsLayoutItem14setMaximumSizeEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

