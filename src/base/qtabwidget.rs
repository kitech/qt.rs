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
use super::qtabbar::QTabBar;
use super::qicon::QIcon;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTabWidget::setCurrentWidget(QWidget * widget);
  fn _ZN10QTabWidget16setCurrentWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTabWidget::count();
  fn _ZNK10QTabWidget5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTabWidget::tabCloseRequested(int index);
  fn _ZN10QTabWidget17tabCloseRequestedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTabWidget::setDocumentMode(bool set);
  fn _ZN10QTabWidget15setDocumentModeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QTabWidget::heightForWidth(int width);
  fn _ZNK10QTabWidget14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTabWidget::addTab(QWidget * widget, const QString & );
  fn _ZN10QTabWidget6addTabEP7QWidgetRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  QString QTabWidget::tabText(int index);
  fn _ZNK10QTabWidget7tabTextEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTabWidget::clear();
  fn _ZN10QTabWidget5clearEv(qthis: *mut c_void) ;
  // proto:  bool QTabWidget::hasHeightForWidth();
  fn _ZNK10QTabWidget17hasHeightForWidthEv(qthis: *mut c_void) -> int8_t;
  // proto:  QTabBar * QTabWidget::tabBar();
  fn _ZNK10QTabWidget6tabBarEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTabWidget::tabsClosable();
  fn _ZNK10QTabWidget12tabsClosableEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QIcon & icon, const QString & label);
  fn _ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_int;
  // proto:  int QTabWidget::addTab(QWidget * widget, const QIcon & icon, const QString & label);
  fn _ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
  // proto:  void QTabWidget::setUsesScrollButtons(bool useButtons);
  fn _ZN10QTabWidget20setUsesScrollButtonsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  const QMetaObject * QTabWidget::metaObject();
  fn _ZNK10QTabWidget10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QString QTabWidget::tabToolTip(int index);
  fn _ZNK10QTabWidget10tabToolTipEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QWidget * QTabWidget::currentWidget();
  fn _ZNK10QTabWidget13currentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabWidget::setIconSize(const QSize & size);
  fn _ZN10QTabWidget11setIconSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QTabWidget::widget(int index);
  fn _ZNK10QTabWidget6widgetEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTabWidget::setMovable(bool movable);
  fn _ZN10QTabWidget10setMovableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QTabWidget::documentMode();
  fn _ZNK10QTabWidget12documentModeEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QTabWidget::tabWhatsThis(int index);
  fn _ZNK10QTabWidget12tabWhatsThisEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTabWidget::tabBarClicked(int index);
  fn _ZN10QTabWidget13tabBarClickedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTabWidget::setTabText(int index, const QString & );
  fn _ZN10QTabWidget10setTabTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTabWidget::NewQTabWidget(const QTabWidget & );
  fn _ZN10QTabWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTabWidget::NewQTabWidget(QWidget * parent);
  fn _ZN10QTabWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QTabWidget::tabBarAutoHide();
  fn _ZNK10QTabWidget14tabBarAutoHideEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTabWidget::currentChanged(int index);
  fn _ZN10QTabWidget14currentChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTabWidget::setTabIcon(int index, const QIcon & icon);
  fn _ZN10QTabWidget10setTabIconEiRK5QIcon(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QIcon QTabWidget::tabIcon(int index);
  fn _ZNK10QTabWidget7tabIconEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabWidget::isTabEnabled(int index);
  fn _ZNK10QTabWidget12isTabEnabledEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QTabWidget::setTabBarAutoHide(bool enabled);
  fn _ZN10QTabWidget17setTabBarAutoHideEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QSize QTabWidget::iconSize();
  fn _ZNK10QTabWidget8iconSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabWidget::setTabsClosable(bool closeable);
  fn _ZN10QTabWidget15setTabsClosableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QSize QTabWidget::minimumSizeHint();
  fn _ZNK10QTabWidget15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabWidget::setCurrentIndex(int index);
  fn _ZN10QTabWidget15setCurrentIndexEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTabWidget::FreeQTabWidget();
  fn _ZN10QTabWidgetD0Ev(qthis: *mut c_void) ;
  // proto:  void QTabWidget::setTabWhatsThis(int index, const QString & text);
  fn _ZN10QTabWidget15setTabWhatsThisEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  QSize QTabWidget::sizeHint();
  fn _ZNK10QTabWidget8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTabWidget::indexOf(QWidget * widget);
  fn _ZNK10QTabWidget7indexOfEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QTabWidget::removeTab(int index);
  fn _ZN10QTabWidget9removeTabEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTabWidget::setTabToolTip(int index, const QString & tip);
  fn _ZN10QTabWidget13setTabToolTipEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  bool QTabWidget::isMovable();
  fn _ZNK10QTabWidget9isMovableEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTabWidget::usesScrollButtons();
  fn _ZNK10QTabWidget17usesScrollButtonsEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTabWidget::tabBarDoubleClicked(int index);
  fn _ZN10QTabWidget19tabBarDoubleClickedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTabWidget::currentIndex();
  fn _ZNK10QTabWidget12currentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QString & );
  fn _ZN10QTabWidget9insertTabEiP7QWidgetRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
  // proto:  void QTabWidget::setTabEnabled(int index, bool );
  fn _ZN10QTabWidget13setTabEnabledEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
}

