// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qmatrix::QMatrix;
use super::qobject::QObject;
use super::qtimeline::QTimeLine;
use super::qgraphicsitem::QGraphicsItem;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsItemAnimation::setPosAt(qreal step, const QPointF & pos);
  fn _ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF(qthis: *mut c_void, arg0: c_double, arg1: *mut c_void) ;
  // proto:  void QGraphicsItemAnimation::NewQGraphicsItemAnimation(const QGraphicsItemAnimation & );
  fn _ZN22QGraphicsItemAnimationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QGraphicsItemAnimation::xTranslationAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation14xTranslationAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QGraphicsItemAnimation::setRotationAt(qreal step, qreal angle);
  fn _ZN22QGraphicsItemAnimation13setRotationAtEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::posList();
  fn _ZNK22QGraphicsItemAnimation7posListEv(qthis: *mut c_void) ;
  // proto:  double QGraphicsItemAnimation::verticalScaleAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation15verticalScaleAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  QPointF QGraphicsItemAnimation::posAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation5posAtEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  double QGraphicsItemAnimation::horizontalShearAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation17horizontalShearAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  double QGraphicsItemAnimation::yTranslationAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation14yTranslationAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  QMatrix QGraphicsItemAnimation::matrixAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation8matrixAtEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  QGraphicsItem * QGraphicsItemAnimation::item();
  fn _ZNK22QGraphicsItemAnimation4itemEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItemAnimation::NewQGraphicsItemAnimation(QObject * parent);
  fn _ZN22QGraphicsItemAnimationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsItemAnimation::FreeQGraphicsItemAnimation();
  fn _ZN22QGraphicsItemAnimationD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsItemAnimation::setScaleAt(qreal step, qreal sx, qreal sy);
  fn _ZN22QGraphicsItemAnimation10setScaleAtEddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double) ;
  // proto:  void QGraphicsItemAnimation::setTranslationAt(qreal step, qreal dx, qreal dy);
  fn _ZN22QGraphicsItemAnimation16setTranslationAtEddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double) ;
  // proto:  void QGraphicsItemAnimation::setShearAt(qreal step, qreal sh, qreal sv);
  fn _ZN22QGraphicsItemAnimation10setShearAtEddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double) ;
  // proto:  double QGraphicsItemAnimation::rotationAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation10rotationAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  const QMetaObject * QGraphicsItemAnimation::metaObject();
  fn _ZNK22QGraphicsItemAnimation10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::scaleList();
  fn _ZNK22QGraphicsItemAnimation9scaleListEv(qthis: *mut c_void) ;
  // proto:  QList<QPair<qreal, qreal> > QGraphicsItemAnimation::rotationList();
  fn _ZNK22QGraphicsItemAnimation12rotationListEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItemAnimation::reset();
  fn _ZN22QGraphicsItemAnimation5resetEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItemAnimation::setTimeLine(QTimeLine * timeLine);
  fn _ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::shearList();
  fn _ZNK22QGraphicsItemAnimation9shearListEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItemAnimation::clear();
  fn _ZN22QGraphicsItemAnimation5clearEv(qthis: *mut c_void) ;
  // proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::translationList();
  fn _ZNK22QGraphicsItemAnimation15translationListEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsItemAnimation::setItem(QGraphicsItem * item);
  fn _ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsItemAnimation::setStep(qreal x);
  fn _ZN22QGraphicsItemAnimation7setStepEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  QTimeLine * QGraphicsItemAnimation::timeLine();
  fn _ZNK22QGraphicsItemAnimation8timeLineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QGraphicsItemAnimation::horizontalScaleAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation17horizontalScaleAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  double QGraphicsItemAnimation::verticalShearAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation15verticalShearAtEd(qthis: *mut c_void, arg0: c_double) -> c_double;
}

