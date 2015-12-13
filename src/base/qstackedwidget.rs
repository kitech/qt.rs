// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QStackedWidget::setCurrentIndex(int index);
  fn _ZN14QStackedWidget15setCurrentIndexEi(arg0: c_int) -> i32;
  // proto: void QStackedWidget::NewQStackedWidget(QWidget * parent);
  fn _ZN14QStackedWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QWidget * QStackedWidget::currentWidget();
  fn _ZNK14QStackedWidget13currentWidgetEv() -> i32;
  // proto: void QStackedWidget::widgetRemoved(int index);
  fn _ZN14QStackedWidget13widgetRemovedEi(arg0: c_int) -> i32;
  // proto: int QStackedWidget::insertWidget(int index, QWidget * w);
  fn _ZN14QStackedWidget12insertWidgetEiP7QWidget(arg0: c_int, arg1: *mut c_void) -> i32;
  // proto: int QStackedWidget::indexOf(QWidget * );
  fn _ZNK14QStackedWidget7indexOfEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QStackedWidget::removeWidget(QWidget * w);
  fn _ZN14QStackedWidget12removeWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: QWidget * QStackedWidget::widget(int );
  fn _ZNK14QStackedWidget6widgetEi(arg0: c_int) -> i32;
  // proto: void QStackedWidget::currentChanged(int );
  fn _ZN14QStackedWidget14currentChangedEi(arg0: c_int) -> i32;
  // proto: int QStackedWidget::addWidget(QWidget * w);
  fn _ZN14QStackedWidget9addWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: int QStackedWidget::currentIndex();
  fn _ZNK14QStackedWidget12currentIndexEv() -> i32;
  // proto: int QStackedWidget::count();
  fn _ZNK14QStackedWidget5countEv() -> i32;
  // proto: void QStackedWidget::setCurrentWidget(QWidget * w);
  fn _ZN14QStackedWidget16setCurrentWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QStackedWidget::FreeQStackedWidget();
  fn _ZN14QStackedWidgetD0Ev() -> i32;
  // proto: void QStackedWidget::NewQStackedWidget(const QStackedWidget & );
  fn _ZN14QStackedWidgetC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: const QMetaObject * QStackedWidget::metaObject();
  fn _ZNK14QStackedWidget10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QStackedWidget)=1
pub struct QStackedWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStackedWidget {
  pub fn setCurrentIndex<T: QStackedWidget_setCurrentIndex>(&mut self, value: T) -> i32 {
    value.setCurrentIndex(self);
    return 1;
  }
}

pub trait QStackedWidget_setCurrentIndex {
  fn setCurrentIndex(self, this: &mut QStackedWidget) -> i32;
}

// proto: void QStackedWidget::setCurrentIndex(int index);
impl<'a> /*trait*/ QStackedWidget_setCurrentIndex for (i32) {
  fn setCurrentIndex(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget15setCurrentIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QStackedWidget15setCurrentIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn NewQStackedWidget<T: QStackedWidget_NewQStackedWidget>(value: T) -> QStackedWidget {
    let rsthis = value.NewQStackedWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QStackedWidget_NewQStackedWidget {
  fn NewQStackedWidget(self) -> QStackedWidget;
}

// proto: void QStackedWidget::NewQStackedWidget(QWidget * parent);
impl<'a> /*trait*/ QStackedWidget_NewQStackedWidget for (&'a mut QWidget) {
  fn NewQStackedWidget(self) -> QStackedWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidgetC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedWidgetC1EP7QWidget(qthis, arg0)};
    let rsthis = QStackedWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn currentWidget<T: QStackedWidget_currentWidget>(&mut self, value: T) -> i32 {
    value.currentWidget(self);
    return 1;
  }
}

pub trait QStackedWidget_currentWidget {
  fn currentWidget(self, this: &mut QStackedWidget) -> i32;
}

// proto: QWidget * QStackedWidget::currentWidget();
impl<'a> /*trait*/ QStackedWidget_currentWidget for () {
  fn currentWidget(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget13currentWidgetEv()};
    unsafe {_ZNK14QStackedWidget13currentWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn widgetRemoved<T: QStackedWidget_widgetRemoved>(&mut self, value: T) -> i32 {
    value.widgetRemoved(self);
    return 1;
  }
}

pub trait QStackedWidget_widgetRemoved {
  fn widgetRemoved(self, this: &mut QStackedWidget) -> i32;
}

// proto: void QStackedWidget::widgetRemoved(int index);
impl<'a> /*trait*/ QStackedWidget_widgetRemoved for (i32) {
  fn widgetRemoved(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget13widgetRemovedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QStackedWidget13widgetRemovedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn insertWidget<T: QStackedWidget_insertWidget>(&mut self, value: T) -> i32 {
    value.insertWidget(self);
    return 1;
  }
}

pub trait QStackedWidget_insertWidget {
  fn insertWidget(self, this: &mut QStackedWidget) -> i32;
}

