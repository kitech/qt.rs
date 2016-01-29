// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qgraphicsitemanimation.h
// dst-file: /src/widgets/qgraphicsitemanimation.rs
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::super::core::qpoint::*; // 771
// use super::qlist::*; // 775
use super::super::gui::qmatrix::*; // 771
use super::qgraphicsitem::*; // 773
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qtimeline::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsItemAnimation_Class_Size() -> c_int;
  // proto:  void QGraphicsItemAnimation::setPosAt(qreal step, const QPointF & pos);
  fn C_ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: *mut c_void);
  // proto:  qreal QGraphicsItemAnimation::xTranslationAt(qreal step);
  fn C_ZNK22QGraphicsItemAnimation14xTranslationAtEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_double;
  // proto:  void QGraphicsItemAnimation::setRotationAt(qreal step, qreal angle);
  fn C_ZN22QGraphicsItemAnimation13setRotationAtEdd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double);
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::posList();
  fn C_ZNK22QGraphicsItemAnimation7posListEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QGraphicsItemAnimation::verticalScaleAt(qreal step);
  fn C_ZNK22QGraphicsItemAnimation15verticalScaleAtEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_double;
  // proto:  QPointF QGraphicsItemAnimation::posAt(qreal step);
  fn C_ZNK22QGraphicsItemAnimation5posAtEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> *mut c_void;
  // proto:  qreal QGraphicsItemAnimation::horizontalShearAt(qreal step);
  fn C_ZNK22QGraphicsItemAnimation17horizontalShearAtEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_double;
  // proto:  qreal QGraphicsItemAnimation::yTranslationAt(qreal step);
  fn C_ZNK22QGraphicsItemAnimation14yTranslationAtEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_double;
  // proto:  QMatrix QGraphicsItemAnimation::matrixAt(qreal step);
  fn C_ZNK22QGraphicsItemAnimation8matrixAtEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItemAnimation::item();
  fn C_ZNK22QGraphicsItemAnimation4itemEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsItemAnimation::QGraphicsItemAnimation(QObject * parent);
  fn C_ZN22QGraphicsItemAnimationC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QGraphicsItemAnimation::~QGraphicsItemAnimation();
  fn C_ZN22QGraphicsItemAnimationD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItemAnimation::setScaleAt(qreal step, qreal sx, qreal sy);
  fn C_ZN22QGraphicsItemAnimation10setScaleAtEddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double);
  // proto:  void QGraphicsItemAnimation::setTranslationAt(qreal step, qreal dx, qreal dy);
  fn C_ZN22QGraphicsItemAnimation16setTranslationAtEddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double);
  // proto:  void QGraphicsItemAnimation::setShearAt(qreal step, qreal sh, qreal sv);
  fn C_ZN22QGraphicsItemAnimation10setShearAtEddd(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_double, arg2: c_double);
  // proto:  qreal QGraphicsItemAnimation::rotationAt(qreal step);
  fn C_ZNK22QGraphicsItemAnimation10rotationAtEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_double;
  // proto:  const QMetaObject * QGraphicsItemAnimation::metaObject();
  fn C_ZNK22QGraphicsItemAnimation10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::scaleList();
  fn C_ZNK22QGraphicsItemAnimation9scaleListEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QPair<qreal, qreal> > QGraphicsItemAnimation::rotationList();
  fn C_ZNK22QGraphicsItemAnimation12rotationListEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsItemAnimation::reset();
  fn C_ZN22QGraphicsItemAnimation5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsItemAnimation::setTimeLine(QTimeLine * timeLine);
  fn C_ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::shearList();
  fn C_ZNK22QGraphicsItemAnimation9shearListEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsItemAnimation::clear();
  fn C_ZN22QGraphicsItemAnimation5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::translationList();
  fn C_ZNK22QGraphicsItemAnimation15translationListEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGraphicsItemAnimation::setItem(QGraphicsItem * item);
  fn C_ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsItemAnimation::setStep(qreal x);
  fn C_ZN22QGraphicsItemAnimation7setStepEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QTimeLine * QGraphicsItemAnimation::timeLine();
  fn C_ZNK22QGraphicsItemAnimation8timeLineEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QGraphicsItemAnimation::horizontalScaleAt(qreal step);
  fn C_ZNK22QGraphicsItemAnimation17horizontalScaleAtEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_double;
  // proto:  qreal QGraphicsItemAnimation::verticalShearAt(qreal step);
  fn C_ZNK22QGraphicsItemAnimation15verticalShearAtEd(qthis: u64 /* *mut c_void*/, arg0: c_double) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsItemAnimation)=1
