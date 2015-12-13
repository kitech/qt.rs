// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qicon::QIcon;
use super::qpoint::QPoint;
use super::qvariant::QVariant;
use super::qwidget::QWidget;
use super::qcolor::QColor;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: bool QTabBar::usesScrollButtons();
  fn _ZNK7QTabBar17usesScrollButtonsEv() -> i32;
  // proto: bool QTabBar::autoHide();
  fn _ZNK7QTabBar8autoHideEv() -> i32;
  // proto: QString QTabBar::tabToolTip(int index);
  fn _ZNK7QTabBar10tabToolTipEi(arg0: c_int) -> i32;
  // proto: bool QTabBar::expanding();
  fn _ZNK7QTabBar9expandingEv() -> i32;
  // proto: void QTabBar::setDocumentMode(bool set);
  fn _ZN7QTabBar15setDocumentModeEb(arg0: int8_t) -> i32;
  // proto: int QTabBar::count();
  fn _ZNK7QTabBar5countEv() -> i32;
  // proto: void QTabBar::setChangeCurrentOnDrag(bool change);
  fn _ZN7QTabBar22setChangeCurrentOnDragEb(arg0: int8_t) -> i32;
  // proto: QIcon QTabBar::tabIcon(int index);
  fn _ZNK7QTabBar7tabIconEi(arg0: c_int) -> i32;
  // proto: void QTabBar::tabBarClicked(int index);
  fn _ZN7QTabBar13tabBarClickedEi(arg0: c_int) -> i32;
  // proto: QSize QTabBar::minimumSizeHint();
  fn _ZNK7QTabBar15minimumSizeHintEv() -> i32;
  // proto: void QTabBar::setTabsClosable(bool closable);
  fn _ZN7QTabBar15setTabsClosableEb(arg0: int8_t) -> i32;
  // proto: bool QTabBar::changeCurrentOnDrag();
  fn _ZNK7QTabBar19changeCurrentOnDragEv() -> i32;
  // proto: void QTabBar::setTabWhatsThis(int index, const QString & text);
  fn _ZN7QTabBar15setTabWhatsThisEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: const QMetaObject * QTabBar::metaObject();
  fn _ZNK7QTabBar10metaObjectEv() -> i32;
  // proto: void QTabBar::NewQTabBar(const QTabBar & );
  fn _ZN7QTabBarC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: int QTabBar::insertTab(int index, const QIcon & icon, const QString & text);
  fn _ZN7QTabBar9insertTabEiRK5QIconRK7QString(arg0: c_int, arg1: *const c_void, arg2: *const c_void) -> i32;
  // proto: void QTabBar::setTabIcon(int index, const QIcon & icon);
  fn _ZN7QTabBar10setTabIconEiRK5QIcon(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: bool QTabBar::isMovable();
  fn _ZNK7QTabBar9isMovableEv() -> i32;
  // proto: void QTabBar::setExpanding(bool enabled);
  fn _ZN7QTabBar12setExpandingEb(arg0: int8_t) -> i32;
  // proto: void QTabBar::removeTab(int index);
  fn _ZN7QTabBar9removeTabEi(arg0: c_int) -> i32;
  // proto: void QTabBar::setTabEnabled(int index, bool );
  fn _ZN7QTabBar13setTabEnabledEib(arg0: c_int, arg1: int8_t) -> i32;
  // proto: bool QTabBar::isTabEnabled(int index);
  fn _ZNK7QTabBar12isTabEnabledEi(arg0: c_int) -> i32;
  // proto: void QTabBar::setCurrentIndex(int index);
  fn _ZN7QTabBar15setCurrentIndexEi(arg0: c_int) -> i32;
  // proto: QRect QTabBar::tabRect(int index);
  fn _ZNK7QTabBar7tabRectEi(arg0: c_int) -> i32;
  // proto: bool QTabBar::tabsClosable();
  fn _ZNK7QTabBar12tabsClosableEv() -> i32;
  // proto: void QTabBar::tabCloseRequested(int index);
  fn _ZN7QTabBar17tabCloseRequestedEi(arg0: c_int) -> i32;
  // proto: void QTabBar::setMovable(bool movable);
  fn _ZN7QTabBar10setMovableEb(arg0: int8_t) -> i32;
  // proto: void QTabBar::setAutoHide(bool hide);
  fn _ZN7QTabBar11setAutoHideEb(arg0: int8_t) -> i32;
  // proto: QSize QTabBar::iconSize();
  fn _ZNK7QTabBar8iconSizeEv() -> i32;
  // proto: QString QTabBar::tabText(int index);
  fn _ZNK7QTabBar7tabTextEi(arg0: c_int) -> i32;
  // proto: QString QTabBar::tabWhatsThis(int index);
  fn _ZNK7QTabBar12tabWhatsThisEi(arg0: c_int) -> i32;
  // proto: bool QTabBar::documentMode();
  fn _ZNK7QTabBar12documentModeEv() -> i32;
  // proto: int QTabBar::tabAt(const QPoint & pos);
  fn _ZNK7QTabBar5tabAtERK6QPoint(arg0: *const c_void) -> i32;
  // proto: void QTabBar::setTabData(int index, const QVariant & data);
  fn _ZN7QTabBar10setTabDataEiRK8QVariant(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTabBar::NewQTabBar(QWidget * parent);
  fn _ZN7QTabBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: QColor QTabBar::tabTextColor(int index);
  fn _ZNK7QTabBar12tabTextColorEi(arg0: c_int) -> i32;
  // proto: void QTabBar::FreeQTabBar();
  fn _ZN7QTabBarD0Ev() -> i32;
  // proto: int QTabBar::insertTab(int index, const QString & text);
  fn _ZN7QTabBar9insertTabEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTabBar::tabBarDoubleClicked(int index);
  fn _ZN7QTabBar19tabBarDoubleClickedEi(arg0: c_int) -> i32;
  // proto: int QTabBar::addTab(const QString & text);
  fn _ZN7QTabBar6addTabERK7QString(arg0: *const c_void) -> i32;
  // proto: int QTabBar::addTab(const QIcon & icon, const QString & text);
  fn _ZN7QTabBar6addTabERK5QIconRK7QString(arg0: *const c_void, arg1: *const c_void) -> i32;
  // proto: void QTabBar::setTabToolTip(int index, const QString & tip);
  fn _ZN7QTabBar13setTabToolTipEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTabBar::currentChanged(int index);
  fn _ZN7QTabBar14currentChangedEi(arg0: c_int) -> i32;
  // proto: void QTabBar::setTabTextColor(int index, const QColor & color);
  fn _ZN7QTabBar15setTabTextColorEiRK6QColor(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTabBar::moveTab(int from, int to);
  fn _ZN7QTabBar7moveTabEii(arg0: c_int, arg1: c_int) -> i32;
  // proto: QVariant QTabBar::tabData(int index);
  fn _ZNK7QTabBar7tabDataEi(arg0: c_int) -> i32;
  // proto: bool QTabBar::drawBase();
  fn _ZNK7QTabBar8drawBaseEv() -> i32;
  // proto: int QTabBar::currentIndex();
  fn _ZNK7QTabBar12currentIndexEv() -> i32;
  // proto: void QTabBar::setDrawBase(bool drawTheBase);
  fn _ZN7QTabBar11setDrawBaseEb(arg0: int8_t) -> i32;
  // proto: void QTabBar::setUsesScrollButtons(bool useButtons);
  fn _ZN7QTabBar20setUsesScrollButtonsEb(arg0: int8_t) -> i32;
  // proto: QSize QTabBar::sizeHint();
  fn _ZNK7QTabBar8sizeHintEv() -> i32;
  // proto: void QTabBar::setIconSize(const QSize & size);
  fn _ZN7QTabBar11setIconSizeERK5QSize(arg0: *const c_void) -> i32;
  // proto: void QTabBar::setTabText(int index, const QString & text);
  fn _ZN7QTabBar10setTabTextEiRK7QString(arg0: c_int, arg1: *const c_void) -> i32;
  // proto: void QTabBar::tabMoved(int from, int to);
  fn _ZN7QTabBar8tabMovedEii(arg0: c_int, arg1: c_int) -> i32;
}

// body block begin
// class sizeof(QTabBar)=1
pub struct QTabBar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTabBar {
  pub fn usesScrollButtons<T: QTabBar_usesScrollButtons>(&mut self, value: T) -> i32 {
    value.usesScrollButtons(self);
    return 1;
  }
}

pub trait QTabBar_usesScrollButtons {
  fn usesScrollButtons(self, this: &mut QTabBar) -> i32;
}

// proto: bool QTabBar::usesScrollButtons();
impl<'a> /*trait*/ QTabBar_usesScrollButtons for () {
  fn usesScrollButtons(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar17usesScrollButtonsEv()};
    unsafe {_ZNK7QTabBar17usesScrollButtonsEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn autoHide<T: QTabBar_autoHide>(&mut self, value: T) -> i32 {
    value.autoHide(self);
    return 1;
  }
}

pub trait QTabBar_autoHide {
  fn autoHide(self, this: &mut QTabBar) -> i32;
}

// proto: bool QTabBar::autoHide();
impl<'a> /*trait*/ QTabBar_autoHide for () {
  fn autoHide(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8autoHideEv()};
    unsafe {_ZNK7QTabBar8autoHideEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabToolTip<T: QTabBar_tabToolTip>(&mut self, value: T) -> i32 {
    value.tabToolTip(self);
    return 1;
  }
}

pub trait QTabBar_tabToolTip {
  fn tabToolTip(self, this: &mut QTabBar) -> i32;
}

// proto: QString QTabBar::tabToolTip(int index);
impl<'a> /*trait*/ QTabBar_tabToolTip for (i32) {
  fn tabToolTip(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar10tabToolTipEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QTabBar10tabToolTipEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn expanding<T: QTabBar_expanding>(&mut self, value: T) -> i32 {
    value.expanding(self);
    return 1;
  }
}

pub trait QTabBar_expanding {
  fn expanding(self, this: &mut QTabBar) -> i32;
}

// proto: bool QTabBar::expanding();
impl<'a> /*trait*/ QTabBar_expanding for () {
  fn expanding(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar9expandingEv()};
    unsafe {_ZNK7QTabBar9expandingEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setDocumentMode<T: QTabBar_setDocumentMode>(&mut self, value: T) -> i32 {
    value.setDocumentMode(self);
    return 1;
  }
}

pub trait QTabBar_setDocumentMode {
  fn setDocumentMode(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setDocumentMode(bool set);
impl<'a> /*trait*/ QTabBar_setDocumentMode for (i8) {
  fn setDocumentMode(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setDocumentModeEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QTabBar15setDocumentModeEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn count<T: QTabBar_count>(&mut self, value: T) -> i32 {
    value.count(self);
    return 1;
  }
}

pub trait QTabBar_count {
  fn count(self, this: &mut QTabBar) -> i32;
}

// proto: int QTabBar::count();
impl<'a> /*trait*/ QTabBar_count for () {
  fn count(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar5countEv()};
    unsafe {_ZNK7QTabBar5countEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setChangeCurrentOnDrag<T: QTabBar_setChangeCurrentOnDrag>(&mut self, value: T) -> i32 {
    value.setChangeCurrentOnDrag(self);
    return 1;
  }
}

pub trait QTabBar_setChangeCurrentOnDrag {
  fn setChangeCurrentOnDrag(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setChangeCurrentOnDrag(bool change);
impl<'a> /*trait*/ QTabBar_setChangeCurrentOnDrag for (i8) {
  fn setChangeCurrentOnDrag(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar22setChangeCurrentOnDragEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QTabBar22setChangeCurrentOnDragEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabIcon<T: QTabBar_tabIcon>(&mut self, value: T) -> i32 {
    value.tabIcon(self);
    return 1;
  }
}

pub trait QTabBar_tabIcon {
  fn tabIcon(self, this: &mut QTabBar) -> i32;
}

// proto: QIcon QTabBar::tabIcon(int index);
impl<'a> /*trait*/ QTabBar_tabIcon for (i32) {
  fn tabIcon(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabIconEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QTabBar7tabIconEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabBarClicked<T: QTabBar_tabBarClicked>(&mut self, value: T) -> i32 {
    value.tabBarClicked(self);
    return 1;
  }
}

pub trait QTabBar_tabBarClicked {
  fn tabBarClicked(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::tabBarClicked(int index);
impl<'a> /*trait*/ QTabBar_tabBarClicked for (i32) {
  fn tabBarClicked(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar13tabBarClickedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QTabBar13tabBarClickedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn minimumSizeHint<T: QTabBar_minimumSizeHint>(&mut self, value: T) -> i32 {
    value.minimumSizeHint(self);
    return 1;
  }
}

pub trait QTabBar_minimumSizeHint {
  fn minimumSizeHint(self, this: &mut QTabBar) -> i32;
}

// proto: QSize QTabBar::minimumSizeHint();
impl<'a> /*trait*/ QTabBar_minimumSizeHint for () {
  fn minimumSizeHint(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar15minimumSizeHintEv()};
    unsafe {_ZNK7QTabBar15minimumSizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabsClosable<T: QTabBar_setTabsClosable>(&mut self, value: T) -> i32 {
    value.setTabsClosable(self);
    return 1;
  }
}

pub trait QTabBar_setTabsClosable {
  fn setTabsClosable(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setTabsClosable(bool closable);
impl<'a> /*trait*/ QTabBar_setTabsClosable for (i8) {
  fn setTabsClosable(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setTabsClosableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QTabBar15setTabsClosableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn changeCurrentOnDrag<T: QTabBar_changeCurrentOnDrag>(&mut self, value: T) -> i32 {
    value.changeCurrentOnDrag(self);
    return 1;
  }
}

pub trait QTabBar_changeCurrentOnDrag {
  fn changeCurrentOnDrag(self, this: &mut QTabBar) -> i32;
}

// proto: bool QTabBar::changeCurrentOnDrag();
impl<'a> /*trait*/ QTabBar_changeCurrentOnDrag for () {
  fn changeCurrentOnDrag(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar19changeCurrentOnDragEv()};
    unsafe {_ZNK7QTabBar19changeCurrentOnDragEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabWhatsThis<T: QTabBar_setTabWhatsThis>(&mut self, value: T) -> i32 {
    value.setTabWhatsThis(self);
    return 1;
  }
}

pub trait QTabBar_setTabWhatsThis {
  fn setTabWhatsThis(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setTabWhatsThis(int index, const QString & text);
impl<'a> /*trait*/ QTabBar_setTabWhatsThis for (i32, &'a  QString) {
  fn setTabWhatsThis(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setTabWhatsThisEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar15setTabWhatsThisEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn metaObject<T: QTabBar_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QTabBar_metaObject {
  fn metaObject(self, this: &mut QTabBar) -> i32;
}

// proto: const QMetaObject * QTabBar::metaObject();
impl<'a> /*trait*/ QTabBar_metaObject for () {
  fn metaObject(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar10metaObjectEv()};
    unsafe {_ZNK7QTabBar10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn NewQTabBar<T: QTabBar_NewQTabBar>(value: T) -> QTabBar {
    let rsthis = value.NewQTabBar();
    return rsthis;
    // return 1;
  }
}

pub trait QTabBar_NewQTabBar {
  fn NewQTabBar(self) -> QTabBar;
}

// proto: void QTabBar::NewQTabBar(const QTabBar & );
impl<'a> /*trait*/ QTabBar_NewQTabBar for (&'a  QTabBar) {
  fn NewQTabBar(self) -> QTabBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBarC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBarC1ERKS_(qthis, arg0)};
    let rsthis = QTabBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn insertTab<T: QTabBar_insertTab>(&mut self, value: T) -> i32 {
    value.insertTab(self);
    return 1;
  }
}

pub trait QTabBar_insertTab {
  fn insertTab(self, this: &mut QTabBar) -> i32;
}

// proto: int QTabBar::insertTab(int index, const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QTabBar_insertTab for (i32, &'a  QIcon, &'a  QString) {
  fn insertTab(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar9insertTabEiRK5QIconRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar9insertTabEiRK5QIconRK7QString(arg0, arg1, arg2)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabIcon<T: QTabBar_setTabIcon>(&mut self, value: T) -> i32 {
    value.setTabIcon(self);
    return 1;
  }
}

pub trait QTabBar_setTabIcon {
  fn setTabIcon(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setTabIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QTabBar_setTabIcon for (i32, &'a  QIcon) {
  fn setTabIcon(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setTabIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar10setTabIconEiRK5QIcon(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn isMovable<T: QTabBar_isMovable>(&mut self, value: T) -> i32 {
    value.isMovable(self);
    return 1;
  }
}

pub trait QTabBar_isMovable {
  fn isMovable(self, this: &mut QTabBar) -> i32;
}

// proto: bool QTabBar::isMovable();
impl<'a> /*trait*/ QTabBar_isMovable for () {
  fn isMovable(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar9isMovableEv()};
    unsafe {_ZNK7QTabBar9isMovableEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setExpanding<T: QTabBar_setExpanding>(&mut self, value: T) -> i32 {
    value.setExpanding(self);
    return 1;
  }
}

pub trait QTabBar_setExpanding {
  fn setExpanding(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setExpanding(bool enabled);
impl<'a> /*trait*/ QTabBar_setExpanding for (i8) {
  fn setExpanding(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar12setExpandingEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QTabBar12setExpandingEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn removeTab<T: QTabBar_removeTab>(&mut self, value: T) -> i32 {
    value.removeTab(self);
    return 1;
  }
}

pub trait QTabBar_removeTab {
  fn removeTab(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::removeTab(int index);
impl<'a> /*trait*/ QTabBar_removeTab for (i32) {
  fn removeTab(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar9removeTabEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QTabBar9removeTabEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabEnabled<T: QTabBar_setTabEnabled>(&mut self, value: T) -> i32 {
    value.setTabEnabled(self);
    return 1;
  }
}

pub trait QTabBar_setTabEnabled {
  fn setTabEnabled(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setTabEnabled(int index, bool );
impl<'a> /*trait*/ QTabBar_setTabEnabled for (i32, i8) {
  fn setTabEnabled(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar13setTabEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
    unsafe {_ZN7QTabBar13setTabEnabledEib(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn isTabEnabled<T: QTabBar_isTabEnabled>(&mut self, value: T) -> i32 {
    value.isTabEnabled(self);
    return 1;
  }
}

pub trait QTabBar_isTabEnabled {
  fn isTabEnabled(self, this: &mut QTabBar) -> i32;
}

// proto: bool QTabBar::isTabEnabled(int index);
impl<'a> /*trait*/ QTabBar_isTabEnabled for (i32) {
  fn isTabEnabled(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12isTabEnabledEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QTabBar12isTabEnabledEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setCurrentIndex<T: QTabBar_setCurrentIndex>(&mut self, value: T) -> i32 {
    value.setCurrentIndex(self);
    return 1;
  }
}

pub trait QTabBar_setCurrentIndex {
  fn setCurrentIndex(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setCurrentIndex(int index);
impl<'a> /*trait*/ QTabBar_setCurrentIndex for (i32) {
  fn setCurrentIndex(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setCurrentIndexEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QTabBar15setCurrentIndexEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabRect<T: QTabBar_tabRect>(&mut self, value: T) -> i32 {
    value.tabRect(self);
    return 1;
  }
}

pub trait QTabBar_tabRect {
  fn tabRect(self, this: &mut QTabBar) -> i32;
}

// proto: QRect QTabBar::tabRect(int index);
impl<'a> /*trait*/ QTabBar_tabRect for (i32) {
  fn tabRect(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabRectEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QTabBar7tabRectEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabsClosable<T: QTabBar_tabsClosable>(&mut self, value: T) -> i32 {
    value.tabsClosable(self);
    return 1;
  }
}

pub trait QTabBar_tabsClosable {
  fn tabsClosable(self, this: &mut QTabBar) -> i32;
}

// proto: bool QTabBar::tabsClosable();
impl<'a> /*trait*/ QTabBar_tabsClosable for () {
  fn tabsClosable(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12tabsClosableEv()};
    unsafe {_ZNK7QTabBar12tabsClosableEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabCloseRequested<T: QTabBar_tabCloseRequested>(&mut self, value: T) -> i32 {
    value.tabCloseRequested(self);
    return 1;
  }
}

pub trait QTabBar_tabCloseRequested {
  fn tabCloseRequested(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::tabCloseRequested(int index);
impl<'a> /*trait*/ QTabBar_tabCloseRequested for (i32) {
  fn tabCloseRequested(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar17tabCloseRequestedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QTabBar17tabCloseRequestedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setMovable<T: QTabBar_setMovable>(&mut self, value: T) -> i32 {
    value.setMovable(self);
    return 1;
  }
}

pub trait QTabBar_setMovable {
  fn setMovable(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setMovable(bool movable);
impl<'a> /*trait*/ QTabBar_setMovable for (i8) {
  fn setMovable(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setMovableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QTabBar10setMovableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setAutoHide<T: QTabBar_setAutoHide>(&mut self, value: T) -> i32 {
    value.setAutoHide(self);
    return 1;
  }
}

pub trait QTabBar_setAutoHide {
  fn setAutoHide(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setAutoHide(bool hide);
impl<'a> /*trait*/ QTabBar_setAutoHide for (i8) {
  fn setAutoHide(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar11setAutoHideEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QTabBar11setAutoHideEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn iconSize<T: QTabBar_iconSize>(&mut self, value: T) -> i32 {
    value.iconSize(self);
    return 1;
  }
}

pub trait QTabBar_iconSize {
  fn iconSize(self, this: &mut QTabBar) -> i32;
}

// proto: QSize QTabBar::iconSize();
impl<'a> /*trait*/ QTabBar_iconSize for () {
  fn iconSize(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8iconSizeEv()};
    unsafe {_ZNK7QTabBar8iconSizeEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabText<T: QTabBar_tabText>(&mut self, value: T) -> i32 {
    value.tabText(self);
    return 1;
  }
}

pub trait QTabBar_tabText {
  fn tabText(self, this: &mut QTabBar) -> i32;
}

// proto: QString QTabBar::tabText(int index);
impl<'a> /*trait*/ QTabBar_tabText for (i32) {
  fn tabText(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabTextEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QTabBar7tabTextEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabWhatsThis<T: QTabBar_tabWhatsThis>(&mut self, value: T) -> i32 {
    value.tabWhatsThis(self);
    return 1;
  }
}

pub trait QTabBar_tabWhatsThis {
  fn tabWhatsThis(self, this: &mut QTabBar) -> i32;
}

// proto: QString QTabBar::tabWhatsThis(int index);
impl<'a> /*trait*/ QTabBar_tabWhatsThis for (i32) {
  fn tabWhatsThis(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12tabWhatsThisEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QTabBar12tabWhatsThisEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn documentMode<T: QTabBar_documentMode>(&mut self, value: T) -> i32 {
    value.documentMode(self);
    return 1;
  }
}

pub trait QTabBar_documentMode {
  fn documentMode(self, this: &mut QTabBar) -> i32;
}

// proto: bool QTabBar::documentMode();
impl<'a> /*trait*/ QTabBar_documentMode for () {
  fn documentMode(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12documentModeEv()};
    unsafe {_ZNK7QTabBar12documentModeEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabAt<T: QTabBar_tabAt>(&mut self, value: T) -> i32 {
    value.tabAt(self);
    return 1;
  }
}

pub trait QTabBar_tabAt {
  fn tabAt(self, this: &mut QTabBar) -> i32;
}

// proto: int QTabBar::tabAt(const QPoint & pos);
impl<'a> /*trait*/ QTabBar_tabAt for (&'a  QPoint) {
  fn tabAt(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar5tabAtERK6QPoint()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZNK7QTabBar5tabAtERK6QPoint(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabData<T: QTabBar_setTabData>(&mut self, value: T) -> i32 {
    value.setTabData(self);
    return 1;
  }
}

pub trait QTabBar_setTabData {
  fn setTabData(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setTabData(int index, const QVariant & data);
impl<'a> /*trait*/ QTabBar_setTabData for (i32, &'a  QVariant) {
  fn setTabData(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setTabDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar10setTabDataEiRK8QVariant(arg0, arg1)};
    return 1;
  }
}

// proto: void QTabBar::NewQTabBar(QWidget * parent);
impl<'a> /*trait*/ QTabBar_NewQTabBar for (&'a mut QWidget) {
  fn NewQTabBar(self) -> QTabBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBarC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QTabBarC1EP7QWidget(qthis, arg0)};
    let rsthis = QTabBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabTextColor<T: QTabBar_tabTextColor>(&mut self, value: T) -> i32 {
    value.tabTextColor(self);
    return 1;
  }
}

pub trait QTabBar_tabTextColor {
  fn tabTextColor(self, this: &mut QTabBar) -> i32;
}

// proto: QColor QTabBar::tabTextColor(int index);
impl<'a> /*trait*/ QTabBar_tabTextColor for (i32) {
  fn tabTextColor(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12tabTextColorEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QTabBar12tabTextColorEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn FreeQTabBar<T: QTabBar_FreeQTabBar>(&mut self, value: T) -> i32 {
    value.FreeQTabBar(self);
    return 1;
  }
}

pub trait QTabBar_FreeQTabBar {
  fn FreeQTabBar(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::FreeQTabBar();
impl<'a> /*trait*/ QTabBar_FreeQTabBar for () {
  fn FreeQTabBar(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBarD0Ev()};
    unsafe {_ZN7QTabBarD0Ev()};
    return 1;
  }
}

// proto: int QTabBar::insertTab(int index, const QString & text);
impl<'a> /*trait*/ QTabBar_insertTab for (i32, &'a  QString) {
  fn insertTab(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar9insertTabEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar9insertTabEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabBarDoubleClicked<T: QTabBar_tabBarDoubleClicked>(&mut self, value: T) -> i32 {
    value.tabBarDoubleClicked(self);
    return 1;
  }
}

pub trait QTabBar_tabBarDoubleClicked {
  fn tabBarDoubleClicked(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::tabBarDoubleClicked(int index);
impl<'a> /*trait*/ QTabBar_tabBarDoubleClicked for (i32) {
  fn tabBarDoubleClicked(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar19tabBarDoubleClickedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QTabBar19tabBarDoubleClickedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn addTab<T: QTabBar_addTab>(&mut self, value: T) -> i32 {
    value.addTab(self);
    return 1;
  }
}

pub trait QTabBar_addTab {
  fn addTab(self, this: &mut QTabBar) -> i32;
}

// proto: int QTabBar::addTab(const QString & text);
impl<'a> /*trait*/ QTabBar_addTab for (&'a  QString) {
  fn addTab(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar6addTabERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar6addTabERK7QString(arg0)};
    return 1;
  }
}

// proto: int QTabBar::addTab(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QTabBar_addTab for (&'a  QIcon, &'a  QString) {
  fn addTab(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar6addTabERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar6addTabERK5QIconRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabToolTip<T: QTabBar_setTabToolTip>(&mut self, value: T) -> i32 {
    value.setTabToolTip(self);
    return 1;
  }
}

pub trait QTabBar_setTabToolTip {
  fn setTabToolTip(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setTabToolTip(int index, const QString & tip);
impl<'a> /*trait*/ QTabBar_setTabToolTip for (i32, &'a  QString) {
  fn setTabToolTip(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar13setTabToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar13setTabToolTipEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn currentChanged<T: QTabBar_currentChanged>(&mut self, value: T) -> i32 {
    value.currentChanged(self);
    return 1;
  }
}

pub trait QTabBar_currentChanged {
  fn currentChanged(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::currentChanged(int index);
impl<'a> /*trait*/ QTabBar_currentChanged for (i32) {
  fn currentChanged(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar14currentChangedEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN7QTabBar14currentChangedEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabTextColor<T: QTabBar_setTabTextColor>(&mut self, value: T) -> i32 {
    value.setTabTextColor(self);
    return 1;
  }
}

pub trait QTabBar_setTabTextColor {
  fn setTabTextColor(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setTabTextColor(int index, const QColor & color);
impl<'a> /*trait*/ QTabBar_setTabTextColor for (i32, &'a  QColor) {
  fn setTabTextColor(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setTabTextColorEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar15setTabTextColorEiRK6QColor(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn moveTab<T: QTabBar_moveTab>(&mut self, value: T) -> i32 {
    value.moveTab(self);
    return 1;
  }
}

pub trait QTabBar_moveTab {
  fn moveTab(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::moveTab(int from, int to);
impl<'a> /*trait*/ QTabBar_moveTab for (i32, i32) {
  fn moveTab(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar7moveTabEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QTabBar7moveTabEii(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabData<T: QTabBar_tabData>(&mut self, value: T) -> i32 {
    value.tabData(self);
    return 1;
  }
}

pub trait QTabBar_tabData {
  fn tabData(self, this: &mut QTabBar) -> i32;
}

// proto: QVariant QTabBar::tabData(int index);
impl<'a> /*trait*/ QTabBar_tabData for (i32) {
  fn tabData(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabDataEi()};
    let arg0 = self  as c_int;
    unsafe {_ZNK7QTabBar7tabDataEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn drawBase<T: QTabBar_drawBase>(&mut self, value: T) -> i32 {
    value.drawBase(self);
    return 1;
  }
}

pub trait QTabBar_drawBase {
  fn drawBase(self, this: &mut QTabBar) -> i32;
}

// proto: bool QTabBar::drawBase();
impl<'a> /*trait*/ QTabBar_drawBase for () {
  fn drawBase(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8drawBaseEv()};
    unsafe {_ZNK7QTabBar8drawBaseEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn currentIndex<T: QTabBar_currentIndex>(&mut self, value: T) -> i32 {
    value.currentIndex(self);
    return 1;
  }
}

pub trait QTabBar_currentIndex {
  fn currentIndex(self, this: &mut QTabBar) -> i32;
}

// proto: int QTabBar::currentIndex();
impl<'a> /*trait*/ QTabBar_currentIndex for () {
  fn currentIndex(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12currentIndexEv()};
    unsafe {_ZNK7QTabBar12currentIndexEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setDrawBase<T: QTabBar_setDrawBase>(&mut self, value: T) -> i32 {
    value.setDrawBase(self);
    return 1;
  }
}

pub trait QTabBar_setDrawBase {
  fn setDrawBase(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setDrawBase(bool drawTheBase);
impl<'a> /*trait*/ QTabBar_setDrawBase for (i8) {
  fn setDrawBase(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar11setDrawBaseEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QTabBar11setDrawBaseEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setUsesScrollButtons<T: QTabBar_setUsesScrollButtons>(&mut self, value: T) -> i32 {
    value.setUsesScrollButtons(self);
    return 1;
  }
}

pub trait QTabBar_setUsesScrollButtons {
  fn setUsesScrollButtons(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setUsesScrollButtons(bool useButtons);
impl<'a> /*trait*/ QTabBar_setUsesScrollButtons for (i8) {
  fn setUsesScrollButtons(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar20setUsesScrollButtonsEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QTabBar20setUsesScrollButtonsEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn sizeHint<T: QTabBar_sizeHint>(&mut self, value: T) -> i32 {
    value.sizeHint(self);
    return 1;
  }
}

pub trait QTabBar_sizeHint {
  fn sizeHint(self, this: &mut QTabBar) -> i32;
}

// proto: QSize QTabBar::sizeHint();
impl<'a> /*trait*/ QTabBar_sizeHint for () {
  fn sizeHint(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8sizeHintEv()};
    unsafe {_ZNK7QTabBar8sizeHintEv()};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setIconSize<T: QTabBar_setIconSize>(&mut self, value: T) -> i32 {
    value.setIconSize(self);
    return 1;
  }
}

pub trait QTabBar_setIconSize {
  fn setIconSize(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setIconSize(const QSize & size);
impl<'a> /*trait*/ QTabBar_setIconSize for (&'a  QSize) {
  fn setIconSize(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar11setIconSizeERK5QSize(arg0)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabText<T: QTabBar_setTabText>(&mut self, value: T) -> i32 {
    value.setTabText(self);
    return 1;
  }
}

pub trait QTabBar_setTabText {
  fn setTabText(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::setTabText(int index, const QString & text);
impl<'a> /*trait*/ QTabBar_setTabText for (i32, &'a  QString) {
  fn setTabText(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setTabTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *const c_void;
    unsafe {_ZN7QTabBar10setTabTextEiRK7QString(arg0, arg1)};
    return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabMoved<T: QTabBar_tabMoved>(&mut self, value: T) -> i32 {
    value.tabMoved(self);
    return 1;
  }
}

pub trait QTabBar_tabMoved {
  fn tabMoved(self, this: &mut QTabBar) -> i32;
}

// proto: void QTabBar::tabMoved(int from, int to);
impl<'a> /*trait*/ QTabBar_tabMoved for (i32, i32) {
  fn tabMoved(self, this: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar8tabMovedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QTabBar8tabMovedEii(arg0, arg1)};
    return 1;
  }
}

