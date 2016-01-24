// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qprogressbar.h
// dst-file: /src/widgets/qprogressbar.rs
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
use super::super::core::qstring::QString; // 771
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QProgressBar_Class_Size() -> c_int;
  // proto:  QString QProgressBar::format();
  fn C_ZNK12QProgressBar6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QProgressBar::reset();
  fn C_ZN12QProgressBar5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QProgressBar::metaObject();
  fn C_ZNK12QProgressBar10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QProgressBar::maximum();
  fn C_ZNK12QProgressBar7maximumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QProgressBar::setFormat(const QString & format);
  fn C_ZN12QProgressBar9setFormatERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QProgressBar::invertedAppearance();
  fn C_ZNK12QProgressBar18invertedAppearanceEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QProgressBar::text();
  fn C_ZNK12QProgressBar4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QProgressBar::setMinimum(int minimum);
  fn C_ZN12QProgressBar10setMinimumEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QProgressBar::QProgressBar(QWidget * parent);
  fn C_ZN12QProgressBarC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QProgressBar::setTextVisible(bool visible);
  fn C_ZN12QProgressBar14setTextVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QProgressBar::value();
  fn C_ZNK12QProgressBar5valueEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QProgressBar::setValue(int value);
  fn C_ZN12QProgressBar8setValueEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QSize QProgressBar::minimumSizeHint();
  fn C_ZNK12QProgressBar15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QProgressBar::minimum();
  fn C_ZNK12QProgressBar7minimumEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QProgressBar::setRange(int minimum, int maximum);
  fn C_ZN12QProgressBar8setRangeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QSize QProgressBar::sizeHint();
  fn C_ZNK12QProgressBar8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QProgressBar::resetFormat();
  fn C_ZN12QProgressBar11resetFormatEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QProgressBar::isTextVisible();
  fn C_ZNK12QProgressBar13isTextVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QProgressBar::setInvertedAppearance(bool invert);
  fn C_ZN12QProgressBar21setInvertedAppearanceEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QProgressBar::~QProgressBar();
  fn C_ZN12QProgressBarD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QProgressBar::setMaximum(int maximum);
  fn C_ZN12QProgressBar10setMaximumEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  fn QProgressBar_SlotProxy_connect__ZN12QProgressBar12valueChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QProgressBar)=1
#[derive(Default)]
pub struct QProgressBar {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _valueChanged: QProgressBar_valueChanged_signal,
}

