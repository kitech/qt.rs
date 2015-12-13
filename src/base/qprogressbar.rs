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
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QProgressBar::format();
  fn _ZNK12QProgressBar6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProgressBar::reset();
  fn _ZN12QProgressBar5resetEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QProgressBar::metaObject();
  fn _ZNK12QProgressBar10metaObjectEv(qthis: *mut c_void) ;
  // proto:  int QProgressBar::maximum();
  fn _ZNK12QProgressBar7maximumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QProgressBar::setFormat(const QString & format);
  fn _ZN12QProgressBar9setFormatERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QProgressBar::invertedAppearance();
  fn _ZNK12QProgressBar18invertedAppearanceEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QProgressBar::text();
  fn _ZNK12QProgressBar4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProgressBar::setMinimum(int minimum);
  fn _ZN12QProgressBar10setMinimumEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QProgressBar::NewQProgressBar(const QProgressBar & );
  fn _ZN12QProgressBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProgressBar::valueChanged(int value);
  fn _ZN12QProgressBar12valueChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QProgressBar::NewQProgressBar(QWidget * parent);
  fn _ZN12QProgressBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QProgressBar::setTextVisible(bool visible);
  fn _ZN12QProgressBar14setTextVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QProgressBar::value();
  fn _ZNK12QProgressBar5valueEv(qthis: *mut c_void) -> c_int;
  // proto:  void QProgressBar::setValue(int value);
  fn _ZN12QProgressBar8setValueEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QSize QProgressBar::minimumSizeHint();
  fn _ZNK12QProgressBar15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QProgressBar::minimum();
  fn _ZNK12QProgressBar7minimumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QProgressBar::setRange(int minimum, int maximum);
  fn _ZN12QProgressBar8setRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QSize QProgressBar::sizeHint();
  fn _ZNK12QProgressBar8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProgressBar::resetFormat();
  fn _ZN12QProgressBar11resetFormatEv(qthis: *mut c_void) ;
  // proto:  bool QProgressBar::isTextVisible();
  fn _ZNK12QProgressBar13isTextVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QProgressBar::setInvertedAppearance(bool invert);
  fn _ZN12QProgressBar21setInvertedAppearanceEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QProgressBar::FreeQProgressBar();
  fn _ZN12QProgressBarD0Ev(qthis: *mut c_void) ;
  // proto:  void QProgressBar::setMaximum(int maximum);
  fn _ZN12QProgressBar10setMaximumEi(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QProgressBar)=1
pub struct QProgressBar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QProgressBar {
  pub fn format<T: QProgressBar_format>(&mut self, value: T) -> QString {
    return value.format(self);
    // return 1;
  }
}

pub trait QProgressBar_format {
  fn format(self, rsthis: &mut QProgressBar) -> QString;
}

// proto:  QString QProgressBar::format();
impl<'a> /*trait*/ QProgressBar_format for () {
  fn format(self, rsthis: &mut QProgressBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar6formatEv()};
    let mut ret = unsafe {_ZNK12QProgressBar6formatEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn reset<T: QProgressBar_reset>(&mut self, value: T)  {
     value.reset(self);
    // return 1;
  }
}

