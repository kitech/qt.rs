// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtWidgets/qabstractslider.h
// dst-file: /src/widgets/qabstractslider.rs
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
use super::qwidget::QWidget; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QAbstractSlider::setSliderPosition(int );
  fn _ZN15QAbstractSlider17setSliderPositionEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAbstractSlider::sliderPressed();
  fn _ZN15QAbstractSlider13sliderPressedEv(qthis: *mut c_void);
  // proto:  bool QAbstractSlider::isSliderDown();
  fn _ZNK15QAbstractSlider12isSliderDownEv(qthis: *mut c_void) -> c_char;
  // proto:  int QAbstractSlider::value();
  fn _ZNK15QAbstractSlider5valueEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAbstractSlider::setInvertedControls(bool );
  fn _ZN15QAbstractSlider19setInvertedControlsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QAbstractSlider::minimum();
  fn _ZNK15QAbstractSlider7minimumEv(qthis: *mut c_void) -> c_int;
  // proto:  int QAbstractSlider::singleStep();
  fn _ZNK15QAbstractSlider10singleStepEv(qthis: *mut c_void) -> c_int;
  // proto:  int QAbstractSlider::pageStep();
  fn _ZNK15QAbstractSlider8pageStepEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAbstractSlider::setMaximum(int );
  fn _ZN15QAbstractSlider10setMaximumEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QAbstractSlider::invertedControls();
  fn _ZNK15QAbstractSlider16invertedControlsEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAbstractSlider::setValue(int );
  fn _ZN15QAbstractSlider8setValueEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAbstractSlider::valueChanged(int value);
  fn _ZN15QAbstractSlider12valueChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAbstractSlider::~QAbstractSlider();
  fn _ZN15QAbstractSliderD0Ev(qthis: *mut c_void);
  // proto:  void QAbstractSlider::setPageStep(int );
  fn _ZN15QAbstractSlider11setPageStepEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAbstractSlider::setSliderDown(bool );
  fn _ZN15QAbstractSlider13setSliderDownEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QAbstractSlider::maximum();
  fn _ZNK15QAbstractSlider7maximumEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QAbstractSlider::metaObject();
  fn _ZNK15QAbstractSlider10metaObjectEv(qthis: *mut c_void);
  // proto:  void QAbstractSlider::actionTriggered(int action);
  fn _ZN15QAbstractSlider15actionTriggeredEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAbstractSlider::setSingleStep(int );
  fn _ZN15QAbstractSlider13setSingleStepEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAbstractSlider::sliderMoved(int position);
  fn _ZN15QAbstractSlider11sliderMovedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QAbstractSlider::setInvertedAppearance(bool );
  fn _ZN15QAbstractSlider21setInvertedAppearanceEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QAbstractSlider::rangeChanged(int min, int max);
  fn _ZN15QAbstractSlider12rangeChangedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  bool QAbstractSlider::hasTracking();
  fn _ZNK15QAbstractSlider11hasTrackingEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAbstractSlider::sliderReleased();
  fn _ZN15QAbstractSlider14sliderReleasedEv(qthis: *mut c_void);
  // proto:  bool QAbstractSlider::invertedAppearance();
  fn _ZNK15QAbstractSlider18invertedAppearanceEv(qthis: *mut c_void) -> c_char;
  // proto:  int QAbstractSlider::sliderPosition();
  fn _ZNK15QAbstractSlider14sliderPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAbstractSlider::setTracking(bool enable);
  fn _ZN15QAbstractSlider11setTrackingEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QAbstractSlider::QAbstractSlider(QWidget * parent);
  fn _ZN15QAbstractSliderC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractSlider::setRange(int min, int max);
  fn _ZN15QAbstractSlider8setRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  void QAbstractSlider::setMinimum(int );
  fn _ZN15QAbstractSlider10setMinimumEi(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractSlider)=1
pub struct QAbstractSlider {
  pub qclsinst: *mut c_void,
}

  // proto:  void QAbstractSlider::setSliderPosition(int );
