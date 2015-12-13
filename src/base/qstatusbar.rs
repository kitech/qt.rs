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
  // proto:  void QStatusBar::FreeQStatusBar();
  fn _ZN10QStatusBarD0Ev(qthis: *mut c_void) ;
  // proto:  int QStatusBar::insertPermanentWidget(int index, QWidget * widget, int stretch);
  fn _ZN10QStatusBar21insertPermanentWidgetEiP7QWidgeti(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) -> c_int;
  // proto:  void QStatusBar::removeWidget(QWidget * widget);
  fn _ZN10QStatusBar12removeWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStatusBar::NewQStatusBar(const QStatusBar & );
  fn _ZN10QStatusBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStatusBar::setSizeGripEnabled(bool );
  fn _ZN10QStatusBar18setSizeGripEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QStatusBar::addPermanentWidget(QWidget * widget, int stretch);
  fn _ZN10QStatusBar18addPermanentWidgetEP7QWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  bool QStatusBar::isSizeGripEnabled();
  fn _ZNK10QStatusBar17isSizeGripEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QStatusBar::clearMessage();
  fn _ZN10QStatusBar12clearMessageEv(qthis: *mut c_void) ;
  // proto:  QString QStatusBar::currentMessage();
  fn _ZNK10QStatusBar14currentMessageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QStatusBar::metaObject();
  fn _ZNK10QStatusBar10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QStatusBar::messageChanged(const QString & text);
  fn _ZN10QStatusBar14messageChangedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QStatusBar::showMessage(const QString & text, int timeout);
  fn _ZN10QStatusBar11showMessageERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  int QStatusBar::insertWidget(int index, QWidget * widget, int stretch);
  fn _ZN10QStatusBar12insertWidgetEiP7QWidgeti(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: c_int) -> c_int;
  // proto:  void QStatusBar::addWidget(QWidget * widget, int stretch);
  fn _ZN10QStatusBar9addWidgetEP7QWidgeti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QStatusBar::NewQStatusBar(QWidget * parent);
  fn _ZN10QStatusBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QStatusBar)=1
pub struct QStatusBar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStatusBar {
  pub fn FreeQStatusBar<T: QStatusBar_FreeQStatusBar>(&mut self, value: T)  {
     value.FreeQStatusBar(self);
    // return 1;
  }
}

pub trait QStatusBar_FreeQStatusBar {
  fn FreeQStatusBar(self, rsthis: &mut QStatusBar) ;
}

