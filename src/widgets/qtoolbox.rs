// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtWidgets/qtoolbox.h
// dst-file: /src/widgets/qtoolbox.rs
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
use super::super::core::qstring::QString; // 771
use super::super::gui::qicon::QIcon; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QToolBox::removeItem(int index);
  fn _ZN8QToolBox10removeItemEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QToolBox::insertItem(int index, QWidget * widget, const QString & text);
  fn _ZN8QToolBox10insertItemEiP7QWidgetRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
  // proto:  QString QToolBox::itemText(int index);
  fn _ZNK8QToolBox8itemTextEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QToolBox::indexOf(QWidget * widget);
  fn _ZNK8QToolBox7indexOfEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QString QToolBox::itemToolTip(int index);
  fn _ZNK8QToolBox11itemToolTipEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QToolBox::QToolBox(const QToolBox & );
  fn _ZN8QToolBoxC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QToolBox::setCurrentWidget(QWidget * widget);
  fn _ZN8QToolBox16setCurrentWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QToolBox::setCurrentIndex(int index);
  fn _ZN8QToolBox15setCurrentIndexEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QToolBox::setItemIcon(int index, const QIcon & icon);
  fn _ZN8QToolBox11setItemIconEiRK5QIcon(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QToolBox::setItemText(int index, const QString & text);
  fn _ZN8QToolBox11setItemTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  int QToolBox::count();
  fn _ZNK8QToolBox5countEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QToolBox::metaObject();
  fn _ZNK8QToolBox10metaObjectEv(qthis: *mut c_void);
  // proto:  QWidget * QToolBox::widget(int index);
  fn _ZNK8QToolBox6widgetEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QToolBox::setItemToolTip(int index, const QString & toolTip);
  fn _ZN8QToolBox14setItemToolTipEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  int QToolBox::currentIndex();
  fn _ZNK8QToolBox12currentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  QWidget * QToolBox::currentWidget();
  fn _ZNK8QToolBox13currentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QToolBox::addItem(QWidget * widget, const QString & text);
  fn _ZN8QToolBox7addItemEP7QWidgetRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  bool QToolBox::isItemEnabled(int index);
  fn _ZNK8QToolBox13isItemEnabledEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QToolBox::setItemEnabled(int index, bool enabled);
  fn _ZN8QToolBox14setItemEnabledEib(qthis: *mut c_void, arg0: c_int, arg1: c_char);
  // proto:  void QToolBox::currentChanged(int index);
  fn _ZN8QToolBox14currentChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QIcon QToolBox::itemIcon(int index);
  fn _ZNK8QToolBox8itemIconEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QToolBox::~QToolBox();
  fn _ZN8QToolBoxD0Ev(qthis: *mut c_void);
  // proto:  int QToolBox::addItem(QWidget * widget, const QIcon & icon, const QString & text);
  fn _ZN8QToolBox7addItemEP7QWidgetRK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
  // proto:  int QToolBox::insertItem(int index, QWidget * widget, const QIcon & icon, const QString & text);
  fn _ZN8QToolBox10insertItemEiP7QWidgetRK5QIconRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QToolBox)=1
pub struct QToolBox {
  pub qclsinst: *mut c_void,
}

  // proto:  void QToolBox::removeItem(int index);
impl /*struct*/ QToolBox {
  pub fn removeItem<RetType, T: QToolBox_removeItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeItem(self);
    // return 1;
  }
}