impl /*struct*/ QProgressBar {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QProgressBar {
    return QProgressBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QProgressBar {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QProgressBar {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  QString QProgressBar::format();
impl /*struct*/ QProgressBar {
  pub fn format<RetType, T: QProgressBar_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QProgressBar_format<RetType> {
  fn format(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  QString QProgressBar::format();
impl<'a> /*trait*/ QProgressBar_format<QString> for () {
  fn format(self , rsthis: & QProgressBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar6formatEv()};
    let mut ret = unsafe {C_ZNK12QProgressBar6formatEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProgressBar::reset();
impl /*struct*/ QProgressBar {
  pub fn reset<RetType, T: QProgressBar_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QProgressBar_reset<RetType> {
  fn reset(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::reset();
impl<'a> /*trait*/ QProgressBar_reset<()> for () {
  fn reset(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar5resetEv()};
     unsafe {C_ZN12QProgressBar5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QProgressBar::metaObject();
impl /*struct*/ QProgressBar {
  pub fn metaObject<RetType, T: QProgressBar_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QProgressBar_metaObject<RetType> {
  fn metaObject(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  const QMetaObject * QProgressBar::metaObject();
impl<'a> /*trait*/ QProgressBar_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QProgressBar) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar10metaObjectEv()};
    let mut ret = unsafe {C_ZNK12QProgressBar10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QProgressBar::maximum();
impl /*struct*/ QProgressBar {
  pub fn maximum<RetType, T: QProgressBar_maximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximum(self);
    // return 1;
  }
}

pub trait QProgressBar_maximum<RetType> {
  fn maximum(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  int QProgressBar::maximum();
impl<'a> /*trait*/ QProgressBar_maximum<i32> for () {
  fn maximum(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar7maximumEv()};
    let mut ret = unsafe {C_ZNK12QProgressBar7maximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProgressBar::setFormat(const QString & format);
impl /*struct*/ QProgressBar {
  pub fn setFormat<RetType, T: QProgressBar_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QProgressBar_setFormat<RetType> {
  fn setFormat(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setFormat(const QString & format);
impl<'a> /*trait*/ QProgressBar_setFormat<()> for (&'a QString) {
  fn setFormat(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar9setFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN12QProgressBar9setFormatERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QProgressBar::invertedAppearance();
impl /*struct*/ QProgressBar {
  pub fn invertedAppearance<RetType, T: QProgressBar_invertedAppearance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invertedAppearance(self);
    // return 1;
  }
}

pub trait QProgressBar_invertedAppearance<RetType> {
  fn invertedAppearance(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  bool QProgressBar::invertedAppearance();
impl<'a> /*trait*/ QProgressBar_invertedAppearance<i8> for () {
  fn invertedAppearance(self , rsthis: & QProgressBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar18invertedAppearanceEv()};
    let mut ret = unsafe {C_ZNK12QProgressBar18invertedAppearanceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QProgressBar::text();
impl /*struct*/ QProgressBar {
  pub fn text<RetType, T: QProgressBar_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QProgressBar_text<RetType> {
  fn text(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  QString QProgressBar::text();
impl<'a> /*trait*/ QProgressBar_text<QString> for () {
  fn text(self , rsthis: & QProgressBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar4textEv()};
    let mut ret = unsafe {C_ZNK12QProgressBar4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProgressBar::setMinimum(int minimum);
impl /*struct*/ QProgressBar {
  pub fn setMinimum<RetType, T: QProgressBar_setMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimum(self);
    // return 1;
  }
}

pub trait QProgressBar_setMinimum<RetType> {
  fn setMinimum(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setMinimum(int minimum);
impl<'a> /*trait*/ QProgressBar_setMinimum<()> for (i32) {
  fn setMinimum(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar10setMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QProgressBar10setMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProgressBar::QProgressBar(QWidget * parent);
impl /*struct*/ QProgressBar {
  pub fn new<T: QProgressBar_new>(value: T) -> QProgressBar {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QProgressBar_new {
  fn new(self) -> QProgressBar;
}

  // proto:  void QProgressBar::QProgressBar(QWidget * parent);
impl<'a> /*trait*/ QProgressBar_new for (&'a QWidget) {
  fn new(self) -> QProgressBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBarC2EP7QWidget()};
    let ctysz: c_int = unsafe{QProgressBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QProgressBarC2EP7QWidget(arg0)};
    let rsthis = QProgressBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QProgressBar::setTextVisible(bool visible);
impl /*struct*/ QProgressBar {
  pub fn setTextVisible<RetType, T: QProgressBar_setTextVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextVisible(self);
    // return 1;
  }
}

pub trait QProgressBar_setTextVisible<RetType> {
  fn setTextVisible(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setTextVisible(bool visible);
impl<'a> /*trait*/ QProgressBar_setTextVisible<()> for (i8) {
  fn setTextVisible(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar14setTextVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QProgressBar14setTextVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QProgressBar::value();
impl /*struct*/ QProgressBar {
  pub fn value<RetType, T: QProgressBar_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QProgressBar_value<RetType> {
  fn value(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  int QProgressBar::value();
impl<'a> /*trait*/ QProgressBar_value<i32> for () {
  fn value(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar5valueEv()};
    let mut ret = unsafe {C_ZNK12QProgressBar5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProgressBar::setValue(int value);
impl /*struct*/ QProgressBar {
  pub fn setValue<RetType, T: QProgressBar_setValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QProgressBar_setValue<RetType> {
  fn setValue(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setValue(int value);
impl<'a> /*trait*/ QProgressBar_setValue<()> for (i32) {
  fn setValue(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar8setValueEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QProgressBar8setValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QProgressBar::minimumSizeHint();
impl /*struct*/ QProgressBar {
  pub fn minimumSizeHint<RetType, T: QProgressBar_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QProgressBar_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  QSize QProgressBar::minimumSizeHint();
impl<'a> /*trait*/ QProgressBar_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QProgressBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar15minimumSizeHintEv()};
    let mut ret = unsafe {C_ZNK12QProgressBar15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QProgressBar::minimum();
impl /*struct*/ QProgressBar {
  pub fn minimum<RetType, T: QProgressBar_minimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimum(self);
    // return 1;
  }
}

pub trait QProgressBar_minimum<RetType> {
  fn minimum(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  int QProgressBar::minimum();
impl<'a> /*trait*/ QProgressBar_minimum<i32> for () {
  fn minimum(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar7minimumEv()};
    let mut ret = unsafe {C_ZNK12QProgressBar7minimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProgressBar::setRange(int minimum, int maximum);
impl /*struct*/ QProgressBar {
  pub fn setRange<RetType, T: QProgressBar_setRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QProgressBar_setRange<RetType> {
  fn setRange(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setRange(int minimum, int maximum);
impl<'a> /*trait*/ QProgressBar_setRange<()> for (i32, i32) {
  fn setRange(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN12QProgressBar8setRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QProgressBar::sizeHint();
impl /*struct*/ QProgressBar {
  pub fn sizeHint<RetType, T: QProgressBar_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QProgressBar_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  QSize QProgressBar::sizeHint();
impl<'a> /*trait*/ QProgressBar_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QProgressBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar8sizeHintEv()};
    let mut ret = unsafe {C_ZNK12QProgressBar8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProgressBar::resetFormat();
impl /*struct*/ QProgressBar {
  pub fn resetFormat<RetType, T: QProgressBar_resetFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetFormat(self);
    // return 1;
  }
}

pub trait QProgressBar_resetFormat<RetType> {
  fn resetFormat(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::resetFormat();
impl<'a> /*trait*/ QProgressBar_resetFormat<()> for () {
  fn resetFormat(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar11resetFormatEv()};
     unsafe {C_ZN12QProgressBar11resetFormatEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QProgressBar::isTextVisible();
impl /*struct*/ QProgressBar {
  pub fn isTextVisible<RetType, T: QProgressBar_isTextVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTextVisible(self);
    // return 1;
  }
}

pub trait QProgressBar_isTextVisible<RetType> {
  fn isTextVisible(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  bool QProgressBar::isTextVisible();
impl<'a> /*trait*/ QProgressBar_isTextVisible<i8> for () {
  fn isTextVisible(self , rsthis: & QProgressBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar13isTextVisibleEv()};
    let mut ret = unsafe {C_ZNK12QProgressBar13isTextVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QProgressBar::setInvertedAppearance(bool invert);
impl /*struct*/ QProgressBar {
  pub fn setInvertedAppearance<RetType, T: QProgressBar_setInvertedAppearance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInvertedAppearance(self);
    // return 1;
  }
}

pub trait QProgressBar_setInvertedAppearance<RetType> {
  fn setInvertedAppearance(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setInvertedAppearance(bool invert);
impl<'a> /*trait*/ QProgressBar_setInvertedAppearance<()> for (i8) {
  fn setInvertedAppearance(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar21setInvertedAppearanceEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN12QProgressBar21setInvertedAppearanceEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProgressBar::~QProgressBar();
impl /*struct*/ QProgressBar {
  pub fn free<RetType, T: QProgressBar_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QProgressBar_free<RetType> {
  fn free(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::~QProgressBar();
impl<'a> /*trait*/ QProgressBar_free<()> for () {
  fn free(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBarD2Ev()};
     unsafe {C_ZN12QProgressBarD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProgressBar::setMaximum(int maximum);
impl /*struct*/ QProgressBar {
  pub fn setMaximum<RetType, T: QProgressBar_setMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximum(self);
    // return 1;
  }
}

pub trait QProgressBar_setMaximum<RetType> {
  fn setMaximum(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setMaximum(int maximum);
impl<'a> /*trait*/ QProgressBar_setMaximum<()> for (i32) {
  fn setMaximum(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar10setMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN12QProgressBar10setMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QProgressBar_valueChanged
pub struct QProgressBar_valueChanged_signal{poi:u64}
impl /* struct */ QProgressBar {
  pub fn valueChanged(&self) -> QProgressBar_valueChanged_signal {
     return QProgressBar_valueChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QProgressBar_valueChanged_signal {
  pub fn connect<T: QProgressBar_valueChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QProgressBar_valueChanged_signal_connect {
  fn connect(self, sigthis: QProgressBar_valueChanged_signal);
}

// valueChanged(int)
extern fn QProgressBar_valueChanged_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QProgressBar_valueChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QProgressBar_valueChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QProgressBar_valueChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QProgressBar_valueChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QProgressBar_SlotProxy_connect__ZN12QProgressBar12valueChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QProgressBar_valueChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QProgressBar_valueChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QProgressBar_valueChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QProgressBar_SlotProxy_connect__ZN12QProgressBar12valueChangedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

