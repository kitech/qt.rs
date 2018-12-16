
// mod ::core::QObject
// package qtcore
// /usr/include/qt/QtCore/qobject.h
// #include <qobject.h>
// #include <QtCore>

//  header block end

//  ext block begin

/*
#include <stdlib.h>
// extern C begin: 0
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
pub struct QObjectList {
    pub qclsinst: usize /* *mut c_void*/,
}
// QList<T> & operator=(const QList<T> &)
impl QObjectList {
pub fn Operator_equal_0(&self) -> usize {
    // QObjectList_operator_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator=(QList<T> &&)
impl QObjectList {
pub fn Operator_equal_1(&self) -> usize {
    // QObjectList_operator_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void swap(QList<T> &)
impl QObjectList {
pub fn Swap_0(&self) -> (/*void*/) {
    // QObjectList_swap_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_swap_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool operator==(const QList<T> &)
impl QObjectList {
pub fn Operator_equal_equal_0(&self) -> bool {
    // QObjectList_operator_equal_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_equal_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool operator!=(const QList<T> &)
impl QObjectList {
pub fn Operator_not_equal_0(&self) -> bool {
    // QObjectList_operator_not_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_not_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int size()
impl QObjectList {
pub fn Size_0(&self) -> i32 {
    // QObjectList_size_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_size_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// void detach()
impl QObjectList {
pub fn Detach_0(&self) -> (/*void*/) {
    // QObjectList_detach_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_detach_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detachShared()
impl QObjectList {
pub fn DetachShared_0(&self) -> (/*void*/) {
    // QObjectList_detachShared_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_detachShared_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isDetached()
impl QObjectList {
pub fn IsDetached_0(&self) -> bool {
    // QObjectList_isDetached_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_isDetached_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void setSharable(bool)
impl QObjectList {
pub fn SetSharable_0(&self) -> (/*void*/) {
    // QObjectList_setSharable_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_setSharable_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isSharedWith(const QList<T> &)
impl QObjectList {
pub fn IsSharedWith_0(&self) -> bool {
    // QObjectList_isSharedWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_isSharedWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool isEmpty()
impl QObjectList {
pub fn IsEmpty_0(&self) -> bool {
    // QObjectList_isEmpty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_isEmpty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// void clear()
impl QObjectList {
pub fn Clear_0(&self) -> (/*void*/) {
    // QObjectList_clear_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_clear_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// const T & at(int)
impl QObjectList {
pub fn At_0(&self) -> usize {
    // QObjectList_at_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_at_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & operator[](int)
impl QObjectList {
pub fn Operator_get_index_0(&self) -> usize {
    // QObjectList_operator_get_index_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_get_index_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & operator[](int)
impl QObjectList {
pub fn Operator_get_index_1(&self) -> usize {
    // QObjectList_operator_get_index_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_get_index_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void reserve(int)
impl QObjectList {
pub fn Reserve_0(&self) -> (/*void*/) {
    // QObjectList_reserve_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_reserve_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const T &)
impl QObjectList {
pub fn Append_0(&self) -> (/*void*/) {
    // QObjectList_append_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_append_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void append(const QList<T> &)
impl QObjectList {
pub fn Append_1(&self) -> (/*void*/) {
    // QObjectList_append_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_append_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void prepend(const T &)
impl QObjectList {
pub fn Prepend_0(&self) -> (/*void*/) {
    // QObjectList_prepend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_prepend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void insert(int, const T &)
impl QObjectList {
pub fn Insert_0(&self) -> (/*void*/) {
    // QObjectList_insert_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_insert_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void replace(int, const T &)
impl QObjectList {
pub fn Replace_0(&self) -> (/*void*/) {
    // QObjectList_replace_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_replace_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeAt(int)
impl QObjectList {
pub fn RemoveAt_0(&self) -> (/*void*/) {
    // QObjectList_removeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_removeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int removeAll(const T &)
impl QObjectList {
pub fn RemoveAll_0(&self) -> i32 {
    // QObjectList_removeAll_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_removeAll_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool removeOne(const T &)
impl QObjectList {
pub fn RemoveOne_0(&self) -> bool {
    // QObjectList_removeOne_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_removeOne_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// T takeAt(int)
impl QObjectList {
pub fn TakeAt_0(&self) -> usize {
    // QObjectList_takeAt_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_takeAt_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeFirst()
impl QObjectList {
pub fn TakeFirst_0(&self) -> usize {
    // QObjectList_takeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_takeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T takeLast()
impl QObjectList {
pub fn TakeLast_0(&self) -> usize {
    // QObjectList_takeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_takeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void move(int, int)
impl QObjectList {
pub fn Move_0(&self) -> (/*void*/) {
    // QObjectList_move_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_move_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void swap(int, int)
impl QObjectList {
pub fn Swap_1(&self) -> (/*void*/) {
    // QObjectList_swap_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_swap_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// int indexOf(const T &, int)
impl QObjectList {
pub fn IndexOf_0(&self) -> i32 {
    // QObjectList_indexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_indexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int lastIndexOf(const T &, int)
impl QObjectList {
pub fn LastIndexOf_0(&self) -> i32 {
    // QObjectList_lastIndexOf_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_lastIndexOf_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// bool contains(const T &)
impl QObjectList {
pub fn Contains_0(&self) -> bool {
    // QObjectList_contains_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_contains_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count(const T &)
impl QObjectList {
pub fn Count_0(&self) -> i32 {
    // QObjectList_count_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_count_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// QList::iterator begin()
impl QObjectList {
pub fn Begin_0(&self) -> usize {
    // QObjectList_begin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_begin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator begin()
impl QObjectList {
pub fn Begin_1(&self) -> usize {
    // QObjectList_begin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_begin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cbegin()
impl QObjectList {
pub fn Cbegin_0(&self) -> usize {
    // QObjectList_cbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_cbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constBegin()
impl QObjectList {
pub fn ConstBegin_0(&self) -> usize {
    // QObjectList_constBegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_constBegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator end()
impl QObjectList {
pub fn End_0(&self) -> usize {
    // QObjectList_end_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_end_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator end()
impl QObjectList {
pub fn End_1(&self) -> usize {
    // QObjectList_end_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_end_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator cend()
impl QObjectList {
pub fn Cend_0(&self) -> usize {
    // QObjectList_cend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_cend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_iterator constEnd()
impl QObjectList {
pub fn ConstEnd_0(&self) -> usize {
    // QObjectList_constEnd_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_constEnd_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rbegin()
impl QObjectList {
pub fn Rbegin_0(&self) -> usize {
    // QObjectList_rbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_rbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::reverse_iterator rend()
impl QObjectList {
pub fn Rend_0(&self) -> usize {
    // QObjectList_rend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_rend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rbegin()
impl QObjectList {
pub fn Rbegin_1(&self) -> usize {
    // QObjectList_rbegin_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_rbegin_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator rend()
impl QObjectList {
pub fn Rend_1(&self) -> usize {
    // QObjectList_rend_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_rend_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crbegin()
impl QObjectList {
pub fn Crbegin_0(&self) -> usize {
    // QObjectList_crbegin_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_crbegin_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::const_reverse_iterator crend()
impl QObjectList {
pub fn Crend_0(&self) -> usize {
    // QObjectList_crend_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_crend_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator insert(QList::iterator, const T &)
impl QObjectList {
pub fn Insert_1(&self) -> usize {
    // QObjectList_insert_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_insert_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator)
impl QObjectList {
pub fn Erase_0(&self) -> usize {
    // QObjectList_erase_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_erase_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList::iterator erase(QList::iterator, QList::iterator)
impl QObjectList {
pub fn Erase_1(&self) -> usize {
    // QObjectList_erase_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_erase_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// int count()
impl QObjectList {
pub fn Count_1(&self) -> i32 {
    // QObjectList_count_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_count_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int length()
impl QObjectList {
pub fn Length_0(&self) -> i32 {
    // QObjectList_length_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_length_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// T & first()
impl QObjectList {
pub fn First_0(&self) -> usize {
    // QObjectList_first_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_first_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constFirst()
impl QObjectList {
pub fn ConstFirst_0(&self) -> usize {
    // QObjectList_constFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_constFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & first()
impl QObjectList {
pub fn First_1(&self) -> usize {
    // QObjectList_first_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_first_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & last()
impl QObjectList {
pub fn Last_0(&self) -> usize {
    // QObjectList_last_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_last_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & last()
impl QObjectList {
pub fn Last_1(&self) -> usize {
    // QObjectList_last_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_last_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & constLast()
impl QObjectList {
pub fn ConstLast_0(&self) -> usize {
    // QObjectList_constLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_constLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void removeFirst()
impl QObjectList {
pub fn RemoveFirst_0(&self) -> (/*void*/) {
    // QObjectList_removeFirst_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_removeFirst_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void removeLast()
impl QObjectList {
pub fn RemoveLast_0(&self) -> (/*void*/) {
    // QObjectList_removeLast_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_removeLast_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool startsWith(const T &)
impl QObjectList {
pub fn StartsWith_0(&self) -> bool {
    // QObjectList_startsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_startsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool endsWith(const T &)
impl QObjectList {
pub fn EndsWith_0(&self) -> bool {
    // QObjectList_endsWith_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_endsWith_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> mid(int, int)
impl QObjectList {
pub fn Mid_0(&self) -> usize {
    // QObjectList_mid_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_mid_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int)
impl QObjectList {
pub fn Value_0(&self) -> usize {
    // QObjectList_value_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_value_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T value(int, const T &)
impl QObjectList {
pub fn Value_1(&self) -> usize {
    // QObjectList_value_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_value_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void push_back(const T &)
impl QObjectList {
pub fn Push_back_0(&self) -> (/*void*/) {
    // QObjectList_push_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_push_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void push_front(const T &)
impl QObjectList {
pub fn Push_front_0(&self) -> (/*void*/) {
    // QObjectList_push_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_push_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// T & front()
impl QObjectList {
pub fn Front_0(&self) -> usize {
    // QObjectList_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & front()
impl QObjectList {
pub fn Front_1(&self) -> usize {
    // QObjectList_front_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_front_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// T & back()
impl QObjectList {
pub fn Back_0(&self) -> usize {
    // QObjectList_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// const T & back()
impl QObjectList {
pub fn Back_1(&self) -> usize {
    // QObjectList_back_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_back_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void pop_front()
impl QObjectList {
pub fn Pop_front_0(&self) -> (/*void*/) {
    // QObjectList_pop_front_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_pop_front_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void pop_back()
impl QObjectList {
pub fn Pop_back_0(&self) -> (/*void*/) {
    // QObjectList_pop_back_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_pop_back_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool empty()
impl QObjectList {
pub fn Empty_0(&self) -> bool {
    // QObjectList_empty_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_empty_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// QList<T> & operator+=(const QList<T> &)
impl QObjectList {
pub fn Operator_add_equal_0(&self) -> usize {
    // QObjectList_operator_add_equal_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_add_equal_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> operator+(const QList<T> &)
impl QObjectList {
pub fn Operator_add_0(&self) -> usize {
    // QObjectList_operator_add_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_add_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator+=(const T &)
impl QObjectList {
pub fn Operator_add_equal_1(&self) -> usize {
    // QObjectList_operator_add_equal_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_add_equal_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const T &)
impl QObjectList {
pub fn Operator_left_shift_0(&self) -> usize {
    // QObjectList_operator_left_shift_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_left_shift_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> & operator<<(const QList<T> &)
impl QObjectList {
pub fn Operator_left_shift_1(&self) -> usize {
    // QObjectList_operator_left_shift_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_operator_left_shift_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QVector<T> toVector()
impl QObjectList {
pub fn ToVector_0(&self) -> usize {
    // QObjectList_toVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_toVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QSet<T> toSet()
impl QObjectList {
pub fn ToSet_0(&self) -> usize {
    // QObjectList_toSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_toSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromVector(const QVector<T> &)
impl QObjectList {
pub fn FromVector_0(&self) -> usize {
    // QObjectList_fromVector_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_fromVector_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromSet(const QSet<T> &)
impl QObjectList {
pub fn FromSet_0(&self) -> usize {
    // QObjectList_fromSet_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_fromSet_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// QList<T> fromStdList(const std::list<T> &)
impl QObjectList {
pub fn FromStdList_0(&self) -> usize {
    // QObjectList_fromStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_fromStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// std::list<T> toStdList()
impl QObjectList {
pub fn ToStdList_0(&self) -> i32 {
    // QObjectList_toStdList_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_toStdList_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :i32 = Default::default();
    return dret;
  }
}
// QList::Node * detach_helper_grow(int, int)
impl QObjectList {
pub fn Detach_helper_grow_0(&self) -> usize {
    // QObjectList_detach_helper_grow_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_detach_helper_grow_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    let dret :usize = Default::default();
    return dret;
  }
}
// void detach_helper(int)
impl QObjectList {
pub fn Detach_helper_0(&self) -> (/*void*/) {
    // QObjectList_detach_helper_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_detach_helper_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void detach_helper()
impl QObjectList {
pub fn Detach_helper_1(&self) -> (/*void*/) {
    // QObjectList_detach_helper_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_detach_helper_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void dealloc(QListData::Data *)
impl QObjectList {
pub fn Dealloc_0(&self) -> (/*void*/) {
    // QObjectList_dealloc_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_dealloc_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_construct(QList::Node *, const T &)
impl QObjectList {
pub fn Node_construct_0(&self) -> (/*void*/) {
    // QObjectList_node_construct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_node_construct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *)
impl QObjectList {
pub fn Node_destruct_0(&self) -> (/*void*/) {
    // QObjectList_node_destruct_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_node_destruct_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_copy(QList::Node *, QList::Node *, QList::Node *)
impl QObjectList {
pub fn Node_copy_0(&self) -> (/*void*/) {
    // QObjectList_node_copy_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_node_copy_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// void node_destruct(QList::Node *, QList::Node *)
impl QObjectList {
pub fn Node_destruct_1(&self) -> (/*void*/) {
    // QObjectList_node_destruct_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_node_destruct_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
  }
}
// bool isValidIterator(const QList::iterator &)
impl QObjectList {
pub fn IsValidIterator_0(&self) -> bool {
    // QObjectList_isValidIterator_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_isValidIterator_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::NotArrayCompatibleLayout)
impl QObjectList {
pub fn Op_eq_impl_0(&self) -> bool {
    // QObjectList_op_eq_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_op_eq_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool op_eq_impl(const QList<T> &, QListData::ArrayCompatibleLayout)
impl QObjectList {
pub fn Op_eq_impl_1(&self) -> bool {
    // QObjectList_op_eq_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_op_eq_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QObjectList {
pub fn Contains_impl_0(&self) -> bool {
    // QObjectList_contains_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_contains_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// bool contains_impl(const T &, QListData::ArrayCompatibleLayout)
impl QObjectList {
pub fn Contains_impl_1(&self) -> bool {
    // QObjectList_contains_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_contains_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0==0;
  }
}
// int count_impl(const T &, QListData::NotArrayCompatibleLayout)
impl QObjectList {
pub fn Count_impl_0(&self) -> i32 {
    // QObjectList_count_impl_0()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_count_impl_0", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
// int count_impl(const T &, QListData::ArrayCompatibleLayout)
impl QObjectList {
pub fn Count_impl_1(&self) -> i32 {
    // QObjectList_count_impl_1()
    // rv, err := qtrt::InvokeQtFunc6("C_QObjectList_count_impl_1", qtrt.FFI_TYPE_POINTER, this.Cthis)
    // qtrt::ErrPrint(err, rv);
    return 0;
  }
}
//  body block end