#[derive(Default)]
pub struct QGraphicsItemAnimation {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsItemAnimation {
    return QGraphicsItemAnimation{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsItemAnimation {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QGraphicsItemAnimation {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QGraphicsItemAnimation::setPosAt(qreal step, const QPointF & pos);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setPosAt<RetType, T: QGraphicsItemAnimation_setPosAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPosAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setPosAt<RetType> {
  fn setPosAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::setPosAt(qreal step, const QPointF & pos);
impl<'a> /*trait*/ QGraphicsItemAnimation_setPosAt<()> for (f64, &'a QPointF) {
  fn setPosAt(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsItemAnimation::xTranslationAt(qreal step);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn xTranslationAt<RetType, T: QGraphicsItemAnimation_xTranslationAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.xTranslationAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_xTranslationAt<RetType> {
  fn xTranslationAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  qreal QGraphicsItemAnimation::xTranslationAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_xTranslationAt<f64> for (f64) {
  fn xTranslationAt(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation14xTranslationAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation14xTranslationAtEd(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::setRotationAt(qreal step, qreal angle);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setRotationAt<RetType, T: QGraphicsItemAnimation_setRotationAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRotationAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setRotationAt<RetType> {
  fn setRotationAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::setRotationAt(qreal step, qreal angle);
impl<'a> /*trait*/ QGraphicsItemAnimation_setRotationAt<()> for (f64, f64) {
  fn setRotationAt(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation13setRotationAtEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN22QGraphicsItemAnimation13setRotationAtEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::posList();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn posList<RetType, T: QGraphicsItemAnimation_posList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.posList(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_posList<RetType> {
  fn posList(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::posList();
impl<'a> /*trait*/ QGraphicsItemAnimation_posList<u64> for () {
  fn posList(self , rsthis: & QGraphicsItemAnimation) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation7posListEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation7posListEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  qreal QGraphicsItemAnimation::verticalScaleAt(qreal step);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn verticalScaleAt<RetType, T: QGraphicsItemAnimation_verticalScaleAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalScaleAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_verticalScaleAt<RetType> {
  fn verticalScaleAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  qreal QGraphicsItemAnimation::verticalScaleAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_verticalScaleAt<f64> for (f64) {
  fn verticalScaleAt(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation15verticalScaleAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation15verticalScaleAtEd(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QPointF QGraphicsItemAnimation::posAt(qreal step);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn posAt<RetType, T: QGraphicsItemAnimation_posAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.posAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_posAt<RetType> {
  fn posAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  QPointF QGraphicsItemAnimation::posAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_posAt<QPointF> for (f64) {
  fn posAt(self , rsthis: & QGraphicsItemAnimation) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation5posAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation5posAtEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItemAnimation::horizontalShearAt(qreal step);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn horizontalShearAt<RetType, T: QGraphicsItemAnimation_horizontalShearAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalShearAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_horizontalShearAt<RetType> {
  fn horizontalShearAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  qreal QGraphicsItemAnimation::horizontalShearAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_horizontalShearAt<f64> for (f64) {
  fn horizontalShearAt(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation17horizontalShearAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation17horizontalShearAtEd(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QGraphicsItemAnimation::yTranslationAt(qreal step);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn yTranslationAt<RetType, T: QGraphicsItemAnimation_yTranslationAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.yTranslationAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_yTranslationAt<RetType> {
  fn yTranslationAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  qreal QGraphicsItemAnimation::yTranslationAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_yTranslationAt<f64> for (f64) {
  fn yTranslationAt(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation14yTranslationAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation14yTranslationAtEd(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QMatrix QGraphicsItemAnimation::matrixAt(qreal step);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn matrixAt<RetType, T: QGraphicsItemAnimation_matrixAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.matrixAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_matrixAt<RetType> {
  fn matrixAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  QMatrix QGraphicsItemAnimation::matrixAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_matrixAt<QMatrix> for (f64) {
  fn matrixAt(self , rsthis: & QGraphicsItemAnimation) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation8matrixAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation8matrixAtEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QMatrix::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsItem * QGraphicsItemAnimation::item();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn item<RetType, T: QGraphicsItemAnimation_item<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.item(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_item<RetType> {
  fn item(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  QGraphicsItem * QGraphicsItemAnimation::item();
impl<'a> /*trait*/ QGraphicsItemAnimation_item<QGraphicsItem> for () {
  fn item(self , rsthis: & QGraphicsItemAnimation) -> QGraphicsItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation4itemEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation4itemEv(rsthis.qclsinst)};
    let mut ret1 = QGraphicsItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::QGraphicsItemAnimation(QObject * parent);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn new<T: QGraphicsItemAnimation_new>(value: T) -> QGraphicsItemAnimation {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_new {
  fn new(self) -> QGraphicsItemAnimation;
}

  // proto:  void QGraphicsItemAnimation::QGraphicsItemAnimation(QObject * parent);
impl<'a> /*trait*/ QGraphicsItemAnimation_new for (&'a QObject) {
  fn new(self) -> QGraphicsItemAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimationC2EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsItemAnimation_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN22QGraphicsItemAnimationC2EP7QObject(arg0)};
    let rsthis = QGraphicsItemAnimation{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::~QGraphicsItemAnimation();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn free<RetType, T: QGraphicsItemAnimation_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_free<RetType> {
  fn free(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::~QGraphicsItemAnimation();
impl<'a> /*trait*/ QGraphicsItemAnimation_free<()> for () {
  fn free(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimationD2Ev()};
     unsafe {C_ZN22QGraphicsItemAnimationD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::setScaleAt(qreal step, qreal sx, qreal sy);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setScaleAt<RetType, T: QGraphicsItemAnimation_setScaleAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScaleAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setScaleAt<RetType> {
  fn setScaleAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::setScaleAt(qreal step, qreal sx, qreal sy);
impl<'a> /*trait*/ QGraphicsItemAnimation_setScaleAt<()> for (f64, f64, f64) {
  fn setScaleAt(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation10setScaleAtEddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {C_ZN22QGraphicsItemAnimation10setScaleAtEddd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::setTranslationAt(qreal step, qreal dx, qreal dy);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setTranslationAt<RetType, T: QGraphicsItemAnimation_setTranslationAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTranslationAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setTranslationAt<RetType> {
  fn setTranslationAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::setTranslationAt(qreal step, qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsItemAnimation_setTranslationAt<()> for (f64, f64, f64) {
  fn setTranslationAt(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation16setTranslationAtEddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {C_ZN22QGraphicsItemAnimation16setTranslationAtEddd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::setShearAt(qreal step, qreal sh, qreal sv);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setShearAt<RetType, T: QGraphicsItemAnimation_setShearAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setShearAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setShearAt<RetType> {
  fn setShearAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::setShearAt(qreal step, qreal sh, qreal sv);
impl<'a> /*trait*/ QGraphicsItemAnimation_setShearAt<()> for (f64, f64, f64) {
  fn setShearAt(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation10setShearAtEddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {C_ZN22QGraphicsItemAnimation10setShearAtEddd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsItemAnimation::rotationAt(qreal step);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn rotationAt<RetType, T: QGraphicsItemAnimation_rotationAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotationAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_rotationAt<RetType> {
  fn rotationAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  qreal QGraphicsItemAnimation::rotationAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_rotationAt<f64> for (f64) {
  fn rotationAt(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation10rotationAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation10rotationAtEd(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsItemAnimation::metaObject();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn metaObject<RetType, T: QGraphicsItemAnimation_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsItemAnimation::metaObject();
impl<'a> /*trait*/ QGraphicsItemAnimation_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGraphicsItemAnimation) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation10metaObjectEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::scaleList();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn scaleList<RetType, T: QGraphicsItemAnimation_scaleList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scaleList(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_scaleList<RetType> {
  fn scaleList(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::scaleList();
impl<'a> /*trait*/ QGraphicsItemAnimation_scaleList<u64> for () {
  fn scaleList(self , rsthis: & QGraphicsItemAnimation) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation9scaleListEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation9scaleListEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QList<QPair<qreal, qreal> > QGraphicsItemAnimation::rotationList();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn rotationList<RetType, T: QGraphicsItemAnimation_rotationList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotationList(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_rotationList<RetType> {
  fn rotationList(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  QList<QPair<qreal, qreal> > QGraphicsItemAnimation::rotationList();
impl<'a> /*trait*/ QGraphicsItemAnimation_rotationList<u64> for () {
  fn rotationList(self , rsthis: & QGraphicsItemAnimation) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation12rotationListEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation12rotationListEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::reset();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn reset<RetType, T: QGraphicsItemAnimation_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_reset<RetType> {
  fn reset(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::reset();
impl<'a> /*trait*/ QGraphicsItemAnimation_reset<()> for () {
  fn reset(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation5resetEv()};
     unsafe {C_ZN22QGraphicsItemAnimation5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::setTimeLine(QTimeLine * timeLine);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setTimeLine<RetType, T: QGraphicsItemAnimation_setTimeLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTimeLine(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setTimeLine<RetType> {
  fn setTimeLine(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::setTimeLine(QTimeLine * timeLine);
impl<'a> /*trait*/ QGraphicsItemAnimation_setTimeLine<()> for (&'a QTimeLine) {
  fn setTimeLine(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::shearList();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn shearList<RetType, T: QGraphicsItemAnimation_shearList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shearList(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_shearList<RetType> {
  fn shearList(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::shearList();
impl<'a> /*trait*/ QGraphicsItemAnimation_shearList<u64> for () {
  fn shearList(self , rsthis: & QGraphicsItemAnimation) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation9shearListEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation9shearListEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::clear();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn clear<RetType, T: QGraphicsItemAnimation_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_clear<RetType> {
  fn clear(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::clear();
impl<'a> /*trait*/ QGraphicsItemAnimation_clear<()> for () {
  fn clear(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation5clearEv()};
     unsafe {C_ZN22QGraphicsItemAnimation5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::translationList();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn translationList<RetType, T: QGraphicsItemAnimation_translationList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translationList(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_translationList<RetType> {
  fn translationList(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::translationList();
impl<'a> /*trait*/ QGraphicsItemAnimation_translationList<u64> for () {
  fn translationList(self , rsthis: & QGraphicsItemAnimation) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation15translationListEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation15translationListEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::setItem(QGraphicsItem * item);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setItem<RetType, T: QGraphicsItemAnimation_setItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItem(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setItem<RetType> {
  fn setItem(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::setItem(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemAnimation_setItem<()> for (&'a QGraphicsItem) {
  fn setItem(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::setStep(qreal x);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn setStep<RetType, T: QGraphicsItemAnimation_setStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStep(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setStep<RetType> {
  fn setStep(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::setStep(qreal x);
impl<'a> /*trait*/ QGraphicsItemAnimation_setStep<()> for (f64) {
  fn setStep(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation7setStepEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN22QGraphicsItemAnimation7setStepEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTimeLine * QGraphicsItemAnimation::timeLine();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn timeLine<RetType, T: QGraphicsItemAnimation_timeLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.timeLine(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_timeLine<RetType> {
  fn timeLine(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  QTimeLine * QGraphicsItemAnimation::timeLine();
impl<'a> /*trait*/ QGraphicsItemAnimation_timeLine<QTimeLine> for () {
  fn timeLine(self , rsthis: & QGraphicsItemAnimation) -> QTimeLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation8timeLineEv()};
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation8timeLineEv(rsthis.qclsinst)};
    let mut ret1 = QTimeLine::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QGraphicsItemAnimation::horizontalScaleAt(qreal step);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn horizontalScaleAt<RetType, T: QGraphicsItemAnimation_horizontalScaleAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalScaleAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_horizontalScaleAt<RetType> {
  fn horizontalScaleAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  qreal QGraphicsItemAnimation::horizontalScaleAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_horizontalScaleAt<f64> for (f64) {
  fn horizontalScaleAt(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation17horizontalScaleAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation17horizontalScaleAtEd(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QGraphicsItemAnimation::verticalShearAt(qreal step);
impl /*struct*/ QGraphicsItemAnimation {
  pub fn verticalShearAt<RetType, T: QGraphicsItemAnimation_verticalShearAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.verticalShearAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_verticalShearAt<RetType> {
  fn verticalShearAt(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  qreal QGraphicsItemAnimation::verticalShearAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_verticalShearAt<f64> for (f64) {
  fn verticalShearAt(self , rsthis: & QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation15verticalShearAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {C_ZNK22QGraphicsItemAnimation15verticalShearAtEd(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

// <= body block end

