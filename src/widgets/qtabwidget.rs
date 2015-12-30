// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtWidgets/qtabwidget.h
// dst-file: /src/widgets/qtabwidget.rs
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
use super::qtabbar::QTabBar; // 773
use super::super::gui::qicon::QIcon; // 771
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTabWidget_Class_Size() -> c_int;
  // proto:  void QTabWidget::setCurrentWidget(QWidget * widget);
  fn _ZN10QTabWidget16setCurrentWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTabWidget::count();
  fn _ZNK10QTabWidget5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTabWidget::tabCloseRequested(int index);
  fn _ZN10QTabWidget17tabCloseRequestedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTabWidget::setDocumentMode(bool set);
  fn _ZN10QTabWidget15setDocumentModeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QTabWidget::heightForWidth(int width);
  fn _ZNK10QTabWidget14heightForWidthEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_int;
  // proto:  int QTabWidget::addTab(QWidget * widget, const QString & );
  fn _ZN10QTabWidget6addTabEP7QWidgetRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  QString QTabWidget::tabText(int index);
  fn _ZNK10QTabWidget7tabTextEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTabWidget::clear();
  fn _ZN10QTabWidget5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTabWidget::hasHeightForWidth();
  fn _ZNK10QTabWidget17hasHeightForWidthEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QTabBar * QTabWidget::tabBar();
  fn _ZNK10QTabWidget6tabBarEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QTabWidget::tabsClosable();
  fn _ZNK10QTabWidget12tabsClosableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QIcon & icon, const QString & label);
  fn _ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_int;
  // proto:  int QTabWidget::addTab(QWidget * widget, const QIcon & icon, const QString & label);
  fn _ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
  // proto:  void QTabWidget::setUsesScrollButtons(bool useButtons);
  fn _ZN10QTabWidget20setUsesScrollButtonsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QTabWidget::metaObject();
  fn _ZNK10QTabWidget10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QTabWidget::tabToolTip(int index);
  fn _ZNK10QTabWidget10tabToolTipEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QWidget * QTabWidget::currentWidget();
  fn _ZNK10QTabWidget13currentWidgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTabWidget::setIconSize(const QSize & size);
  fn _ZN10QTabWidget11setIconSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QWidget * QTabWidget::widget(int index);
  fn _ZNK10QTabWidget6widgetEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTabWidget::setMovable(bool movable);
  fn _ZN10QTabWidget10setMovableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QTabWidget::documentMode();
  fn _ZNK10QTabWidget12documentModeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QTabWidget::tabWhatsThis(int index);
  fn _ZNK10QTabWidget12tabWhatsThisEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTabWidget::tabBarClicked(int index);
  fn _ZN10QTabWidget13tabBarClickedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTabWidget::setTabText(int index, const QString & );
  fn _ZN10QTabWidget10setTabTextEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTabWidget::QTabWidget(const QTabWidget & );
  fn dector_ZN10QTabWidgetC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QTabWidgetC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTabWidget::QTabWidget(QWidget * parent);
  fn dector_ZN10QTabWidgetC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QTabWidgetC1EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QTabWidget::tabBarAutoHide();
  fn _ZNK10QTabWidget14tabBarAutoHideEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTabWidget::currentChanged(int index);
  fn _ZN10QTabWidget14currentChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTabWidget::setTabIcon(int index, const QIcon & icon);
  fn _ZN10QTabWidget10setTabIconEiRK5QIcon(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QIcon QTabWidget::tabIcon(int index);
  fn _ZNK10QTabWidget7tabIconEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabWidget::isTabEnabled(int index);
  fn _ZNK10QTabWidget12isTabEnabledEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QTabWidget::setTabBarAutoHide(bool enabled);
  fn _ZN10QTabWidget17setTabBarAutoHideEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QSize QTabWidget::iconSize();
  fn _ZNK10QTabWidget8iconSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTabWidget::setTabsClosable(bool closeable);
  fn _ZN10QTabWidget15setTabsClosableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QSize QTabWidget::minimumSizeHint();
  fn _ZNK10QTabWidget15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTabWidget::setCurrentIndex(int index);
  fn _ZN10QTabWidget15setCurrentIndexEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTabWidget::~QTabWidget();
  fn _ZN10QTabWidgetD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTabWidget::setTabWhatsThis(int index, const QString & text);
  fn _ZN10QTabWidget15setTabWhatsThisEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  QSize QTabWidget::sizeHint();
  fn _ZNK10QTabWidget8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTabWidget::indexOf(QWidget * widget);
  fn _ZNK10QTabWidget7indexOfEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QTabWidget::removeTab(int index);
  fn _ZN10QTabWidget9removeTabEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTabWidget::setTabToolTip(int index, const QString & tip);
  fn _ZN10QTabWidget13setTabToolTipEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  bool QTabWidget::isMovable();
  fn _ZNK10QTabWidget9isMovableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QTabWidget::usesScrollButtons();
  fn _ZNK10QTabWidget17usesScrollButtonsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTabWidget::tabBarDoubleClicked(int index);
  fn _ZN10QTabWidget19tabBarDoubleClickedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QTabWidget::currentIndex();
  fn _ZNK10QTabWidget12currentIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QString & );
  fn _ZN10QTabWidget9insertTabEiP7QWidgetRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
  // proto:  void QTabWidget::setTabEnabled(int index, bool );
  fn _ZN10QTabWidget13setTabEnabledEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  fn QTabWidget_SlotProxy_connect__ZN10QTabWidget19tabBarDoubleClickedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTabWidget_SlotProxy_connect__ZN10QTabWidget13tabBarClickedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTabWidget_SlotProxy_connect__ZN10QTabWidget17tabCloseRequestedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTabWidget_SlotProxy_connect__ZN10QTabWidget14currentChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTabWidget)=1
