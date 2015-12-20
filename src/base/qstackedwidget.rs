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
  // proto:  void QStackedWidget::setCurrentIndex(int index);
  fn _ZN14QStackedWidget15setCurrentIndexEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStackedWidget::QStackedWidget(QWidget * parent);
  fn _ZN14QStackedWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QStackedWidget::currentWidget();
  fn _ZNK14QStackedWidget13currentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStackedWidget::widgetRemoved(int index);
  fn _ZN14QStackedWidget13widgetRemovedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QStackedWidget::insertWidget(int index, QWidget * w);
  fn _ZN14QStackedWidget12insertWidgetEiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_int;
  // proto:  int QStackedWidget::indexOf(QWidget * );
  fn _ZNK14QStackedWidget7indexOfEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QStackedWidget::removeWidget(QWidget * w);
  fn _ZN14QStackedWidget12removeWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QStackedWidget::widget(int );
  fn _ZNK14QStackedWidget6widgetEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QStackedWidget::currentChanged(int );
  fn _ZN14QStackedWidget14currentChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QStackedWidget::addWidget(QWidget * w);
  fn _ZN14QStackedWidget9addWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QStackedWidget::currentIndex();
  fn _ZNK14QStackedWidget12currentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  int QStackedWidget::count();
  fn _ZNK14QStackedWidget5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QStackedWidget::setCurrentWidget(QWidget * w);
  fn _ZN14QStackedWidget16setCurrentWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStackedWidget::~QStackedWidget();
  fn _ZN14QStackedWidgetD0Ev(qthis: *mut c_void);
  // proto:  void QStackedWidget::QStackedWidget(const QStackedWidget & );
  fn _ZN14QStackedWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QStackedWidget::metaObject();
  fn _ZNK14QStackedWidget10metaObjectEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QStackedWidget)=1
pub struct QStackedWidget {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStackedWidget::setCurrentIndex(int index);
impl /*struct*/ QStackedWidget {
  pub fn setCurrentIndex<RetType, T: QStackedWidget_setCurrentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QStackedWidget_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::setCurrentIndex(int index);
impl<'a> /*trait*/ QStackedWidget_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: &mut QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QStackedWidget15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStackedWidget::QStackedWidget(QWidget * parent);
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

  // proto:  void QStackedWidget::QStackedWidget(QWidget * parent);
impl<'a> /*trait*/ QStackedWidget_NewQStackedWidget for (QWidget) {
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

