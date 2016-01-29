// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qtabbar.h
// dst-file: /src/widgets/qtabbar.rs
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
use super::qwidget::*; // 773
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::super::gui::qicon::*; // 771
use super::super::core::qsize::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qrect::*; // 771
use super::super::core::qpoint::*; // 771
use super::super::core::qvariant::*; // 771
use super::super::gui::qcolor::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTabBar_Class_Size() -> c_int;
  // proto:  bool QTabBar::usesScrollButtons();
  fn C_ZNK7QTabBar17usesScrollButtonsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QTabBar::autoHide();
  fn C_ZNK7QTabBar8autoHideEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QTabBar::tabToolTip(int index);
  fn C_ZNK7QTabBar10tabToolTipEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabBar::expanding();
  fn C_ZNK7QTabBar9expandingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTabBar::setDocumentMode(bool set);
  fn C_ZN7QTabBar15setDocumentModeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  int QTabBar::count();
  fn C_ZNK7QTabBar5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTabBar::setChangeCurrentOnDrag(bool change);
  fn C_ZN7QTabBar22setChangeCurrentOnDragEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QIcon QTabBar::tabIcon(int index);
  fn C_ZNK7QTabBar7tabIconEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QSize QTabBar::minimumSizeHint();
  fn C_ZNK7QTabBar15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTabBar::setTabsClosable(bool closable);
  fn C_ZN7QTabBar15setTabsClosableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QTabBar::changeCurrentOnDrag();
  fn C_ZNK7QTabBar19changeCurrentOnDragEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTabBar::setTabWhatsThis(int index, const QString & text);
  fn C_ZN7QTabBar15setTabWhatsThisEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  const QMetaObject * QTabBar::metaObject();
  fn C_ZNK7QTabBar10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTabBar::insertTab(int index, const QIcon & icon, const QString & text);
  fn C_ZN7QTabBar9insertTabEiRK5QIconRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
  // proto:  void QTabBar::setTabIcon(int index, const QIcon & icon);
  fn C_ZN7QTabBar10setTabIconEiRK5QIcon(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  bool QTabBar::isMovable();
  fn C_ZNK7QTabBar9isMovableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTabBar::setExpanding(bool enabled);
  fn C_ZN7QTabBar12setExpandingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTabBar::removeTab(int index);
  fn C_ZN7QTabBar9removeTabEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTabBar::setTabEnabled(int index, bool );
  fn C_ZN7QTabBar13setTabEnabledEib(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_char);
  // proto:  bool QTabBar::isTabEnabled(int index);
  fn C_ZNK7QTabBar12isTabEnabledEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  void QTabBar::setCurrentIndex(int index);
  fn C_ZN7QTabBar15setCurrentIndexEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  QRect QTabBar::tabRect(int index);
  fn C_ZNK7QTabBar7tabRectEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabBar::tabsClosable();
  fn C_ZNK7QTabBar12tabsClosableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTabBar::setMovable(bool movable);
  fn C_ZN7QTabBar10setMovableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTabBar::setAutoHide(bool hide);
  fn C_ZN7QTabBar11setAutoHideEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QSize QTabBar::iconSize();
  fn C_ZNK7QTabBar8iconSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QTabBar::tabText(int index);
  fn C_ZNK7QTabBar7tabTextEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QString QTabBar::tabWhatsThis(int index);
  fn C_ZNK7QTabBar12tabWhatsThisEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabBar::documentMode();
  fn C_ZNK7QTabBar12documentModeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTabBar::tabAt(const QPoint & pos);
  fn C_ZNK7QTabBar5tabAtERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  void QTabBar::setTabData(int index, const QVariant & data);
  fn C_ZN7QTabBar10setTabDataEiRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTabBar::QTabBar(QWidget * parent);
  fn C_ZN7QTabBarC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  QColor QTabBar::tabTextColor(int index);
  fn C_ZNK7QTabBar12tabTextColorEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTabBar::~QTabBar();
  fn C_ZN7QTabBarD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QTabBar::insertTab(int index, const QString & text);
  fn C_ZN7QTabBar9insertTabEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_int;
  // proto:  int QTabBar::addTab(const QString & text);
  fn C_ZN7QTabBar6addTabERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  int QTabBar::addTab(const QIcon & icon, const QString & text);
  fn C_ZN7QTabBar6addTabERK5QIconRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  void QTabBar::setTabToolTip(int index, const QString & tip);
  fn C_ZN7QTabBar13setTabToolTipEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTabBar::setTabTextColor(int index, const QColor & color);
  fn C_ZN7QTabBar15setTabTextColorEiRK6QColor(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTabBar::moveTab(int from, int to);
  fn C_ZN7QTabBar7moveTabEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QVariant QTabBar::tabData(int index);
  fn C_ZNK7QTabBar7tabDataEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabBar::drawBase();
  fn C_ZNK7QTabBar8drawBaseEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTabBar::currentIndex();
  fn C_ZNK7QTabBar12currentIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTabBar::setDrawBase(bool drawTheBase);
  fn C_ZN7QTabBar11setDrawBaseEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTabBar::setUsesScrollButtons(bool useButtons);
  fn C_ZN7QTabBar20setUsesScrollButtonsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QSize QTabBar::sizeHint();
  fn C_ZNK7QTabBar8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTabBar::setIconSize(const QSize & size);
  fn C_ZN7QTabBar11setIconSizeERK5QSize(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTabBar::setTabText(int index, const QString & text);
  fn C_ZN7QTabBar10setTabTextEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  fn QTabBar_SlotProxy_connect__ZN7QTabBar13tabBarClickedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTabBar_SlotProxy_connect__ZN7QTabBar19tabBarDoubleClickedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTabBar_SlotProxy_connect__ZN7QTabBar17tabCloseRequestedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTabBar_SlotProxy_connect__ZN7QTabBar8tabMovedEii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTabBar_SlotProxy_connect__ZN7QTabBar14currentChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTabBar)=1
#[derive(Default)]
pub struct QTabBar {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _tabCloseRequested: QTabBar_tabCloseRequested_signal,
  pub _tabBarDoubleClicked: QTabBar_tabBarDoubleClicked_signal,
  pub _tabMoved: QTabBar_tabMoved_signal,
  pub _tabBarClicked: QTabBar_tabBarClicked_signal,
  pub _currentChanged: QTabBar_currentChanged_signal,
}

impl /*struct*/ QTabBar {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTabBar {
    return QTabBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTabBar {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QTabBar {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  bool QTabBar::usesScrollButtons();
impl /*struct*/ QTabBar {
  pub fn usesScrollButtons<RetType, T: QTabBar_usesScrollButtons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.usesScrollButtons(self);
    // return 1;
  }
}

pub trait QTabBar_usesScrollButtons<RetType> {
  fn usesScrollButtons(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  bool QTabBar::usesScrollButtons();
impl<'a> /*trait*/ QTabBar_usesScrollButtons<i8> for () {
  fn usesScrollButtons(self , rsthis: & QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar17usesScrollButtonsEv()};
    let mut ret = unsafe {C_ZNK7QTabBar17usesScrollButtonsEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QTabBar::autoHide();
impl /*struct*/ QTabBar {
  pub fn autoHide<RetType, T: QTabBar_autoHide<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoHide(self);
    // return 1;
  }
}

pub trait QTabBar_autoHide<RetType> {
  fn autoHide(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  bool QTabBar::autoHide();
impl<'a> /*trait*/ QTabBar_autoHide<i8> for () {
  fn autoHide(self , rsthis: & QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8autoHideEv()};
    let mut ret = unsafe {C_ZNK7QTabBar8autoHideEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QTabBar::tabToolTip(int index);
impl /*struct*/ QTabBar {
  pub fn tabToolTip<RetType, T: QTabBar_tabToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabToolTip(self);
    // return 1;
  }
}

pub trait QTabBar_tabToolTip<RetType> {
  fn tabToolTip(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  QString QTabBar::tabToolTip(int index);
impl<'a> /*trait*/ QTabBar_tabToolTip<QString> for (i32) {
  fn tabToolTip(self , rsthis: & QTabBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar10tabToolTipEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QTabBar10tabToolTipEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTabBar::expanding();
impl /*struct*/ QTabBar {
  pub fn expanding<RetType, T: QTabBar_expanding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expanding(self);
    // return 1;
  }
}

pub trait QTabBar_expanding<RetType> {
  fn expanding(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  bool QTabBar::expanding();
impl<'a> /*trait*/ QTabBar_expanding<i8> for () {
  fn expanding(self , rsthis: & QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar9expandingEv()};
    let mut ret = unsafe {C_ZNK7QTabBar9expandingEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTabBar::setDocumentMode(bool set);
impl /*struct*/ QTabBar {
  pub fn setDocumentMode<RetType, T: QTabBar_setDocumentMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMode(self);
    // return 1;
  }
}

pub trait QTabBar_setDocumentMode<RetType> {
  fn setDocumentMode(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setDocumentMode(bool set);
impl<'a> /*trait*/ QTabBar_setDocumentMode<()> for (i8) {
  fn setDocumentMode(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setDocumentModeEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QTabBar15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTabBar::count();
impl /*struct*/ QTabBar {
  pub fn count<RetType, T: QTabBar_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QTabBar_count<RetType> {
  fn count(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  int QTabBar::count();
impl<'a> /*trait*/ QTabBar_count<i32> for () {
  fn count(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar5countEv()};
    let mut ret = unsafe {C_ZNK7QTabBar5countEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTabBar::setChangeCurrentOnDrag(bool change);
impl /*struct*/ QTabBar {
  pub fn setChangeCurrentOnDrag<RetType, T: QTabBar_setChangeCurrentOnDrag<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setChangeCurrentOnDrag(self);
    // return 1;
  }
}

pub trait QTabBar_setChangeCurrentOnDrag<RetType> {
  fn setChangeCurrentOnDrag(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setChangeCurrentOnDrag(bool change);
impl<'a> /*trait*/ QTabBar_setChangeCurrentOnDrag<()> for (i8) {
  fn setChangeCurrentOnDrag(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar22setChangeCurrentOnDragEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QTabBar22setChangeCurrentOnDragEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QTabBar::tabIcon(int index);
impl /*struct*/ QTabBar {
  pub fn tabIcon<RetType, T: QTabBar_tabIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabIcon(self);
    // return 1;
  }
}

pub trait QTabBar_tabIcon<RetType> {
  fn tabIcon(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  QIcon QTabBar::tabIcon(int index);
impl<'a> /*trait*/ QTabBar_tabIcon<QIcon> for (i32) {
  fn tabIcon(self , rsthis: & QTabBar) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabIconEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QTabBar7tabIconEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QTabBar::minimumSizeHint();
impl /*struct*/ QTabBar {
  pub fn minimumSizeHint<RetType, T: QTabBar_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QTabBar_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  QSize QTabBar::minimumSizeHint();
impl<'a> /*trait*/ QTabBar_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QTabBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar15minimumSizeHintEv()};
    let mut ret = unsafe {C_ZNK7QTabBar15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabBar::setTabsClosable(bool closable);
impl /*struct*/ QTabBar {
  pub fn setTabsClosable<RetType, T: QTabBar_setTabsClosable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabsClosable(self);
    // return 1;
  }
}

pub trait QTabBar_setTabsClosable<RetType> {
  fn setTabsClosable(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setTabsClosable(bool closable);
impl<'a> /*trait*/ QTabBar_setTabsClosable<()> for (i8) {
  fn setTabsClosable(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setTabsClosableEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QTabBar15setTabsClosableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTabBar::changeCurrentOnDrag();
impl /*struct*/ QTabBar {
  pub fn changeCurrentOnDrag<RetType, T: QTabBar_changeCurrentOnDrag<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.changeCurrentOnDrag(self);
    // return 1;
  }
}

pub trait QTabBar_changeCurrentOnDrag<RetType> {
  fn changeCurrentOnDrag(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  bool QTabBar::changeCurrentOnDrag();
impl<'a> /*trait*/ QTabBar_changeCurrentOnDrag<i8> for () {
  fn changeCurrentOnDrag(self , rsthis: & QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar19changeCurrentOnDragEv()};
    let mut ret = unsafe {C_ZNK7QTabBar19changeCurrentOnDragEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTabBar::setTabWhatsThis(int index, const QString & text);
impl /*struct*/ QTabBar {
  pub fn setTabWhatsThis<RetType, T: QTabBar_setTabWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabWhatsThis(self);
    // return 1;
  }
}

pub trait QTabBar_setTabWhatsThis<RetType> {
  fn setTabWhatsThis(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setTabWhatsThis(int index, const QString & text);
impl<'a> /*trait*/ QTabBar_setTabWhatsThis<()> for (i32, &'a QString) {
  fn setTabWhatsThis(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setTabWhatsThisEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QTabBar15setTabWhatsThisEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QTabBar::metaObject();
impl /*struct*/ QTabBar {
  pub fn metaObject<RetType, T: QTabBar_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTabBar_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  const QMetaObject * QTabBar::metaObject();
impl<'a> /*trait*/ QTabBar_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QTabBar) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar10metaObjectEv()};
    let mut ret = unsafe {C_ZNK7QTabBar10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTabBar::insertTab(int index, const QIcon & icon, const QString & text);
impl /*struct*/ QTabBar {
  pub fn insertTab<RetType, T: QTabBar_insertTab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertTab(self);
    // return 1;
  }
}

pub trait QTabBar_insertTab<RetType> {
  fn insertTab(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  int QTabBar::insertTab(int index, const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QTabBar_insertTab<i32> for (i32, &'a QIcon, &'a QString) {
  fn insertTab(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar9insertTabEiRK5QIconRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QTabBar9insertTabEiRK5QIconRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTabBar::setTabIcon(int index, const QIcon & icon);
impl /*struct*/ QTabBar {
  pub fn setTabIcon<RetType, T: QTabBar_setTabIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabIcon(self);
    // return 1;
  }
}

pub trait QTabBar_setTabIcon<RetType> {
  fn setTabIcon(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setTabIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QTabBar_setTabIcon<()> for (i32, &'a QIcon) {
  fn setTabIcon(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setTabIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QTabBar10setTabIconEiRK5QIcon(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QTabBar::isMovable();
impl /*struct*/ QTabBar {
  pub fn isMovable<RetType, T: QTabBar_isMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isMovable(self);
    // return 1;
  }
}

pub trait QTabBar_isMovable<RetType> {
  fn isMovable(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  bool QTabBar::isMovable();
impl<'a> /*trait*/ QTabBar_isMovable<i8> for () {
  fn isMovable(self , rsthis: & QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar9isMovableEv()};
    let mut ret = unsafe {C_ZNK7QTabBar9isMovableEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTabBar::setExpanding(bool enabled);
impl /*struct*/ QTabBar {
  pub fn setExpanding<RetType, T: QTabBar_setExpanding<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExpanding(self);
    // return 1;
  }
}

pub trait QTabBar_setExpanding<RetType> {
  fn setExpanding(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setExpanding(bool enabled);
impl<'a> /*trait*/ QTabBar_setExpanding<()> for (i8) {
  fn setExpanding(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar12setExpandingEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QTabBar12setExpandingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabBar::removeTab(int index);
impl /*struct*/ QTabBar {
  pub fn removeTab<RetType, T: QTabBar_removeTab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeTab(self);
    // return 1;
  }
}

pub trait QTabBar_removeTab<RetType> {
  fn removeTab(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::removeTab(int index);
impl<'a> /*trait*/ QTabBar_removeTab<()> for (i32) {
  fn removeTab(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar9removeTabEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QTabBar9removeTabEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabBar::setTabEnabled(int index, bool );
impl /*struct*/ QTabBar {
  pub fn setTabEnabled<RetType, T: QTabBar_setTabEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabEnabled(self);
    // return 1;
  }
}

pub trait QTabBar_setTabEnabled<RetType> {
  fn setTabEnabled(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setTabEnabled(int index, bool );
impl<'a> /*trait*/ QTabBar_setTabEnabled<()> for (i32, i8) {
  fn setTabEnabled(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar13setTabEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN7QTabBar13setTabEnabledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QTabBar::isTabEnabled(int index);
impl /*struct*/ QTabBar {
  pub fn isTabEnabled<RetType, T: QTabBar_isTabEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTabEnabled(self);
    // return 1;
  }
}

pub trait QTabBar_isTabEnabled<RetType> {
  fn isTabEnabled(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  bool QTabBar::isTabEnabled(int index);
impl<'a> /*trait*/ QTabBar_isTabEnabled<i8> for (i32) {
  fn isTabEnabled(self , rsthis: & QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12isTabEnabledEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QTabBar12isTabEnabledEi(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTabBar::setCurrentIndex(int index);
impl /*struct*/ QTabBar {
  pub fn setCurrentIndex<RetType, T: QTabBar_setCurrentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QTabBar_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setCurrentIndex(int index);
impl<'a> /*trait*/ QTabBar_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QTabBar15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QTabBar::tabRect(int index);
impl /*struct*/ QTabBar {
  pub fn tabRect<RetType, T: QTabBar_tabRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabRect(self);
    // return 1;
  }
}

pub trait QTabBar_tabRect<RetType> {
  fn tabRect(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  QRect QTabBar::tabRect(int index);
impl<'a> /*trait*/ QTabBar_tabRect<QRect> for (i32) {
  fn tabRect(self , rsthis: & QTabBar) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabRectEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QTabBar7tabRectEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTabBar::tabsClosable();
impl /*struct*/ QTabBar {
  pub fn tabsClosable<RetType, T: QTabBar_tabsClosable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabsClosable(self);
    // return 1;
  }
}

pub trait QTabBar_tabsClosable<RetType> {
  fn tabsClosable(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  bool QTabBar::tabsClosable();
impl<'a> /*trait*/ QTabBar_tabsClosable<i8> for () {
  fn tabsClosable(self , rsthis: & QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12tabsClosableEv()};
    let mut ret = unsafe {C_ZNK7QTabBar12tabsClosableEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTabBar::setMovable(bool movable);
impl /*struct*/ QTabBar {
  pub fn setMovable<RetType, T: QTabBar_setMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMovable(self);
    // return 1;
  }
}

pub trait QTabBar_setMovable<RetType> {
  fn setMovable(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setMovable(bool movable);
impl<'a> /*trait*/ QTabBar_setMovable<()> for (i8) {
  fn setMovable(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setMovableEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QTabBar10setMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabBar::setAutoHide(bool hide);
impl /*struct*/ QTabBar {
  pub fn setAutoHide<RetType, T: QTabBar_setAutoHide<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoHide(self);
    // return 1;
  }
}

pub trait QTabBar_setAutoHide<RetType> {
  fn setAutoHide(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setAutoHide(bool hide);
impl<'a> /*trait*/ QTabBar_setAutoHide<()> for (i8) {
  fn setAutoHide(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar11setAutoHideEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QTabBar11setAutoHideEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QTabBar::iconSize();
impl /*struct*/ QTabBar {
  pub fn iconSize<RetType, T: QTabBar_iconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QTabBar_iconSize<RetType> {
  fn iconSize(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  QSize QTabBar::iconSize();
impl<'a> /*trait*/ QTabBar_iconSize<QSize> for () {
  fn iconSize(self , rsthis: & QTabBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8iconSizeEv()};
    let mut ret = unsafe {C_ZNK7QTabBar8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTabBar::tabText(int index);
impl /*struct*/ QTabBar {
  pub fn tabText<RetType, T: QTabBar_tabText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabText(self);
    // return 1;
  }
}

pub trait QTabBar_tabText<RetType> {
  fn tabText(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  QString QTabBar::tabText(int index);
impl<'a> /*trait*/ QTabBar_tabText<QString> for (i32) {
  fn tabText(self , rsthis: & QTabBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabTextEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QTabBar7tabTextEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTabBar::tabWhatsThis(int index);
impl /*struct*/ QTabBar {
  pub fn tabWhatsThis<RetType, T: QTabBar_tabWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabWhatsThis(self);
    // return 1;
  }
}

pub trait QTabBar_tabWhatsThis<RetType> {
  fn tabWhatsThis(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  QString QTabBar::tabWhatsThis(int index);
impl<'a> /*trait*/ QTabBar_tabWhatsThis<QString> for (i32) {
  fn tabWhatsThis(self , rsthis: & QTabBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12tabWhatsThisEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QTabBar12tabWhatsThisEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTabBar::documentMode();
impl /*struct*/ QTabBar {
  pub fn documentMode<RetType, T: QTabBar_documentMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentMode(self);
    // return 1;
  }
}

pub trait QTabBar_documentMode<RetType> {
  fn documentMode(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  bool QTabBar::documentMode();
impl<'a> /*trait*/ QTabBar_documentMode<i8> for () {
  fn documentMode(self , rsthis: & QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12documentModeEv()};
    let mut ret = unsafe {C_ZNK7QTabBar12documentModeEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QTabBar::tabAt(const QPoint & pos);
impl /*struct*/ QTabBar {
  pub fn tabAt<RetType, T: QTabBar_tabAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabAt(self);
    // return 1;
  }
}

pub trait QTabBar_tabAt<RetType> {
  fn tabAt(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  int QTabBar::tabAt(const QPoint & pos);
impl<'a> /*trait*/ QTabBar_tabAt<i32> for (&'a QPoint) {
  fn tabAt(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar5tabAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QTabBar5tabAtERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTabBar::setTabData(int index, const QVariant & data);
impl /*struct*/ QTabBar {
  pub fn setTabData<RetType, T: QTabBar_setTabData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabData(self);
    // return 1;
  }
}

pub trait QTabBar_setTabData<RetType> {
  fn setTabData(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setTabData(int index, const QVariant & data);
impl<'a> /*trait*/ QTabBar_setTabData<()> for (i32, &'a QVariant) {
  fn setTabData(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setTabDataEiRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QTabBar10setTabDataEiRK8QVariant(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTabBar::QTabBar(QWidget * parent);
impl /*struct*/ QTabBar {
  pub fn new<T: QTabBar_new>(value: T) -> QTabBar {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTabBar_new {
  fn new(self) -> QTabBar;
}

  // proto:  void QTabBar::QTabBar(QWidget * parent);
impl<'a> /*trait*/ QTabBar_new for (&'a QWidget) {
  fn new(self) -> QTabBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBarC2EP7QWidget()};
    let ctysz: c_int = unsafe{QTabBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QTabBarC2EP7QWidget(arg0)};
    let rsthis = QTabBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QColor QTabBar::tabTextColor(int index);
impl /*struct*/ QTabBar {
  pub fn tabTextColor<RetType, T: QTabBar_tabTextColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabTextColor(self);
    // return 1;
  }
}

pub trait QTabBar_tabTextColor<RetType> {
  fn tabTextColor(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  QColor QTabBar::tabTextColor(int index);
impl<'a> /*trait*/ QTabBar_tabTextColor<QColor> for (i32) {
  fn tabTextColor(self , rsthis: & QTabBar) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12tabTextColorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QTabBar12tabTextColorEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabBar::~QTabBar();
impl /*struct*/ QTabBar {
  pub fn free<RetType, T: QTabBar_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTabBar_free<RetType> {
  fn free(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::~QTabBar();
impl<'a> /*trait*/ QTabBar_free<()> for () {
  fn free(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBarD2Ev()};
     unsafe {C_ZN7QTabBarD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTabBar::insertTab(int index, const QString & text);
impl<'a> /*trait*/ QTabBar_insertTab<i32> for (i32, &'a QString) {
  fn insertTab(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar9insertTabEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QTabBar9insertTabEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTabBar::addTab(const QString & text);
impl /*struct*/ QTabBar {
  pub fn addTab<RetType, T: QTabBar_addTab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addTab(self);
    // return 1;
  }
}

pub trait QTabBar_addTab<RetType> {
  fn addTab(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  int QTabBar::addTab(const QString & text);
impl<'a> /*trait*/ QTabBar_addTab<i32> for (&'a QString) {
  fn addTab(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar6addTabERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QTabBar6addTabERK7QString(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTabBar::addTab(const QIcon & icon, const QString & text);
impl<'a> /*trait*/ QTabBar_addTab<i32> for (&'a QIcon, &'a QString) {
  fn addTab(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar6addTabERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QTabBar6addTabERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTabBar::setTabToolTip(int index, const QString & tip);
impl /*struct*/ QTabBar {
  pub fn setTabToolTip<RetType, T: QTabBar_setTabToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabToolTip(self);
    // return 1;
  }
}

pub trait QTabBar_setTabToolTip<RetType> {
  fn setTabToolTip(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setTabToolTip(int index, const QString & tip);
impl<'a> /*trait*/ QTabBar_setTabToolTip<()> for (i32, &'a QString) {
  fn setTabToolTip(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar13setTabToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QTabBar13setTabToolTipEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTabBar::setTabTextColor(int index, const QColor & color);
impl /*struct*/ QTabBar {
  pub fn setTabTextColor<RetType, T: QTabBar_setTabTextColor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabTextColor(self);
    // return 1;
  }
}

pub trait QTabBar_setTabTextColor<RetType> {
  fn setTabTextColor(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setTabTextColor(int index, const QColor & color);
impl<'a> /*trait*/ QTabBar_setTabTextColor<()> for (i32, &'a QColor) {
  fn setTabTextColor(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar15setTabTextColorEiRK6QColor()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QTabBar15setTabTextColorEiRK6QColor(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTabBar::moveTab(int from, int to);
impl /*struct*/ QTabBar {
  pub fn moveTab<RetType, T: QTabBar_moveTab<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.moveTab(self);
    // return 1;
  }
}

pub trait QTabBar_moveTab<RetType> {
  fn moveTab(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::moveTab(int from, int to);
impl<'a> /*trait*/ QTabBar_moveTab<()> for (i32, i32) {
  fn moveTab(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar7moveTabEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN7QTabBar7moveTabEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QVariant QTabBar::tabData(int index);
impl /*struct*/ QTabBar {
  pub fn tabData<RetType, T: QTabBar_tabData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabData(self);
    // return 1;
  }
}

pub trait QTabBar_tabData<RetType> {
  fn tabData(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  QVariant QTabBar::tabData(int index);
impl<'a> /*trait*/ QTabBar_tabData<QVariant> for (i32) {
  fn tabData(self , rsthis: & QTabBar) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar7tabDataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QTabBar7tabDataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTabBar::drawBase();
impl /*struct*/ QTabBar {
  pub fn drawBase<RetType, T: QTabBar_drawBase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawBase(self);
    // return 1;
  }
}

pub trait QTabBar_drawBase<RetType> {
  fn drawBase(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  bool QTabBar::drawBase();
impl<'a> /*trait*/ QTabBar_drawBase<i8> for () {
  fn drawBase(self , rsthis: & QTabBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8drawBaseEv()};
    let mut ret = unsafe {C_ZNK7QTabBar8drawBaseEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QTabBar::currentIndex();
impl /*struct*/ QTabBar {
  pub fn currentIndex<RetType, T: QTabBar_currentIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QTabBar_currentIndex<RetType> {
  fn currentIndex(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  int QTabBar::currentIndex();
impl<'a> /*trait*/ QTabBar_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: & QTabBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar12currentIndexEv()};
    let mut ret = unsafe {C_ZNK7QTabBar12currentIndexEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTabBar::setDrawBase(bool drawTheBase);
impl /*struct*/ QTabBar {
  pub fn setDrawBase<RetType, T: QTabBar_setDrawBase<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDrawBase(self);
    // return 1;
  }
}

pub trait QTabBar_setDrawBase<RetType> {
  fn setDrawBase(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setDrawBase(bool drawTheBase);
impl<'a> /*trait*/ QTabBar_setDrawBase<()> for (i8) {
  fn setDrawBase(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar11setDrawBaseEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QTabBar11setDrawBaseEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabBar::setUsesScrollButtons(bool useButtons);
impl /*struct*/ QTabBar {
  pub fn setUsesScrollButtons<RetType, T: QTabBar_setUsesScrollButtons<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUsesScrollButtons(self);
    // return 1;
  }
}

pub trait QTabBar_setUsesScrollButtons<RetType> {
  fn setUsesScrollButtons(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setUsesScrollButtons(bool useButtons);
impl<'a> /*trait*/ QTabBar_setUsesScrollButtons<()> for (i8) {
  fn setUsesScrollButtons(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar20setUsesScrollButtonsEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN7QTabBar20setUsesScrollButtonsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QTabBar::sizeHint();
impl /*struct*/ QTabBar {
  pub fn sizeHint<RetType, T: QTabBar_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QTabBar_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  QSize QTabBar::sizeHint();
impl<'a> /*trait*/ QTabBar_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QTabBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QTabBar8sizeHintEv()};
    let mut ret = unsafe {C_ZNK7QTabBar8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabBar::setIconSize(const QSize & size);
impl /*struct*/ QTabBar {
  pub fn setIconSize<RetType, T: QTabBar_setIconSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QTabBar_setIconSize<RetType> {
  fn setIconSize(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setIconSize(const QSize & size);
impl<'a> /*trait*/ QTabBar_setIconSize<()> for (&'a QSize) {
  fn setIconSize(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QTabBar11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabBar::setTabText(int index, const QString & text);
impl /*struct*/ QTabBar {
  pub fn setTabText<RetType, T: QTabBar_setTabText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabText(self);
    // return 1;
  }
}

pub trait QTabBar_setTabText<RetType> {
  fn setTabText(self , rsthis: & QTabBar) -> RetType;
}

  // proto:  void QTabBar::setTabText(int index, const QString & text);
impl<'a> /*trait*/ QTabBar_setTabText<()> for (i32, &'a QString) {
  fn setTabText(self , rsthis: & QTabBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QTabBar10setTabTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN7QTabBar10setTabTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

#[derive(Default)] // for QTabBar_tabCloseRequested
pub struct QTabBar_tabCloseRequested_signal{poi:u64}
impl /* struct */ QTabBar {
  pub fn tabCloseRequested(&self) -> QTabBar_tabCloseRequested_signal {
     return QTabBar_tabCloseRequested_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTabBar_tabCloseRequested_signal {
  pub fn connect<T: QTabBar_tabCloseRequested_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTabBar_tabCloseRequested_signal_connect {
  fn connect(self, sigthis: QTabBar_tabCloseRequested_signal);
}

#[derive(Default)] // for QTabBar_tabBarDoubleClicked
pub struct QTabBar_tabBarDoubleClicked_signal{poi:u64}
impl /* struct */ QTabBar {
  pub fn tabBarDoubleClicked(&self) -> QTabBar_tabBarDoubleClicked_signal {
     return QTabBar_tabBarDoubleClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTabBar_tabBarDoubleClicked_signal {
  pub fn connect<T: QTabBar_tabBarDoubleClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTabBar_tabBarDoubleClicked_signal_connect {
  fn connect(self, sigthis: QTabBar_tabBarDoubleClicked_signal);
}

#[derive(Default)] // for QTabBar_tabMoved
pub struct QTabBar_tabMoved_signal{poi:u64}
impl /* struct */ QTabBar {
  pub fn tabMoved(&self) -> QTabBar_tabMoved_signal {
     return QTabBar_tabMoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTabBar_tabMoved_signal {
  pub fn connect<T: QTabBar_tabMoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTabBar_tabMoved_signal_connect {
  fn connect(self, sigthis: QTabBar_tabMoved_signal);
}

#[derive(Default)] // for QTabBar_tabBarClicked
pub struct QTabBar_tabBarClicked_signal{poi:u64}
impl /* struct */ QTabBar {
  pub fn tabBarClicked(&self) -> QTabBar_tabBarClicked_signal {
     return QTabBar_tabBarClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTabBar_tabBarClicked_signal {
  pub fn connect<T: QTabBar_tabBarClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTabBar_tabBarClicked_signal_connect {
  fn connect(self, sigthis: QTabBar_tabBarClicked_signal);
}

#[derive(Default)] // for QTabBar_currentChanged
pub struct QTabBar_currentChanged_signal{poi:u64}
impl /* struct */ QTabBar {
  pub fn currentChanged(&self) -> QTabBar_currentChanged_signal {
     return QTabBar_currentChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTabBar_currentChanged_signal {
  pub fn connect<T: QTabBar_currentChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTabBar_currentChanged_signal_connect {
  fn connect(self, sigthis: QTabBar_currentChanged_signal);
}

// tabBarClicked(int)
extern fn QTabBar_tabBarClicked_signal_connect_cb_0(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QTabBar_tabBarClicked_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTabBar_tabBarClicked_signal_connect for fn(i32) {
  fn connect(self, sigthis: QTabBar_tabBarClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabBar_tabBarClicked_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTabBar_SlotProxy_connect__ZN7QTabBar13tabBarClickedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTabBar_tabBarClicked_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QTabBar_tabBarClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabBar_tabBarClicked_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTabBar_SlotProxy_connect__ZN7QTabBar13tabBarClickedEi(arg0, arg1, arg2)};
  }
}
// tabBarDoubleClicked(int)
extern fn QTabBar_tabBarDoubleClicked_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QTabBar_tabBarDoubleClicked_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTabBar_tabBarDoubleClicked_signal_connect for fn(i32) {
  fn connect(self, sigthis: QTabBar_tabBarDoubleClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabBar_tabBarDoubleClicked_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTabBar_SlotProxy_connect__ZN7QTabBar19tabBarDoubleClickedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTabBar_tabBarDoubleClicked_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QTabBar_tabBarDoubleClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabBar_tabBarDoubleClicked_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTabBar_SlotProxy_connect__ZN7QTabBar19tabBarDoubleClickedEi(arg0, arg1, arg2)};
  }
}
// tabCloseRequested(int)
extern fn QTabBar_tabCloseRequested_signal_connect_cb_2(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QTabBar_tabCloseRequested_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTabBar_tabCloseRequested_signal_connect for fn(i32) {
  fn connect(self, sigthis: QTabBar_tabCloseRequested_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabBar_tabCloseRequested_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTabBar_SlotProxy_connect__ZN7QTabBar17tabCloseRequestedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTabBar_tabCloseRequested_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QTabBar_tabCloseRequested_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabBar_tabCloseRequested_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTabBar_SlotProxy_connect__ZN7QTabBar17tabCloseRequestedEi(arg0, arg1, arg2)};
  }
}
// tabMoved(int, int)
extern fn QTabBar_tabMoved_signal_connect_cb_3(rsfptr:fn(i32, i32), arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  rsfptr(rsarg0,rsarg1);
}
extern fn QTabBar_tabMoved_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn(i32, i32)>, arg0: c_int, arg1: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  // rsfptr(rsarg0,rsarg1);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1)};
}
impl /* trait */ QTabBar_tabMoved_signal_connect for fn(i32, i32) {
  fn connect(self, sigthis: QTabBar_tabMoved_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabBar_tabMoved_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTabBar_SlotProxy_connect__ZN7QTabBar8tabMovedEii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTabBar_tabMoved_signal_connect for Box<Fn(i32, i32)> {
  fn connect(self, sigthis: QTabBar_tabMoved_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabBar_tabMoved_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTabBar_SlotProxy_connect__ZN7QTabBar8tabMovedEii(arg0, arg1, arg2)};
  }
}
// currentChanged(int)
extern fn QTabBar_currentChanged_signal_connect_cb_4(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QTabBar_currentChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTabBar_currentChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QTabBar_currentChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabBar_currentChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTabBar_SlotProxy_connect__ZN7QTabBar14currentChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTabBar_currentChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QTabBar_currentChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTabBar_currentChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTabBar_SlotProxy_connect__ZN7QTabBar14currentChangedEi(arg0, arg1, arg2)};
  }
}
// <= body block end

