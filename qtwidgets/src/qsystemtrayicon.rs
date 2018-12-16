

// mod ::widgets::QSystemTrayIcon
// package qtwidgets
// /usr/include/qt/QtWidgets/qsystemtrayicon.h
// #include <qsystemtrayicon.h>
// #include <QtWidgets>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 4
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
// import "github.com/kitech/qt.go/qtcore"
use qtcore::*; // super::super::%!s(MISSING)::*;
// import "github.com/kitech/qt.go/qtgui"
use qtgui::*; // super::super::%!s(MISSING)::*;
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QSystemTrayIcon) InheritEvent(f func(event *qtcore.QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSystemTrayIcon)=16
pub struct QSystemTrayIcon {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSystemTrayIcon_ITF interface {
//    qtcore.QObject_ITF
//    QSystemTrayIcon_PTR() *QSystemTrayIcon
//}
//func (ptr *QSystemTrayIcon) QSystemTrayIcon_PTR() *QSystemTrayIcon { return ptr }

impl /*struct*/ QSystemTrayIcon {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSystemTrayIcon {
    return QSystemTrayIcon{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSystemTrayIcon {
//  type Target = QSystemTrayIconBASE;
//
//  fn deref(&self) -> &QSystemTrayIconBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSystemTrayIconBASE> for QSystemTrayIcon {
//  fn as_ref(& self) -> & QSystemTrayIconBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtWidgets/qsystemtrayicon.h:63
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSystemTrayIcon {
  pub fn metaObject_0<RetType, T: QSystemTrayIcon_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSystemTrayIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSystemTrayIcon10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:69
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSystemTrayIcon(QObject *)

/*
Constructs a QSystemTrayIcon object with the given parent.

The icon is initially invisible.

See also visible.
*/
// QSystemTrayIcon(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSystemTrayIcon {
  pub fn QSystemTrayIcon_0<T: QSystemTrayIcon_QSystemTrayIcon_0>(value: T) -> QSystemTrayIcon {
    let rsthis = value.QSystemTrayIcon_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSystemTrayIcon_QSystemTrayIcon_0 {
  fn QSystemTrayIcon_0(self) -> QSystemTrayIcon;
}
// QSystemTrayIcon(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSystemTrayIcon_QSystemTrayIcon_0 for (usize) {
  fn QSystemTrayIcon_0(self) -> QSystemTrayIcon {
    // unsafe{_ZN15QSystemTrayIconC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QSystemTrayIconC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSystemTrayIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:70
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QSystemTrayIcon(const QIcon &, QObject *)

/*
Constructs a QSystemTrayIcon object with the given parent.

The icon is initially invisible.

See also visible.
*/
// QSystemTrayIcon(const QIcon &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSystemTrayIcon {
  pub fn QSystemTrayIcon_1<T: QSystemTrayIcon_QSystemTrayIcon_1>(value: T) -> QSystemTrayIcon {
    let rsthis = value.QSystemTrayIcon_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSystemTrayIcon_QSystemTrayIcon_1 {
  fn QSystemTrayIcon_1(self) -> QSystemTrayIcon;
}
// QSystemTrayIcon(const QIcon &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSystemTrayIcon_QSystemTrayIcon_1 for (usize,usize) {
  fn QSystemTrayIcon_1(self) -> QSystemTrayIcon {
    // unsafe{_ZN15QSystemTrayIconC2ERK5QIconP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN15QSystemTrayIconC2ERK5QIconP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSystemTrayIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSystemTrayIcon()

/*

*/
pub fn DeleteQSystemTrayIcon(this :*mut QSystemTrayIcon) {
    // let rv = qtrt::InvokeQtFunc6("_ZN15QSystemTrayIconD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtWidgets/qsystemtrayicon.h:82
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setContextMenu(QMenu *)

/*
Sets the specified menu to be the context menu for the system tray icon.

The menu will pop up when the user requests the context menu for the system tray icon by clicking the mouse button.

On macOS, this is currenly converted to a NSMenu, so the aboutToHide() signal is not emitted.

Note: The system tray icon does not take ownership of the menu. You must ensure that it is deleted at the appropriate time by, for example, creating the menu with a suitable parent object.

See also contextMenu().
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn setContextMenu_0<RetType, T: QSystemTrayIcon_setContextMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setContextMenu_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_setContextMenu_0<RetType> {
  fn setContextMenu_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_setContextMenu_0<(/*void*/)> for (usize) {
  fn setContextMenu_0(self , rsthis: & QSystemTrayIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon14setContextMenuEP5QMenu", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:83
// index:0
// Public Visibility=Default Availability=Available
// [8] QMenu * contextMenu() const

/*
Returns the current context menu for the system tray entry.

See also setContextMenu().
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn contextMenu_0<RetType, T: QSystemTrayIcon_contextMenu_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contextMenu_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_contextMenu_0<RetType> {
  fn contextMenu_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_contextMenu_0<usize> for () {
  fn contextMenu_0(self , rsthis: & QSystemTrayIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSystemTrayIcon11contextMenuEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:86
// index:0
// Public Visibility=Default Availability=Available
// [8] QIcon icon() const

/*

*/
impl /*struct*/ QSystemTrayIcon {
  pub fn icon_0<RetType, T: QSystemTrayIcon_icon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.icon_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_icon_0<RetType> {
  fn icon_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_icon_0<usize> for () {
  fn icon_0(self , rsthis: & QSystemTrayIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSystemTrayIcon4iconEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:87
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIcon(const QIcon &)

/*

*/
impl /*struct*/ QSystemTrayIcon {
  pub fn setIcon_0<RetType, T: QSystemTrayIcon_setIcon_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIcon_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_setIcon_0<RetType> {
  fn setIcon_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_setIcon_0<(/*void*/)> for (usize) {
  fn setIcon_0(self , rsthis: & QSystemTrayIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon7setIconERK5QIcon", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:89
// index:0
// Public Visibility=Default Availability=Available
// [8] QString toolTip() const

/*

*/
impl /*struct*/ QSystemTrayIcon {
  pub fn toolTip_0<RetType, T: QSystemTrayIcon_toolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.toolTip_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_toolTip_0<RetType> {
  fn toolTip_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_toolTip_0<usize> for () {
  fn toolTip_0(self , rsthis: & QSystemTrayIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSystemTrayIcon7toolTipEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:90
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setToolTip(const QString &)

/*

*/
impl /*struct*/ QSystemTrayIcon {
  pub fn setToolTip_0<RetType, T: QSystemTrayIcon_setToolTip_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setToolTip_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_setToolTip_0<RetType> {
  fn setToolTip_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_setToolTip_0<(/*void*/)> for (usize) {
  fn setToolTip_0(self , rsthis: & QSystemTrayIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon10setToolTipERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:92
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isSystemTrayAvailable()

/*
Returns true if the system tray is available; otherwise returns false.

If the system tray is currently unavailable but becomes available later, QSystemTrayIcon will automatically add an entry in the system tray if it is visible.
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn isSystemTrayAvailable_0<RetType, T: QSystemTrayIcon_isSystemTrayAvailable_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSystemTrayAvailable_0();
    // return 1;
  }
}
pub trait QSystemTrayIcon_isSystemTrayAvailable_0<RetType> {
  fn isSystemTrayAvailable_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_isSystemTrayAvailable_0<bool> for () {
  fn isSystemTrayAvailable_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon21isSystemTrayAvailableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:93
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool supportsMessages()

/*
Returns true if the system tray supports balloon messages; otherwise returns false.

See also showMessage().
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn supportsMessages_0<RetType, T: QSystemTrayIcon_supportsMessages_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportsMessages_0();
    // return 1;
  }
}
pub trait QSystemTrayIcon_supportsMessages_0<RetType> {
  fn supportsMessages_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_supportsMessages_0<bool> for () {
  fn supportsMessages_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon16supportsMessagesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:97
// index:0
// Public Visibility=Default Availability=Available
// [16] QRect geometry() const

/*
Returns the geometry of the system tray icon in screen coordinates.

This function was introduced in  Qt 4.3.

See also visible.
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn geometry_0<RetType, T: QSystemTrayIcon_geometry_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.geometry_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_geometry_0<RetType> {
  fn geometry_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_geometry_0<usize> for () {
  fn geometry_0(self , rsthis: & QSystemTrayIcon) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSystemTrayIcon8geometryEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:98
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isVisible() const

/*

*/
impl /*struct*/ QSystemTrayIcon {
  pub fn isVisible_0<RetType, T: QSystemTrayIcon_isVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isVisible_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_isVisible_0<RetType> {
  fn isVisible_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_isVisible_0<bool> for () {
  fn isVisible_0(self , rsthis: & QSystemTrayIcon) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK15QSystemTrayIcon9isVisibleEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:101
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setVisible(bool)

/*

*/
impl /*struct*/ QSystemTrayIcon {
  pub fn setVisible_0<RetType, T: QSystemTrayIcon_setVisible_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setVisible_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_setVisible_0<RetType> {
  fn setVisible_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_setVisible_0<(/*void*/)> for (bool) {
  fn setVisible_0(self , rsthis: & QSystemTrayIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon10setVisibleEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:102
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void show()

/*
Shows the icon in the system tray.

See also hide() and visible.
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn show_0<RetType, T: QSystemTrayIcon_show_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.show_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_show_0<RetType> {
  fn show_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_show_0<(/*void*/)> for () {
  fn show_0(self , rsthis: & QSystemTrayIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon4showEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:103
// index:0
// Public inline Visibility=Default Availability=Available
// [-2] void hide()

/*
Hides the system tray entry.

See also show() and visible.
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn hide_0<RetType, T: QSystemTrayIcon_hide_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.hide_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_hide_0<RetType> {
  fn hide_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_hide_0<(/*void*/)> for () {
  fn hide_0(self , rsthis: & QSystemTrayIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon4hideEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:104
// index:0
// Public Visibility=Default Availability=Available
// [-2] void showMessage(const QString &, const QString &, const QIcon &, int)

/*
Shows a balloon message for the entry with the given title, message and icon for the time specified in millisecondsTimeoutHint. title and message must be plain text strings.

Message can be clicked by the user; the messageClicked() signal will emitted when this occurs.

Note that display of messages are dependent on the system configuration and user preferences, and that messages may not appear at all. Hence, it should not be relied upon as the sole means for providing critical information.

On Windows, the millisecondsTimeoutHint is usually ignored by the system when the application has focus.

Has been turned into a slot in Qt 5.2.

This function was introduced in  Qt 4.3.

See also show() and supportsMessages().
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn showMessage_0<RetType, T: QSystemTrayIcon_showMessage_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMessage_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_showMessage_0<RetType> {
  fn showMessage_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_showMessage_0<(/*void*/)> for (usize,usize,usize,i32) {
  fn showMessage_0(self , rsthis: & QSystemTrayIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon11showMessageERK7QStringS2_RK5QIconi", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:105
// index:1
// Public Visibility=Default Availability=Available
// [-2] void showMessage(const QString &, const QString &, QSystemTrayIcon::MessageIcon, int)

/*
Shows a balloon message for the entry with the given title, message and icon for the time specified in millisecondsTimeoutHint. title and message must be plain text strings.

Message can be clicked by the user; the messageClicked() signal will emitted when this occurs.

Note that display of messages are dependent on the system configuration and user preferences, and that messages may not appear at all. Hence, it should not be relied upon as the sole means for providing critical information.

On Windows, the millisecondsTimeoutHint is usually ignored by the system when the application has focus.

Has been turned into a slot in Qt 5.2.

This function was introduced in  Qt 4.3.

See also show() and supportsMessages().
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn showMessage_1<RetType, T: QSystemTrayIcon_showMessage_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.showMessage_1(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_showMessage_1<RetType> {
  fn showMessage_1(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_showMessage_1<(/*void*/)> for (usize,usize,i32,i32) {
  fn showMessage_1(self , rsthis: & QSystemTrayIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2) as *const i32 as usize;
    let arg3 = (&self.3) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon11showMessageERK7QStringS2_NS_11MessageIconEi", 4,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:109
// index:0
// Public Visibility=Default Availability=Available
// [-2] void activated(QSystemTrayIcon::ActivationReason)

/*
This signal is emitted when the user activates the system tray icon. reason specifies the reason for activation. QSystemTrayIcon::ActivationReason enumerates the various reasons.

See also QSystemTrayIcon::ActivationReason.
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn activated_0<RetType, T: QSystemTrayIcon_activated_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.activated_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_activated_0<RetType> {
  fn activated_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_activated_0<(/*void*/)> for (i32) {
  fn activated_0(self , rsthis: & QSystemTrayIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon9activatedENS_16ActivationReasonE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:110
// index:0
// Public Visibility=Default Availability=Available
// [-2] void messageClicked()

/*
This signal is emitted when the message displayed using showMessage() was clicked by the user.

Currently this signal is not sent on macOS.

Note: We follow Microsoft Windows behavior, so the signal is also emitted when the user clicks on a tray icon with a balloon message displayed.

See also activated().
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn messageClicked_0<RetType, T: QSystemTrayIcon_messageClicked_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.messageClicked_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_messageClicked_0<RetType> {
  fn messageClicked_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_messageClicked_0<(/*void*/)> for () {
  fn messageClicked_0(self , rsthis: & QSystemTrayIcon) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon14messageClickedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtWidgets/qsystemtrayicon.h:113
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSystemTrayIcon {
  pub fn event_0<RetType, T: QSystemTrayIcon_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSystemTrayIcon_event_0<RetType> {
  fn event_0(self , rsthis: & QSystemTrayIcon) -> RetType;
}
impl<'a> /*trait*/ QSystemTrayIcon_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSystemTrayIcon) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN15QSystemTrayIcon5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
This enum describes the reason the system tray was activated.



Note: On macOS, a double click will only be emitted if no context menu is set, since the menu opens on mouse press



See also activated().

*/
pub type QSystemTrayIcon__ActivationReason = i32;
// Unknown reason
pub const QSystemTrayIcon__Unknown :QSystemTrayIcon__ActivationReason = 0;
// The context menu for the system tray entry was requested
pub const QSystemTrayIcon__Context :QSystemTrayIcon__ActivationReason = 1;
// The system tray entry was double clicked.
pub const QSystemTrayIcon__DoubleClick :QSystemTrayIcon__ActivationReason = 2;
// The system tray entry was clicked
pub const QSystemTrayIcon__Trigger :QSystemTrayIcon__ActivationReason = 3;
// The system tray entry was clicked with the middle mouse button
pub const QSystemTrayIcon__MiddleClick :QSystemTrayIcon__ActivationReason = 4;
pub fn QSystemTrayIcon_ActivationReasonItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSystemTrayIcon", val);
}
pub fn QSystemTrayIcon_ActivationReasonItemName_s(val: i32) ->String {
  //var nilthis *QSystemTrayIcon
  //return nilthis.ActivationReasonItemName(val);
  return QSystemTrayIcon_ActivationReasonItemName(val);
}


/*
This enum describes the icon that is shown when a balloon message is displayed.



See also QMessageBox.

*/
pub type QSystemTrayIcon__MessageIcon = i32;
// No icon is shown.
pub const QSystemTrayIcon__NoIcon :QSystemTrayIcon__MessageIcon = 0;
// An information icon is shown.
pub const QSystemTrayIcon__Information :QSystemTrayIcon__MessageIcon = 1;
// A standard warning icon is shown.
pub const QSystemTrayIcon__Warning :QSystemTrayIcon__MessageIcon = 2;
// A critical warning icon is shown.
pub const QSystemTrayIcon__Critical :QSystemTrayIcon__MessageIcon = 3;
pub fn QSystemTrayIcon_MessageIconItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSystemTrayIcon", val);
}
pub fn QSystemTrayIcon_MessageIconItemName_s(val: i32) ->String {
  //var nilthis *QSystemTrayIcon
  //return nilthis.MessageIconItemName(val);
  return QSystemTrayIcon_MessageIconItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
