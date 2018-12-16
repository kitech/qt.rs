

// mod ::core::QSignalMapper
// package qtcore
// /usr/include/qt/QtCore/qsignalmapper.h
// #include <qsignalmapper.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 20
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
#[derive(Default)] // class sizeof(QSignalMapper)=16
pub struct QSignalMapper {
  qbase: QObject,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QSignalMapper_ITF interface {
//    QObject_ITF
//    QSignalMapper_PTR() *QSignalMapper
//}
//func (ptr *QSignalMapper) QSignalMapper_PTR() *QSignalMapper { return ptr }

impl /*struct*/ QSignalMapper {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QSignalMapper {
    return QSignalMapper{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QSignalMapper {
//  type Target = QSignalMapperBASE;
//
//  fn deref(&self) -> &QSignalMapperBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QSignalMapperBASE> for QSignalMapper {
//  fn as_ref(& self) -> & QSignalMapperBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qsignalmapper.h:53
// index:0
// Public virtual Visibility=Default Availability=Available
// [8] const QMetaObject * metaObject() const

/*

*/
impl /*struct*/ QSignalMapper {
  pub fn metaObject_0<RetType, T: QSignalMapper_metaObject_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.metaObject_0(self);
    // return 1;
  }
}
pub trait QSignalMapper_metaObject_0<RetType> {
  fn metaObject_0(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_metaObject_0<usize> for () {
  fn metaObject_0(self , rsthis: & QSignalMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSignalMapper10metaObjectEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:56
// index:0
// Public Visibility=Default Availability=Available
// [-2] void QSignalMapper(QObject *)

/*
This function is deprecated.

Constructs a QSignalMapper with parent parent.
*/
// QSignalMapper(QObject *) ctx.fn_proto_cpp
impl /*struct*/ QSignalMapper {
  pub fn QSignalMapper_0<T: QSignalMapper_QSignalMapper_0>(value: T) -> QSignalMapper {
    let rsthis = value.QSignalMapper_0();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalMapper_QSignalMapper_0 {
  fn QSignalMapper_0(self) -> QSignalMapper;
}
// QSignalMapper(QObject *) ctx.fn_proto_cpp
impl<'a> /*trait*/ QSignalMapper_QSignalMapper_0 for (usize) {
  fn QSignalMapper_0(self) -> QSignalMapper {
    // unsafe{_ZN13QSignalMapperC2EP7QObject()};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let qthis: usize = qtrt::InvokeQtFunc6("_ZN13QSignalMapperC2EP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    let rsthis = QSignalMapper{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:57
// index:0
// Public virtual Visibility=Default Availability=Available
// [-2] void ~QSignalMapper()

/*

*/
pub fn DeleteQSignalMapper(this :*mut QSignalMapper) {
    // let rv = qtrt::InvokeQtFunc6("_ZN13QSignalMapperD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis());
    // qtrt.Cmemset(this.GetCthis(), 9, 16)
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
// /usr/include/qt/QtCore/qsignalmapper.h:59
// index:0
// Public Visibility=Default Availability=Available
// [-2] void setMapping(QObject *, int)

/*
Adds a mapping so that when map() is signalled from the given sender, the signal mapped(id) is emitted.

There may be at most one integer ID for each sender.

See also mapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn setMapping_0<RetType, T: QSignalMapper_setMapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMapping_0(self);
    // return 1;
  }
}
pub trait QSignalMapper_setMapping_0<RetType> {
  fn setMapping_0(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_setMapping_0<(/*void*/)> for (usize,i32) {
  fn setMapping_0(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper10setMappingEP7QObjecti", 2,qtrt::FFITY_POINTER,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:60
// index:1
// Public Visibility=Default Availability=Available
// [-2] void setMapping(QObject *, const QString &)

/*
Adds a mapping so that when map() is signalled from the given sender, the signal mapped(id) is emitted.

There may be at most one integer ID for each sender.

See also mapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn setMapping_1<RetType, T: QSignalMapper_setMapping_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMapping_1(self);
    // return 1;
  }
}
pub trait QSignalMapper_setMapping_1<RetType> {
  fn setMapping_1(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_setMapping_1<(/*void*/)> for (usize,usize) {
  fn setMapping_1(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper10setMappingEP7QObjectRK7QString", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:61
// index:2
// Public Visibility=Default Availability=Available
// [-2] void setMapping(QObject *, QWidget *)

/*
Adds a mapping so that when map() is signalled from the given sender, the signal mapped(id) is emitted.

There may be at most one integer ID for each sender.

See also mapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn setMapping_2<RetType, T: QSignalMapper_setMapping_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMapping_2(self);
    // return 1;
  }
}
pub trait QSignalMapper_setMapping_2<RetType> {
  fn setMapping_2(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_setMapping_2<(/*void*/)> for (usize,usize) {
  fn setMapping_2(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper10setMappingEP7QObjectP7QWidget", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:62
// index:3
// Public Visibility=Default Availability=Available
// [-2] void setMapping(QObject *, QObject *)

/*
Adds a mapping so that when map() is signalled from the given sender, the signal mapped(id) is emitted.

There may be at most one integer ID for each sender.

See also mapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn setMapping_3<RetType, T: QSignalMapper_setMapping_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.setMapping_3(self);
    // return 1;
  }
}
pub trait QSignalMapper_setMapping_3<RetType> {
  fn setMapping_3(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_setMapping_3<(/*void*/)> for (usize,usize) {
  fn setMapping_3(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self.0/*.qclsinst*/) as *const usize as usize;
    let arg1 = (&self.1/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper10setMappingEP7QObjectS1_", 2,qtrt::FFITY_POINTER,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,arg1,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:63
// index:0
// Public Visibility=Default Availability=Available
// [-2] void removeMappings(QObject *)

/*
Removes all mappings for sender.

This is done automatically when mapped objects are destroyed.

Note: This does not disconnect any signals. If sender is not destroyed then this will need to be done explicitly if required.
*/
impl /*struct*/ QSignalMapper {
  pub fn removeMappings_0<RetType, T: QSignalMapper_removeMappings_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.removeMappings_0(self);
    // return 1;
  }
}
pub trait QSignalMapper_removeMappings_0<RetType> {
  fn removeMappings_0(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_removeMappings_0<(/*void*/)> for (usize) {
  fn removeMappings_0(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper14removeMappingsEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:65
// index:0
// Public Visibility=Default Availability=Available
// [8] QObject * mapping(int) const

/*
Returns the sender QObject that is associated with the id.

See also setMapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn mapping_0<RetType, T: QSignalMapper_mapping_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapping_0(self);
    // return 1;
  }
}
pub trait QSignalMapper_mapping_0<RetType> {
  fn mapping_0(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_mapping_0<usize> for (i32) {
  fn mapping_0(self , rsthis: & QSignalMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSignalMapper7mappingEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:66
// index:1
// Public Visibility=Default Availability=Available
// [8] QObject * mapping(const QString &) const

/*
Returns the sender QObject that is associated with the id.

See also setMapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn mapping_1<RetType, T: QSignalMapper_mapping_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapping_1(self);
    // return 1;
  }
}
pub trait QSignalMapper_mapping_1<RetType> {
  fn mapping_1(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_mapping_1<usize> for (usize) {
  fn mapping_1(self , rsthis: & QSignalMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSignalMapper7mappingERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:67
// index:2
// Public Visibility=Default Availability=Available
// [8] QObject * mapping(QWidget *) const

/*
Returns the sender QObject that is associated with the id.

See also setMapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn mapping_2<RetType, T: QSignalMapper_mapping_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapping_2(self);
    // return 1;
  }
}
pub trait QSignalMapper_mapping_2<RetType> {
  fn mapping_2(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_mapping_2<usize> for (usize) {
  fn mapping_2(self , rsthis: & QSignalMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSignalMapper7mappingEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:68
// index:3
// Public Visibility=Default Availability=Available
// [8] QObject * mapping(QObject *) const

/*
Returns the sender QObject that is associated with the id.

See also setMapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn mapping_3<RetType, T: QSignalMapper_mapping_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapping_3(self);
    // return 1;
  }
}
pub trait QSignalMapper_mapping_3<RetType> {
  fn mapping_3(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_mapping_3<usize> for (usize) {
  fn mapping_3(self , rsthis: & QSignalMapper) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK13QSignalMapper7mappingEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:71
// index:0
// Public Visibility=Default Availability=Available
// [-2] void mapped(int)

/*
This signal is emitted when map() is signalled from an object that has an integer mapping set. The object's mapped integer is passed in i.

Note: Signal mapped is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(signalMapper, QOverload<int>::of(&QSignalMapper::mapped),
      [=](int i){ /-* ... *-/ });



See also setMapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn mapped_0<RetType, T: QSignalMapper_mapped_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapped_0(self);
    // return 1;
  }
}
pub trait QSignalMapper_mapped_0<RetType> {
  fn mapped_0(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_mapped_0<(/*void*/)> for (i32) {
  fn mapped_0(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self) as *const i32 as usize;
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper6mappedEi", 1,qtrt::FFITY_INT,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:72
// index:1
// Public Visibility=Default Availability=Available
// [-2] void mapped(const QString &)

/*
This signal is emitted when map() is signalled from an object that has an integer mapping set. The object's mapped integer is passed in i.

Note: Signal mapped is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(signalMapper, QOverload<int>::of(&QSignalMapper::mapped),
      [=](int i){ /-* ... *-/ });



See also setMapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn mapped_1<RetType, T: QSignalMapper_mapped_1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapped_1(self);
    // return 1;
  }
}
pub trait QSignalMapper_mapped_1<RetType> {
  fn mapped_1(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_mapped_1<(/*void*/)> for (usize) {
  fn mapped_1(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper6mappedERK7QString", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:73
// index:2
// Public Visibility=Default Availability=Available
// [-2] void mapped(QWidget *)

/*
This signal is emitted when map() is signalled from an object that has an integer mapping set. The object's mapped integer is passed in i.

Note: Signal mapped is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(signalMapper, QOverload<int>::of(&QSignalMapper::mapped),
      [=](int i){ /-* ... *-/ });



See also setMapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn mapped_2<RetType, T: QSignalMapper_mapped_2<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapped_2(self);
    // return 1;
  }
}
pub trait QSignalMapper_mapped_2<RetType> {
  fn mapped_2(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_mapped_2<(/*void*/)> for (usize) {
  fn mapped_2(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper6mappedEP7QWidget", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:74
// index:3
// Public Visibility=Default Availability=Available
// [-2] void mapped(QObject *)

/*
This signal is emitted when map() is signalled from an object that has an integer mapping set. The object's mapped integer is passed in i.

Note: Signal mapped is overloaded in this class. To connect to this signal by using the function pointer syntax, Qt provides a convenient helper for obtaining the function pointer as shown in this example:


  connect(signalMapper, QOverload<int>::of(&QSignalMapper::mapped),
      [=](int i){ /-* ... *-/ });



See also setMapping().
*/
impl /*struct*/ QSignalMapper {
  pub fn mapped_3<RetType, T: QSignalMapper_mapped_3<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.mapped_3(self);
    // return 1;
  }
}
pub trait QSignalMapper_mapped_3<RetType> {
  fn mapped_3(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_mapped_3<(/*void*/)> for (usize) {
  fn mapped_3(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper6mappedEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:77
// index:0
// Public Visibility=Default Availability=Available
// [-2] void map()

/*
This slot emits signals based on which object sends signals to it.
*/
impl /*struct*/ QSignalMapper {
  pub fn map__0<RetType, T: QSignalMapper_map__0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__0(self);
    // return 1;
  }
}
pub trait QSignalMapper_map__0<RetType> {
  fn map__0(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_map__0<(/*void*/)> for () {
  fn map__0(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper3mapEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

// /usr/include/qt/QtCore/qsignalmapper.h:78
// index:1
// Public Visibility=Default Availability=Available
// [-2] void map(QObject *)

/*
This slot emits signals based on which object sends signals to it.
*/
impl /*struct*/ QSignalMapper {
  pub fn map__1<RetType, T: QSignalMapper_map__1<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.map__1(self);
    // return 1;
  }
}
pub trait QSignalMapper_map__1<RetType> {
  fn map__1(self , rsthis: & QSignalMapper) -> RetType;
}
impl<'a> /*trait*/ QSignalMapper_map__1<(/*void*/)> for (usize) {
  fn map__1(self , rsthis: & QSignalMapper) -> (/*void*/) {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
     qtrt::InvokeQtFunc6("_ZN13QSignalMapper3mapEP7QObject", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
  }
}

//  body block end

//  keep block begin

//  keep block end
