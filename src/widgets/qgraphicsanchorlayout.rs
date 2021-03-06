// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qgraphicsanchorlayout.h
// dst-file: /src/widgets/qgraphicsanchorlayout.rs
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
use super::qgraphicslayout::*; // 773
use std::ops::Deref;
use super::qgraphicslayoutitem::*; // 773
use super::super::core::qrect::*; // 771
use super::super::core::qobject::*; // 771
use super::super::core::qobjectdefs::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsAnchorLayout_Class_Size() -> c_int;
  // proto:  void QGraphicsAnchorLayout::QGraphicsAnchorLayout(QGraphicsLayoutItem * parent);
  fn C_ZN21QGraphicsAnchorLayoutC2EP19QGraphicsLayoutItem(arg0: *mut c_void) -> u64;
  // proto:  qreal QGraphicsAnchorLayout::verticalSpacing();
  fn C_ZNK21QGraphicsAnchorLayout15verticalSpacingEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsAnchorLayout::setSpacing(qreal spacing);
  fn C_ZN21QGraphicsAnchorLayout10setSpacingEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  int QGraphicsAnchorLayout::count();
  fn C_ZNK21QGraphicsAnchorLayout5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  qreal QGraphicsAnchorLayout::horizontalSpacing();
  fn C_ZNK21QGraphicsAnchorLayout17horizontalSpacingEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsAnchorLayout::invalidate();
  fn C_ZN21QGraphicsAnchorLayout10invalidateEv(qthis: u64 /* *mut c_void*/);
  // proto:  QGraphicsLayoutItem * QGraphicsAnchorLayout::itemAt(int index);
  fn C_ZNK21QGraphicsAnchorLayout6itemAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QGraphicsAnchorLayout::setVerticalSpacing(qreal spacing);
  fn C_ZN21QGraphicsAnchorLayout18setVerticalSpacingEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsAnchorLayout::setGeometry(const QRectF & rect);
  fn C_ZN21QGraphicsAnchorLayout11setGeometryERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsAnchorLayout::setHorizontalSpacing(qreal spacing);
  fn C_ZN21QGraphicsAnchorLayout20setHorizontalSpacingEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsAnchorLayout::~QGraphicsAnchorLayout();
  fn C_ZN21QGraphicsAnchorLayoutD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsAnchorLayout::removeAt(int index);
  fn C_ZN21QGraphicsAnchorLayout8removeAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QGraphicsAnchor_Class_Size() -> c_int;
  // proto:  void QGraphicsAnchor::~QGraphicsAnchor();
  fn C_ZN15QGraphicsAnchorD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsAnchor::unsetSpacing();
  fn C_ZN15QGraphicsAnchor12unsetSpacingEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
  fn C_ZN15QGraphicsAnchor10setSpacingEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  const QMetaObject * QGraphicsAnchor::metaObject();
  fn C_ZNK15QGraphicsAnchor10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QGraphicsAnchor::spacing();
  fn C_ZNK15QGraphicsAnchor7spacingEv(qthis: u64 /* *mut c_void*/) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsAnchorLayout)=1
#[derive(Default)]
pub struct QGraphicsAnchorLayout {
  qbase: QGraphicsLayout,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGraphicsAnchor)=1
