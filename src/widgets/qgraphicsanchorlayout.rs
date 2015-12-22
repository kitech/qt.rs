// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
use super::qgraphicslayout::QGraphicsLayout; // 773
use std::ops::Deref;
use super::qgraphicslayoutitem::QGraphicsLayoutItem; // 773
use super::super::core::qrect::QRectF; // 771
use super::super::core::qobject::QObject; // 771
// use super::qgraphicsanchorlayout::QGraphicsAnchorLayout; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QGraphicsAnchorLayout::QGraphicsAnchorLayout(QGraphicsLayoutItem * parent);
  fn _ZN21QGraphicsAnchorLayoutC1EP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsAnchorLayout::QGraphicsAnchorLayout(const QGraphicsAnchorLayout & );
  fn _ZN21QGraphicsAnchorLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QGraphicsAnchorLayout::verticalSpacing();
  fn _ZNK21QGraphicsAnchorLayout15verticalSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsAnchorLayout::setSpacing(qreal spacing);
  fn _ZN21QGraphicsAnchorLayout10setSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  int QGraphicsAnchorLayout::count();
  fn _ZNK21QGraphicsAnchorLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  qreal QGraphicsAnchorLayout::horizontalSpacing();
  fn _ZNK21QGraphicsAnchorLayout17horizontalSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsAnchorLayout::invalidate();
  fn _ZN21QGraphicsAnchorLayout10invalidateEv(qthis: *mut c_void);
  // proto:  QGraphicsLayoutItem * QGraphicsAnchorLayout::itemAt(int index);
  fn _ZNK21QGraphicsAnchorLayout6itemAtEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QGraphicsAnchorLayout::setVerticalSpacing(qreal spacing);
  fn _ZN21QGraphicsAnchorLayout18setVerticalSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsAnchorLayout::setGeometry(const QRectF & rect);
  fn _ZN21QGraphicsAnchorLayout11setGeometryERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsAnchorLayout::setHorizontalSpacing(qreal spacing);
  fn _ZN21QGraphicsAnchorLayout20setHorizontalSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsAnchorLayout::~QGraphicsAnchorLayout();
  fn _ZN21QGraphicsAnchorLayoutD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsAnchorLayout::removeAt(int index);
  fn _ZN21QGraphicsAnchorLayout8removeAtEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QGraphicsAnchor::~QGraphicsAnchor();
  fn _ZN15QGraphicsAnchorD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsAnchor::unsetSpacing();
  fn _ZN15QGraphicsAnchor12unsetSpacingEv(qthis: *mut c_void);
  // proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
  fn _ZN15QGraphicsAnchor10setSpacingEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsAnchor::QGraphicsAnchor(QGraphicsAnchorLayout * parent);
  fn _ZN15QGraphicsAnchorC1EP21QGraphicsAnchorLayout(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QGraphicsAnchor::metaObject();
  fn _ZNK15QGraphicsAnchor10metaObjectEv(qthis: *mut c_void);
  // proto:  qreal QGraphicsAnchor::spacing();
  fn _ZNK15QGraphicsAnchor7spacingEv(qthis: *mut c_void) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsAnchorLayout)=1
pub struct QGraphicsAnchorLayout {
  qbase: QGraphicsLayout,
  pub qclsinst: *mut c_void,
}