// body block begin
// class sizeof(QTabWidget)=1
pub struct QTabWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTabWidget {
  pub fn setCurrentWidget<T: QTabWidget_setCurrentWidget>(&mut self, value: T)  {
     value.setCurrentWidget(self);
    // return 1;
  }
}

pub trait QTabWidget_setCurrentWidget {
  fn setCurrentWidget(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setCurrentWidget(QWidget * widget);
impl<'a> /*trait*/ QTabWidget_setCurrentWidget for (&'a mut QWidget) {
  fn setCurrentWidget(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget16setCurrentWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn count<T: QTabWidget_count>(&mut self, value: T) -> i32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QTabWidget_count {
  fn count(self, rsthis: &mut QTabWidget) -> i32;
}

// proto:  int QTabWidget::count();
impl<'a> /*trait*/ QTabWidget_count for () {
  fn count(self, rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget5countEv()};
    let mut ret = unsafe {_ZNK10QTabWidget5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabCloseRequested<T: QTabWidget_tabCloseRequested>(&mut self, value: T)  {
     value.tabCloseRequested(self);
    // return 1;
  }
}

pub trait QTabWidget_tabCloseRequested {
  fn tabCloseRequested(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::tabCloseRequested(int index);
impl<'a> /*trait*/ QTabWidget_tabCloseRequested for (i32) {
  fn tabCloseRequested(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget17tabCloseRequestedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget17tabCloseRequestedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setDocumentMode<T: QTabWidget_setDocumentMode>(&mut self, value: T)  {
     value.setDocumentMode(self);
    // return 1;
  }
}

pub trait QTabWidget_setDocumentMode {
  fn setDocumentMode(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setDocumentMode(bool set);
impl<'a> /*trait*/ QTabWidget_setDocumentMode for (i8) {
  fn setDocumentMode(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setDocumentModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QTabWidget15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn heightForWidth<T: QTabWidget_heightForWidth>(&mut self, value: T) -> i32 {
    return value.heightForWidth(self);
    // return 1;
  }
}

pub trait QTabWidget_heightForWidth {
  fn heightForWidth(self, rsthis: &mut QTabWidget) -> i32;
}

// proto:  int QTabWidget::heightForWidth(int width);
impl<'a> /*trait*/ QTabWidget_heightForWidth for (i32) {
  fn heightForWidth(self, rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn addTab<T: QTabWidget_addTab>(&mut self, value: T) -> i32 {
    return value.addTab(self);
    // return 1;
  }
}

pub trait QTabWidget_addTab {
  fn addTab(self, rsthis: &mut QTabWidget) -> i32;
}

// proto:  int QTabWidget::addTab(QWidget * widget, const QString & );
impl<'a> /*trait*/ QTabWidget_addTab for (&'a mut QWidget, &'a  QString) {
  fn addTab(self, rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget6addTabEP7QWidgetRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTabWidget6addTabEP7QWidgetRK7QString(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabText<T: QTabWidget_tabText>(&mut self, value: T) -> QString {
    return value.tabText(self);
    // return 1;
  }
}

pub trait QTabWidget_tabText {
  fn tabText(self, rsthis: &mut QTabWidget) -> QString;
}

// proto:  QString QTabWidget::tabText(int index);
impl<'a> /*trait*/ QTabWidget_tabText for (i32) {
  fn tabText(self, rsthis: &mut QTabWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7tabTextEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget7tabTextEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn clear<T: QTabWidget_clear>(&mut self, value: T)  {
     value.clear(self);
    // return 1;
  }
}

pub trait QTabWidget_clear {
  fn clear(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::clear();
impl<'a> /*trait*/ QTabWidget_clear for () {
  fn clear(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget5clearEv()};
     unsafe {_ZN10QTabWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn hasHeightForWidth<T: QTabWidget_hasHeightForWidth>(&mut self, value: T) -> i8 {
    return value.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QTabWidget_hasHeightForWidth {
  fn hasHeightForWidth(self, rsthis: &mut QTabWidget) -> i8;
}

// proto:  bool QTabWidget::hasHeightForWidth();
impl<'a> /*trait*/ QTabWidget_hasHeightForWidth for () {
  fn hasHeightForWidth(self, rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK10QTabWidget17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabBar<T: QTabWidget_tabBar>(&mut self, value: T) -> QTabBar {
    return value.tabBar(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBar {
  fn tabBar(self, rsthis: &mut QTabWidget) -> QTabBar;
}

// proto:  QTabBar * QTabWidget::tabBar();
impl<'a> /*trait*/ QTabWidget_tabBar for () {
  fn tabBar(self, rsthis: &mut QTabWidget) -> QTabBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget6tabBarEv()};
    let mut ret = unsafe {_ZNK10QTabWidget6tabBarEv(rsthis.qclsinst)};
    let mut ret1 = QTabBar{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabsClosable<T: QTabWidget_tabsClosable>(&mut self, value: T) -> i8 {
    return value.tabsClosable(self);
    // return 1;
  }
}

pub trait QTabWidget_tabsClosable {
  fn tabsClosable(self, rsthis: &mut QTabWidget) -> i8;
}

// proto:  bool QTabWidget::tabsClosable();
impl<'a> /*trait*/ QTabWidget_tabsClosable for () {
  fn tabsClosable(self, rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12tabsClosableEv()};
    let mut ret = unsafe {_ZNK10QTabWidget12tabsClosableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn insertTab<T: QTabWidget_insertTab>(&mut self, value: T) -> i32 {
    return value.insertTab(self);
    // return 1;
  }
}

pub trait QTabWidget_insertTab {
  fn insertTab(self, rsthis: &mut QTabWidget) -> i32;
}

// proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QIcon & icon, const QString & label);
impl<'a> /*trait*/ QTabWidget_insertTab for (i32, &'a mut QWidget, &'a  QIcon, &'a  QString) {
  fn insertTab(self, rsthis: &mut QTabWidget) -> i32 {
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
impl<'a> /*trait*/ QTabWidget_addTab for (&'a mut QWidget, &'a  QIcon, &'a  QString) {
  fn addTab(self, rsthis: &mut QTabWidget) -> i32 {
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

impl /*struct*/ QTabWidget {
  pub fn setUsesScrollButtons<T: QTabWidget_setUsesScrollButtons>(&mut self, value: T)  {
     value.setUsesScrollButtons(self);
    // return 1;
  }
}

pub trait QTabWidget_setUsesScrollButtons {
  fn setUsesScrollButtons(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setUsesScrollButtons(bool useButtons);
impl<'a> /*trait*/ QTabWidget_setUsesScrollButtons for (i8) {
  fn setUsesScrollButtons(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget20setUsesScrollButtonsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QTabWidget20setUsesScrollButtonsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn metaObject<T: QTabWidget_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTabWidget_metaObject {
  fn metaObject(self, rsthis: &mut QTabWidget) ;
}

// proto:  const QMetaObject * QTabWidget::metaObject();
impl<'a> /*trait*/ QTabWidget_metaObject for () {
  fn metaObject(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget10metaObjectEv()};
     unsafe {_ZNK10QTabWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabToolTip<T: QTabWidget_tabToolTip>(&mut self, value: T) -> QString {
    return value.tabToolTip(self);
    // return 1;
  }
}

pub trait QTabWidget_tabToolTip {
  fn tabToolTip(self, rsthis: &mut QTabWidget) -> QString;
}

// proto:  QString QTabWidget::tabToolTip(int index);
impl<'a> /*trait*/ QTabWidget_tabToolTip for (i32) {
  fn tabToolTip(self, rsthis: &mut QTabWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget10tabToolTipEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget10tabToolTipEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn currentWidget<T: QTabWidget_currentWidget>(&mut self, value: T) -> QWidget {
    return value.currentWidget(self);
    // return 1;
  }
}

pub trait QTabWidget_currentWidget {
  fn currentWidget(self, rsthis: &mut QTabWidget) -> QWidget;
}

// proto:  QWidget * QTabWidget::currentWidget();
impl<'a> /*trait*/ QTabWidget_currentWidget for () {
  fn currentWidget(self, rsthis: &mut QTabWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget13currentWidgetEv()};
    let mut ret = unsafe {_ZNK10QTabWidget13currentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setIconSize<T: QTabWidget_setIconSize>(&mut self, value: T)  {
     value.setIconSize(self);
    // return 1;
  }
}

pub trait QTabWidget_setIconSize {
  fn setIconSize(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setIconSize(const QSize & size);
impl<'a> /*trait*/ QTabWidget_setIconSize for (&'a  QSize) {
  fn setIconSize(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn widget<T: QTabWidget_widget>(&mut self, value: T) -> QWidget {
    return value.widget(self);
    // return 1;
  }
}

pub trait QTabWidget_widget {
  fn widget(self, rsthis: &mut QTabWidget) -> QWidget;
}

// proto:  QWidget * QTabWidget::widget(int index);
impl<'a> /*trait*/ QTabWidget_widget for (i32) {
  fn widget(self, rsthis: &mut QTabWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget6widgetEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget6widgetEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setMovable<T: QTabWidget_setMovable>(&mut self, value: T)  {
     value.setMovable(self);
    // return 1;
  }
}

pub trait QTabWidget_setMovable {
  fn setMovable(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setMovable(bool movable);
impl<'a> /*trait*/ QTabWidget_setMovable for (i8) {
  fn setMovable(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setMovableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QTabWidget10setMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn documentMode<T: QTabWidget_documentMode>(&mut self, value: T) -> i8 {
    return value.documentMode(self);
    // return 1;
  }
}

pub trait QTabWidget_documentMode {
  fn documentMode(self, rsthis: &mut QTabWidget) -> i8;
}

// proto:  bool QTabWidget::documentMode();
impl<'a> /*trait*/ QTabWidget_documentMode for () {
  fn documentMode(self, rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12documentModeEv()};
    let mut ret = unsafe {_ZNK10QTabWidget12documentModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabWhatsThis<T: QTabWidget_tabWhatsThis>(&mut self, value: T) -> QString {
    return value.tabWhatsThis(self);
    // return 1;
  }
}

pub trait QTabWidget_tabWhatsThis {
  fn tabWhatsThis(self, rsthis: &mut QTabWidget) -> QString;
}

// proto:  QString QTabWidget::tabWhatsThis(int index);
impl<'a> /*trait*/ QTabWidget_tabWhatsThis for (i32) {
  fn tabWhatsThis(self, rsthis: &mut QTabWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12tabWhatsThisEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget12tabWhatsThisEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabBarClicked<T: QTabWidget_tabBarClicked>(&mut self, value: T)  {
     value.tabBarClicked(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBarClicked {
  fn tabBarClicked(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::tabBarClicked(int index);
impl<'a> /*trait*/ QTabWidget_tabBarClicked for (i32) {
  fn tabBarClicked(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13tabBarClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget13tabBarClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabText<T: QTabWidget_setTabText>(&mut self, value: T)  {
     value.setTabText(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabText {
  fn setTabText(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setTabText(int index, const QString & );
impl<'a> /*trait*/ QTabWidget_setTabText for (i32, &'a  QString) {
  fn setTabText(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setTabTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget10setTabTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn NewQTabWidget<T: QTabWidget_NewQTabWidget>(value: T) -> QTabWidget {
    let rsthis = value.NewQTabWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QTabWidget_NewQTabWidget {
  fn NewQTabWidget(self) -> QTabWidget;
}

// proto: void QTabWidget::NewQTabWidget(const QTabWidget & );
impl<'a> /*trait*/ QTabWidget_NewQTabWidget for (&'a  QTabWidget) {
  fn NewQTabWidget(self) -> QTabWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTabWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QTabWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QTabWidget::NewQTabWidget(QWidget * parent);
impl<'a> /*trait*/ QTabWidget_NewQTabWidget for (&'a mut QWidget) {
  fn NewQTabWidget(self) -> QTabWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidgetC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTabWidgetC1EP7QWidget(qthis, arg0)};
    let rsthis = QTabWidget{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabBarAutoHide<T: QTabWidget_tabBarAutoHide>(&mut self, value: T) -> i8 {
    return value.tabBarAutoHide(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBarAutoHide {
  fn tabBarAutoHide(self, rsthis: &mut QTabWidget) -> i8;
}

// proto:  bool QTabWidget::tabBarAutoHide();
impl<'a> /*trait*/ QTabWidget_tabBarAutoHide for () {
  fn tabBarAutoHide(self, rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget14tabBarAutoHideEv()};
    let mut ret = unsafe {_ZNK10QTabWidget14tabBarAutoHideEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn currentChanged<T: QTabWidget_currentChanged>(&mut self, value: T)  {
     value.currentChanged(self);
    // return 1;
  }
}

pub trait QTabWidget_currentChanged {
  fn currentChanged(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::currentChanged(int index);
impl<'a> /*trait*/ QTabWidget_currentChanged for (i32) {
  fn currentChanged(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget14currentChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget14currentChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabIcon<T: QTabWidget_setTabIcon>(&mut self, value: T)  {
     value.setTabIcon(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabIcon {
  fn setTabIcon(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setTabIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QTabWidget_setTabIcon for (i32, &'a  QIcon) {
  fn setTabIcon(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setTabIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget10setTabIconEiRK5QIcon(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabIcon<T: QTabWidget_tabIcon>(&mut self, value: T) -> QIcon {
    return value.tabIcon(self);
    // return 1;
  }
}

pub trait QTabWidget_tabIcon {
  fn tabIcon(self, rsthis: &mut QTabWidget) -> QIcon;
}

// proto:  QIcon QTabWidget::tabIcon(int index);
impl<'a> /*trait*/ QTabWidget_tabIcon for (i32) {
  fn tabIcon(self, rsthis: &mut QTabWidget) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7tabIconEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget7tabIconEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn isTabEnabled<T: QTabWidget_isTabEnabled>(&mut self, value: T) -> i8 {
    return value.isTabEnabled(self);
    // return 1;
  }
}

pub trait QTabWidget_isTabEnabled {
  fn isTabEnabled(self, rsthis: &mut QTabWidget) -> i8;
}

// proto:  bool QTabWidget::isTabEnabled(int index);
impl<'a> /*trait*/ QTabWidget_isTabEnabled for (i32) {
  fn isTabEnabled(self, rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12isTabEnabledEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget12isTabEnabledEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabBarAutoHide<T: QTabWidget_setTabBarAutoHide>(&mut self, value: T)  {
     value.setTabBarAutoHide(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabBarAutoHide {
  fn setTabBarAutoHide(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setTabBarAutoHide(bool enabled);
impl<'a> /*trait*/ QTabWidget_setTabBarAutoHide for (i8) {
  fn setTabBarAutoHide(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget17setTabBarAutoHideEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QTabWidget17setTabBarAutoHideEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn iconSize<T: QTabWidget_iconSize>(&mut self, value: T) -> QSize {
    return value.iconSize(self);
    // return 1;
  }
}

pub trait QTabWidget_iconSize {
  fn iconSize(self, rsthis: &mut QTabWidget) -> QSize;
}

// proto:  QSize QTabWidget::iconSize();
impl<'a> /*trait*/ QTabWidget_iconSize for () {
  fn iconSize(self, rsthis: &mut QTabWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget8iconSizeEv()};
    let mut ret = unsafe {_ZNK10QTabWidget8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabsClosable<T: QTabWidget_setTabsClosable>(&mut self, value: T)  {
     value.setTabsClosable(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabsClosable {
  fn setTabsClosable(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setTabsClosable(bool closeable);
impl<'a> /*trait*/ QTabWidget_setTabsClosable for (i8) {
  fn setTabsClosable(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setTabsClosableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN10QTabWidget15setTabsClosableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn minimumSizeHint<T: QTabWidget_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QTabWidget_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QTabWidget) -> QSize;
}

// proto:  QSize QTabWidget::minimumSizeHint();
impl<'a> /*trait*/ QTabWidget_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QTabWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK10QTabWidget15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setCurrentIndex<T: QTabWidget_setCurrentIndex>(&mut self, value: T)  {
     value.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QTabWidget_setCurrentIndex {
  fn setCurrentIndex(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setCurrentIndex(int index);
impl<'a> /*trait*/ QTabWidget_setCurrentIndex for (i32) {
  fn setCurrentIndex(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn FreeQTabWidget<T: QTabWidget_FreeQTabWidget>(&mut self, value: T)  {
     value.FreeQTabWidget(self);
    // return 1;
  }
}

pub trait QTabWidget_FreeQTabWidget {
  fn FreeQTabWidget(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::FreeQTabWidget();
impl<'a> /*trait*/ QTabWidget_FreeQTabWidget for () {
  fn FreeQTabWidget(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidgetD0Ev()};
     unsafe {_ZN10QTabWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabWhatsThis<T: QTabWidget_setTabWhatsThis>(&mut self, value: T)  {
     value.setTabWhatsThis(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabWhatsThis {
  fn setTabWhatsThis(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setTabWhatsThis(int index, const QString & text);
impl<'a> /*trait*/ QTabWidget_setTabWhatsThis for (i32, &'a  QString) {
  fn setTabWhatsThis(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setTabWhatsThisEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget15setTabWhatsThisEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn sizeHint<T: QTabWidget_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QTabWidget_sizeHint {
  fn sizeHint(self, rsthis: &mut QTabWidget) -> QSize;
}

// proto:  QSize QTabWidget::sizeHint();
impl<'a> /*trait*/ QTabWidget_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QTabWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget8sizeHintEv()};
    let mut ret = unsafe {_ZNK10QTabWidget8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn indexOf<T: QTabWidget_indexOf>(&mut self, value: T) -> i32 {
    return value.indexOf(self);
    // return 1;
  }
}

pub trait QTabWidget_indexOf {
  fn indexOf(self, rsthis: &mut QTabWidget) -> i32;
}

// proto:  int QTabWidget::indexOf(QWidget * widget);
impl<'a> /*trait*/ QTabWidget_indexOf for (&'a mut QWidget) {
  fn indexOf(self, rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTabWidget7indexOfEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn removeTab<T: QTabWidget_removeTab>(&mut self, value: T)  {
     value.removeTab(self);
    // return 1;
  }
}

pub trait QTabWidget_removeTab {
  fn removeTab(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::removeTab(int index);
impl<'a> /*trait*/ QTabWidget_removeTab for (i32) {
  fn removeTab(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget9removeTabEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget9removeTabEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabToolTip<T: QTabWidget_setTabToolTip>(&mut self, value: T)  {
     value.setTabToolTip(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabToolTip {
  fn setTabToolTip(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setTabToolTip(int index, const QString & tip);
impl<'a> /*trait*/ QTabWidget_setTabToolTip for (i32, &'a  QString) {
  fn setTabToolTip(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13setTabToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget13setTabToolTipEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn isMovable<T: QTabWidget_isMovable>(&mut self, value: T) -> i8 {
    return value.isMovable(self);
    // return 1;
  }
}

pub trait QTabWidget_isMovable {
  fn isMovable(self, rsthis: &mut QTabWidget) -> i8;
}

// proto:  bool QTabWidget::isMovable();
impl<'a> /*trait*/ QTabWidget_isMovable for () {
  fn isMovable(self, rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget9isMovableEv()};
    let mut ret = unsafe {_ZNK10QTabWidget9isMovableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn usesScrollButtons<T: QTabWidget_usesScrollButtons>(&mut self, value: T) -> i8 {
    return value.usesScrollButtons(self);
    // return 1;
  }
}

pub trait QTabWidget_usesScrollButtons {
  fn usesScrollButtons(self, rsthis: &mut QTabWidget) -> i8;
}

// proto:  bool QTabWidget::usesScrollButtons();
impl<'a> /*trait*/ QTabWidget_usesScrollButtons for () {
  fn usesScrollButtons(self, rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget17usesScrollButtonsEv()};
    let mut ret = unsafe {_ZNK10QTabWidget17usesScrollButtonsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabBarDoubleClicked<T: QTabWidget_tabBarDoubleClicked>(&mut self, value: T)  {
     value.tabBarDoubleClicked(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBarDoubleClicked {
  fn tabBarDoubleClicked(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::tabBarDoubleClicked(int index);
impl<'a> /*trait*/ QTabWidget_tabBarDoubleClicked for (i32) {
  fn tabBarDoubleClicked(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget19tabBarDoubleClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget19tabBarDoubleClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn currentIndex<T: QTabWidget_currentIndex>(&mut self, value: T) -> i32 {
    return value.currentIndex(self);
    // return 1;
  }
}

pub trait QTabWidget_currentIndex {
  fn currentIndex(self, rsthis: &mut QTabWidget) -> i32;
}

// proto:  int QTabWidget::currentIndex();
impl<'a> /*trait*/ QTabWidget_currentIndex for () {
  fn currentIndex(self, rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12currentIndexEv()};
    let mut ret = unsafe {_ZNK10QTabWidget12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QString & );
impl<'a> /*trait*/ QTabWidget_insertTab for (i32, &'a mut QWidget, &'a  QString) {
  fn insertTab(self, rsthis: &mut QTabWidget) -> i32 {
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

impl /*struct*/ QTabWidget {
  pub fn setTabEnabled<T: QTabWidget_setTabEnabled>(&mut self, value: T)  {
     value.setTabEnabled(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabEnabled {
  fn setTabEnabled(self, rsthis: &mut QTabWidget) ;
}

// proto:  void QTabWidget::setTabEnabled(int index, bool );
impl<'a> /*trait*/ QTabWidget_setTabEnabled for (i32, i8) {
  fn setTabEnabled(self, rsthis: &mut QTabWidget)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13setTabEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN10QTabWidget13setTabEnabledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

