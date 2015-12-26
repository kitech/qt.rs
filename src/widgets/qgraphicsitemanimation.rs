// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qpoint::QPointF; // 771
use super::super::gui::qmatrix::QMatrix; // 771
use super::super::core::qtimeline::QTimeLine; // 771
use super::qgraphicsitem::QGraphicsItem; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsItemAnimation_Class_Size() -> c_int;
  // proto:  void QGraphicsItemAnimation::setPosAt(qreal step, const QPointF & pos);
  fn _ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF(qthis: *mut c_void, arg0: c_double, arg1: *mut c_void);
  // proto:  void QGraphicsItemAnimation::QGraphicsItemAnimation(const QGraphicsItemAnimation & );
  fn dector_ZN22QGraphicsItemAnimationC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN22QGraphicsItemAnimationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QGraphicsItemAnimation::xTranslationAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation14xTranslationAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QGraphicsItemAnimation::setRotationAt(qreal step, qreal angle);
  fn _ZN22QGraphicsItemAnimation13setRotationAtEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::posList();
  fn _ZNK22QGraphicsItemAnimation7posListEv(qthis: *mut c_void);
  // proto:  qreal QGraphicsItemAnimation::verticalScaleAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation15verticalScaleAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  QPointF QGraphicsItemAnimation::posAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation5posAtEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  qreal QGraphicsItemAnimation::horizontalShearAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation17horizontalShearAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  qreal QGraphicsItemAnimation::yTranslationAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation14yTranslationAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  QMatrix QGraphicsItemAnimation::matrixAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation8matrixAtEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItemAnimation::item();
  fn _ZNK22QGraphicsItemAnimation4itemEv(qthis: *mut c_void);
  // proto:  void QGraphicsItemAnimation::QGraphicsItemAnimation(QObject * parent);
  fn dector_ZN22QGraphicsItemAnimationC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN22QGraphicsItemAnimationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsItemAnimation::~QGraphicsItemAnimation();
  fn _ZN22QGraphicsItemAnimationD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsItemAnimation::setScaleAt(qreal step, qreal sx, qreal sy);
  fn _ZN22QGraphicsItemAnimation10setScaleAtEddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double);
  // proto:  void QGraphicsItemAnimation::setTranslationAt(qreal step, qreal dx, qreal dy);
  fn _ZN22QGraphicsItemAnimation16setTranslationAtEddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double);
  // proto:  void QGraphicsItemAnimation::setShearAt(qreal step, qreal sh, qreal sv);
  fn _ZN22QGraphicsItemAnimation10setShearAtEddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double);
  // proto:  qreal QGraphicsItemAnimation::rotationAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation10rotationAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  const QMetaObject * QGraphicsItemAnimation::metaObject();
  fn _ZNK22QGraphicsItemAnimation10metaObjectEv(qthis: *mut c_void);
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::scaleList();
  fn _ZNK22QGraphicsItemAnimation9scaleListEv(qthis: *mut c_void);
  // proto:  QList<QPair<qreal, qreal> > QGraphicsItemAnimation::rotationList();
  fn _ZNK22QGraphicsItemAnimation12rotationListEv(qthis: *mut c_void);
  // proto:  void QGraphicsItemAnimation::reset();
  fn _ZN22QGraphicsItemAnimation5resetEv(qthis: *mut c_void);
  // proto:  void QGraphicsItemAnimation::setTimeLine(QTimeLine * timeLine);
  fn _ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::shearList();
  fn _ZNK22QGraphicsItemAnimation9shearListEv(qthis: *mut c_void);
  // proto:  void QGraphicsItemAnimation::clear();
  fn _ZN22QGraphicsItemAnimation5clearEv(qthis: *mut c_void);
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::translationList();
  fn _ZNK22QGraphicsItemAnimation15translationListEv(qthis: *mut c_void);
  // proto:  void QGraphicsItemAnimation::setItem(QGraphicsItem * item);
  fn _ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsItemAnimation::setStep(qreal x);
  fn _ZN22QGraphicsItemAnimation7setStepEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QTimeLine * QGraphicsItemAnimation::timeLine();
  fn _ZNK22QGraphicsItemAnimation8timeLineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QGraphicsItemAnimation::horizontalScaleAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation17horizontalScaleAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  qreal QGraphicsItemAnimation::verticalShearAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation15verticalShearAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsItemAnimation)=1