// class sizeof(QGraphicsAnchor)=1
pub struct QGraphicsAnchor {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsAnchorLayout {
  pub fn inheritFrom(qthis: *mut c_void) -> QGraphicsAnchorLayout {
    return QGraphicsAnchorLayout{qbase: QGraphicsLayout::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QGraphicsAnchorLayout {
  type Target = QGraphicsLayout;

  fn deref(&self) -> &QGraphicsLayout {
    return &self.qbase;
  }
}
impl AsRef<QGraphicsLayout> for QGraphicsAnchorLayout {
  fn as_ref(&self) -> &QGraphicsLayout {
    return &self.qbase;
  }
}
  // proto:  void QGraphicsAnchorLayout::QGraphicsAnchorLayout(QGraphicsLayoutItem * parent);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn NewQGraphicsAnchorLayout<T: QGraphicsAnchorLayout_NewQGraphicsAnchorLayout>(value: T) -> QGraphicsAnchorLayout {
    let rsthis = value.NewQGraphicsAnchorLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_NewQGraphicsAnchorLayout {
  fn NewQGraphicsAnchorLayout(self) -> QGraphicsAnchorLayout;
}

  // proto:  void QGraphicsAnchorLayout::QGraphicsAnchorLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsAnchorLayout_NewQGraphicsAnchorLayout for (QGraphicsLayoutItem) {
  fn NewQGraphicsAnchorLayout(self) -> QGraphicsAnchorLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayoutC1EP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QGraphicsAnchorLayoutC1EP19QGraphicsLayoutItem(qthis, arg0)};
    let rsthis = QGraphicsAnchorLayout{/**/qbase: QGraphicsLayout::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::QGraphicsAnchorLayout(const QGraphicsAnchorLayout & );
impl<'a> /*trait*/ QGraphicsAnchorLayout_NewQGraphicsAnchorLayout for (QGraphicsAnchorLayout) {
  fn NewQGraphicsAnchorLayout(self) -> QGraphicsAnchorLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QGraphicsAnchorLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsAnchorLayout{/**/qbase: QGraphicsLayout::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QGraphicsAnchorLayout::verticalSpacing();
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn verticalSpacing<RetType, T: QGraphicsAnchorLayout_verticalSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.verticalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_verticalSpacing<RetType> {
  fn verticalSpacing(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  qreal QGraphicsAnchorLayout::verticalSpacing();
impl<'a> /*trait*/ QGraphicsAnchorLayout_verticalSpacing<f64> for () {
  fn verticalSpacing(self , rsthis: &mut QGraphicsAnchorLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout15verticalSpacingEv()};
    let mut ret = unsafe {_ZNK21QGraphicsAnchorLayout15verticalSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::setSpacing(qreal spacing);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setSpacing<RetType, T: QGraphicsAnchorLayout_setSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_setSpacing<RetType> {
  fn setSpacing(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setSpacing<()> for (f64) {
  fn setSpacing(self , rsthis: &mut QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout10setSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN21QGraphicsAnchorLayout10setSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGraphicsAnchorLayout::count();
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn count<RetType, T: QGraphicsAnchorLayout_count<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_count<RetType> {
  fn count(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  int QGraphicsAnchorLayout::count();
impl<'a> /*trait*/ QGraphicsAnchorLayout_count<i32> for () {
  fn count(self , rsthis: &mut QGraphicsAnchorLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout5countEv()};
    let mut ret = unsafe {_ZNK21QGraphicsAnchorLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  qreal QGraphicsAnchorLayout::horizontalSpacing();
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn horizontalSpacing<RetType, T: QGraphicsAnchorLayout_horizontalSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.horizontalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_horizontalSpacing<RetType> {
  fn horizontalSpacing(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  qreal QGraphicsAnchorLayout::horizontalSpacing();
impl<'a> /*trait*/ QGraphicsAnchorLayout_horizontalSpacing<f64> for () {
  fn horizontalSpacing(self , rsthis: &mut QGraphicsAnchorLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout17horizontalSpacingEv()};
    let mut ret = unsafe {_ZNK21QGraphicsAnchorLayout17horizontalSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::invalidate();
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn invalidate<RetType, T: QGraphicsAnchorLayout_invalidate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_invalidate<RetType> {
  fn invalidate(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::invalidate();
impl<'a> /*trait*/ QGraphicsAnchorLayout_invalidate<()> for () {
  fn invalidate(self , rsthis: &mut QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout10invalidateEv()};
     unsafe {_ZN21QGraphicsAnchorLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QGraphicsLayoutItem * QGraphicsAnchorLayout::itemAt(int index);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn itemAt<RetType, T: QGraphicsAnchorLayout_itemAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  QGraphicsLayoutItem * QGraphicsAnchorLayout::itemAt(int index);
impl<'a> /*trait*/ QGraphicsAnchorLayout_itemAt<()> for (i32) {
  fn itemAt(self , rsthis: &mut QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsAnchorLayout6itemAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK21QGraphicsAnchorLayout6itemAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::setVerticalSpacing(qreal spacing);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setVerticalSpacing<RetType, T: QGraphicsAnchorLayout_setVerticalSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setVerticalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_setVerticalSpacing<RetType> {
  fn setVerticalSpacing(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::setVerticalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setVerticalSpacing<()> for (f64) {
  fn setVerticalSpacing(self , rsthis: &mut QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout18setVerticalSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN21QGraphicsAnchorLayout18setVerticalSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::setGeometry(const QRectF & rect);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setGeometry<RetType, T: QGraphicsAnchorLayout_setGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_setGeometry<RetType> {
  fn setGeometry(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setGeometry<()> for (QRectF) {
  fn setGeometry(self , rsthis: &mut QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QGraphicsAnchorLayout11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::setHorizontalSpacing(qreal spacing);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn setHorizontalSpacing<RetType, T: QGraphicsAnchorLayout_setHorizontalSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHorizontalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_setHorizontalSpacing<RetType> {
  fn setHorizontalSpacing(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::setHorizontalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchorLayout_setHorizontalSpacing<()> for (f64) {
  fn setHorizontalSpacing(self , rsthis: &mut QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout20setHorizontalSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN21QGraphicsAnchorLayout20setHorizontalSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::~QGraphicsAnchorLayout();
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn FreeQGraphicsAnchorLayout<RetType, T: QGraphicsAnchorLayout_FreeQGraphicsAnchorLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsAnchorLayout(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_FreeQGraphicsAnchorLayout<RetType> {
  fn FreeQGraphicsAnchorLayout(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::~QGraphicsAnchorLayout();
impl<'a> /*trait*/ QGraphicsAnchorLayout_FreeQGraphicsAnchorLayout<()> for () {
  fn FreeQGraphicsAnchorLayout(self , rsthis: &mut QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayoutD0Ev()};
     unsafe {_ZN21QGraphicsAnchorLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchorLayout::removeAt(int index);
impl /*struct*/ QGraphicsAnchorLayout {
  pub fn removeAt<RetType, T: QGraphicsAnchorLayout_removeAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeAt(self);
    // return 1;
  }
}

pub trait QGraphicsAnchorLayout_removeAt<RetType> {
  fn removeAt(self , rsthis: &mut QGraphicsAnchorLayout) -> RetType;
}

  // proto:  void QGraphicsAnchorLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsAnchorLayout_removeAt<()> for (i32) {
  fn removeAt(self , rsthis: &mut QGraphicsAnchorLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsAnchorLayout8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN21QGraphicsAnchorLayout8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsAnchor {
  pub fn inheritFrom(qthis: *mut c_void) -> QGraphicsAnchor {
    return QGraphicsAnchor{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QGraphicsAnchor {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return &self.qbase;
  }
}
impl AsRef<QObject> for QGraphicsAnchor {
  fn as_ref(&self) -> &QObject {
    return &self.qbase;
  }
}
  // proto:  void QGraphicsAnchor::~QGraphicsAnchor();
impl /*struct*/ QGraphicsAnchor {
  pub fn FreeQGraphicsAnchor<RetType, T: QGraphicsAnchor_FreeQGraphicsAnchor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsAnchor(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_FreeQGraphicsAnchor<RetType> {
  fn FreeQGraphicsAnchor(self , rsthis: &mut QGraphicsAnchor) -> RetType;
}

  // proto:  void QGraphicsAnchor::~QGraphicsAnchor();
impl<'a> /*trait*/ QGraphicsAnchor_FreeQGraphicsAnchor<()> for () {
  fn FreeQGraphicsAnchor(self , rsthis: &mut QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchorD0Ev()};
     unsafe {_ZN15QGraphicsAnchorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchor::unsetSpacing();
impl /*struct*/ QGraphicsAnchor {
  pub fn unsetSpacing<RetType, T: QGraphicsAnchor_unsetSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unsetSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_unsetSpacing<RetType> {
  fn unsetSpacing(self , rsthis: &mut QGraphicsAnchor) -> RetType;
}

  // proto:  void QGraphicsAnchor::unsetSpacing();
impl<'a> /*trait*/ QGraphicsAnchor_unsetSpacing<()> for () {
  fn unsetSpacing(self , rsthis: &mut QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchor12unsetSpacingEv()};
     unsafe {_ZN15QGraphicsAnchor12unsetSpacingEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
impl /*struct*/ QGraphicsAnchor {
  pub fn setSpacing<RetType, T: QGraphicsAnchor_setSpacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_setSpacing<RetType> {
  fn setSpacing(self , rsthis: &mut QGraphicsAnchor) -> RetType;
}

  // proto:  void QGraphicsAnchor::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsAnchor_setSpacing<()> for (f64) {
  fn setSpacing(self , rsthis: &mut QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchor10setSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN15QGraphicsAnchor10setSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsAnchor::QGraphicsAnchor(QGraphicsAnchorLayout * parent);
impl /*struct*/ QGraphicsAnchor {
  pub fn NewQGraphicsAnchor<T: QGraphicsAnchor_NewQGraphicsAnchor>(value: T) -> QGraphicsAnchor {
    let rsthis = value.NewQGraphicsAnchor();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsAnchor_NewQGraphicsAnchor {
  fn NewQGraphicsAnchor(self) -> QGraphicsAnchor;
}

  // proto:  void QGraphicsAnchor::QGraphicsAnchor(QGraphicsAnchorLayout * parent);
impl<'a> /*trait*/ QGraphicsAnchor_NewQGraphicsAnchor for (QGraphicsAnchorLayout) {
  fn NewQGraphicsAnchor(self) -> QGraphicsAnchor {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsAnchorC1EP21QGraphicsAnchorLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsAnchorC1EP21QGraphicsAnchorLayout(qthis, arg0)};
    let rsthis = QGraphicsAnchor{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsAnchor::metaObject();
impl /*struct*/ QGraphicsAnchor {
  pub fn metaObject<RetType, T: QGraphicsAnchor_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsAnchor) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsAnchor::metaObject();
impl<'a> /*trait*/ QGraphicsAnchor_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsAnchor) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsAnchor10metaObjectEv()};
     unsafe {_ZNK15QGraphicsAnchor10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsAnchor::spacing();
impl /*struct*/ QGraphicsAnchor {
  pub fn spacing<RetType, T: QGraphicsAnchor_spacing<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.spacing(self);
    // return 1;
  }
}

pub trait QGraphicsAnchor_spacing<RetType> {
  fn spacing(self , rsthis: &mut QGraphicsAnchor) -> RetType;
}

  // proto:  qreal QGraphicsAnchor::spacing();
impl<'a> /*trait*/ QGraphicsAnchor_spacing<f64> for () {
  fn spacing(self , rsthis: &mut QGraphicsAnchor) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsAnchor7spacingEv()};
    let mut ret = unsafe {_ZNK15QGraphicsAnchor7spacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// <= body block end

