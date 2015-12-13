// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qicon::QIcon;
use super::qobject::QObject;
use super::qmenu::QMenu;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QSystemTrayIcon::FreeQSystemTrayIcon();
  fn _ZN15QSystemTrayIconD0Ev() -> i32;
  // proto: void QSystemTrayIcon::setVisible(bool visible);
  fn _ZN15QSystemTrayIcon10setVisibleEb(arg0: int8_t) -> i32;
  // proto: QString QSystemTrayIcon::toolTip();
  fn _ZNK15QSystemTrayIcon7toolTipEv() -> i32;
  // proto: void QSystemTrayIcon::NewQSystemTrayIcon(const QIcon & icon, QObject * parent);
  fn _ZN15QSystemTrayIconC1ERK5QIconP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QSystemTrayIcon::hide();
  fn _ZN15QSystemTrayIcon4hideEv() -> i32;
  // proto: const QMetaObject * QSystemTrayIcon::metaObject();
  fn _ZNK15QSystemTrayIcon10metaObjectEv() -> i32;
  // proto: void QSystemTrayIcon::NewQSystemTrayIcon(const QSystemTrayIcon & );
  fn _ZN15QSystemTrayIconC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QSystemTrayIcon::setIcon(const QIcon & icon);
  fn _ZN15QSystemTrayIcon7setIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: bool QSystemTrayIcon::isVisible();
  fn _ZNK15QSystemTrayIcon9isVisibleEv() -> i32;
  // proto: void QSystemTrayIcon::NewQSystemTrayIcon(QObject * parent);
  fn _ZN15QSystemTrayIconC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QSystemTrayIcon::show();
  fn _ZN15QSystemTrayIcon4showEv() -> i32;
  // proto: bool QSystemTrayIcon::supportsMessages();
  fn _ZN15QSystemTrayIcon16supportsMessagesEv() -> i32;
  // proto: void QSystemTrayIcon::messageClicked();
  fn _ZN15QSystemTrayIcon14messageClickedEv() -> i32;
  // proto: void QSystemTrayIcon::setContextMenu(QMenu * menu);
  fn _ZN15QSystemTrayIcon14setContextMenuEP5QMenu(arg0: *mut c_void) -> i32;
  // proto: QRect QSystemTrayIcon::geometry();
  fn _ZNK15QSystemTrayIcon8geometryEv() -> i32;
  // proto: void QSystemTrayIcon::setToolTip(const QString & tip);
  fn _ZN15QSystemTrayIcon10setToolTipERK7QString(arg0: *const c_void) -> i32;
  // proto: QIcon QSystemTrayIcon::icon();
  fn _ZNK15QSystemTrayIcon4iconEv() -> i32;
  // proto: QMenu * QSystemTrayIcon::contextMenu();
  fn _ZNK15QSystemTrayIcon11contextMenuEv() -> i32;
  // proto: bool QSystemTrayIcon::isSystemTrayAvailable();
  fn _ZN15QSystemTrayIcon21isSystemTrayAvailableEv() -> i32;
}