#[derive(Default)]
pub struct QTabWidget {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _tabCloseRequested_1: QTabWidget_tabCloseRequested_signal,
  pub _tabBarDoubleClicked_1: QTabWidget_tabBarDoubleClicked_signal,
  pub _tabBarClicked_1: QTabWidget_tabBarClicked_signal,
  pub _currentChanged_1: QTabWidget_currentChanged_signal,
}

impl /*struct*/ QTabWidget {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTabWidget {
    return QTabWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTabWidget {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QTabWidget {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QTabWidget::setCurrentWidget(QWidget * widget);
impl /*struct*/ QTabWidget {
  pub fn setCurrentWidget<RetType, T: QTabWidget_setCurrentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWidget(self);
    // return 1;
  }
}

pub trait QTabWidget_setCurrentWidget<RetType> {
  fn setCurrentWidget(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setCurrentWidget(QWidget * widget);
impl<'a> /*trait*/ QTabWidget_setCurrentWidget<()> for (&'a QWidget) {
  fn setCurrentWidget(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget16setCurrentWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTabWidget::count();
impl /*struct*/ QTabWidget {
  pub fn count<RetType, T: QTabWidget_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QTabWidget_count<RetType> {
  fn count(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::count();
impl<'a> /*trait*/ QTabWidget_count<i32> for () {
  fn count(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget5countEv()};
    let mut ret = unsafe {_ZNK10QTabWidget5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTabWidget::tabCloseRequested(int index);
impl /*struct*/ QTabWidget {
  pub fn tabCloseRequested<RetType, T: QTabWidget_tabCloseRequested<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabCloseRequested(self);
    // return 1;
  }
}

pub trait QTabWidget_tabCloseRequested<RetType> {
  fn tabCloseRequested(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::tabCloseRequested(int index);
impl<'a> /*trait*/ QTabWidget_tabCloseRequested<()> for (i32) {
  fn tabCloseRequested(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget17tabCloseRequestedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget17tabCloseRequestedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabWidget::setDocumentMode(bool set);
impl /*struct*/ QTabWidget {
  pub fn setDocumentMode<RetType, T: QTabWidget_setDocumentMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMode(self);
    // return 1;
  }
}

pub trait QTabWidget_setDocumentMode<RetType> {
  fn setDocumentMode(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setDocumentMode(bool set);
impl<'a> /*trait*/ QTabWidget_setDocumentMode<()> for (i8) {
  fn setDocumentMode(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setDocumentModeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTabWidget15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTabWidget::heightForWidth(int width);
impl /*struct*/ QTabWidget {
  pub fn heightForWidth<RetType, T: QTabWidget_heightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QTabWidget_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::heightForWidth(int width);
impl<'a> /*trait*/ QTabWidget_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTabWidget::addTab(QWidget * widget, const QString & );
impl /*struct*/ QTabWidget {
  pub fn addTab<RetType, T: QTabWidget_addTab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addTab(self);
    // return 1;
  }
}

pub trait QTabWidget_addTab<RetType> {
  fn addTab(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::addTab(QWidget * widget, const QString & );
impl<'a> /*trait*/ QTabWidget_addTab<i32> for (&'a QWidget, &'a QString) {
  fn addTab(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget6addTabEP7QWidgetRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTabWidget6addTabEP7QWidgetRK7QString(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QTabWidget::tabText(int index);
impl /*struct*/ QTabWidget {
  pub fn tabText<RetType, T: QTabWidget_tabText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabText(self);
    // return 1;
  }
}

pub trait QTabWidget_tabText<RetType> {
  fn tabText(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  QString QTabWidget::tabText(int index);
impl<'a> /*trait*/ QTabWidget_tabText<QString> for (i32) {
  fn tabText(self , rsthis: & QTabWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7tabTextEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget7tabTextEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::clear();
impl /*struct*/ QTabWidget {
  pub fn clear<RetType, T: QTabWidget_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QTabWidget_clear<RetType> {
  fn clear(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::clear();
impl<'a> /*trait*/ QTabWidget_clear<()> for () {
  fn clear(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget5clearEv()};
     unsafe {_ZN10QTabWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTabWidget::hasHeightForWidth();
impl /*struct*/ QTabWidget {
  pub fn hasHeightForWidth<RetType, T: QTabWidget_hasHeightForWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QTabWidget_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::hasHeightForWidth();
impl<'a> /*trait*/ QTabWidget_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: & QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK10QTabWidget17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTabBar * QTabWidget::tabBar();
impl /*struct*/ QTabWidget {
  pub fn tabBar<RetType, T: QTabWidget_tabBar<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabBar(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBar<RetType> {
  fn tabBar(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  QTabBar * QTabWidget::tabBar();
impl<'a> /*trait*/ QTabWidget_tabBar<QTabBar> for () {
  fn tabBar(self , rsthis: & QTabWidget) -> QTabBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget6tabBarEv()};
    let mut ret = unsafe {_ZNK10QTabWidget6tabBarEv(rsthis.qclsinst)};
    let mut ret1 = QTabBar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTabWidget::tabsClosable();
impl /*struct*/ QTabWidget {
  pub fn tabsClosable<RetType, T: QTabWidget_tabsClosable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabsClosable(self);
    // return 1;
  }
}

pub trait QTabWidget_tabsClosable<RetType> {
  fn tabsClosable(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::tabsClosable();
impl<'a> /*trait*/ QTabWidget_tabsClosable<i8> for () {
  fn tabsClosable(self , rsthis: & QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12tabsClosableEv()};
    let mut ret = unsafe {_ZNK10QTabWidget12tabsClosableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QIcon & icon, const QString & label);
impl /*struct*/ QTabWidget {
  pub fn insertTab<RetType, T: QTabWidget_insertTab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertTab(self);
    // return 1;
  }
}

pub trait QTabWidget_insertTab<RetType> {
  fn insertTab(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QIcon & icon, const QString & label);
impl<'a> /*trait*/ QTabWidget_insertTab<i32> for (i32, &'a QWidget, &'a QIcon, &'a QString) {
  fn insertTab(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTabWidget::addTab(QWidget * widget, const QIcon & icon, const QString & label);
impl<'a> /*trait*/ QTabWidget_addTab<i32> for (&'a QWidget, &'a QIcon, &'a QString) {
  fn addTab(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTabWidget::setUsesScrollButtons(bool useButtons);
impl /*struct*/ QTabWidget {
  pub fn setUsesScrollButtons<RetType, T: QTabWidget_setUsesScrollButtons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUsesScrollButtons(self);
    // return 1;
  }
}

pub trait QTabWidget_setUsesScrollButtons<RetType> {
  fn setUsesScrollButtons(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setUsesScrollButtons(bool useButtons);
impl<'a> /*trait*/ QTabWidget_setUsesScrollButtons<()> for (i8) {
  fn setUsesScrollButtons(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget20setUsesScrollButtonsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTabWidget20setUsesScrollButtonsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QTabWidget::metaObject();
impl /*struct*/ QTabWidget {
  pub fn metaObject<RetType, T: QTabWidget_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTabWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  const QMetaObject * QTabWidget::metaObject();
impl<'a> /*trait*/ QTabWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget10metaObjectEv()};
     unsafe {_ZNK10QTabWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QTabWidget::tabToolTip(int index);
impl /*struct*/ QTabWidget {
  pub fn tabToolTip<RetType, T: QTabWidget_tabToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabToolTip(self);
    // return 1;
  }
}

pub trait QTabWidget_tabToolTip<RetType> {
  fn tabToolTip(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  QString QTabWidget::tabToolTip(int index);
impl<'a> /*trait*/ QTabWidget_tabToolTip<QString> for (i32) {
  fn tabToolTip(self , rsthis: & QTabWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget10tabToolTipEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget10tabToolTipEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QTabWidget::currentWidget();
impl /*struct*/ QTabWidget {
  pub fn currentWidget<RetType, T: QTabWidget_currentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentWidget(self);
    // return 1;
  }
}

pub trait QTabWidget_currentWidget<RetType> {
  fn currentWidget(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  QWidget * QTabWidget::currentWidget();
impl<'a> /*trait*/ QTabWidget_currentWidget<QWidget> for () {
  fn currentWidget(self , rsthis: & QTabWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget13currentWidgetEv()};
    let mut ret = unsafe {_ZNK10QTabWidget13currentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::setIconSize(const QSize & size);
impl /*struct*/ QTabWidget {
  pub fn setIconSize<RetType, T: QTabWidget_setIconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QTabWidget_setIconSize<RetType> {
  fn setIconSize(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setIconSize(const QSize & size);
impl<'a> /*trait*/ QTabWidget_setIconSize<()> for (&'a QSize) {
  fn setIconSize(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QTabWidget::widget(int index);
impl /*struct*/ QTabWidget {
  pub fn widget<RetType, T: QTabWidget_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QTabWidget_widget<RetType> {
  fn widget(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  QWidget * QTabWidget::widget(int index);
impl<'a> /*trait*/ QTabWidget_widget<QWidget> for (i32) {
  fn widget(self , rsthis: & QTabWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget6widgetEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget6widgetEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::setMovable(bool movable);
impl /*struct*/ QTabWidget {
  pub fn setMovable<RetType, T: QTabWidget_setMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMovable(self);
    // return 1;
  }
}

pub trait QTabWidget_setMovable<RetType> {
  fn setMovable(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setMovable(bool movable);
impl<'a> /*trait*/ QTabWidget_setMovable<()> for (i8) {
  fn setMovable(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setMovableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTabWidget10setMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTabWidget::documentMode();
impl /*struct*/ QTabWidget {
  pub fn documentMode<RetType, T: QTabWidget_documentMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentMode(self);
    // return 1;
  }
}

pub trait QTabWidget_documentMode<RetType> {
  fn documentMode(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::documentMode();
impl<'a> /*trait*/ QTabWidget_documentMode<i8> for () {
  fn documentMode(self , rsthis: & QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12documentModeEv()};
    let mut ret = unsafe {_ZNK10QTabWidget12documentModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QTabWidget::tabWhatsThis(int index);
impl /*struct*/ QTabWidget {
  pub fn tabWhatsThis<RetType, T: QTabWidget_tabWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabWhatsThis(self);
    // return 1;
  }
}

pub trait QTabWidget_tabWhatsThis<RetType> {
  fn tabWhatsThis(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  QString QTabWidget::tabWhatsThis(int index);
impl<'a> /*trait*/ QTabWidget_tabWhatsThis<QString> for (i32) {
  fn tabWhatsThis(self , rsthis: & QTabWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12tabWhatsThisEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget12tabWhatsThisEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::tabBarClicked(int index);
impl /*struct*/ QTabWidget {
  pub fn tabBarClicked<RetType, T: QTabWidget_tabBarClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabBarClicked(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBarClicked<RetType> {
  fn tabBarClicked(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::tabBarClicked(int index);
impl<'a> /*trait*/ QTabWidget_tabBarClicked<()> for (i32) {
  fn tabBarClicked(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13tabBarClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget13tabBarClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabText(int index, const QString & );
impl /*struct*/ QTabWidget {
  pub fn setTabText<RetType, T: QTabWidget_setTabText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabText(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabText<RetType> {
  fn setTabText(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabText(int index, const QString & );
impl<'a> /*trait*/ QTabWidget_setTabText<()> for (i32, &'a QString) {
  fn setTabText(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setTabTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget10setTabTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTabWidget::QTabWidget(const QTabWidget & );
impl /*struct*/ QTabWidget {
  pub fn New<T: QTabWidget_New>(value: T) -> QTabWidget {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTabWidget_New {
  fn New(self) -> QTabWidget;
}

  // proto:  void QTabWidget::QTabWidget(const QTabWidget & );
impl<'a> /*trait*/ QTabWidget_New for (&'a QTabWidget) {
  fn New(self) -> QTabWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidgetC1ERKS_()};
    let ctysz: c_int = unsafe{QTabWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QTabWidgetC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QTabWidgetC1ERKS_(arg0)} as u64;
    let rsthis = QTabWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTabWidget::QTabWidget(QWidget * parent);
impl<'a> /*trait*/ QTabWidget_New for (&'a QWidget) {
  fn New(self) -> QTabWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidgetC1EP7QWidget()};
    let ctysz: c_int = unsafe{QTabWidget_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QTabWidgetC1EP7QWidget(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QTabWidgetC1EP7QWidget(arg0)} as u64;
    let rsthis = QTabWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTabWidget::tabBarAutoHide();
impl /*struct*/ QTabWidget {
  pub fn tabBarAutoHide<RetType, T: QTabWidget_tabBarAutoHide<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabBarAutoHide(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBarAutoHide<RetType> {
  fn tabBarAutoHide(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::tabBarAutoHide();
impl<'a> /*trait*/ QTabWidget_tabBarAutoHide<i8> for () {
  fn tabBarAutoHide(self , rsthis: & QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget14tabBarAutoHideEv()};
    let mut ret = unsafe {_ZNK10QTabWidget14tabBarAutoHideEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTabWidget::currentChanged(int index);
impl /*struct*/ QTabWidget {
  pub fn currentChanged<RetType, T: QTabWidget_currentChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentChanged(self);
    // return 1;
  }
}

pub trait QTabWidget_currentChanged<RetType> {
  fn currentChanged(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::currentChanged(int index);
impl<'a> /*trait*/ QTabWidget_currentChanged<()> for (i32) {
  fn currentChanged(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget14currentChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget14currentChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabIcon(int index, const QIcon & icon);
impl /*struct*/ QTabWidget {
  pub fn setTabIcon<RetType, T: QTabWidget_setTabIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabIcon(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabIcon<RetType> {
  fn setTabIcon(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QTabWidget_setTabIcon<()> for (i32, &'a QIcon) {
  fn setTabIcon(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setTabIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget10setTabIconEiRK5QIcon(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QIcon QTabWidget::tabIcon(int index);
impl /*struct*/ QTabWidget {
  pub fn tabIcon<RetType, T: QTabWidget_tabIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabIcon(self);
    // return 1;
  }
}

pub trait QTabWidget_tabIcon<RetType> {
  fn tabIcon(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  QIcon QTabWidget::tabIcon(int index);
impl<'a> /*trait*/ QTabWidget_tabIcon<QIcon> for (i32) {
  fn tabIcon(self , rsthis: & QTabWidget) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7tabIconEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget7tabIconEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTabWidget::isTabEnabled(int index);
impl /*struct*/ QTabWidget {
  pub fn isTabEnabled<RetType, T: QTabWidget_isTabEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTabEnabled(self);
    // return 1;
  }
}

pub trait QTabWidget_isTabEnabled<RetType> {
  fn isTabEnabled(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::isTabEnabled(int index);
impl<'a> /*trait*/ QTabWidget_isTabEnabled<i8> for (i32) {
  fn isTabEnabled(self , rsthis: & QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12isTabEnabledEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget12isTabEnabledEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabBarAutoHide(bool enabled);
impl /*struct*/ QTabWidget {
  pub fn setTabBarAutoHide<RetType, T: QTabWidget_setTabBarAutoHide<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabBarAutoHide(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabBarAutoHide<RetType> {
  fn setTabBarAutoHide(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabBarAutoHide(bool enabled);
impl<'a> /*trait*/ QTabWidget_setTabBarAutoHide<()> for (i8) {
  fn setTabBarAutoHide(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget17setTabBarAutoHideEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTabWidget17setTabBarAutoHideEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QTabWidget::iconSize();
impl /*struct*/ QTabWidget {
  pub fn iconSize<RetType, T: QTabWidget_iconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QTabWidget_iconSize<RetType> {
  fn iconSize(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  QSize QTabWidget::iconSize();
impl<'a> /*trait*/ QTabWidget_iconSize<QSize> for () {
  fn iconSize(self , rsthis: & QTabWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget8iconSizeEv()};
    let mut ret = unsafe {_ZNK10QTabWidget8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabsClosable(bool closeable);
impl /*struct*/ QTabWidget {
  pub fn setTabsClosable<RetType, T: QTabWidget_setTabsClosable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabsClosable(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabsClosable<RetType> {
  fn setTabsClosable(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabsClosable(bool closeable);
impl<'a> /*trait*/ QTabWidget_setTabsClosable<()> for (i8) {
  fn setTabsClosable(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setTabsClosableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTabWidget15setTabsClosableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QTabWidget::minimumSizeHint();
impl /*struct*/ QTabWidget {
  pub fn minimumSizeHint<RetType, T: QTabWidget_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QTabWidget_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  QSize QTabWidget::minimumSizeHint();
impl<'a> /*trait*/ QTabWidget_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QTabWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK10QTabWidget15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::setCurrentIndex(int index);
impl /*struct*/ QTabWidget {
  pub fn setCurrentIndex<RetType, T: QTabWidget_setCurrentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QTabWidget_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setCurrentIndex(int index);
impl<'a> /*trait*/ QTabWidget_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabWidget::~QTabWidget();
impl /*struct*/ QTabWidget {
  pub fn Free<RetType, T: QTabWidget_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTabWidget_Free<RetType> {
  fn Free(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::~QTabWidget();
impl<'a> /*trait*/ QTabWidget_Free<()> for () {
  fn Free(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidgetD0Ev()};
     unsafe {_ZN10QTabWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabWhatsThis(int index, const QString & text);
impl /*struct*/ QTabWidget {
  pub fn setTabWhatsThis<RetType, T: QTabWidget_setTabWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabWhatsThis(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabWhatsThis<RetType> {
  fn setTabWhatsThis(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabWhatsThis(int index, const QString & text);
impl<'a> /*trait*/ QTabWidget_setTabWhatsThis<()> for (i32, &'a QString) {
  fn setTabWhatsThis(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setTabWhatsThisEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget15setTabWhatsThisEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QTabWidget::sizeHint();
impl /*struct*/ QTabWidget {
  pub fn sizeHint<RetType, T: QTabWidget_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QTabWidget_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  QSize QTabWidget::sizeHint();
impl<'a> /*trait*/ QTabWidget_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QTabWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget8sizeHintEv()};
    let mut ret = unsafe {_ZNK10QTabWidget8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTabWidget::indexOf(QWidget * widget);
impl /*struct*/ QTabWidget {
  pub fn indexOf<RetType, T: QTabWidget_indexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QTabWidget_indexOf<RetType> {
  fn indexOf(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::indexOf(QWidget * widget);
impl<'a> /*trait*/ QTabWidget_indexOf<i32> for (&'a QWidget) {
  fn indexOf(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTabWidget7indexOfEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTabWidget::removeTab(int index);
impl /*struct*/ QTabWidget {
  pub fn removeTab<RetType, T: QTabWidget_removeTab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeTab(self);
    // return 1;
  }
}

pub trait QTabWidget_removeTab<RetType> {
  fn removeTab(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::removeTab(int index);
impl<'a> /*trait*/ QTabWidget_removeTab<()> for (i32) {
  fn removeTab(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget9removeTabEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget9removeTabEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabToolTip(int index, const QString & tip);
impl /*struct*/ QTabWidget {
  pub fn setTabToolTip<RetType, T: QTabWidget_setTabToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabToolTip(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabToolTip<RetType> {
  fn setTabToolTip(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabToolTip(int index, const QString & tip);
impl<'a> /*trait*/ QTabWidget_setTabToolTip<()> for (i32, &'a QString) {
  fn setTabToolTip(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13setTabToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget13setTabToolTipEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QTabWidget::isMovable();
impl /*struct*/ QTabWidget {
  pub fn isMovable<RetType, T: QTabWidget_isMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isMovable(self);
    // return 1;
  }
}

pub trait QTabWidget_isMovable<RetType> {
  fn isMovable(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::isMovable();
impl<'a> /*trait*/ QTabWidget_isMovable<i8> for () {
  fn isMovable(self , rsthis: & QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget9isMovableEv()};
    let mut ret = unsafe {_ZNK10QTabWidget9isMovableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTabWidget::usesScrollButtons();
impl /*struct*/ QTabWidget {
  pub fn usesScrollButtons<RetType, T: QTabWidget_usesScrollButtons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.usesScrollButtons(self);
    // return 1;
  }
}

pub trait QTabWidget_usesScrollButtons<RetType> {
  fn usesScrollButtons(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::usesScrollButtons();
impl<'a> /*trait*/ QTabWidget_usesScrollButtons<i8> for () {
  fn usesScrollButtons(self , rsthis: & QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget17usesScrollButtonsEv()};
    let mut ret = unsafe {_ZNK10QTabWidget17usesScrollButtonsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTabWidget::tabBarDoubleClicked(int index);
impl /*struct*/ QTabWidget {
  pub fn tabBarDoubleClicked<RetType, T: QTabWidget_tabBarDoubleClicked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabBarDoubleClicked(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBarDoubleClicked<RetType> {
  fn tabBarDoubleClicked(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::tabBarDoubleClicked(int index);
impl<'a> /*trait*/ QTabWidget_tabBarDoubleClicked<()> for (i32) {
  fn tabBarDoubleClicked(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget19tabBarDoubleClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget19tabBarDoubleClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTabWidget::currentIndex();
impl /*struct*/ QTabWidget {
  pub fn currentIndex<RetType, T: QTabWidget_currentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QTabWidget_currentIndex<RetType> {
  fn currentIndex(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::currentIndex();
impl<'a> /*trait*/ QTabWidget_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12currentIndexEv()};
    let mut ret = unsafe {_ZNK10QTabWidget12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QString & );
impl<'a> /*trait*/ QTabWidget_insertTab<i32> for (i32, &'a QWidget, &'a QString) {
  fn insertTab(self , rsthis: & QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget9insertTabEiP7QWidgetRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTabWidget9insertTabEiP7QWidgetRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabEnabled(int index, bool );
impl /*struct*/ QTabWidget {
  pub fn setTabEnabled<RetType, T: QTabWidget_setTabEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabEnabled(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabEnabled<RetType> {
  fn setTabEnabled(self , rsthis: & QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabEnabled(int index, bool );
impl<'a> /*trait*/ QTabWidget_setTabEnabled<()> for (i32, i8) {
  fn setTabEnabled(self , rsthis: & QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13setTabEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN10QTabWidget13setTabEnabledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

#[derive(Default)] // for QTabWidget_tabCloseRequested
pub struct QTabWidget_tabCloseRequested_signal{poi:u64}
impl /* struct */ QTabWidget {
  pub fn tabCloseRequested_1(&self) -> QTabWidget_tabCloseRequested_signal {
     return QTabWidget_tabCloseRequested_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTabWidget_tabCloseRequested_signal {
  pub fn connect<T: QTabWidget_tabCloseRequested_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTabWidget_tabCloseRequested_signal_connect {
  fn connect(self, sigthis: QTabWidget_tabCloseRequested_signal);
}

#[derive(Default)] // for QTabWidget_tabBarDoubleClicked
pub struct QTabWidget_tabBarDoubleClicked_signal{poi:u64}
impl /* struct */ QTabWidget {
  pub fn tabBarDoubleClicked_1(&self) -> QTabWidget_tabBarDoubleClicked_signal {
     return QTabWidget_tabBarDoubleClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTabWidget_tabBarDoubleClicked_signal {
  pub fn connect<T: QTabWidget_tabBarDoubleClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTabWidget_tabBarDoubleClicked_signal_connect {
  fn connect(self, sigthis: QTabWidget_tabBarDoubleClicked_signal);
}

#[derive(Default)] // for QTabWidget_tabBarClicked
pub struct QTabWidget_tabBarClicked_signal{poi:u64}
impl /* struct */ QTabWidget {
  pub fn tabBarClicked_1(&self) -> QTabWidget_tabBarClicked_signal {
     return QTabWidget_tabBarClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTabWidget_tabBarClicked_signal {
  pub fn connect<T: QTabWidget_tabBarClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTabWidget_tabBarClicked_signal_connect {
  fn connect(self, sigthis: QTabWidget_tabBarClicked_signal);
}

#[derive(Default)] // for QTabWidget_currentChanged
pub struct QTabWidget_currentChanged_signal{poi:u64}
impl /* struct */ QTabWidget {
  pub fn currentChanged_1(&self) -> QTabWidget_currentChanged_signal {
     return QTabWidget_currentChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTabWidget_currentChanged_signal {
  pub fn connect<T: QTabWidget_currentChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTabWidget_currentChanged_signal_connect {
  fn connect(self, sigthis: QTabWidget_currentChanged_signal);
}

// tabBarDoubleClicked(int)
extern fn QTabWidget_tabBarDoubleClicked_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QTabWidget_tabBarDoubleClicked_signal_connect_cb_box_0(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QTabWidget_tabBarDoubleClicked_signal_connect for fn(i32) {
  fn connect(self, sigthis: QTabWidget_tabBarDoubleClicked_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabWidget_tabBarDoubleClicked_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTabWidget_SlotProxy_connect__ZN10QTabWidget19tabBarDoubleClickedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTabWidget_tabBarDoubleClicked_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QTabWidget_tabBarDoubleClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabWidget_tabBarDoubleClicked_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTabWidget_SlotProxy_connect__ZN10QTabWidget19tabBarDoubleClickedEi(arg0, arg1, arg2)};
  }
}
// tabBarClicked(int)
extern fn QTabWidget_tabBarClicked_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QTabWidget_tabBarClicked_signal_connect_cb_box_1(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QTabWidget_tabBarClicked_signal_connect for fn(i32) {
  fn connect(self, sigthis: QTabWidget_tabBarClicked_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabWidget_tabBarClicked_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTabWidget_SlotProxy_connect__ZN10QTabWidget13tabBarClickedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTabWidget_tabBarClicked_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QTabWidget_tabBarClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabWidget_tabBarClicked_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTabWidget_SlotProxy_connect__ZN10QTabWidget13tabBarClickedEi(arg0, arg1, arg2)};
  }
}
// tabCloseRequested(int)
extern fn QTabWidget_tabCloseRequested_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QTabWidget_tabCloseRequested_signal_connect_cb_box_2(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QTabWidget_tabCloseRequested_signal_connect for fn(i32) {
  fn connect(self, sigthis: QTabWidget_tabCloseRequested_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabWidget_tabCloseRequested_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTabWidget_SlotProxy_connect__ZN10QTabWidget17tabCloseRequestedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTabWidget_tabCloseRequested_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QTabWidget_tabCloseRequested_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabWidget_tabCloseRequested_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTabWidget_SlotProxy_connect__ZN10QTabWidget17tabCloseRequestedEi(arg0, arg1, arg2)};
  }
}
// currentChanged(int)
extern fn QTabWidget_currentChanged_signal_connect_cb_3(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QTabWidget_currentChanged_signal_connect_cb_box_3(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QTabWidget_currentChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QTabWidget_currentChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabWidget_currentChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTabWidget_SlotProxy_connect__ZN10QTabWidget14currentChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTabWidget_currentChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QTabWidget_currentChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabWidget_currentChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QTabWidget_SlotProxy_connect__ZN10QTabWidget14currentChangedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