#[derive(Default)]
pub struct QGraphicsAnchor {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsAnchorLayout {
    return QGraphicsAnchorLayout{qbase: QGraphicsLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsAnchorLayout {
  type Target = QGraphicsLayout;

  fn deref(&self) -> &QGraphicsLayout {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsLayout> for QGraphicsAnchorLayout {
  fn as_ref(& self) -> & QGraphicsLayout {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsAnchorLayout::QGraphicsAnchorLayout(QGraphicsLayoutItem * parent);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn new<T: QGraphicsAnchorLayout_new>(value: T) -> QGraphicsAnchorLayout {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_new {
  fn new(self) -> QGraphicsAnchorLayout;
}

  // proto:  void QGraphicsAnchorLayout::QGraphicsAnchorLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsAnchorLayout_new for (Option<&'a QGraphicsLayoutItem>) {
  fn new(self) -> QGraphicsAnchorLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayoutC2EP19QGraphicsLayoutItem()};
    let ctysz: c_int = unsafe{QGraphicsAnchorLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QGraphicsAnchorLayoutC2EP19QGraphicsLayoutItem(arg0)};
    let rsthis = QGraphicsAnchorLayout{qbase: QGraphicsLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QGraphicsAnchorLayout::verticalSpacing();
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn verticalSpacing<RetType, T: QGraphicsAnchorLayout_verticalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_verticalSpacing<RetType> {
  fn verticalSpacing(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  qreal QGraphicsAnchorLayout::verticalSpacing();
impl<'a> /*trait*/ QGraphicsAnchorLayout_verticalSpacing<f64> for () {
  fn verticalSpacing(self , rsthis: & QGraphicsAnchorLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout15verticalSpacingEv()};
    let mut ret = unsafe {C_ZNK21QGraphicsAnchorLayout15verticalSpacingEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::setSpacing(qreal spacing);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setSpacing<RetType, T: QGraphicsAnchorLayout_setSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_setSpacing<RetType> {
  fn setSpacing(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setSpacing<()> for (f64) {
  fn setSpacing(self , rsthis: & QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout10setSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN21QGraphicsAnchorLayout10setSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGraphicsAnchorLayout::count();
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn count<RetType, T: QGraphicsAnchorLayout_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_count<RetType> {
  fn count(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  int QGraphicsAnchorLayout::count();
impl<'a> /*trait*/ QGraphicsAnchorLayout_count<i32> for () {
  fn count(self , rsthis: & QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout5countEv()};
    let mut ret = unsafe {C_ZNK21QGraphicsAnchorLayout5countEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  qreal QGraphicsAnchorLayout::horizontalSpacing();
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn horizontalSpacing<RetType, T: QGraphicsAnchorLayout_horizontalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_horizontalSpacing<RetType> {
  fn horizontalSpacing(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  qreal QGraphicsAnchorLayout::horizontalSpacing();
impl<'a> /*trait*/ QGraphicsAnchorLayout_horizontalSpacing<f64> for () {
  fn horizontalSpacing(self , rsthis: & QGraphicsAnchorLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout17horizontalSpacingEv()};
    let mut ret = unsafe {C_ZNK21QGraphicsAnchorLayout17horizontalSpacingEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::invalidate();
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn invalidate<RetType, T: QGraphicsAnchorLayout_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_invalidate<RetType> {
  fn invalidate(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::invalidate();
impl<'a> /*trait*/ QGraphicsAnchorLayout_invalidate<()> for () {
  fn invalidate(self , rsthis: & QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout10invalidateEv()};
     unsafe {C_ZN21QGraphicsAnchorLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QGraphicsLayoutItem * QGraphicsAnchorLayout::itemAt(int index);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn itemAt<RetType, T: QGraphicsAnchorLayout_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  QGraphicsLayoutItem * QGraphicsAnchorLayout::itemAt(int index);
impl<'a> /*trait*/ QGraphicsAnchorLayout_itemAt<QGraphicsLayoutItem> for (i32) {
  fn itemAt(self , rsthis: & QGraphicsAnchorLayout) -> QGraphicsLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK21QGraphicsAnchorLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QGraphicsLayoutItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::setVerticalSpacing(qreal spacing);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setVerticalSpacing<RetType, T: QGraphicsAnchorLayout_setVerticalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVerticalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_setVerticalSpacing<RetType> {
  fn setVerticalSpacing(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::setVerticalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setVerticalSpacing<()> for (f64) {
  fn setVerticalSpacing(self , rsthis: & QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout18setVerticalSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN21QGraphicsAnchorLayout18setVerticalSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::setGeometry(const QRectF & rect);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setGeometry<RetType, T: QGraphicsAnchorLayout_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setGeometry<()> for (&'a QRectF) {
  fn setGeometry(self , rsthis: & QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QGraphicsAnchorLayout11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::setHorizontalSpacing(qreal spacing);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setHorizontalSpacing<RetType, T: QGraphicsAnchorLayout_setHorizontalSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_setHorizontalSpacing<RetType> {
  fn setHorizontalSpacing(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::setHorizontalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setHorizontalSpacing<()> for (f64) {
  fn setHorizontalSpacing(self , rsthis: & QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout20setHorizontalSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN21QGraphicsAnchorLayout20setHorizontalSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::~QGraphicsAnchorLayout();
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn free<RetType, T: QGraphicsAnchorLayout_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_free<RetType> {
  fn free(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::~QGraphicsAnchorLayout();
impl<'a> /*trait*/ QGraphicsAnchorLayout_free<()> for () {
  fn free(self , rsthis: & QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayoutD2Ev()};
     unsafe {C_ZN21QGraphicsAnchorLayoutD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::removeAt(int index);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn removeAt<RetType, T: QGraphicsAnchorLayout_removeAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAt(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_removeAt<RetType> {
  fn removeAt(self , rsthis: & QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsAnchorLayout_removeAt<()> for (i32) {
  fn removeAt(self , rsthis: & QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN21QGraphicsAnchorLayout8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsAnchor {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsAnchor {
    return QGraphicsAnchor{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsAnchor {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QGraphicsAnchor {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsAnchor::~QGraphicsAnchor();
impl /*struct*/ QGraphicsAnchor {
  pub fn free<RetType, T: QGraphicsAnchor_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_free<RetType> {
  fn free(self , rsthis: & QGraphicsAnchor) -> RetType;
}

  // proto:  void QGraphicsAnchor::~QGraphicsAnchor();
impl<'a> /*trait*/ QGraphicsAnchor_free<()> for () {
  fn free(self , rsthis: & QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchorD2Ev()};
     unsafe {C_ZN15QGraphicsAnchorD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchor::unsetSpacing();
impl /*struct*/ QGraphicsAnchor {
  pub fn unsetSpacing<RetType, T: QGraphicsAnchor_unsetSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_unsetSpacing<RetType> {
  fn unsetSpacing(self , rsthis: & QGraphicsAnchor) -> RetType;
}

  // proto:  void QGraphicsAnchor::unsetSpacing();
impl<'a> /*trait*/ QGraphicsAnchor_unsetSpacing<()> for () {
  fn unsetSpacing(self , rsthis: & QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchor12unsetSpacingEv()};
     unsafe {C_ZN15QGraphicsAnchor12unsetSpacingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
impl /*struct*/ QGraphicsAnchor {
  pub fn setSpacing<RetType, T: QGraphicsAnchor_setSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_setSpacing<RetType> {
  fn setSpacing(self , rsthis: & QGraphicsAnchor) -> RetType;
}

  // proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchor_setSpacing<()> for (f64) {
  fn setSpacing(self , rsthis: & QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchor10setSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN15QGraphicsAnchor10setSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsAnchor::metaObject();
impl /*struct*/ QGraphicsAnchor {
  pub fn metaObject<RetType, T: QGraphicsAnchor_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsAnchor) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsAnchor::metaObject();
impl<'a> /*trait*/ QGraphicsAnchor_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsAnchor) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsAnchor10metaObjectEv()};
    let mut ret = unsafe {C_ZNK15QGraphicsAnchor10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsAnchor::spacing();
impl /*struct*/ QGraphicsAnchor {
  pub fn spacing<RetType, T: QGraphicsAnchor_spacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.spacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_spacing<RetType> {
  fn spacing(self , rsthis: & QGraphicsAnchor) -> RetType;
}

  // proto:  qreal QGraphicsAnchor::spacing();
impl<'a> /*trait*/ QGraphicsAnchor_spacing<f64> for () {
  fn spacing(self , rsthis: & QGraphicsAnchor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsAnchor7spacingEv()};
    let mut ret = unsafe {C_ZNK15QGraphicsAnchor7spacingEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

// <= body block end