pub struct QGraphicsItemAnimation {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn inheritFrom(qthis: *mut c_void) -> QGraphicsItemAnimation {
    return QGraphicsItemAnimation{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
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
     unsafe {_ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::QGraphicsItemAnimation(const QGraphicsItemAnimation & );
impl /*struct*/ QGraphicsItemAnimation {
  pub fn New<T: QGraphicsItemAnimation_New>(value: T) -> QGraphicsItemAnimation {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_New {
  fn New(self) -> QGraphicsItemAnimation;
}

  // proto:  void QGraphicsItemAnimation::QGraphicsItemAnimation(const QGraphicsItemAnimation & );
impl<'a> /*trait*/ QGraphicsItemAnimation_New for (&'a QGraphicsItemAnimation) {
  fn New(self) -> QGraphicsItemAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimationC1ERKS_()};
    let ctysz: c_int = unsafe{QGraphicsItemAnimation_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN22QGraphicsItemAnimationC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN22QGraphicsItemAnimationC1ERKS_(arg0)};
    let rsthis = QGraphicsItemAnimation{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
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
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation14xTranslationAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
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
     unsafe {_ZN22QGraphicsItemAnimation13setRotationAtEdd(rsthis.qclsinst, arg0, arg1)};
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
impl<'a> /*trait*/ QGraphicsItemAnimation_posList<()> for () {
  fn posList(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation7posListEv()};
     unsafe {_ZNK22QGraphicsItemAnimation7posListEv(rsthis.qclsinst)};
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
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation15verticalScaleAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
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
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation5posAtEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation17horizontalShearAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
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
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation14yTranslationAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
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
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation8matrixAtEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QMatrix::inheritFrom(ret);
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
impl<'a> /*trait*/ QGraphicsItemAnimation_item<()> for () {
  fn item(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation4itemEv()};
     unsafe {_ZNK22QGraphicsItemAnimation4itemEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::QGraphicsItemAnimation(QObject * parent);
impl<'a> /*trait*/ QGraphicsItemAnimation_New for (&'a QObject) {
  fn New(self) -> QGraphicsItemAnimation {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimationC1EP7QObject()};
    let ctysz: c_int = unsafe{QGraphicsItemAnimation_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN22QGraphicsItemAnimationC1EP7QObject(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN22QGraphicsItemAnimationC1EP7QObject(arg0)};
    let rsthis = QGraphicsItemAnimation{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsItemAnimation::~QGraphicsItemAnimation();
impl /*struct*/ QGraphicsItemAnimation {
  pub fn Free<RetType, T: QGraphicsItemAnimation_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_Free<RetType> {
  fn Free(self , rsthis: & QGraphicsItemAnimation) -> RetType;
}

  // proto:  void QGraphicsItemAnimation::~QGraphicsItemAnimation();
impl<'a> /*trait*/ QGraphicsItemAnimation_Free<()> for () {
  fn Free(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimationD0Ev()};
     unsafe {_ZN22QGraphicsItemAnimationD0Ev(rsthis.qclsinst)};
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
     unsafe {_ZN22QGraphicsItemAnimation10setScaleAtEddd(rsthis.qclsinst, arg0, arg1, arg2)};
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
     unsafe {_ZN22QGraphicsItemAnimation16setTranslationAtEddd(rsthis.qclsinst, arg0, arg1, arg2)};
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
     unsafe {_ZN22QGraphicsItemAnimation10setShearAtEddd(rsthis.qclsinst, arg0, arg1, arg2)};
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
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation10rotationAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
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
impl<'a> /*trait*/ QGraphicsItemAnimation_metaObject<()> for () {
  fn metaObject(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation10metaObjectEv()};
     unsafe {_ZNK22QGraphicsItemAnimation10metaObjectEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QGraphicsItemAnimation_scaleList<()> for () {
  fn scaleList(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation9scaleListEv()};
     unsafe {_ZNK22QGraphicsItemAnimation9scaleListEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QGraphicsItemAnimation_rotationList<()> for () {
  fn rotationList(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation12rotationListEv()};
     unsafe {_ZNK22QGraphicsItemAnimation12rotationListEv(rsthis.qclsinst)};
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
     unsafe {_ZN22QGraphicsItemAnimation5resetEv(rsthis.qclsinst)};
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
     unsafe {_ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QGraphicsItemAnimation_shearList<()> for () {
  fn shearList(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation9shearListEv()};
     unsafe {_ZNK22QGraphicsItemAnimation9shearListEv(rsthis.qclsinst)};
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
     unsafe {_ZN22QGraphicsItemAnimation5clearEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QGraphicsItemAnimation_translationList<()> for () {
  fn translationList(self , rsthis: & QGraphicsItemAnimation) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation15translationListEv()};
     unsafe {_ZNK22QGraphicsItemAnimation15translationListEv(rsthis.qclsinst)};
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
     unsafe {_ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem(rsthis.qclsinst, arg0)};
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
     unsafe {_ZN22QGraphicsItemAnimation7setStepEd(rsthis.qclsinst, arg0)};
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
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation8timeLineEv(rsthis.qclsinst)};
    let mut ret1 = QTimeLine::inheritFrom(ret);
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
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation17horizontalScaleAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
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
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation15verticalShearAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

// <= body block end

