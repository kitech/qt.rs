// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QStatusBar::FreeQStatusBar();
  fn _ZN10QStatusBarD0Ev() -> i32;
  // proto: int QStatusBar::insertPermanentWidget(int index, QWidget * widget, int stretch);
  fn _ZN10QStatusBar21insertPermanentWidgetEiP7QWidgeti(arg0: c_int, arg1: *mut c_void, arg2: c_int) -> i32;
  // proto: void QStatusBar::removeWidget(QWidget * widget);
  fn _ZN10QStatusBar12removeWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QStatusBar::NewQStatusBar(const QStatusBar & );
  fn _ZN10QStatusBarC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QStatusBar::setSizeGripEnabled(bool );
  fn _ZN10QStatusBar18setSizeGripEnabledEb(arg0: int8_t) -> i32;
  // proto: void QStatusBar::addPermanentWidget(QWidget * widget, int stretch);
  fn _ZN10QStatusBar18addPermanentWidgetEP7QWidgeti(arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: bool QStatusBar::isSizeGripEnabled();
  fn _ZNK10QStatusBar17isSizeGripEnabledEv() -> i32;
  // proto: void QStatusBar::clearMessage();
  fn _ZN10QStatusBar12clearMessageEv() -> i32;
  // proto: QString QStatusBar::currentMessage();
  fn _ZNK10QStatusBar14currentMessageEv() -> i32;
  // proto: const QMetaObject * QStatusBar::metaObject();
  fn _ZNK10QStatusBar10metaObjectEv() -> i32;
  // proto: void QStatusBar::messageChanged(const QString & text);
  fn _ZN10QStatusBar14messageChangedERK7QString(arg0: *const c_void) -> i32;
  // proto: void QStatusBar::showMessage(const QString & text, int timeout);
  fn _ZN10QStatusBar11showMessageERK7QStringi(arg0: *const c_void, arg1: c_int) -> i32;
  // proto: int QStatusBar::insertWidget(int index, QWidget * widget, int stretch);
  fn _ZN10QStatusBar12insertWidgetEiP7QWidgeti(arg0: c_int, arg1: *mut c_void, arg2: c_int) -> i32;
  // proto: void QStatusBar::addWidget(QWidget * widget, int stretch);
  fn _ZN10QStatusBar9addWidgetEP7QWidgeti(arg0: *mut c_void, arg1: c_int) -> i32;
  // proto: void QStatusBar::NewQStatusBar(QWidget * parent);
  fn _ZN10QStatusBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
}

// body block begin
// class sizeof(QStatusBar)=1
pub struct QStatusBar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStatusBar {
  pub fn FreeQStatusBar<T: QStatusBar_FreeQStatusBar>(&mut self, value: T) -> i32 {
    value.FreeQStatusBar(self);
    return 1;
  }
}

pub trait QStatusBar_FreeQStatusBar {
  fn FreeQStatusBar(self, this: &mut QStatusBar) -> i32;
}

// proto: void QStatusBar::FreeQStatusBar();
impl<'a> /*trait*/ QStatusBar_FreeQStatusBar for () {
  fn FreeQStatusBar(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBarD0Ev()};
    unsafe {_ZN10QStatusBarD0Ev()};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn insertPermanentWidget<T: QStatusBar_insertPermanentWidget>(&mut self, value: T) -> i32 {
    value.insertPermanentWidget(self);
    return 1;
  }
}

pub trait QStatusBar_insertPermanentWidget {
  fn insertPermanentWidget(self, this: &mut QStatusBar) -> i32;
}

// proto: int QStatusBar::insertPermanentWidget(int index, QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_insertPermanentWidget for (i32, &'a mut QWidget, i32) {
  fn insertPermanentWidget(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar21insertPermanentWidgetEiP7QWidgeti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN10QStatusBar21insertPermanentWidgetEiP7QWidgeti(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn removeWidget<T: QStatusBar_removeWidget>(&mut self, value: T) -> i32 {
    value.removeWidget(self);
    return 1;
  }
}

pub trait QStatusBar_removeWidget {
  fn removeWidget(self, this: &mut QStatusBar) -> i32;
}

// proto: void QStatusBar::removeWidget(QWidget * widget);
impl<'a> /*trait*/ QStatusBar_removeWidget for (&'a mut QWidget) {
  fn removeWidget(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar12removeWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QStatusBar12removeWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn NewQStatusBar<T: QStatusBar_NewQStatusBar>(value: T) -> QStatusBar {
    let rsthis = value.NewQStatusBar();
    return rsthis;
    // return 1;
  }
}

pub trait QStatusBar_NewQStatusBar {
  fn NewQStatusBar(self) -> QStatusBar;
}

// proto: void QStatusBar::NewQStatusBar(const QStatusBar & );
impl<'a> /*trait*/ QStatusBar_NewQStatusBar for (&'a  QStatusBar) {
  fn NewQStatusBar(self) -> QStatusBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBarC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QStatusBarC1ERKS_(qthis, arg0)};
    let rsthis = QStatusBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn setSizeGripEnabled<T: QStatusBar_setSizeGripEnabled>(&mut self, value: T) -> i32 {
    value.setSizeGripEnabled(self);
    return 1;
  }
}

pub trait QStatusBar_setSizeGripEnabled {
  fn setSizeGripEnabled(self, this: &mut QStatusBar) -> i32;
}

