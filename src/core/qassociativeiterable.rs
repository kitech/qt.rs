

// mod ::core::QAssociativeIterable
// package qtcore
// /usr/include/qt/QtCore/qvariant.h
// #include <qvariant.h>
// #include <QtCore>

//  header block end

//  main block begin

//  main block end

//  use block begin

//  use block end

//  ext block begin


/*
#include <stdlib.h>
// extern C begin: 5
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



/*

*/
#[derive(Default)] // class sizeof(QAssociativeIterable)=112
pub struct QAssociativeIterable {
  // qbase: none,
  pub qclsinst: usize /* *mut c_void*/,
}
// type QAssociativeIterable_ITF interface {
//    QAssociativeIterable_PTR() *QAssociativeIterable
//}
//func (ptr *QAssociativeIterable) QAssociativeIterable_PTR() *QAssociativeIterable { return ptr }

impl /*struct*/ QAssociativeIterable {
  pub fn inheritFrom(qthis: usize /* *mut c_void*/) -> QAssociativeIterable {
    return QAssociativeIterable{qclsinst: qthis, ..Default::default()};
  }
}
//impl Deref for QAssociativeIterable {
//  type Target = QAssociativeIterableBASE;
//
//  fn deref(&self) -> &QAssociativeIterableBASE {
//    return & self.qbase;
//  }
//}
//impl AsRef<QAssociativeIterableBASE> for QAssociativeIterable {
//  fn as_ref(& self) -> & QAssociativeIterableBASE {
//    return & self.qbase;
//  }
//}
// /usr/include/qt/QtCore/qvariant.h:680
// index:0
// Public Visibility=Default Availability=Available
// [120] QAssociativeIterable::const_iterator begin() const

/*

*/
impl /*struct*/ QAssociativeIterable {
  pub fn begin_0<RetType, T: QAssociativeIterable_begin_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.begin_0(self);
    // return 1;
  }
}
pub trait QAssociativeIterable_begin_0<RetType> {
  fn begin_0(self , rsthis: & QAssociativeIterable) -> RetType;
}
impl<'a> /*trait*/ QAssociativeIterable_begin_0<usize> for () {
  fn begin_0(self , rsthis: & QAssociativeIterable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAssociativeIterable5beginEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:681
// index:0
// Public Visibility=Default Availability=Available
// [120] QAssociativeIterable::const_iterator end() const

/*

*/
impl /*struct*/ QAssociativeIterable {
  pub fn end_0<RetType, T: QAssociativeIterable_end_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.end_0(self);
    // return 1;
  }
}
pub trait QAssociativeIterable_end_0<RetType> {
  fn end_0(self , rsthis: & QAssociativeIterable) -> RetType;
}
impl<'a> /*trait*/ QAssociativeIterable_end_0<usize> for () {
  fn end_0(self , rsthis: & QAssociativeIterable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAssociativeIterable3endEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:682
// index:0
// Public Visibility=Default Availability=Available
// [120] QAssociativeIterable::const_iterator find(const QVariant &) const

/*

*/
impl /*struct*/ QAssociativeIterable {
  pub fn find_0<RetType, T: QAssociativeIterable_find_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.find_0(self);
    // return 1;
  }
}
pub trait QAssociativeIterable_find_0<RetType> {
  fn find_0(self , rsthis: & QAssociativeIterable) -> RetType;
}
impl<'a> /*trait*/ QAssociativeIterable_find_0<usize> for (usize) {
  fn find_0(self , rsthis: & QAssociativeIterable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAssociativeIterable4findERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:684
// index:0
// Public Visibility=Default Availability=Available
// [16] QVariant value(const QVariant &) const

/*
Returns the stored value converted to the template type T. Call canConvert() to find out whether a type can be converted. If the value cannot be converted, a default-constructed value will be returned.

If the type T is supported by QVariant, this function behaves exactly as toString(), toInt() etc.

Example:


  QVariant v;

  MyCustomStruct c;
  if (v.canConvert<MyCustomStruct>())
      c = v.value<MyCustomStruct>();

  v = 7;
  int i = v.value<int>();                        // same as v.toInt()
  QString s = v.value<QString>();                // same as v.toString(), s is now "7"
  MyCustomStruct c2 = v.value<MyCustomStruct>(); // conversion failed, c2 is empty



If the QVariant contains a pointer to a type derived from QObject then T may be any QObject type. If the pointer stored in the QVariant can be qobject_cast to T, then that result is returned. Otherwise a null pointer is returned. Note that this only works for QObject subclasses which use the Q_OBJECT macro.

If the QVariant contains a sequential container and T is QVariantList, the elements of the container will be converted into QVariants and returned as a QVariantList.


  QList<int> intList = {7, 11, 42};

  QVariant variant = QVariant::fromValue(intList);
  if (variant.canConvert<QVariantList>()) {
      QSequentialIterable iterable = variant.value<QSequentialIterable>();
      // Can use foreach:
      foreach (const QVariant &v, iterable) {
          qDebug() << v;
      }
      // Can use C++11 range-for:
      for (const QVariant &v : iterable) {
          qDebug() << v;
      }
      // Can use iterators:
      QSequentialIterable::const_iterator it = iterable.begin();
      const QSequentialIterable::const_iterator end = iterable.end();
      for ( ; it != end; ++it) {
          qDebug() << *it;
      }
  }



See also setValue(), fromValue(), canConvert(), and Q_DECLARE_SEQUENTIAL_CONTAINER_METATYPE().
*/
impl /*struct*/ QAssociativeIterable {
  pub fn value_0<RetType, T: QAssociativeIterable_value_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.value_0(self);
    // return 1;
  }
}
pub trait QAssociativeIterable_value_0<RetType> {
  fn value_0(self , rsthis: & QAssociativeIterable) -> RetType;
}
impl<'a> /*trait*/ QAssociativeIterable_value_0<usize> for (usize) {
  fn value_0(self , rsthis: & QAssociativeIterable) -> usize {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let arg0 = (&self/*.qclsinst*/) as *const usize as usize;
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAssociativeIterable5valueERK8QVariant", 1,qtrt::FFITY_POINTER,0,0,0,0,0,0,0,0,0,0,0,0,0,0,arg0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: usize = Default::default(); return dret;
  }
}

// /usr/include/qt/QtCore/qvariant.h:686
// index:0
// Public Visibility=Default Availability=Available
// [4] int size() const

/*

*/
impl /*struct*/ QAssociativeIterable {
  pub fn size_0<RetType, T: QAssociativeIterable_size_0<RetType>>(&self,  overload_args: T) -> RetType {
    return overload_args.size_0(self);
    // return 1;
  }
}
pub trait QAssociativeIterable_size_0<RetType> {
  fn size_0(self , rsthis: & QAssociativeIterable) -> RetType;
}
impl<'a> /*trait*/ QAssociativeIterable_size_0<i32> for () {
  fn size_0(self , rsthis: & QAssociativeIterable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, ctx.ctysz)};
    let mut ret = qtrt::InvokeQtFunc6("_ZNK20QAssociativeIterable4sizeEv", 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0);
    // return 1;
    let dret: i32 = Default::default(); return dret;
  }
}


pub fn DeleteQAssociativeIterable(this :*mut QAssociativeIterable) {
    // rv, err := qtrt::InvokeQtFunc6("_ZN20QAssociativeIterableD2Ev", qtrt.FFI_TYPE_VOID, this.GetCthis())
    // qtrt.ErrPrint(err, rv)
    // this.SetCthis(nil)
}
//  body block end

//  keep block begin

//  keep block end
