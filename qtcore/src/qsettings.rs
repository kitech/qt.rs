

// mod ::core::QSettings
// package qtcore
// /usr/include/qt/QtCore/qsettings.h
// #include <qsettings.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 12
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use super::super::qtrt;
use super::*;
//  ext block end

//  body block begin

// bool event(QEvent *)
// func (this *QSettings) InheritEvent(f func(event *QEvent/*777 QEvent **/) bool) {
//  qtrt.SetAllInheritCallback(this, "event", f)
// }



/*

*/
#[derive(Default)] // class sizeof(QSettings)=16
pub struct QSettings {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSettings_ITF interface {
//    QObject_ITF
//    QSettings_PTR() *QSettings
//}
//func (ptr *QSettings) QSettings_PTR() *QSettings { return ptr }

impl /*struct*/ QSettings {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSettings {
    return QSettings{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSettings {
//  type Target = QSettingsBASE;
//
//  fn deref(&self) -> &QSettingsBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSettingsBASE> for QSettings {
//  fn as_ref(& self) -> & QSettingsBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsettings.h:71
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSettings {
  pub fn metaObject_0<RetType, T: QSettings_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSettings_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSettings) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:127
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSettings(const QString &, const QString &, QObject *)

/*
Constructs a QSettings object for accessing settings of the application called application from the organization called organization, and with parent parent.

Example:


  QSettings settings("Moose Tech", "Facturo-Pro");



The scope is set to QSettings::UserScope, and the format is set to QSettings::NativeFormat (i.e. calling setDefaultFormat() before calling this constructor has no effect).

See also setDefaultFormat() and Fallback Mechanism.
*/
// QSettings(const QString &, const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSettings {
  pub fn QSettings_0<T: QSettings_QSettings_0>(value: T) -> QSettings {
    let rsthis = value.QSettings_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSettings_QSettings_0 {
  fn QSettings_0(self) -> QSettings;
}
// QSettings(const QString &, const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSettings_QSettings_0 for (usize,usize,usize) {
  fn QSettings_0(self) -> QSettings {
    // unsafe{_ZN9QSettingsC2ERK7QStringS2_P7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSettingsC2ERK7QStringS2_P7QObject", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSettings{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:129
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QSettings(QSettings::Scope, const QString &, const QString &, QObject *)

/*
Constructs a QSettings object for accessing settings of the application called application from the organization called organization, and with parent parent.

Example:


  QSettings settings("Moose Tech", "Facturo-Pro");



The scope is set to QSettings::UserScope, and the format is set to QSettings::NativeFormat (i.e. calling setDefaultFormat() before calling this constructor has no effect).

See also setDefaultFormat() and Fallback Mechanism.
*/
// QSettings(QSettings::Scope, const QString &, const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSettings {
  pub fn QSettings_1<T: QSettings_QSettings_1>(value: T) -> QSettings {
    let rsthis = value.QSettings_1();
    return rsthis;
    // return 1;
  }
}

pub trait QSettings_QSettings_1 {
  fn QSettings_1(self) -> QSettings;
}
// QSettings(QSettings::Scope, const QString &, const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSettings_QSettings_1 for (i32,usize,usize,usize) {
  fn QSettings_1(self) -> QSettings {
    // unsafe{_ZN9QSettingsC2ENS_5ScopeERK7QStringS3_P7QObject()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSettingsC2ENS_5ScopeERK7QStringS3_P7QObject", 4,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSettings{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:131
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QSettings(QSettings::Format, QSettings::Scope, const QString &, const QString &, QObject *)

/*
Constructs a QSettings object for accessing settings of the application called application from the organization called organization, and with parent parent.

Example:


  QSettings settings("Moose Tech", "Facturo-Pro");



The scope is set to QSettings::UserScope, and the format is set to QSettings::NativeFormat (i.e. calling setDefaultFormat() before calling this constructor has no effect).

See also setDefaultFormat() and Fallback Mechanism.
*/
// QSettings(QSettings::Format, QSettings::Scope, const QString &, const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSettings {
  pub fn QSettings_2<T: QSettings_QSettings_2>(value: T) -> QSettings {
    let rsthis = value.QSettings_2();
    return rsthis;
    // return 1;
  }
}

pub trait QSettings_QSettings_2 {
  fn QSettings_2(self) -> QSettings;
}
// QSettings(QSettings::Format, QSettings::Scope, const QString &, const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSettings_QSettings_2 for (i32,i32,usize,usize,usize) {
  fn QSettings_2(self) -> QSettings {
    // unsafe{_ZN9QSettingsC2ENS_6FormatENS_5ScopeERK7QStringS4_P7QObject()};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let arg3 = (&self.3/*.qclsinst*/) as *const usize as usize;
    let arg4 = (&self.4/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSettingsC2ENS_6FormatENS_5ScopeERK7QStringS4_P7QObject", 5,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,arg3,arg4,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSettings{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:133
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QSettings(const QString &, QSettings::Format, QObject *)

/*
Constructs a QSettings object for accessing settings of the application called application from the organization called organization, and with parent parent.

Example:


  QSettings settings("Moose Tech", "Facturo-Pro");



The scope is set to QSettings::UserScope, and the format is set to QSettings::NativeFormat (i.e. calling setDefaultFormat() before calling this constructor has no effect).

See also setDefaultFormat() and Fallback Mechanism.
*/
// QSettings(const QString &, QSettings::Format, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSettings {
  pub fn QSettings_3<T: QSettings_QSettings_3>(value: T) -> QSettings {
    let rsthis = value.QSettings_3();
    return rsthis;
    // return 1;
  }
}

pub trait QSettings_QSettings_3 {
  fn QSettings_3(self) -> QSettings;
}
// QSettings(const QString &, QSettings::Format, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSettings_QSettings_3 for (usize,i32,usize) {
  fn QSettings_3(self) -> QSettings {
    // unsafe{_ZN9QSettingsC2ERK7QStringNS_6FormatEP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSettingsC2ERK7QStringNS_6FormatEP7QObject", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSettings{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:134
// index:4
// Public Visibility=Default Availability=Available
// [-2] void QSettings(QObject *)

/*
Constructs a QSettings object for accessing settings of the application called application from the organization called organization, and with parent parent.

Example:


  QSettings settings("Moose Tech", "Facturo-Pro");



The scope is set to QSettings::UserScope, and the format is set to QSettings::NativeFormat (i.e. calling setDefaultFormat() before calling this constructor has no effect).

See also setDefaultFormat() and Fallback Mechanism.
*/
// QSettings(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSettings {
  pub fn QSettings_4<T: QSettings_QSettings_4>(value: T) -> QSettings {
    let rsthis = value.QSettings_4();
    return rsthis;
    // return 1;
  }
}

pub trait QSettings_QSettings_4 {
  fn QSettings_4(self) -> QSettings;
}
// QSettings(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSettings_QSettings_4 for (usize) {
  fn QSettings_4(self) -> QSettings {
    // unsafe{_ZN9QSettingsC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN9QSettingsC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSettings{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:144
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSettings()

/*

*/
pub fn DeleteQSettings(this :*mut QSettings) {
    // let rv = qtrt::InvokeQtFunc6("_ZN9QSettingsD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsettings.h:146
// index:0
// Public Visibility=Default Availability=Available
// [-2] void clear()

/*
Removes all entries in the primary location associated to this QSettings object.

Entries in fallback locations are not removed.

If you only want to remove the entries in the current group(), use remove("") instead.

See also remove() and setFallbacksEnabled().
*/
impl /*struct*/ QSettings {
  pub fn clear_0<RetType, T: QSettings_clear_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.clear_0(self);
    // return 1;
  }
}
pub trait QSettings_clear_0<RetType> {
  fn clear_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_clear_0<(/*void*/)> for () {
  fn clear_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QSettings5clearEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:147
// index:0
// Public Visibility=Default Availability=Available
// [-2] void sync()

/*
Writes any unsaved changes to permanent storage, and reloads any settings that have been changed in the meantime by another application.

This function is called automatically from QSettings's destructor and by the event loop at regular intervals, so you normally don't need to call it yourself.

See also status().
*/
impl /*struct*/ QSettings {
  pub fn sync_0<RetType, T: QSettings_sync_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.sync_0(self);
    // return 1;
  }
}
pub trait QSettings_sync_0<RetType> {
  fn sync_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_sync_0<(/*void*/)> for () {
  fn sync_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QSettings4syncEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:148
// index:0
// Public Visibility=Default Availability=Available
// [4] QSettings::Status status() const

/*
Returns a status code indicating the first error that was met by QSettings, or QSettings::NoError if no error occurred.

Be aware that QSettings delays performing some operations. For this reason, you might want to call sync() to ensure that the data stored in QSettings is written to disk before calling status().

See also sync().
*/
impl /*struct*/ QSettings {
  pub fn status_0<RetType, T: QSettings_status_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.status_0(self);
    // return 1;
  }
}
pub trait QSettings_status_0<RetType> {
  fn status_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_status_0<i32> for () {
  fn status_0(self , rsthis: & QSettings) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings6statusEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:149
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isAtomicSyncRequired() const

/*
Returns true if QSettings is only allowed to perform atomic saving and reloading (synchronization) of the settings. Returns false if it is allowed to save the settings contents directly to the configuration file.

The default is true.

This function was introduced in  Qt 5.10.

See also setAtomicSyncRequired() and QSaveFile.
*/
impl /*struct*/ QSettings {
  pub fn isAtomicSyncRequired_0<RetType, T: QSettings_isAtomicSyncRequired_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isAtomicSyncRequired_0(self);
    // return 1;
  }
}
pub trait QSettings_isAtomicSyncRequired_0<RetType> {
  fn isAtomicSyncRequired_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_isAtomicSyncRequired_0<bool> for () {
  fn isAtomicSyncRequired_0(self , rsthis: & QSettings) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings20isAtomicSyncRequiredEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:150
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setAtomicSyncRequired(bool)

/*
Configures whether QSettings is required to perform atomic saving and reloading (synchronization) of the settings. If the enable argument is true (the default), sync() will only perform synchronization operations that are atomic. If this is not possible, sync() will fail and status() will be an error condition.

Setting this property to false will allow QSettings to write directly to the configuration file and ignore any errors trying to lock it against other processes trying to write at the same time. Because of the potential for corruption, this option should be used with care, but is required in certain conditions, like a QSettings::IniFormat configuration file that exists in an otherwise non-writeable directory or NTFS Alternate Data Streams.

See QSaveFile for more information on the feature.

This function was introduced in  Qt 5.10.

See also isAtomicSyncRequired() and QSaveFile.
*/
impl /*struct*/ QSettings {
  pub fn setAtomicSyncRequired_0<RetType, T: QSettings_setAtomicSyncRequired_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setAtomicSyncRequired_0(self);
    // return 1;
  }
}
pub trait QSettings_setAtomicSyncRequired_0<RetType> {
  fn setAtomicSyncRequired_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_setAtomicSyncRequired_0<(/*void*/)> for (bool) {
  fn setAtomicSyncRequired_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings21setAtomicSyncRequiredEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:152
// index:0
// Public Visibility=Default Availability=Available
// [-2] void beginGroup(const QString &)

/*
Appends prefix to the current group.

The current group is automatically prepended to all keys specified to QSettings. In addition, query functions such as childGroups(), childKeys(), and allKeys() are based on the group. By default, no group is set.

Groups are useful to avoid typing in the same setting paths over and over. For example:


  settings.beginGroup("mainwindow");
  settings.setValue("size", win->size());
  settings.setValue("fullScreen", win->isFullScreen());
  settings.endGroup();

  settings.beginGroup("outputpanel");
  settings.setValue("visible", panel->isVisible());
  settings.endGroup();



This will set the value of three settings:


mainwindow/size
mainwindow/fullScreen
outputpanel/visible


Call endGroup() to reset the current group to what it was before the corresponding beginGroup() call. Groups can be nested.

See also endGroup() and group().
*/
impl /*struct*/ QSettings {
  pub fn beginGroup_0<RetType, T: QSettings_beginGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginGroup_0(self);
    // return 1;
  }
}
pub trait QSettings_beginGroup_0<RetType> {
  fn beginGroup_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_beginGroup_0<(/*void*/)> for (usize) {
  fn beginGroup_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings10beginGroupERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:153
// index:0
// Public Visibility=Default Availability=Available
// [-2] void endGroup()

/*
Resets the group to what it was before the corresponding beginGroup() call.

Example:


  settings.beginGroup("alpha");
  // settings.group() == "alpha"

  settings.beginGroup("beta");
  // settings.group() == "alpha/beta"

  settings.endGroup();
  // settings.group() == "alpha"

  settings.endGroup();
  // settings.group() == ""



See also beginGroup() and group().
*/
impl /*struct*/ QSettings {
  pub fn endGroup_0<RetType, T: QSettings_endGroup_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endGroup_0(self);
    // return 1;
  }
}
pub trait QSettings_endGroup_0<RetType> {
  fn endGroup_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_endGroup_0<(/*void*/)> for () {
  fn endGroup_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QSettings8endGroupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:154
// index:0
// Public Visibility=Default Availability=Available
// [8] QString group() const

/*
Returns the current group.

See also beginGroup() and endGroup().
*/
impl /*struct*/ QSettings {
  pub fn group_0<RetType, T: QSettings_group_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.group_0(self);
    // return 1;
  }
}
pub trait QSettings_group_0<RetType> {
  fn group_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_group_0<usize> for () {
  fn group_0(self , rsthis: & QSettings) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings5groupEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:156
// index:0
// Public Visibility=Default Availability=Available
// [4] int beginReadArray(const QString &)

/*
Adds prefix to the current group and starts reading from an array. Returns the size of the array.

Example:


  struct Login {
      QString userName;
      QString password;
  };
  QList<Login> logins;
  ...

  QSettings settings;
  int size = settings.beginReadArray("logins");
  for (int i = 0; i < size; ++i) {
      settings.setArrayIndex(i);
      Login login;
      login.userName = settings.value("userName").toString();
      login.password = settings.value("password").toString();
      logins.append(login);
  }
  settings.endArray();



Use beginWriteArray() to write the array in the first place.

See also beginWriteArray(), endArray(), and setArrayIndex().
*/
impl /*struct*/ QSettings {
  pub fn beginReadArray_0<RetType, T: QSettings_beginReadArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginReadArray_0(self);
    // return 1;
  }
}
pub trait QSettings_beginReadArray_0<RetType> {
  fn beginReadArray_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_beginReadArray_0<i32> for (usize) {
  fn beginReadArray_0(self , rsthis: & QSettings) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSettings14beginReadArrayERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:157
// index:0
// Public Visibility=Default Availability=Available
// [-2] void beginWriteArray(const QString &, int)

/*
Adds prefix to the current group and starts writing an array of size size. If size is -1 (the default), it is automatically determined based on the indexes of the entries written.

If you have many occurrences of a certain set of keys, you can use arrays to make your life easier. For example, let's suppose that you want to save a variable-length list of user names and passwords. You could then write:


  struct Login {
      QString userName;
      QString password;
  };
  QList<Login> logins;
  ...

  QSettings settings;
  settings.beginWriteArray("logins");
  for (int i = 0; i < logins.size(); ++i) {
      settings.setArrayIndex(i);
      settings.setValue("userName", list.at(i).userName);
      settings.setValue("password", list.at(i).password);
  }
  settings.endArray();



The generated keys will have the form


logins/size
logins/1/userName
logins/1/password
logins/2/userName
logins/2/password
logins/3/userName
logins/3/password
...


To read back an array, use beginReadArray().

See also beginReadArray(), endArray(), and setArrayIndex().
*/
impl /*struct*/ QSettings {
  pub fn beginWriteArray_0<RetType, T: QSettings_beginWriteArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.beginWriteArray_0(self);
    // return 1;
  }
}
pub trait QSettings_beginWriteArray_0<RetType> {
  fn beginWriteArray_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_beginWriteArray_0<(/*void*/)> for (usize,i32) {
  fn beginWriteArray_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings15beginWriteArrayERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:158
// index:0
// Public Visibility=Default Availability=Available
// [-2] void endArray()

/*
Closes the array that was started using beginReadArray() or beginWriteArray().

See also beginReadArray() and beginWriteArray().
*/
impl /*struct*/ QSettings {
  pub fn endArray_0<RetType, T: QSettings_endArray_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.endArray_0(self);
    // return 1;
  }
}
pub trait QSettings_endArray_0<RetType> {
  fn endArray_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_endArray_0<(/*void*/)> for () {
  fn endArray_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN9QSettings8endArrayEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:159
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setArrayIndex(int)

/*
Sets the current array index to i. Calls to functions such as setValue(), value(), remove(), and contains() will operate on the array entry at that index.

You must call beginReadArray() or beginWriteArray() before you can call this function.
*/
impl /*struct*/ QSettings {
  pub fn setArrayIndex_0<RetType, T: QSettings_setArrayIndex_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setArrayIndex_0(self);
    // return 1;
  }
}
pub trait QSettings_setArrayIndex_0<RetType> {
  fn setArrayIndex_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_setArrayIndex_0<(/*void*/)> for (i32) {
  fn setArrayIndex_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings13setArrayIndexEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:161
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList allKeys() const

/*
Returns a list of all keys, including subkeys, that can be read using the QSettings object.

Example:


  QSettings settings;
  settings.setValue("fridge/color", QColor(Qt::white));
  settings.setValue("fridge/size", QSize(32, 96));
  settings.setValue("sofa", true);
  settings.setValue("tv", false);

  QStringList keys = settings.allKeys();
  // keys: ["fridge/color", "fridge/size", "sofa", "tv"]



If a group is set using beginGroup(), only the keys in the group are returned, without the group prefix:


  settings.beginGroup("fridge");
  keys = settings.allKeys();
  // keys: ["color", "size"]



See also childGroups() and childKeys().
*/
impl /*struct*/ QSettings {
  pub fn allKeys_0<RetType, T: QSettings_allKeys_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.allKeys_0(self);
    // return 1;
  }
}
pub trait QSettings_allKeys_0<RetType> {
  fn allKeys_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_allKeys_0<usize> for () {
  fn allKeys_0(self , rsthis: & QSettings) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings7allKeysEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:162
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList childKeys() const

/*
Returns a list of all top-level keys that can be read using the QSettings object.

Example:


  QSettings settings;
  settings.setValue("fridge/color", QColor(Qt::white));
  settings.setValue("fridge/size", QSize(32, 96));
  settings.setValue("sofa", true);
  settings.setValue("tv", false);

  QStringList keys = settings.childKeys();
  // keys: ["sofa", "tv"]



If a group is set using beginGroup(), the top-level keys in that group are returned, without the group prefix:


  settings.beginGroup("fridge");
  keys = settings.childKeys();
  // keys: ["color", "size"]



You can navigate through the entire setting hierarchy using childKeys() and childGroups() recursively.

See also childGroups() and allKeys().
*/
impl /*struct*/ QSettings {
  pub fn childKeys_0<RetType, T: QSettings_childKeys_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childKeys_0(self);
    // return 1;
  }
}
pub trait QSettings_childKeys_0<RetType> {
  fn childKeys_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_childKeys_0<usize> for () {
  fn childKeys_0(self , rsthis: & QSettings) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings9childKeysEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:163
// index:0
// Public Visibility=Default Availability=Available
// [8] QStringList childGroups() const

/*
Returns a list of all key top-level groups that contain keys that can be read using the QSettings object.

Example:


  QSettings settings;
  settings.setValue("fridge/color", QColor(Qt::white));
  settings.setValue("fridge/size", QSize(32, 96));
  settings.setValue("sofa", true);
  settings.setValue("tv", false);

  QStringList groups = settings.childGroups();
  // groups: ["fridge"]



If a group is set using beginGroup(), the first-level keys in that group are returned, without the group prefix.


  settings.beginGroup("fridge");
  groups = settings.childGroups();
  // groups: []



You can navigate through the entire setting hierarchy using childKeys() and childGroups() recursively.

See also childKeys() and allKeys().
*/
impl /*struct*/ QSettings {
  pub fn childGroups_0<RetType, T: QSettings_childGroups_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.childGroups_0(self);
    // return 1;
  }
}
pub trait QSettings_childGroups_0<RetType> {
  fn childGroups_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_childGroups_0<usize> for () {
  fn childGroups_0(self , rsthis: & QSettings) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings11childGroupsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:164
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isWritable() const

/*
Returns true if settings can be written using this QSettings object; returns false otherwise.

One reason why isWritable() might return false is if QSettings operates on a read-only file.

Warning: This function is not perfectly reliable, because the file permissions can change at any time.

See also fileName(), status(), and sync().
*/
impl /*struct*/ QSettings {
  pub fn isWritable_0<RetType, T: QSettings_isWritable_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isWritable_0(self);
    // return 1;
  }
}
pub trait QSettings_isWritable_0<RetType> {
  fn isWritable_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_isWritable_0<bool> for () {
  fn isWritable_0(self , rsthis: & QSettings) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings10isWritableEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:166
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setValue(const QString &, const QVariant &)

/*
Sets the value of setting key to value. If the key already exists, the previous value is overwritten.

Note that the Windows registry and INI files use case-insensitive keys, whereas the CFPreferences API on macOS and iOS uses case-sensitive keys. To avoid portability problems, see the Section and Key Syntax rules.

Example:


  QSettings settings;
  settings.setValue("interval", 30);
  settings.value("interval").toInt();     // returns 30

  settings.setValue("interval", 6.55);
  settings.value("interval").toDouble();  // returns 6.55



See also value(), remove(), and contains().
*/
impl /*struct*/ QSettings {
  pub fn setValue_0<RetType, T: QSettings_setValue_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setValue_0(self);
    // return 1;
  }
}
pub trait QSettings_setValue_0<RetType> {
  fn setValue_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_setValue_0<(/*void*/)> for (usize,usize) {
  fn setValue_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings8setValueERK7QStringRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:167
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant value(const QString &, const QVariant &) const

/*
Returns the value for setting key. If the setting doesn't exist, returns defaultValue.

If no default value is specified, a default QVariant is returned.

Note that the Windows registry and INI files use case-insensitive keys, whereas the CFPreferences API on macOS and iOS uses case-sensitive keys. To avoid portability problems, see the Section and Key Syntax rules.

Example:


  QSettings settings;
  settings.setValue("animal/snake", 58);
  settings.value("animal/snake", 1024).toInt();   // returns 58
  settings.value("animal/zebra", 1024).toInt();   // returns 1024
  settings.value("animal/zebra").toInt();         // returns 0



See also setValue(), contains(), and remove().
*/
impl /*struct*/ QSettings {
  pub fn value_0<RetType, T: QSettings_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QSettings_value_0<RetType> {
  fn value_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_value_0<usize> for (usize,usize) {
  fn value_0(self , rsthis: & QSettings) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings5valueERK7QStringRK8QVariant", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:169
// index:0
// Public Visibility=Default Availability=Available
// [-2] void remove(const QString &)

/*
Removes the setting key and any sub-settings of key.

Example:


  QSettings settings;
  settings.setValue("ape");
  settings.setValue("monkey", 1);
  settings.setValue("monkey/sea", 2);
  settings.setValue("monkey/doe", 4);

  settings.remove("monkey");
  QStringList keys = settings.allKeys();
  // keys: ["ape"]



Be aware that if one of the fallback locations contains a setting with the same key, that setting will be visible after calling remove().

If key is an empty string, all keys in the current group() are removed. For example:


  QSettings settings;
  settings.setValue("ape");
  settings.setValue("monkey", 1);
  settings.setValue("monkey/sea", 2);
  settings.setValue("monkey/doe", 4);

  settings.beginGroup("monkey");
  settings.remove("");
  settings.endGroup();

  QStringList keys = settings.allKeys();
  // keys: ["ape"]



Note that the Windows registry and INI files use case-insensitive keys, whereas the CFPreferences API on macOS and iOS uses case-sensitive keys. To avoid portability problems, see the Section and Key Syntax rules.

See also setValue(), value(), and contains().
*/
impl /*struct*/ QSettings {
  pub fn remove_0<RetType, T: QSettings_remove_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.remove_0(self);
    // return 1;
  }
}
pub trait QSettings_remove_0<RetType> {
  fn remove_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_remove_0<(/*void*/)> for (usize) {
  fn remove_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings6removeERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:170
// index:0
// Public Visibility=Default Availability=Available
// [1] bool contains(const QString &) const

/*
Returns true if there exists a setting called key; returns false otherwise.

If a group is set using beginGroup(), key is taken to be relative to that group.

Note that the Windows registry and INI files use case-insensitive keys, whereas the CFPreferences API on macOS and iOS uses case-sensitive keys. To avoid portability problems, see the Section and Key Syntax rules.

See also value() and setValue().
*/
impl /*struct*/ QSettings {
  pub fn contains_0<RetType, T: QSettings_contains_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.contains_0(self);
    // return 1;
  }
}
pub trait QSettings_contains_0<RetType> {
  fn contains_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_contains_0<bool> for (usize) {
  fn contains_0(self , rsthis: & QSettings) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings8containsERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:172
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFallbacksEnabled(bool)

/*
Sets whether fallbacks are enabled to b.

By default, fallbacks are enabled.

See also fallbacksEnabled().
*/
impl /*struct*/ QSettings {
  pub fn setFallbacksEnabled_0<RetType, T: QSettings_setFallbacksEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFallbacksEnabled_0(self);
    // return 1;
  }
}
pub trait QSettings_setFallbacksEnabled_0<RetType> {
  fn setFallbacksEnabled_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_setFallbacksEnabled_0<(/*void*/)> for (bool) {
  fn setFallbacksEnabled_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const bool as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings19setFallbacksEnabledEb", 1,qtrt::FFITY_SINT8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:173
// index:0
// Public Visibility=Default Availability=Available
// [1] bool fallbacksEnabled() const

/*
Returns true if fallbacks are enabled; returns false otherwise.

By default, fallbacks are enabled.

See also setFallbacksEnabled().
*/
impl /*struct*/ QSettings {
  pub fn fallbacksEnabled_0<RetType, T: QSettings_fallbacksEnabled_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fallbacksEnabled_0(self);
    // return 1;
  }
}
pub trait QSettings_fallbacksEnabled_0<RetType> {
  fn fallbacksEnabled_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_fallbacksEnabled_0<bool> for () {
  fn fallbacksEnabled_0(self , rsthis: & QSettings) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings16fallbacksEnabledEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:175
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*
Returns the path where settings written using this QSettings object are stored.

On Windows, if the format is QSettings::NativeFormat, the return value is a system registry path, not a file path.

See also isWritable() and format().
*/
impl /*struct*/ QSettings {
  pub fn fileName_0<RetType, T: QSettings_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QSettings_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QSettings) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:176
// index:0
// Public Visibility=Default Availability=Available
// [4] QSettings::Format format() const

/*
Returns the format used for storing the settings.

This function was introduced in  Qt 4.4.

See also defaultFormat(), fileName(), scope(), organizationName(), and applicationName().
*/
impl /*struct*/ QSettings {
  pub fn format_0<RetType, T: QSettings_format_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.format_0(self);
    // return 1;
  }
}
pub trait QSettings_format_0<RetType> {
  fn format_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_format_0<i32> for () {
  fn format_0(self , rsthis: & QSettings) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings6formatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:177
// index:0
// Public Visibility=Default Availability=Available
// [4] QSettings::Scope scope() const

/*
Returns the scope used for storing the settings.

This function was introduced in  Qt 4.4.

See also format(), organizationName(), and applicationName().
*/
impl /*struct*/ QSettings {
  pub fn scope_0<RetType, T: QSettings_scope_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.scope_0(self);
    // return 1;
  }
}
pub trait QSettings_scope_0<RetType> {
  fn scope_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_scope_0<i32> for () {
  fn scope_0(self , rsthis: & QSettings) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings5scopeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:178
// index:0
// Public Visibility=Default Availability=Available
// [8] QString organizationName() const

/*
Returns the organization name used for storing the settings.

This function was introduced in  Qt 4.4.

See also QCoreApplication::organizationName(), format(), scope(), and applicationName().
*/
impl /*struct*/ QSettings {
  pub fn organizationName_0<RetType, T: QSettings_organizationName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.organizationName_0(self);
    // return 1;
  }
}
pub trait QSettings_organizationName_0<RetType> {
  fn organizationName_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_organizationName_0<usize> for () {
  fn organizationName_0(self , rsthis: & QSettings) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings16organizationNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:179
// index:0
// Public Visibility=Default Availability=Available
// [8] QString applicationName() const

/*
Returns the application name used for storing the settings.

This function was introduced in  Qt 4.4.

See also QCoreApplication::applicationName(), format(), scope(), and organizationName().
*/
impl /*struct*/ QSettings {
  pub fn applicationName_0<RetType, T: QSettings_applicationName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.applicationName_0(self);
    // return 1;
  }
}
pub trait QSettings_applicationName_0<RetType> {
  fn applicationName_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_applicationName_0<usize> for () {
  fn applicationName_0(self , rsthis: & QSettings) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings15applicationNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:182
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setIniCodec(QTextCodec *)

/*
Sets the codec for accessing INI files (including .conf files on Unix) to codec. The codec is used for decoding any data that is read from the INI file, and for encoding any data that is written to the file. By default, no codec is used, and non-ASCII characters are encoded using standard INI escape sequences.

Warning: The codec must be set immediately after creating the QSettings object, before accessing any data.

This function was introduced in  Qt 4.5.

See also iniCodec().
*/
impl /*struct*/ QSettings {
  pub fn setIniCodec_0<RetType, T: QSettings_setIniCodec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIniCodec_0(self);
    // return 1;
  }
}
pub trait QSettings_setIniCodec_0<RetType> {
  fn setIniCodec_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_setIniCodec_0<(/*void*/)> for (usize) {
  fn setIniCodec_0(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings11setIniCodecEP10QTextCodec", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:183
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setIniCodec(const char *)

/*
Sets the codec for accessing INI files (including .conf files on Unix) to codec. The codec is used for decoding any data that is read from the INI file, and for encoding any data that is written to the file. By default, no codec is used, and non-ASCII characters are encoded using standard INI escape sequences.

Warning: The codec must be set immediately after creating the QSettings object, before accessing any data.

This function was introduced in  Qt 4.5.

See also iniCodec().
*/
impl /*struct*/ QSettings {
  pub fn setIniCodec_1<RetType, T: QSettings_setIniCodec_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setIniCodec_1(self);
    // return 1;
  }
}
pub trait QSettings_setIniCodec_1<RetType> {
  fn setIniCodec_1(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_setIniCodec_1<(/*void*/)> for (usize) {
  fn setIniCodec_1(self , rsthis: & QSettings) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings11setIniCodecEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:184
// index:0
// Public Visibility=Default Availability=Available
// [8] QTextCodec * iniCodec() const

/*
Returns the codec that is used for accessing INI files. By default, no codec is used, so a null pointer is returned.

This function was introduced in  Qt 4.5.

See also setIniCodec().
*/
impl /*struct*/ QSettings {
  pub fn iniCodec_0<RetType, T: QSettings_iniCodec_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.iniCodec_0(self);
    // return 1;
  }
}
pub trait QSettings_iniCodec_0<RetType> {
  fn iniCodec_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_iniCodec_0<usize> for () {
  fn iniCodec_0(self , rsthis: & QSettings) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK9QSettings8iniCodecEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:187
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setDefaultFormat(QSettings::Format)

/*
Sets the default file format to the given format, which is used for storing settings for the QSettings(QObject *) constructor.

If no default format is set, QSettings::NativeFormat is used. See the documentation for the QSettings constructor you are using to see if that constructor will ignore this function.

This function was introduced in  Qt 4.4.

See also defaultFormat() and format().
*/
impl /*struct*/ QSettings {
  pub fn setDefaultFormat_0<RetType, T: QSettings_setDefaultFormat_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultFormat_0();
    // return 1;
  }
}
pub trait QSettings_setDefaultFormat_0<RetType> {
  fn setDefaultFormat_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSettings_setDefaultFormat_0<(/*void*/)> for (i32) {
  fn setDefaultFormat_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings16setDefaultFormatENS_6FormatE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:188
// index:0
// Public static Visibility=Default Availability=Available
// [4] QSettings::Format defaultFormat()

/*
Returns default file format used for storing settings for the QSettings(QObject *) constructor. If no default format is set, QSettings::NativeFormat is used.

This function was introduced in  Qt 4.4.

See also setDefaultFormat() and format().
*/
impl /*struct*/ QSettings {
  pub fn defaultFormat_0<RetType, T: QSettings_defaultFormat_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultFormat_0();
    // return 1;
  }
}
pub trait QSettings_defaultFormat_0<RetType> {
  fn defaultFormat_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSettings_defaultFormat_0<i32> for () {
  fn defaultFormat_0(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSettings13defaultFormatEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsettings.h:189
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setSystemIniPath(const QString &)

/*

*/
impl /*struct*/ QSettings {
  pub fn setSystemIniPath_0<RetType, T: QSettings_setSystemIniPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setSystemIniPath_0();
    // return 1;
  }
}
pub trait QSettings_setSystemIniPath_0<RetType> {
  fn setSystemIniPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSettings_setSystemIniPath_0<(/*void*/)> for (usize) {
  fn setSystemIniPath_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings16setSystemIniPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:190
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setUserIniPath(const QString &)

/*

*/
impl /*struct*/ QSettings {
  pub fn setUserIniPath_0<RetType, T: QSettings_setUserIniPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setUserIniPath_0();
    // return 1;
  }
}
pub trait QSettings_setUserIniPath_0<RetType> {
  fn setUserIniPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSettings_setUserIniPath_0<(/*void*/)> for (usize) {
  fn setUserIniPath_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings14setUserIniPathERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:191
// index:0
// Public static Visibility=Default Availability=Available
// [-2] void setPath(QSettings::Format, QSettings::Scope, const QString &)

/*
Sets the path used for storing settings for the given format and scope, to path. The format can be a custom format.

The table below summarizes the default values:


 PlatformFormatScopePath
WindowsIniFormatUserScopeFOLDERID_RoamingAppData
SystemScopeFOLDERID_ProgramData
UnixNativeFormat, IniFormatUserScope$HOME/.config
SystemScope/etc/xdg
Qt for Embedded LinuxNativeFormat, IniFormatUserScope$HOME/Settings
SystemScope/etc/xdg
macOS and iOSIniFormatUserScope$HOME/.config
SystemScope/etc/xdg


The default UserScope paths on Unix, macOS, and iOS ($HOME/.config or $HOME/Settings) can be overridden by the user by setting the XDG_CONFIG_HOME environment variable. The default SystemScope paths on Unix, macOS, and iOS (/etc/xdg) can be overridden when building the Qt library using the configure script's -sysconfdir flag (see QLibraryInfo for details).

Setting the NativeFormat paths on Windows, macOS, and iOS has no effect.

Warning: This function doesn't affect existing QSettings objects.

This function was introduced in  Qt 4.1.

See also registerFormat().
*/
impl /*struct*/ QSettings {
  pub fn setPath_0<RetType, T: QSettings_setPath_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPath_0();
    // return 1;
  }
}
pub trait QSettings_setPath_0<RetType> {
  fn setPath_0(self ) -> RetType;
}
impl<'a> /*trait*/ QSettings_setPath_0<(/*void*/)> for (i32,i32,usize) {
  fn setPath_0(self ) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0) as *const i32 as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN9QSettings7setPathENS_6FormatENS_5ScopeERK7QString", 3,qtrt::FFITY_INT,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsettings.h:202
// index:0
// Protected virtual Visibility=Default Availability=Available
// [1] bool event(QEvent *)

/*
Reimplemented from QObject::event().
*/
impl /*struct*/ QSettings {
  pub fn event_0<RetType, T: QSettings_event_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.event_0(self);
    // return 1;
  }
}
pub trait QSettings_event_0<RetType> {
  fn event_0(self , rsthis: & QSettings) -> RetType;
}
impl<'a> /*trait*/ QSettings_event_0<bool> for (usize) {
  fn event_0(self , rsthis: & QSettings) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN9QSettings5eventEP6QEvent", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}


/*
The following status values are possible:



See also status().

*/
pub type QSettings__Status = i32;
// No error occurred.
pub const QSettings__NoError :QSettings__Status = 0;
// An access error occurred (e.g. trying to write to a read-only file).
pub const QSettings__AccessError :QSettings__Status = 1;
// A format error occurred (e.g. loading a malformed INI file).
pub const QSettings__FormatError :QSettings__Status = 2;
pub fn QSettings_StatusItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSettings", val);
}
pub fn QSettings_StatusItemName_s(val: i32) ->String {
  //var nilthis *QSettings
  //return nilthis.StatusItemName(val);
  return QSettings_StatusItemName(val);
}


/*
This enum type specifies the storage format used by QSettings.



On Unix, NativeFormat and IniFormat mean the same thing, except that the file extension is different (.conf for NativeFormat, .ini for IniFormat).

The INI file format is a Windows file format that Qt supports on all platforms. In the absence of an INI standard, we try to follow what Microsoft does, with the following exceptions:


If you store types that QVariant can't convert to QString (e.g., QPoint, QRect, and QSize), Qt uses an @-based syntax to encode the type. For example:
  pos = @Point(100 100)


To minimize compatibility issues, any @ that doesn't appear at the first position in the value or that isn't followed by a Qt type (Point, Rect, Size, etc.) is treated as a normal character.

Although backslash is a special character in INI files, most Windows applications don't escape backslashes (\) in file paths:
  windir = C:\Windows


QSettings always treats backslash as a special character and provides no API for reading or writing such entries.

The INI file format has severe restrictions on the syntax of a key. Qt works around this by using % as an escape character in keys. In addition, if you save a top-level setting (a key with no slashes in it, e.g., "someKey"), it will appear in the INI file's "General" section. To avoid overwriting other keys, if you save something using a key such as "General/someKey", the key will be located in the "%General" section, not in the "General" section.
Following the philosophy that we should be liberal in what we accept and conservative in what we generate, QSettings will accept Latin-1 encoded INI files, but generate pure ASCII files, where non-ASCII values are encoded using standard INI escape sequences. To make the INI files more readable (but potentially less compatible), call setIniCodec().


See also registerFormat() and setPath().

*/
pub type QSettings__Format = i32;
// Store the settings using the most appropriate storage format for the platform. On Windows, this means the system registry; on macOS and iOS, this means the CFPreferences API; on Unix, this means textual configuration files in INI format.
pub const QSettings__NativeFormat :QSettings__Format = 0;
// Store the settings in INI files.
pub const QSettings__IniFormat :QSettings__Format = 1;
// 
pub const QSettings__InvalidFormat :QSettings__Format = 16;
// 
pub const QSettings__CustomFormat1 :QSettings__Format = 17;
// 
pub const QSettings__CustomFormat2 :QSettings__Format = 18;
// 
pub const QSettings__CustomFormat3 :QSettings__Format = 19;
// 
pub const QSettings__CustomFormat4 :QSettings__Format = 20;
// 
pub const QSettings__CustomFormat5 :QSettings__Format = 21;
// 
pub const QSettings__CustomFormat6 :QSettings__Format = 22;
// 
pub const QSettings__CustomFormat7 :QSettings__Format = 23;
// 
pub const QSettings__CustomFormat8 :QSettings__Format = 24;
// 
pub const QSettings__CustomFormat9 :QSettings__Format = 25;
// 
pub const QSettings__CustomFormat10 :QSettings__Format = 26;
// 
pub const QSettings__CustomFormat11 :QSettings__Format = 27;
// 
pub const QSettings__CustomFormat12 :QSettings__Format = 28;
// 
pub const QSettings__CustomFormat13 :QSettings__Format = 29;
// 
pub const QSettings__CustomFormat14 :QSettings__Format = 30;
// 
pub const QSettings__CustomFormat15 :QSettings__Format = 31;
// 
pub const QSettings__CustomFormat16 :QSettings__Format = 32;
pub fn QSettings_FormatItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSettings", val);
}
pub fn QSettings_FormatItemName_s(val: i32) ->String {
  //var nilthis *QSettings
  //return nilthis.FormatItemName(val);
  return QSettings_FormatItemName(val);
}


/*
This enum specifies whether settings are user-specific or shared by all users of the same system.



See also setPath().

*/
pub type QSettings__Scope = i32;
// Store settings in a location specific to the current user (e.g., in the user's home directory).
pub const QSettings__UserScope :QSettings__Scope = 0;
// Store settings in a global location, so that all users on the same machine access the same set of settings.
pub const QSettings__SystemScope :QSettings__Scope = 1;
pub fn QSettings_ScopeItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QSettings", val);
}
pub fn QSettings_ScopeItemName_s(val: i32) ->String {
  //var nilthis *QSettings
  //return nilthis.ScopeItemName(val);
  return QSettings_ScopeItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
