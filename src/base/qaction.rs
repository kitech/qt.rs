// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qobject::QObject;
use super::qfont::QFont;
use super::qvariant::QVariant;
use super::qicon::QIcon;
use super::qwidget::QWidget;
use super::qkeysequence::QKeySequence;
use super::qactiongroup::QActionGroup;
use super::qmenu::QMenu;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: QList<QWidget *> QAction::associatedWidgets();
  fn _ZNK7QAction17associatedWidgetsEv() -> i32;
  // proto: void QAction::setAutoRepeat(bool );
  fn _ZN7QAction13setAutoRepeatEb(arg0: int8_t) -> i32;
  // proto: QString QAction::whatsThis();
  fn _ZNK7QAction9whatsThisEv() -> i32;
  // proto: void QAction::NewQAction(const QString & text, QObject * parent);
  fn _ZN7QActionC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QAction::FreeQAction();
  fn _ZN7QActionD0Ev() -> i32;
  // proto: bool QAction::isVisible();
  fn _ZNK7QAction9isVisibleEv() -> i32;
  // proto: void QAction::setFont(const QFont & font);
  fn _ZN7QAction7setFontERK5QFont(arg0: *const c_void) -> i32;
  // proto: void QAction::setData(const QVariant & var);
  fn _ZN7QAction7setDataERK8QVariant(arg0: *const c_void) -> i32;
  // proto: void QAction::setIcon(const QIcon & icon);
  fn _ZN7QAction7setIconERK5QIcon(arg0: *const c_void) -> i32;
  // proto: void QAction::NewQAction(QObject * parent);
  fn _ZN7QActionC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QAction::metaObject();
  fn _ZNK7QAction10metaObjectEv() -> i32;
  // proto: void QAction::triggered(bool checked);
  fn _ZN7QAction9triggeredEb(arg0: int8_t) -> i32;
  // proto: void QAction::toggled(bool );
  fn _ZN7QAction7toggledEb(arg0: int8_t) -> i32;
  // proto: void QAction::setText(const QString & text);
  fn _ZN7QAction7setTextERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QAction::showStatusText(QWidget * widget);
  fn _ZN7QAction14showStatusTextEP7QWidget(arg0: *mut c_void) -> i32;
  // proto: QString QAction::iconText();
  fn _ZNK7QAction8iconTextEv() -> i32;
  // proto: void QAction::setIconVisibleInMenu(bool visible);
  fn _ZN7QAction20setIconVisibleInMenuEb(arg0: int8_t) -> i32;
  // proto: QString QAction::statusTip();
  fn _ZNK7QAction9statusTipEv() -> i32;
  // proto: bool QAction::isCheckable();
  fn _ZNK7QAction11isCheckableEv() -> i32;
  // proto: void QAction::setWhatsThis(const QString & what);
  fn _ZN7QAction12setWhatsThisERK7QString(arg0: *const c_void) -> i32;
  // proto: QMenu * QAction::menu();
  fn _ZNK7QAction4menuEv() -> i32;
  // proto: void QAction::trigger();
  fn _ZN7QAction7triggerEv() -> i32;
  // proto: void QAction::changed();
  fn _ZN7QAction7changedEv() -> i32;
  // proto: QFont QAction::font();
  fn _ZNK7QAction4fontEv() -> i32;
  // proto: void QAction::NewQAction(const QIcon & icon, const QString & text, QObject * parent);
  fn _ZN7QActionC1ERK5QIconRK7QStringP7QObject(qthis: *mut c_void, arg0: *const c_void, arg1: *const c_void, arg2: *mut c_void) -> i32;
  // proto: void QAction::setChecked(bool );
  fn _ZN7QAction10setCheckedEb(arg0: int8_t) -> i32;
  // proto: void QAction::setDisabled(bool b);
  fn _ZN7QAction11setDisabledEb(arg0: int8_t) -> i32;
  // proto: void QAction::setShortcut(const QKeySequence & shortcut);
  fn _ZN7QAction11setShortcutERK12QKeySequence(arg0: *const c_void) -> i32;
  // proto: void QAction::toggle();
  fn _ZN7QAction6toggleEv() -> i32;
  // proto: QKeySequence QAction::shortcut();
  fn _ZNK7QAction8shortcutEv() -> i32;
  // proto: QIcon QAction::icon();
  fn _ZNK7QAction4iconEv() -> i32;
  // proto: void QAction::setActionGroup(QActionGroup * group);
  fn _ZN7QAction14setActionGroupEP12QActionGroup(arg0: *mut c_void) -> i32;
  // proto: void QAction::setMenu(QMenu * menu);
  fn _ZN7QAction7setMenuEP5QMenu(arg0: *mut c_void) -> i32;
  // proto: QList<QKeySequence> QAction::shortcuts();
  fn _ZNK7QAction9shortcutsEv() -> i32;
  // proto: void QAction::setCheckable(bool );
  fn _ZN7QAction12setCheckableEb(arg0: int8_t) -> i32;
  // proto: QString QAction::toolTip();
  fn _ZNK7QAction7toolTipEv() -> i32;
  // proto: QWidget * QAction::parentWidget();
  fn _ZNK7QAction12parentWidgetEv() -> i32;
  // proto: void QAction::setSeparator(bool b);
  fn _ZN7QAction12setSeparatorEb(arg0: int8_t) -> i32;
  // proto: QList<QGraphicsWidget *> QAction::associatedGraphicsWidgets();
  fn _ZNK7QAction25associatedGraphicsWidgetsEv() -> i32;
  // proto: void QAction::setVisible(bool );
  fn _ZN7QAction10setVisibleEb(arg0: int8_t) -> i32;
  // proto: bool QAction::isSeparator();
  fn _ZNK7QAction11isSeparatorEv() -> i32;
  // proto: void QAction::setIconText(const QString & text);
  fn _ZN7QAction11setIconTextERK7QString(arg0: *const c_void) -> i32;
  // proto: QActionGroup * QAction::actionGroup();
  fn _ZNK7QAction11actionGroupEv() -> i32;
  // proto: void QAction::setStatusTip(const QString & statusTip);
  fn _ZN7QAction12setStatusTipERK7QString(arg0: *const c_void) -> i32;
  // proto: void QAction::hovered();
  fn _ZN7QAction7hoveredEv() -> i32;
  // proto: QVariant QAction::data();
  fn _ZNK7QAction4dataEv() -> i32;
  // proto: void QAction::NewQAction(const QAction & );
  fn _ZN7QActionC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: bool QAction::isChecked();
  fn _ZNK7QAction9isCheckedEv() -> i32;
  // proto: bool QAction::autoRepeat();
  fn _ZNK7QAction10autoRepeatEv() -> i32;
  // proto: bool QAction::isEnabled();
  fn _ZNK7QAction9isEnabledEv() -> i32;
  // proto: QString QAction::text();
  fn _ZNK7QAction4textEv() -> i32;
  // proto: void QAction::hover();
  fn _ZN7QAction5hoverEv() -> i32;
  // proto: bool QAction::isIconVisibleInMenu();
  fn _ZNK7QAction19isIconVisibleInMenuEv() -> i32;
  // proto: void QAction::setToolTip(const QString & tip);
  fn _ZN7QAction10setToolTipERK7QString(arg0: *const c_void) -> i32;
  // proto: void QAction::setEnabled(bool );
  fn _ZN7QAction10setEnabledEb(arg0: int8_t) -> i32;
}

