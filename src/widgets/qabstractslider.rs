// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
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
use std::ops::Deref;
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractSlider_Class_Size() -> c_int;
  // proto:  void QAbstractSlider::setSliderPosition(int );
  fn _ZN15QAbstractSlider17setSliderPositionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QAbstractSlider::isSliderDown();
  fn _ZNK15QAbstractSlider12isSliderDownEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QAbstractSlider::value();
  fn _ZNK15QAbstractSlider5valueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QAbstractSlider::setInvertedControls(bool );
  fn _ZN15QAbstractSlider19setInvertedControlsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QAbstractSlider::QAbstractSlider(const QAbstractSlider & );
  fn _ZN15QAbstractSliderC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QAbstractSlider::minimum();
  fn _ZNK15QAbstractSlider7minimumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QAbstractSlider::singleStep();
  fn _ZNK15QAbstractSlider10singleStepEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QAbstractSlider::pageStep();
  fn _ZNK15QAbstractSlider8pageStepEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QAbstractSlider::setMaximum(int );
  fn _ZN15QAbstractSlider10setMaximumEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QAbstractSlider::invertedControls();
  fn _ZNK15QAbstractSlider16invertedControlsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractSlider::setValue(int );
  fn _ZN15QAbstractSlider8setValueEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QAbstractSlider::~QAbstractSlider();
  fn _ZN15QAbstractSliderD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractSlider::setPageStep(int );
  fn _ZN15QAbstractSlider11setPageStepEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QAbstractSlider::setSliderDown(bool );
  fn _ZN15QAbstractSlider13setSliderDownEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QAbstractSlider::maximum();
  fn _ZNK15QAbstractSlider7maximumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QMetaObject * QAbstractSlider::metaObject();
  fn _ZNK15QAbstractSlider10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractSlider::setSingleStep(int );
  fn _ZN15QAbstractSlider13setSingleStepEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QAbstractSlider::setInvertedAppearance(bool );
  fn _ZN15QAbstractSlider21setInvertedAppearanceEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QAbstractSlider::hasTracking();
  fn _ZNK15QAbstractSlider11hasTrackingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QAbstractSlider::invertedAppearance();
  fn _ZNK15QAbstractSlider18invertedAppearanceEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QAbstractSlider::sliderPosition();
  fn _ZNK15QAbstractSlider14sliderPositionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QAbstractSlider::setTracking(bool enable);
  fn _ZN15QAbstractSlider11setTrackingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QAbstractSlider::QAbstractSlider(QWidget * parent);
  fn _ZN15QAbstractSliderC2EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractSlider::setRange(int min, int max);
  fn _ZN15QAbstractSlider8setRangeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QAbstractSlider::setMinimum(int );
  fn _ZN15QAbstractSlider10setMinimumEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider13sliderPressedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider15actionTriggeredEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider11sliderMovedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider12rangeChangedEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider12valueChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider14sliderReleasedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractSlider)=1
#[derive(Default)]
pub struct QAbstractSlider {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _sliderReleased: QAbstractSlider_sliderReleased_signal,
  pub _rangeChanged: QAbstractSlider_rangeChanged_signal,
  pub _sliderPressed: QAbstractSlider_sliderPressed_signal,
  pub _actionTriggered: QAbstractSlider_actionTriggered_signal,
  pub _valueChanged: QAbstractSlider_valueChanged_signal,
  pub _sliderMoved: QAbstractSlider_sliderMoved_signal,
}