impl /*struct*/ QAbstractSlider {
  pub fn setSliderPosition<RetType, T: QAbstractSlider_setSliderPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSliderPosition(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setSliderPosition<RetType> {
  fn setSliderPosition(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setSliderPosition(int );
impl<'a> /*trait*/ QAbstractSlider_setSliderPosition<()> for (i32) {
  fn setSliderPosition(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider17setSliderPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider17setSliderPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::sliderPressed();
impl /*struct*/ QAbstractSlider {
  pub fn sliderPressed<RetType, T: QAbstractSlider_sliderPressed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sliderPressed(self);
    // return 1;
  }
}

pub trait QAbstractSlider_sliderPressed<RetType> {
  fn sliderPressed(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::sliderPressed();
impl<'a> /*trait*/ QAbstractSlider_sliderPressed<()> for () {
  fn sliderPressed(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider13sliderPressedEv()};
     unsafe {_ZN15QAbstractSlider13sliderPressedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractSlider::isSliderDown();
impl /*struct*/ QAbstractSlider {
  pub fn isSliderDown<RetType, T: QAbstractSlider_isSliderDown<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isSliderDown(self);
    // return 1;
  }
}

pub trait QAbstractSlider_isSliderDown<RetType> {
  fn isSliderDown(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  bool QAbstractSlider::isSliderDown();
impl<'a> /*trait*/ QAbstractSlider_isSliderDown<i8> for () {
  fn isSliderDown(self , rsthis: &mut QAbstractSlider) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider12isSliderDownEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider12isSliderDownEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QAbstractSlider::value();
impl /*struct*/ QAbstractSlider {
  pub fn value<RetType, T: QAbstractSlider_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QAbstractSlider_value<RetType> {
  fn value(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::value();
impl<'a> /*trait*/ QAbstractSlider_value<i32> for () {
  fn value(self , rsthis: &mut QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider5valueEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setInvertedControls(bool );
impl /*struct*/ QAbstractSlider {
  pub fn setInvertedControls<RetType, T: QAbstractSlider_setInvertedControls<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setInvertedControls(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setInvertedControls<RetType> {
  fn setInvertedControls(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setInvertedControls(bool );
impl<'a> /*trait*/ QAbstractSlider_setInvertedControls<()> for (i8) {
  fn setInvertedControls(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider19setInvertedControlsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractSlider19setInvertedControlsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAbstractSlider::minimum();
impl /*struct*/ QAbstractSlider {
  pub fn minimum<RetType, T: QAbstractSlider_minimum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimum(self);
    // return 1;
  }
}

pub trait QAbstractSlider_minimum<RetType> {
  fn minimum(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::minimum();
impl<'a> /*trait*/ QAbstractSlider_minimum<i32> for () {
  fn minimum(self , rsthis: &mut QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider7minimumEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider7minimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QAbstractSlider::singleStep();
impl /*struct*/ QAbstractSlider {
  pub fn singleStep<RetType, T: QAbstractSlider_singleStep<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.singleStep(self);
    // return 1;
  }
}

pub trait QAbstractSlider_singleStep<RetType> {
  fn singleStep(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::singleStep();
impl<'a> /*trait*/ QAbstractSlider_singleStep<i32> for () {
  fn singleStep(self , rsthis: &mut QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider10singleStepEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider10singleStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QAbstractSlider::pageStep();
impl /*struct*/ QAbstractSlider {
  pub fn pageStep<RetType, T: QAbstractSlider_pageStep<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pageStep(self);
    // return 1;
  }
}

pub trait QAbstractSlider_pageStep<RetType> {
  fn pageStep(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::pageStep();
impl<'a> /*trait*/ QAbstractSlider_pageStep<i32> for () {
  fn pageStep(self , rsthis: &mut QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider8pageStepEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider8pageStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setMaximum(int );
impl /*struct*/ QAbstractSlider {
  pub fn setMaximum<RetType, T: QAbstractSlider_setMaximum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMaximum(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setMaximum<RetType> {
  fn setMaximum(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setMaximum(int );
impl<'a> /*trait*/ QAbstractSlider_setMaximum<()> for (i32) {
  fn setMaximum(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider10setMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider10setMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractSlider::invertedControls();
impl /*struct*/ QAbstractSlider {
  pub fn invertedControls<RetType, T: QAbstractSlider_invertedControls<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.invertedControls(self);
    // return 1;
  }
}

pub trait QAbstractSlider_invertedControls<RetType> {
  fn invertedControls(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  bool QAbstractSlider::invertedControls();
impl<'a> /*trait*/ QAbstractSlider_invertedControls<i8> for () {
  fn invertedControls(self , rsthis: &mut QAbstractSlider) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider16invertedControlsEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider16invertedControlsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setValue(int );
impl /*struct*/ QAbstractSlider {
  pub fn setValue<RetType, T: QAbstractSlider_setValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setValue<RetType> {
  fn setValue(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setValue(int );
impl<'a> /*trait*/ QAbstractSlider_setValue<()> for (i32) {
  fn setValue(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider8setValueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider8setValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::valueChanged(int value);
impl /*struct*/ QAbstractSlider {
  pub fn valueChanged<RetType, T: QAbstractSlider_valueChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.valueChanged(self);
    // return 1;
  }
}

pub trait QAbstractSlider_valueChanged<RetType> {
  fn valueChanged(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::valueChanged(int value);
impl<'a> /*trait*/ QAbstractSlider_valueChanged<()> for (i32) {
  fn valueChanged(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider12valueChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider12valueChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::~QAbstractSlider();
impl /*struct*/ QAbstractSlider {
  pub fn FreeQAbstractSlider<RetType, T: QAbstractSlider_FreeQAbstractSlider<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAbstractSlider(self);
    // return 1;
  }
}

pub trait QAbstractSlider_FreeQAbstractSlider<RetType> {
  fn FreeQAbstractSlider(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::~QAbstractSlider();
impl<'a> /*trait*/ QAbstractSlider_FreeQAbstractSlider<()> for () {
  fn FreeQAbstractSlider(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSliderD0Ev()};
     unsafe {_ZN15QAbstractSliderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setPageStep(int );
impl /*struct*/ QAbstractSlider {
  pub fn setPageStep<RetType, T: QAbstractSlider_setPageStep<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPageStep(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setPageStep<RetType> {
  fn setPageStep(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setPageStep(int );
impl<'a> /*trait*/ QAbstractSlider_setPageStep<()> for (i32) {
  fn setPageStep(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider11setPageStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider11setPageStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setSliderDown(bool );
impl /*struct*/ QAbstractSlider {
  pub fn setSliderDown<RetType, T: QAbstractSlider_setSliderDown<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSliderDown(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setSliderDown<RetType> {
  fn setSliderDown(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setSliderDown(bool );
impl<'a> /*trait*/ QAbstractSlider_setSliderDown<()> for (i8) {
  fn setSliderDown(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider13setSliderDownEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractSlider13setSliderDownEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAbstractSlider::maximum();
impl /*struct*/ QAbstractSlider {
  pub fn maximum<RetType, T: QAbstractSlider_maximum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.maximum(self);
    // return 1;
  }
}

pub trait QAbstractSlider_maximum<RetType> {
  fn maximum(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::maximum();
impl<'a> /*trait*/ QAbstractSlider_maximum<i32> for () {
  fn maximum(self , rsthis: &mut QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider7maximumEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider7maximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractSlider::metaObject();
impl /*struct*/ QAbstractSlider {
  pub fn metaObject<RetType, T: QAbstractSlider_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractSlider_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  const QMetaObject * QAbstractSlider::metaObject();
impl<'a> /*trait*/ QAbstractSlider_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider10metaObjectEv()};
     unsafe {_ZNK15QAbstractSlider10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::actionTriggered(int action);
impl /*struct*/ QAbstractSlider {
  pub fn actionTriggered<RetType, T: QAbstractSlider_actionTriggered<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.actionTriggered(self);
    // return 1;
  }
}

pub trait QAbstractSlider_actionTriggered<RetType> {
  fn actionTriggered(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::actionTriggered(int action);
impl<'a> /*trait*/ QAbstractSlider_actionTriggered<()> for (i32) {
  fn actionTriggered(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider15actionTriggeredEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider15actionTriggeredEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setSingleStep(int );
impl /*struct*/ QAbstractSlider {
  pub fn setSingleStep<RetType, T: QAbstractSlider_setSingleStep<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSingleStep(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setSingleStep<RetType> {
  fn setSingleStep(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setSingleStep(int );
impl<'a> /*trait*/ QAbstractSlider_setSingleStep<()> for (i32) {
  fn setSingleStep(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider13setSingleStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider13setSingleStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::sliderMoved(int position);
impl /*struct*/ QAbstractSlider {
  pub fn sliderMoved<RetType, T: QAbstractSlider_sliderMoved<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sliderMoved(self);
    // return 1;
  }
}

pub trait QAbstractSlider_sliderMoved<RetType> {
  fn sliderMoved(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::sliderMoved(int position);
impl<'a> /*trait*/ QAbstractSlider_sliderMoved<()> for (i32) {
  fn sliderMoved(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider11sliderMovedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider11sliderMovedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setInvertedAppearance(bool );
impl /*struct*/ QAbstractSlider {
  pub fn setInvertedAppearance<RetType, T: QAbstractSlider_setInvertedAppearance<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setInvertedAppearance(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setInvertedAppearance<RetType> {
  fn setInvertedAppearance(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setInvertedAppearance(bool );
impl<'a> /*trait*/ QAbstractSlider_setInvertedAppearance<()> for (i8) {
  fn setInvertedAppearance(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider21setInvertedAppearanceEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractSlider21setInvertedAppearanceEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::rangeChanged(int min, int max);
impl /*struct*/ QAbstractSlider {
  pub fn rangeChanged<RetType, T: QAbstractSlider_rangeChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rangeChanged(self);
    // return 1;
  }
}

pub trait QAbstractSlider_rangeChanged<RetType> {
  fn rangeChanged(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::rangeChanged(int min, int max);
impl<'a> /*trait*/ QAbstractSlider_rangeChanged<()> for (i32, i32) {
  fn rangeChanged(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider12rangeChangedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN15QAbstractSlider12rangeChangedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QAbstractSlider::hasTracking();
impl /*struct*/ QAbstractSlider {
  pub fn hasTracking<RetType, T: QAbstractSlider_hasTracking<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasTracking(self);
    // return 1;
  }
}

pub trait QAbstractSlider_hasTracking<RetType> {
  fn hasTracking(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  bool QAbstractSlider::hasTracking();
impl<'a> /*trait*/ QAbstractSlider_hasTracking<i8> for () {
  fn hasTracking(self , rsthis: &mut QAbstractSlider) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider11hasTrackingEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider11hasTrackingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::sliderReleased();
impl /*struct*/ QAbstractSlider {
  pub fn sliderReleased<RetType, T: QAbstractSlider_sliderReleased<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sliderReleased(self);
    // return 1;
  }
}

pub trait QAbstractSlider_sliderReleased<RetType> {
  fn sliderReleased(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::sliderReleased();
impl<'a> /*trait*/ QAbstractSlider_sliderReleased<()> for () {
  fn sliderReleased(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider14sliderReleasedEv()};
     unsafe {_ZN15QAbstractSlider14sliderReleasedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractSlider::invertedAppearance();
impl /*struct*/ QAbstractSlider {
  pub fn invertedAppearance<RetType, T: QAbstractSlider_invertedAppearance<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.invertedAppearance(self);
    // return 1;
  }
}

pub trait QAbstractSlider_invertedAppearance<RetType> {
  fn invertedAppearance(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  bool QAbstractSlider::invertedAppearance();
impl<'a> /*trait*/ QAbstractSlider_invertedAppearance<i8> for () {
  fn invertedAppearance(self , rsthis: &mut QAbstractSlider) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider18invertedAppearanceEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider18invertedAppearanceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QAbstractSlider::sliderPosition();
impl /*struct*/ QAbstractSlider {
  pub fn sliderPosition<RetType, T: QAbstractSlider_sliderPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sliderPosition(self);
    // return 1;
  }
}

pub trait QAbstractSlider_sliderPosition<RetType> {
  fn sliderPosition(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::sliderPosition();
impl<'a> /*trait*/ QAbstractSlider_sliderPosition<i32> for () {
  fn sliderPosition(self , rsthis: &mut QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider14sliderPositionEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider14sliderPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setTracking(bool enable);
impl /*struct*/ QAbstractSlider {
  pub fn setTracking<RetType, T: QAbstractSlider_setTracking<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTracking(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setTracking<RetType> {
  fn setTracking(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setTracking(bool enable);
impl<'a> /*trait*/ QAbstractSlider_setTracking<()> for (i8) {
  fn setTracking(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider11setTrackingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractSlider11setTrackingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::QAbstractSlider(QWidget * parent);
impl /*struct*/ QAbstractSlider {
  pub fn NewQAbstractSlider<T: QAbstractSlider_NewQAbstractSlider>(value: T) -> QAbstractSlider {
    let rsthis = value.NewQAbstractSlider();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractSlider_NewQAbstractSlider {
  fn NewQAbstractSlider(self) -> QAbstractSlider;
}

  // proto:  void QAbstractSlider::QAbstractSlider(QWidget * parent);
impl<'a> /*trait*/ QAbstractSlider_NewQAbstractSlider for (QWidget) {
  fn NewQAbstractSlider(self) -> QAbstractSlider {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSliderC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QAbstractSliderC1EP7QWidget(qthis, arg0)};
    let rsthis = QAbstractSlider{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setRange(int min, int max);
impl /*struct*/ QAbstractSlider {
  pub fn setRange<RetType, T: QAbstractSlider_setRange<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setRange<RetType> {
  fn setRange(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setRange(int min, int max);
impl<'a> /*trait*/ QAbstractSlider_setRange<()> for (i32, i32) {
  fn setRange(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN15QAbstractSlider8setRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setMinimum(int );
impl /*struct*/ QAbstractSlider {
  pub fn setMinimum<RetType, T: QAbstractSlider_setMinimum<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMinimum(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setMinimum<RetType> {
  fn setMinimum(self , rsthis: &mut QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setMinimum(int );
impl<'a> /*trait*/ QAbstractSlider_setMinimum<()> for (i32) {
  fn setMinimum(self , rsthis: &mut QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider10setMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider10setMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