pub trait QToolBox_removeItem<RetType> {
  fn removeItem(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  void QToolBox::removeItem(int index);
impl<'a> /*trait*/ QToolBox_removeItem<()> for (i32) {
  fn removeItem(self , rsthis: &mut QToolBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox10removeItemEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QToolBox10removeItemEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QToolBox::insertItem(int index, QWidget * widget, const QString & text);
impl /*struct*/ QToolBox {
  pub fn insertItem<RetType, T: QToolBox_insertItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertItem(self);
    // return 1;
  }
}

pub trait QToolBox_insertItem<RetType> {
  fn insertItem(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  int QToolBox::insertItem(int index, QWidget * widget, const QString & text);
impl<'a> /*trait*/ QToolBox_insertItem<i32> for (i32, QWidget, QString) {
  fn insertItem(self , rsthis: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox10insertItemEiP7QWidgetRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBox10insertItemEiP7QWidgetRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QToolBox::itemText(int index);
impl /*struct*/ QToolBox {
  pub fn itemText<RetType, T: QToolBox_itemText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemText(self);
    // return 1;
  }
}

pub trait QToolBox_itemText<RetType> {
  fn itemText(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  QString QToolBox::itemText(int index);
impl<'a> /*trait*/ QToolBox_itemText<QString> for (i32) {
  fn itemText(self , rsthis: &mut QToolBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox8itemTextEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK8QToolBox8itemTextEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QToolBox::indexOf(QWidget * widget);
impl /*struct*/ QToolBox {
  pub fn indexOf<RetType, T: QToolBox_indexOf<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QToolBox_indexOf<RetType> {
  fn indexOf(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  int QToolBox::indexOf(QWidget * widget);
impl<'a> /*trait*/ QToolBox_indexOf<i32> for (QWidget) {
  fn indexOf(self , rsthis: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QToolBox7indexOfEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QToolBox::itemToolTip(int index);
impl /*struct*/ QToolBox {
  pub fn itemToolTip<RetType, T: QToolBox_itemToolTip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemToolTip(self);
    // return 1;
  }
}

pub trait QToolBox_itemToolTip<RetType> {
  fn itemToolTip(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  QString QToolBox::itemToolTip(int index);
impl<'a> /*trait*/ QToolBox_itemToolTip<QString> for (i32) {
  fn itemToolTip(self , rsthis: &mut QToolBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox11itemToolTipEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK8QToolBox11itemToolTipEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBox::QToolBox(const QToolBox & );
impl /*struct*/ QToolBox {
  pub fn NewQToolBox<T: QToolBox_NewQToolBox>(value: T) -> QToolBox {
    let rsthis = value.NewQToolBox();
    return rsthis;
    // return 1;
  }
}

pub trait QToolBox_NewQToolBox {
  fn NewQToolBox(self) -> QToolBox;
}

  // proto:  void QToolBox::QToolBox(const QToolBox & );
impl<'a> /*trait*/ QToolBox_NewQToolBox for (QToolBox) {
  fn NewQToolBox(self) -> QToolBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QToolBoxC1ERKS_(qthis, arg0)};
    let rsthis = QToolBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QToolBox::setCurrentWidget(QWidget * widget);
impl /*struct*/ QToolBox {
  pub fn setCurrentWidget<RetType, T: QToolBox_setCurrentWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWidget(self);
    // return 1;
  }
}

pub trait QToolBox_setCurrentWidget<RetType> {
  fn setCurrentWidget(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  void QToolBox::setCurrentWidget(QWidget * widget);
impl<'a> /*trait*/ QToolBox_setCurrentWidget<()> for (QWidget) {
  fn setCurrentWidget(self , rsthis: &mut QToolBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolBox16setCurrentWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QToolBox::setCurrentIndex(int index);
impl /*struct*/ QToolBox {
  pub fn setCurrentIndex<RetType, T: QToolBox_setCurrentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QToolBox_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  void QToolBox::setCurrentIndex(int index);
impl<'a> /*trait*/ QToolBox_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: &mut QToolBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QToolBox15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QToolBox::setItemIcon(int index, const QIcon & icon);
impl /*struct*/ QToolBox {
  pub fn setItemIcon<RetType, T: QToolBox_setItemIcon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setItemIcon(self);
    // return 1;
  }
}

pub trait QToolBox_setItemIcon<RetType> {
  fn setItemIcon(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  void QToolBox::setItemIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QToolBox_setItemIcon<()> for (i32, QIcon) {
  fn setItemIcon(self , rsthis: &mut QToolBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox11setItemIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolBox11setItemIconEiRK5QIcon(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QToolBox::setItemText(int index, const QString & text);
impl /*struct*/ QToolBox {
  pub fn setItemText<RetType, T: QToolBox_setItemText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setItemText(self);
    // return 1;
  }
}

pub trait QToolBox_setItemText<RetType> {
  fn setItemText(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  void QToolBox::setItemText(int index, const QString & text);
impl<'a> /*trait*/ QToolBox_setItemText<()> for (i32, QString) {
  fn setItemText(self , rsthis: &mut QToolBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox11setItemTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolBox11setItemTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QToolBox::count();
impl /*struct*/ QToolBox {
  pub fn count<RetType, T: QToolBox_count<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QToolBox_count<RetType> {
  fn count(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  int QToolBox::count();
impl<'a> /*trait*/ QToolBox_count<i32> for () {
  fn count(self , rsthis: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox5countEv()};
    let mut ret = unsafe {_ZNK8QToolBox5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QToolBox::metaObject();
impl /*struct*/ QToolBox {
  pub fn metaObject<RetType, T: QToolBox_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QToolBox_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  const QMetaObject * QToolBox::metaObject();
impl<'a> /*trait*/ QToolBox_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QToolBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox10metaObjectEv()};
     unsafe {_ZNK8QToolBox10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QWidget * QToolBox::widget(int index);
impl /*struct*/ QToolBox {
  pub fn widget<RetType, T: QToolBox_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QToolBox_widget<RetType> {
  fn widget(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  QWidget * QToolBox::widget(int index);
impl<'a> /*trait*/ QToolBox_widget<QWidget> for (i32) {
  fn widget(self , rsthis: &mut QToolBox) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox6widgetEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK8QToolBox6widgetEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBox::setItemToolTip(int index, const QString & toolTip);
impl /*struct*/ QToolBox {
  pub fn setItemToolTip<RetType, T: QToolBox_setItemToolTip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setItemToolTip(self);
    // return 1;
  }
}

pub trait QToolBox_setItemToolTip<RetType> {
  fn setItemToolTip(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  void QToolBox::setItemToolTip(int index, const QString & toolTip);
impl<'a> /*trait*/ QToolBox_setItemToolTip<()> for (i32, QString) {
  fn setItemToolTip(self , rsthis: &mut QToolBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox14setItemToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN8QToolBox14setItemToolTipEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QToolBox::currentIndex();
impl /*struct*/ QToolBox {
  pub fn currentIndex<RetType, T: QToolBox_currentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QToolBox_currentIndex<RetType> {
  fn currentIndex(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  int QToolBox::currentIndex();
impl<'a> /*trait*/ QToolBox_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox12currentIndexEv()};
    let mut ret = unsafe {_ZNK8QToolBox12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QWidget * QToolBox::currentWidget();
impl /*struct*/ QToolBox {
  pub fn currentWidget<RetType, T: QToolBox_currentWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentWidget(self);
    // return 1;
  }
}

pub trait QToolBox_currentWidget<RetType> {
  fn currentWidget(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  QWidget * QToolBox::currentWidget();
impl<'a> /*trait*/ QToolBox_currentWidget<QWidget> for () {
  fn currentWidget(self , rsthis: &mut QToolBox) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox13currentWidgetEv()};
    let mut ret = unsafe {_ZNK8QToolBox13currentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QToolBox::addItem(QWidget * widget, const QString & text);
impl /*struct*/ QToolBox {
  pub fn addItem<RetType, T: QToolBox_addItem<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addItem(self);
    // return 1;
  }
}

pub trait QToolBox_addItem<RetType> {
  fn addItem(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  int QToolBox::addItem(QWidget * widget, const QString & text);
impl<'a> /*trait*/ QToolBox_addItem<i32> for (QWidget, QString) {
  fn addItem(self , rsthis: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox7addItemEP7QWidgetRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBox7addItemEP7QWidgetRK7QString(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QToolBox::isItemEnabled(int index);
impl /*struct*/ QToolBox {
  pub fn isItemEnabled<RetType, T: QToolBox_isItemEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isItemEnabled(self);
    // return 1;
  }
}

pub trait QToolBox_isItemEnabled<RetType> {
  fn isItemEnabled(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  bool QToolBox::isItemEnabled(int index);
impl<'a> /*trait*/ QToolBox_isItemEnabled<i8> for (i32) {
  fn isItemEnabled(self , rsthis: &mut QToolBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox13isItemEnabledEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK8QToolBox13isItemEnabledEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QToolBox::setItemEnabled(int index, bool enabled);
impl /*struct*/ QToolBox {
  pub fn setItemEnabled<RetType, T: QToolBox_setItemEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setItemEnabled(self);
    // return 1;
  }
}

pub trait QToolBox_setItemEnabled<RetType> {
  fn setItemEnabled(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  void QToolBox::setItemEnabled(int index, bool enabled);
impl<'a> /*trait*/ QToolBox_setItemEnabled<()> for (i32, i8) {
  fn setItemEnabled(self , rsthis: &mut QToolBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox14setItemEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN8QToolBox14setItemEnabledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QToolBox::currentChanged(int index);
impl /*struct*/ QToolBox {
  pub fn currentChanged<RetType, T: QToolBox_currentChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentChanged(self);
    // return 1;
  }
}

pub trait QToolBox_currentChanged<RetType> {
  fn currentChanged(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  void QToolBox::currentChanged(int index);
impl<'a> /*trait*/ QToolBox_currentChanged<()> for (i32) {
  fn currentChanged(self , rsthis: &mut QToolBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox14currentChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN8QToolBox14currentChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QToolBox::itemIcon(int index);
impl /*struct*/ QToolBox {
  pub fn itemIcon<RetType, T: QToolBox_itemIcon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemIcon(self);
    // return 1;
  }
}

pub trait QToolBox_itemIcon<RetType> {
  fn itemIcon(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  QIcon QToolBox::itemIcon(int index);
impl<'a> /*trait*/ QToolBox_itemIcon<QIcon> for (i32) {
  fn itemIcon(self , rsthis: &mut QToolBox) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QToolBox8itemIconEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK8QToolBox8itemIconEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QToolBox::~QToolBox();
impl /*struct*/ QToolBox {
  pub fn FreeQToolBox<RetType, T: QToolBox_FreeQToolBox<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQToolBox(self);
    // return 1;
  }
}

pub trait QToolBox_FreeQToolBox<RetType> {
  fn FreeQToolBox(self , rsthis: &mut QToolBox) -> RetType;
}

  // proto:  void QToolBox::~QToolBox();
impl<'a> /*trait*/ QToolBox_FreeQToolBox<()> for () {
  fn FreeQToolBox(self , rsthis: &mut QToolBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBoxD0Ev()};
     unsafe {_ZN8QToolBoxD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QToolBox::addItem(QWidget * widget, const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QToolBox_addItem<i32> for (QWidget, QIcon, QString) {
  fn addItem(self , rsthis: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox7addItemEP7QWidgetRK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBox7addItemEP7QWidgetRK5QIconRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QToolBox::insertItem(int index, QWidget * widget, const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QToolBox_insertItem<i32> for (i32, QWidget, QIcon, QString) {
  fn insertItem(self , rsthis: &mut QToolBox) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QToolBox10insertItemEiP7QWidgetRK5QIconRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QToolBox10insertItemEiP7QWidgetRK5QIconRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