// proto: void QStatusBar::setSizeGripEnabled(bool );
impl<'a> /*trait*/ QStatusBar_setSizeGripEnabled for (i8) {
  fn setSizeGripEnabled(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar18setSizeGripEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QStatusBar18setSizeGripEnabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn addPermanentWidget<T: QStatusBar_addPermanentWidget>(&mut self, value: T) -> i32 {
    value.addPermanentWidget(self);
    return 1;
  }
}

pub trait QStatusBar_addPermanentWidget {
  fn addPermanentWidget(self, this: &mut QStatusBar) -> i32;
}

// proto: void QStatusBar::addPermanentWidget(QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_addPermanentWidget for (&'a mut QWidget, i32) {
  fn addPermanentWidget(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar18addPermanentWidgetEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QStatusBar18addPermanentWidgetEP7QWidgeti(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn isSizeGripEnabled<T: QStatusBar_isSizeGripEnabled>(&mut self, value: T) -> i32 {
    value.isSizeGripEnabled(self);
    return 1;
  }
}

pub trait QStatusBar_isSizeGripEnabled {
  fn isSizeGripEnabled(self, this: &mut QStatusBar) -> i32;
}

// proto: bool QStatusBar::isSizeGripEnabled();
impl<'a> /*trait*/ QStatusBar_isSizeGripEnabled for () {
  fn isSizeGripEnabled(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar17isSizeGripEnabledEv()};
    unsafe {_ZNK10QStatusBar17isSizeGripEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn clearMessage<T: QStatusBar_clearMessage>(&mut self, value: T) -> i32 {
    value.clearMessage(self);
    return 1;
  }
}

pub trait QStatusBar_clearMessage {
  fn clearMessage(self, this: &mut QStatusBar) -> i32;
}

// proto: void QStatusBar::clearMessage();
impl<'a> /*trait*/ QStatusBar_clearMessage for () {
  fn clearMessage(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar12clearMessageEv()};
    unsafe {_ZN10QStatusBar12clearMessageEv()};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn currentMessage<T: QStatusBar_currentMessage>(&mut self, value: T) -> i32 {
    value.currentMessage(self);
    return 1;
  }
}

pub trait QStatusBar_currentMessage {
  fn currentMessage(self, this: &mut QStatusBar) -> i32;
}

// proto: QString QStatusBar::currentMessage();
impl<'a> /*trait*/ QStatusBar_currentMessage for () {
  fn currentMessage(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar14currentMessageEv()};
    unsafe {_ZNK10QStatusBar14currentMessageEv()};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn metaObject<T: QStatusBar_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QStatusBar_metaObject {
  fn metaObject(self, this: &mut QStatusBar) -> i32;
}

// proto: const QMetaObject * QStatusBar::metaObject();
impl<'a> /*trait*/ QStatusBar_metaObject for () {
  fn metaObject(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar10metaObjectEv()};
    unsafe {_ZNK10QStatusBar10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn messageChanged<T: QStatusBar_messageChanged>(&mut self, value: T) -> i32 {
    value.messageChanged(self);
    return 1;
  }
}

pub trait QStatusBar_messageChanged {
  fn messageChanged(self, this: &mut QStatusBar) -> i32;
}

// proto: void QStatusBar::messageChanged(const QString & text);
impl<'a> /*trait*/ QStatusBar_messageChanged for (&'a  QString) {
  fn messageChanged(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar14messageChangedERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QStatusBar14messageChangedERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn showMessage<T: QStatusBar_showMessage>(&mut self, value: T) -> i32 {
    value.showMessage(self);
    return 1;
  }
}

pub trait QStatusBar_showMessage {
  fn showMessage(self, this: &mut QStatusBar) -> i32;
}

// proto: void QStatusBar::showMessage(const QString & text, int timeout);
impl<'a> /*trait*/ QStatusBar_showMessage for (&'a  QString, i32) {
  fn showMessage(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar11showMessageERK7QStringi()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QStatusBar11showMessageERK7QStringi(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn insertWidget<T: QStatusBar_insertWidget>(&mut self, value: T) -> i32 {
    value.insertWidget(self);
    return 1;
  }
}

pub trait QStatusBar_insertWidget {
  fn insertWidget(self, this: &mut QStatusBar) -> i32;
}

// proto: int QStatusBar::insertWidget(int index, QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_insertWidget for (i32, &'a mut QWidget, i32) {
  fn insertWidget(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar12insertWidgetEiP7QWidgeti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    unsafe {_ZN10QStatusBar12insertWidgetEiP7QWidgeti(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn addWidget<T: QStatusBar_addWidget>(&mut self, value: T) -> i32 {
    value.addWidget(self);
    return 1;
  }
}

pub trait QStatusBar_addWidget {
  fn addWidget(self, this: &mut QStatusBar) -> i32;
}

// proto: void QStatusBar::addWidget(QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_addWidget for (&'a mut QWidget, i32) {
  fn addWidget(self, this: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar9addWidgetEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN10QStatusBar9addWidgetEP7QWidgeti(arg0, arg1)};
    return 1;
  }
}

// proto: void QStatusBar::NewQStatusBar(QWidget * parent);
impl<'a> /*trait*/ QStatusBar_NewQStatusBar for (&'a mut QWidget) {
  fn NewQStatusBar(self) -> QStatusBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBarC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QStatusBarC1EP7QWidget(qthis, arg0)};
    let rsthis = QStatusBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