// body block begin
// class sizeof(QAction)=1
pub struct QAction {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAction {
  pub fn associatedWidgets<T: QAction_associatedWidgets>(&mut self, value: T) -> i32 {
    value.associatedWidgets(self);
    return 1;
  }
}

pub trait QAction_associatedWidgets {
  fn associatedWidgets(self, this: &mut QAction) -> i32;
}

// proto: QList<QWidget *> QAction::associatedWidgets();
impl<'a> /*trait*/ QAction_associatedWidgets for () {
  fn associatedWidgets(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction17associatedWidgetsEv()};
    unsafe {_ZNK7QAction17associatedWidgetsEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setAutoRepeat<T: QAction_setAutoRepeat>(&mut self, value: T) -> i32 {
    value.setAutoRepeat(self);
    return 1;
  }
}

pub trait QAction_setAutoRepeat {
  fn setAutoRepeat(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setAutoRepeat(bool );
impl<'a> /*trait*/ QAction_setAutoRepeat for (i8) {
  fn setAutoRepeat(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction13setAutoRepeatEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QAction13setAutoRepeatEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn whatsThis<T: QAction_whatsThis>(&mut self, value: T) -> i32 {
    value.whatsThis(self);
    return 1;
  }
}

pub trait QAction_whatsThis {
  fn whatsThis(self, this: &mut QAction) -> i32;
}

// proto: QString QAction::whatsThis();
impl<'a> /*trait*/ QAction_whatsThis for () {
  fn whatsThis(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9whatsThisEv()};
    unsafe {_ZNK7QAction9whatsThisEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn NewQAction<T: QAction_NewQAction>(value: T) -> QAction {
    let rsthis = value.NewQAction();
    return rsthis;
    // return 1;
  }
}

pub trait QAction_NewQAction {
  fn NewQAction(self) -> QAction;
}

// proto: void QAction::NewQAction(const QString & text, QObject * parent);
impl<'a> /*trait*/ QAction_NewQAction for (&'a  QString, &'a mut QObject) {
  fn NewQAction(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QAction{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn FreeQAction<T: QAction_FreeQAction>(&mut self, value: T) -> i32 {
    value.FreeQAction(self);
    return 1;
  }
}

pub trait QAction_FreeQAction {
  fn FreeQAction(self, this: &mut QAction) -> i32;
}

// proto: void QAction::FreeQAction();
impl<'a> /*trait*/ QAction_FreeQAction for () {
  fn FreeQAction(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionD0Ev()};
    unsafe {_ZN7QActionD0Ev()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isVisible<T: QAction_isVisible>(&mut self, value: T) -> i32 {
    value.isVisible(self);
    return 1;
  }
}

pub trait QAction_isVisible {
  fn isVisible(self, this: &mut QAction) -> i32;
}

// proto: bool QAction::isVisible();
impl<'a> /*trait*/ QAction_isVisible for () {
  fn isVisible(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9isVisibleEv()};
    unsafe {_ZNK7QAction9isVisibleEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setFont<T: QAction_setFont>(&mut self, value: T) -> i32 {
    value.setFont(self);
    return 1;
  }
}

pub trait QAction_setFont {
  fn setFont(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setFont(const QFont & font);
impl<'a> /*trait*/ QAction_setFont for (&'a  QFont) {
  fn setFont(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QAction7setFontERK5QFont(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setData<T: QAction_setData>(&mut self, value: T) -> i32 {
    value.setData(self);
    return 1;
  }
}

pub trait QAction_setData {
  fn setData(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setData(const QVariant & var);
impl<'a> /*trait*/ QAction_setData for (&'a  QVariant) {
  fn setData(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setDataERK8QVariant()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QAction7setDataERK8QVariant(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setIcon<T: QAction_setIcon>(&mut self, value: T) -> i32 {
    value.setIcon(self);
    return 1;
  }
}

pub trait QAction_setIcon {
  fn setIcon(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QAction_setIcon for (&'a  QIcon) {
  fn setIcon(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QAction7setIconERK5QIcon(arg0)};
    return 1;
  }
}

// proto: void QAction::NewQAction(QObject * parent);
impl<'a> /*trait*/ QAction_NewQAction for (&'a mut QObject) {
  fn NewQAction(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1EP7QObject(qthis, arg0)};
    let rsthis = QAction{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn metaObject<T: QAction_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QAction_metaObject {
  fn metaObject(self, this: &mut QAction) -> i32;
}

// proto: const QMetaObject * QAction::metaObject();
impl<'a> /*trait*/ QAction_metaObject for () {
  fn metaObject(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction10metaObjectEv()};
    unsafe {_ZNK7QAction10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn triggered<T: QAction_triggered>(&mut self, value: T) -> i32 {
    value.triggered(self);
    return 1;
  }
}

pub trait QAction_triggered {
  fn triggered(self, this: &mut QAction) -> i32;
}

// proto: void QAction::triggered(bool checked);
impl<'a> /*trait*/ QAction_triggered for (i8) {
  fn triggered(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction9triggeredEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QAction9triggeredEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn toggled<T: QAction_toggled>(&mut self, value: T) -> i32 {
    value.toggled(self);
    return 1;
  }
}

pub trait QAction_toggled {
  fn toggled(self, this: &mut QAction) -> i32;
}

// proto: void QAction::toggled(bool );
impl<'a> /*trait*/ QAction_toggled for (i8) {
  fn toggled(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7toggledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QAction7toggledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setText<T: QAction_setText>(&mut self, value: T) -> i32 {
    value.setText(self);
    return 1;
  }
}

pub trait QAction_setText {
  fn setText(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setText(const QString & text);
impl<'a> /*trait*/ QAction_setText for (&'a  QString) {
  fn setText(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QAction7setTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn showStatusText<T: QAction_showStatusText>(&mut self, value: T) -> i32 {
    value.showStatusText(self);
    return 1;
  }
}

pub trait QAction_showStatusText {
  fn showStatusText(self, this: &mut QAction) -> i32;
}

// proto: bool QAction::showStatusText(QWidget * widget);
impl<'a> /*trait*/ QAction_showStatusText for (&'a mut QWidget) {
  fn showStatusText(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction14showStatusTextEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QAction14showStatusTextEP7QWidget(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn iconText<T: QAction_iconText>(&mut self, value: T) -> i32 {
    value.iconText(self);
    return 1;
  }
}

pub trait QAction_iconText {
  fn iconText(self, this: &mut QAction) -> i32;
}

// proto: QString QAction::iconText();
impl<'a> /*trait*/ QAction_iconText for () {
  fn iconText(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction8iconTextEv()};
    unsafe {_ZNK7QAction8iconTextEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setIconVisibleInMenu<T: QAction_setIconVisibleInMenu>(&mut self, value: T) -> i32 {
    value.setIconVisibleInMenu(self);
    return 1;
  }
}

pub trait QAction_setIconVisibleInMenu {
  fn setIconVisibleInMenu(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setIconVisibleInMenu(bool visible);
impl<'a> /*trait*/ QAction_setIconVisibleInMenu for (i8) {
  fn setIconVisibleInMenu(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction20setIconVisibleInMenuEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QAction20setIconVisibleInMenuEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn statusTip<T: QAction_statusTip>(&mut self, value: T) -> i32 {
    value.statusTip(self);
    return 1;
  }
}

pub trait QAction_statusTip {
  fn statusTip(self, this: &mut QAction) -> i32;
}

// proto: QString QAction::statusTip();
impl<'a> /*trait*/ QAction_statusTip for () {
  fn statusTip(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9statusTipEv()};
    unsafe {_ZNK7QAction9statusTipEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isCheckable<T: QAction_isCheckable>(&mut self, value: T) -> i32 {
    value.isCheckable(self);
    return 1;
  }
}

pub trait QAction_isCheckable {
  fn isCheckable(self, this: &mut QAction) -> i32;
}

// proto: bool QAction::isCheckable();
impl<'a> /*trait*/ QAction_isCheckable for () {
  fn isCheckable(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction11isCheckableEv()};
    unsafe {_ZNK7QAction11isCheckableEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setWhatsThis<T: QAction_setWhatsThis>(&mut self, value: T) -> i32 {
    value.setWhatsThis(self);
    return 1;
  }
}

pub trait QAction_setWhatsThis {
  fn setWhatsThis(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setWhatsThis(const QString & what);
impl<'a> /*trait*/ QAction_setWhatsThis for (&'a  QString) {
  fn setWhatsThis(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QAction12setWhatsThisERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn menu<T: QAction_menu>(&mut self, value: T) -> i32 {
    value.menu(self);
    return 1;
  }
}

pub trait QAction_menu {
  fn menu(self, this: &mut QAction) -> i32;
}

// proto: QMenu * QAction::menu();
impl<'a> /*trait*/ QAction_menu for () {
  fn menu(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4menuEv()};
    unsafe {_ZNK7QAction4menuEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn trigger<T: QAction_trigger>(&mut self, value: T) -> i32 {
    value.trigger(self);
    return 1;
  }
}

pub trait QAction_trigger {
  fn trigger(self, this: &mut QAction) -> i32;
}

// proto: void QAction::trigger();
impl<'a> /*trait*/ QAction_trigger for () {
  fn trigger(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7triggerEv()};
    unsafe {_ZN7QAction7triggerEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn changed<T: QAction_changed>(&mut self, value: T) -> i32 {
    value.changed(self);
    return 1;
  }
}

pub trait QAction_changed {
  fn changed(self, this: &mut QAction) -> i32;
}

// proto: void QAction::changed();
impl<'a> /*trait*/ QAction_changed for () {
  fn changed(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7changedEv()};
    unsafe {_ZN7QAction7changedEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn font<T: QAction_font>(&mut self, value: T) -> i32 {
    value.font(self);
    return 1;
  }
}

pub trait QAction_font {
  fn font(self, this: &mut QAction) -> i32;
}

// proto: QFont QAction::font();
impl<'a> /*trait*/ QAction_font for () {
  fn font(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4fontEv()};
    unsafe {_ZNK7QAction4fontEv()};
    return 1;
  }
}

// proto: void QAction::NewQAction(const QIcon & icon, const QString & text, QObject * parent);
impl<'a> /*trait*/ QAction_NewQAction for (&'a  QIcon, &'a  QString, &'a mut QObject) {
  fn NewQAction(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1ERK5QIconRK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *const c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1ERK5QIconRK7QStringP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QAction{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setChecked<T: QAction_setChecked>(&mut self, value: T) -> i32 {
    value.setChecked(self);
    return 1;
  }
}

pub trait QAction_setChecked {
  fn setChecked(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setChecked(bool );
impl<'a> /*trait*/ QAction_setChecked for (i8) {
  fn setChecked(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setCheckedEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QAction10setCheckedEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setDisabled<T: QAction_setDisabled>(&mut self, value: T) -> i32 {
    value.setDisabled(self);
    return 1;
  }
}

pub trait QAction_setDisabled {
  fn setDisabled(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setDisabled(bool b);
impl<'a> /*trait*/ QAction_setDisabled for (i8) {
  fn setDisabled(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction11setDisabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QAction11setDisabledEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setShortcut<T: QAction_setShortcut>(&mut self, value: T) -> i32 {
    value.setShortcut(self);
    return 1;
  }
}

pub trait QAction_setShortcut {
  fn setShortcut(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setShortcut(const QKeySequence & shortcut);
impl<'a> /*trait*/ QAction_setShortcut for (&'a  QKeySequence) {
  fn setShortcut(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction11setShortcutERK12QKeySequence()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QAction11setShortcutERK12QKeySequence(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn toggle<T: QAction_toggle>(&mut self, value: T) -> i32 {
    value.toggle(self);
    return 1;
  }
}

pub trait QAction_toggle {
  fn toggle(self, this: &mut QAction) -> i32;
}

// proto: void QAction::toggle();
impl<'a> /*trait*/ QAction_toggle for () {
  fn toggle(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction6toggleEv()};
    unsafe {_ZN7QAction6toggleEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn shortcut<T: QAction_shortcut>(&mut self, value: T) -> i32 {
    value.shortcut(self);
    return 1;
  }
}

pub trait QAction_shortcut {
  fn shortcut(self, this: &mut QAction) -> i32;
}

// proto: QKeySequence QAction::shortcut();
impl<'a> /*trait*/ QAction_shortcut for () {
  fn shortcut(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction8shortcutEv()};
    unsafe {_ZNK7QAction8shortcutEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn icon<T: QAction_icon>(&mut self, value: T) -> i32 {
    value.icon(self);
    return 1;
  }
}

pub trait QAction_icon {
  fn icon(self, this: &mut QAction) -> i32;
}

// proto: QIcon QAction::icon();
impl<'a> /*trait*/ QAction_icon for () {
  fn icon(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4iconEv()};
    unsafe {_ZNK7QAction4iconEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setActionGroup<T: QAction_setActionGroup>(&mut self, value: T) -> i32 {
    value.setActionGroup(self);
    return 1;
  }
}

pub trait QAction_setActionGroup {
  fn setActionGroup(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setActionGroup(QActionGroup * group);
impl<'a> /*trait*/ QAction_setActionGroup for (&'a mut QActionGroup) {
  fn setActionGroup(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction14setActionGroupEP12QActionGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QAction14setActionGroupEP12QActionGroup(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setMenu<T: QAction_setMenu>(&mut self, value: T) -> i32 {
    value.setMenu(self);
    return 1;
  }
}

pub trait QAction_setMenu {
  fn setMenu(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setMenu(QMenu * menu);
impl<'a> /*trait*/ QAction_setMenu for (&'a mut QMenu) {
  fn setMenu(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QAction7setMenuEP5QMenu(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn shortcuts<T: QAction_shortcuts>(&mut self, value: T) -> i32 {
    value.shortcuts(self);
    return 1;
  }
}

pub trait QAction_shortcuts {
  fn shortcuts(self, this: &mut QAction) -> i32;
}

// proto: QList<QKeySequence> QAction::shortcuts();
impl<'a> /*trait*/ QAction_shortcuts for () {
  fn shortcuts(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9shortcutsEv()};
    unsafe {_ZNK7QAction9shortcutsEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setCheckable<T: QAction_setCheckable>(&mut self, value: T) -> i32 {
    value.setCheckable(self);
    return 1;
  }
}

pub trait QAction_setCheckable {
  fn setCheckable(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setCheckable(bool );
impl<'a> /*trait*/ QAction_setCheckable for (i8) {
  fn setCheckable(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setCheckableEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QAction12setCheckableEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn toolTip<T: QAction_toolTip>(&mut self, value: T) -> i32 {
    value.toolTip(self);
    return 1;
  }
}

pub trait QAction_toolTip {
  fn toolTip(self, this: &mut QAction) -> i32;
}

// proto: QString QAction::toolTip();
impl<'a> /*trait*/ QAction_toolTip for () {
  fn toolTip(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction7toolTipEv()};
    unsafe {_ZNK7QAction7toolTipEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn parentWidget<T: QAction_parentWidget>(&mut self, value: T) -> i32 {
    value.parentWidget(self);
    return 1;
  }
}

pub trait QAction_parentWidget {
  fn parentWidget(self, this: &mut QAction) -> i32;
}

// proto: QWidget * QAction::parentWidget();
impl<'a> /*trait*/ QAction_parentWidget for () {
  fn parentWidget(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction12parentWidgetEv()};
    unsafe {_ZNK7QAction12parentWidgetEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setSeparator<T: QAction_setSeparator>(&mut self, value: T) -> i32 {
    value.setSeparator(self);
    return 1;
  }
}

pub trait QAction_setSeparator {
  fn setSeparator(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setSeparator(bool b);
impl<'a> /*trait*/ QAction_setSeparator for (i8) {
  fn setSeparator(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setSeparatorEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QAction12setSeparatorEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn associatedGraphicsWidgets<T: QAction_associatedGraphicsWidgets>(&mut self, value: T) -> i32 {
    value.associatedGraphicsWidgets(self);
    return 1;
  }
}

pub trait QAction_associatedGraphicsWidgets {
  fn associatedGraphicsWidgets(self, this: &mut QAction) -> i32;
}

// proto: QList<QGraphicsWidget *> QAction::associatedGraphicsWidgets();
impl<'a> /*trait*/ QAction_associatedGraphicsWidgets for () {
  fn associatedGraphicsWidgets(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction25associatedGraphicsWidgetsEv()};
    unsafe {_ZNK7QAction25associatedGraphicsWidgetsEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setVisible<T: QAction_setVisible>(&mut self, value: T) -> i32 {
    value.setVisible(self);
    return 1;
  }
}

pub trait QAction_setVisible {
  fn setVisible(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setVisible(bool );
impl<'a> /*trait*/ QAction_setVisible for (i8) {
  fn setVisible(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setVisibleEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QAction10setVisibleEb(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isSeparator<T: QAction_isSeparator>(&mut self, value: T) -> i32 {
    value.isSeparator(self);
    return 1;
  }
}

pub trait QAction_isSeparator {
  fn isSeparator(self, this: &mut QAction) -> i32;
}

// proto: bool QAction::isSeparator();
impl<'a> /*trait*/ QAction_isSeparator for () {
  fn isSeparator(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction11isSeparatorEv()};
    unsafe {_ZNK7QAction11isSeparatorEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setIconText<T: QAction_setIconText>(&mut self, value: T) -> i32 {
    value.setIconText(self);
    return 1;
  }
}

pub trait QAction_setIconText {
  fn setIconText(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setIconText(const QString & text);
impl<'a> /*trait*/ QAction_setIconText for (&'a  QString) {
  fn setIconText(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction11setIconTextERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QAction11setIconTextERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn actionGroup<T: QAction_actionGroup>(&mut self, value: T) -> i32 {
    value.actionGroup(self);
    return 1;
  }
}

pub trait QAction_actionGroup {
  fn actionGroup(self, this: &mut QAction) -> i32;
}

// proto: QActionGroup * QAction::actionGroup();
impl<'a> /*trait*/ QAction_actionGroup for () {
  fn actionGroup(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction11actionGroupEv()};
    unsafe {_ZNK7QAction11actionGroupEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setStatusTip<T: QAction_setStatusTip>(&mut self, value: T) -> i32 {
    value.setStatusTip(self);
    return 1;
  }
}

pub trait QAction_setStatusTip {
  fn setStatusTip(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QAction_setStatusTip for (&'a  QString) {
  fn setStatusTip(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QAction12setStatusTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn hovered<T: QAction_hovered>(&mut self, value: T) -> i32 {
    value.hovered(self);
    return 1;
  }
}

pub trait QAction_hovered {
  fn hovered(self, this: &mut QAction) -> i32;
}

// proto: void QAction::hovered();
impl<'a> /*trait*/ QAction_hovered for () {
  fn hovered(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7hoveredEv()};
    unsafe {_ZN7QAction7hoveredEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn data<T: QAction_data>(&mut self, value: T) -> i32 {
    value.data(self);
    return 1;
  }
}

pub trait QAction_data {
  fn data(self, this: &mut QAction) -> i32;
}

// proto: QVariant QAction::data();
impl<'a> /*trait*/ QAction_data for () {
  fn data(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4dataEv()};
    unsafe {_ZNK7QAction4dataEv()};
    return 1;
  }
}

// proto: void QAction::NewQAction(const QAction & );
impl<'a> /*trait*/ QAction_NewQAction for (&'a  QAction) {
  fn NewQAction(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QActionC1ERKS_(qthis, arg0)};
    let rsthis = QAction{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isChecked<T: QAction_isChecked>(&mut self, value: T) -> i32 {
    value.isChecked(self);
    return 1;
  }
}

pub trait QAction_isChecked {
  fn isChecked(self, this: &mut QAction) -> i32;
}

// proto: bool QAction::isChecked();
impl<'a> /*trait*/ QAction_isChecked for () {
  fn isChecked(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9isCheckedEv()};
    unsafe {_ZNK7QAction9isCheckedEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn autoRepeat<T: QAction_autoRepeat>(&mut self, value: T) -> i32 {
    value.autoRepeat(self);
    return 1;
  }
}

pub trait QAction_autoRepeat {
  fn autoRepeat(self, this: &mut QAction) -> i32;
}

// proto: bool QAction::autoRepeat();
impl<'a> /*trait*/ QAction_autoRepeat for () {
  fn autoRepeat(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction10autoRepeatEv()};
    unsafe {_ZNK7QAction10autoRepeatEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isEnabled<T: QAction_isEnabled>(&mut self, value: T) -> i32 {
    value.isEnabled(self);
    return 1;
  }
}

pub trait QAction_isEnabled {
  fn isEnabled(self, this: &mut QAction) -> i32;
}

// proto: bool QAction::isEnabled();
impl<'a> /*trait*/ QAction_isEnabled for () {
  fn isEnabled(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9isEnabledEv()};
    unsafe {_ZNK7QAction9isEnabledEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn text<T: QAction_text>(&mut self, value: T) -> i32 {
    value.text(self);
    return 1;
  }
}

pub trait QAction_text {
  fn text(self, this: &mut QAction) -> i32;
}

// proto: QString QAction::text();
impl<'a> /*trait*/ QAction_text for () {
  fn text(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4textEv()};
    unsafe {_ZNK7QAction4textEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn hover<T: QAction_hover>(&mut self, value: T) -> i32 {
    value.hover(self);
    return 1;
  }
}

pub trait QAction_hover {
  fn hover(self, this: &mut QAction) -> i32;
}

// proto: void QAction::hover();
impl<'a> /*trait*/ QAction_hover for () {
  fn hover(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction5hoverEv()};
    unsafe {_ZN7QAction5hoverEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isIconVisibleInMenu<T: QAction_isIconVisibleInMenu>(&mut self, value: T) -> i32 {
    value.isIconVisibleInMenu(self);
    return 1;
  }
}

pub trait QAction_isIconVisibleInMenu {
  fn isIconVisibleInMenu(self, this: &mut QAction) -> i32;
}

// proto: bool QAction::isIconVisibleInMenu();
impl<'a> /*trait*/ QAction_isIconVisibleInMenu for () {
  fn isIconVisibleInMenu(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction19isIconVisibleInMenuEv()};
    unsafe {_ZNK7QAction19isIconVisibleInMenuEv()};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setToolTip<T: QAction_setToolTip>(&mut self, value: T) -> i32 {
    value.setToolTip(self);
    return 1;
  }
}

pub trait QAction_setToolTip {
  fn setToolTip(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setToolTip(const QString & tip);
impl<'a> /*trait*/ QAction_setToolTip for (&'a  QString) {
  fn setToolTip(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN7QAction10setToolTipERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setEnabled<T: QAction_setEnabled>(&mut self, value: T) -> i32 {
    value.setEnabled(self);
    return 1;
  }
}

pub trait QAction_setEnabled {
  fn setEnabled(self, this: &mut QAction) -> i32;
}

// proto: void QAction::setEnabled(bool );
impl<'a> /*trait*/ QAction_setEnabled for (i8) {
  fn setEnabled(self, this: &mut QAction) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setEnabledEb()};
    let arg0 = self  as int8_t;
    unsafe {_ZN7QAction10setEnabledEb(arg0)};
    return 1;
  }
}