impl /*struct*/ QAbstractSlider {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractSlider {
    return QAbstractSlider{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractSlider {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QAbstractSlider {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QAbstractSlider::setSliderPosition(int );
impl /*struct*/ QAbstractSlider {
  pub fn setSliderPosition<RetType, T: QAbstractSlider_setSliderPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSliderPosition(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setSliderPosition<RetType> {
  fn setSliderPosition(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setSliderPosition(int );
impl<'a> /*trait*/ QAbstractSlider_setSliderPosition<()> for (i32) {
  fn setSliderPosition(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider17setSliderPositionEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider17setSliderPositionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractSlider::isSliderDown();
impl /*struct*/ QAbstractSlider {
  pub fn isSliderDown<RetType, T: QAbstractSlider_isSliderDown<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSliderDown(self);
    // return 1;
  }
}

pub trait QAbstractSlider_isSliderDown<RetType> {
  fn isSliderDown(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  bool QAbstractSlider::isSliderDown();
impl<'a> /*trait*/ QAbstractSlider_isSliderDown<i8> for () {
  fn isSliderDown(self , rsthis: & QAbstractSlider) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider12isSliderDownEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider12isSliderDownEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QAbstractSlider::value();
impl /*struct*/ QAbstractSlider {
  pub fn value<RetType, T: QAbstractSlider_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QAbstractSlider_value<RetType> {
  fn value(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::value();
impl<'a> /*trait*/ QAbstractSlider_value<i32> for () {
  fn value(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider5valueEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setInvertedControls(bool );
impl /*struct*/ QAbstractSlider {
  pub fn setInvertedControls<RetType, T: QAbstractSlider_setInvertedControls<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInvertedControls(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setInvertedControls<RetType> {
  fn setInvertedControls(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setInvertedControls(bool );
impl<'a> /*trait*/ QAbstractSlider_setInvertedControls<()> for (i8) {
  fn setInvertedControls(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider19setInvertedControlsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractSlider19setInvertedControlsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::QAbstractSlider(const QAbstractSlider & );
impl /*struct*/ QAbstractSlider {
  pub fn new<T: QAbstractSlider_new>(value: T) -> QAbstractSlider {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractSlider_new {
  fn new(self) -> QAbstractSlider;
}

  // proto:  void QAbstractSlider::QAbstractSlider(const QAbstractSlider & );
impl<'a> /*trait*/ QAbstractSlider_new for (&'a QAbstractSlider) {
  fn new(self) -> QAbstractSlider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSliderC2ERKS_()};
    let ctysz: c_int = unsafe{QAbstractSlider_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QAbstractSliderC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QAbstractSlider{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QAbstractSlider::minimum();
impl /*struct*/ QAbstractSlider {
  pub fn minimum<RetType, T: QAbstractSlider_minimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimum(self);
    // return 1;
  }
}

pub trait QAbstractSlider_minimum<RetType> {
  fn minimum(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::minimum();
impl<'a> /*trait*/ QAbstractSlider_minimum<i32> for () {
  fn minimum(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider7minimumEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider7minimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QAbstractSlider::singleStep();
impl /*struct*/ QAbstractSlider {
  pub fn singleStep<RetType, T: QAbstractSlider_singleStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.singleStep(self);
    // return 1;
  }
}

pub trait QAbstractSlider_singleStep<RetType> {
  fn singleStep(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::singleStep();
impl<'a> /*trait*/ QAbstractSlider_singleStep<i32> for () {
  fn singleStep(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider10singleStepEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider10singleStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QAbstractSlider::pageStep();
impl /*struct*/ QAbstractSlider {
  pub fn pageStep<RetType, T: QAbstractSlider_pageStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pageStep(self);
    // return 1;
  }
}

pub trait QAbstractSlider_pageStep<RetType> {
  fn pageStep(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::pageStep();
impl<'a> /*trait*/ QAbstractSlider_pageStep<i32> for () {
  fn pageStep(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider8pageStepEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider8pageStepEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setMaximum(int );
impl /*struct*/ QAbstractSlider {
  pub fn setMaximum<RetType, T: QAbstractSlider_setMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximum(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setMaximum<RetType> {
  fn setMaximum(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setMaximum(int );
impl<'a> /*trait*/ QAbstractSlider_setMaximum<()> for (i32) {
  fn setMaximum(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider10setMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider10setMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractSlider::invertedControls();
impl /*struct*/ QAbstractSlider {
  pub fn invertedControls<RetType, T: QAbstractSlider_invertedControls<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invertedControls(self);
    // return 1;
  }
}

pub trait QAbstractSlider_invertedControls<RetType> {
  fn invertedControls(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  bool QAbstractSlider::invertedControls();
impl<'a> /*trait*/ QAbstractSlider_invertedControls<i8> for () {
  fn invertedControls(self , rsthis: & QAbstractSlider) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider16invertedControlsEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider16invertedControlsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setValue(int );
impl /*struct*/ QAbstractSlider {
  pub fn setValue<RetType, T: QAbstractSlider_setValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setValue<RetType> {
  fn setValue(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setValue(int );
impl<'a> /*trait*/ QAbstractSlider_setValue<()> for (i32) {
  fn setValue(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider8setValueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider8setValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::~QAbstractSlider();
impl /*struct*/ QAbstractSlider {
  pub fn free<RetType, T: QAbstractSlider_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractSlider_free<RetType> {
  fn free(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::~QAbstractSlider();
impl<'a> /*trait*/ QAbstractSlider_free<()> for () {
  fn free(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSliderD2Ev()};
     unsafe {_ZN15QAbstractSliderD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setPageStep(int );
impl /*struct*/ QAbstractSlider {
  pub fn setPageStep<RetType, T: QAbstractSlider_setPageStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPageStep(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setPageStep<RetType> {
  fn setPageStep(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setPageStep(int );
impl<'a> /*trait*/ QAbstractSlider_setPageStep<()> for (i32) {
  fn setPageStep(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider11setPageStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider11setPageStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setSliderDown(bool );
impl /*struct*/ QAbstractSlider {
  pub fn setSliderDown<RetType, T: QAbstractSlider_setSliderDown<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSliderDown(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setSliderDown<RetType> {
  fn setSliderDown(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setSliderDown(bool );
impl<'a> /*trait*/ QAbstractSlider_setSliderDown<()> for (i8) {
  fn setSliderDown(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider13setSliderDownEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractSlider13setSliderDownEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QAbstractSlider::maximum();
impl /*struct*/ QAbstractSlider {
  pub fn maximum<RetType, T: QAbstractSlider_maximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximum(self);
    // return 1;
  }
}

pub trait QAbstractSlider_maximum<RetType> {
  fn maximum(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::maximum();
impl<'a> /*trait*/ QAbstractSlider_maximum<i32> for () {
  fn maximum(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider7maximumEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider7maximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractSlider::metaObject();
impl /*struct*/ QAbstractSlider {
  pub fn metaObject<RetType, T: QAbstractSlider_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractSlider_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  const QMetaObject * QAbstractSlider::metaObject();
impl<'a> /*trait*/ QAbstractSlider_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider10metaObjectEv()};
     unsafe {_ZNK15QAbstractSlider10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setSingleStep(int );
impl /*struct*/ QAbstractSlider {
  pub fn setSingleStep<RetType, T: QAbstractSlider_setSingleStep<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSingleStep(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setSingleStep<RetType> {
  fn setSingleStep(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setSingleStep(int );
impl<'a> /*trait*/ QAbstractSlider_setSingleStep<()> for (i32) {
  fn setSingleStep(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider13setSingleStepEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider13setSingleStepEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setInvertedAppearance(bool );
impl /*struct*/ QAbstractSlider {
  pub fn setInvertedAppearance<RetType, T: QAbstractSlider_setInvertedAppearance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInvertedAppearance(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setInvertedAppearance<RetType> {
  fn setInvertedAppearance(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setInvertedAppearance(bool );
impl<'a> /*trait*/ QAbstractSlider_setInvertedAppearance<()> for (i8) {
  fn setInvertedAppearance(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider21setInvertedAppearanceEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractSlider21setInvertedAppearanceEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractSlider::hasTracking();
impl /*struct*/ QAbstractSlider {
  pub fn hasTracking<RetType, T: QAbstractSlider_hasTracking<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasTracking(self);
    // return 1;
  }
}

pub trait QAbstractSlider_hasTracking<RetType> {
  fn hasTracking(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  bool QAbstractSlider::hasTracking();
impl<'a> /*trait*/ QAbstractSlider_hasTracking<i8> for () {
  fn hasTracking(self , rsthis: & QAbstractSlider) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider11hasTrackingEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider11hasTrackingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractSlider::invertedAppearance();
impl /*struct*/ QAbstractSlider {
  pub fn invertedAppearance<RetType, T: QAbstractSlider_invertedAppearance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invertedAppearance(self);
    // return 1;
  }
}

pub trait QAbstractSlider_invertedAppearance<RetType> {
  fn invertedAppearance(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  bool QAbstractSlider::invertedAppearance();
impl<'a> /*trait*/ QAbstractSlider_invertedAppearance<i8> for () {
  fn invertedAppearance(self , rsthis: & QAbstractSlider) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider18invertedAppearanceEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider18invertedAppearanceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QAbstractSlider::sliderPosition();
impl /*struct*/ QAbstractSlider {
  pub fn sliderPosition<RetType, T: QAbstractSlider_sliderPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sliderPosition(self);
    // return 1;
  }
}

pub trait QAbstractSlider_sliderPosition<RetType> {
  fn sliderPosition(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  int QAbstractSlider::sliderPosition();
impl<'a> /*trait*/ QAbstractSlider_sliderPosition<i32> for () {
  fn sliderPosition(self , rsthis: & QAbstractSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QAbstractSlider14sliderPositionEv()};
    let mut ret = unsafe {_ZNK15QAbstractSlider14sliderPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setTracking(bool enable);
impl /*struct*/ QAbstractSlider {
  pub fn setTracking<RetType, T: QAbstractSlider_setTracking<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTracking(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setTracking<RetType> {
  fn setTracking(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setTracking(bool enable);
impl<'a> /*trait*/ QAbstractSlider_setTracking<()> for (i8) {
  fn setTracking(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider11setTrackingEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QAbstractSlider11setTrackingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSlider::QAbstractSlider(QWidget * parent);
impl<'a> /*trait*/ QAbstractSlider_new for (&'a QWidget) {
  fn new(self) -> QAbstractSlider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSliderC2EP7QWidget()};
    let ctysz: c_int = unsafe{QAbstractSlider_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QAbstractSliderC2EP7QWidget(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QAbstractSlider{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractSlider::setRange(int min, int max);
impl /*struct*/ QAbstractSlider {
  pub fn setRange<RetType, T: QAbstractSlider_setRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setRange<RetType> {
  fn setRange(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setRange(int min, int max);
impl<'a> /*trait*/ QAbstractSlider_setRange<()> for (i32, i32) {
  fn setRange(self , rsthis: & QAbstractSlider) -> () {
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
  pub fn setMinimum<RetType, T: QAbstractSlider_setMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimum(self);
    // return 1;
  }
}

pub trait QAbstractSlider_setMinimum<RetType> {
  fn setMinimum(self , rsthis: & QAbstractSlider) -> RetType;
}

  // proto:  void QAbstractSlider::setMinimum(int );
impl<'a> /*trait*/ QAbstractSlider_setMinimum<()> for (i32) {
  fn setMinimum(self , rsthis: & QAbstractSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QAbstractSlider10setMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN15QAbstractSlider10setMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QAbstractSlider_sliderReleased
pub struct QAbstractSlider_sliderReleased_signal{poi:u64}
impl /* struct */ QAbstractSlider {
  pub fn sliderReleased(&self) -> QAbstractSlider_sliderReleased_signal {
     return QAbstractSlider_sliderReleased_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSlider_sliderReleased_signal {
  pub fn connect<T: QAbstractSlider_sliderReleased_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSlider_sliderReleased_signal_connect {
  fn connect(self, sigthis: QAbstractSlider_sliderReleased_signal);
}

#[derive(Default)] // for QAbstractSlider_rangeChanged
pub struct QAbstractSlider_rangeChanged_signal{poi:u64}
impl /* struct */ QAbstractSlider {
  pub fn rangeChanged(&self) -> QAbstractSlider_rangeChanged_signal {
     return QAbstractSlider_rangeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSlider_rangeChanged_signal {
  pub fn connect<T: QAbstractSlider_rangeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSlider_rangeChanged_signal_connect {
  fn connect(self, sigthis: QAbstractSlider_rangeChanged_signal);
}

#[derive(Default)] // for QAbstractSlider_sliderPressed
pub struct QAbstractSlider_sliderPressed_signal{poi:u64}
impl /* struct */ QAbstractSlider {
  pub fn sliderPressed(&self) -> QAbstractSlider_sliderPressed_signal {
     return QAbstractSlider_sliderPressed_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSlider_sliderPressed_signal {
  pub fn connect<T: QAbstractSlider_sliderPressed_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSlider_sliderPressed_signal_connect {
  fn connect(self, sigthis: QAbstractSlider_sliderPressed_signal);
}

#[derive(Default)] // for QAbstractSlider_actionTriggered
pub struct QAbstractSlider_actionTriggered_signal{poi:u64}
impl /* struct */ QAbstractSlider {
  pub fn actionTriggered(&self) -> QAbstractSlider_actionTriggered_signal {
     return QAbstractSlider_actionTriggered_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSlider_actionTriggered_signal {
  pub fn connect<T: QAbstractSlider_actionTriggered_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSlider_actionTriggered_signal_connect {
  fn connect(self, sigthis: QAbstractSlider_actionTriggered_signal);
}

#[derive(Default)] // for QAbstractSlider_valueChanged
pub struct QAbstractSlider_valueChanged_signal{poi:u64}
impl /* struct */ QAbstractSlider {
  pub fn valueChanged(&self) -> QAbstractSlider_valueChanged_signal {
     return QAbstractSlider_valueChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSlider_valueChanged_signal {
  pub fn connect<T: QAbstractSlider_valueChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSlider_valueChanged_signal_connect {
  fn connect(self, sigthis: QAbstractSlider_valueChanged_signal);
}

#[derive(Default)] // for QAbstractSlider_sliderMoved
pub struct QAbstractSlider_sliderMoved_signal{poi:u64}
impl /* struct */ QAbstractSlider {
  pub fn sliderMoved(&self) -> QAbstractSlider_sliderMoved_signal {
     return QAbstractSlider_sliderMoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSlider_sliderMoved_signal {
  pub fn connect<T: QAbstractSlider_sliderMoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSlider_sliderMoved_signal_connect {
  fn connect(self, sigthis: QAbstractSlider_sliderMoved_signal);
}

// sliderPressed()
extern fn QAbstractSlider_sliderPressed_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QAbstractSlider_sliderPressed_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QAbstractSlider_sliderPressed_signal_connect for fn() {
  fn connect(self, sigthis: QAbstractSlider_sliderPressed_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_sliderPressed_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider13sliderPressedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSlider_sliderPressed_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QAbstractSlider_sliderPressed_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_sliderPressed_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider13sliderPressedEv(arg0, arg1, arg2)};
  }
}
// actionTriggered(int)
extern fn QAbstractSlider_actionTriggered_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QAbstractSlider_actionTriggered_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractSlider_actionTriggered_signal_connect for fn(i32) {
  fn connect(self, sigthis: QAbstractSlider_actionTriggered_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_actionTriggered_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider15actionTriggeredEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSlider_actionTriggered_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QAbstractSlider_actionTriggered_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_actionTriggered_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider15actionTriggeredEi(arg0, arg1, arg2)};
  }
}
// sliderMoved(int)
extern fn QAbstractSlider_sliderMoved_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QAbstractSlider_sliderMoved_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractSlider_sliderMoved_signal_connect for fn(i32) {
  fn connect(self, sigthis: QAbstractSlider_sliderMoved_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_sliderMoved_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider11sliderMovedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSlider_sliderMoved_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QAbstractSlider_sliderMoved_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_sliderMoved_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider11sliderMovedEi(arg0, arg1, arg2)};
  }
}
// rangeChanged(int, int)
extern fn QAbstractSlider_rangeChanged_signal_connect_cb_3(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QAbstractSlider_rangeChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QAbstractSlider_rangeChanged_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QAbstractSlider_rangeChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_rangeChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider12rangeChangedEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSlider_rangeChanged_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QAbstractSlider_rangeChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_rangeChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider12rangeChangedEii(arg0, arg1, arg2)};
  }
}
// valueChanged(int)
extern fn QAbstractSlider_valueChanged_signal_connect_cb_4(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QAbstractSlider_valueChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QAbstractSlider_valueChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QAbstractSlider_valueChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_valueChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider12valueChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSlider_valueChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QAbstractSlider_valueChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_valueChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider12valueChangedEi(arg0, arg1, arg2)};
  }
}
// sliderReleased()
extern fn QAbstractSlider_sliderReleased_signal_connect_cb_5(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QAbstractSlider_sliderReleased_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QAbstractSlider_sliderReleased_signal_connect for fn() {
  fn connect(self, sigthis: QAbstractSlider_sliderReleased_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_sliderReleased_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider14sliderReleasedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSlider_sliderReleased_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QAbstractSlider_sliderReleased_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSlider_sliderReleased_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSlider_SlotProxy_connect__ZN15QAbstractSlider14sliderReleasedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