// proto:  void QStatusBar::FreeQStatusBar();
impl<'a> /*trait*/ QStatusBar_FreeQStatusBar for () {
  fn FreeQStatusBar(self, rsthis: &mut QStatusBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBarD0Ev()};
     unsafe {_ZN10QStatusBarD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn insertPermanentWidget<T: QStatusBar_insertPermanentWidget>(&mut self, value: T) -> i32 {
    return value.insertPermanentWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_insertPermanentWidget {
  fn insertPermanentWidget(self, rsthis: &mut QStatusBar) -> i32;
}

// proto:  int QStatusBar::insertPermanentWidget(int index, QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_insertPermanentWidget for (i32, &'a mut QWidget, i32) {
  fn insertPermanentWidget(self, rsthis: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar21insertPermanentWidgetEiP7QWidgeti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN10QStatusBar21insertPermanentWidgetEiP7QWidgeti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn removeWidget<T: QStatusBar_removeWidget>(&mut self, value: T)  {
     value.removeWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_removeWidget {
  fn removeWidget(self, rsthis: &mut QStatusBar) ;
}

// proto:  void QStatusBar::removeWidget(QWidget * widget);
impl<'a> /*trait*/ QStatusBar_removeWidget for (&'a mut QWidget) {
  fn removeWidget(self, rsthis: &mut QStatusBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar12removeWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QStatusBar12removeWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QStatusBarC1ERKS_(qthis, arg0)};
    let rsthis = QStatusBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn setSizeGripEnabled<T: QStatusBar_setSizeGripEnabled>(&mut self, value: T)  {
     value.setSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QStatusBar_setSizeGripEnabled {
  fn setSizeGripEnabled(self, rsthis: &mut QStatusBar) ;
}

// proto:  void QStatusBar::setSizeGripEnabled(bool );
impl<'a> /*trait*/ QStatusBar_setSizeGripEnabled for (i8) {
  fn setSizeGripEnabled(self, rsthis: &mut QStatusBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar18setSizeGripEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QStatusBar18setSizeGripEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn addPermanentWidget<T: QStatusBar_addPermanentWidget>(&mut self, value: T)  {
     value.addPermanentWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_addPermanentWidget {
  fn addPermanentWidget(self, rsthis: &mut QStatusBar) ;
}

// proto:  void QStatusBar::addPermanentWidget(QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_addPermanentWidget for (&'a mut QWidget, i32) {
  fn addPermanentWidget(self, rsthis: &mut QStatusBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar18addPermanentWidgetEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QStatusBar18addPermanentWidgetEP7QWidgeti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn isSizeGripEnabled<T: QStatusBar_isSizeGripEnabled>(&mut self, value: T) -> i8 {
    return value.isSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QStatusBar_isSizeGripEnabled {
  fn isSizeGripEnabled(self, rsthis: &mut QStatusBar) -> i8;
}

// proto:  bool QStatusBar::isSizeGripEnabled();
impl<'a> /*trait*/ QStatusBar_isSizeGripEnabled for () {
  fn isSizeGripEnabled(self, rsthis: &mut QStatusBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar17isSizeGripEnabledEv()};
    let mut ret = unsafe {_ZNK10QStatusBar17isSizeGripEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn clearMessage<T: QStatusBar_clearMessage>(&mut self, value: T)  {
     value.clearMessage(self);
    // return 1;
  }
}

pub trait QStatusBar_clearMessage {
  fn clearMessage(self, rsthis: &mut QStatusBar) ;
}

// proto:  void QStatusBar::clearMessage();
impl<'a> /*trait*/ QStatusBar_clearMessage for () {
  fn clearMessage(self, rsthis: &mut QStatusBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar12clearMessageEv()};
     unsafe {_ZN10QStatusBar12clearMessageEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn currentMessage<T: QStatusBar_currentMessage>(&mut self, value: T) -> QString {
    return value.currentMessage(self);
    // return 1;
  }
}

pub trait QStatusBar_currentMessage {
  fn currentMessage(self, rsthis: &mut QStatusBar) -> QString;
}

// proto:  QString QStatusBar::currentMessage();
impl<'a> /*trait*/ QStatusBar_currentMessage for () {
  fn currentMessage(self, rsthis: &mut QStatusBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar14currentMessageEv()};
    let mut ret = unsafe {_ZNK10QStatusBar14currentMessageEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn metaObject<T: QStatusBar_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QStatusBar_metaObject {
  fn metaObject(self, rsthis: &mut QStatusBar) ;
}

// proto:  const QMetaObject * QStatusBar::metaObject();
impl<'a> /*trait*/ QStatusBar_metaObject for () {
  fn metaObject(self, rsthis: &mut QStatusBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar10metaObjectEv()};
     unsafe {_ZNK10QStatusBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn messageChanged<T: QStatusBar_messageChanged>(&mut self, value: T)  {
     value.messageChanged(self);
    // return 1;
  }
}

pub trait QStatusBar_messageChanged {
  fn messageChanged(self, rsthis: &mut QStatusBar) ;
}

// proto:  void QStatusBar::messageChanged(const QString & text);
impl<'a> /*trait*/ QStatusBar_messageChanged for (&'a  QString) {
  fn messageChanged(self, rsthis: &mut QStatusBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar14messageChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QStatusBar14messageChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn showMessage<T: QStatusBar_showMessage>(&mut self, value: T)  {
     value.showMessage(self);
    // return 1;
  }
}

pub trait QStatusBar_showMessage {
  fn showMessage(self, rsthis: &mut QStatusBar) ;
}

// proto:  void QStatusBar::showMessage(const QString & text, int timeout);
impl<'a> /*trait*/ QStatusBar_showMessage for (&'a  QString, i32) {
  fn showMessage(self, rsthis: &mut QStatusBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar11showMessageERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QStatusBar11showMessageERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn insertWidget<T: QStatusBar_insertWidget>(&mut self, value: T) -> i32 {
    return value.insertWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_insertWidget {
  fn insertWidget(self, rsthis: &mut QStatusBar) -> i32;
}

// proto:  int QStatusBar::insertWidget(int index, QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_insertWidget for (i32, &'a mut QWidget, i32) {
  fn insertWidget(self, rsthis: &mut QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar12insertWidgetEiP7QWidgeti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN10QStatusBar12insertWidgetEiP7QWidgeti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QStatusBar {
  pub fn addWidget<T: QStatusBar_addWidget>(&mut self, value: T)  {
     value.addWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_addWidget {
  fn addWidget(self, rsthis: &mut QStatusBar) ;
}

// proto:  void QStatusBar::addWidget(QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_addWidget for (&'a mut QWidget, i32) {
  fn addWidget(self, rsthis: &mut QStatusBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar9addWidgetEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QStatusBar9addWidgetEP7QWidgeti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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

