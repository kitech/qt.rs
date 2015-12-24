// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtWidgets/qaction.h
// dst-file: /src/widgets/qaction.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::gui::qfont::QFont; // 771
use super::super::core::qvariant::QVariant; // 771
use super::super::gui::qicon::QIcon; // 771
use super::qwidget::QWidget; // 773
use super::qmenu::QMenu; // 773
use super::super::gui::qkeysequence::QKeySequence; // 771
use super::qactiongroup::QActionGroup; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  QList<QWidget *> QAction::associatedWidgets();
  fn _ZNK7QAction17associatedWidgetsEv(qthis: *mut c_void);
  // proto:  void QAction::setAutoRepeat(bool );
  fn _ZN7QAction13setAutoRepeatEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QString QAction::whatsThis();
  fn _ZNK7QAction9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::QAction(const QString & text, QObject * parent);
  fn _ZN7QActionC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QAction::~QAction();
  fn _ZN7QActionD0Ev(qthis: *mut c_void);
  // proto:  bool QAction::isVisible();
  fn _ZNK7QAction9isVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAction::setFont(const QFont & font);
  fn _ZN7QAction7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAction::setData(const QVariant & var);
  fn _ZN7QAction7setDataERK8QVariant(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAction::setIcon(const QIcon & icon);
  fn _ZN7QAction7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAction::QAction(QObject * parent);
  fn _ZN7QActionC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QAction::metaObject();
  fn _ZNK7QAction10metaObjectEv(qthis: *mut c_void);
  // proto:  void QAction::triggered(bool checked);
  fn _ZN7QAction9triggeredEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QAction::toggled(bool );
  fn _ZN7QAction7toggledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QAction::setText(const QString & text);
  fn _ZN7QAction7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QAction::showStatusText(QWidget * widget);
  fn _ZN7QAction14showStatusTextEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QString QAction::iconText();
  fn _ZNK7QAction8iconTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::setIconVisibleInMenu(bool visible);
  fn _ZN7QAction20setIconVisibleInMenuEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QString QAction::statusTip();
  fn _ZNK7QAction9statusTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QAction::isCheckable();
  fn _ZNK7QAction11isCheckableEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAction::setWhatsThis(const QString & what);
  fn _ZN7QAction12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QMenu * QAction::menu();
  fn _ZNK7QAction4menuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::trigger();
  fn _ZN7QAction7triggerEv(qthis: *mut c_void);
  // proto:  void QAction::changed();
  fn _ZN7QAction7changedEv(qthis: *mut c_void);
  // proto:  QFont QAction::font();
  fn _ZNK7QAction4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::QAction(const QIcon & icon, const QString & text, QObject * parent);
  fn _ZN7QActionC1ERK5QIconRK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QAction::setChecked(bool );
  fn _ZN7QAction10setCheckedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QAction::setDisabled(bool b);
  fn _ZN7QAction11setDisabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QAction::setShortcut(const QKeySequence & shortcut);
  fn _ZN7QAction11setShortcutERK12QKeySequence(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAction::toggle();
  fn _ZN7QAction6toggleEv(qthis: *mut c_void);
  // proto:  QKeySequence QAction::shortcut();
  fn _ZNK7QAction8shortcutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QIcon QAction::icon();
  fn _ZNK7QAction4iconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::setActionGroup(QActionGroup * group);
  fn _ZN7QAction14setActionGroupEP12QActionGroup(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAction::setMenu(QMenu * menu);
  fn _ZN7QAction7setMenuEP5QMenu(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QList<QKeySequence> QAction::shortcuts();
  fn _ZNK7QAction9shortcutsEv(qthis: *mut c_void);
  // proto:  void QAction::setCheckable(bool );
  fn _ZN7QAction12setCheckableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QString QAction::toolTip();
  fn _ZNK7QAction7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QAction::parentWidget();
  fn _ZNK7QAction12parentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::setSeparator(bool b);
  fn _ZN7QAction12setSeparatorEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QList<QGraphicsWidget *> QAction::associatedGraphicsWidgets();
  fn _ZNK7QAction25associatedGraphicsWidgetsEv(qthis: *mut c_void);
  // proto:  void QAction::setVisible(bool );
  fn _ZN7QAction10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QAction::isSeparator();
  fn _ZNK7QAction11isSeparatorEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAction::setIconText(const QString & text);
  fn _ZN7QAction11setIconTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QActionGroup * QAction::actionGroup();
  fn _ZNK7QAction11actionGroupEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::setStatusTip(const QString & statusTip);
  fn _ZN7QAction12setStatusTipERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAction::hovered();
  fn _ZN7QAction7hoveredEv(qthis: *mut c_void);
  // proto:  QVariant QAction::data();
  fn _ZNK7QAction4dataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::QAction(const QAction & );
  fn _ZN7QActionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QAction::isChecked();
  fn _ZNK7QAction9isCheckedEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QAction::autoRepeat();
  fn _ZNK7QAction10autoRepeatEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QAction::isEnabled();
  fn _ZNK7QAction9isEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QAction::text();
  fn _ZNK7QAction4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::hover();
  fn _ZN7QAction5hoverEv(qthis: *mut c_void);
  // proto:  bool QAction::isIconVisibleInMenu();
  fn _ZNK7QAction19isIconVisibleInMenuEv(qthis: *mut c_void) -> c_char;
  // proto:  void QAction::setToolTip(const QString & tip);
  fn _ZN7QAction10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAction::setEnabled(bool );
  fn _ZN7QAction10setEnabledEb(qthis: *mut c_void, arg0: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QAction)=1
pub struct QAction {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAction {
  pub fn inheritFrom(qthis: *mut c_void) -> QAction {
    return QAction{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QAction {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAction {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QList<QWidget *> QAction::associatedWidgets();
impl /*struct*/ QAction {
  pub fn associatedWidgets<RetType, T: QAction_associatedWidgets<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.associatedWidgets(self);
    // return 1;
  }
}

pub trait QAction_associatedWidgets<RetType> {
  fn associatedWidgets(self , rsthis: & QAction) -> RetType;
}

  // proto:  QList<QWidget *> QAction::associatedWidgets();
impl<'a> /*trait*/ QAction_associatedWidgets<()> for () {
  fn associatedWidgets(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction17associatedWidgetsEv()};
     unsafe {_ZNK7QAction17associatedWidgetsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAction::setAutoRepeat(bool );
impl /*struct*/ QAction {
  pub fn setAutoRepeat<RetType, T: QAction_setAutoRepeat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoRepeat(self);
    // return 1;
  }
}

pub trait QAction_setAutoRepeat<RetType> {
  fn setAutoRepeat(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setAutoRepeat(bool );
impl<'a> /*trait*/ QAction_setAutoRepeat<()> for (i8) {
  fn setAutoRepeat(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction13setAutoRepeatEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QAction13setAutoRepeatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QAction::whatsThis();
impl /*struct*/ QAction {
  pub fn whatsThis<RetType, T: QAction_whatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.whatsThis(self);
    // return 1;
  }
}

pub trait QAction_whatsThis<RetType> {
  fn whatsThis(self , rsthis: & QAction) -> RetType;
}

  // proto:  QString QAction::whatsThis();
impl<'a> /*trait*/ QAction_whatsThis<QString> for () {
  fn whatsThis(self , rsthis: & QAction) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9whatsThisEv()};
    let mut ret = unsafe {_ZNK7QAction9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAction::QAction(const QString & text, QObject * parent);
impl /*struct*/ QAction {
  pub fn New<T: QAction_New>(value: T) -> QAction {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAction_New {
  fn New(self) -> QAction;
}

  // proto:  void QAction::QAction(const QString & text, QObject * parent);
impl<'a> /*trait*/ QAction_New for (&'a QString, &'a QObject) {
  fn New(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QAction{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAction::~QAction();
impl /*struct*/ QAction {
  pub fn Free<RetType, T: QAction_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAction_Free<RetType> {
  fn Free(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::~QAction();
impl<'a> /*trait*/ QAction_Free<()> for () {
  fn Free(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionD0Ev()};
     unsafe {_ZN7QActionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAction::isVisible();
impl /*struct*/ QAction {
  pub fn isVisible<RetType, T: QAction_isVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QAction_isVisible<RetType> {
  fn isVisible(self , rsthis: & QAction) -> RetType;
}

  // proto:  bool QAction::isVisible();
impl<'a> /*trait*/ QAction_isVisible<i8> for () {
  fn isVisible(self , rsthis: & QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9isVisibleEv()};
    let mut ret = unsafe {_ZNK7QAction9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAction::setFont(const QFont & font);
impl /*struct*/ QAction {
  pub fn setFont<RetType, T: QAction_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QAction_setFont<RetType> {
  fn setFont(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setFont(const QFont & font);
impl<'a> /*trait*/ QAction_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::setData(const QVariant & var);
impl /*struct*/ QAction {
  pub fn setData<RetType, T: QAction_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QAction_setData<RetType> {
  fn setData(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setData(const QVariant & var);
impl<'a> /*trait*/ QAction_setData<()> for (&'a QVariant) {
  fn setData(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setDataERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction7setDataERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::setIcon(const QIcon & icon);
impl /*struct*/ QAction {
  pub fn setIcon<RetType, T: QAction_setIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QAction_setIcon<RetType> {
  fn setIcon(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QAction_setIcon<()> for (&'a QIcon) {
  fn setIcon(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::QAction(QObject * parent);
impl<'a> /*trait*/ QAction_New for (&'a QObject) {
  fn New(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1EP7QObject(qthis, arg0)};
    let rsthis = QAction{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAction::metaObject();
impl /*struct*/ QAction {
  pub fn metaObject<RetType, T: QAction_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAction_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAction) -> RetType;
}

  // proto:  const QMetaObject * QAction::metaObject();
impl<'a> /*trait*/ QAction_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction10metaObjectEv()};
     unsafe {_ZNK7QAction10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAction::triggered(bool checked);
impl /*struct*/ QAction {
  pub fn triggered<RetType, T: QAction_triggered<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.triggered(self);
    // return 1;
  }
}

pub trait QAction_triggered<RetType> {
  fn triggered(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::triggered(bool checked);
impl<'a> /*trait*/ QAction_triggered<()> for (i8) {
  fn triggered(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction9triggeredEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QAction9triggeredEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::toggled(bool );
impl /*struct*/ QAction {
  pub fn toggled<RetType, T: QAction_toggled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toggled(self);
    // return 1;
  }
}

pub trait QAction_toggled<RetType> {
  fn toggled(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::toggled(bool );
impl<'a> /*trait*/ QAction_toggled<()> for (i8) {
  fn toggled(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7toggledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QAction7toggledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::setText(const QString & text);
impl /*struct*/ QAction {
  pub fn setText<RetType, T: QAction_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QAction_setText<RetType> {
  fn setText(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setText(const QString & text);
impl<'a> /*trait*/ QAction_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAction::showStatusText(QWidget * widget);
impl /*struct*/ QAction {
  pub fn showStatusText<RetType, T: QAction_showStatusText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showStatusText(self);
    // return 1;
  }
}

pub trait QAction_showStatusText<RetType> {
  fn showStatusText(self , rsthis: & QAction) -> RetType;
}

  // proto:  bool QAction::showStatusText(QWidget * widget);
impl<'a> /*trait*/ QAction_showStatusText<i8> for (&'a QWidget) {
  fn showStatusText(self , rsthis: & QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction14showStatusTextEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QAction14showStatusTextEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QAction::iconText();
impl /*struct*/ QAction {
  pub fn iconText<RetType, T: QAction_iconText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.iconText(self);
    // return 1;
  }
}

pub trait QAction_iconText<RetType> {
  fn iconText(self , rsthis: & QAction) -> RetType;
}

  // proto:  QString QAction::iconText();
impl<'a> /*trait*/ QAction_iconText<QString> for () {
  fn iconText(self , rsthis: & QAction) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction8iconTextEv()};
    let mut ret = unsafe {_ZNK7QAction8iconTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAction::setIconVisibleInMenu(bool visible);
impl /*struct*/ QAction {
  pub fn setIconVisibleInMenu<RetType, T: QAction_setIconVisibleInMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconVisibleInMenu(self);
    // return 1;
  }
}

pub trait QAction_setIconVisibleInMenu<RetType> {
  fn setIconVisibleInMenu(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setIconVisibleInMenu(bool visible);
impl<'a> /*trait*/ QAction_setIconVisibleInMenu<()> for (i8) {
  fn setIconVisibleInMenu(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction20setIconVisibleInMenuEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QAction20setIconVisibleInMenuEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QAction::statusTip();
impl /*struct*/ QAction {
  pub fn statusTip<RetType, T: QAction_statusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.statusTip(self);
    // return 1;
  }
}

pub trait QAction_statusTip<RetType> {
  fn statusTip(self , rsthis: & QAction) -> RetType;
}

  // proto:  QString QAction::statusTip();
impl<'a> /*trait*/ QAction_statusTip<QString> for () {
  fn statusTip(self , rsthis: & QAction) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9statusTipEv()};
    let mut ret = unsafe {_ZNK7QAction9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAction::isCheckable();
impl /*struct*/ QAction {
  pub fn isCheckable<RetType, T: QAction_isCheckable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCheckable(self);
    // return 1;
  }
}

pub trait QAction_isCheckable<RetType> {
  fn isCheckable(self , rsthis: & QAction) -> RetType;
}

  // proto:  bool QAction::isCheckable();
impl<'a> /*trait*/ QAction_isCheckable<i8> for () {
  fn isCheckable(self , rsthis: & QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction11isCheckableEv()};
    let mut ret = unsafe {_ZNK7QAction11isCheckableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAction::setWhatsThis(const QString & what);
impl /*struct*/ QAction {
  pub fn setWhatsThis<RetType, T: QAction_setWhatsThis<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWhatsThis(self);
    // return 1;
  }
}

pub trait QAction_setWhatsThis<RetType> {
  fn setWhatsThis(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setWhatsThis(const QString & what);
impl<'a> /*trait*/ QAction_setWhatsThis<()> for (&'a QString) {
  fn setWhatsThis(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMenu * QAction::menu();
impl /*struct*/ QAction {
  pub fn menu<RetType, T: QAction_menu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.menu(self);
    // return 1;
  }
}

pub trait QAction_menu<RetType> {
  fn menu(self , rsthis: & QAction) -> RetType;
}

  // proto:  QMenu * QAction::menu();
impl<'a> /*trait*/ QAction_menu<QMenu> for () {
  fn menu(self , rsthis: & QAction) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4menuEv()};
    let mut ret = unsafe {_ZNK7QAction4menuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAction::trigger();
impl /*struct*/ QAction {
  pub fn trigger<RetType, T: QAction_trigger<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.trigger(self);
    // return 1;
  }
}

pub trait QAction_trigger<RetType> {
  fn trigger(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::trigger();
impl<'a> /*trait*/ QAction_trigger<()> for () {
  fn trigger(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7triggerEv()};
     unsafe {_ZN7QAction7triggerEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAction::changed();
impl /*struct*/ QAction {
  pub fn changed<RetType, T: QAction_changed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.changed(self);
    // return 1;
  }
}

pub trait QAction_changed<RetType> {
  fn changed(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::changed();
impl<'a> /*trait*/ QAction_changed<()> for () {
  fn changed(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7changedEv()};
     unsafe {_ZN7QAction7changedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QFont QAction::font();
impl /*struct*/ QAction {
  pub fn font<RetType, T: QAction_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QAction_font<RetType> {
  fn font(self , rsthis: & QAction) -> RetType;
}

  // proto:  QFont QAction::font();
impl<'a> /*trait*/ QAction_font<QFont> for () {
  fn font(self , rsthis: & QAction) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4fontEv()};
    let mut ret = unsafe {_ZNK7QAction4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAction::QAction(const QIcon & icon, const QString & text, QObject * parent);
impl<'a> /*trait*/ QAction_New for (&'a QIcon, &'a QString, &'a QObject) {
  fn New(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1ERK5QIconRK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1ERK5QIconRK7QStringP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QAction{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAction::setChecked(bool );
impl /*struct*/ QAction {
  pub fn setChecked<RetType, T: QAction_setChecked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setChecked(self);
    // return 1;
  }
}

pub trait QAction_setChecked<RetType> {
  fn setChecked(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setChecked(bool );
impl<'a> /*trait*/ QAction_setChecked<()> for (i8) {
  fn setChecked(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setCheckedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QAction10setCheckedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::setDisabled(bool b);
impl /*struct*/ QAction {
  pub fn setDisabled<RetType, T: QAction_setDisabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDisabled(self);
    // return 1;
  }
}

pub trait QAction_setDisabled<RetType> {
  fn setDisabled(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setDisabled(bool b);
impl<'a> /*trait*/ QAction_setDisabled<()> for (i8) {
  fn setDisabled(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction11setDisabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QAction11setDisabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::setShortcut(const QKeySequence & shortcut);
impl /*struct*/ QAction {
  pub fn setShortcut<RetType, T: QAction_setShortcut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setShortcut(self);
    // return 1;
  }
}

pub trait QAction_setShortcut<RetType> {
  fn setShortcut(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setShortcut(const QKeySequence & shortcut);
impl<'a> /*trait*/ QAction_setShortcut<()> for (&'a QKeySequence) {
  fn setShortcut(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction11setShortcutERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction11setShortcutERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::toggle();
impl /*struct*/ QAction {
  pub fn toggle<RetType, T: QAction_toggle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toggle(self);
    // return 1;
  }
}

pub trait QAction_toggle<RetType> {
  fn toggle(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::toggle();
impl<'a> /*trait*/ QAction_toggle<()> for () {
  fn toggle(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction6toggleEv()};
     unsafe {_ZN7QAction6toggleEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QKeySequence QAction::shortcut();
impl /*struct*/ QAction {
  pub fn shortcut<RetType, T: QAction_shortcut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shortcut(self);
    // return 1;
  }
}

pub trait QAction_shortcut<RetType> {
  fn shortcut(self , rsthis: & QAction) -> RetType;
}

  // proto:  QKeySequence QAction::shortcut();
impl<'a> /*trait*/ QAction_shortcut<QKeySequence> for () {
  fn shortcut(self , rsthis: & QAction) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction8shortcutEv()};
    let mut ret = unsafe {_ZNK7QAction8shortcutEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QIcon QAction::icon();
impl /*struct*/ QAction {
  pub fn icon<RetType, T: QAction_icon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QAction_icon<RetType> {
  fn icon(self , rsthis: & QAction) -> RetType;
}

  // proto:  QIcon QAction::icon();
impl<'a> /*trait*/ QAction_icon<QIcon> for () {
  fn icon(self , rsthis: & QAction) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4iconEv()};
    let mut ret = unsafe {_ZNK7QAction4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAction::setActionGroup(QActionGroup * group);
impl /*struct*/ QAction {
  pub fn setActionGroup<RetType, T: QAction_setActionGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActionGroup(self);
    // return 1;
  }
}

pub trait QAction_setActionGroup<RetType> {
  fn setActionGroup(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setActionGroup(QActionGroup * group);
impl<'a> /*trait*/ QAction_setActionGroup<()> for (&'a QActionGroup) {
  fn setActionGroup(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction14setActionGroupEP12QActionGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction14setActionGroupEP12QActionGroup(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::setMenu(QMenu * menu);
impl /*struct*/ QAction {
  pub fn setMenu<RetType, T: QAction_setMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMenu(self);
    // return 1;
  }
}

pub trait QAction_setMenu<RetType> {
  fn setMenu(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setMenu(QMenu * menu);
impl<'a> /*trait*/ QAction_setMenu<()> for (&'a QMenu) {
  fn setMenu(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction7setMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QKeySequence> QAction::shortcuts();
impl /*struct*/ QAction {
  pub fn shortcuts<RetType, T: QAction_shortcuts<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shortcuts(self);
    // return 1;
  }
}

pub trait QAction_shortcuts<RetType> {
  fn shortcuts(self , rsthis: & QAction) -> RetType;
}

  // proto:  QList<QKeySequence> QAction::shortcuts();
impl<'a> /*trait*/ QAction_shortcuts<()> for () {
  fn shortcuts(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9shortcutsEv()};
     unsafe {_ZNK7QAction9shortcutsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAction::setCheckable(bool );
impl /*struct*/ QAction {
  pub fn setCheckable<RetType, T: QAction_setCheckable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCheckable(self);
    // return 1;
  }
}

pub trait QAction_setCheckable<RetType> {
  fn setCheckable(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setCheckable(bool );
impl<'a> /*trait*/ QAction_setCheckable<()> for (i8) {
  fn setCheckable(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setCheckableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QAction12setCheckableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QAction::toolTip();
impl /*struct*/ QAction {
  pub fn toolTip<RetType, T: QAction_toolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QAction_toolTip<RetType> {
  fn toolTip(self , rsthis: & QAction) -> RetType;
}

  // proto:  QString QAction::toolTip();
impl<'a> /*trait*/ QAction_toolTip<QString> for () {
  fn toolTip(self , rsthis: & QAction) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction7toolTipEv()};
    let mut ret = unsafe {_ZNK7QAction7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QAction::parentWidget();
impl /*struct*/ QAction {
  pub fn parentWidget<RetType, T: QAction_parentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parentWidget(self);
    // return 1;
  }
}

pub trait QAction_parentWidget<RetType> {
  fn parentWidget(self , rsthis: & QAction) -> RetType;
}

  // proto:  QWidget * QAction::parentWidget();
impl<'a> /*trait*/ QAction_parentWidget<QWidget> for () {
  fn parentWidget(self , rsthis: & QAction) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction12parentWidgetEv()};
    let mut ret = unsafe {_ZNK7QAction12parentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAction::setSeparator(bool b);
impl /*struct*/ QAction {
  pub fn setSeparator<RetType, T: QAction_setSeparator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSeparator(self);
    // return 1;
  }
}

pub trait QAction_setSeparator<RetType> {
  fn setSeparator(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setSeparator(bool b);
impl<'a> /*trait*/ QAction_setSeparator<()> for (i8) {
  fn setSeparator(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setSeparatorEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QAction12setSeparatorEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QGraphicsWidget *> QAction::associatedGraphicsWidgets();
impl /*struct*/ QAction {
  pub fn associatedGraphicsWidgets<RetType, T: QAction_associatedGraphicsWidgets<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.associatedGraphicsWidgets(self);
    // return 1;
  }
}

pub trait QAction_associatedGraphicsWidgets<RetType> {
  fn associatedGraphicsWidgets(self , rsthis: & QAction) -> RetType;
}

  // proto:  QList<QGraphicsWidget *> QAction::associatedGraphicsWidgets();
impl<'a> /*trait*/ QAction_associatedGraphicsWidgets<()> for () {
  fn associatedGraphicsWidgets(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction25associatedGraphicsWidgetsEv()};
     unsafe {_ZNK7QAction25associatedGraphicsWidgetsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAction::setVisible(bool );
impl /*struct*/ QAction {
  pub fn setVisible<RetType, T: QAction_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QAction_setVisible<RetType> {
  fn setVisible(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setVisible(bool );
impl<'a> /*trait*/ QAction_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QAction10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAction::isSeparator();
impl /*struct*/ QAction {
  pub fn isSeparator<RetType, T: QAction_isSeparator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSeparator(self);
    // return 1;
  }
}

pub trait QAction_isSeparator<RetType> {
  fn isSeparator(self , rsthis: & QAction) -> RetType;
}

  // proto:  bool QAction::isSeparator();
impl<'a> /*trait*/ QAction_isSeparator<i8> for () {
  fn isSeparator(self , rsthis: & QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction11isSeparatorEv()};
    let mut ret = unsafe {_ZNK7QAction11isSeparatorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAction::setIconText(const QString & text);
impl /*struct*/ QAction {
  pub fn setIconText<RetType, T: QAction_setIconText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIconText(self);
    // return 1;
  }
}

pub trait QAction_setIconText<RetType> {
  fn setIconText(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setIconText(const QString & text);
impl<'a> /*trait*/ QAction_setIconText<()> for (&'a QString) {
  fn setIconText(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction11setIconTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction11setIconTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QActionGroup * QAction::actionGroup();
impl /*struct*/ QAction {
  pub fn actionGroup<RetType, T: QAction_actionGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actionGroup(self);
    // return 1;
  }
}

pub trait QAction_actionGroup<RetType> {
  fn actionGroup(self , rsthis: & QAction) -> RetType;
}

  // proto:  QActionGroup * QAction::actionGroup();
impl<'a> /*trait*/ QAction_actionGroup<QActionGroup> for () {
  fn actionGroup(self , rsthis: & QAction) -> QActionGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction11actionGroupEv()};
    let mut ret = unsafe {_ZNK7QAction11actionGroupEv(rsthis.qclsinst)};
    let mut ret1 = QActionGroup::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAction::setStatusTip(const QString & statusTip);
impl /*struct*/ QAction {
  pub fn setStatusTip<RetType, T: QAction_setStatusTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStatusTip(self);
    // return 1;
  }
}

pub trait QAction_setStatusTip<RetType> {
  fn setStatusTip(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QAction_setStatusTip<()> for (&'a QString) {
  fn setStatusTip(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::hovered();
impl /*struct*/ QAction {
  pub fn hovered<RetType, T: QAction_hovered<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hovered(self);
    // return 1;
  }
}

pub trait QAction_hovered<RetType> {
  fn hovered(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::hovered();
impl<'a> /*trait*/ QAction_hovered<()> for () {
  fn hovered(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7hoveredEv()};
     unsafe {_ZN7QAction7hoveredEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariant QAction::data();
impl /*struct*/ QAction {
  pub fn data<RetType, T: QAction_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QAction_data<RetType> {
  fn data(self , rsthis: & QAction) -> RetType;
}

  // proto:  QVariant QAction::data();
impl<'a> /*trait*/ QAction_data<QVariant> for () {
  fn data(self , rsthis: & QAction) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4dataEv()};
    let mut ret = unsafe {_ZNK7QAction4dataEv(rsthis.qclsinst)};
    let mut ret1 = QVariant::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAction::QAction(const QAction & );
impl<'a> /*trait*/ QAction_New for (&'a QAction) {
  fn New(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1ERKS_(qthis, arg0)};
    let rsthis = QAction{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QAction::isChecked();
impl /*struct*/ QAction {
  pub fn isChecked<RetType, T: QAction_isChecked<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isChecked(self);
    // return 1;
  }
}

pub trait QAction_isChecked<RetType> {
  fn isChecked(self , rsthis: & QAction) -> RetType;
}

  // proto:  bool QAction::isChecked();
impl<'a> /*trait*/ QAction_isChecked<i8> for () {
  fn isChecked(self , rsthis: & QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9isCheckedEv()};
    let mut ret = unsafe {_ZNK7QAction9isCheckedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAction::autoRepeat();
impl /*struct*/ QAction {
  pub fn autoRepeat<RetType, T: QAction_autoRepeat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoRepeat(self);
    // return 1;
  }
}

pub trait QAction_autoRepeat<RetType> {
  fn autoRepeat(self , rsthis: & QAction) -> RetType;
}

  // proto:  bool QAction::autoRepeat();
impl<'a> /*trait*/ QAction_autoRepeat<i8> for () {
  fn autoRepeat(self , rsthis: & QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction10autoRepeatEv()};
    let mut ret = unsafe {_ZNK7QAction10autoRepeatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAction::isEnabled();
impl /*struct*/ QAction {
  pub fn isEnabled<RetType, T: QAction_isEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QAction_isEnabled<RetType> {
  fn isEnabled(self , rsthis: & QAction) -> RetType;
}

  // proto:  bool QAction::isEnabled();
impl<'a> /*trait*/ QAction_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: & QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9isEnabledEv()};
    let mut ret = unsafe {_ZNK7QAction9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QAction::text();
impl /*struct*/ QAction {
  pub fn text<RetType, T: QAction_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QAction_text<RetType> {
  fn text(self , rsthis: & QAction) -> RetType;
}

  // proto:  QString QAction::text();
impl<'a> /*trait*/ QAction_text<QString> for () {
  fn text(self , rsthis: & QAction) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4textEv()};
    let mut ret = unsafe {_ZNK7QAction4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAction::hover();
impl /*struct*/ QAction {
  pub fn hover<RetType, T: QAction_hover<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hover(self);
    // return 1;
  }
}

pub trait QAction_hover<RetType> {
  fn hover(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::hover();
impl<'a> /*trait*/ QAction_hover<()> for () {
  fn hover(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction5hoverEv()};
     unsafe {_ZN7QAction5hoverEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAction::isIconVisibleInMenu();
impl /*struct*/ QAction {
  pub fn isIconVisibleInMenu<RetType, T: QAction_isIconVisibleInMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isIconVisibleInMenu(self);
    // return 1;
  }
}

pub trait QAction_isIconVisibleInMenu<RetType> {
  fn isIconVisibleInMenu(self , rsthis: & QAction) -> RetType;
}

  // proto:  bool QAction::isIconVisibleInMenu();
impl<'a> /*trait*/ QAction_isIconVisibleInMenu<i8> for () {
  fn isIconVisibleInMenu(self , rsthis: & QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction19isIconVisibleInMenuEv()};
    let mut ret = unsafe {_ZNK7QAction19isIconVisibleInMenuEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAction::setToolTip(const QString & tip);
impl /*struct*/ QAction {
  pub fn setToolTip<RetType, T: QAction_setToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QAction_setToolTip<RetType> {
  fn setToolTip(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setToolTip(const QString & tip);
impl<'a> /*trait*/ QAction_setToolTip<()> for (&'a QString) {
  fn setToolTip(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAction::setEnabled(bool );
impl /*struct*/ QAction {
  pub fn setEnabled<RetType, T: QAction_setEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QAction_setEnabled<RetType> {
  fn setEnabled(self , rsthis: & QAction) -> RetType;
}

  // proto:  void QAction::setEnabled(bool );
impl<'a> /*trait*/ QAction_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: & QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QAction10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

