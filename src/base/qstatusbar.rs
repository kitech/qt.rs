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

// proto:  void QStatusBar::FreeQStatusBar();
impl /*struct*/ QStatusBar {
  pub fn FreeQStatusBar<RetType, T: QStatusBar_FreeQStatusBar<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQStatusBar(self);
    // return 1;
  }
}

pub trait QStatusBar_FreeQStatusBar<RetType> {
  fn FreeQStatusBar(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  void QStatusBar::FreeQStatusBar();
impl<'a> /*trait*/ QStatusBar_FreeQStatusBar<()> for () {
  fn FreeQStatusBar(self , rsthis: &mut QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBarD0Ev()};
     unsafe {_ZN10QStatusBarD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QStatusBar::insertPermanentWidget(int index, QWidget * widget, int stretch);
impl /*struct*/ QStatusBar {
  pub fn insertPermanentWidget<RetType, T: QStatusBar_insertPermanentWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertPermanentWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_insertPermanentWidget<RetType> {
  fn insertPermanentWidget(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  int QStatusBar::insertPermanentWidget(int index, QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_insertPermanentWidget<i32> for (i32, &'a mut QWidget, i32) {
  fn insertPermanentWidget(self , rsthis: &mut QStatusBar) -> i32 {
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

// proto:  void QStatusBar::removeWidget(QWidget * widget);
impl /*struct*/ QStatusBar {
  pub fn removeWidget<RetType, T: QStatusBar_removeWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.removeWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_removeWidget<RetType> {
  fn removeWidget(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  void QStatusBar::removeWidget(QWidget * widget);
impl<'a> /*trait*/ QStatusBar_removeWidget<()> for (&'a mut QWidget) {
  fn removeWidget(self , rsthis: &mut QStatusBar) -> () {
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

// proto:  void QStatusBar::setSizeGripEnabled(bool );
impl /*struct*/ QStatusBar {
  pub fn setSizeGripEnabled<RetType, T: QStatusBar_setSizeGripEnabled<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QStatusBar_setSizeGripEnabled<RetType> {
  fn setSizeGripEnabled(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  void QStatusBar::setSizeGripEnabled(bool );
impl<'a> /*trait*/ QStatusBar_setSizeGripEnabled<()> for (i8) {
  fn setSizeGripEnabled(self , rsthis: &mut QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar18setSizeGripEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QStatusBar18setSizeGripEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStatusBar::addPermanentWidget(QWidget * widget, int stretch);
impl /*struct*/ QStatusBar {
  pub fn addPermanentWidget<RetType, T: QStatusBar_addPermanentWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addPermanentWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_addPermanentWidget<RetType> {
  fn addPermanentWidget(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  void QStatusBar::addPermanentWidget(QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_addPermanentWidget<()> for (&'a mut QWidget, i32) {
  fn addPermanentWidget(self , rsthis: &mut QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar18addPermanentWidgetEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QStatusBar18addPermanentWidgetEP7QWidgeti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  bool QStatusBar::isSizeGripEnabled();
impl /*struct*/ QStatusBar {
  pub fn isSizeGripEnabled<RetType, T: QStatusBar_isSizeGripEnabled<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.isSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QStatusBar_isSizeGripEnabled<RetType> {
  fn isSizeGripEnabled(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  bool QStatusBar::isSizeGripEnabled();
impl<'a> /*trait*/ QStatusBar_isSizeGripEnabled<i8> for () {
  fn isSizeGripEnabled(self , rsthis: &mut QStatusBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar17isSizeGripEnabledEv()};
    let mut ret = unsafe {_ZNK10QStatusBar17isSizeGripEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QStatusBar::clearMessage();
impl /*struct*/ QStatusBar {
  pub fn clearMessage<RetType, T: QStatusBar_clearMessage<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.clearMessage(self);
    // return 1;
  }
}

pub trait QStatusBar_clearMessage<RetType> {
  fn clearMessage(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  void QStatusBar::clearMessage();
impl<'a> /*trait*/ QStatusBar_clearMessage<()> for () {
  fn clearMessage(self , rsthis: &mut QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar12clearMessageEv()};
     unsafe {_ZN10QStatusBar12clearMessageEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  QString QStatusBar::currentMessage();
impl /*struct*/ QStatusBar {
  pub fn currentMessage<RetType, T: QStatusBar_currentMessage<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.currentMessage(self);
    // return 1;
  }
}

pub trait QStatusBar_currentMessage<RetType> {
  fn currentMessage(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  QString QStatusBar::currentMessage();
impl<'a> /*trait*/ QStatusBar_currentMessage<QString> for () {
  fn currentMessage(self , rsthis: &mut QStatusBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar14currentMessageEv()};
    let mut ret = unsafe {_ZNK10QStatusBar14currentMessageEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  const QMetaObject * QStatusBar::metaObject();
impl /*struct*/ QStatusBar {
  pub fn metaObject<RetType, T: QStatusBar_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStatusBar_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  const QMetaObject * QStatusBar::metaObject();
impl<'a> /*trait*/ QStatusBar_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar10metaObjectEv()};
     unsafe {_ZNK10QStatusBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QStatusBar::messageChanged(const QString & text);
impl /*struct*/ QStatusBar {
  pub fn messageChanged<RetType, T: QStatusBar_messageChanged<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.messageChanged(self);
    // return 1;
  }
}

pub trait QStatusBar_messageChanged<RetType> {
  fn messageChanged(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  void QStatusBar::messageChanged(const QString & text);
impl<'a> /*trait*/ QStatusBar_messageChanged<()> for (&'a  QString) {
  fn messageChanged(self , rsthis: &mut QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar14messageChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QStatusBar14messageChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QStatusBar::showMessage(const QString & text, int timeout);
impl /*struct*/ QStatusBar {
  pub fn showMessage<RetType, T: QStatusBar_showMessage<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.showMessage(self);
    // return 1;
  }
}

pub trait QStatusBar_showMessage<RetType> {
  fn showMessage(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  void QStatusBar::showMessage(const QString & text, int timeout);
impl<'a> /*trait*/ QStatusBar_showMessage<()> for (&'a  QString, i32) {
  fn showMessage(self , rsthis: &mut QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar11showMessageERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QStatusBar11showMessageERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  int QStatusBar::insertWidget(int index, QWidget * widget, int stretch);
impl /*struct*/ QStatusBar {
  pub fn insertWidget<RetType, T: QStatusBar_insertWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.insertWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_insertWidget<RetType> {
  fn insertWidget(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  int QStatusBar::insertWidget(int index, QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_insertWidget<i32> for (i32, &'a mut QWidget, i32) {
  fn insertWidget(self , rsthis: &mut QStatusBar) -> i32 {
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

// proto:  void QStatusBar::addWidget(QWidget * widget, int stretch);
impl /*struct*/ QStatusBar {
  pub fn addWidget<RetType, T: QStatusBar_addWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.addWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_addWidget<RetType> {
  fn addWidget(self , rsthis: &mut QStatusBar) -> RetType;
}

// proto:  void QStatusBar::addWidget(QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_addWidget<()> for (&'a mut QWidget, i32) {
  fn addWidget(self , rsthis: &mut QStatusBar) -> () {
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