// proto: int QStackedWidget::insertWidget(int index, QWidget * w);
impl<'a> /*trait*/ QStackedWidget_insertWidget for (i32, &'a mut QWidget) {
  fn insertWidget(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget12insertWidgetEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedWidget12insertWidgetEiP7QWidget(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn indexOf<T: QStackedWidget_indexOf>(&mut self, value: T) -> i32 {
    value.indexOf(self);
    return 1;
  }
}

pub trait QStackedWidget_indexOf {
  fn indexOf(self, this: &mut QStackedWidget) -> i32;
}

// proto: int QStackedWidget::indexOf(QWidget * );
impl<'a> /*trait*/ QStackedWidget_indexOf for (&'a mut QWidget) {
  fn indexOf(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK14QStackedWidget7indexOfEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn removeWidget<T: QStackedWidget_removeWidget>(&mut self, value: T) -> i32 {
    value.removeWidget(self);
    return 1;
  }
}

pub trait QStackedWidget_removeWidget {
  fn removeWidget(self, this: &mut QStackedWidget) -> i32;
}

// proto: void QStackedWidget::removeWidget(QWidget * w);
impl<'a> /*trait*/ QStackedWidget_removeWidget for (&'a mut QWidget) {
  fn removeWidget(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget12removeWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedWidget12removeWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn widget<T: QStackedWidget_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QStackedWidget_widget {
  fn widget(self, this: &mut QStackedWidget) -> i32;
}

// proto: QWidget * QStackedWidget::widget(int );
impl<'a> /*trait*/ QStackedWidget_widget for (i32) {
  fn widget(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget6widgetEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK14QStackedWidget6widgetEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn currentChanged<T: QStackedWidget_currentChanged>(&mut self, value: T) -> i32 {
    value.currentChanged(self);
    return 1;
  }
}

pub trait QStackedWidget_currentChanged {
  fn currentChanged(self, this: &mut QStackedWidget) -> i32;
}

// proto: void QStackedWidget::currentChanged(int );
impl<'a> /*trait*/ QStackedWidget_currentChanged for (i32) {
  fn currentChanged(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget14currentChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN14QStackedWidget14currentChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn addWidget<T: QStackedWidget_addWidget>(&mut self, value: T) -> i32 {
    value.addWidget(self);
    return 1;
  }
}

pub trait QStackedWidget_addWidget {
  fn addWidget(self, this: &mut QStackedWidget) -> i32;
}

// proto: int QStackedWidget::addWidget(QWidget * w);
impl<'a> /*trait*/ QStackedWidget_addWidget for (&'a mut QWidget) {
  fn addWidget(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedWidget9addWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn currentIndex<T: QStackedWidget_currentIndex>(&mut self, value: T) -> i32 {
    value.currentIndex(self);
    return 1;
  }
}

pub trait QStackedWidget_currentIndex {
  fn currentIndex(self, this: &mut QStackedWidget) -> i32;
}

// proto: int QStackedWidget::currentIndex();
impl<'a> /*trait*/ QStackedWidget_currentIndex for () {
  fn currentIndex(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget12currentIndexEv()};
    unsafe {_ZNK14QStackedWidget12currentIndexEv()};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn count<T: QStackedWidget_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QStackedWidget_count {
  fn count(self, this: &mut QStackedWidget) -> i32;
}

// proto: int QStackedWidget::count();
impl<'a> /*trait*/ QStackedWidget_count for () {
  fn count(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget5countEv()};
    unsafe {_ZNK14QStackedWidget5countEv()};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn setCurrentWidget<T: QStackedWidget_setCurrentWidget>(&mut self, value: T) -> i32 {
    value.setCurrentWidget(self);
    return 1;
  }
}

pub trait QStackedWidget_setCurrentWidget {
  fn setCurrentWidget(self, this: &mut QStackedWidget) -> i32;
}

// proto: void QStackedWidget::setCurrentWidget(QWidget * w);
impl<'a> /*trait*/ QStackedWidget_setCurrentWidget for (&'a mut QWidget) {
  fn setCurrentWidget(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedWidget16setCurrentWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn FreeQStackedWidget<T: QStackedWidget_FreeQStackedWidget>(&mut self, value: T) -> i32 {
    value.FreeQStackedWidget(self);
    return 1;
  }
}

pub trait QStackedWidget_FreeQStackedWidget {
  fn FreeQStackedWidget(self, this: &mut QStackedWidget) -> i32;
}

// proto: void QStackedWidget::FreeQStackedWidget();
impl<'a> /*trait*/ QStackedWidget_FreeQStackedWidget for () {
  fn FreeQStackedWidget(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidgetD0Ev()};
    unsafe {_ZN14QStackedWidgetD0Ev()};
    return 1;
  }
}

// proto: void QStackedWidget::NewQStackedWidget(const QStackedWidget & );
impl<'a> /*trait*/ QStackedWidget_NewQStackedWidget for (&'a  QStackedWidget) {
  fn NewQStackedWidget(self) -> QStackedWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN14QStackedWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QStackedWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QStackedWidget {
  pub fn metaObject<T: QStackedWidget_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QStackedWidget_metaObject {
  fn metaObject(self, this: &mut QStackedWidget) -> i32;
}

// proto: const QMetaObject * QStackedWidget::metaObject();
impl<'a> /*trait*/ QStackedWidget_metaObject for () {
  fn metaObject(self, this: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget10metaObjectEv()};
    unsafe {_ZNK14QStackedWidget10metaObjectEv()};
    return 1;
  }
}