  // proto:  QWidget * QStackedWidget::currentWidget();
impl /*struct*/ QStackedWidget {
  pub fn currentWidget<RetType, T: QStackedWidget_currentWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_currentWidget<RetType> {
  fn currentWidget(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  QWidget * QStackedWidget::currentWidget();
impl<'a> /*trait*/ QStackedWidget_currentWidget<QWidget> for () {
  fn currentWidget(self , rsthis: &mut QStackedWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget13currentWidgetEv()};
    let mut ret = unsafe {_ZNK14QStackedWidget13currentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QStackedWidget::widgetRemoved(int index);
impl /*struct*/ QStackedWidget {
  pub fn widgetRemoved<RetType, T: QStackedWidget_widgetRemoved<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widgetRemoved(self);
    // return 1;
  }
}

pub trait QStackedWidget_widgetRemoved<RetType> {
  fn widgetRemoved(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::widgetRemoved(int index);
impl<'a> /*trait*/ QStackedWidget_widgetRemoved<()> for (i32) {
  fn widgetRemoved(self , rsthis: &mut QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget13widgetRemovedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QStackedWidget13widgetRemovedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QStackedWidget::insertWidget(int index, QWidget * w);
impl /*struct*/ QStackedWidget {
  pub fn insertWidget<RetType, T: QStackedWidget_insertWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_insertWidget<RetType> {
  fn insertWidget(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  int QStackedWidget::insertWidget(int index, QWidget * w);
impl<'a> /*trait*/ QStackedWidget_insertWidget<i32> for (i32, QWidget) {
  fn insertWidget(self , rsthis: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget12insertWidgetEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QStackedWidget12insertWidgetEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStackedWidget::indexOf(QWidget * );
impl /*struct*/ QStackedWidget {
  pub fn indexOf<RetType, T: QStackedWidget_indexOf<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QStackedWidget_indexOf<RetType> {
  fn indexOf(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  int QStackedWidget::indexOf(QWidget * );
impl<'a> /*trait*/ QStackedWidget_indexOf<i32> for (QWidget) {
  fn indexOf(self , rsthis: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QStackedWidget7indexOfEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStackedWidget::removeWidget(QWidget * w);
impl /*struct*/ QStackedWidget {
  pub fn removeWidget<RetType, T: QStackedWidget_removeWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_removeWidget<RetType> {
  fn removeWidget(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::removeWidget(QWidget * w);
impl<'a> /*trait*/ QStackedWidget_removeWidget<()> for (QWidget) {
  fn removeWidget(self , rsthis: &mut QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget12removeWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QStackedWidget12removeWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QStackedWidget::widget(int );
impl /*struct*/ QStackedWidget {
  pub fn widget<RetType, T: QStackedWidget_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QStackedWidget_widget<RetType> {
  fn widget(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  QWidget * QStackedWidget::widget(int );
impl<'a> /*trait*/ QStackedWidget_widget<QWidget> for (i32) {
  fn widget(self , rsthis: &mut QStackedWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget6widgetEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK14QStackedWidget6widgetEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QStackedWidget::currentChanged(int );
impl /*struct*/ QStackedWidget {
  pub fn currentChanged<RetType, T: QStackedWidget_currentChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentChanged(self);
    // return 1;
  }
}

pub trait QStackedWidget_currentChanged<RetType> {
  fn currentChanged(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::currentChanged(int );
impl<'a> /*trait*/ QStackedWidget_currentChanged<()> for (i32) {
  fn currentChanged(self , rsthis: &mut QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget14currentChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN14QStackedWidget14currentChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QStackedWidget::addWidget(QWidget * w);
impl /*struct*/ QStackedWidget {
  pub fn addWidget<RetType, T: QStackedWidget_addWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_addWidget<RetType> {
  fn addWidget(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  int QStackedWidget::addWidget(QWidget * w);
impl<'a> /*trait*/ QStackedWidget_addWidget<i32> for (QWidget) {
  fn addWidget(self , rsthis: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget9addWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QStackedWidget9addWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStackedWidget::currentIndex();
impl /*struct*/ QStackedWidget {
  pub fn currentIndex<RetType, T: QStackedWidget_currentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QStackedWidget_currentIndex<RetType> {
  fn currentIndex(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  int QStackedWidget::currentIndex();
impl<'a> /*trait*/ QStackedWidget_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget12currentIndexEv()};
    let mut ret = unsafe {_ZNK14QStackedWidget12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStackedWidget::count();
impl /*struct*/ QStackedWidget {
  pub fn count<RetType, T: QStackedWidget_count<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QStackedWidget_count<RetType> {
  fn count(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  int QStackedWidget::count();
impl<'a> /*trait*/ QStackedWidget_count<i32> for () {
  fn count(self , rsthis: &mut QStackedWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget5countEv()};
    let mut ret = unsafe {_ZNK14QStackedWidget5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStackedWidget::setCurrentWidget(QWidget * w);
impl /*struct*/ QStackedWidget {
  pub fn setCurrentWidget<RetType, T: QStackedWidget_setCurrentWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_setCurrentWidget<RetType> {
  fn setCurrentWidget(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::setCurrentWidget(QWidget * w);
impl<'a> /*trait*/ QStackedWidget_setCurrentWidget<()> for (QWidget) {
  fn setCurrentWidget(self , rsthis: &mut QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidget16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN14QStackedWidget16setCurrentWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStackedWidget::~QStackedWidget();
impl /*struct*/ QStackedWidget {
  pub fn FreeQStackedWidget<RetType, T: QStackedWidget_FreeQStackedWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQStackedWidget(self);
    // return 1;
  }
}

pub trait QStackedWidget_FreeQStackedWidget<RetType> {
  fn FreeQStackedWidget(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  void QStackedWidget::~QStackedWidget();
impl<'a> /*trait*/ QStackedWidget_FreeQStackedWidget<()> for () {
  fn FreeQStackedWidget(self , rsthis: &mut QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidgetD0Ev()};
     unsafe {_ZN14QStackedWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStackedWidget::QStackedWidget(const QStackedWidget & );
impl<'a> /*trait*/ QStackedWidget_NewQStackedWidget for (QStackedWidget) {
  fn NewQStackedWidget(self) -> QStackedWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStackedWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QStackedWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QStackedWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QStackedWidget::metaObject();
impl /*struct*/ QStackedWidget {
  pub fn metaObject<RetType, T: QStackedWidget_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStackedWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QStackedWidget) -> RetType;
}

  // proto:  const QMetaObject * QStackedWidget::metaObject();
impl<'a> /*trait*/ QStackedWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QStackedWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QStackedWidget10metaObjectEv()};
     unsafe {_ZNK14QStackedWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

