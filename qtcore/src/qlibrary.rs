

// mod ::core::QLibrary
// package qtcore
// /usr/include/qt/QtCore/qlibrary.h
// #include <qlibrary.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 35
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
#[derive(Default)] // class sizeof(QLibrary)=32
pub struct QLibrary {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QLibrary_ITF interface {
//    QObject_ITF
//    QLibrary_PTR() *QLibrary
//}
//func (ptr *QLibrary) QLibrary_PTR() *QLibrary { return ptr }

impl /*struct*/ QLibrary {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QLibrary {
    return QLibrary{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QLibrary {
//  type Target = QLibraryBASE;
//
//  fn deref(&self) -> &QLibraryBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QLibraryBASE> for QLibrary {
//  fn as_ref(& self) -> & QLibraryBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qlibrary.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QLibrary {
  pub fn metaObject_0<RetType, T: QLibrary_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QLibrary_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QLibrary) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QLibrary10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:68
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QLibrary(QObject *)

/*
Constructs a library with the given parent.
*/
// QLibrary(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QLibrary {
  pub fn QLibrary_0<T: QLibrary_QLibrary_0>(value: T) -> QLibrary {
    let rsthis = value.QLibrary_0();
    return rsthis;
    // return 1;
  }
}

pub trait QLibrary_QLibrary_0 {
  fn QLibrary_0(self) -> QLibrary;
}
// QLibrary(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLibrary_QLibrary_0 for (usize) {
  fn QLibrary_0(self) -> QLibrary {
    // unsafe{_ZN8QLibraryC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QLibraryC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLibrary{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:69
// index:1
// Public Visibility=Default Availability=Available
// [-2] void QLibrary(const QString &, QObject *)

/*
Constructs a library with the given parent.
*/
// QLibrary(const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QLibrary {
  pub fn QLibrary_1<T: QLibrary_QLibrary_1>(value: T) -> QLibrary {
    let rsthis = value.QLibrary_1();
    return rsthis;
    // return 1;
  }
}

pub trait QLibrary_QLibrary_1 {
  fn QLibrary_1(self) -> QLibrary;
}
// QLibrary(const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLibrary_QLibrary_1 for (usize,usize) {
  fn QLibrary_1(self) -> QLibrary {
    // unsafe{_ZN8QLibraryC2ERK7QStringP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QLibraryC2ERK7QStringP7QObject", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLibrary{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:70
// index:2
// Public Visibility=Default Availability=Available
// [-2] void QLibrary(const QString &, int, QObject *)

/*
Constructs a library with the given parent.
*/
// QLibrary(const QString &, int, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QLibrary {
  pub fn QLibrary_2<T: QLibrary_QLibrary_2>(value: T) -> QLibrary {
    let rsthis = value.QLibrary_2();
    return rsthis;
    // return 1;
  }
}

pub trait QLibrary_QLibrary_2 {
  fn QLibrary_2(self) -> QLibrary;
}
// QLibrary(const QString &, int, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLibrary_QLibrary_2 for (usize,i32,usize) {
  fn QLibrary_2(self) -> QLibrary {
    // unsafe{_ZN8QLibraryC2ERK7QStringiP7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QLibraryC2ERK7QStringiP7QObject", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLibrary{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:71
// index:3
// Public Visibility=Default Availability=Available
// [-2] void QLibrary(const QString &, const QString &, QObject *)

/*
Constructs a library with the given parent.
*/
// QLibrary(const QString &, const QString &, QObject *) ctx.fn_proto_cpp
impl /*struct*/ QLibrary {
  pub fn QLibrary_3<T: QLibrary_QLibrary_3>(value: T) -> QLibrary {
    let rsthis = value.QLibrary_3();
    return rsthis;
    // return 1;
  }
}

pub trait QLibrary_QLibrary_3 {
  fn QLibrary_3(self) -> QLibrary;
}
// QLibrary(const QString &, const QString &, QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QLibrary_QLibrary_3 for (usize,usize,usize) {
  fn QLibrary_3(self) -> QLibrary {
    // unsafe{_ZN8QLibraryC2ERK7QStringS2_P7QObject()};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (&self.2/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN8QLibraryC2ERK7QStringS2_P7QObject", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QLibrary{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:72
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QLibrary()

/*

*/
pub fn DeleteQLibrary(this :*mut QLibrary) {
    // let rv = qtrt::InvokeQtFunc6("_ZN8QLibraryD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 32)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qlibrary.h:74
// index:0
// Public Visibility=Default Availability=Available
// [8] QFunctionPointer resolve(const char *)

/*
Returns the address of the exported symbol symbol. The library is loaded if necessary. The function returns 0 if the symbol could not be resolved or if the library could not be loaded.

Example:


  typedef int (*AvgFunction)(int, int);

  AvgFunction avg = (AvgFunction) library->resolve("avg");
  if (avg)
      return avg(5, 8);
  else
      return -1;



The symbol must be exported as a C function from the library. This means that the function must be wrapped in an extern "C" if the library is compiled with a C++ compiler. On Windows you must also explicitly export the function from the DLL using the __declspec(dllexport) compiler directive, for example:


  extern "C" MY_EXPORT int avg(int a, int b)
  {
      return (a + b) / 2;
  }



with MY_EXPORT defined as


  #ifdef Q_OS_WIN
  #define MY_EXPORT __declspec(dllexport)
  #else
  #define MY_EXPORT
  #endif
*/
impl /*struct*/ QLibrary {
  pub fn resolve_0<RetType, T: QLibrary_resolve_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.resolve_0(self);
    // return 1;
  }
}
pub trait QLibrary_resolve_0<RetType> {
  fn resolve_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_resolve_0<usize> for (usize) {
  fn resolve_0(self , rsthis: & QLibrary) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (self) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QLibrary7resolveEPKc", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:75
// index:1
// Public static Visibility=Default Availability=Available
// [8] QFunctionPointer resolve(const QString &, const char *)

/*
Returns the address of the exported symbol symbol. The library is loaded if necessary. The function returns 0 if the symbol could not be resolved or if the library could not be loaded.

Example:


  typedef int (*AvgFunction)(int, int);

  AvgFunction avg = (AvgFunction) library->resolve("avg");
  if (avg)
      return avg(5, 8);
  else
      return -1;



The symbol must be exported as a C function from the library. This means that the function must be wrapped in an extern "C" if the library is compiled with a C++ compiler. On Windows you must also explicitly export the function from the DLL using the __declspec(dllexport) compiler directive, for example:


  extern "C" MY_EXPORT int avg(int a, int b)
  {
      return (a + b) / 2;
  }



with MY_EXPORT defined as


  #ifdef Q_OS_WIN
  #define MY_EXPORT __declspec(dllexport)
  #else
  #define MY_EXPORT
  #endif
*/
impl /*struct*/ QLibrary {
  pub fn resolve_1<RetType, T: QLibrary_resolve_1<RetType>>( overload_args: T) -> RetType {
    return overload_args.resolve_1();
    // return 1;
  }
}
pub trait QLibrary_resolve_1<RetType> {
  fn resolve_1(self ) -> RetType;
}
impl<'a> /*trait*/ QLibrary_resolve_1<usize> for (usize,usize) {
  fn resolve_1(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (self.1) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QLibrary7resolveERK7QStringPKc", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:76
// index:2
// Public static Visibility=Default Availability=Available
// [8] QFunctionPointer resolve(const QString &, int, const char *)

/*
Returns the address of the exported symbol symbol. The library is loaded if necessary. The function returns 0 if the symbol could not be resolved or if the library could not be loaded.

Example:


  typedef int (*AvgFunction)(int, int);

  AvgFunction avg = (AvgFunction) library->resolve("avg");
  if (avg)
      return avg(5, 8);
  else
      return -1;



The symbol must be exported as a C function from the library. This means that the function must be wrapped in an extern "C" if the library is compiled with a C++ compiler. On Windows you must also explicitly export the function from the DLL using the __declspec(dllexport) compiler directive, for example:


  extern "C" MY_EXPORT int avg(int a, int b)
  {
      return (a + b) / 2;
  }



with MY_EXPORT defined as


  #ifdef Q_OS_WIN
  #define MY_EXPORT __declspec(dllexport)
  #else
  #define MY_EXPORT
  #endif
*/
impl /*struct*/ QLibrary {
  pub fn resolve_2<RetType, T: QLibrary_resolve_2<RetType>>( overload_args: T) -> RetType {
    return overload_args.resolve_2();
    // return 1;
  }
}
pub trait QLibrary_resolve_2<RetType> {
  fn resolve_2(self ) -> RetType;
}
impl<'a> /*trait*/ QLibrary_resolve_2<usize> for (usize,i32,usize) {
  fn resolve_2(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
    let arg2 = (self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QLibrary7resolveERK7QStringiPKc", 3,qtrt::FFITY_POINTER,qtrt::FFITY_INT,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:77
// index:3
// Public static Visibility=Default Availability=Available
// [8] QFunctionPointer resolve(const QString &, const QString &, const char *)

/*
Returns the address of the exported symbol symbol. The library is loaded if necessary. The function returns 0 if the symbol could not be resolved or if the library could not be loaded.

Example:


  typedef int (*AvgFunction)(int, int);

  AvgFunction avg = (AvgFunction) library->resolve("avg");
  if (avg)
      return avg(5, 8);
  else
      return -1;



The symbol must be exported as a C function from the library. This means that the function must be wrapped in an extern "C" if the library is compiled with a C++ compiler. On Windows you must also explicitly export the function from the DLL using the __declspec(dllexport) compiler directive, for example:


  extern "C" MY_EXPORT int avg(int a, int b)
  {
      return (a + b) / 2;
  }



with MY_EXPORT defined as


  #ifdef Q_OS_WIN
  #define MY_EXPORT __declspec(dllexport)
  #else
  #define MY_EXPORT
  #endif
*/
impl /*struct*/ QLibrary {
  pub fn resolve_3<RetType, T: QLibrary_resolve_3<RetType>>( overload_args: T) -> RetType {
    return overload_args.resolve_3();
    // return 1;
  }
}
pub trait QLibrary_resolve_3<RetType> {
  fn resolve_3(self ) -> RetType;
}
impl<'a> /*trait*/ QLibrary_resolve_3<usize> for (usize,usize,usize) {
  fn resolve_3(self ) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
    let arg2 = (self.2) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QLibrary7resolveERK7QStringS2_PKc", 3,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,arg2,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:79
// index:0
// Public Visibility=Default Availability=Available
// [1] bool load()

/*
Loads the library and returns true if the library was loaded successfully; otherwise returns false. Since resolve() always calls this function before resolving any symbols it is not necessary to call it explicitly. In some situations you might want the library loaded in advance, in which case you would use this function.

See also unload().
*/
impl /*struct*/ QLibrary {
  pub fn load_0<RetType, T: QLibrary_load_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.load_0(self);
    // return 1;
  }
}
pub trait QLibrary_load_0<RetType> {
  fn load_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_load_0<bool> for () {
  fn load_0(self , rsthis: & QLibrary) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QLibrary4loadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:80
// index:0
// Public Visibility=Default Availability=Available
// [1] bool unload()

/*
Unloads the library and returns true if the library could be unloaded; otherwise returns false.

This happens automatically on application termination, so you shouldn't normally need to call this function.

If other instances of QLibrary are using the same library, the call will fail, and unloading will only happen when every instance has called unload().

Note that on Mac OS X 10.3 (Panther), dynamic libraries cannot be unloaded.

See also resolve() and load().
*/
impl /*struct*/ QLibrary {
  pub fn unload_0<RetType, T: QLibrary_unload_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.unload_0(self);
    // return 1;
  }
}
pub trait QLibrary_unload_0<RetType> {
  fn unload_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_unload_0<bool> for () {
  fn unload_0(self , rsthis: & QLibrary) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QLibrary6unloadEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:81
// index:0
// Public Visibility=Default Availability=Available
// [1] bool isLoaded() const

/*
Returns true if the library is loaded; otherwise returns false.

See also load().
*/
impl /*struct*/ QLibrary {
  pub fn isLoaded_0<RetType, T: QLibrary_isLoaded_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.isLoaded_0(self);
    // return 1;
  }
}
pub trait QLibrary_isLoaded_0<RetType> {
  fn isLoaded_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_isLoaded_0<bool> for () {
  fn isLoaded_0(self , rsthis: & QLibrary) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QLibrary8isLoadedEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:83
// index:0
// Public static Visibility=Default Availability=Available
// [1] bool isLibrary(const QString &)

/*
Returns true if fileName has a valid suffix for a loadable library; otherwise returns false.


 PlatformValid suffixes
Windows.dll, .DLL
Unix/Linux.so
AIX.a
HP-UX.sl, .so (HP-UXi)
macOS and iOS.dylib, .bundle, .so


Trailing versioning numbers on Unix are ignored.
*/
impl /*struct*/ QLibrary {
  pub fn isLibrary_0<RetType, T: QLibrary_isLibrary_0<RetType>>( overload_args: T) -> RetType {
    return overload_args.isLibrary_0();
    // return 1;
  }
}
pub trait QLibrary_isLibrary_0<RetType> {
  fn isLibrary_0(self ) -> RetType;
}
impl<'a> /*trait*/ QLibrary_isLibrary_0<bool> for (usize) {
  fn isLibrary_0(self ) -> bool {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZN8QLibrary9isLibraryERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: bool = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:85
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileName(const QString &)

/*

*/
impl /*struct*/ QLibrary {
  pub fn setFileName_0<RetType, T: QLibrary_setFileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileName_0(self);
    // return 1;
  }
}
pub trait QLibrary_setFileName_0<RetType> {
  fn setFileName_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_setFileName_0<(/*void*/)> for (usize) {
  fn setFileName_0(self , rsthis: & QLibrary) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QLibrary11setFileNameERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:86
// index:0
// Public Visibility=Default Availability=Available
// [8] QString fileName() const

/*

*/
impl /*struct*/ QLibrary {
  pub fn fileName_0<RetType, T: QLibrary_fileName_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.fileName_0(self);
    // return 1;
  }
}
pub trait QLibrary_fileName_0<RetType> {
  fn fileName_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_fileName_0<usize> for () {
  fn fileName_0(self , rsthis: & QLibrary) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QLibrary8fileNameEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:88
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setFileNameAndVersion(const QString &, int)

/*
Sets the fileName property and major version number to fileName and versionNumber respectively. The versionNumber is ignored on Windows.

See also setFileName().
*/
impl /*struct*/ QLibrary {
  pub fn setFileNameAndVersion_0<RetType, T: QLibrary_setFileNameAndVersion_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileNameAndVersion_0(self);
    // return 1;
  }
}
pub trait QLibrary_setFileNameAndVersion_0<RetType> {
  fn setFileNameAndVersion_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_setFileNameAndVersion_0<(/*void*/)> for (usize,i32) {
  fn setFileNameAndVersion_0(self , rsthis: & QLibrary) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QLibrary21setFileNameAndVersionERK7QStringi", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:89
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setFileNameAndVersion(const QString &, const QString &)

/*
Sets the fileName property and major version number to fileName and versionNumber respectively. The versionNumber is ignored on Windows.

See also setFileName().
*/
impl /*struct*/ QLibrary {
  pub fn setFileNameAndVersion_1<RetType, T: QLibrary_setFileNameAndVersion_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setFileNameAndVersion_1(self);
    // return 1;
  }
}
pub trait QLibrary_setFileNameAndVersion_1<RetType> {
  fn setFileNameAndVersion_1(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_setFileNameAndVersion_1<(/*void*/)> for (usize,usize) {
  fn setFileNameAndVersion_1(self , rsthis: & QLibrary) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN8QLibrary21setFileNameAndVersionERK7QStringS2_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:90
// index:0
// Public Visibility=Default Availability=Available
// [8] QString errorString() const

/*
Returns a text string with the description of the last error that occurred. Currently, errorString will only be set if load(), unload() or resolve() for some reason fails.

This function was introduced in  Qt 4.2.
*/
impl /*struct*/ QLibrary {
  pub fn errorString_0<RetType, T: QLibrary_errorString_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.errorString_0(self);
    // return 1;
  }
}
pub trait QLibrary_errorString_0<RetType> {
  fn errorString_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_errorString_0<usize> for () {
  fn errorString_0(self , rsthis: & QLibrary) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QLibrary11errorStringEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:92
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setLoadHints(QLibrary::LoadHints)

/*

*/
impl /*struct*/ QLibrary {
  pub fn setLoadHints_0<RetType, T: QLibrary_setLoadHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setLoadHints_0(self);
    // return 1;
  }
}
pub trait QLibrary_setLoadHints_0<RetType> {
  fn setLoadHints_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_setLoadHints_0<(/*void*/)> for (i32) {
  fn setLoadHints_0(self , rsthis: & QLibrary) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN8QLibrary12setLoadHintsE6QFlagsINS_8LoadHintEE", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qlibrary.h:93
// index:0
// Public Visibility=Default Availability=Available
// [4] QLibrary::LoadHints loadHints() const

/*

*/
impl /*struct*/ QLibrary {
  pub fn loadHints_0<RetType, T: QLibrary_loadHints_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.loadHints_0(self);
    // return 1;
  }
}
pub trait QLibrary_loadHints_0<RetType> {
  fn loadHints_0(self , rsthis: & QLibrary) -> RetType;
}
impl<'a> /*trait*/ QLibrary_loadHints_0<i32> for () {
  fn loadHints_0(self , rsthis: & QLibrary) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK8QLibrary9loadHintsEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


/*


*/
pub type QLibrary__LoadHint = i32;
// 
pub const QLibrary__ResolveAllSymbolsHint :QLibrary__LoadHint = 1;
// 
pub const QLibrary__ExportExternalSymbolsHint :QLibrary__LoadHint = 2;
// 
pub const QLibrary__LoadArchiveMemberHint :QLibrary__LoadHint = 4;
// 
pub const QLibrary__PreventUnloadHint :QLibrary__LoadHint = 8;
// 
pub const QLibrary__DeepBindHint :QLibrary__LoadHint = 16;
pub fn QLibrary_LoadHintItemName(val: i32) ->String {
  return qtrt::GetClassEnumItemName("QLibrary", val);
}
pub fn QLibrary_LoadHintItemName_s(val: i32) ->String {
  //var nilthis *QLibrary
  //return nilthis.LoadHintItemName(val);
  return QLibrary_LoadHintItemName(val);
}

//  body block end

//  keep block begin

//  keep block end