pub trait QProgressBar_reset {
  fn reset(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::reset();
impl<'a> /*trait*/ QProgressBar_reset for () {
  fn reset(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar5resetEv()};
     unsafe {_ZN12QProgressBar5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn metaObject<T: QProgressBar_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QProgressBar_metaObject {
  fn metaObject(self, rsthis: &mut QProgressBar) ;
}

// proto:  const QMetaObject * QProgressBar::metaObject();
impl<'a> /*trait*/ QProgressBar_metaObject for () {
  fn metaObject(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar10metaObjectEv()};
     unsafe {_ZNK12QProgressBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn maximum<T: QProgressBar_maximum>(&mut self, value: T) -> i32 {
    return value.maximum(self);
    // return 1;
  }
}

pub trait QProgressBar_maximum {
  fn maximum(self, rsthis: &mut QProgressBar) -> i32;
}

// proto:  int QProgressBar::maximum();
impl<'a> /*trait*/ QProgressBar_maximum for () {
  fn maximum(self, rsthis: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar7maximumEv()};
    let mut ret = unsafe {_ZNK12QProgressBar7maximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setFormat<T: QProgressBar_setFormat>(&mut self, value: T)  {
     value.setFormat(self);
    // return 1;
  }
}

pub trait QProgressBar_setFormat {
  fn setFormat(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::setFormat(const QString & format);
impl<'a> /*trait*/ QProgressBar_setFormat for (&'a  QString) {
  fn setFormat(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar9setFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QProgressBar9setFormatERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn invertedAppearance<T: QProgressBar_invertedAppearance>(&mut self, value: T) -> i8 {
    return value.invertedAppearance(self);
    // return 1;
  }
}

pub trait QProgressBar_invertedAppearance {
  fn invertedAppearance(self, rsthis: &mut QProgressBar) -> i8;
}

// proto:  bool QProgressBar::invertedAppearance();
impl<'a> /*trait*/ QProgressBar_invertedAppearance for () {
  fn invertedAppearance(self, rsthis: &mut QProgressBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar18invertedAppearanceEv()};
    let mut ret = unsafe {_ZNK12QProgressBar18invertedAppearanceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn text<T: QProgressBar_text>(&mut self, value: T) -> QString {
    return value.text(self);
    // return 1;
  }
}

pub trait QProgressBar_text {
  fn text(self, rsthis: &mut QProgressBar) -> QString;
}

// proto:  QString QProgressBar::text();
impl<'a> /*trait*/ QProgressBar_text for () {
  fn text(self, rsthis: &mut QProgressBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar4textEv()};
    let mut ret = unsafe {_ZNK12QProgressBar4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setMinimum<T: QProgressBar_setMinimum>(&mut self, value: T)  {
     value.setMinimum(self);
    // return 1;
  }
}

pub trait QProgressBar_setMinimum {
  fn setMinimum(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::setMinimum(int minimum);
impl<'a> /*trait*/ QProgressBar_setMinimum for (i32) {
  fn setMinimum(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar10setMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QProgressBar10setMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QProgressBarC1ERKS_(qthis, arg0)};
    let rsthis = QProgressBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn valueChanged<T: QProgressBar_valueChanged>(&mut self, value: T)  {
     value.valueChanged(self);
    // return 1;
  }
}

pub trait QProgressBar_valueChanged {
  fn valueChanged(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::valueChanged(int value);
impl<'a> /*trait*/ QProgressBar_valueChanged for (i32) {
  fn valueChanged(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar12valueChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QProgressBar12valueChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
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
  pub fn setTextVisible<T: QProgressBar_setTextVisible>(&mut self, value: T)  {
     value.setTextVisible(self);
    // return 1;
  }
}

pub trait QProgressBar_setTextVisible {
  fn setTextVisible(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::setTextVisible(bool visible);
impl<'a> /*trait*/ QProgressBar_setTextVisible for (i8) {
  fn setTextVisible(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar14setTextVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QProgressBar14setTextVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn value<T: QProgressBar_value>(&mut self, value: T) -> i32 {
    return value.value(self);
    // return 1;
  }
}

pub trait QProgressBar_value {
  fn value(self, rsthis: &mut QProgressBar) -> i32;
}

// proto:  int QProgressBar::value();
impl<'a> /*trait*/ QProgressBar_value for () {
  fn value(self, rsthis: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar5valueEv()};
    let mut ret = unsafe {_ZNK12QProgressBar5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setValue<T: QProgressBar_setValue>(&mut self, value: T)  {
     value.setValue(self);
    // return 1;
  }
}

pub trait QProgressBar_setValue {
  fn setValue(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::setValue(int value);
impl<'a> /*trait*/ QProgressBar_setValue for (i32) {
  fn setValue(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar8setValueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QProgressBar8setValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn minimumSizeHint<T: QProgressBar_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QProgressBar_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QProgressBar) -> QSize;
}

// proto:  QSize QProgressBar::minimumSizeHint();
impl<'a> /*trait*/ QProgressBar_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QProgressBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK12QProgressBar15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn minimum<T: QProgressBar_minimum>(&mut self, value: T) -> i32 {
    return value.minimum(self);
    // return 1;
  }
}

pub trait QProgressBar_minimum {
  fn minimum(self, rsthis: &mut QProgressBar) -> i32;
}

// proto:  int QProgressBar::minimum();
impl<'a> /*trait*/ QProgressBar_minimum for () {
  fn minimum(self, rsthis: &mut QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar7minimumEv()};
    let mut ret = unsafe {_ZNK12QProgressBar7minimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setRange<T: QProgressBar_setRange>(&mut self, value: T)  {
     value.setRange(self);
    // return 1;
  }
}

pub trait QProgressBar_setRange {
  fn setRange(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::setRange(int minimum, int maximum);
impl<'a> /*trait*/ QProgressBar_setRange for (i32, i32) {
  fn setRange(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QProgressBar8setRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn sizeHint<T: QProgressBar_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QProgressBar_sizeHint {
  fn sizeHint(self, rsthis: &mut QProgressBar) -> QSize;
}

// proto:  QSize QProgressBar::sizeHint();
impl<'a> /*trait*/ QProgressBar_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QProgressBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar8sizeHintEv()};
    let mut ret = unsafe {_ZNK12QProgressBar8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn resetFormat<T: QProgressBar_resetFormat>(&mut self, value: T)  {
     value.resetFormat(self);
    // return 1;
  }
}

pub trait QProgressBar_resetFormat {
  fn resetFormat(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::resetFormat();
impl<'a> /*trait*/ QProgressBar_resetFormat for () {
  fn resetFormat(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar11resetFormatEv()};
     unsafe {_ZN12QProgressBar11resetFormatEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn isTextVisible<T: QProgressBar_isTextVisible>(&mut self, value: T) -> i8 {
    return value.isTextVisible(self);
    // return 1;
  }
}

pub trait QProgressBar_isTextVisible {
  fn isTextVisible(self, rsthis: &mut QProgressBar) -> i8;
}

// proto:  bool QProgressBar::isTextVisible();
impl<'a> /*trait*/ QProgressBar_isTextVisible for () {
  fn isTextVisible(self, rsthis: &mut QProgressBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar13isTextVisibleEv()};
    let mut ret = unsafe {_ZNK12QProgressBar13isTextVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setInvertedAppearance<T: QProgressBar_setInvertedAppearance>(&mut self, value: T)  {
     value.setInvertedAppearance(self);
    // return 1;
  }
}

pub trait QProgressBar_setInvertedAppearance {
  fn setInvertedAppearance(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::setInvertedAppearance(bool invert);
impl<'a> /*trait*/ QProgressBar_setInvertedAppearance for (i8) {
  fn setInvertedAppearance(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar21setInvertedAppearanceEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QProgressBar21setInvertedAppearanceEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn FreeQProgressBar<T: QProgressBar_FreeQProgressBar>(&mut self, value: T)  {
     value.FreeQProgressBar(self);
    // return 1;
  }
}

pub trait QProgressBar_FreeQProgressBar {
  fn FreeQProgressBar(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::FreeQProgressBar();
impl<'a> /*trait*/ QProgressBar_FreeQProgressBar for () {
  fn FreeQProgressBar(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBarD0Ev()};
     unsafe {_ZN12QProgressBarD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QProgressBar {
  pub fn setMaximum<T: QProgressBar_setMaximum>(&mut self, value: T)  {
     value.setMaximum(self);
    // return 1;
  }
}

pub trait QProgressBar_setMaximum {
  fn setMaximum(self, rsthis: &mut QProgressBar) ;
}

// proto:  void QProgressBar::setMaximum(int maximum);
impl<'a> /*trait*/ QProgressBar_setMaximum for (i32) {
  fn setMaximum(self, rsthis: &mut QProgressBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar10setMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QProgressBar10setMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