// body block begin
// class sizeof(QGraphicsItemAnimation)=1
pub struct QGraphicsItemAnimation {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setPosAt<T: QGraphicsItemAnimation_setPosAt>(&mut self, value: T)  {
     value.setPosAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setPosAt {
  fn setPosAt(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::setPosAt(qreal step, const QPointF & pos);
impl<'a> /*trait*/ QGraphicsItemAnimation_setPosAt for (f64, &'a  QPointF) {
  fn setPosAt(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn NewQGraphicsItemAnimation<T: QGraphicsItemAnimation_NewQGraphicsItemAnimation>(value: T) -> QGraphicsItemAnimation {
    let rsthis = value.NewQGraphicsItemAnimation();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_NewQGraphicsItemAnimation {
  fn NewQGraphicsItemAnimation(self) -> QGraphicsItemAnimation;
}

// proto: void QGraphicsItemAnimation::NewQGraphicsItemAnimation(const QGraphicsItemAnimation & );
impl<'a> /*trait*/ QGraphicsItemAnimation_NewQGraphicsItemAnimation for (&'a  QGraphicsItemAnimation) {
  fn NewQGraphicsItemAnimation(self) -> QGraphicsItemAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimationC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QGraphicsItemAnimationC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsItemAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn xTranslationAt<T: QGraphicsItemAnimation_xTranslationAt>(&mut self, value: T) -> f64 {
    return value.xTranslationAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_xTranslationAt {
  fn xTranslationAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64;
}

// proto:  double QGraphicsItemAnimation::xTranslationAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_xTranslationAt for (f64) {
  fn xTranslationAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation14xTranslationAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation14xTranslationAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setRotationAt<T: QGraphicsItemAnimation_setRotationAt>(&mut self, value: T)  {
     value.setRotationAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setRotationAt {
  fn setRotationAt(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::setRotationAt(qreal step, qreal angle);
impl<'a> /*trait*/ QGraphicsItemAnimation_setRotationAt for (f64, f64) {
  fn setRotationAt(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation13setRotationAtEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN22QGraphicsItemAnimation13setRotationAtEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn posList<T: QGraphicsItemAnimation_posList>(&mut self, value: T)  {
     value.posList(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_posList {
  fn posList(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::posList();
impl<'a> /*trait*/ QGraphicsItemAnimation_posList for () {
  fn posList(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation7posListEv()};
     unsafe {_ZNK22QGraphicsItemAnimation7posListEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn verticalScaleAt<T: QGraphicsItemAnimation_verticalScaleAt>(&mut self, value: T) -> f64 {
    return value.verticalScaleAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_verticalScaleAt {
  fn verticalScaleAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64;
}

// proto:  double QGraphicsItemAnimation::verticalScaleAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_verticalScaleAt for (f64) {
  fn verticalScaleAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation15verticalScaleAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation15verticalScaleAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn posAt<T: QGraphicsItemAnimation_posAt>(&mut self, value: T) -> QPointF {
    return value.posAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_posAt {
  fn posAt(self, rsthis: &mut QGraphicsItemAnimation) -> QPointF;
}

// proto:  QPointF QGraphicsItemAnimation::posAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_posAt for (f64) {
  fn posAt(self, rsthis: &mut QGraphicsItemAnimation) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation5posAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation5posAtEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn horizontalShearAt<T: QGraphicsItemAnimation_horizontalShearAt>(&mut self, value: T) -> f64 {
    return value.horizontalShearAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_horizontalShearAt {
  fn horizontalShearAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64;
}

// proto:  double QGraphicsItemAnimation::horizontalShearAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_horizontalShearAt for (f64) {
  fn horizontalShearAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation17horizontalShearAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation17horizontalShearAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn yTranslationAt<T: QGraphicsItemAnimation_yTranslationAt>(&mut self, value: T) -> f64 {
    return value.yTranslationAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_yTranslationAt {
  fn yTranslationAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64;
}

// proto:  double QGraphicsItemAnimation::yTranslationAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_yTranslationAt for (f64) {
  fn yTranslationAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation14yTranslationAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation14yTranslationAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn matrixAt<T: QGraphicsItemAnimation_matrixAt>(&mut self, value: T) -> QMatrix {
    return value.matrixAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_matrixAt {
  fn matrixAt(self, rsthis: &mut QGraphicsItemAnimation) -> QMatrix;
}

// proto:  QMatrix QGraphicsItemAnimation::matrixAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_matrixAt for (f64) {
  fn matrixAt(self, rsthis: &mut QGraphicsItemAnimation) -> QMatrix {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation8matrixAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation8matrixAtEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QMatrix{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn item<T: QGraphicsItemAnimation_item>(&mut self, value: T)  {
     value.item(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_item {
  fn item(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  QGraphicsItem * QGraphicsItemAnimation::item();
impl<'a> /*trait*/ QGraphicsItemAnimation_item for () {
  fn item(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation4itemEv()};
     unsafe {_ZNK22QGraphicsItemAnimation4itemEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QGraphicsItemAnimation::NewQGraphicsItemAnimation(QObject * parent);
impl<'a> /*trait*/ QGraphicsItemAnimation_NewQGraphicsItemAnimation for (&'a mut QObject) {
  fn NewQGraphicsItemAnimation(self) -> QGraphicsItemAnimation {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimationC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QGraphicsItemAnimationC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsItemAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn FreeQGraphicsItemAnimation<T: QGraphicsItemAnimation_FreeQGraphicsItemAnimation>(&mut self, value: T)  {
     value.FreeQGraphicsItemAnimation(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_FreeQGraphicsItemAnimation {
  fn FreeQGraphicsItemAnimation(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::FreeQGraphicsItemAnimation();
impl<'a> /*trait*/ QGraphicsItemAnimation_FreeQGraphicsItemAnimation for () {
  fn FreeQGraphicsItemAnimation(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimationD0Ev()};
     unsafe {_ZN22QGraphicsItemAnimationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setScaleAt<T: QGraphicsItemAnimation_setScaleAt>(&mut self, value: T)  {
     value.setScaleAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setScaleAt {
  fn setScaleAt(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::setScaleAt(qreal step, qreal sx, qreal sy);
impl<'a> /*trait*/ QGraphicsItemAnimation_setScaleAt for (f64, f64, f64) {
  fn setScaleAt(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation10setScaleAtEddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN22QGraphicsItemAnimation10setScaleAtEddd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setTranslationAt<T: QGraphicsItemAnimation_setTranslationAt>(&mut self, value: T)  {
     value.setTranslationAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setTranslationAt {
  fn setTranslationAt(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::setTranslationAt(qreal step, qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsItemAnimation_setTranslationAt for (f64, f64, f64) {
  fn setTranslationAt(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation16setTranslationAtEddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN22QGraphicsItemAnimation16setTranslationAtEddd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setShearAt<T: QGraphicsItemAnimation_setShearAt>(&mut self, value: T)  {
     value.setShearAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setShearAt {
  fn setShearAt(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::setShearAt(qreal step, qreal sh, qreal sv);
impl<'a> /*trait*/ QGraphicsItemAnimation_setShearAt for (f64, f64, f64) {
  fn setShearAt(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation10setShearAtEddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN22QGraphicsItemAnimation10setShearAtEddd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn rotationAt<T: QGraphicsItemAnimation_rotationAt>(&mut self, value: T) -> f64 {
    return value.rotationAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_rotationAt {
  fn rotationAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64;
}

// proto:  double QGraphicsItemAnimation::rotationAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_rotationAt for (f64) {
  fn rotationAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation10rotationAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation10rotationAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn metaObject<T: QGraphicsItemAnimation_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_metaObject {
  fn metaObject(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  const QMetaObject * QGraphicsItemAnimation::metaObject();
impl<'a> /*trait*/ QGraphicsItemAnimation_metaObject for () {
  fn metaObject(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation10metaObjectEv()};
     unsafe {_ZNK22QGraphicsItemAnimation10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn scaleList<T: QGraphicsItemAnimation_scaleList>(&mut self, value: T)  {
     value.scaleList(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_scaleList {
  fn scaleList(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::scaleList();
impl<'a> /*trait*/ QGraphicsItemAnimation_scaleList for () {
  fn scaleList(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation9scaleListEv()};
     unsafe {_ZNK22QGraphicsItemAnimation9scaleListEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn rotationList<T: QGraphicsItemAnimation_rotationList>(&mut self, value: T)  {
     value.rotationList(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_rotationList {
  fn rotationList(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  QList<QPair<qreal, qreal> > QGraphicsItemAnimation::rotationList();
impl<'a> /*trait*/ QGraphicsItemAnimation_rotationList for () {
  fn rotationList(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation12rotationListEv()};
     unsafe {_ZNK22QGraphicsItemAnimation12rotationListEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn reset<T: QGraphicsItemAnimation_reset>(&mut self, value: T)  {
     value.reset(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_reset {
  fn reset(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::reset();
impl<'a> /*trait*/ QGraphicsItemAnimation_reset for () {
  fn reset(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation5resetEv()};
     unsafe {_ZN22QGraphicsItemAnimation5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setTimeLine<T: QGraphicsItemAnimation_setTimeLine>(&mut self, value: T)  {
     value.setTimeLine(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setTimeLine {
  fn setTimeLine(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::setTimeLine(QTimeLine * timeLine);
impl<'a> /*trait*/ QGraphicsItemAnimation_setTimeLine for (&'a mut QTimeLine) {
  fn setTimeLine(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn shearList<T: QGraphicsItemAnimation_shearList>(&mut self, value: T)  {
     value.shearList(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_shearList {
  fn shearList(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::shearList();
impl<'a> /*trait*/ QGraphicsItemAnimation_shearList for () {
  fn shearList(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation9shearListEv()};
     unsafe {_ZNK22QGraphicsItemAnimation9shearListEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn clear<T: QGraphicsItemAnimation_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_clear {
  fn clear(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::clear();
impl<'a> /*trait*/ QGraphicsItemAnimation_clear for () {
  fn clear(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation5clearEv()};
     unsafe {_ZN22QGraphicsItemAnimation5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn translationList<T: QGraphicsItemAnimation_translationList>(&mut self, value: T)  {
     value.translationList(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_translationList {
  fn translationList(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::translationList();
impl<'a> /*trait*/ QGraphicsItemAnimation_translationList for () {
  fn translationList(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation15translationListEv()};
     unsafe {_ZNK22QGraphicsItemAnimation15translationListEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setItem<T: QGraphicsItemAnimation_setItem>(&mut self, value: T)  {
     value.setItem(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setItem {
  fn setItem(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::setItem(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemAnimation_setItem for (&'a mut QGraphicsItem) {
  fn setItem(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setStep<T: QGraphicsItemAnimation_setStep>(&mut self, value: T)  {
     value.setStep(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_setStep {
  fn setStep(self, rsthis: &mut QGraphicsItemAnimation) ;
}

// proto:  void QGraphicsItemAnimation::setStep(qreal x);
impl<'a> /*trait*/ QGraphicsItemAnimation_setStep for (f64) {
  fn setStep(self, rsthis: &mut QGraphicsItemAnimation)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation7setStepEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN22QGraphicsItemAnimation7setStepEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn timeLine<T: QGraphicsItemAnimation_timeLine>(&mut self, value: T) -> QTimeLine {
    return value.timeLine(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_timeLine {
  fn timeLine(self, rsthis: &mut QGraphicsItemAnimation) -> QTimeLine;
}

// proto:  QTimeLine * QGraphicsItemAnimation::timeLine();
impl<'a> /*trait*/ QGraphicsItemAnimation_timeLine for () {
  fn timeLine(self, rsthis: &mut QGraphicsItemAnimation) -> QTimeLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation8timeLineEv()};
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation8timeLineEv(rsthis.qclsinst)};
    let mut ret1 = QTimeLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn horizontalScaleAt<T: QGraphicsItemAnimation_horizontalScaleAt>(&mut self, value: T) -> f64 {
    return value.horizontalScaleAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_horizontalScaleAt {
  fn horizontalScaleAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64;
}

// proto:  double QGraphicsItemAnimation::horizontalScaleAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_horizontalScaleAt for (f64) {
  fn horizontalScaleAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation17horizontalScaleAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation17horizontalScaleAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn verticalShearAt<T: QGraphicsItemAnimation_verticalShearAt>(&mut self, value: T) -> f64 {
    return value.verticalShearAt(self);
    // return 1;
  }
}

pub trait QGraphicsItemAnimation_verticalShearAt {
  fn verticalShearAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64;
}

// proto:  double QGraphicsItemAnimation::verticalShearAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_verticalShearAt for (f64) {
  fn verticalShearAt(self, rsthis: &mut QGraphicsItemAnimation) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation15verticalShearAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK22QGraphicsItemAnimation15verticalShearAtEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

