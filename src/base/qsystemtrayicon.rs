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
use super::qobject::QObject;
use super::qmenu::QMenu;
use super::qrect::QRect;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSystemTrayIcon::FreeQSystemTrayIcon();
  fn _ZN15QSystemTrayIconD0Ev(qthis: *mut c_void) ;
  // proto:  void QSystemTrayIcon::setVisible(bool visible);
  fn _ZN15QSystemTrayIcon10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QSystemTrayIcon::toolTip();
  fn _ZNK15QSystemTrayIcon7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSystemTrayIcon::NewQSystemTrayIcon(const QIcon & icon, QObject * parent);
  fn _ZN15QSystemTrayIconC1ERK5QIconP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QSystemTrayIcon::hide();
  fn _ZN15QSystemTrayIcon4hideEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QSystemTrayIcon::metaObject();
  fn _ZNK15QSystemTrayIcon10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QSystemTrayIcon::NewQSystemTrayIcon(const QSystemTrayIcon & );
  fn _ZN15QSystemTrayIconC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSystemTrayIcon::setIcon(const QIcon & icon);
  fn _ZN15QSystemTrayIcon7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QSystemTrayIcon::isVisible();
  fn _ZNK15QSystemTrayIcon9isVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QSystemTrayIcon::NewQSystemTrayIcon(QObject * parent);
  fn _ZN15QSystemTrayIconC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSystemTrayIcon::show();
  fn _ZN15QSystemTrayIcon4showEv(qthis: *mut c_void) ;
  // proto: static bool QSystemTrayIcon::supportsMessages();
  fn _ZN15QSystemTrayIcon16supportsMessagesEv() -> int8_t;
  // proto:  void QSystemTrayIcon::messageClicked();
  fn _ZN15QSystemTrayIcon14messageClickedEv(qthis: *mut c_void) ;
  // proto:  void QSystemTrayIcon::setContextMenu(QMenu * menu);
  fn _ZN15QSystemTrayIcon14setContextMenuEP5QMenu(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QRect QSystemTrayIcon::geometry();
  fn _ZNK15QSystemTrayIcon8geometryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSystemTrayIcon::setToolTip(const QString & tip);
  fn _ZN15QSystemTrayIcon10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QIcon QSystemTrayIcon::icon();
  fn _ZNK15QSystemTrayIcon4iconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QMenu * QSystemTrayIcon::contextMenu();
  fn _ZNK15QSystemTrayIcon11contextMenuEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static bool QSystemTrayIcon::isSystemTrayAvailable();
  fn _ZN15QSystemTrayIcon21isSystemTrayAvailableEv() -> int8_t;
}

