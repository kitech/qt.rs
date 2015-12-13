// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qobject::QObject;
use super::qtimeline::QTimeLine;
use super::qgraphicsitem::QGraphicsItem;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsItemAnimation::setPosAt(qreal step, const QPointF & pos);
  fn _ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF(arg0: c_double, arg1: *const c_void) -> i32;
  // proto: void QGraphicsItemAnimation::NewQGraphicsItemAnimation(const QGraphicsItemAnimation & );
  fn _ZN22QGraphicsItemAnimationC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: double QGraphicsItemAnimation::xTranslationAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation14xTranslationAtEd(arg0: c_double) -> i32;
  // proto: void QGraphicsItemAnimation::setRotationAt(qreal step, qreal angle);
  fn _ZN22QGraphicsItemAnimation13setRotationAtEdd(arg0: c_double, arg1: c_double) -> i32;
  // proto: QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::posList();
  fn _ZNK22QGraphicsItemAnimation7posListEv() -> i32;
  // proto: double QGraphicsItemAnimation::verticalScaleAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation15verticalScaleAtEd(arg0: c_double) -> i32;
  // proto: QPointF QGraphicsItemAnimation::posAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation5posAtEd(arg0: c_double) -> i32;
  // proto: double QGraphicsItemAnimation::horizontalShearAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation17horizontalShearAtEd(arg0: c_double) -> i32;
  // proto: double QGraphicsItemAnimation::yTranslationAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation14yTranslationAtEd(arg0: c_double) -> i32;
  // proto: QMatrix QGraphicsItemAnimation::matrixAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation8matrixAtEd(arg0: c_double) -> i32;
  // proto: QGraphicsItem * QGraphicsItemAnimation::item();
  fn _ZNK22QGraphicsItemAnimation4itemEv() -> i32;
  // proto: void QGraphicsItemAnimation::NewQGraphicsItemAnimation(QObject * parent);
  fn _ZN22QGraphicsItemAnimationC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QGraphicsItemAnimation::FreeQGraphicsItemAnimation();
  fn _ZN22QGraphicsItemAnimationD0Ev() -> i32;
  // proto: void QGraphicsItemAnimation::setScaleAt(qreal step, qreal sx, qreal sy);
  fn _ZN22QGraphicsItemAnimation10setScaleAtEddd(arg0: c_double, arg1: c_double, arg2: c_double) -> i32;
  // proto: void QGraphicsItemAnimation::setTranslationAt(qreal step, qreal dx, qreal dy);
  fn _ZN22QGraphicsItemAnimation16setTranslationAtEddd(arg0: c_double, arg1: c_double, arg2: c_double) -> i32;
  // proto: void QGraphicsItemAnimation::setShearAt(qreal step, qreal sh, qreal sv);
  fn _ZN22QGraphicsItemAnimation10setShearAtEddd(arg0: c_double, arg1: c_double, arg2: c_double) -> i32;
  // proto: double QGraphicsItemAnimation::rotationAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation10rotationAtEd(arg0: c_double) -> i32;
  // proto: const QMetaObject * QGraphicsItemAnimation::metaObject();
  fn _ZNK22QGraphicsItemAnimation10metaObjectEv() -> i32;
  // proto: QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::scaleList();
  fn _ZNK22QGraphicsItemAnimation9scaleListEv() -> i32;
  // proto: QList<QPair<qreal, qreal> > QGraphicsItemAnimation::rotationList();
  fn _ZNK22QGraphicsItemAnimation12rotationListEv() -> i32;
  // proto: void QGraphicsItemAnimation::reset();
  fn _ZN22QGraphicsItemAnimation5resetEv() -> i32;
  // proto: void QGraphicsItemAnimation::setTimeLine(QTimeLine * timeLine);
  fn _ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine(arg0: *mut c_void) -> i32;
  // proto: QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::shearList();
  fn _ZNK22QGraphicsItemAnimation9shearListEv() -> i32;
  // proto: void QGraphicsItemAnimation::clear();
  fn _ZN22QGraphicsItemAnimation5clearEv() -> i32;
  // proto: QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::translationList();
  fn _ZNK22QGraphicsItemAnimation15translationListEv() -> i32;
  // proto: void QGraphicsItemAnimation::setItem(QGraphicsItem * item);
  fn _ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsItemAnimation::setStep(qreal x);
  fn _ZN22QGraphicsItemAnimation7setStepEd(arg0: c_double) -> i32;
  // proto: QTimeLine * QGraphicsItemAnimation::timeLine();
  fn _ZNK22QGraphicsItemAnimation8timeLineEv() -> i32;
  // proto: double QGraphicsItemAnimation::horizontalScaleAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation17horizontalScaleAtEd(arg0: c_double) -> i32;
  // proto: double QGraphicsItemAnimation::verticalShearAt(qreal step);
  fn _ZNK22QGraphicsItemAnimation15verticalShearAtEd(arg0: c_double) -> i32;
}

