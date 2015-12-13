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
use super::qsize::QSize;
use super::qrect::QRect;
use super::qpoint::QPoint;
use super::qvariant::QVariant;
use super::qwidget::QWidget;
use super::qcolor::QColor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QTabBar::usesScrollButtons();
  fn _ZNK7QTabBar17usesScrollButtonsEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QTabBar::autoHide();
  fn _ZNK7QTabBar8autoHideEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QTabBar::tabToolTip(int index);
  fn _ZNK7QTabBar10tabToolTipEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabBar::expanding();
  fn _ZNK7QTabBar9expandingEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTabBar::setDocumentMode(bool set);
  fn _ZN7QTabBar15setDocumentModeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QTabBar::count();
  fn _ZNK7QTabBar5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTabBar::setChangeCurrentOnDrag(bool change);
  fn _ZN7QTabBar22setChangeCurrentOnDragEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QIcon QTabBar::tabIcon(int index);
  fn _ZNK7QTabBar7tabIconEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTabBar::tabBarClicked(int index);
  fn _ZN7QTabBar13tabBarClickedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QSize QTabBar::minimumSizeHint();
  fn _ZNK7QTabBar15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabBar::setTabsClosable(bool closable);
  fn _ZN7QTabBar15setTabsClosableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QTabBar::changeCurrentOnDrag();
  fn _ZNK7QTabBar19changeCurrentOnDragEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTabBar::setTabWhatsThis(int index, const QString & text);
  fn _ZN7QTabBar15setTabWhatsThisEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  const QMetaObject * QTabBar::metaObject();
  fn _ZNK7QTabBar10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QTabBar::NewQTabBar(const QTabBar & );
  fn _ZN7QTabBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QTabBar::insertTab(int index, const QIcon & icon, const QString & text);
  fn _ZN7QTabBar9insertTabEiRK5QIconRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
  // proto:  void QTabBar::setTabIcon(int index, const QIcon & icon);
  fn _ZN7QTabBar10setTabIconEiRK5QIcon(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  bool QTabBar::isMovable();
  fn _ZNK7QTabBar9isMovableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTabBar::setExpanding(bool enabled);
  fn _ZN7QTabBar12setExpandingEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTabBar::removeTab(int index);
  fn _ZN7QTabBar9removeTabEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTabBar::setTabEnabled(int index, bool );
  fn _ZN7QTabBar13setTabEnabledEib(qthis: *mut c_void, arg0: c_int, arg1: int8_t) ;
  // proto:  bool QTabBar::isTabEnabled(int index);
  fn _ZNK7QTabBar12isTabEnabledEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QTabBar::setCurrentIndex(int index);
  fn _ZN7QTabBar15setCurrentIndexEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  QRect QTabBar::tabRect(int index);
  fn _ZNK7QTabBar7tabRectEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabBar::tabsClosable();
  fn _ZNK7QTabBar12tabsClosableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QTabBar::tabCloseRequested(int index);
  fn _ZN7QTabBar17tabCloseRequestedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTabBar::setMovable(bool movable);
  fn _ZN7QTabBar10setMovableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTabBar::setAutoHide(bool hide);
  fn _ZN7QTabBar11setAutoHideEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QSize QTabBar::iconSize();
  fn _ZNK7QTabBar8iconSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTabBar::tabText(int index);
  fn _ZNK7QTabBar7tabTextEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QString QTabBar::tabWhatsThis(int index);
  fn _ZNK7QTabBar12tabWhatsThisEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabBar::documentMode();
  fn _ZNK7QTabBar12documentModeEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTabBar::tabAt(const QPoint & pos);
  fn _ZNK7QTabBar5tabAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QTabBar::setTabData(int index, const QVariant & data);
  fn _ZN7QTabBar10setTabDataEiRK8QVariant(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTabBar::NewQTabBar(QWidget * parent);
  fn _ZN7QTabBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QColor QTabBar::tabTextColor(int index);
  fn _ZNK7QTabBar12tabTextColorEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTabBar::FreeQTabBar();
  fn _ZN7QTabBarD0Ev(qthis: *mut c_void) ;
  // proto:  int QTabBar::insertTab(int index, const QString & text);
  fn _ZN7QTabBar9insertTabEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> c_int;
  // proto:  void QTabBar::tabBarDoubleClicked(int index);
  fn _ZN7QTabBar19tabBarDoubleClickedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QTabBar::addTab(const QString & text);
  fn _ZN7QTabBar6addTabERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  int QTabBar::addTab(const QIcon & icon, const QString & text);
  fn _ZN7QTabBar6addTabERK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  void QTabBar::setTabToolTip(int index, const QString & tip);
  fn _ZN7QTabBar13setTabToolTipEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTabBar::currentChanged(int index);
  fn _ZN7QTabBar14currentChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QTabBar::setTabTextColor(int index, const QColor & color);
  fn _ZN7QTabBar15setTabTextColorEiRK6QColor(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTabBar::moveTab(int from, int to);
  fn _ZN7QTabBar7moveTabEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  QVariant QTabBar::tabData(int index);
  fn _ZNK7QTabBar7tabDataEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabBar::drawBase();
  fn _ZNK7QTabBar8drawBaseEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QTabBar::currentIndex();
  fn _ZNK7QTabBar12currentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTabBar::setDrawBase(bool drawTheBase);
  fn _ZN7QTabBar11setDrawBaseEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QTabBar::setUsesScrollButtons(bool useButtons);
  fn _ZN7QTabBar20setUsesScrollButtonsEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QSize QTabBar::sizeHint();
  fn _ZNK7QTabBar8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabBar::setIconSize(const QSize & size);
  fn _ZN7QTabBar11setIconSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QTabBar::setTabText(int index, const QString & text);
  fn _ZN7QTabBar10setTabTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QTabBar::tabMoved(int from, int to);
  fn _ZN7QTabBar8tabMovedEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
}

// body block begin
// class sizeof(QTabBar)=1
pub struct QTabBar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTabBar {
  pub fn usesScrollButtons<T: QTabBar_usesScrollButtons>(&mut self, value: T) -> i8 {
    return value.usesScrollButtons(self);
    // return 1;
  }
}

pub trait QTabBar_usesScrollButtons {
  fn usesScrollButtons(self, rsthis: &mut QTabBar) -> i8;
}

// proto:  bool QTabBar::usesScrollButtons();
impl<'a> /*trait*/ QTabBar_usesScrollButtons for () {
  fn usesScrollButtons(self, rsthis: &mut QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar17usesScrollButtonsEv()};
    let mut ret = unsafe {_ZNK7QTabBar17usesScrollButtonsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn autoHide<T: QTabBar_autoHide>(&mut self, value: T) -> i8 {
    return value.autoHide(self);
    // return 1;
  }
}

pub trait QTabBar_autoHide {
  fn autoHide(self, rsthis: &mut QTabBar) -> i8;
}

// proto:  bool QTabBar::autoHide();
impl<'a> /*trait*/ QTabBar_autoHide for () {
  fn autoHide(self, rsthis: &mut QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8autoHideEv()};
    let mut ret = unsafe {_ZNK7QTabBar8autoHideEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabToolTip<T: QTabBar_tabToolTip>(&mut self, value: T) -> QString {
    return value.tabToolTip(self);
    // return 1;
  }
}

pub trait QTabBar_tabToolTip {
  fn tabToolTip(self, rsthis: &mut QTabBar) -> QString;
}

// proto:  QString QTabBar::tabToolTip(int index);
impl<'a> /*trait*/ QTabBar_tabToolTip for (i32) {
  fn tabToolTip(self, rsthis: &mut QTabBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar10tabToolTipEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QTabBar10tabToolTipEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn expanding<T: QTabBar_expanding>(&mut self, value: T) -> i8 {
    return value.expanding(self);
    // return 1;
  }
}

pub trait QTabBar_expanding {
  fn expanding(self, rsthis: &mut QTabBar) -> i8;
}

// proto:  bool QTabBar::expanding();
impl<'a> /*trait*/ QTabBar_expanding for () {
  fn expanding(self, rsthis: &mut QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar9expandingEv()};
    let mut ret = unsafe {_ZNK7QTabBar9expandingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setDocumentMode<T: QTabBar_setDocumentMode>(&mut self, value: T)  {
     value.setDocumentMode(self);
    // return 1;
  }
}

pub trait QTabBar_setDocumentMode {
  fn setDocumentMode(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setDocumentMode(bool set);
impl<'a> /*trait*/ QTabBar_setDocumentMode for (i8) {
  fn setDocumentMode(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setDocumentModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QTabBar15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn count<T: QTabBar_count>(&mut self, value: T) -> i32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QTabBar_count {
  fn count(self, rsthis: &mut QTabBar) -> i32;
}

// proto:  int QTabBar::count();
impl<'a> /*trait*/ QTabBar_count for () {
  fn count(self, rsthis: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar5countEv()};
    let mut ret = unsafe {_ZNK7QTabBar5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setChangeCurrentOnDrag<T: QTabBar_setChangeCurrentOnDrag>(&mut self, value: T)  {
     value.setChangeCurrentOnDrag(self);
    // return 1;
  }
}

pub trait QTabBar_setChangeCurrentOnDrag {
  fn setChangeCurrentOnDrag(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setChangeCurrentOnDrag(bool change);
impl<'a> /*trait*/ QTabBar_setChangeCurrentOnDrag for (i8) {
  fn setChangeCurrentOnDrag(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar22setChangeCurrentOnDragEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QTabBar22setChangeCurrentOnDragEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabIcon<T: QTabBar_tabIcon>(&mut self, value: T) -> QIcon {
    return value.tabIcon(self);
    // return 1;
  }
}

pub trait QTabBar_tabIcon {
  fn tabIcon(self, rsthis: &mut QTabBar) -> QIcon;
}

// proto:  QIcon QTabBar::tabIcon(int index);
impl<'a> /*trait*/ QTabBar_tabIcon for (i32) {
  fn tabIcon(self, rsthis: &mut QTabBar) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabIconEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QTabBar7tabIconEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabBarClicked<T: QTabBar_tabBarClicked>(&mut self, value: T)  {
     value.tabBarClicked(self);
    // return 1;
  }
}

pub trait QTabBar_tabBarClicked {
  fn tabBarClicked(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::tabBarClicked(int index);
impl<'a> /*trait*/ QTabBar_tabBarClicked for (i32) {
  fn tabBarClicked(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar13tabBarClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QTabBar13tabBarClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn minimumSizeHint<T: QTabBar_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QTabBar_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QTabBar) -> QSize;
}

// proto:  QSize QTabBar::minimumSizeHint();
impl<'a> /*trait*/ QTabBar_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QTabBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK7QTabBar15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabsClosable<T: QTabBar_setTabsClosable>(&mut self, value: T)  {
     value.setTabsClosable(self);
    // return 1;
  }
}

pub trait QTabBar_setTabsClosable {
  fn setTabsClosable(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setTabsClosable(bool closable);
impl<'a> /*trait*/ QTabBar_setTabsClosable for (i8) {
  fn setTabsClosable(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setTabsClosableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QTabBar15setTabsClosableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn changeCurrentOnDrag<T: QTabBar_changeCurrentOnDrag>(&mut self, value: T) -> i8 {
    return value.changeCurrentOnDrag(self);
    // return 1;
  }
}

pub trait QTabBar_changeCurrentOnDrag {
  fn changeCurrentOnDrag(self, rsthis: &mut QTabBar) -> i8;
}

// proto:  bool QTabBar::changeCurrentOnDrag();
impl<'a> /*trait*/ QTabBar_changeCurrentOnDrag for () {
  fn changeCurrentOnDrag(self, rsthis: &mut QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar19changeCurrentOnDragEv()};
    let mut ret = unsafe {_ZNK7QTabBar19changeCurrentOnDragEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabWhatsThis<T: QTabBar_setTabWhatsThis>(&mut self, value: T)  {
     value.setTabWhatsThis(self);
    // return 1;
  }
}

pub trait QTabBar_setTabWhatsThis {
  fn setTabWhatsThis(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setTabWhatsThis(int index, const QString & text);
impl<'a> /*trait*/ QTabBar_setTabWhatsThis for (i32, &'a  QString) {
  fn setTabWhatsThis(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setTabWhatsThisEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QTabBar15setTabWhatsThisEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn metaObject<T: QTabBar_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QTabBar_metaObject {
  fn metaObject(self, rsthis: &mut QTabBar) ;
}

// proto:  const QMetaObject * QTabBar::metaObject();
impl<'a> /*trait*/ QTabBar_metaObject for () {
  fn metaObject(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar10metaObjectEv()};
     unsafe {_ZNK7QTabBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
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
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QTabBarC1ERKS_(qthis, arg0)};
    let rsthis = QTabBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn insertTab<T: QTabBar_insertTab>(&mut self, value: T) -> i32 {
    return value.insertTab(self);
    // return 1;
  }
}

pub trait QTabBar_insertTab {
  fn insertTab(self, rsthis: &mut QTabBar) -> i32;
}

// proto:  int QTabBar::insertTab(int index, const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QTabBar_insertTab for (i32, &'a  QIcon, &'a  QString) {
  fn insertTab(self, rsthis: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar9insertTabEiRK5QIconRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QTabBar9insertTabEiRK5QIconRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabIcon<T: QTabBar_setTabIcon>(&mut self, value: T)  {
     value.setTabIcon(self);
    // return 1;
  }
}

pub trait QTabBar_setTabIcon {
  fn setTabIcon(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setTabIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QTabBar_setTabIcon for (i32, &'a  QIcon) {
  fn setTabIcon(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setTabIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QTabBar10setTabIconEiRK5QIcon(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn isMovable<T: QTabBar_isMovable>(&mut self, value: T) -> i8 {
    return value.isMovable(self);
    // return 1;
  }
}

pub trait QTabBar_isMovable {
  fn isMovable(self, rsthis: &mut QTabBar) -> i8;
}

// proto:  bool QTabBar::isMovable();
impl<'a> /*trait*/ QTabBar_isMovable for () {
  fn isMovable(self, rsthis: &mut QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar9isMovableEv()};
    let mut ret = unsafe {_ZNK7QTabBar9isMovableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setExpanding<T: QTabBar_setExpanding>(&mut self, value: T)  {
     value.setExpanding(self);
    // return 1;
  }
}

pub trait QTabBar_setExpanding {
  fn setExpanding(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setExpanding(bool enabled);
impl<'a> /*trait*/ QTabBar_setExpanding for (i8) {
  fn setExpanding(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar12setExpandingEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QTabBar12setExpandingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn removeTab<T: QTabBar_removeTab>(&mut self, value: T)  {
     value.removeTab(self);
    // return 1;
  }
}

pub trait QTabBar_removeTab {
  fn removeTab(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::removeTab(int index);
impl<'a> /*trait*/ QTabBar_removeTab for (i32) {
  fn removeTab(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar9removeTabEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QTabBar9removeTabEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabEnabled<T: QTabBar_setTabEnabled>(&mut self, value: T)  {
     value.setTabEnabled(self);
    // return 1;
  }
}

pub trait QTabBar_setTabEnabled {
  fn setTabEnabled(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setTabEnabled(int index, bool );
impl<'a> /*trait*/ QTabBar_setTabEnabled for (i32, i8) {
  fn setTabEnabled(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar13setTabEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as int8_t;
     unsafe {_ZN7QTabBar13setTabEnabledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn isTabEnabled<T: QTabBar_isTabEnabled>(&mut self, value: T) -> i8 {
    return value.isTabEnabled(self);
    // return 1;
  }
}

pub trait QTabBar_isTabEnabled {
  fn isTabEnabled(self, rsthis: &mut QTabBar) -> i8;
}

// proto:  bool QTabBar::isTabEnabled(int index);
impl<'a> /*trait*/ QTabBar_isTabEnabled for (i32) {
  fn isTabEnabled(self, rsthis: &mut QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12isTabEnabledEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QTabBar12isTabEnabledEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setCurrentIndex<T: QTabBar_setCurrentIndex>(&mut self, value: T)  {
     value.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QTabBar_setCurrentIndex {
  fn setCurrentIndex(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setCurrentIndex(int index);
impl<'a> /*trait*/ QTabBar_setCurrentIndex for (i32) {
  fn setCurrentIndex(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QTabBar15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabRect<T: QTabBar_tabRect>(&mut self, value: T) -> QRect {
    return value.tabRect(self);
    // return 1;
  }
}

pub trait QTabBar_tabRect {
  fn tabRect(self, rsthis: &mut QTabBar) -> QRect;
}

// proto:  QRect QTabBar::tabRect(int index);
impl<'a> /*trait*/ QTabBar_tabRect for (i32) {
  fn tabRect(self, rsthis: &mut QTabBar) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabRectEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QTabBar7tabRectEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabsClosable<T: QTabBar_tabsClosable>(&mut self, value: T) -> i8 {
    return value.tabsClosable(self);
    // return 1;
  }
}

pub trait QTabBar_tabsClosable {
  fn tabsClosable(self, rsthis: &mut QTabBar) -> i8;
}

// proto:  bool QTabBar::tabsClosable();
impl<'a> /*trait*/ QTabBar_tabsClosable for () {
  fn tabsClosable(self, rsthis: &mut QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12tabsClosableEv()};
    let mut ret = unsafe {_ZNK7QTabBar12tabsClosableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabCloseRequested<T: QTabBar_tabCloseRequested>(&mut self, value: T)  {
     value.tabCloseRequested(self);
    // return 1;
  }
}

pub trait QTabBar_tabCloseRequested {
  fn tabCloseRequested(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::tabCloseRequested(int index);
impl<'a> /*trait*/ QTabBar_tabCloseRequested for (i32) {
  fn tabCloseRequested(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar17tabCloseRequestedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QTabBar17tabCloseRequestedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setMovable<T: QTabBar_setMovable>(&mut self, value: T)  {
     value.setMovable(self);
    // return 1;
  }
}

pub trait QTabBar_setMovable {
  fn setMovable(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setMovable(bool movable);
impl<'a> /*trait*/ QTabBar_setMovable for (i8) {
  fn setMovable(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setMovableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QTabBar10setMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setAutoHide<T: QTabBar_setAutoHide>(&mut self, value: T)  {
     value.setAutoHide(self);
    // return 1;
  }
}

pub trait QTabBar_setAutoHide {
  fn setAutoHide(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setAutoHide(bool hide);
impl<'a> /*trait*/ QTabBar_setAutoHide for (i8) {
  fn setAutoHide(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar11setAutoHideEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QTabBar11setAutoHideEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn iconSize<T: QTabBar_iconSize>(&mut self, value: T) -> QSize {
    return value.iconSize(self);
    // return 1;
  }
}

pub trait QTabBar_iconSize {
  fn iconSize(self, rsthis: &mut QTabBar) -> QSize;
}

// proto:  QSize QTabBar::iconSize();
impl<'a> /*trait*/ QTabBar_iconSize for () {
  fn iconSize(self, rsthis: &mut QTabBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8iconSizeEv()};
    let mut ret = unsafe {_ZNK7QTabBar8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabText<T: QTabBar_tabText>(&mut self, value: T) -> QString {
    return value.tabText(self);
    // return 1;
  }
}

pub trait QTabBar_tabText {
  fn tabText(self, rsthis: &mut QTabBar) -> QString;
}

// proto:  QString QTabBar::tabText(int index);
impl<'a> /*trait*/ QTabBar_tabText for (i32) {
  fn tabText(self, rsthis: &mut QTabBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabTextEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QTabBar7tabTextEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabWhatsThis<T: QTabBar_tabWhatsThis>(&mut self, value: T) -> QString {
    return value.tabWhatsThis(self);
    // return 1;
  }
}

pub trait QTabBar_tabWhatsThis {
  fn tabWhatsThis(self, rsthis: &mut QTabBar) -> QString;
}

// proto:  QString QTabBar::tabWhatsThis(int index);
impl<'a> /*trait*/ QTabBar_tabWhatsThis for (i32) {
  fn tabWhatsThis(self, rsthis: &mut QTabBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12tabWhatsThisEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QTabBar12tabWhatsThisEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn documentMode<T: QTabBar_documentMode>(&mut self, value: T) -> i8 {
    return value.documentMode(self);
    // return 1;
  }
}

pub trait QTabBar_documentMode {
  fn documentMode(self, rsthis: &mut QTabBar) -> i8;
}

// proto:  bool QTabBar::documentMode();
impl<'a> /*trait*/ QTabBar_documentMode for () {
  fn documentMode(self, rsthis: &mut QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12documentModeEv()};
    let mut ret = unsafe {_ZNK7QTabBar12documentModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabAt<T: QTabBar_tabAt>(&mut self, value: T) -> i32 {
    return value.tabAt(self);
    // return 1;
  }
}

pub trait QTabBar_tabAt {
  fn tabAt(self, rsthis: &mut QTabBar) -> i32;
}

// proto:  int QTabBar::tabAt(const QPoint & pos);
impl<'a> /*trait*/ QTabBar_tabAt for (&'a  QPoint) {
  fn tabAt(self, rsthis: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar5tabAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QTabBar5tabAtERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabData<T: QTabBar_setTabData>(&mut self, value: T)  {
     value.setTabData(self);
    // return 1;
  }
}

pub trait QTabBar_setTabData {
  fn setTabData(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setTabData(int index, const QVariant & data);
impl<'a> /*trait*/ QTabBar_setTabData for (i32, &'a  QVariant) {
  fn setTabData(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setTabDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QTabBar10setTabDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
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
  pub fn tabTextColor<T: QTabBar_tabTextColor>(&mut self, value: T) -> QColor {
    return value.tabTextColor(self);
    // return 1;
  }
}

pub trait QTabBar_tabTextColor {
  fn tabTextColor(self, rsthis: &mut QTabBar) -> QColor;
}

// proto:  QColor QTabBar::tabTextColor(int index);
impl<'a> /*trait*/ QTabBar_tabTextColor for (i32) {
  fn tabTextColor(self, rsthis: &mut QTabBar) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12tabTextColorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QTabBar12tabTextColorEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn FreeQTabBar<T: QTabBar_FreeQTabBar>(&mut self, value: T)  {
     value.FreeQTabBar(self);
    // return 1;
  }
}

pub trait QTabBar_FreeQTabBar {
  fn FreeQTabBar(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::FreeQTabBar();
impl<'a> /*trait*/ QTabBar_FreeQTabBar for () {
  fn FreeQTabBar(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBarD0Ev()};
     unsafe {_ZN7QTabBarD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  int QTabBar::insertTab(int index, const QString & text);
impl<'a> /*trait*/ QTabBar_insertTab for (i32, &'a  QString) {
  fn insertTab(self, rsthis: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar9insertTabEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QTabBar9insertTabEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabBarDoubleClicked<T: QTabBar_tabBarDoubleClicked>(&mut self, value: T)  {
     value.tabBarDoubleClicked(self);
    // return 1;
  }
}

pub trait QTabBar_tabBarDoubleClicked {
  fn tabBarDoubleClicked(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::tabBarDoubleClicked(int index);
impl<'a> /*trait*/ QTabBar_tabBarDoubleClicked for (i32) {
  fn tabBarDoubleClicked(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar19tabBarDoubleClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QTabBar19tabBarDoubleClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn addTab<T: QTabBar_addTab>(&mut self, value: T) -> i32 {
    return value.addTab(self);
    // return 1;
  }
}

pub trait QTabBar_addTab {
  fn addTab(self, rsthis: &mut QTabBar) -> i32;
}

// proto:  int QTabBar::addTab(const QString & text);
impl<'a> /*trait*/ QTabBar_addTab for (&'a  QString) {
  fn addTab(self, rsthis: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar6addTabERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QTabBar6addTabERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto:  int QTabBar::addTab(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QTabBar_addTab for (&'a  QIcon, &'a  QString) {
  fn addTab(self, rsthis: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar6addTabERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QTabBar6addTabERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabToolTip<T: QTabBar_setTabToolTip>(&mut self, value: T)  {
     value.setTabToolTip(self);
    // return 1;
  }
}

pub trait QTabBar_setTabToolTip {
  fn setTabToolTip(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setTabToolTip(int index, const QString & tip);
impl<'a> /*trait*/ QTabBar_setTabToolTip for (i32, &'a  QString) {
  fn setTabToolTip(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar13setTabToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QTabBar13setTabToolTipEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn currentChanged<T: QTabBar_currentChanged>(&mut self, value: T)  {
     value.currentChanged(self);
    // return 1;
  }
}

pub trait QTabBar_currentChanged {
  fn currentChanged(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::currentChanged(int index);
impl<'a> /*trait*/ QTabBar_currentChanged for (i32) {
  fn currentChanged(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar14currentChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN7QTabBar14currentChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabTextColor<T: QTabBar_setTabTextColor>(&mut self, value: T)  {
     value.setTabTextColor(self);
    // return 1;
  }
}

pub trait QTabBar_setTabTextColor {
  fn setTabTextColor(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setTabTextColor(int index, const QColor & color);
impl<'a> /*trait*/ QTabBar_setTabTextColor for (i32, &'a  QColor) {
  fn setTabTextColor(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setTabTextColorEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QTabBar15setTabTextColorEiRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn moveTab<T: QTabBar_moveTab>(&mut self, value: T)  {
     value.moveTab(self);
    // return 1;
  }
}

pub trait QTabBar_moveTab {
  fn moveTab(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::moveTab(int from, int to);
impl<'a> /*trait*/ QTabBar_moveTab for (i32, i32) {
  fn moveTab(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar7moveTabEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QTabBar7moveTabEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabData<T: QTabBar_tabData>(&mut self, value: T) -> QVariant {
    return value.tabData(self);
    // return 1;
  }
}

pub trait QTabBar_tabData {
  fn tabData(self, rsthis: &mut QTabBar) -> QVariant;
}

// proto:  QVariant QTabBar::tabData(int index);
impl<'a> /*trait*/ QTabBar_tabData for (i32) {
  fn tabData(self, rsthis: &mut QTabBar) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabDataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK7QTabBar7tabDataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn drawBase<T: QTabBar_drawBase>(&mut self, value: T) -> i8 {
    return value.drawBase(self);
    // return 1;
  }
}

pub trait QTabBar_drawBase {
  fn drawBase(self, rsthis: &mut QTabBar) -> i8;
}

// proto:  bool QTabBar::drawBase();
impl<'a> /*trait*/ QTabBar_drawBase for () {
  fn drawBase(self, rsthis: &mut QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8drawBaseEv()};
    let mut ret = unsafe {_ZNK7QTabBar8drawBaseEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn currentIndex<T: QTabBar_currentIndex>(&mut self, value: T) -> i32 {
    return value.currentIndex(self);
    // return 1;
  }
}

pub trait QTabBar_currentIndex {
  fn currentIndex(self, rsthis: &mut QTabBar) -> i32;
}

// proto:  int QTabBar::currentIndex();
impl<'a> /*trait*/ QTabBar_currentIndex for () {
  fn currentIndex(self, rsthis: &mut QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12currentIndexEv()};
    let mut ret = unsafe {_ZNK7QTabBar12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setDrawBase<T: QTabBar_setDrawBase>(&mut self, value: T)  {
     value.setDrawBase(self);
    // return 1;
  }
}

pub trait QTabBar_setDrawBase {
  fn setDrawBase(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setDrawBase(bool drawTheBase);
impl<'a> /*trait*/ QTabBar_setDrawBase for (i8) {
  fn setDrawBase(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar11setDrawBaseEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QTabBar11setDrawBaseEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setUsesScrollButtons<T: QTabBar_setUsesScrollButtons>(&mut self, value: T)  {
     value.setUsesScrollButtons(self);
    // return 1;
  }
}

pub trait QTabBar_setUsesScrollButtons {
  fn setUsesScrollButtons(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setUsesScrollButtons(bool useButtons);
impl<'a> /*trait*/ QTabBar_setUsesScrollButtons for (i8) {
  fn setUsesScrollButtons(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar20setUsesScrollButtonsEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QTabBar20setUsesScrollButtonsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn sizeHint<T: QTabBar_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QTabBar_sizeHint {
  fn sizeHint(self, rsthis: &mut QTabBar) -> QSize;
}

// proto:  QSize QTabBar::sizeHint();
impl<'a> /*trait*/ QTabBar_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QTabBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8sizeHintEv()};
    let mut ret = unsafe {_ZNK7QTabBar8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setIconSize<T: QTabBar_setIconSize>(&mut self, value: T)  {
     value.setIconSize(self);
    // return 1;
  }
}

pub trait QTabBar_setIconSize {
  fn setIconSize(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setIconSize(const QSize & size);
impl<'a> /*trait*/ QTabBar_setIconSize for (&'a  QSize) {
  fn setIconSize(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QTabBar11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn setTabText<T: QTabBar_setTabText>(&mut self, value: T)  {
     value.setTabText(self);
    // return 1;
  }
}

pub trait QTabBar_setTabText {
  fn setTabText(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::setTabText(int index, const QString & text);
impl<'a> /*trait*/ QTabBar_setTabText for (i32, &'a  QString) {
  fn setTabText(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setTabTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QTabBar10setTabTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTabBar {
  pub fn tabMoved<T: QTabBar_tabMoved>(&mut self, value: T)  {
     value.tabMoved(self);
    // return 1;
  }
}

pub trait QTabBar_tabMoved {
  fn tabMoved(self, rsthis: &mut QTabBar) ;
}

// proto:  void QTabBar::tabMoved(int from, int to);
impl<'a> /*trait*/ QTabBar_tabMoved for (i32, i32) {
  fn tabMoved(self, rsthis: &mut QTabBar)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar8tabMovedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QTabBar8tabMovedEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

