// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QString QProgressBar::format();
  fn _ZNK12QProgressBar6formatEv() -> i32;
  // proto: void QProgressBar::reset();
  fn _ZN12QProgressBar5resetEv() -> i32;
  // proto: const QMetaObject * QProgressBar::metaObject();
  fn _ZNK12QProgressBar10metaObjectEv() -> i32;
  // proto: int QProgressBar::maximum();
  fn _ZNK12QProgressBar7maximumEv() -> i32;
  // proto: void QProgressBar::setFormat(const QString & format);
  fn _ZN12QProgressBar9setFormatERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QProgressBar::invertedAppearance();
  fn _ZNK12QProgressBar18invertedAppearanceEv() -> i32;
  // proto: QString QProgressBar::text();
  fn _ZNK12QProgressBar4textEv() -> i32;
  // proto: void QProgressBar::setMinimum(int minimum);
  fn _ZN12QProgressBar10setMinimumEi(arg0: c_int) -> i32;
  // proto: void QProgressBar::NewQProgressBar(const QProgressBar & );
  fn _ZN12QProgressBarC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QProgressBar::valueChanged(int value);
  fn _ZN12QProgressBar12valueChangedEi(arg0: c_int) -> i32;
  // proto: void QProgressBar::NewQProgressBar(QWidget * parent);
  fn _ZN12QProgressBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QProgressBar::setTextVisible(bool visible);
  fn _ZN12QProgressBar14setTextVisibleEb(arg0: int8_t) -> i32;
  // proto: int QProgressBar::value();
  fn _ZNK12QProgressBar5valueEv() -> i32;
  // proto: void QProgressBar::setValue(int value);
  fn _ZN12QProgressBar8setValueEi(arg0: c_int) -> i32;
  // proto: QSize QProgressBar::minimumSizeHint();
  fn _ZNK12QProgressBar15minimumSizeHintEv() -> i32;
  // proto: int QProgressBar::minimum();
  fn _ZNK12QProgressBar7minimumEv() -> i32;
  // proto: void QProgressBar::setRange(int minimum, int maximum);
  fn _ZN12QProgressBar8setRangeEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QSize QProgressBar::sizeHint();
  fn _ZNK12QProgressBar8sizeHintEv() -> i32;
  // proto: void QProgressBar::resetFormat();
  fn _ZN12QProgressBar11resetFormatEv() -> i32;
  // proto: bool QProgressBar::isTextVisible();
  fn _ZNK12QProgressBar13isTextVisibleEv() -> i32;
  // proto: void QProgressBar::setInvertedAppearance(bool invert);
  fn _ZN12QProgressBar21setInvertedAppearanceEb(arg0: int8_t) -> i32;
  // proto: void QProgressBar::FreeQProgressBar();
  fn _ZN12QProgressBarD0Ev() -> i32;
  // proto: void QProgressBar::setMaximum(int maximum);
  fn _ZN12QProgressBar10setMaximumEi(arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QProgressBar)=1
pub struct QProgressBar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QProgressBar {
  pub fn format<T: QProgressBar_format>(&mut self, value: T) -> i32 {
    value.format(self);
    return 1;
  }
}

pub trait QProgressBar_format {
  fn format(self, this: &mut QProgressBar) -> i32;
}

// proto: QString QProgressBar::format();
impl<'a> /*trait*/ QProgressBar_format for () {
  fn format(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar6formatEv()};
    unsafe {_ZNK12QProgressBar6formatEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn reset<T: QProgressBar_reset>(&mut self, value: T) -> i32 {
    value.reset(self);
    return 1;
  }
}