// body block begin
// class sizeof(QSystemTrayIcon)=1
pub struct QSystemTrayIcon {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSystemTrayIcon {
  pub fn FreeQSystemTrayIcon<T: QSystemTrayIcon_FreeQSystemTrayIcon>(&mut self, value: T) -> i32 {
    value.FreeQSystemTrayIcon(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_FreeQSystemTrayIcon {
  fn FreeQSystemTrayIcon(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: void QSystemTrayIcon::FreeQSystemTrayIcon();
impl<'a> /*trait*/ QSystemTrayIcon_FreeQSystemTrayIcon for () {
  fn FreeQSystemTrayIcon(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIconD0Ev()};
    unsafe {_ZN15QSystemTrayIconD0Ev()};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn setVisible<T: QSystemTrayIcon_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_setVisible {
  fn setVisible(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: void QSystemTrayIcon::setVisible(bool visible);
impl<'a> /*trait*/ QSystemTrayIcon_setVisible for (i8) {
  fn setVisible(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN15QSystemTrayIcon10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn toolTip<T: QSystemTrayIcon_toolTip>(&mut self, value: T) -> i32 {
    value.toolTip(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_toolTip {
  fn toolTip(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: QString QSystemTrayIcon::toolTip();
impl<'a> /*trait*/ QSystemTrayIcon_toolTip for () {
  fn toolTip(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon7toolTipEv()};
    unsafe {_ZNK15QSystemTrayIcon7toolTipEv()};
    return 1;
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
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN15QSystemTrayIconC1ERK5QIconP7QObject(qthis, arg0, arg1)};
    let rsthis = QSystemTrayIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn hide<T: QSystemTrayIcon_hide>(&mut self, value: T) -> i32 {
    value.hide(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_hide {
  fn hide(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: void QSystemTrayIcon::hide();
impl<'a> /*trait*/ QSystemTrayIcon_hide for () {
  fn hide(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon4hideEv()};
    unsafe {_ZN15QSystemTrayIcon4hideEv()};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn metaObject<T: QSystemTrayIcon_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_metaObject {
  fn metaObject(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: const QMetaObject * QSystemTrayIcon::metaObject();
impl<'a> /*trait*/ QSystemTrayIcon_metaObject for () {
  fn metaObject(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon10metaObjectEv()};
    unsafe {_ZNK15QSystemTrayIcon10metaObjectEv()};
    return 1;
  }
}

// proto: void QSystemTrayIcon::NewQSystemTrayIcon(const QSystemTrayIcon & );
impl<'a> /*trait*/ QSystemTrayIcon_NewQSystemTrayIcon for (&'a  QSystemTrayIcon) {
  fn NewQSystemTrayIcon(self) -> QSystemTrayIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIconC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QSystemTrayIconC1ERKS_(qthis, arg0)};
    let rsthis = QSystemTrayIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn setIcon<T: QSystemTrayIcon_setIcon>(&mut self, value: T) -> i32 {
    value.setIcon(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_setIcon {
  fn setIcon(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: void QSystemTrayIcon::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QSystemTrayIcon_setIcon for (&'a  QIcon) {
  fn setIcon(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QSystemTrayIcon7setIconERK5QIcon(arg0)};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn isVisible<T: QSystemTrayIcon_isVisible>(&mut self, value: T) -> i32 {
    value.isVisible(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_isVisible {
  fn isVisible(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: bool QSystemTrayIcon::isVisible();
impl<'a> /*trait*/ QSystemTrayIcon_isVisible for () {
  fn isVisible(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon9isVisibleEv()};
    unsafe {_ZNK15QSystemTrayIcon9isVisibleEv()};
    return 1;
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
  pub fn show<T: QSystemTrayIcon_show>(&mut self, value: T) -> i32 {
    value.show(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_show {
  fn show(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: void QSystemTrayIcon::show();
impl<'a> /*trait*/ QSystemTrayIcon_show for () {
  fn show(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon4showEv()};
    unsafe {_ZN15QSystemTrayIcon4showEv()};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn supportsMessages<T: QSystemTrayIcon_supportsMessages>(&mut self, value: T) -> i32 {
    value.supportsMessages(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_supportsMessages {
  fn supportsMessages(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: bool QSystemTrayIcon::supportsMessages();
impl<'a> /*trait*/ QSystemTrayIcon_supportsMessages for () {
  fn supportsMessages(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon16supportsMessagesEv()};
    unsafe {_ZN15QSystemTrayIcon16supportsMessagesEv()};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn messageClicked<T: QSystemTrayIcon_messageClicked>(&mut self, value: T) -> i32 {
    value.messageClicked(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_messageClicked {
  fn messageClicked(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: void QSystemTrayIcon::messageClicked();
impl<'a> /*trait*/ QSystemTrayIcon_messageClicked for () {
  fn messageClicked(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon14messageClickedEv()};
    unsafe {_ZN15QSystemTrayIcon14messageClickedEv()};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn setContextMenu<T: QSystemTrayIcon_setContextMenu>(&mut self, value: T) -> i32 {
    value.setContextMenu(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_setContextMenu {
  fn setContextMenu(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: void QSystemTrayIcon::setContextMenu(QMenu * menu);
impl<'a> /*trait*/ QSystemTrayIcon_setContextMenu for (&'a mut QMenu) {
  fn setContextMenu(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon14setContextMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QSystemTrayIcon14setContextMenuEP5QMenu(arg0)};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn geometry<T: QSystemTrayIcon_geometry>(&mut self, value: T) -> i32 {
    value.geometry(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_geometry {
  fn geometry(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: QRect QSystemTrayIcon::geometry();
impl<'a> /*trait*/ QSystemTrayIcon_geometry for () {
  fn geometry(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon8geometryEv()};
    unsafe {_ZNK15QSystemTrayIcon8geometryEv()};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn setToolTip<T: QSystemTrayIcon_setToolTip>(&mut self, value: T) -> i32 {
    value.setToolTip(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_setToolTip {
  fn setToolTip(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: void QSystemTrayIcon::setToolTip(const QString & tip);
impl<'a> /*trait*/ QSystemTrayIcon_setToolTip for (&'a  QString) {
  fn setToolTip(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN15QSystemTrayIcon10setToolTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn icon<T: QSystemTrayIcon_icon>(&mut self, value: T) -> i32 {
    value.icon(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_icon {
  fn icon(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: QIcon QSystemTrayIcon::icon();
impl<'a> /*trait*/ QSystemTrayIcon_icon for () {
  fn icon(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon4iconEv()};
    unsafe {_ZNK15QSystemTrayIcon4iconEv()};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn contextMenu<T: QSystemTrayIcon_contextMenu>(&mut self, value: T) -> i32 {
    value.contextMenu(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_contextMenu {
  fn contextMenu(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: QMenu * QSystemTrayIcon::contextMenu();
impl<'a> /*trait*/ QSystemTrayIcon_contextMenu for () {
  fn contextMenu(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon11contextMenuEv()};
    unsafe {_ZNK15QSystemTrayIcon11contextMenuEv()};
    return 1;
  }
}

impl /*struct*/ QSystemTrayIcon {
  pub fn isSystemTrayAvailable<T: QSystemTrayIcon_isSystemTrayAvailable>(&mut self, value: T) -> i32 {
    value.isSystemTrayAvailable(self);
    return 1;
  }
}

pub trait QSystemTrayIcon_isSystemTrayAvailable {
  fn isSystemTrayAvailable(self, this: &mut QSystemTrayIcon) -> i32;
}

// proto: bool QSystemTrayIcon::isSystemTrayAvailable();
impl<'a> /*trait*/ QSystemTrayIcon_isSystemTrayAvailable for () {
  fn isSystemTrayAvailable(self, this: &mut QSystemTrayIcon) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon21isSystemTrayAvailableEv()};
    unsafe {_ZN15QSystemTrayIcon21isSystemTrayAvailableEv()};
    return 1;
  }
}