// body block begin
// class sizeof(QGraphicsItemAnimation)=1
pub struct QGraphicsItemAnimation {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setPosAt<T: QGraphicsItemAnimation_setPosAt>(&mut self, value: T) -> i32 {
    value.setPosAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_setPosAt {
  fn setPosAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::setPosAt(qreal step, const QPointF & pos);
impl<'a> /*trait*/ QGraphicsItemAnimation_setPosAt for (f64, &'a  QPointF) {
  fn setPosAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN22QGraphicsItemAnimation8setPosAtEdRK7QPointF(arg0, arg1)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN22QGraphicsItemAnimationC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsItemAnimation{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn xTranslationAt<T: QGraphicsItemAnimation_xTranslationAt>(&mut self, value: T) -> i32 {
    value.xTranslationAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_xTranslationAt {
  fn xTranslationAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: double QGraphicsItemAnimation::xTranslationAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_xTranslationAt for (f64) {
  fn xTranslationAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation14xTranslationAtEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK22QGraphicsItemAnimation14xTranslationAtEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setRotationAt<T: QGraphicsItemAnimation_setRotationAt>(&mut self, value: T) -> i32 {
    value.setRotationAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_setRotationAt {
  fn setRotationAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::setRotationAt(qreal step, qreal angle);
impl<'a> /*trait*/ QGraphicsItemAnimation_setRotationAt for (f64, f64) {
  fn setRotationAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation13setRotationAtEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN22QGraphicsItemAnimation13setRotationAtEdd(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn posList<T: QGraphicsItemAnimation_posList>(&mut self, value: T) -> i32 {
    value.posList(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_posList {
  fn posList(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::posList();
impl<'a> /*trait*/ QGraphicsItemAnimation_posList for () {
  fn posList(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation7posListEv()};
    unsafe {_ZNK22QGraphicsItemAnimation7posListEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn verticalScaleAt<T: QGraphicsItemAnimation_verticalScaleAt>(&mut self, value: T) -> i32 {
    value.verticalScaleAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_verticalScaleAt {
  fn verticalScaleAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: double QGraphicsItemAnimation::verticalScaleAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_verticalScaleAt for (f64) {
  fn verticalScaleAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation15verticalScaleAtEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK22QGraphicsItemAnimation15verticalScaleAtEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn posAt<T: QGraphicsItemAnimation_posAt>(&mut self, value: T) -> i32 {
    value.posAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_posAt {
  fn posAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: QPointF QGraphicsItemAnimation::posAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_posAt for (f64) {
  fn posAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation5posAtEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK22QGraphicsItemAnimation5posAtEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn horizontalShearAt<T: QGraphicsItemAnimation_horizontalShearAt>(&mut self, value: T) -> i32 {
    value.horizontalShearAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_horizontalShearAt {
  fn horizontalShearAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: double QGraphicsItemAnimation::horizontalShearAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_horizontalShearAt for (f64) {
  fn horizontalShearAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation17horizontalShearAtEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK22QGraphicsItemAnimation17horizontalShearAtEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn yTranslationAt<T: QGraphicsItemAnimation_yTranslationAt>(&mut self, value: T) -> i32 {
    value.yTranslationAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_yTranslationAt {
  fn yTranslationAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: double QGraphicsItemAnimation::yTranslationAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_yTranslationAt for (f64) {
  fn yTranslationAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation14yTranslationAtEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK22QGraphicsItemAnimation14yTranslationAtEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn matrixAt<T: QGraphicsItemAnimation_matrixAt>(&mut self, value: T) -> i32 {
    value.matrixAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_matrixAt {
  fn matrixAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: QMatrix QGraphicsItemAnimation::matrixAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_matrixAt for (f64) {
  fn matrixAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation8matrixAtEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK22QGraphicsItemAnimation8matrixAtEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn item<T: QGraphicsItemAnimation_item>(&mut self, value: T) -> i32 {
    value.item(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_item {
  fn item(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: QGraphicsItem * QGraphicsItemAnimation::item();
impl<'a> /*trait*/ QGraphicsItemAnimation_item for () {
  fn item(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation4itemEv()};
    unsafe {_ZNK22QGraphicsItemAnimation4itemEv()};
    return 1;
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
  pub fn FreeQGraphicsItemAnimation<T: QGraphicsItemAnimation_FreeQGraphicsItemAnimation>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsItemAnimation(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_FreeQGraphicsItemAnimation {
  fn FreeQGraphicsItemAnimation(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::FreeQGraphicsItemAnimation();
impl<'a> /*trait*/ QGraphicsItemAnimation_FreeQGraphicsItemAnimation for () {
  fn FreeQGraphicsItemAnimation(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimationD0Ev()};
    unsafe {_ZN22QGraphicsItemAnimationD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setScaleAt<T: QGraphicsItemAnimation_setScaleAt>(&mut self, value: T) -> i32 {
    value.setScaleAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_setScaleAt {
  fn setScaleAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::setScaleAt(qreal step, qreal sx, qreal sy);
impl<'a> /*trait*/ QGraphicsItemAnimation_setScaleAt for (f64, f64, f64) {
  fn setScaleAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation10setScaleAtEddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZN22QGraphicsItemAnimation10setScaleAtEddd(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setTranslationAt<T: QGraphicsItemAnimation_setTranslationAt>(&mut self, value: T) -> i32 {
    value.setTranslationAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_setTranslationAt {
  fn setTranslationAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::setTranslationAt(qreal step, qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsItemAnimation_setTranslationAt for (f64, f64, f64) {
  fn setTranslationAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation16setTranslationAtEddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZN22QGraphicsItemAnimation16setTranslationAtEddd(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setShearAt<T: QGraphicsItemAnimation_setShearAt>(&mut self, value: T) -> i32 {
    value.setShearAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_setShearAt {
  fn setShearAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::setShearAt(qreal step, qreal sh, qreal sv);
impl<'a> /*trait*/ QGraphicsItemAnimation_setShearAt for (f64, f64, f64) {
  fn setShearAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation10setShearAtEddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    unsafe {_ZN22QGraphicsItemAnimation10setShearAtEddd(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn rotationAt<T: QGraphicsItemAnimation_rotationAt>(&mut self, value: T) -> i32 {
    value.rotationAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_rotationAt {
  fn rotationAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: double QGraphicsItemAnimation::rotationAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_rotationAt for (f64) {
  fn rotationAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation10rotationAtEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK22QGraphicsItemAnimation10rotationAtEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn metaObject<T: QGraphicsItemAnimation_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_metaObject {
  fn metaObject(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: const QMetaObject * QGraphicsItemAnimation::metaObject();
impl<'a> /*trait*/ QGraphicsItemAnimation_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation10metaObjectEv()};
    unsafe {_ZNK22QGraphicsItemAnimation10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn scaleList<T: QGraphicsItemAnimation_scaleList>(&mut self, value: T) -> i32 {
    value.scaleList(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_scaleList {
  fn scaleList(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::scaleList();
impl<'a> /*trait*/ QGraphicsItemAnimation_scaleList for () {
  fn scaleList(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation9scaleListEv()};
    unsafe {_ZNK22QGraphicsItemAnimation9scaleListEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn rotationList<T: QGraphicsItemAnimation_rotationList>(&mut self, value: T) -> i32 {
    value.rotationList(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_rotationList {
  fn rotationList(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: QList<QPair<qreal, qreal> > QGraphicsItemAnimation::rotationList();
impl<'a> /*trait*/ QGraphicsItemAnimation_rotationList for () {
  fn rotationList(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation12rotationListEv()};
    unsafe {_ZNK22QGraphicsItemAnimation12rotationListEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn reset<T: QGraphicsItemAnimation_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_reset {
  fn reset(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::reset();
impl<'a> /*trait*/ QGraphicsItemAnimation_reset for () {
  fn reset(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation5resetEv()};
    unsafe {_ZN22QGraphicsItemAnimation5resetEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setTimeLine<T: QGraphicsItemAnimation_setTimeLine>(&mut self, value: T) -> i32 {
    value.setTimeLine(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_setTimeLine {
  fn setTimeLine(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::setTimeLine(QTimeLine * timeLine);
impl<'a> /*trait*/ QGraphicsItemAnimation_setTimeLine for (&'a mut QTimeLine) {
  fn setTimeLine(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QGraphicsItemAnimation11setTimeLineEP9QTimeLine(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn shearList<T: QGraphicsItemAnimation_shearList>(&mut self, value: T) -> i32 {
    value.shearList(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_shearList {
  fn shearList(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::shearList();
impl<'a> /*trait*/ QGraphicsItemAnimation_shearList for () {
  fn shearList(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation9shearListEv()};
    unsafe {_ZNK22QGraphicsItemAnimation9shearListEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn clear<T: QGraphicsItemAnimation_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_clear {
  fn clear(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::clear();
impl<'a> /*trait*/ QGraphicsItemAnimation_clear for () {
  fn clear(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation5clearEv()};
    unsafe {_ZN22QGraphicsItemAnimation5clearEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn translationList<T: QGraphicsItemAnimation_translationList>(&mut self, value: T) -> i32 {
    value.translationList(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_translationList {
  fn translationList(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: QList<QPair<qreal, QPointF> > QGraphicsItemAnimation::translationList();
impl<'a> /*trait*/ QGraphicsItemAnimation_translationList for () {
  fn translationList(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation15translationListEv()};
    unsafe {_ZNK22QGraphicsItemAnimation15translationListEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setItem<T: QGraphicsItemAnimation_setItem>(&mut self, value: T) -> i32 {
    value.setItem(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_setItem {
  fn setItem(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::setItem(QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsItemAnimation_setItem for (&'a mut QGraphicsItem) {
  fn setItem(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QGraphicsItemAnimation7setItemEP13QGraphicsItem(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn setStep<T: QGraphicsItemAnimation_setStep>(&mut self, value: T) -> i32 {
    value.setStep(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_setStep {
  fn setStep(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: void QGraphicsItemAnimation::setStep(qreal x);
impl<'a> /*trait*/ QGraphicsItemAnimation_setStep for (f64) {
  fn setStep(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsItemAnimation7setStepEd()};
    let arg0 = self  as c_double;
    unsafe {_ZN22QGraphicsItemAnimation7setStepEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn timeLine<T: QGraphicsItemAnimation_timeLine>(&mut self, value: T) -> i32 {
    value.timeLine(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_timeLine {
  fn timeLine(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: QTimeLine * QGraphicsItemAnimation::timeLine();
impl<'a> /*trait*/ QGraphicsItemAnimation_timeLine for () {
  fn timeLine(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation8timeLineEv()};
    unsafe {_ZNK22QGraphicsItemAnimation8timeLineEv()};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn horizontalScaleAt<T: QGraphicsItemAnimation_horizontalScaleAt>(&mut self, value: T) -> i32 {
    value.horizontalScaleAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_horizontalScaleAt {
  fn horizontalScaleAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: double QGraphicsItemAnimation::horizontalScaleAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_horizontalScaleAt for (f64) {
  fn horizontalScaleAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation17horizontalScaleAtEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK22QGraphicsItemAnimation17horizontalScaleAtEd(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsItemAnimation {
  pub fn verticalShearAt<T: QGraphicsItemAnimation_verticalShearAt>(&mut self, value: T) -> i32 {
    value.verticalShearAt(self);
    return 1;
  }
}

pub trait QGraphicsItemAnimation_verticalShearAt {
  fn verticalShearAt(self, this: &mut QGraphicsItemAnimation) -> i32;
}

// proto: double QGraphicsItemAnimation::verticalShearAt(qreal step);
impl<'a> /*trait*/ QGraphicsItemAnimation_verticalShearAt for (f64) {
  fn verticalShearAt(self, this: &mut QGraphicsItemAnimation) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsItemAnimation15verticalShearAtEd()};
    let arg0 = self  as c_double;
    unsafe {_ZNK22QGraphicsItemAnimation15verticalShearAtEd(arg0)};
    return 1;
  }
}