pub trait QProgressBar_reset {
  fn reset(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::reset();
impl<'a> /*trait*/ QProgressBar_reset for () {
  fn reset(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar5resetEv()};
    unsafe {_ZN12QProgressBar5resetEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn metaObject<T: QProgressBar_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QProgressBar_metaObject {
  fn metaObject(self, this: &mut QProgressBar) -> i32;
}

// proto: const QMetaObject * QProgressBar::metaObject();
impl<'a> /*trait*/ QProgressBar_metaObject for () {
  fn metaObject(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar10metaObjectEv()};
    unsafe {_ZNK12QProgressBar10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn maximum<T: QProgressBar_maximum>(&mut self, value: T) -> i32 {
    value.maximum(self);
    return 1;
  }
}

pub trait QProgressBar_maximum {
  fn maximum(self, this: &mut QProgressBar) -> i32;
}

// proto: int QProgressBar::maximum();
impl<'a> /*trait*/ QProgressBar_maximum for () {
  fn maximum(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar7maximumEv()};
    unsafe {_ZNK12QProgressBar7maximumEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setFormat<T: QProgressBar_setFormat>(&mut self, value: T) -> i32 {
    value.setFormat(self);
    return 1;
  }
}

pub trait QProgressBar_setFormat {
  fn setFormat(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::setFormat(const QString & format);
impl<'a> /*trait*/ QProgressBar_setFormat for (&'a  QString) {
  fn setFormat(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar9setFormatERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QProgressBar9setFormatERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn invertedAppearance<T: QProgressBar_invertedAppearance>(&mut self, value: T) -> i32 {
    value.invertedAppearance(self);
    return 1;
  }
}

pub trait QProgressBar_invertedAppearance {
  fn invertedAppearance(self, this: &mut QProgressBar) -> i32;
}

// proto: bool QProgressBar::invertedAppearance();
impl<'a> /*trait*/ QProgressBar_invertedAppearance for () {
  fn invertedAppearance(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar18invertedAppearanceEv()};
    unsafe {_ZNK12QProgressBar18invertedAppearanceEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn text<T: QProgressBar_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QProgressBar_text {
  fn text(self, this: &mut QProgressBar) -> i32;
}

// proto: QString QProgressBar::text();
impl<'a> /*trait*/ QProgressBar_text for () {
  fn text(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar4textEv()};
    unsafe {_ZNK12QProgressBar4textEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setMinimum<T: QProgressBar_setMinimum>(&mut self, value: T) -> i32 {
    value.setMinimum(self);
    return 1;
  }
}

pub trait QProgressBar_setMinimum {
  fn setMinimum(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::setMinimum(int minimum);
impl<'a> /*trait*/ QProgressBar_setMinimum for (i32) {
  fn setMinimum(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar10setMinimumEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QProgressBar10setMinimumEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn NewQProgressBar<T: QProgressBar_NewQProgressBar>(value: T) -> QProgressBar {
    let rsthis = value.NewQProgressBar();
    return rsthis;
    // return 1;
  }
}

pub trait QProgressBar_NewQProgressBar {
  fn NewQProgressBar(self) -> QProgressBar;
}

// proto: void QProgressBar::NewQProgressBar(const QProgressBar & );
impl<'a> /*trait*/ QProgressBar_NewQProgressBar for (&'a  QProgressBar) {
  fn NewQProgressBar(self) -> QProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBarC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN12QProgressBarC1ERKS_(qthis, arg0)};
    let rsthis = QProgressBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn valueChanged<T: QProgressBar_valueChanged>(&mut self, value: T) -> i32 {
    value.valueChanged(self);
    return 1;
  }
}

pub trait QProgressBar_valueChanged {
  fn valueChanged(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::valueChanged(int value);
impl<'a> /*trait*/ QProgressBar_valueChanged for (i32) {
  fn valueChanged(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar12valueChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QProgressBar12valueChangedEi(arg0)};
    return 1;
  }
}

// proto: void QProgressBar::NewQProgressBar(QWidget * parent);
impl<'a> /*trait*/ QProgressBar_NewQProgressBar for (&'a mut QWidget) {
  fn NewQProgressBar(self) -> QProgressBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBarC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QProgressBarC1EP7QWidget(qthis, arg0)};
    let rsthis = QProgressBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setTextVisible<T: QProgressBar_setTextVisible>(&mut self, value: T) -> i32 {
    value.setTextVisible(self);
    return 1;
  }
}

pub trait QProgressBar_setTextVisible {
  fn setTextVisible(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::setTextVisible(bool visible);
impl<'a> /*trait*/ QProgressBar_setTextVisible for (i8) {
  fn setTextVisible(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar14setTextVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QProgressBar14setTextVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn value<T: QProgressBar_value>(&mut self, value: T) -> i32 {
    value.value(self);
    return 1;
  }
}

pub trait QProgressBar_value {
  fn value(self, this: &mut QProgressBar) -> i32;
}

// proto: int QProgressBar::value();
impl<'a> /*trait*/ QProgressBar_value for () {
  fn value(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar5valueEv()};
    unsafe {_ZNK12QProgressBar5valueEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setValue<T: QProgressBar_setValue>(&mut self, value: T) -> i32 {
    value.setValue(self);
    return 1;
  }
}

pub trait QProgressBar_setValue {
  fn setValue(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::setValue(int value);
impl<'a> /*trait*/ QProgressBar_setValue for (i32) {
  fn setValue(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar8setValueEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QProgressBar8setValueEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn minimumSizeHint<T: QProgressBar_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QProgressBar_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QProgressBar) -> i32;
}

// proto: QSize QProgressBar::minimumSizeHint();
impl<'a> /*trait*/ QProgressBar_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar15minimumSizeHintEv()};
    unsafe {_ZNK12QProgressBar15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn minimum<T: QProgressBar_minimum>(&mut self, value: T) -> i32 {
    value.minimum(self);
    return 1;
  }
}

pub trait QProgressBar_minimum {
  fn minimum(self, this: &mut QProgressBar) -> i32;
}

// proto: int QProgressBar::minimum();
impl<'a> /*trait*/ QProgressBar_minimum for () {
  fn minimum(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar7minimumEv()};
    unsafe {_ZNK12QProgressBar7minimumEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setRange<T: QProgressBar_setRange>(&mut self, value: T) -> i32 {
    value.setRange(self);
    return 1;
  }
}

pub trait QProgressBar_setRange {
  fn setRange(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::setRange(int minimum, int maximum);
impl<'a> /*trait*/ QProgressBar_setRange for (i32, i32) {
  fn setRange(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN12QProgressBar8setRangeEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn sizeHint<T: QProgressBar_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QProgressBar_sizeHint {
  fn sizeHint(self, this: &mut QProgressBar) -> i32;
}

// proto: QSize QProgressBar::sizeHint();
impl<'a> /*trait*/ QProgressBar_sizeHint for () {
  fn sizeHint(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar8sizeHintEv()};
    unsafe {_ZNK12QProgressBar8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn resetFormat<T: QProgressBar_resetFormat>(&mut self, value: T) -> i32 {
    value.resetFormat(self);
    return 1;
  }
}

pub trait QProgressBar_resetFormat {
  fn resetFormat(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::resetFormat();
impl<'a> /*trait*/ QProgressBar_resetFormat for () {
  fn resetFormat(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar11resetFormatEv()};
    unsafe {_ZN12QProgressBar11resetFormatEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn isTextVisible<T: QProgressBar_isTextVisible>(&mut self, value: T) -> i32 {
    value.isTextVisible(self);
    return 1;
  }
}

pub trait QProgressBar_isTextVisible {
  fn isTextVisible(self, this: &mut QProgressBar) -> i32;
}

// proto: bool QProgressBar::isTextVisible();
impl<'a> /*trait*/ QProgressBar_isTextVisible for () {
  fn isTextVisible(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar13isTextVisibleEv()};
    unsafe {_ZNK12QProgressBar13isTextVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setInvertedAppearance<T: QProgressBar_setInvertedAppearance>(&mut self, value: T) -> i32 {
    value.setInvertedAppearance(self);
    return 1;
  }
}

pub trait QProgressBar_setInvertedAppearance {
  fn setInvertedAppearance(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::setInvertedAppearance(bool invert);
impl<'a> /*trait*/ QProgressBar_setInvertedAppearance for (i8) {
  fn setInvertedAppearance(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar21setInvertedAppearanceEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN12QProgressBar21setInvertedAppearanceEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn FreeQProgressBar<T: QProgressBar_FreeQProgressBar>(&mut self, value: T) -> i32 {
    value.FreeQProgressBar(self);
    return 1;
  }
}

pub trait QProgressBar_FreeQProgressBar {
  fn FreeQProgressBar(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::FreeQProgressBar();
impl<'a> /*trait*/ QProgressBar_FreeQProgressBar for () {
  fn FreeQProgressBar(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBarD0Ev()};
    unsafe {_ZN12QProgressBarD0Ev()};
    return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setMaximum<T: QProgressBar_setMaximum>(&mut self, value: T) -> i32 {
    value.setMaximum(self);
    return 1;
  }
}

pub trait QProgressBar_setMaximum {
  fn setMaximum(self, this: &mut QProgressBar) -> i32;
}

// proto: void QProgressBar::setMaximum(int maximum);
impl<'a> /*trait*/ QProgressBar_setMaximum for (i32) {
  fn setMaximum(self, this: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar10setMaximumEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN12QProgressBar10setMaximumEi(arg0)};
    return 1;
  }
}

