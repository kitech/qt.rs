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
use super::qicon::QIcon;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QTabWidget::setCurrentWidget(QWidget * widget);
  fn _ZN10QTabWidget16setCurrentWidgetEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: int QTabWidget::count();
  fn _ZNK10QTabWidget5countEv() -> i32;
  // proto: void QTabWidget::tabCloseRequested(int index);
  fn _ZN10QTabWidget17tabCloseRequestedEi(arg0: c_int) -> i32;
  // proto: void QTabWidget::setDocumentMode(bool set);
  fn _ZN10QTabWidget15setDocumentModeEb(arg0: int8_t) -> i32;
  // proto: int QTabWidget::heightForWidth(int width);
  fn _ZNK10QTabWidget14heightForWidthEi(arg0: c_int) -> i32;
  // proto: int QTabWidget::addTab(QWidget * widget, const QString & );
  fn _ZN10QTabWidget6addTabEP7QWidgetRK7QString(arg0: *mut c_void, arg1: *const c_void) -> i32;
  // proto: QString QTabWidget::tabText(int index);
  fn _ZNK10QTabWidget7tabTextEi(arg0: c_int) -> i32;
  // proto: void QTabWidget::clear();
  fn _ZN10QTabWidget5clearEv() -> i32;
  // proto: bool QTabWidget::hasHeightForWidth();
  fn _ZNK10QTabWidget17hasHeightForWidthEv() -> i32;
  // proto: QTabBar * QTabWidget::tabBar();
  fn _ZNK10QTabWidget6tabBarEv() -> i32;
  // proto: bool QTabWidget::tabsClosable();
  fn _ZNK10QTabWidget12tabsClosableEv() -> i32;
  // proto: int QTabWidget::insertTab(int index, QWidget * widget, const QIcon & icon, const QString & label);
  fn _ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString(arg0: c_int, arg1: *mut c_void, arg2: *const c_void, arg3: *const c_void) -> i32;
  // proto: int QTabWidget::addTab(QWidget * widget, const QIcon & icon, const QString & label);
  fn _ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString(arg0: *mut c_void, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QTabWidget::setUsesScrollButtons(bool useButtons);
  fn _ZN10QTabWidget20setUsesScrollButtonsEb(arg0: int8_t) -> i32;
  // proto: const QMetaObject * QTabWidget::metaObject();
  fn _ZNK10QTabWidget10metaObjectEv() -> i32;
  // proto: QString QTabWidget::tabToolTip(int index);
  fn _ZNK10QTabWidget10tabToolTipEi(arg0: c_int) -> i32;
  // proto: QWidget * QTabWidget::currentWidget();
  fn _ZNK10QTabWidget13currentWidgetEv() -> i32;
  // proto: void QTabWidget::setIconSize(const QSize & size);
  fn _ZN10QTabWidget11setIconSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: QWidget * QTabWidget::widget(int index);
  fn _ZNK10QTabWidget6widgetEi(arg0: c_int) -> i32;
  // proto: void QTabWidget::setMovable(bool movable);
  fn _ZN10QTabWidget10setMovableEb(arg0: int8_t) -> i32;
  // proto: bool QTabWidget::documentMode();
  fn _ZNK10QTabWidget12documentModeEv() -> i32;
  // proto: QString QTabWidget::tabWhatsThis(int index);
  fn _ZNK10QTabWidget12tabWhatsThisEi(arg0: c_int) -> i32;
  // proto: void QTabWidget::tabBarClicked(int index);
  fn _ZN10QTabWidget13tabBarClickedEi(arg0: c_int) -> i32;
  // proto: void QTabWidget::setTabText(int index, const QString & );
  fn _ZN10QTabWidget10setTabTextEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTabWidget::NewQTabWidget(const QTabWidget & );
  fn _ZN10QTabWidgetC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QTabWidget::NewQTabWidget(QWidget * parent);
  fn _ZN10QTabWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: bool QTabWidget::tabBarAutoHide();
  fn _ZNK10QTabWidget14tabBarAutoHideEv() -> i32;
  // proto: void QTabWidget::currentChanged(int index);
  fn _ZN10QTabWidget14currentChangedEi(arg0: c_int) -> i32;
  // proto: void QTabWidget::setTabIcon(int index, const QIcon & icon);
  fn _ZN10QTabWidget10setTabIconEiRK5QIcon(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: QIcon QTabWidget::tabIcon(int index);
  fn _ZNK10QTabWidget7tabIconEi(arg0: c_int) -> i32;
  // proto: bool QTabWidget::isTabEnabled(int index);
  fn _ZNK10QTabWidget12isTabEnabledEi(arg0: c_int) -> i32;
  // proto: void QTabWidget::setTabBarAutoHide(bool enabled);
  fn _ZN10QTabWidget17setTabBarAutoHideEb(arg0: int8_t) -> i32;
  // proto: QSize QTabWidget::iconSize();
  fn _ZNK10QTabWidget8iconSizeEv() -> i32;
  // proto: void QTabWidget::setTabsClosable(bool closeable);
  fn _ZN10QTabWidget15setTabsClosableEb(arg0: int8_t) -> i32;
  // proto: QSize QTabWidget::minimumSizeHint();
  fn _ZNK10QTabWidget15minimumSizeHintEv() -> i32;
  // proto: void QTabWidget::setCurrentIndex(int index);
  fn _ZN10QTabWidget15setCurrentIndexEi(arg0: c_int) -> i32;
  // proto: void QTabWidget::FreeQTabWidget();
  fn _ZN10QTabWidgetD0Ev() -> i32;
  // proto: void QTabWidget::setTabWhatsThis(int index, const QString & text);
  fn _ZN10QTabWidget15setTabWhatsThisEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: QSize QTabWidget::sizeHint();
  fn _ZNK10QTabWidget8sizeHintEv() -> i32;
  // proto: int QTabWidget::indexOf(QWidget * widget);
  fn _ZNK10QTabWidget7indexOfEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: void QTabWidget::removeTab(int index);
  fn _ZN10QTabWidget9removeTabEi(arg0: c_int) -> i32;
  // proto: void QTabWidget::setTabToolTip(int index, const QString & tip);
  fn _ZN10QTabWidget13setTabToolTipEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: bool QTabWidget::isMovable();
  fn _ZNK10QTabWidget9isMovableEv() -> i32;
  // proto: bool QTabWidget::usesScrollButtons();
  fn _ZNK10QTabWidget17usesScrollButtonsEv() -> i32;
  // proto: void QTabWidget::tabBarDoubleClicked(int index);
  fn _ZN10QTabWidget19tabBarDoubleClickedEi(arg0: c_int) -> i32;
  // proto: int QTabWidget::currentIndex();
  fn _ZNK10QTabWidget12currentIndexEv() -> i32;
  // proto: int QTabWidget::insertTab(int index, QWidget * widget, const QString & );
  fn _ZN10QTabWidget9insertTabEiP7QWidgetRK7QString(arg0: c_int, arg1: *mut c_void, arg2: *const c_void) -> i32;
  // proto: void QTabWidget::setTabEnabled(int index, bool );
  fn _ZN10QTabWidget13setTabEnabledEib(arg0: c_int, arg1: int8_t) -> i32;
}

// body block begin
// class sizeof(QTabWidget)=1
pub struct QTabWidget {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTabWidget {
  pub fn setCurrentWidget<T: QTabWidget_setCurrentWidget>(&mut self, value: T) -> i32 {
    value.setCurrentWidget(self);
    return 1;
  }
}

pub trait QTabWidget_setCurrentWidget {
  fn setCurrentWidget(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setCurrentWidget(QWidget * widget);
impl<'a> /*trait*/ QTabWidget_setCurrentWidget for (&'a mut QWidget) {
  fn setCurrentWidget(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTabWidget16setCurrentWidgetEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn count<T: QTabWidget_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QTabWidget_count {
  fn count(self, this: &mut QTabWidget) -> i32;
}

// proto: int QTabWidget::count();
impl<'a> /*trait*/ QTabWidget_count for () {
  fn count(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget5countEv()};
    unsafe {_ZNK10QTabWidget5countEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabCloseRequested<T: QTabWidget_tabCloseRequested>(&mut self, value: T) -> i32 {
    value.tabCloseRequested(self);
    return 1;
  }
}

pub trait QTabWidget_tabCloseRequested {
  fn tabCloseRequested(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::tabCloseRequested(int index);
impl<'a> /*trait*/ QTabWidget_tabCloseRequested for (i32) {
  fn tabCloseRequested(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget17tabCloseRequestedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTabWidget17tabCloseRequestedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setDocumentMode<T: QTabWidget_setDocumentMode>(&mut self, value: T) -> i32 {
    value.setDocumentMode(self);
    return 1;
  }
}

pub trait QTabWidget_setDocumentMode {
  fn setDocumentMode(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setDocumentMode(bool set);
impl<'a> /*trait*/ QTabWidget_setDocumentMode for (i8) {
  fn setDocumentMode(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setDocumentModeEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QTabWidget15setDocumentModeEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn heightForWidth<T: QTabWidget_heightForWidth>(&mut self, value: T) -> i32 {
    value.heightForWidth(self);
    return 1;
  }
}

pub trait QTabWidget_heightForWidth {
  fn heightForWidth(self, this: &mut QTabWidget) -> i32;
}

// proto: int QTabWidget::heightForWidth(int width);
impl<'a> /*trait*/ QTabWidget_heightForWidth for (i32) {
  fn heightForWidth(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget14heightForWidthEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTabWidget14heightForWidthEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn addTab<T: QTabWidget_addTab>(&mut self, value: T) -> i32 {
    value.addTab(self);
    return 1;
  }
}

pub trait QTabWidget_addTab {
  fn addTab(self, this: &mut QTabWidget) -> i32;
}

// proto: int QTabWidget::addTab(QWidget * widget, const QString & );
impl<'a> /*trait*/ QTabWidget_addTab for (&'a mut QWidget, &'a  QString) {
  fn addTab(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget6addTabEP7QWidgetRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QTabWidget6addTabEP7QWidgetRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabText<T: QTabWidget_tabText>(&mut self, value: T) -> i32 {
    value.tabText(self);
    return 1;
  }
}

pub trait QTabWidget_tabText {
  fn tabText(self, this: &mut QTabWidget) -> i32;
}

// proto: QString QTabWidget::tabText(int index);
impl<'a> /*trait*/ QTabWidget_tabText for (i32) {
  fn tabText(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7tabTextEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTabWidget7tabTextEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn clear<T: QTabWidget_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QTabWidget_clear {
  fn clear(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::clear();
impl<'a> /*trait*/ QTabWidget_clear for () {
  fn clear(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget5clearEv()};
    unsafe {_ZN10QTabWidget5clearEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn hasHeightForWidth<T: QTabWidget_hasHeightForWidth>(&mut self, value: T) -> i32 {
    value.hasHeightForWidth(self);
    return 1;
  }
}

pub trait QTabWidget_hasHeightForWidth {
  fn hasHeightForWidth(self, this: &mut QTabWidget) -> i32;
}

// proto: bool QTabWidget::hasHeightForWidth();
impl<'a> /*trait*/ QTabWidget_hasHeightForWidth for () {
  fn hasHeightForWidth(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget17hasHeightForWidthEv()};
    unsafe {_ZNK10QTabWidget17hasHeightForWidthEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabBar<T: QTabWidget_tabBar>(&mut self, value: T) -> i32 {
    value.tabBar(self);
    return 1;
  }
}

pub trait QTabWidget_tabBar {
  fn tabBar(self, this: &mut QTabWidget) -> i32;
}

// proto: QTabBar * QTabWidget::tabBar();
impl<'a> /*trait*/ QTabWidget_tabBar for () {
  fn tabBar(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget6tabBarEv()};
    unsafe {_ZNK10QTabWidget6tabBarEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabsClosable<T: QTabWidget_tabsClosable>(&mut self, value: T) -> i32 {
    value.tabsClosable(self);
    return 1;
  }
}

pub trait QTabWidget_tabsClosable {
  fn tabsClosable(self, this: &mut QTabWidget) -> i32;
}

// proto: bool QTabWidget::tabsClosable();
impl<'a> /*trait*/ QTabWidget_tabsClosable for () {
  fn tabsClosable(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12tabsClosableEv()};
    unsafe {_ZNK10QTabWidget12tabsClosableEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn insertTab<T: QTabWidget_insertTab>(&mut self, value: T) -> i32 {
    value.insertTab(self);
    return 1;
  }
}

pub trait QTabWidget_insertTab {
  fn insertTab(self, this: &mut QTabWidget) -> i32;
}

// proto: int QTabWidget::insertTab(int index, QWidget * widget, const QIcon & icon, const QString & label);
impl<'a> /*trait*/ QTabWidget_insertTab for (i32, &'a mut QWidget, &'a  QIcon, &'a  QString) {
  fn insertTab(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    let arg3 = self.3.qclsinst  as *const c_void;
    unsafe {_ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString(arg0, arg1, arg2, arg3)};
    return 1;
  }
}

// proto: int QTabWidget::addTab(QWidget * widget, const QIcon & icon, const QString & label);
impl<'a> /*trait*/ QTabWidget_addTab for (&'a mut QWidget, &'a  QIcon, &'a  QString) {
  fn addTab(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setUsesScrollButtons<T: QTabWidget_setUsesScrollButtons>(&mut self, value: T) -> i32 {
    value.setUsesScrollButtons(self);
    return 1;
  }
}

pub trait QTabWidget_setUsesScrollButtons {
  fn setUsesScrollButtons(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setUsesScrollButtons(bool useButtons);
impl<'a> /*trait*/ QTabWidget_setUsesScrollButtons for (i8) {
  fn setUsesScrollButtons(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget20setUsesScrollButtonsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QTabWidget20setUsesScrollButtonsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn metaObject<T: QTabWidget_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTabWidget_metaObject {
  fn metaObject(self, this: &mut QTabWidget) -> i32;
}

// proto: const QMetaObject * QTabWidget::metaObject();
impl<'a> /*trait*/ QTabWidget_metaObject for () {
  fn metaObject(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget10metaObjectEv()};
    unsafe {_ZNK10QTabWidget10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabToolTip<T: QTabWidget_tabToolTip>(&mut self, value: T) -> i32 {
    value.tabToolTip(self);
    return 1;
  }
}

pub trait QTabWidget_tabToolTip {
  fn tabToolTip(self, this: &mut QTabWidget) -> i32;
}

// proto: QString QTabWidget::tabToolTip(int index);
impl<'a> /*trait*/ QTabWidget_tabToolTip for (i32) {
  fn tabToolTip(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget10tabToolTipEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTabWidget10tabToolTipEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn currentWidget<T: QTabWidget_currentWidget>(&mut self, value: T) -> i32 {
    value.currentWidget(self);
    return 1;
  }
}

pub trait QTabWidget_currentWidget {
  fn currentWidget(self, this: &mut QTabWidget) -> i32;
}

// proto: QWidget * QTabWidget::currentWidget();
impl<'a> /*trait*/ QTabWidget_currentWidget for () {
  fn currentWidget(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget13currentWidgetEv()};
    unsafe {_ZNK10QTabWidget13currentWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setIconSize<T: QTabWidget_setIconSize>(&mut self, value: T) -> i32 {
    value.setIconSize(self);
    return 1;
  }
}

pub trait QTabWidget_setIconSize {
  fn setIconSize(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setIconSize(const QSize & size);
impl<'a> /*trait*/ QTabWidget_setIconSize for (&'a  QSize) {
  fn setIconSize(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN10QTabWidget11setIconSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn widget<T: QTabWidget_widget>(&mut self, value: T) -> i32 {
    value.widget(self);
    return 1;
  }
}

pub trait QTabWidget_widget {
  fn widget(self, this: &mut QTabWidget) -> i32;
}

// proto: QWidget * QTabWidget::widget(int index);
impl<'a> /*trait*/ QTabWidget_widget for (i32) {
  fn widget(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget6widgetEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTabWidget6widgetEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setMovable<T: QTabWidget_setMovable>(&mut self, value: T) -> i32 {
    value.setMovable(self);
    return 1;
  }
}

pub trait QTabWidget_setMovable {
  fn setMovable(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setMovable(bool movable);
impl<'a> /*trait*/ QTabWidget_setMovable for (i8) {
  fn setMovable(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setMovableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QTabWidget10setMovableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn documentMode<T: QTabWidget_documentMode>(&mut self, value: T) -> i32 {
    value.documentMode(self);
    return 1;
  }
}

pub trait QTabWidget_documentMode {
  fn documentMode(self, this: &mut QTabWidget) -> i32;
}

// proto: bool QTabWidget::documentMode();
impl<'a> /*trait*/ QTabWidget_documentMode for () {
  fn documentMode(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12documentModeEv()};
    unsafe {_ZNK10QTabWidget12documentModeEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabWhatsThis<T: QTabWidget_tabWhatsThis>(&mut self, value: T) -> i32 {
    value.tabWhatsThis(self);
    return 1;
  }
}

pub trait QTabWidget_tabWhatsThis {
  fn tabWhatsThis(self, this: &mut QTabWidget) -> i32;
}

// proto: QString QTabWidget::tabWhatsThis(int index);
impl<'a> /*trait*/ QTabWidget_tabWhatsThis for (i32) {
  fn tabWhatsThis(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12tabWhatsThisEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTabWidget12tabWhatsThisEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabBarClicked<T: QTabWidget_tabBarClicked>(&mut self, value: T) -> i32 {
    value.tabBarClicked(self);
    return 1;
  }
}

pub trait QTabWidget_tabBarClicked {
  fn tabBarClicked(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::tabBarClicked(int index);
impl<'a> /*trait*/ QTabWidget_tabBarClicked for (i32) {
  fn tabBarClicked(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13tabBarClickedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTabWidget13tabBarClickedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabText<T: QTabWidget_setTabText>(&mut self, value: T) -> i32 {
    value.setTabText(self);
    return 1;
  }
}

pub trait QTabWidget_setTabText {
  fn setTabText(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setTabText(int index, const QString & );
impl<'a> /*trait*/ QTabWidget_setTabText for (i32, &'a  QString) {
  fn setTabText(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setTabTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QTabWidget10setTabTextEiRK7QString(arg0, arg1)};
    return 1;
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
    let arg0 = self.qclsinst  as *const c_void;
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
  pub fn tabBarAutoHide<T: QTabWidget_tabBarAutoHide>(&mut self, value: T) -> i32 {
    value.tabBarAutoHide(self);
    return 1;
  }
}

pub trait QTabWidget_tabBarAutoHide {
  fn tabBarAutoHide(self, this: &mut QTabWidget) -> i32;
}

// proto: bool QTabWidget::tabBarAutoHide();
impl<'a> /*trait*/ QTabWidget_tabBarAutoHide for () {
  fn tabBarAutoHide(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget14tabBarAutoHideEv()};
    unsafe {_ZNK10QTabWidget14tabBarAutoHideEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn currentChanged<T: QTabWidget_currentChanged>(&mut self, value: T) -> i32 {
    value.currentChanged(self);
    return 1;
  }
}

pub trait QTabWidget_currentChanged {
  fn currentChanged(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::currentChanged(int index);
impl<'a> /*trait*/ QTabWidget_currentChanged for (i32) {
  fn currentChanged(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget14currentChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTabWidget14currentChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabIcon<T: QTabWidget_setTabIcon>(&mut self, value: T) -> i32 {
    value.setTabIcon(self);
    return 1;
  }
}

pub trait QTabWidget_setTabIcon {
  fn setTabIcon(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setTabIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QTabWidget_setTabIcon for (i32, &'a  QIcon) {
  fn setTabIcon(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setTabIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QTabWidget10setTabIconEiRK5QIcon(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabIcon<T: QTabWidget_tabIcon>(&mut self, value: T) -> i32 {
    value.tabIcon(self);
    return 1;
  }
}

pub trait QTabWidget_tabIcon {
  fn tabIcon(self, this: &mut QTabWidget) -> i32;
}

// proto: QIcon QTabWidget::tabIcon(int index);
impl<'a> /*trait*/ QTabWidget_tabIcon for (i32) {
  fn tabIcon(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7tabIconEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTabWidget7tabIconEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn isTabEnabled<T: QTabWidget_isTabEnabled>(&mut self, value: T) -> i32 {
    value.isTabEnabled(self);
    return 1;
  }
}

pub trait QTabWidget_isTabEnabled {
  fn isTabEnabled(self, this: &mut QTabWidget) -> i32;
}

// proto: bool QTabWidget::isTabEnabled(int index);
impl<'a> /*trait*/ QTabWidget_isTabEnabled for (i32) {
  fn isTabEnabled(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12isTabEnabledEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK10QTabWidget12isTabEnabledEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabBarAutoHide<T: QTabWidget_setTabBarAutoHide>(&mut self, value: T) -> i32 {
    value.setTabBarAutoHide(self);
    return 1;
  }
}

pub trait QTabWidget_setTabBarAutoHide {
  fn setTabBarAutoHide(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setTabBarAutoHide(bool enabled);
impl<'a> /*trait*/ QTabWidget_setTabBarAutoHide for (i8) {
  fn setTabBarAutoHide(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget17setTabBarAutoHideEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QTabWidget17setTabBarAutoHideEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn iconSize<T: QTabWidget_iconSize>(&mut self, value: T) -> i32 {
    value.iconSize(self);
    return 1;
  }
}

pub trait QTabWidget_iconSize {
  fn iconSize(self, this: &mut QTabWidget) -> i32;
}

// proto: QSize QTabWidget::iconSize();
impl<'a> /*trait*/ QTabWidget_iconSize for () {
  fn iconSize(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget8iconSizeEv()};
    unsafe {_ZNK10QTabWidget8iconSizeEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabsClosable<T: QTabWidget_setTabsClosable>(&mut self, value: T) -> i32 {
    value.setTabsClosable(self);
    return 1;
  }
}

pub trait QTabWidget_setTabsClosable {
  fn setTabsClosable(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setTabsClosable(bool closeable);
impl<'a> /*trait*/ QTabWidget_setTabsClosable for (i8) {
  fn setTabsClosable(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setTabsClosableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN10QTabWidget15setTabsClosableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn minimumSizeHint<T: QTabWidget_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QTabWidget_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QTabWidget) -> i32;
}

// proto: QSize QTabWidget::minimumSizeHint();
impl<'a> /*trait*/ QTabWidget_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget15minimumSizeHintEv()};
    unsafe {_ZNK10QTabWidget15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setCurrentIndex<T: QTabWidget_setCurrentIndex>(&mut self, value: T) -> i32 {
    value.setCurrentIndex(self);
    return 1;
  }
}

pub trait QTabWidget_setCurrentIndex {
  fn setCurrentIndex(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setCurrentIndex(int index);
impl<'a> /*trait*/ QTabWidget_setCurrentIndex for (i32) {
  fn setCurrentIndex(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setCurrentIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTabWidget15setCurrentIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn FreeQTabWidget<T: QTabWidget_FreeQTabWidget>(&mut self, value: T) -> i32 {
    value.FreeQTabWidget(self);
    return 1;
  }
}

pub trait QTabWidget_FreeQTabWidget {
  fn FreeQTabWidget(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::FreeQTabWidget();
impl<'a> /*trait*/ QTabWidget_FreeQTabWidget for () {
  fn FreeQTabWidget(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidgetD0Ev()};
    unsafe {_ZN10QTabWidgetD0Ev()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabWhatsThis<T: QTabWidget_setTabWhatsThis>(&mut self, value: T) -> i32 {
    value.setTabWhatsThis(self);
    return 1;
  }
}

pub trait QTabWidget_setTabWhatsThis {
  fn setTabWhatsThis(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setTabWhatsThis(int index, const QString & text);
impl<'a> /*trait*/ QTabWidget_setTabWhatsThis for (i32, &'a  QString) {
  fn setTabWhatsThis(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setTabWhatsThisEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QTabWidget15setTabWhatsThisEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn sizeHint<T: QTabWidget_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QTabWidget_sizeHint {
  fn sizeHint(self, this: &mut QTabWidget) -> i32;
}

// proto: QSize QTabWidget::sizeHint();
impl<'a> /*trait*/ QTabWidget_sizeHint for () {
  fn sizeHint(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget8sizeHintEv()};
    unsafe {_ZNK10QTabWidget8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn indexOf<T: QTabWidget_indexOf>(&mut self, value: T) -> i32 {
    value.indexOf(self);
    return 1;
  }
}

pub trait QTabWidget_indexOf {
  fn indexOf(self, this: &mut QTabWidget) -> i32;
}

// proto: int QTabWidget::indexOf(QWidget * widget);
impl<'a> /*trait*/ QTabWidget_indexOf for (&'a mut QWidget) {
  fn indexOf(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK10QTabWidget7indexOfEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn removeTab<T: QTabWidget_removeTab>(&mut self, value: T) -> i32 {
    value.removeTab(self);
    return 1;
  }
}

pub trait QTabWidget_removeTab {
  fn removeTab(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::removeTab(int index);
impl<'a> /*trait*/ QTabWidget_removeTab for (i32) {
  fn removeTab(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget9removeTabEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTabWidget9removeTabEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabToolTip<T: QTabWidget_setTabToolTip>(&mut self, value: T) -> i32 {
    value.setTabToolTip(self);
    return 1;
  }
}

pub trait QTabWidget_setTabToolTip {
  fn setTabToolTip(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setTabToolTip(int index, const QString & tip);
impl<'a> /*trait*/ QTabWidget_setTabToolTip for (i32, &'a  QString) {
  fn setTabToolTip(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13setTabToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN10QTabWidget13setTabToolTipEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn isMovable<T: QTabWidget_isMovable>(&mut self, value: T) -> i32 {
    value.isMovable(self);
    return 1;
  }
}

pub trait QTabWidget_isMovable {
  fn isMovable(self, this: &mut QTabWidget) -> i32;
}

// proto: bool QTabWidget::isMovable();
impl<'a> /*trait*/ QTabWidget_isMovable for () {
  fn isMovable(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget9isMovableEv()};
    unsafe {_ZNK10QTabWidget9isMovableEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn usesScrollButtons<T: QTabWidget_usesScrollButtons>(&mut self, value: T) -> i32 {
    value.usesScrollButtons(self);
    return 1;
  }
}

pub trait QTabWidget_usesScrollButtons {
  fn usesScrollButtons(self, this: &mut QTabWidget) -> i32;
}

// proto: bool QTabWidget::usesScrollButtons();
impl<'a> /*trait*/ QTabWidget_usesScrollButtons for () {
  fn usesScrollButtons(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget17usesScrollButtonsEv()};
    unsafe {_ZNK10QTabWidget17usesScrollButtonsEv()};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn tabBarDoubleClicked<T: QTabWidget_tabBarDoubleClicked>(&mut self, value: T) -> i32 {
    value.tabBarDoubleClicked(self);
    return 1;
  }
}

pub trait QTabWidget_tabBarDoubleClicked {
  fn tabBarDoubleClicked(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::tabBarDoubleClicked(int index);
impl<'a> /*trait*/ QTabWidget_tabBarDoubleClicked for (i32) {
  fn tabBarDoubleClicked(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget19tabBarDoubleClickedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN10QTabWidget19tabBarDoubleClickedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn currentIndex<T: QTabWidget_currentIndex>(&mut self, value: T) -> i32 {
    value.currentIndex(self);
    return 1;
  }
}

pub trait QTabWidget_currentIndex {
  fn currentIndex(self, this: &mut QTabWidget) -> i32;
}

// proto: int QTabWidget::currentIndex();
impl<'a> /*trait*/ QTabWidget_currentIndex for () {
  fn currentIndex(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12currentIndexEv()};
    unsafe {_ZNK10QTabWidget12currentIndexEv()};
    return 1;
  }
}

// proto: int QTabWidget::insertTab(int index, QWidget * widget, const QString & );
impl<'a> /*trait*/ QTabWidget_insertTab for (i32, &'a mut QWidget, &'a  QString) {
  fn insertTab(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget9insertTabEiP7QWidgetRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN10QTabWidget9insertTabEiP7QWidgetRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTabWidget {
  pub fn setTabEnabled<T: QTabWidget_setTabEnabled>(&mut self, value: T) -> i32 {
    value.setTabEnabled(self);
    return 1;
  }
}

pub trait QTabWidget_setTabEnabled {
  fn setTabEnabled(self, this: &mut QTabWidget) -> i32;
}

// proto: void QTabWidget::setTabEnabled(int index, bool );
impl<'a> /*trait*/ QTabWidget_setTabEnabled for (i32, i8) {
  fn setTabEnabled(self, this: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13setTabEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN10QTabWidget13setTabEnabledEib(arg0, arg1)};
    return 1;
  }
}

