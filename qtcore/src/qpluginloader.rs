

// mod ::core::QPluginLoader
// package qtcore
// /usr/include/qt/QtCore/qpluginloader.h
// #include <qpluginloader.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 1
*/
// import "C"
// import "unsafe"
// import "reflect"
// import "fmt"
// import "log"
// import "github.com/kitech/qt.go/qtrt"
use std::default::Default;
use std::ops::Deref;
use qtrt; // super::super::qtrt;
use super::*;
//  ext block end

//  body block begin



/*

*/
#[derive(Default)] // class sizeof(QPluginLoader)=32
pub struct QPluginLoader {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QPluginLoader_ITF interface {
//    QObject_ITF
//    QPluginLoader_PTR() *QPluginLoader
//}
//func (ptr *QPluginLoader) QPluginLoader_PTR() *QPluginLoader { return ptr }

impl /*struct*/ QPluginLoader {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QPluginLoader {
    return QPluginLoader{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QPluginLoader {
//  type Target = QPluginLoaderBASE;
//
//  fn deref(&self) -> &QPluginLoaderBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QPluginLoaderBASE> for QPluginLoader {
//  fn as_ref(& self) -> & QPluginLoaderBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qpluginloader.h:58
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QPluginLoader {
  pub fn metaObject_0<RetType, T: QPluginLoader_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QPluginLoader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPluginLoader10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:62
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QPluginLoader(QObject *)

/*
Constructs a plugin loader with the given parent.
*/
// QPluginLoader(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QPluginLoader {
  pub fn QPluginLoader_0<T: QPluginLoader_QPluginLoader_0>(value: T) -> QPluginLoader {
    let rsthis = value.QPluginLoader_0();
    return rsthis;
    // return 1;
  }
}

pub trait QPluginLoader_QPluginLoader_0 {
  fn QPluginLoader_0(self) -> QPluginLoader;
}
// QPluginLoader(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPluginLoader_QPluginLoader_0 for (usize) {
  fn QPluginLoader_0(self) -> QPluginLoader {
    // unsafe{_ZN13QPluginLoaderC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QPluginLoaderC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPluginLoader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:63
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QPluginLoader(const QString &, QObject *)

/*
Constructs a plugin loader with the given parent.
*/
// QPluginLoader(const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QPluginLoader {
  pub fn QPluginLoader_1<T: QPluginLoader_QPluginLoader_1>(value: T) -> QPluginLoader {
    let rsthis = value.QPluginLoader_1();
    return rsthis;
    // return 1;
  }
}

pub trait QPluginLoader_QPluginLoader_1 {
  fn QPluginLoader_1(self) -> QPluginLoader;
}
// QPluginLoader(const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QPluginLoader_QPluginLoader_1 for (usize,usize) {
  fn QPluginLoader_1(self) -> QPluginLoader {
    // unsafe{_ZN13QPluginLoaderC2ERK7QStringP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QPluginLoaderC2ERK7QStringP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QPluginLoader{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:64
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QPluginLoader()

/*

*/
pub fn DeleteQPluginLoader(this :*mut QPluginLoader) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QPluginLoaderD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qpluginloader.h:66
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * instance()

/*
Returns the root component object of the plugin. The plugin is loaded if necessary. The function returns 0 if the plugin could not be loaded or if the root component object could not be instantiated.

If the root component object was destroyed, calling this function creates a new instance.

The root component, returned by this function, is not deleted when the QPluginLoader is destroyed. If you want to ensure that the root component is deleted, you should call unload() as soon you don't need to access the core component anymore. When the library is finally unloaded, the root component will automatically be deleted.

The component object is a QObject. Use qobject_cast() to access interfaces you are interested in.

See also load().
*/
impl /*struct*/ QPluginLoader {
  pub fn instance_0<RetType, T: QPluginLoader_instance_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.instance_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_instance_0<RetType> {
  fn instance_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_instance_0<usize> for () {
  fn instance_0(self , rsthis: & QPluginLoader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QPluginLoader8instanceEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:67
// index:0
// Public Visibility=Default Availability=Available
// [16] QJsonObject metaData() const

/*
Returns the meta data for this plugin. The meta data is data specified in a json format using the Q_PLUGIN_METADATA() macro when compiling the plugin.

The meta data can be queried in a fast and inexpensive way without actually loading the plugin. This makes it possible to e.g. store capabilities of the plugin in there, and make the decision whether to load the plugin dependent on this meta data.
*/
impl /*struct*/ QPluginLoader {
  pub fn metaData_0<RetType, T: QPluginLoader_metaData_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaData_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_metaData_0<RetType> {
  fn metaData_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_metaData_0<usize> for () {
  fn metaData_0(self , rsthis: & QPluginLoader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPluginLoader8metaDataEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:69
// index:0
// Public static Visibility=Default Availability=Available
// [8] QObjectList staticInstances()

/*
Returns a list of static plugin instances (root components) held by the plugin loader.

See also staticPlugins().
*/
impl /*struct*/ QPluginLoader {
  pub fn staticInstances_0<RetType, T: QPluginLoader_staticInstances_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.staticInstances_0();
    // return 1;
  }
}
pub trait QPluginLoader_staticInstances_0<RetType> {
  fn staticInstances_0(self ) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_staticInstances_0<usize> for () {
  fn staticInstances_0(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QPluginLoader15staticInstancesEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:72
// index:0
// Public Visibility=Default Availability=Available
// [1] bool load()

/*
Loads the plugin and returns true if the plugin was loaded successfully; otherwise returns false. Since instance() always calls this function before resolving any symbols it is not necessary to call it explicitly. In some situations you might want the plugin loaded in advance, in which case you would use this function.

See also unload().
*/
impl /*struct*/ QPluginLoader {
  pub fn load_0<RetType, T: QPluginLoader_load_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.load_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_load_0<RetType> {
  fn load_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_load_0<bool> for () {
  fn load_0(self , rsthis: & QPluginLoader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QPluginLoader4loadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:73
// index:0
// Public Visibility=Default Availability=Available
// [1] bool unload()

/*
Unloads the plugin and returns true if the plugin could be unloaded; otherwise returns false.

This happens automatically on application termination, so you shouldn't normally need to call this function.

If other instances of QPluginLoader are using the same plugin, the call will fail, and unloading will only happen when every instance has called unload().

Don't try to delete the root component. Instead rely on that unload() will automatically delete it when needed.

See also instance() and load().
*/
impl /*struct*/ QPluginLoader {
  pub fn unload_0<RetType, T: QPluginLoader_unload_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unload_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_unload_0<RetType> {
  fn unload_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_unload_0<bool> for () {
  fn unload_0(self , rsthis: & QPluginLoader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN13QPluginLoader6unloadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:74
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isLoaded() const

/*
Returns true if the plugin is loaded; otherwise returns false.

See also load().
*/
impl /*struct*/ QPluginLoader {
  pub fn isLoaded_0<RetType, T: QPluginLoader_isLoaded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLoaded_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_isLoaded_0<RetType> {
  fn isLoaded_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_isLoaded_0<bool> for () {
  fn isLoaded_0(self , rsthis: & QPluginLoader) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPluginLoader8isLoadedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:76
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileName(const QString &)

/*

*/
impl /*struct*/ QPluginLoader {
  pub fn setFileName_0<RetType, T: QPluginLoader_setFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileName_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_setFileName_0<RetType> {
  fn setFileName_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_setFileName_0<(/*void*/)> for (usize) {
  fn setFileName_0(self , rsthis: & QPluginLoader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QPluginLoader11setFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:77
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*

*/
impl /*struct*/ QPluginLoader {
  pub fn fileName_0<RetType, T: QPluginLoader_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QPluginLoader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPluginLoader8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:79
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns a text string with the description of the last error that occurred.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QPluginLoader {
  pub fn errorString_0<RetType, T: QPluginLoader_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QPluginLoader) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPluginLoader11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:81
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLoadHints(QLibrary::LoadHints)

/*

*/
impl /*struct*/ QPluginLoader {
  pub fn setLoadHints_0<RetType, T: QPluginLoader_setLoadHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLoadHints_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_setLoadHints_0<RetType> {
  fn setLoadHints_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_setLoadHints_0<(/*void*/)> for (i32) {
  fn setLoadHints_0(self , rsthis: & QPluginLoader) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QPluginLoader12setLoadHintsE6QFlagsIN8QLibrary8LoadHintEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qpluginloader.h:82
// index:0
// Public Visibility=Default Availability=Available
// [4] QLibrary::LoadHints loadHints() const

/*

*/
impl /*struct*/ QPluginLoader {
  pub fn loadHints_0<RetType, T: QPluginLoader_loadHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadHints_0(self);
    // return 1;
  }
}
pub trait QPluginLoader_loadHints_0<RetType> {
  fn loadHints_0(self , rsthis: & QPluginLoader) -> RetType;
}
impl<'a> /*trait*/ QPluginLoader_loadHints_0<i32> for () {
  fn loadHints_0(self , rsthis: & QPluginLoader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QPluginLoader9loadHintsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}

//  body block end

//  keep block begin

//  keep block end