// body block begin
// class sizeof(QSystemTrayIcon)=1
pub struct QSystemTrayIcon {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSystemTrayIcon {
  pub fn FreeQSystemTrayIcon<RetType, T: QSystemTrayIcon_FreeQSystemTrayIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQSystemTrayIcon(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_FreeQSystemTrayIcon<RetType> {
  fn FreeQSystemTrayIcon(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  void QSystemTrayIcon::FreeQSystemTrayIcon();
impl<'a> /*trait*/ QSystemTrayIcon_FreeQSystemTrayIcon<()> for () {
  fn FreeQSystemTrayIcon(self, rsthis: &mut QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIconD0Ev()};
     unsafe {_ZN15QSystemTrayIconD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn setVisible<RetType, T: QSystemTrayIcon_setVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.setVisible(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_setVisible<RetType> {
  fn setVisible(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  void QSystemTrayIcon::setVisible(bool visible);
impl<'a> /*trait*/ QSystemTrayIcon_setVisible<()> for (i8) {
  fn setVisible(self, rsthis: &mut QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN15QSystemTrayIcon10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn toolTip<RetType, T: QSystemTrayIcon_toolTip<RetType>>(&mut self, value: T) -> RetType {
    return value.toolTip(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_toolTip<RetType> {
  fn toolTip(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  QString QSystemTrayIcon::toolTip();
impl<'a> /*trait*/ QSystemTrayIcon_toolTip<QString> for () {
  fn toolTip(self, rsthis: &mut QSystemTrayIcon) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon7toolTipEv()};
    let mut ret = unsafe {_ZNK15QSystemTrayIcon7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn NewQSystemTrayIcon<T: QSystemTrayIcon_NewQSystemTrayIcon>(value: T) -> QSystemTrayIcon {
    let rsthis = value.NewQSystemTrayIcon();
    return rsthis;
    // return 1;
  }
}

pub trait QSystemTrayIcon_NewQSystemTrayIcon {
  fn NewQSystemTrayIcon(self) -> QSystemTrayIcon;
}

// proto: void QSystemTrayIcon::NewQSystemTrayIcon(const QIcon & icon, QObject * parent);
impl<'a> /*trait*/ QSystemTrayIcon_NewQSystemTrayIcon for (&'a  QIcon, &'a mut QObject) {
  fn NewQSystemTrayIcon(self) -> QSystemTrayIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIconC1ERK5QIconP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QSystemTrayIconC1ERK5QIconP7QObject(qthis, arg0, arg1)};
    let rsthis = QSystemTrayIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn hide<RetType, T: QSystemTrayIcon_hide<RetType>>(&mut self, value: T) -> RetType {
    return value.hide(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_hide<RetType> {
  fn hide(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  void QSystemTrayIcon::hide();
impl<'a> /*trait*/ QSystemTrayIcon_hide<()> for () {
  fn hide(self, rsthis: &mut QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon4hideEv()};
     unsafe {_ZN15QSystemTrayIcon4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn metaObject<RetType, T: QSystemTrayIcon_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  const QMetaObject * QSystemTrayIcon::metaObject();
impl<'a> /*trait*/ QSystemTrayIcon_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon10metaObjectEv()};
     unsafe {_ZNK15QSystemTrayIcon10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QSystemTrayIcon::NewQSystemTrayIcon(const QSystemTrayIcon & );
impl<'a> /*trait*/ QSystemTrayIcon_NewQSystemTrayIcon for (&'a  QSystemTrayIcon) {
  fn NewQSystemTrayIcon(self) -> QSystemTrayIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIconC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QSystemTrayIconC1ERKS_(qthis, arg0)};
    let rsthis = QSystemTrayIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn setIcon<RetType, T: QSystemTrayIcon_setIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.setIcon(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_setIcon<RetType> {
  fn setIcon(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  void QSystemTrayIcon::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QSystemTrayIcon_setIcon<()> for (&'a  QIcon) {
  fn setIcon(self, rsthis: &mut QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QSystemTrayIcon7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn isVisible<RetType, T: QSystemTrayIcon_isVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.isVisible(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_isVisible<RetType> {
  fn isVisible(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  bool QSystemTrayIcon::isVisible();
impl<'a> /*trait*/ QSystemTrayIcon_isVisible<i8> for () {
  fn isVisible(self, rsthis: &mut QSystemTrayIcon) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon9isVisibleEv()};
    let mut ret = unsafe {_ZNK15QSystemTrayIcon9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QSystemTrayIcon::NewQSystemTrayIcon(QObject * parent);
impl<'a> /*trait*/ QSystemTrayIcon_NewQSystemTrayIcon for (&'a mut QObject) {
  fn NewQSystemTrayIcon(self) -> QSystemTrayIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIconC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QSystemTrayIconC1EP7QObject(qthis, arg0)};
    let rsthis = QSystemTrayIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn show<RetType, T: QSystemTrayIcon_show<RetType>>(&mut self, value: T) -> RetType {
    return value.show(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_show<RetType> {
  fn show(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  void QSystemTrayIcon::show();
impl<'a> /*trait*/ QSystemTrayIcon_show<()> for () {
  fn show(self, rsthis: &mut QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon4showEv()};
     unsafe {_ZN15QSystemTrayIcon4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn supportsMessages<RetType, T: QSystemTrayIcon_supportsMessages<RetType>>(&mut self, value: T) -> RetType {
    return value.supportsMessages(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_supportsMessages<RetType> {
  fn supportsMessages(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto: static bool QSystemTrayIcon::supportsMessages();
impl<'a> /*trait*/ QSystemTrayIcon_supportsMessages<i8> for () {
  fn supportsMessages(self, rsthis: &mut QSystemTrayIcon) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon16supportsMessagesEv()};
    let mut ret = unsafe {_ZN15QSystemTrayIcon16supportsMessagesEv()};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn messageClicked<RetType, T: QSystemTrayIcon_messageClicked<RetType>>(&mut self, value: T) -> RetType {
    return value.messageClicked(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_messageClicked<RetType> {
  fn messageClicked(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  void QSystemTrayIcon::messageClicked();
impl<'a> /*trait*/ QSystemTrayIcon_messageClicked<()> for () {
  fn messageClicked(self, rsthis: &mut QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon14messageClickedEv()};
     unsafe {_ZN15QSystemTrayIcon14messageClickedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn setContextMenu<RetType, T: QSystemTrayIcon_setContextMenu<RetType>>(&mut self, value: T) -> RetType {
    return value.setContextMenu(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_setContextMenu<RetType> {
  fn setContextMenu(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  void QSystemTrayIcon::setContextMenu(QMenu * menu);
impl<'a> /*trait*/ QSystemTrayIcon_setContextMenu<()> for (&'a mut QMenu) {
  fn setContextMenu(self, rsthis: &mut QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon14setContextMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QSystemTrayIcon14setContextMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn geometry<RetType, T: QSystemTrayIcon_geometry<RetType>>(&mut self, value: T) -> RetType {
    return value.geometry(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_geometry<RetType> {
  fn geometry(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  QRect QSystemTrayIcon::geometry();
impl<'a> /*trait*/ QSystemTrayIcon_geometry<QRect> for () {
  fn geometry(self, rsthis: &mut QSystemTrayIcon) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon8geometryEv()};
    let mut ret = unsafe {_ZNK15QSystemTrayIcon8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn setToolTip<RetType, T: QSystemTrayIcon_setToolTip<RetType>>(&mut self, value: T) -> RetType {
    return value.setToolTip(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_setToolTip<RetType> {
  fn setToolTip(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  void QSystemTrayIcon::setToolTip(const QString & tip);
impl<'a> /*trait*/ QSystemTrayIcon_setToolTip<()> for (&'a  QString) {
  fn setToolTip(self, rsthis: &mut QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QSystemTrayIcon10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn icon<RetType, T: QSystemTrayIcon_icon<RetType>>(&mut self, value: T) -> RetType {
    return value.icon(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_icon<RetType> {
  fn icon(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  QIcon QSystemTrayIcon::icon();
impl<'a> /*trait*/ QSystemTrayIcon_icon<QIcon> for () {
  fn icon(self, rsthis: &mut QSystemTrayIcon) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon4iconEv()};
    let mut ret = unsafe {_ZNK15QSystemTrayIcon4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn contextMenu<RetType, T: QSystemTrayIcon_contextMenu<RetType>>(&mut self, value: T) -> RetType {
    return value.contextMenu(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_contextMenu<RetType> {
  fn contextMenu(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto:  QMenu * QSystemTrayIcon::contextMenu();
impl<'a> /*trait*/ QSystemTrayIcon_contextMenu<QMenu> for () {
  fn contextMenu(self, rsthis: &mut QSystemTrayIcon) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon11contextMenuEv()};
    let mut ret = unsafe {_ZNK15QSystemTrayIcon11contextMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn isSystemTrayAvailable<RetType, T: QSystemTrayIcon_isSystemTrayAvailable<RetType>>(&mut self, value: T) -> RetType {
    return value.isSystemTrayAvailable(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_isSystemTrayAvailable<RetType> {
  fn isSystemTrayAvailable(self, rsthis: &mut QSystemTrayIcon) -> RetType;
}

// proto: static bool QSystemTrayIcon::isSystemTrayAvailable();
impl<'a> /*trait*/ QSystemTrayIcon_isSystemTrayAvailable<i8> for () {
  fn isSystemTrayAvailable(self, rsthis: &mut QSystemTrayIcon) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon21isSystemTrayAvailableEv()};
    let mut ret = unsafe {_ZN15QSystemTrayIcon21isSystemTrayAvailableEv()};
    return ret as i8;
    // return 1;
  }
}

